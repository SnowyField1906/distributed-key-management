use mongodb::bson::{
	doc,
	oid::ObjectId,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct KeyAssignment {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub index: String,
	pub address: String,
	pub public_key: String,
	pub public_key_x: String,
	pub public_key_y: String,
	pub owner: String,
}
