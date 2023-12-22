use actix_web::{
    http::StatusCode,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

pub struct Error {
    status: StatusCode,
    message: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    code: u16,
    name: &'static str,
    message: &'static str,
}

impl Error {
    pub fn get_body(&self) -> Body {
        Body {
            code: self.status.as_u16(),
            name: self.status.canonical_reason().unwrap(),
            message: self.message,
        }
    }

    pub fn get_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(self.get_body())
    }
}


pub const COMMITMENT_NOT_FOUND: Error = Error {
    status: StatusCode::NOT_FOUND,
    message: "The Commitment is not found"
};
pub const COMMITMENT_EXISTED: Error = Error {
    status: StatusCode::CONFLICT,
    message: "The Commitment already existed"
};
pub const SHARED_KEY_NOT_FOUND: Error = Error {
    status: StatusCode::NOT_FOUND,
    message: "The Shared Key is not found"
};
pub const SHARED_KEY_EXISTED: Error = Error {
    status: StatusCode::CONFLICT,
    message: "The Shared Key already existed"
};
pub const VERIFIER_NOT_SUPPORT: Error = Error {
    status: StatusCode::BAD_REQUEST,
    message: "This Verifier is not supported"
};
pub const INVALID_NODE_SIGNATURE: Error = Error {
    status: StatusCode::BAD_REQUEST,
    message: "The Node Signature is invalid"
};
pub const WALLET_NOT_FOUND: Error = Error {
    status: StatusCode::NOT_FOUND,
    message: "The Wallet is not found"
};