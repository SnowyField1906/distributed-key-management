use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub owner: String,
    pub pub_key: String,
    pub address: String,
}