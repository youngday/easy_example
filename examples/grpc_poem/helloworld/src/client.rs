use poem_grpc::{ClientConfig, Request};

// let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
// tonic_build::configure()
//     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
//     .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
//     .unwrap();
// // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();
// tonic_build::configure()
//     .compile(&["proto/echo/echo.proto"], &["proto"])
//     .unwrap();

poem_grpc::include_proto!("helloworld");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = GreeterClient::new(
        ClientConfig::builder()
            .uri("http://localhost:3000")
            .build()
            .unwrap(),
    );
    let request = Request::new(HelloRequest {
        name: "Poem".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={response:?}");
    Ok(())
}
