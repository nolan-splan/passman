use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// Enum defining available subcommands
#[derive(Subcommand)]
pub enum Commands {
    // Adds a new password with a name and password
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        password: String,
    },
    // Gets a password by name
    Get {
        #[arg(short, long)]
        name: String,
    },
    // Lists all passwords
    List,
    // Removes a password by name
    Remove {
        #[arg(short, long)]
        name: String,
    },
}
