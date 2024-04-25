// use sha2::digest::consts::U8;
use crate::account::{PrivateKey};
// use crate::{Address, OfflineQuery, ProgramManager, ProvingKey, RecordPlaintext, VerifyingKey};
use anyhow;
use futures::executor;
use crate::{Address, ProgramManager};

// 0.3.1
#[flutter_rust_bridge::frb(sync)]
pub fn private_key_new() -> String { return PrivateKey::new().to_string(); }

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_from_seed(seed: Vec<u8>) -> String {
    return PrivateKey::from_seed_unchecked(&seed).to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_to_view_key(private_key: String) -> String {
    return PrivateKey::from_string(&private_key).unwrap().to_view_key().to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn private_key_to_address(private_key: String) -> String {
    return PrivateKey::from_string(&private_key).unwrap().to_address().to_string();
}


#[flutter_rust_bridge::frb(sync)]
pub fn sign_message(message_bytes: Vec<u8>, private_key: String) -> String {
    let pk = PrivateKey::from_string(&private_key).unwrap();
    return pk.sign(&message_bytes).to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_public_transfer_delegate(private_key: String, recipient: String, amount_credits: f64, fee_credits: f64) -> Vec<String> {
    let pk = PrivateKey::from_string(&private_key).unwrap();

    let result = ProgramManager::delegate_transfer_public(
        &pk,
        amount_credits,
        &recipient,
        "public",
        None,
        fee_credits,
        None,
    );
    return result.unwrap();
}


pub fn build_transaction() -> String {
    let seed = "6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd".to_string();

    let bytes = hex::decode(seed).unwrap();

    let pk = PrivateKey::from_seed_unchecked(&bytes);

    let transaction = executor::block_on(ProgramManager::transfer(
        &pk,
        10000f64,
        "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x",
        "public",
        None,
        0.1,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));

    return transaction.unwrap().to_string();
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





