use std::str::FromStr;

use crate::config::SolanaRpcUrl;
use crate::menu;
use crate::store;
use crate::utils::{read_line, select_from_list};
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::{keypair::Keypair, Signer};
use solana_sdk::signers::Signers;

fn get_rpc_url() -> String {
    dotenv().ok();
    let env_url = SolanaRpcUrl::Devnet;
    env_url.as_str().to_string()
}

pub fn create_keypair() {
    let key_pair = Keypair::new();
    let pubkey = key_pair.pubkey();
    println!("Keypair created: {:?}", pubkey);
    store::insert(pubkey.to_string().as_str(), &key_pair.to_bytes());
}

pub fn get_balance(pubkey: &str) {
    let client = RpcClient::new(get_rpc_url());
    let pubkey = Pubkey::from_str(pubkey).unwrap();
    let balance = client.get_balance(&pubkey);
    println!("Balance: {:?}", balance);
}

pub fn sign_message(pubkey: &str, message: &str) {
    let pubkey = Pubkey::from_str(pubkey).unwrap();
    let keypair = Keypair::from_bytes(&store::get(pubkey.to_string().as_str()).unwrap());
    let message = message.as_bytes();
    let signature = keypair.sign_message(message);
    println!("Signature: {:?}", signature);
}

pub fn verify_message(pubkey: &str, message: &str, signature: &str) {
    let pubkey = Pubkey::from_str(pubkey).unwrap();
    let message_bytes = message.as_bytes();
    let signature =
        solana_sdk::signature::Signature::from_str(signature).expect("Invalid signature format");
    let verified = signature.verify(&pubkey.to_bytes(), message_bytes);
    println!("Signature verification result: {}", verified);
}

pub fn handle_wallet_selection() {
    let keys = store::list();
    if keys.is_empty() {
        println!("No keypairs available!");
        return;
    }

    let key_idx = select_from_list("Select a wallet", &keys);
    let selected_pubkey = &keys[key_idx];

    loop {
        let action = menu::show_wallet_actions(selected_pubkey);
        match action {
            0 => println!("Sending SOL... (not implemented)"),
            1 => println!("Hashing transaction... (not implemented)"),
            2 => {
                let message = read_line("Enter the message to sign: ");
                sign_message(selected_pubkey, &message);
            }
            3 => {
                let message = read_line("Enter the message to verify: ");
                let signature = read_line("Enter the signature: ");
                verify_message(selected_pubkey, &message, &signature);
            }
            4 => {
                get_balance(selected_pubkey);
            }
            5 => break,
            _ => println!("Invalid option"),
        }
    }
}
