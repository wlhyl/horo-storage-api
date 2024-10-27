use bcrypt::{hash, DEFAULT_COST};

use crate::error::Error;

pub fn encode(s: &str) -> Result<String, Error> {
    // hash(s, DEFAULT_COST).map_err(|error| error.into())
    Ok(hash(s, DEFAULT_COST)?)
}

pub fn verify(s: &str, hashed: &str) -> Result<bool, Error> {
    // bcrypt::verify(s, hashed).map_err(|error| error.into())
    Ok(bcrypt::verify(s, hashed)?)
}
