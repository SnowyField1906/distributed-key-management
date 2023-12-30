use actix_web::{
    web,
    HttpResponse,
    post,
    get,
};
use crate::{
    services::wallet_service,
    dtos::lookup_wallet_dto::LookupWalletDto,
};

#[post("wallet")]
pub async fn lookup_wallet(data: web::Json<LookupWalletDto>) -> HttpResponse {
    let data: LookupWalletDto = data.into_inner();

    match wallet_service::find_by_owner(&data.owner).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(_) => {}
    }

    match wallet_service::create(data.owner, data.pub_key, data.address).await {
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
pub async fn get_wallet(data: web::Path<String>) -> HttpResponse {
    let data: String = data.into_inner();

    match wallet_service::find_by_owner(&data).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(_) => {}
    }

    match wallet_service::find_by_address(&data).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(error) => error.get_response(),
    }
}