use actix_web::{
	get,
	web,
	HttpResponse,
};

use crate::controllers::*;

#[get("/")]
async fn ping() -> HttpResponse {
	for i in 0..100000 {
		println!("Hello {}", i);
	}
	HttpResponse::Ok().body("Pong")
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
	cfg.service(ping).service(
		web::scope("/api")
			.service(ping)
			.service(commitment_controller::create_commitment)
			.service(commitment_controller::get_commitment)
			.service(key_assignment_controller::broadcast_all)
			.service(shared_key_controller::lookup_shared_secret)
			.service(wallet_controller::lookup_wallet)
			.service(wallet_controller::get_wallet),
	);
}
