mod impls;
use impls::*;
use std::io;

fn main() {
    println!("Do you want to (E)ncrypt or (D)ecrypt?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().to_lowercase();

    if choice == "e" || choice == "encrypt" {
        println!("Enter plaintext:");
        let mut plaintext = String::new();
        io::stdin().read_line(&mut plaintext).unwrap();
        let plaintext = plaintext.trim().to_string();

        let ascii = to_ascii(plaintext.clone());
        let key = generate_random_key_of_same_length(ascii.clone());
        let cipher_bytes = encrypt(plaintext.clone(), key.clone());

        let cipher_hex = to_hex(&cipher_bytes);
        let key_hex = to_hex(&key);

        println!("\n✅ Encryption complete!");
        println!("Plaintext: {}", plaintext);
        println!("Cipher (hex): {}", cipher_hex);
        println!("Key (hex): {}", key_hex);

        // Save to file
        let data = CipherData {
            cipher_hex: cipher_hex.clone(),
            key_hex: key_hex.clone(),
            plaintext_hint: Some(plaintext.clone()),
        };

        println!("\nEnter filename to save (e.g. secret.json):");
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).unwrap();
        let filename = filename.trim();

        if let Err(err) = save_to_json(filename, &data) {
            eprintln!("❌ Failed to save file: {}", err);
        } else {
            println!("💾 Saved to '{}'", filename);
        }
    } else if choice == "d" || choice == "decrypt" {
        println!("Enter filename of JSON to decrypt (e.g. secret.json):");
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).unwrap();
        let filename = filename.trim();

        match load_from_json(filename) {
            Ok(data) => {
                let cipher_bytes = from_hex(&data.cipher_hex);
                let key_bytes = from_hex(&data.key_hex);
                let decrypted = decrypt(cipher_bytes, key_bytes);

                println!("\n✅ Decryption complete!");
                println!("Decrypted text: {}", decrypted);
            }
            Err(err) => eprintln!("❌ Failed to read file: {}", err),
        }
    } else {
        println!("Invalid choice — please enter E or D.");
    }
}
