use sha2::digest::consts::U8;
use crate::account::{PrivateKey};
use crate::Address;


#[flutter_rust_bridge::frb(sync)]
pub fn private_key_new() -> String { return PrivateKey::new().to_string(); }

#[flutter_rust_bridge::frb(sync)]
pub fn from_seed_unchecked(seed: Vec<u8>) -> String {
    // 16 进制转 u8 的代码， 这里直接传递 u8，所以不需要
    // let bytes = hex::decode(seed).unwrap();
    // let bytes_slice: &[u8] = &bytes;

    return PrivateKey::from_seed_unchecked(&seed).to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_address(private_key: String) -> String {
    return PrivateKey::from_string(&private_key).unwrap().to_address().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_view_key(pk: String) -> String {
    return PrivateKey::from_string(&pk).unwrap().to_view_key().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn sign(message_hex: Vec<u8>, private_key: String) -> String {
    let pk = PrivateKey::from_string(&private_key).unwrap();
    return pk.sign(&message_hex).to_string();
}





