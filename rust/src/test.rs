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

    // #[test]
    // fn  test_derive_path(){
    //     // 示例使用，实际使用时应根据需要调整
    //     let root_seed = "f2812b1b381e14d4fa7e0f3545ff3d1570df3cbdd4f5ab5f84f3f6da71989642beab844da5ad56fd1795a3bb55c6319c3228ceac1273706be5946ecc778e5153".to_string();
    //     let path = "m/44'/0'/0'/0'".to_string();
    //     match crate::api::derive_path::derive_path(&path, &root_seed, crate::api::derive_path::HARDENED_OFFSET) {
    //         Ok((seed, chain_code)) => {
    //             println!("Seed: {:x?}", hex::encode(seed));
    //             println!("Chain Code: {:x?}", hex::encode(chain_code));
    //         },
    //         /// 6f2f42f8777c6d333bd72e67800fa956f812d7ada2457953f9811d078d6c6e62
    //         Err(e) => println!("Error: {}", e),
    //     }
    //
    // }
}