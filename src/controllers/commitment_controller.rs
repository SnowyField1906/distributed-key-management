use actix_web::{
    web,
    HttpResponse,
    post,
    get,
};
use crate::{
    dtos::{
        create_commitment_dto::CreateCommitmentDto,
        node_commitment_dto::NodeCommitmentDto,
    },
    services::commitment_service,
    common::messages,
};
use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
    Message,
    ecdsa::Signature,
    hashes::sha256::Hash,
};

#[post("commitment")]
pub async fn create_commitment(
    data: web::Json<CreateCommitmentDto>,
) -> HttpResponse {
    let data: CreateCommitmentDto = data.into_inner();

    if commitment_service::find_commitment(&data.commitment).await.is_ok() {
        return messages::COMMITMENT_EXISTED.get_response();
    }

    if let Err(err) = commitment_service::create(data.clone()).await {
        return err.get_response();
    }
    
    let secp: Secp256k1<All> = Secp256k1::new();
    let raw_priv_key: String = dotenv::var("PRIVATE_KEY").unwrap();
    let priv_key: SecretKey = SecretKey::from_slice(&hex::decode(raw_priv_key).unwrap()[..]).unwrap();
    let pub_key: PublicKey = priv_key.public_key(&secp);

    let raw_data: String = format!("{},{}", data.commitment, data.temp_pub);
    let data: Message = Message::from_hashed_data::<Hash>(raw_data.as_bytes());
    let signature: Signature = secp.sign_ecdsa(&data, &priv_key);

    HttpResponse::Ok().json(NodeCommitmentDto {
        data: raw_data,
        signature: signature.serialize_der().to_string(),
        pub_key: hex::encode(pub_key.serialize()),
    })
}

#[get("commitment/{commitment}")]
pub async fn get_commitment(
    commitment: web::Path<String>,
) -> HttpResponse {
    match commitment_service::find_commitment(&commitment.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => err.get_response(),
    }
}