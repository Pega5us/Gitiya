use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitiya", version = "1.1", about = "A simple version control system")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { file: String },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Init) => println!("init command not supported!"),
        Some(Commands::Add { file }) => println!("add command not supported"),
        None => println!("No command provided."),
    }
}