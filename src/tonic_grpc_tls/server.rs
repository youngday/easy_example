use futures::Stream;
use std::pin::Pin;
use tonic::{
    transport::{
        server::{TcpConnectInfo, TlsConnectInfo},
        Certificate, Identity, Server, ServerTlsConfig,
    },
    Request, Response, Status, Streaming,
};
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod echo {
    tonic::include_proto!("grpc.examples.echo");
}

use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

use echo::{
    echo_server::{Echo, EchoServer},
    EchoRequest, EchoResponse,
};

static mut CNT: i32 = 0;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = tokio::fs::read("data/tls/server.pem").await?;
    let key = tokio::fs::read("data/tls/server.key").await?;
    let client_ca_cert = tokio::fs::read("data/tls/client_ca.pem").await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    let server_identity = Identity::from_pem(cert, key);

    let addr = "127.0.0.1:50051".parse().unwrap();

    let greeter = GreeterServer::new(MyGreeter::default());
    let echo = EchoServer::new(MyEcho::default());

    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        .tls_config(tls)?
        .add_service(greeter)
        .add_service(echo)
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MyEcho;

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("data start. \n");

        let conn_info = request
            .extensions()
            .get::<TlsConnectInfo<TcpConnectInfo>>()
            .unwrap();
        println!(
            "Got a request from {:?} with info {:?}",
            request.remote_addr(),
            conn_info
        );
       
        unsafe{
            println!("data end,cnt: {}.",CNT);
            CNT+=1;
        }
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }

    type ServerStreamingEchoStream = ResponseStream;

    async fn server_streaming_echo(
        &self,
        _: Request<EchoRequest>,
    ) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn client_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<EchoResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn bidirectional_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}
