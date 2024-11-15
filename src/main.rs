mod cli;
mod encryption;

extern crate rpassword;

use memoize::memoize;
use home::home_dir;
use clap::Parser;

use std::{
    fs, 
    path::{PathBuf, Path}
};
use serde_json::json;
use crate::encryption::{decrypt_password_file, encrypt_data, PasswordEntry, PasswordData};
use crate::cli::{Cli, Commands};

pub const PASSWORD_FILEPATH: &'static str = ".config/passman/passwords.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    get_master_password();

    set_up_password_file()?;

    match &cli.command.unwrap() {
        Commands::Add { name, password } => {
            add_password(name.clone(), password.clone())?;
        },
        Commands::Get { name } => {
            get_password_by_name(name.clone())?;
        },
        Commands::List => {
            list_passwords()?;
        },
        Commands::Remove { name } => {
            remove_password_by_name(name.clone())?;
        },
    }

    Ok(())
}

fn set_up_password_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&password_file_path()).exists() {
        println!("Password file does not exist. Setting up password file...");
        fs::File::create(&password_file_path()).expect("Failed to create password file");
        let data = json!({ "passwords": [] });
        encrypt_data(data.to_string())?;
    }
    return Ok(());
}

fn add_password(name: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    // Construct a new password entry struct
    let password_entry = PasswordEntry {
        name: name.trim().to_string(),
        password: password.trim().to_string()
    };

    // Decrypt the password file
    let mut decrypted_data: PasswordData = decrypt_password_file()?;

    // Before adding the new entry, check if a password with the same name already exists:
    let existing_password = decrypted_data.passwords.iter().find(|p| p.name == password_entry.name);

    if existing_password.is_some() {
        println!("A password for '{}' already exists. Please use a different name.", name);
        return Ok(());
    }

    // Add the new entry to the decrypted data
    decrypted_data.passwords.push(password_entry.clone());

    // Serialize updated data to JSON
    let updated_json = serde_json::to_string(&decrypted_data)?;

    // Encrypt the updated data
    encrypt_data(updated_json.clone())?;

    println!("Password added."); 

    Ok(())
}

fn get_password_by_name(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let password_data = decrypt_password_file()?;

    let passwords = password_data.passwords;

    let matching_password: Vec<_> = passwords
        .iter()
        .map(|p| p)
        .filter(|pass| pass.name == name)
        .collect();

    if matching_password.len() == 0 {
        println!("No password found for '{}'", name);
    } else {
        println!("Password for '{}': '{}'", name, matching_password[0].password);
    }

    Ok(())
}

fn list_passwords() -> Result<(), Box<dyn std::error::Error>> {
    let password_data = decrypt_password_file()?;

    let passwords = password_data.passwords;

    println!("Passwords:");
    for password in passwords {
        println!("   {}: '{}'", password.name, password.password);
    }

    Ok(())
}

fn remove_password_by_name(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let password_data = decrypt_password_file()?;

    let mut passwords = password_data.passwords;

    let index = passwords.iter().position(|p| *p.name == name).unwrap();
    passwords.remove(index);

    let updated_data = PasswordData {
        passwords
    };

    let updated_json = serde_json::to_string(&updated_data)?;

    encrypt_data(updated_json)?;

    println!("Password for {} removed.", name);

    Ok(())
}

#[memoize]
pub fn get_master_password() -> String {
    let password = rpassword::prompt_password("Please enter your master password: ").unwrap();
    println!("\r");
    return password;
}

#[memoize]
pub fn password_file_path() -> PathBuf {
    let home_dir = home_dir().expect("Failed to get home directory");

    return home_dir.join(PASSWORD_FILEPATH);
}
