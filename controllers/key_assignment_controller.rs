use actix_web::{
    web,
    HttpResponse,
    post,
};
use crate::{
    common::messages,
    grpc::service,
    dtos::assign_key_dto::AssignKeyDto,
};

#[post("key-assignment")]
async fn broadcast_all(data: web::Json<AssignKeyDto>) -> HttpResponse {
    let data: AssignKeyDto = data.into_inner();
    
    match service::broadcast_all().await {
        Ok(_) => {
            messages::OK.get_response()
        },
        Err(error) => {
            return error.get_response();
        }
    }
}