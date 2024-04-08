pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub mod programs;
pub use programs::*;

pub mod account;
pub use account::*;


pub mod record;
pub use record::*;

pub mod types;
pub use types::Field;

#[cfg(not(test))]
mod thread_pool;
mod test;

use wasm_bindgen::prelude::*;

#[cfg(not(test))]
use thread_pool::ThreadPool;

use std::str::FromStr;

use types::native::RecordPlaintextNative;

// // Facilities for cross-platform logging in both web browsers and nodeJS
// #[wasm_bindgen]
// extern "C" {
//     // Log a &str the console in the browser or console.log in nodejs
//     #[wasm_bindgen(js_namespace = console)]
//     pub fn log(s: &str);
// }
pub fn log(s: &str){}

/// A trait providing convenient methods for accessing the amount of Aleo present in a record
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

#[cfg(not(test))]
#[doc(hidden)]
pub use thread_pool::run_rayon_thread;
use types::native;

#[cfg(not(test))]
#[wasm_bindgen(js_name = "initThreadPool")]
pub async fn init_thread_pool(url: web_sys::Url, num_threads: usize) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    ThreadPool::builder().url(url).num_threads(num_threads).build_global().await?;

    Ok(())
}