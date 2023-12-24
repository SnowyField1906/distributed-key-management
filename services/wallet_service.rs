use mongodb::{
    bson::doc,
    Collection,
};
use crate::{
    common::messages,
    config::db::collection,
    dtos::create_wallet_dto::CreateWalletDto,
    schemas::wallet_schema::Wallet,
};

async fn this() -> Collection<Wallet> {
    collection("wallet").await
}

async fn create(data: CreateWalletDto) -> Result<(), messages::Error> {
    let this: Collection<Wallet> = this().await;

    let new_wallet: Wallet = Wallet {
        id: None,
        owner: data.owner,
        pub_key: data.pub_key,
        address: data.address,
    };

    match this.insert_one(new_wallet, None).await {
        Ok(res) => {
            print!("Created wallet: {:?}", res.inserted_id);
            Ok(())
        },
        _ => Err(messages::WALLET_EXISTED),
    }
}

async fn find_all() -> Result<Vec<Wallet>, messages::Error> {
    let this: Collection<Wallet> = this().await;

    match this.find(doc! {}, None).await {
        Ok(res) => Ok(res),
        _ => Err(messages::WALLET_NOT_FOUND),
    }
}

async fn find_by_owner(owner: String) -> Result<Wallet, messages::Error> {
    let this: Collection<Wallet> = this().await;

    match this.find_one(doc! { "owner": owner }, None).await {
        Ok(res) => Ok(res),
        _ => Err(messages::WALLET_NOT_FOUND),
    }
}

async fn find_by_address(address: String) -> Result<Wallet, messages::Error> {
    let this: Collection<Wallet> = this().await;

    match this.find_one(doc! { "address": address }, None).await {
        Ok(res) => Ok(res),
        _ => Err(messages::WALLET_NOT_FOUND),
    }
}

async fn drop_all_by_owner(owner: String) -> Result<(), messages::Error> {
    let this: Collection<Wallet> = this().await;

    match this.delete_many(doc! { "owner": owner }, None).await {
        Ok(_) => Ok(()),
        _ => Err(messages::WALLET_NOT_FOUND),
    }
}