mod common;
mod config;
mod controllers;
mod dtos;
mod grpc;
mod schemas;
mod services;
// mod utils;
// mod verifier;

use tonic::transport;
use actix_web::{
    web::Data,
    App,
    HttpServer,
    dev::Service
};
use futures::{
    future::{
        self,
        Future,
        join,
        ok,
        abortable,
    },
    FutureExt,
};
use tokio::signal::ctrl_c;
use to_unit::ToUnit;
use mongodb::Database;
use dotenv::dotenv;
use std::{
    env,
    path,
    cmp,
    io
};
use crate::{
    config::{
        db,
        app
    },
    grpc::controller
};

async fn tokio_main(tonic_future: impl Future<Output = Result<(), transport::Error>>) -> Result<(), transport::Error> {
    let (f_tonic, aborter) = abortable(tonic_future);

    let f_sigint = async move {
        ctrl_c().await.to_unit();
        aborter.abort();
    };

    let r = join(f_tonic, f_sigint).await;
    match r.0 {
        Ok(Err(e_tonic)) => Err(e_tonic),
        _ => Ok(())
    }
}

async fn actix_main(actix_future: impl Future<Output = Result<(), io::Error>>) -> io::Result<()> {
    let fake_future = ok::<(), ()>(());
    let r = join(actix_future, fake_future);
    
    r.await.0
}

fn formatted_log(str1: String, str2: String) {
    let max_len = cmp::max(str1.len(), str2.len());
    let horizontal_line = format!("+-{}-+", "-".repeat(max_len));

    print!(
        "{}\n{}\n{}\n{}\n",
        horizontal_line,
        format!("| {}{}{} |", "\x1b[32m", str1, "\x1b[0m"),
        format!("| {}{}{} |", "\x1b[32m", str2, "\x1b[0m"),
        horizontal_line
    )
}

#[actix_web::main]
async fn main() -> io::Result<()> {
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


    let client: Database = db::database().await;
    let host: String = dotenv::var("HOST").unwrap();
    let http_port: String = dotenv::var("HTTP_PORT").unwrap();
    let grpc_port: String = dotenv::var("GRPC_PORT").unwrap();

    formatted_log(
        format!("HTTP Server listening on http://{}:{}", host, http_port),
        format!("GRPC Server listening on http://{}:{}", host, grpc_port)
    );

    let http_server = HttpServer::new(move || {
        App::new()
            .configure(app::config_services)
            .app_data(Data::new(client.clone()))
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
        })
        .bind(&format!("{}:{}", host, http_port))
        .unwrap()
        .run();

    let grpc_server = transport::Server::builder()
        .add_service(controller::P2PController::new())
        .serve(format!("{}:{}", host, grpc_port).parse().unwrap());

    let r_actix = actix_main(http_server);
    let r_tokio = tokio_main(grpc_server);
    
    let r = future::join(r_actix, r_tokio).await;
    match r {
        (Ok(..), Ok(..)) => {},
        _ => {
            panic!("Error when running servers");
        }
    };

    Ok(())
}