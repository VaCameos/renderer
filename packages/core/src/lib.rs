use wasm_bindgen::prelude::*;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex;

// AES 256 CBC
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[wasm_bindgen]
pub fn encrypt(key: &str, iv: &str, data: &str) -> String {
    let key_bytes = hex::decode(key).expect("Invalid key format");
    let iv_bytes = hex::decode(iv).expect("Invalid IV format");
    let data_bytes = data.as_bytes();

    let cipher = Aes256Cbc::new_from_slices(&key_bytes, &iv_bytes).unwrap();
    let encrypted_data = cipher.encrypt_vec(data_bytes);

    hex::encode(&encrypted_data)
}

#[wasm_bindgen]
pub fn decrypt(key: &str, iv: &str, encrypted_data: &str) -> String {
    let key_bytes = hex::decode(key).expect("Invalid key format");
    let iv_bytes = hex::decode(iv).expect("Invalid IV format");
    let encrypted_bytes = hex::decode(encrypted_data).expect("Invalid encrypted data format");

    let cipher = Aes256Cbc::new_from_slices(&key_bytes, &iv_bytes).unwrap();
    let decrypted_data = cipher.decrypt_vec(&encrypted_bytes).unwrap();

    String::from_utf8(decrypted_data).expect("Invalid UTF-8")
}