use std::fs;
use std::env;

use alloy_primitives::{hex, Address, PrimitiveSignature};
use alloy_consensus::TxEnvelope;
use alloy_eips::eip2718::Decodable2718;

fn main() {
    // Read file name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];

    // Read the file contents
    let file_contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    // Decode the hex-encoded transaction data
    let data = hex::decode(file_contents.trim())
        .expect("Failed to decode hex data");

    // Decode the transaction
    let tx: TxEnvelope = TxEnvelope::decode_2718(&mut data.as_slice())
        .expect("Failed to decode transaction");

    println!("Transaction: {:?}\n\n", tx);

    // Get the signature hash
    let pre_hash = tx.signature_hash();

    // Extract signature values
    let tx_r = tx.signature().r();
    let tx_s = tx.signature().s();
    let tx_v = tx.signature().v();

    println!("keccak(rlp(data)): {:?}\nr: {:?}\ns: {:?}\nv: {:?}\n", pre_hash, tx_r, tx_s, tx_v);

    
    // Recover the address
    let sig: PrimitiveSignature = PrimitiveSignature::new(tx_r, tx_s, tx_v);
    let addr: Address = sig.recover_address_from_prehash(&pre_hash)
        .expect("Failed to recover address");

    // Print the recovered address
    println!("Recovered Address: {:?}", addr);
}