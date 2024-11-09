mod encryption;

extern crate rpassword;

use memoize::memoize;
use home::home_dir;

use std::{
    fs, 
    io, 
    path::{PathBuf, Path}
};
use serde_json::json;
use crate::encryption::{decrypt_password_file, encrypt_data, PasswordEntry, PasswordData};

pub const PASSWORD_FILEPATH: &'static str = ".config/passman/passwords.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let master_password = get_master_password();

    if !Path::new(&password_file_path()).exists() {
        set_up_passwords_file(master_password.clone())?;
    }

    println!("Select an option:");
    println!("   1. Add a new password");
    println!("   2. View all passwords");
    println!("   3. Exit");
    println!("\r");

    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Failed to read option");

    match option.trim() {
        "1" => {
            add_password(master_password.clone())?;
        },
        "2" => {
            let password_data = decrypt_password_file(master_password.clone())?;

            let passwords = password_data.passwords;

            println!("Passwords:");
            for password in passwords {
                println!("   {}: '{}'", password.name, password.password);
            }
        },
        "3" => {
            std::process::exit(0);
        },
        _ => {
            println!("Invalid option");
        }
    }

    Ok(())
}

fn get_master_password() -> String {
    let password = rpassword::prompt_password("Please enter your master password: ").unwrap();
    println!("\r");
    return password;
}

fn set_up_passwords_file(master_password: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("Password file does not exist. Setting up password file...");
    
    let password_file_path = password_file_path();

    fs::File::create(&password_file_path).expect("Failed to create password file");

    let initial_data = json!({ "passwords": [] });

    encrypt_data(initial_data.to_string(), master_password.clone())?;

    return Ok(password_file_path);
}

fn add_password(master_password: String) -> Result<(), Box<dyn std::error::Error>> {
    // Get the password name
    println!("Please enter the name of the password:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    // Get the password
    println!("Please enter the password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");

    // Construct a new password entry struct
    let password_entry = PasswordEntry {
        name: name.trim().to_string(),
        password: password.trim().to_string()
    };

    let mut decrypted_data: PasswordData = decrypt_password_file(master_password.clone())?;

    // Add the new entry
    decrypted_data.passwords.push(password_entry.clone());

    // Serialize updated data to JSON
    let updated_json = serde_json::to_string(&decrypted_data)?;

    let _ = encrypt_data(updated_json.clone(), master_password.clone())?;

    println!("Password added."); 

    Ok(())
}

#[memoize]
pub fn password_file_path() -> PathBuf {
    println!("password_file_path called");
    let home_dir = home_dir().expect("Failed to get home directory");

    return home_dir.join(PASSWORD_FILEPATH);
}
