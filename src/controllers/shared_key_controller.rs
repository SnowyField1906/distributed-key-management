use actix_web::{
    web,
    HttpResponse,
    post,
    get,
};
use crate::{
    dtos::{
        lookup_shared_secret_dto::LookupSharedSecretDto,
        node_shared_secret_dto::{NodeSharedSecretDto, Metadata},
    },
    services::{
        shared_key_service,
        commitment_service,
    },
    common::messages,
};
use secp256k1::{
    Secp256k1,
    SecretKey,
    PublicKey,
    All,
    Message,
    hashes::sha256::Hash,
};
use keccak_hash::keccak256;
use ecies::{
    encrypt,
    decrypt,
};

#[post("shared-key")]
async fn lookup_shared_secret(
    data: web::Json<LookupSharedSecretDto>,
) -> HttpResponse {
    let data: LookupSharedSecretDto = data.into_inner();
    
    let token_id: Vec<u8> = hex::decode(data.token_id.clone()).unwrap();
    keccak_hash::keccak256(&mut token_id);

    if commitment_service::find_commitment(&hex::encode(token_id)).await.is_err() {
        return messages::COMMITMENT_NOT_FOUND.get_response();
    }

    // TODO: find wallet

    let secp: Secp256k1<All> = Secp256k1::new();
    let raw_priv_key: String = dotenv::var("PRIVATE_KEY").unwrap();
    let priv_key: SecretKey = SecretKey::from_slice(&hex::decode(raw_priv_key).unwrap()[..]).unwrap();
    let pub_key: PublicKey = priv_key.public_key(&secp);

    if data.node_signatures.iter().find(|&&node| node.pub_key == hex::encode(pub_key.serialize())).is_none() {
        return messages::INVALID_NODE_SIGNATURE.get_response();
    }

    let shared_key = shared_key_service::find_shared_key_by_owner(&data.owner).await;

    match shared_key_service::find_shared_key_by_owner(&data.owner).await {
        Ok(shared_key) => {
            let shared_secret: String = shared_key.shared_secret.unwrap();

            let encrypted = encrypt( &pub_key.serialize(), shared_secret.as_bytes()).unwrap();

            HttpResponse::Ok().json(NodeSharedSecretDto {
                threshold: 1,
                share: encrypted.encrypt_to_base64(&data.token_id),
                metadata: Metadata {
                    iv: encrypted.get_iv().unwrap(),
                    ephem_pub_key: encrypted.get_ephemeral_public_key().unwrap(),
                    mac: encrypted.get_mac().unwrap(),
                },
                pub_key: hex::encode(pub_key.serialize()),
            })
        },
        Err(_) => return messages::SHARED_KEY_NOT_FOUND.get_response(),
    }
}