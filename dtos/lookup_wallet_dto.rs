use serde::{
	Deserialize,
	Serialize,
};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct LookupWalletDto {
	#[validate(length(min = 1))]
	pub owner: String,
}
