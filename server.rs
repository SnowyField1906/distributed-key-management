mod common;
mod config;
mod controllers;
mod dtos;
mod grpc;
mod schemas;
mod services;
// mod utils;
// mod verifier;

use std::{
	cmp,
	env,
	io,
	path,
	sync::Arc,
	thread,
	time::Duration,
};

use actix_web::{
	dev::Service,
	rt,
	web::Data,
	App,
	HttpServer,
};
use dotenv::dotenv;
use futures::{
	executor::block_on,
	future::{
		self,
		abortable,
		join,
		ok,
		Future,
	},
	FutureExt,
};
use to_unit::ToUnit;
use tokio::{
	net::unix::SocketAddr,
	signal::ctrl_c,
};
use tonic::transport;

use crate::{
	config::{
		app,
		database::DatabasePool,
		microservice::GrpcPool,
	},
	grpc::controller,
};

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

	let env_path: path::PathBuf = env::current_dir()
		.and_then(|a| Ok(a.join("config").join("node_info").join(format!("node{}.env", node))))
		.unwrap();
	dotenv::from_path(env_path.as_path()).ok();

	let grpc_server_thread = tokio::spawn(async move {
		let host: String = dotenv::var("HOST").unwrap();
		let grpc_port: String = dotenv::var("GRPC_PORT").unwrap();

		block_on(async move {
			let database_pool: Arc<DatabasePool> = Arc::new(DatabasePool::new().await);
			let database_data: Data<Arc<DatabasePool>> = Data::new(database_pool);

			println!("starting grpc server at {}:{}", host, grpc_port);

			transport::Server::builder()
				.add_service(controller::P2PController::new(database_data).await)
				.serve(format!("{}:{}", host, grpc_port).parse().unwrap())
		})
	});

	let http_server_thread = tokio::spawn(async move {
		let host: String = dotenv::var("HOST").unwrap();
		let http_port: String = dotenv::var("HTTP_PORT").unwrap();

		block_on(async move {
			let database_pool: Arc<DatabasePool> = Arc::new(DatabasePool::new().await);
			let database_data: Data<Arc<DatabasePool>> = Data::new(database_pool);

			let grpc_pool: Arc<GrpcPool> = Arc::new(GrpcPool::new().await);
			let grpc_data: Data<Arc<GrpcPool>> = Data::new(grpc_pool);

			HttpServer::new(move || {
				App::new()
					.configure(app::config_services)
					.app_data(grpc_data.clone())
					.app_data(database_data.clone())
					.wrap_fn(|req, srv| srv.call(req).map(|res| res))
			})
			.bind(format!("{}:{}", host, http_port))
			.unwrap()
			.run()
			.await
		})
	});

	let _ = join(grpc_server_thread, http_server_thread).await;

	Ok(())
}
