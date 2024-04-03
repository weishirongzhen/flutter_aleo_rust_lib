use crate::account::{PrivateKey};


#[flutter_rust_bridge::frb(sync)]
pub fn private_key_new() -> String { return PrivateKey::new().to_string(); }

#[flutter_rust_bridge::frb(sync)]
pub fn from_seed_unchecked(seed: String) -> String{

    let bytes = hex::decode(seed).unwrap();
    let bytes_slice: &[u8] = &bytes;

    return PrivateKey::from_seed_unchecked(bytes_slice).to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_address(pk: String) -> String {
   return PrivateKey::from_string(&pk).unwrap().to_address().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_view_key(pk: String) -> String {
    return PrivateKey::from_string(&pk).unwrap().to_view_key().to_string()
}

