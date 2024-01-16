use actix_web::web::Data;
use mongodb::{
	bson::doc,
	Collection,
};

use crate::{
	common::messages,
	config::database::DatabasePool,
	schemas::wallet_schema::Wallet,
};

pub async fn create(
	database_pool: Data<DatabasePool>, owner: String, pub_key: String, address: String,
) -> Result<Wallet, messages::Error> {
	let this: Collection<Wallet> = database_pool.get_collection_mut("wallet").await;

	let new_wallet: Wallet = Wallet {
		id: None,
		owner,
		pub_key,
		address,
	};

	match this.insert_one(&new_wallet, None).await {
		Ok(_) => Ok(new_wallet),
		_ => Err(messages::WALLET_EXISTED),
	}
}

// pub async fn find_all() -> Result<Vec<Wallet>, messages::Error> {
//     let this: Collection<Wallet> = this().await;

//     match this.find(
//         doc! {},
//         FindOptions::builder().build()
//     ).await {
//         Ok(res) => res.next().await,
//         _ => Err(messages::WALLET_NOT_FOUND),
//     }
// }

pub async fn find_by_owner(
	database_pool: Data<DatabasePool>, owner: &String,
) -> Result<Wallet, messages::Error> {
	let this: Collection<Wallet> = database_pool.get_collection("wallet").await;

	match this.find_one(doc! { "owner": owner }, None).await {
		Ok(res) => {
			match res {
				Some(wallet) => Ok(wallet),
				None => Err(messages::WALLET_NOT_FOUND),
			}
		}
		_ => Err(messages::WALLET_NOT_FOUND),
	}
}

pub async fn find_by_address(
	database_pool: Data<DatabasePool>, address: &String,
) -> Result<Wallet, messages::Error> {
	let this: Collection<Wallet> = database_pool.get_collection("wallet").await;

	match this.find_one(doc! { "address": address }, None).await {
		Ok(res) => {
			match res {
				Some(wallet) => Ok(wallet),
				None => Err(messages::WALLET_NOT_FOUND),
			}
		}
		_ => Err(messages::WALLET_NOT_FOUND),
	}
}

pub async fn drop_all_by_owner(
	database_pool: Data<DatabasePool>, owner: String,
) -> Result<(), messages::Error> {
	let this: Collection<Wallet> = database_pool.get_collection_mut("wallet").await;

	match this.delete_many(doc! { "owner": owner }, None).await {
		Ok(_) => Ok(()),
		_ => Err(messages::WALLET_NOT_FOUND),
	}
}
