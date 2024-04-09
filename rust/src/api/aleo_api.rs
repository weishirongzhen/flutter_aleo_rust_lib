use crate::account::{PrivateKey};
use anyhow;
use crate::aleo_archived_api::*;


// 0.3.1
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
pub fn to_view_key(private_key: String) -> String {
    return PrivateKey::from_string(&private_key).unwrap().to_view_key().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn sign(message_bytes: Vec<u8>, private_key: String) -> String {
    let pk = PrivateKey::from_string(&private_key).unwrap();
    return pk.sign(&message_bytes).to_string();
}


#[flutter_rust_bridge::frb(sync)]
pub fn transfer(recipient: String,
                transfer_type: String,
                amount: f64,
                fee: f64,
                private_fee: bool,
                private_key: String,
                amount_record: Option<String>,
                fee_record: Option<String>,
                endpoint: Option<String>, ) -> String {
    return do_transfer(recipient, transfer_type, amount, fee, private_fee, private_key, amount_record, fee_record, endpoint).unwrap();
}

// #[flutter_rust_bridge::frb(sync)]
// pub fn transfer(private_key: String,
//                 amount_credits: f64,
//                 recipient: String,
//                 transfer_type: String,
//                 amount_record: Option<String>,
//                 fee_credits: f64,
//                 fee_record: Option<String>,
//                 url: Option<String>,
//                 transfer_proving_key: Option<String>,
//                 transfer_verifying_key: Option<String>,
//                 fee_proving_key: Option<String>,
//                 fee_verifying_key: Option<String>,
//                 offline_query: Option<String>,
// ) -> String {
//     // let pk = PrivateKey::from_string(&private_key).unwrap();
//     let pk = PrivateKey::from_string("APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8").unwrap();
//
//     let v = executor::block_on(ProgramManager::transfer(&pk, 0.1, "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x", "public", None, 0.29, None, None, None, None, None, None, None));
//     return v.unwrap().to_string();
//
//     // ProgramManager::transfer(&pk, amount_credits, &recipient, &transfer_type, None, fee_credits, None, url, None, None, None, None, None);
//     // return result.unwrap().to_string();
// }





