use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SharedKey {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub secret: String,
    pub owner: String,
    pub received_shares: Vec<String>,
    pub shared_secret: Option<String>,
}
