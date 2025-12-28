use std::{fs, io, path, sync::Arc};

use anyhow::Context;
use clap::Parser;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::aws_lc_rs,
};
use url::Url;

// Custom certificate verifier that doesn't verify certificates
#[derive(Debug)]
struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Always accept the certificate
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        aws_lc_rs::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "https://localhost:4443")]
    url: Url,

    /// Accept the certificates at this path, encoded as PEM.
    #[arg(long)]
    tls_cert: Option<path::PathBuf>,

    /// Dangerous: Disable TLS certificate verification.
    #[arg(long, default_value = "false")]
    tls_disable_verify: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable info logging.
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    let client = if args.tls_disable_verify {
        log::warn!("disabling TLS certificate verification; a MITM attack is possible");

        // Create a custom TLS configuration that doesn't verify certificates
        let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13])?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoCertificateVerification))
        .with_no_client_auth();
        config.alpn_protocols = vec![web_transport_quinn::ALPN.as_bytes().to_vec()];

        let config: quinn::crypto::rustls::QuicClientConfig = config.try_into()?;
        let config = quinn::ClientConfig::new(Arc::new(config));

        let client = quinn::Endpoint::client("[::]:0".parse()?)?;
        web_transport_quinn::Client::new(client, config)
    } else if let Some(path) = &args.tls_cert {
        // Read the PEM certificate chain
        let chain = fs::File::open(path).context("failed to open cert file")?;
        let mut chain = io::BufReader::new(chain);

        let chain: Vec<CertificateDer> = rustls_pemfile::certs(&mut chain)
            .collect::<Result<_, _>>()
            .context("failed to load certs")?;

        anyhow::ensure!(!chain.is_empty(), "could not find certificate");

        // Create a client builder with custom certificates
        let client = web_transport_quinn::ClientBuilder::new();
        client.with_server_certificates(chain)?
    } else {
        // Accept any certificate that matches a system root.
        let client = web_transport_quinn::ClientBuilder::new();
        client.with_system_roots()?
    };

    log::info!("connecting to {}", args.url);

    // Connect to the given URL.
    let session = client.connect(args.url).await?;

    log::info!("connected");

    // Create a bidirectional stream.
    let (mut send, mut recv) = session.open_bi().await?;

    log::info!("created stream");

    // Send a message.
    let msg = "hello world".to_string();
    send.write_all(msg.as_bytes()).await?;
    log::info!("sent: {msg}");

    // Shut down the send stream.
    send.finish()?;

    // Read back the message.
    let msg = recv.read_to_end(1024).await?;
    log::info!("recv: {}", String::from_utf8_lossy(&msg));

    Ok(())
}
