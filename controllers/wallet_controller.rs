use actix_web::{
    web,
    HttpResponse,
    post,
    get,
};
use crate::{
    services::wallet_service,
    grpc::service,
    schemas::wallet_schema::Wallet,
};

#[post("wallet/{owner}")]
pub async fn lookup_wallet(owner: web::Path<String>) -> HttpResponse {
    let owner: String = owner.into_inner();

    match wallet_service::find_by_owner(&owner).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(_) => {}
    }

    let new_wallet: Wallet = match service::generate_shared_secret(&owner).await {
        Ok(wallet) => wallet,
        Err(error) => return error.get_response(),
    };

    match wallet_service::create(
        new_wallet.owner,
        new_wallet.pub_key,
        new_wallet.address,
    ).await {
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
pub async fn get_wallet(owner: web::Path<String>) -> HttpResponse {
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