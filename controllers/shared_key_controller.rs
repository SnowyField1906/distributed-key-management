use actix_web::{
	post,
	web,
	HttpResponse,
};
use ecies::encrypt;
use secp256k1::{
	All,
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
		lookup_shared_secret_dto::LookupSharedSecretDto,
		node_shared_secret_dto::NodeSharedSecretDto,
	},
	services::{
		commitment_service,
		shared_key_service,
		wallet_service,
	},
};

#[post("shared-key")]
pub async fn lookup_shared_secret(data: web::Json<LookupSharedSecretDto>) -> HttpResponse {
	let data: LookupSharedSecretDto = data.into_inner();

	let token_id: &mut Vec<u8> = &mut hex::decode(data.token_id.clone()).unwrap();

	crypto::create_keccak256(token_id);

	match commitment_service::find(&hex::encode(token_id)).await {
		Ok(_) => {}
		Err(error) => return error.get_response(),
	}

	match wallet_service::find_by_owner(&data.owner).await {
		Ok(_) => {}
		Err(error) => return error.get_response(),
	}

	let secp: Secp256k1<All> = Secp256k1::new();
	let raw_priv_key: String = dotenv::var("PRIVATE_KEY").unwrap();
	let priv_key: SecretKey =
		SecretKey::from_slice(&hex::decode(raw_priv_key).unwrap()[..]).unwrap();
	let pub_key: PublicKey = priv_key.public_key(&secp);

	if data
		.node_signatures
		.iter()
		.find(|&node| node.pub_key == hex::encode(pub_key.serialize()))
		.is_none()
	{
		return messages::INVALID_NODE_SIGNATURE.get_response();
	}

	match shared_key_service::find_by_owner(&data.owner).await {
		Ok(shared_key) => {
			let shared_secret: String = shared_key.shared_secret.unwrap();

			let data: Vec<u8> = encrypt(&pub_key.serialize(), shared_secret.as_bytes()).unwrap();

			HttpResponse::Ok().json(NodeSharedSecretDto {
				threshold: 1,
				share: hex::encode(data),
				pub_key: hex::encode(pub_key.serialize()),
			})
		}
		Err(_) => return messages::SHARED_KEY_NOT_FOUND.get_response(),
	}
}
