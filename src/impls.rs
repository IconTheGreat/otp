use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

// ---------- Core Logic ----------

pub fn to_ascii(input: String) -> Vec<u8> {
    input.chars().map(|c| c as u8).collect()
}

pub fn to_binary(input: Vec<u8>) -> Vec<String> {
    input.iter().map(|b| format!("{:08b}", b)).collect()
}

pub fn xoring(input: Vec<String>, key: Vec<String>) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(i, byte)| {
            let b1 = u8::from_str_radix(byte, 2).unwrap();
            let b2 = u8::from_str_radix(&key[i % key.len()], 2).unwrap();
            b1 ^ b2
        })
        .collect()
}

pub fn generate_random_key_of_same_length(ascii: Vec<u8>) -> Vec<u8> {
    let mut rng = rand::rng();
    (0..ascii.len())
        .map(|_| rng.random_range(0..=255))
        .collect()
}

pub fn encrypt(input: String, key: Vec<u8>) -> Vec<u8> {
    let ascii = to_ascii(input);
    let ascii_binary = to_binary(ascii.clone());
    let key_binary = to_binary(key.clone());
    xoring(ascii_binary, key_binary)
}

pub fn decrypt(cipher_bytes: Vec<u8>, key: Vec<u8>) -> String {
    let cipher_binary = to_binary(cipher_bytes);
    let key_binary = to_binary(key);
    let xored = xoring(cipher_binary, key_binary);
    xored.iter().map(|&b| b as char).collect()
}

// ---------- HEX HELPERS ----------
// Convert bytes to hex string and vice versa
// Cause of rust format macro, the program might encounter some unprintable characters when displaying cipher and key.
// To avoid that, i save them as hex strings in a JSON file.
// Bare in mind that this is NOT secure storage, just a simple way to save/load data for this exercise 
// and also the ascii, to_binary function, xoring function etc are still used and working with raw bytes.

pub fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub fn from_hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

// ---------- JSON STRUCTS ----------

#[derive(Serialize, Deserialize)]
pub struct CipherData {
    pub cipher_hex: String,
    pub key_hex: String,
    pub plaintext_hint: Option<String>, // to remind what it was
}

// ---------- File Helpers ----------

pub fn save_to_json(filename: &str, data: &CipherData) -> io::Result<()> {
    let json = serde_json::to_string_pretty(data).unwrap();
    let mut file = fs::File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_from_json(filename: &str) -> io::Result<CipherData> {
    let contents = fs::read_to_string(filename)?;
    let data: CipherData = serde_json::from_str(&contents).unwrap();
    Ok(data)
}
