use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "passman")]
#[command(version, about = "A terminal-based password manager.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// Enum defining available subcommands
#[derive(Subcommand)]
pub enum Commands {
    // Adds a new password with a name and password
    #[command(about = "Adds a new password to your encrypted passwords file.", long_about = None)]
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        password: String,
    },
    // Gets a password by name
    #[command(about = "Retrieves the password with the name you've provided.", long_about = None)]
    Get {
        #[arg(short, long)]
        name: String,
    },
    // Lists all passwords
    #[command(about = "Lists all of the passwords you currently have stored.", long_about = None)]
    List,
    // Removes a password by name
    #[command(about = "Removes the password with the name you've provided.", long_about = None)]
    Remove {
        #[arg(short, long)]
        name: String,
    },
}
