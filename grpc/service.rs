use tonic::{transport::Server, Request, Response, Status};
use crate::grpc::types::*;

pub mod p2p {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("p2p");
}

pub struct Node1 {}
pub struct Node2 {}
pub struct Node3 {}

impl P2PService for Node1 {

}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = p2p::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}