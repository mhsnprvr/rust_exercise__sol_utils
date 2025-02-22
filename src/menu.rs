use crate::utils::select_from_list;

pub fn show_main_menu() -> usize {
    let options = vec![
        "create a new keypair",
        "select wallet",
        "see the list of keypairs",
        "delete a keypair",
        "exit",
    ];
    select_from_list("What do you want to do?", &options)
}

pub fn show_wallet_actions(wallet_pubkey: &str) -> usize {
    let options = vec![
        "fill with faucet",
        "hash a transaction",
        "sign a message",
        "verify a message",
        "get balance",
        "back to main menu",
    ];
    select_from_list(
        &format!(
            "Selected wallet: {}. What would you like to do?",
            wallet_pubkey
        ),
        &options,
    )
}
