use serde::{
	Deserialize,
	Serialize,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSharedSecretDto {
	pub threshold: u8,
	pub share: String,
	pub pub_key: String,
}
