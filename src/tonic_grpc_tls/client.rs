pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod echo {
    tonic::include_proto!("grpc.examples.echo");
}

// use std::thread::sleep;

use tokio::time::sleep;
use std::time::Duration;

use echo::{echo_client::EchoClient, EchoRequest};
use hello_world::{greeter_client::GreeterClient, HelloRequest};
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint,Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pem = tokio::fs::read("data/tls/ca.pem").await?;
    let server_ca = Certificate::from_pem(pem);
    let client_cert = tokio::fs::read("data/tls/client1.pem").await?;
    let client_key = tokio::fs::read("data/tls/client1.key").await?;
    let client_identity = Identity::from_pem(client_cert, client_key);
    let tls = ClientTlsConfig::new()
        .ca_certificate(server_ca)
        .identity(client_identity)
        .domain_name("example.com");

    let channel = Endpoint::from_static("https://127.0.0.1:50051")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut greeter_client = GreeterClient::new(channel.clone());
    let mut echo_client = EchoClient::new(channel);
    let mut cnt:i32=0;
    loop {
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });
    
        let response = greeter_client.say_hello(request).await?;
    
        println!("GREETER RESPONSE={:?}", response);
    
        let request = tonic::Request::new(EchoRequest {
            message: "hello".into(),
        });
    
        let response = echo_client.unary_echo(request).await?;
    
        cnt+=1;

   
        println!("ECHO cnt:{0} RESPONSE=\n{1:?}", cnt,response);
        sleep(Duration::from_secs(1)).await;
    }
  

    // Ok(())
}
