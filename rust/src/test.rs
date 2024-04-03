#[cfg(test)]
mod tests {
    use crate::PrivateKey;

    fn hex_to_bytes32(hex: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex)?;
        let bytes_array = bytes.try_into().map_err(|_| "Failed to convert to a 32-byte array")?;
        Ok(bytes_array)
    }

    #[test]
    fn test_private_key_from_seed()-> Result<(), Box<dyn std::error::Error>>{
        let hex_str = "4fc075d5a84a474b766d6222fc39f3200d2e1f1d13b444a50db885643890c171";
        let bytes = hex::decode(hex_str)?;
        let bytes_slice: &[u8] = &bytes;

        let pk = PrivateKey::from_seed_unchecked(
            bytes_slice
        );
        println!("address              {}", pk.to_string());
        Ok(())
    }
}