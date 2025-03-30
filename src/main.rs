use clap::Parser;

mod constants;
mod commands;
mod cli;
mod util;
mod repository;

fn main() {
    let args = cli::Cli::parse();
    match args.command {
        Some(cli::Commands::Init { path }) => commands::init::cmd_init(&path),
        None => println!("No command provided. Run `gitiya --help` for usage.")
    }
}
