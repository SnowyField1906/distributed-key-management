use std::sync::Arc; // Add this line to import the Arc type

use actix_web::web::Data;
use mongodb::{
	bson::doc,
	Collection,
};

use crate::{
	common::messages,
	config::database::DatabasePool,
	dtos::create_commitment_dto::CreateCommitmentDto,
	schemas::commitment_schema::Commitment,
};

pub async fn create(
	database_pool: &Data<Arc<DatabasePool>>, data: CreateCommitmentDto,
) -> Result<(), messages::Error> {
	let this: Collection<Commitment> = database_pool.get_collection_mut("shared_keys").await;

	let new_commitment: Commitment = Commitment {
		id: None,
		commitment: data.commitment,
		temp_pub: Some(data.temp_pub),
	};

	match this.insert_one(new_commitment, None).await {
		Ok(res) => {
			print!("Created commitment: {:?}", res.inserted_id);
			Ok(())
		}
		_ => Err(messages::COMMITMENT_EXISTED),
	}
}

pub async fn find(
	database_pool: &Data<Arc<DatabasePool>>, commitment: &str,
) -> Result<Commitment, messages::Error> {
	let this: Collection<Commitment> = database_pool.get_collection("shared_keys").await;

	match this.find_one(doc! { "commitment": commitment }, None).await {
		Ok(Some(res)) => Ok(res),
		_ => Err(messages::COMMITMENT_NOT_FOUND),
	}
}
