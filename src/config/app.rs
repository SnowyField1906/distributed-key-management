#[allow(unused_imports)]

use actix_web::{ web, get, HttpResponse };
use log::info;
use crate::controllers::*;

#[get("/")]
async fn ping() -> HttpResponse {
    info!("Ping");
    HttpResponse::Ok().body("Pong")
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(ping).service(
        web::scope("/api")
            .service(ping)
            .service(commitment_controller::create_commitment)
            .service(commitment_controller::get_commitment)
    );
}
