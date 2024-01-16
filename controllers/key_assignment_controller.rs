use std::sync::Arc;

use actix_web::{
	post,
	web::{
		Data,
		Json,
	},
	HttpResponse,
};

use crate::{
	common::messages,
	config::microservice::GrpcPool,
	grpc::service,
};

#[post("key-assignment")]
pub async fn broadcast_all(grpc_pool: Data<Arc<GrpcPool>>) -> HttpResponse {
	match service::broadcast_all(grpc_pool).await {
		Ok(_) => messages::OK.get_response(),
		Err(error) => {
			return error.get_response();
		}
	}
}
