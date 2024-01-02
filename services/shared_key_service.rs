use mongodb::{
    bson::doc,
    Collection,
};
use secp256k1::{
    SecretKey,
    PublicKey,
    constants::CURVE_ORDER,
};
use num_bigint::BigUint;
use crate::{
    common::{
        messages,
        crypto,
    },
    config::db::collection,
    schemas::shared_key_schema::SharedKey,
};

async fn this() -> Collection<SharedKey> {
    collection("shared_keys").await
}

pub async fn create(owner: &str) -> Result<PublicKey, messages::Error> {
    let this: Collection<SharedKey> = this().await;

    let (priv_key, pub_key): (SecretKey, PublicKey) = crypto::generate_keypair();

    let new_shared_key: SharedKey = SharedKey {
        id: None,
        secret: crypto::priv_key_to_str(&priv_key),
        owner: owner.to_string(),
        received_shares: vec![],
        shared_secret: None,
    };

    match this.insert_one(new_shared_key, None).await {
        Ok(_) => Ok(pub_key),
        _ => Err(messages::SHARED_KEY_EXISTED),
    }
}

pub async fn find_by_owner(owner: &str) -> Result<SharedKey, messages::Error> {
    let this: Collection<SharedKey> = this().await;

    match this.find_one(
        doc! { "owner": owner },
        None
    ).await {
        Ok(res) => match res {
            Some(shared_key) => Ok(shared_key),
            None => Err(messages::COMMITMENT_NOT_FOUND),
        },
        _ => Err(messages::SHARED_KEY_NOT_FOUND),
    }
}

pub async fn add_received_share(
    owner: &str,
    received_share: &str,
) -> Result<(), messages::Error> {
    let this: Collection<SharedKey> = this().await;

    match this.update_one(
        doc! { "owner": owner }, 
        doc! { "$push": { "received_shares": received_share } },
        None
    ).await {
        Ok(_) => Ok(()),
        _ => Err(messages::SHARED_KEY_NOT_FOUND),
    }
}

pub async fn derive_shared_secret(owner: &str) -> Result<(), messages::Error> {
    let this: Collection<SharedKey> = this().await;

    let shared_key: SharedKey = find_by_owner(owner).await?;
    
    let mut shared_secret: BigUint = BigUint::default();
    let received_shares: Vec<BigUint> = shared_key.received_shares.iter().map(|received_share| {
        BigUint::from_bytes_le(&hex::decode(received_share).unwrap()[..])
    }).collect();

    let n_secp256k1: BigUint = BigUint::from_bytes_be(&CURVE_ORDER);

    for i in 0..received_shares.len() {
        let current = received_shares.get(i).unwrap();
        shared_secret = (shared_secret + current) % &n_secp256k1;
    }

    match this.update_one(
        doc! { "owner": owner },
        doc! { "$set": { "shared_secret": shared_secret.to_str_radix(16) } },
        None
    ).await {
        Ok(_) => Ok(()),
        _ => Err(messages::SHARED_KEY_NOT_FOUND),
    }
}