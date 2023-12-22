use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub iv: String,
    pub ephem_pub_key: String,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSharedSecretDto {
    pub threshold: u8,
    pub share: String,
    pub metadata: Metadata,
    pub pub_key: String,
}