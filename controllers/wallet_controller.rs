use std::sync::Arc;

use actix_web::{
	get,
	post,
	web::{
		Data,
		Path,
	},
	HttpResponse,
};
use mongodb::Database;

use crate::{
	config::microservice::GrpcPool,
	grpc::service,
	schemas::wallet_schema::Wallet,
	services::wallet_service,
};

#[post("wallet/{owner}")]
pub async fn lookup_wallet(
	grpc_pool: Data<Arc<GrpcPool>>, database_pool: Data<Database>, owner: Path<String>,
) -> HttpResponse {
	let owner: String = owner.into_inner();

	match wallet_service::find_by_owner(database_pool, &owner).await {
		Ok(wallet) => return HttpResponse::Ok().json(wallet),
		Err(_) => {}
	}

	let new_wallet: Wallet = match service::generate_shared_secret(grpc_pool, &owner).await {
		Ok(wallet) => wallet,
		Err(error) => return error.get_response(),
	};

	match wallet_service::create(
		database_pool,
		new_wallet.owner,
		new_wallet.pub_key,
		new_wallet.address,
	)
	.await
	{
		Ok(wallet) => HttpResponse::Ok().json(wallet),
		Err(error) => error.get_response(),
	}
}

// #[get("wallet")]
// pub async fn get_all_wallets() -> HttpResponse {
//     match wallet_service::find_all().await {
//         Ok(wallets) => HttpResponse::Ok().json(wallets),
//         Err(error) => error.get_response(),
//     }
// }

#[get("wallet/{owner}")]
pub async fn get_wallet(owner: Path<String>) -> HttpResponse {
	let owner: String = owner.into_inner();

	match wallet_service::find_by_owner(&owner).await {
		Ok(wallet) => return HttpResponse::Ok().json(wallet),
		Err(_) => {}
	}

	match wallet_service::find_by_address(&owner).await {
		Ok(wallet) => return HttpResponse::Ok().json(wallet),
		Err(error) => error.get_response(),
	}
}
