use actix_web::{
    web,
    HttpResponse,
    post,
};
use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
};
use ecies::encrypt;
use crate::{
    dtos::{
        lookup_shared_secret_dto::LookupSharedSecretDto,
        node_shared_secret_dto::NodeSharedSecretDto,
    },
    services::{
        shared_key_service,
        commitment_service,
    },
    common::{
        messages,
        crypto
    },
};

#[post("shared-key")]
async fn lookup_shared_secret(data: web::Json<LookupSharedSecretDto>) -> HttpResponse {
    let data: LookupSharedSecretDto = data.into_inner();
    
    let token_id: &mut Vec<u8> = &mut hex::decode(data.token_id.clone()).unwrap();
    
    crypto::create_keccak256(token_id);

    if commitment_service::find(&hex::encode(token_id)).await.is_err() {
        return messages::COMMITMENT_NOT_FOUND.get_response();
    }

    // TODO: find wallet

    let secp: Secp256k1<All> = Secp256k1::new();
    let raw_priv_key: String = dotenv::var("PRIVATE_KEY").unwrap();
    let priv_key: SecretKey = SecretKey::from_slice(&hex::decode(raw_priv_key).unwrap()[..]).unwrap();
    let pub_key: PublicKey = priv_key.public_key(&secp);

    if data.node_signatures.iter().find(|&node| node.pub_key == hex::encode(pub_key.serialize())).is_none() {
        return messages::INVALID_NODE_SIGNATURE.get_response();
    }

    match shared_key_service::find_by_owner(&data.owner).await {
        Ok(shared_key) => {
            let shared_secret: String = shared_key.shared_secret.unwrap();

            let data: Vec<u8> = encrypt( &pub_key.serialize(), shared_secret.as_bytes()).unwrap();

            HttpResponse::Ok().json(NodeSharedSecretDto {
                threshold: 1,
                share: hex::encode(data),
                pub_key: hex::encode(pub_key.serialize()),
            })
        },
        Err(_) => return messages::SHARED_KEY_NOT_FOUND.get_response(),
    }
}