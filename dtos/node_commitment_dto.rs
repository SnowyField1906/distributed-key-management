use serde::{
    Deserialize, Serialize
};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCommitmentDto {
    pub data: String,
    pub signature: String,
    pub pub_key: String,
}