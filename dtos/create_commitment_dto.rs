use serde::{
    Deserialize, Serialize
};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize, Clone)]
pub struct CreateCommitmentDto {
    #[validate(length(min = 1))]
    pub commitment: String,
    #[validate(length(min = 1))]
    pub temp_pub: String,
}