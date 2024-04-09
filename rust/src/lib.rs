pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */


pub mod account;

pub use account::*;

pub mod types;

pub use types::Field;

pub mod record;
pub use record::*;

use types::native;
use std::str::FromStr;

use types::native::RecordPlaintextNative;

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