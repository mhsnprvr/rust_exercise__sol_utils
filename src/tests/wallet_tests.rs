use crate::{store, wallet};
use serial_test::serial;
use solana_sdk::signer::{keypair::Keypair, Signer};
use test_log::test;

fn setup() {
    store::clear();
}

#[test]
#[serial]
fn test_create_and_store_keypair() {
    setup();
    let initial_count = store::list().len();
    println!("Initial keypair count: {}", initial_count);

    wallet::create_keypair();
    let final_count = store::list().len();
    println!("Final keypair count: {}", final_count);

    assert_eq!(
        final_count,
        initial_count + 1,
        "Keypair count didn't increase after creation"
    );
}

#[test]
#[serial]
fn test_sign_and_verify_message() {
    setup();
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let message = "test message";
    println!("Testing with pubkey: {}", pubkey);
    println!("Test message: {}", message);

    // Store keypair
    println!("Storing keypair...");
    store::insert(&pubkey, &keypair.to_bytes());

    // Sign message
    println!("Signing message...");
    let signature = keypair.sign_message(message.as_bytes());
    let signature_str = signature.to_string();
    println!("Generated signature: {}", signature_str);

    // Verify signature
    println!("Verifying signature...");
    let verified = wallet::verify_message(&pubkey, message, &signature_str);
    assert!(verified, "Signature verification failed");
}

#[test]
#[serial]
fn test_delete_keypair() {
    setup();
    println!("Creating test keypair...");
    wallet::create_keypair();
    let initial_count = store::list().len();
    println!("Initial keypair count: {}", initial_count);

    println!("Deleting first keypair...");
    store::delete(store::list()[0].as_str());
    let final_count = store::list().len();
    println!("Final keypair count: {}", final_count);

    assert_eq!(
        final_count,
        initial_count - 1,
        "Keypair count didn't decrease after deletion"
    );
}
