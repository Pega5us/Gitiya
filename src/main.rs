use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitiya", version = "1.1", about = "A simple version control system")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init { args: Vec<String> },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Init { args }) => cmd_init(args),
        None => println!("No command provided. Run `gitiya --help` for usage.")
    }
}

fn cmd_init(args: Vec<String>) { println!("Executing init with args: {:?}", args); }