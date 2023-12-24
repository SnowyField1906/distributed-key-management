use serde::{Deserialize, Serialize};
use crate::dtos::node_commitment_dto::NodeCommitmentDto;
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LookupSharedSecretDto {
    #[validate(length(min = 1))]
    pub owner: String,
    #[validate(length(min = 1))]
    pub token_id: String,
    #[validate(length(min = 1))]
    pub temp_pub_key: String,
    pub node_signatures: Vec<NodeCommitmentDto>,
}