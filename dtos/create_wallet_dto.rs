use serde::{
	Deserialize,
	Serialize,
};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize, Clone)]
pub struct CreateWalletDto {
	#[validate(length(min = 1))]
	pub owner: String,
	#[validate(length(min = 1))]
	pub pub_key: String,
	#[validate(length(min = 1))]
	pub address: String,
}
