use clap::{Parser, Subcommand};
use crate::constants;

#[derive(Parser)]
#[command(name = "gitiya", version = "1.1", about = "A simple version control system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(default_value = constants::CURRENT_DIRECTORY)]
        path: String,
    },
}