mod common;
mod config;
mod controllers;
mod dtos;
mod grpc;
mod schemas;
mod services;
// mod utils;
// mod verifier;

use tonic::transport::Server;
use futures::FutureExt;
use dotenv::dotenv;
use std::{
    env,
    path
};
use crate::{
    config::{
        db,
        app
    },
    grpc::controller,
};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().ok();

    let node: usize = env::args().nth(1).unwrap().parse().unwrap();

    let env_path: path::PathBuf = env::current_dir().and_then(|a| Ok(a
        .join("config")
        .join("node_info")
        .join(format!("node{}.env", node))
    )).unwrap();
    dotenv::from_path(env_path.as_path()).ok();

    let host: String = dotenv::var("HOST").unwrap();
    let grpc_port: String = dotenv::var("GRPC_PORT").unwrap();

    println!("GRPC Server listening on http://{}:{}", host, grpc_port);

    Server::builder()
        .add_service(controller::P2PController::new())
        .serve(format!("{}:{}", host, grpc_port).parse().unwrap())
        .await;
}