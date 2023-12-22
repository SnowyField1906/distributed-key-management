use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct LookupWallet {
    #[validate(length(min = 1))]
    owner: String,
    address: String,
    public_key: String,
}