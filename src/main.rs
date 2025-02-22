use solana_utils::{menu, store, utils, wallet};

fn main() {
    // Create initial wallet if none exists
    if store::list().is_empty() {
        println!("No keypairs found. Creating initial keypair...");
        wallet::create_keypair();
    }

    loop {
        let selection = menu::show_main_menu();
        match selection {
            0 => wallet::create_keypair(),
            1 => wallet::handle_wallet_selection(),
            2 => {
                for key in store::list() {
                    println!("{}", key);
                }
            }
            3 => {
                let keys = store::list();
                if keys.is_empty() {
                    println!("No keypairs to delete!");
                    continue;
                }
                let key_idx =
                    utils::select_from_list("Which keypair do you want to delete?", &keys);
                store::delete(keys[key_idx].as_str());
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}
