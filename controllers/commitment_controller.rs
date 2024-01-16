use actix_web::{
	get,
	post,
	web,
	HttpResponse,
};
use secp256k1::{
	ecdsa::Signature,
	hashes::sha256::Hash,
	All,
	Message,
	PublicKey,
	Secp256k1,
	SecretKey,
};

use crate::{
	common::{
		crypto,
		messages,
	},
	dtos::{
		create_commitment_dto::CreateCommitmentDto,
		node_commitment_dto::NodeCommitmentDto,
	},
	services::commitment_service,
};

#[post("commitment")]
pub async fn create_commitment(data: web::Json<CreateCommitmentDto>) -> HttpResponse {
	let data: CreateCommitmentDto = data.into_inner();

	if commitment_service::find(&data.commitment).await.is_ok() {
		return messages::COMMITMENT_EXISTED.get_response();
	}

	if let Err(err) = commitment_service::create(data.clone()).await {
		return err.get_response();
	}

	let secp: Secp256k1<All> = Secp256k1::new();
	let raw_priv_key: String = dotenv::var("PRIVATE_KEY").unwrap();
	let priv_key: SecretKey =
		SecretKey::from_slice(&hex::decode(raw_priv_key).unwrap()[..]).unwrap();
	let pub_key: PublicKey = priv_key.public_key(&secp);

	let raw_data: String = format!("{},{}", data.commitment, data.temp_pub);
	let data: Message = Message::from_hashed_data::<Hash>(raw_data.as_bytes());
	let signature: Signature = secp.sign_ecdsa(&data, &priv_key);

	HttpResponse::Ok().json(NodeCommitmentDto {
		data: raw_data,
		signature: signature.serialize_der().to_string(),
		pub_key: crypto::pub_key_to_str(&pub_key),
	})
}

#[get("commitment/{commitment}")]
pub async fn get_commitment(commitment: web::Path<String>) -> HttpResponse {
	match commitment_service::find(&commitment.into_inner()).await {
		Ok(result) => HttpResponse::Ok().json(result),
		Err(err) => err.get_response(),
	}
}
