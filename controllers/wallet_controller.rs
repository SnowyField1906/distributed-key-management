use actix_web::{
    web,
    HttpResponse,
    post,
};
use crate::{
    services::wallet_service,
    dtos::lookup_wallet_dto::LookupWalletDto,
};

#[post("wallet")]
async fn lookup_wallet(data: web::Json<LookupWalletDto>) -> HttpResponse {
    let data: LookupWalletDto = data.into_inner();

    match wallet_service::find_by_owner(data.owner.clone()).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(_) => {}
    }

    match wallet_service::create(data.owner, data.pub_key, data.address).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(error) => {
            return error.get_response();
        }
    }
}

#[post("wallet/{owner}")]
async fn get_wallet(data: web::Path<String>) -> HttpResponse {
    let owner: String = data.into_inner();

    match wallet_service::find_by_owner(owner).await {
        Ok(wallet) => return HttpResponse::Ok().json(wallet),
        Err(error) => {
            return error.get_response();
        }
    }
}