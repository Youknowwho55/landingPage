// src/bin/admin.rs
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    AddUser { username: String },
    ListUsers,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::AddUser { username } => println!("Adding user: {}", username),
        Command::ListUsers => println!("Listing users..."),
    }
}
