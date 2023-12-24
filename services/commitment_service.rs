use mongodb::{
    bson::doc,
    Collection,
};
use crate::{
    common::messages,
    config::db::collection,
    dtos::create_commitment_dto::CreateCommitmentDto,
    schemas::commitment_schema::Commitment,
};

async fn this() -> Collection<Commitment> {
    collection("commitments").await
}

pub async fn create(data: CreateCommitmentDto) -> Result<(), messages::Error> {
    let this: Collection<Commitment> = this().await;

    let new_commitment: Commitment = Commitment {
        id: None,
        commitment: data.commitment,
        temp_pub: Some(data.temp_pub),
    };

    match this.insert_one(new_commitment, None).await {
        Ok(res) => {
            print!("Created commitment: {:?}", res.inserted_id);
            Ok(())
        },
        _ => Err(messages::COMMITMENT_EXISTED),
    }
}

pub async fn find(commitment: &str) -> Result<Commitment, messages::Error> {
    let this: Collection<Commitment> = this().await;

    match this.find_one(
        doc! { "commitment": commitment },
        None
    ).await {
        Ok(Some(res)) => Ok(res),
        _ => Err(messages::COMMITMENT_NOT_FOUND),
    }
}