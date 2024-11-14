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
    #[command(about = "Adds a new password.", long_about = None)]
    Add {
        #[arg(short, long)]
        name: String,
    },
    // Gets a password by name
    #[command(about = "Gets a password by name", long_about = None)]
    Get {
        #[arg(short, long)]
        name: String,
    },
    // Lists all passwords
    #[command(about = "Lists all passwords", long_about = None)]
    List,
    // Updates a password by name
    #[command(about = "Updates a password by name", long_about = None)]
    Update {
        #[arg(short, long)]
        name: String,
    },
    // Removes a password by name
    #[command(about = "Removes a password by name", long_about = None)]
    Remove {
        #[arg(short, long)]
        name: String,
    },
}
