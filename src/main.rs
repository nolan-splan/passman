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
    // Use `try_parse` to handle cases with no arguments gracefully
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e); // Print the error (it shows usage/help information)
            std::process::exit(1); // Exit gracefully
        }
    };

    if cli.command.is_none() {
        eprintln!("No command provided. Use 'passman --help' for usage information.");
        std::process::exit(1);
    }

    get_master_password();

    set_up_password_file()?;

    match &cli.command.unwrap() {
        Commands::Add { name } => {
            add_password(name.clone())?;
        },
        Commands::Get { name } => {
            get_password_by_name(name.clone())?;
        },
        Commands::List => {
            list_passwords()?;
        },
        Commands::Update { name } => {
            update_password(name.clone())?;
        },
        Commands::Remove { name } => {
            remove_password_by_name(name.clone())?;
        },
    }

    Ok(())
}

fn set_up_password_file() -> Result<(), Box<dyn std::error::Error>> {
    let path = password_file_path();
    let dir_path = Path::new(&path).parent().unwrap();

    // Create the `passman` directory if it doesn't exist
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    if !Path::new(&password_file_path()).exists() {

        // If the password file doesn't exist, that means this is the first time the user
        // is running the program. Since it's the first time they're running the program,
        // we should make them confirm their master password before creating and encrypting
        // the password file. This way, we can ensure that the user has entered their master
        // password correctly before we start storing their passwords.

        confirm_master_password("Please confirm your master password: ".to_string(), 1);

        println!("Password file does not exist. Setting up password file...");
        println!("\r");
        fs::File::create(&password_file_path()).expect("Failed to create password file");
        let data = json!({ "passwords": [] });
        encrypt_data(data.to_string())?;
    }
    return Ok(());
}

fn add_password(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = format!("Please enter the password for '{}': ", name);

    let password = rpassword::prompt_password(prompt).unwrap();
    println!("\r");

    // Construct a new password entry struct
    let password_entry = PasswordEntry {
        name: name.trim().to_string(),
        password: password.trim().to_string()
    };

    // Decrypt the password file
    let mut decrypted_data: PasswordData = decrypt_password_file()?;

    // Before adding the new entry, check if a password with the same name already exists:
    let existing_password = decrypted_data.passwords.iter().find(|p| p.name == password_entry.name);

    // Maybe prompt to update instead?
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

    if passwords.len() == 0 {
        println!("No passwords found. Add a password using 'passman add'");
        return Ok(());
    }

    println!("Passwords:");
    for password in passwords {
        println!("   {}: '{}'", password.name, password.password);
    }

    Ok(())
}

fn update_password(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut password_data = decrypt_password_file()?;

    // let mut passwords = password_data.passwords;

    let index = password_data.passwords.iter().position(|p| *p.name == name);

    if index.is_none() {
        println!("No password found for '{}'", name);
        return Ok(());
    }

    let prompt = format!("Please enter the new password for '{}': ", name);

    let new_password = rpassword::prompt_password(prompt).unwrap();
    println!("\r");

    password_data.passwords.remove(index.unwrap());

    // Construct a new password entry struct
    let password_entry = PasswordEntry {
        name: name.trim().to_string(),
        password: new_password.trim().to_string()
    };

    // Add the new entry to the decrypted data
    password_data.passwords.push(password_entry.clone());

    // Serialize updated data to JSON
    let updated_json = serde_json::to_string(&password_data)?;

    // Encrypt the updated data
    encrypt_data(updated_json.clone())?;

    println!("Password for '{}' updated.", name);

    Ok(())
}

fn remove_password_by_name(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let password_data = decrypt_password_file()?;

    let mut passwords = password_data.passwords;

    let index = passwords.iter().position(|p| *p.name == name);

    if index.is_none() {
        println!("No password found for '{}'", name);
        return Ok(());
    }

    passwords.remove(index.unwrap());

    let updated_data = PasswordData {
        passwords
    };

    let updated_json = serde_json::to_string(&updated_data)?;

    encrypt_data(updated_json)?;

    println!("Password for {} removed.", name);

    Ok(())
}

fn confirm_master_password(prompt: String, count: u8) {
    if count > 3 {
        println!("Too many attempts to confirm master password. Exiting...");
        std::process::exit(1);
    }

    let password = rpassword::prompt_password(prompt).unwrap();
    println!("\r");

    if password != get_master_password() {
        confirm_master_password("Passwords do not match, please try again: ".to_string(), count + 1);
    }
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
