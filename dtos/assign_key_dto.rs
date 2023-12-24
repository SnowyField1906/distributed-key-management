use serde::{Deserialize, Serialize};
use crate::common::verifiers::VERIFIERS;
use validator::{ Validate, ValidationError };

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct AssignKey {
    #[validate(email)]
    email: String,
    #[validate(custom = "is_allowed_verifier")]
    verifier: String,
}

fn is_allowed_verifier(val: &str) -> Result<(), ValidationError> {
    if VERIFIERS.contains(&val) {
        Ok(())
    } else {
        Err(ValidationError::new("The Verifier is not allowed"))
    }
}