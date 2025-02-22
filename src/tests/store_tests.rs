use crate::store;
use serial_test::serial;
use solana_sdk::signer::{keypair::Keypair, Signer};
use test_log::test;

fn setup() {
    store::clear();
}

#[test]
#[serial]
fn test_store_operations() {
    setup();
    let key_pair = Keypair::new();
    let pubkey = key_pair.pubkey().to_string();
    println!("Testing with pubkey: {}", pubkey);

    // Test insert
    println!("Testing insert operation...");
    store::insert(&pubkey, &key_pair.to_bytes());
    assert!(
        store::get(&pubkey).is_some(),
        "Failed to retrieve stored keypair"
    );

    // Test list
    println!("Testing list operation...");
    let keys = store::list();
    println!("Current store keys: {:?}", keys);
    assert!(keys.contains(&pubkey), "Pubkey not found in store list");

    // Test delete
    println!("Testing delete operation...");
    store::delete(&pubkey);
    assert!(
        !store::list().contains(&pubkey),
        "Pubkey still exists after deletion"
    );
}
