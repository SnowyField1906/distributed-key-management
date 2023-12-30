mod common;
mod config;
mod controllers;
mod dtos;
mod grpc;
mod schemas;
mod services;
// mod utils;
// mod verifier;

use actix_cors::Cors;
use actix_web::{
    http,
    web::Data,
    App,
    HttpServer,
    dev::Service
};
use futures::FutureExt;
use mongodb::Database;
use dotenv::dotenv;
use std::{
    env,
    path
};
use crate::config::{
    db,
    app
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    println!("HTTP Server listening on http://{}:{}", host, http_port);

    HttpServer::new(move || {
        App::new()
            .configure(app::config_services)
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(Data::new(client.clone()))
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
    })
        .bind(&format!("{}:{}", host, http_port))?
        .run()
        .await
}