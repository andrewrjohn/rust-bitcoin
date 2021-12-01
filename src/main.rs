use bsv_wasm::{ECIESCiphertext, PrivateKey, ToHex};

// Convert to binary/buffer
// JS - Buffer.from("my string")
// Rust - "my string".as_bytes()

fn bytes_to_str(bytes: Vec<u8>) -> String {
    return std::str::from_utf8(&bytes[..]).unwrap().to_string();
}

fn main() {
    println!("{:?}", "hello".as_bytes());
    let priv_key = PrivateKey::from_random();
    let pub_key = priv_key.to_public_key().unwrap();
    let wif = priv_key.to_wif().unwrap();

    //  Encryption
    let secret_text = "hello world my name in Andrew";
    let encrypted_message = priv_key
        .encrypt_message(secret_text.as_bytes())
        .expect(format!("Can't encrypt message: {}", secret_text).as_str())
        .to_bytes()
        .to_hex();
    println!("Encrypted: {}", encrypted_message);

    //
    // ------------------------------
    //

    // Decryption
    let new_priv_key = PrivateKey::from_wif(wif.as_str())
        .expect("Can't create Private Key from provided WIF string");

    let decoded_encrypted_messaged =
        &hex::decode(encrypted_message).expect("Can't decode encrypted message from string")[..];
    let new_cipher = ECIESCiphertext::from_bytes(decoded_encrypted_messaged, true)
        .expect("Can't create new cipher");

    let decrypted_message =
        bytes_to_str(new_priv_key.decrypt_message(&new_cipher, &pub_key).unwrap());

    println!("1st wif: {}", wif);
    println!("2nd wif: {}", new_priv_key.to_wif().unwrap());
    println!("Decrypted: {}", decrypted_message);
}
