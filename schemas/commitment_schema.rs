use mongodb::bson::{
	doc,
	oid::ObjectId,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Commitment {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub commitment: String,
	pub temp_pub: Option<String>,
}
