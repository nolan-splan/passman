use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key
};
use pbkdf2::pbkdf2_hmac;
use rand::Rng;
use sha2::Sha256;
use serde::{Deserialize, Serialize};

use std::{
    fs::File,
    path::PathBuf,
    io::{BufReader, Read, Write},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordData {
    pub passwords: Vec<PasswordEntry>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordEntry {
    pub name: String,
    pub password: String
}

pub fn encrypt_data(data: String, master_password: String, filepath: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);

    let key = derive_key_from_password(&master_password, &salt);

    let json_bytes = data.as_bytes();

    let cipher = Aes256Gcm::new(&key);

    let nonce_array: [u8; 12] = rand::random(); // Create the random nonce as a 12-byte array
    let nonce = Nonce::from_slice(&nonce_array); // Create nonce from slice

    // Encrypt the JSON data
    let ciphertext = cipher.encrypt(nonce, json_bytes)
        .expect("encryption failure!");

    // Save salt, nonce, and ciphertext to a new file
    let mut encrypted_file = File::create(filepath.clone())?;
    encrypted_file.write_all(&salt)?;       // Write salt
    encrypted_file.write_all(nonce)?;       // Write nonce
    encrypted_file.write_all(&ciphertext)?; // Write ciphertext

    Ok(())
}

pub fn decrypt_file(filepath: PathBuf, master_password: String) -> Result<PasswordData, Box<dyn std::error::Error>> {
    // Open the encrypted file and read the contents
    let mut file = BufReader::new(File::open(filepath)?);

    // Read salt (16 bytes)
    let mut salt = [0u8; 16];
    file.read_exact(&mut salt)?;

    let key = derive_key_from_password(&master_password, &salt);

    // Read nonce (12 bytes)
    let mut nonce_bytes = [0u8; 12];
    file.read_exact(&mut nonce_bytes)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Read the rest as the ciphertext
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    // Set up the cipher and decrypt the data
    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(data) => data,
        Err(_) => {
            println!("Decryption failed. Incorrect password or corrupted data.");
            std::process::exit(1);
        }
    };

    // Deserialize decrypted JSON into PasswordData struct
    let password_data: PasswordData = serde_json::from_slice(&decrypted_data)?;

    Ok(password_data)
}

// Derive the 256-bit key using PBKDF2-HMAC with the same passphrase and salt
fn derive_key_from_password(master_password: &str, salt: &[u8]) -> Key<Aes256Gcm> {
    // Initialize a mutable array to hold the derived key bytes
    let mut key_bytes = [0u8; 32];

    // Derive the key using PBKDF2 with SHA-256
    pbkdf2_hmac::<Sha256>(
        master_password.as_bytes(),
        salt,
        100_000,
        &mut key_bytes,
    );

    // Convert the derived bytes into a `Key<Aes256Gcm>`
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

