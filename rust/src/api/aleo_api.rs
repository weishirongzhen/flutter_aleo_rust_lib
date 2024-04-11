use aleo::commands::AccountNew;
use crate::aleo_archived_api::*;


#[flutter_rust_bridge::frb(sync)]
pub fn private_key_from_seed(seed: Vec<u8>) -> String {
    let pk = AccountNew::private_key_from_seed(&seed);
    return AccountNew::private_key_to_string(pk);
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_view_key(private_key: String) -> String {
    let pk = AccountNew::private_key_from_string(&private_key);
    let view_key = AccountNew::private_key_to_view_key(pk);
    return view_key.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_address(private_key: String) -> String {
    let pk = AccountNew::private_key_from_string(&private_key);
    let view_key = AccountNew::private_key_to_view_key(pk);
    let address = AccountNew::view_key_to_address(view_key);
    return address.to_string();
}


//
// #[flutter_rust_bridge::frb(sync)]
// pub fn sign(message_bytes: Vec<u8>, private_key: String) -> String {
//     let pk = PrivateKey::from_string(&private_key).unwrap();
//     return pk.sign(&message_bytes).to_string();
// }


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

mod test {
    use std::time::SystemTime;
    use aleo::commands::AccountNew;
    use aleo::CurrentNetwork;
    use aleo_rust::PrivateKey;
    use crate::api::aleo_api::*;

    #[test]
    fn account_new_all_fn() {
        let bytes = AccountNew::hex_to_u8_bytes("f4d5f3fcc76853544c434423fc06de9daec8e8f5123127b5ee2743c68a21e41d");
        let pk = private_key_from_seed(bytes);
        let view_key = to_view_key(pk.clone());
        let address = to_address(pk.clone());
        println!("pk = {}", pk.clone());
        println!("view_key = {}", view_key);
        println!("address = {}", address);
    }

    #[test]
    fn trr() {
        let sys_time1 = SystemTime::now();
        println!("开始 {:?}", sys_time1);
        let id = transfer(
            "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x".to_string(),
            "public".to_string(),
            100000000.0,
            0.001,
            false,
            "APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8".to_string(),
            None, None, None,
        );
        let sys_time2 = SystemTime::now();
        let difference = sys_time2.duration_since(sys_time1)
            .expect("Clock may have gone backwards");
        println!("结束{:?}", sys_time2);
        println!("id = {}", id);
        println!("{:?}", difference);
    }
}



