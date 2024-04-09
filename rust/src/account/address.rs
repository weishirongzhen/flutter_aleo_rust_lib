use crate::account::{PrivateKey, Signature, ViewKey};

use crate::types::native::AddressNative;
use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};

/// Public address of an Aleo account
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address(AddressNative);

impl Address {
    /// Derive an Aleo address from a private key
    ///
    /// @param {PrivateKey} private_key The private key to derive the address from
    /// @returns {Address} Address corresponding to the private key
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        Self(AddressNative::try_from(**private_key).unwrap())
    }

    /// Derive an Aleo address from a view key
    ///
    /// @param {ViewKey} view_key The view key to derive the address from
    /// @returns {Address} Address corresponding to the view key
    pub fn from_view_key(view_key: &ViewKey) -> Self {
        Self(AddressNative::try_from(**view_key).unwrap())
    }

    /// Create an aleo address object from a string representation of an address
    ///
    /// @param {string} address String representation of an addressm
    /// @returns {Address} Address
    pub fn from_string(address: &str) -> Self {
        Self::from_str(address).unwrap()
    }

    /// Get a string representation of an Aleo address object
    ///
    /// @param {Address} Address
    /// @returns {string} String representation of the address
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Verify a signature for a message signed by the address
    ///
    /// @param {Uint8Array} Byte array representing a message signed by the address
    /// @returns {boolean} Boolean representing whether or not the signature is valid
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        signature.verify(self, message)
    }
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Ok(Self(AddressNative::from_str(address)?))
    }
}

impl From<AddressNative> for Address {
    fn from(value: AddressNative) -> Self {
        Self(value)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Address {
    type Target = AddressNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::PrivateKey;


    const ITERATIONS: u64 = 1_000;

    #[test]
    pub fn test_from_private_key() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = PrivateKey::new();
            let expected = Address::from_private_key(&private_key);

            // Check the address derived from the view key.
            let view_key = private_key.to_view_key();
            assert_eq!(expected, Address::from_view_key(&view_key));
        }
    }
}
