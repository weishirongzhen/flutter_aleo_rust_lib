pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */


pub mod account;

pub use account::*;


pub mod programs;

pub use programs::*;

pub mod types;

pub use types::Field;

pub mod record;

pub use record::*;

use types::native;
use std::str::FromStr;

use types::native::RecordPlaintextNative;

pub fn log(s: &str) {
    println!("wtf {}", s);
}

pub trait Credits {
    /// Get the amount of credits in the record if the record possesses Aleo credits
    fn credits(&self) -> Result<f64, String> {
        Ok(self.microcredits()? as f64 / 1_000_000.0)
    }

    /// Get the amount of microcredits in the record if the record possesses Aleo credits
    fn microcredits(&self) -> Result<u64, String>;
}

impl Credits for RecordPlaintextNative {
    fn microcredits(&self) -> Result<u64, String> {
        match self
            .find(&[native::IdentifierNative::from_str("microcredits").map_err(|e| e.to_string())?])
            .map_err(|e| e.to_string())?
        {
            native::Entry::Private(native::PlaintextNative::Literal(native::LiteralNative::U64(amount), _)) => {
                Ok(*amount)
            }
            _ => Err("The record provided does not contain a microcredits field".to_string()),
        }
    }
}

mod test {


    // lesson maid remove boring swift floor produce crouch kangaroo action kick pole
    // root seed: 5bb185ddba9203bc4af35877ee5545b2247c633031dc8df733c72cd2ea5b0011ba5b14371916305d7c20e845a6286842a645475de84c8332f90c5e3db6eafebc
    // index 0 seed: 6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd --  private key: APrivateKey1zkp8a8td4dUmwEh4uiejD2yZdj4a6Ttp7uEiwXPRazfANeJ
    // index 1 seed: eee3c5c60eb4bbdbc61340e79047ca412811cbc7d5058c93d0bab6ca95a3ab42 --  private key: APrivateKey1zkpHj3e8S4AGVvCjFheuEHPz6ukSbrnMwa8yTJXmRhK8V9K

    use std::result;
    use std::sync::{Arc, RwLock};
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use snarkvm_synthesizer::Trace;
    use crate::{Address, PrivateKey, ProgramManager};
    use crate::types::native::CurrentAleo;

    #[test]
    fn private_key_from_seed() {
        let seed = "6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd".to_string();

        let bytes = hex::decode(seed).unwrap();

        let pk = PrivateKey::from_seed_unchecked(&bytes);

        println!("private key = {}", pk.to_string())
    }

    #[tokio::test]
    async fn transfer() {
        let seed = "6ee24c8b8a66957256b6ff2959d7a882a7791df6fb9049427e670dc7fb6e42dd".to_string();

        let bytes = hex::decode(seed).unwrap();

        let pk = PrivateKey::from_seed_unchecked(&bytes);

        let result = ProgramManager::transfer(
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
        ).await;

        println!("result {:?}", result.unwrap());
    }

    #[test]
    fn transfer_delegate() {
        let seed = "6f2f42f8777c6d333bd72e67800fa956f812d7ada2457953f9811d078d6c6e62".to_string();

        let bytes = hex::decode(seed).unwrap();

        let pk = PrivateKey::from_seed_unchecked(&bytes);

        println!("address = {}", Address::from_private_key(&pk).to_string());
        let result = ProgramManager::delegate_transfer_public(
            &pk,
            1000f64,
            "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x",
            "public",
            None,
            0.001,
            None,
        );

        println!("authorization {:?}", result.clone().unwrap()[0]);
        println!("program {:?}", result.clone().unwrap()[1]);
        println!("fee authorization {:?}", result.clone().unwrap()[2]);
    }

}