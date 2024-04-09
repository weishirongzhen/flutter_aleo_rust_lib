use std::str::FromStr;
use aleo:: {helpers::TransferTypeArg, CurrentNetwork};
use aleo_rust::{
    Address,
    AleoAPIClient,
    Ciphertext,
    Encryptor,
    Plaintext,
    PrivateKey,
    ProgramManager,
    Record,
    RecordFinder,
    TransferType,
};

use anyhow::{anyhow, ensure, Result};
use colored::*;


pub fn do_transfer(
    recipient: String,
    transfer_type: String,
    amount: f64,
    fee: f64,
    private_fee: bool,
    private_key: String,
    amount_record: Option<String>,
    fee_record: Option<String>,
    endpoint: Option<String>,
) -> Result<String>{


    let transfer_type = TransferTypeArg::Public;



    // Convert transfer amount and fee to microcredits
    let amount_microcredits = (amount * 1000000.0) as u64;
    let fee_credits = fee;
    let fee_microcredits = (fee_credits * 1000000.0) as u64;

    println!(
        "{}",
        format!(
            "Attempting to transfer {} credits to {} with a fee of {} credits...",
            amount, recipient, fee_credits
        )
            .bright_blue()
    );

    // Setup the API client to use configured peer or default to https://api.explorer.aleo.org/v1/testnet3
    let api_client =
        endpoint
        .map_or_else(
            || {
                println!(
                    "Using default peer: {}",
                    "https://api.explorer.aleo.org/v1/testnet3".bright_blue().bold()
                );
                Ok(AleoAPIClient::<CurrentNetwork>::testnet3())
            },
            |peer| AleoAPIClient::<CurrentNetwork>::new(&peer, "testnet3"),
        )
        .map_err(|e| anyhow!("{:?}", e))?;

    // Create the program manager
    let program_manager = ProgramManager::<CurrentNetwork>::new(
        PrivateKey::from_str(&private_key).ok(),
        None,
        Some(api_client.clone()),
        None,
        false,
    )?;

    // Find the input records from the Aleo Network if not provided
    // let private_key =  PrivateKey::from_str(&private_key).unwrap();
    // let record_finder = RecordFinder::new(api_client);
    //
    // let mut fee_nonce = None;

    // let fee_record = if self.private_fee {
    //     let fee_record = if let Some(fee_record) = self.fee_record {
    //         fee_record
    //     } else {
    //         record_finder.find_one_record(&private_key, fee_microcredits, None)?
    //     };
    //     Some(fee_record)
    // } else {
    //     None
    // };

    // let amount_record = match transfer_type {
    //     TransferType::Public => None,
    //     TransferType::PublicToPrivate => None,
    //     _ => {
    //         if let Some(fee_record) = fee_record.as_ref() {
    //             fee_nonce = Some([*fee_record.nonce()]);
    //         };
    //
    //         if let Some(amount_record) = self.amount_record {
    //             Some(amount_record)
    //         } else {
    //             Some(record_finder.find_one_record(
    //                 &private_key,
    //                 amount_microcredits,
    //                 fee_nonce.as_ref().map(|nonces| &nonces[..]),
    //             )?)
    //         }
    //     }
    // };

    // Execute the transfer
    let transfer = program_manager.transfer(
        amount_microcredits,
        fee_microcredits,
        Address::from_str(&recipient).unwrap(),
        TransferType::from(transfer_type),
        None,
        None,
        None,
    );

    // Inform the user of the result of the transfer
    if transfer.is_err() {
        println!("{}", "Transfer failed with error:".to_string().red().bold());
    } else {
        println!("{}", "Transfer successful!".to_string().bright_green().bold());
        println!("Transaction ID:");
    }
    return transfer;
}