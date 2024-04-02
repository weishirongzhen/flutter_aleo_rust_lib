use crate::{PrivateKey};

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_new() -> String {return PrivateKey::new().to_string();}