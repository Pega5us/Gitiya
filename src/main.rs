use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitiya", version = "1.1", about = "A simple version control system")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { args: Vec<String> },
    CatFile { args: Vec<String> },
    CheckIgnore { args: Vec<String> },
    Checkout { args: Vec<String> },
    Commit { args: Vec<String> },
    HashObject { args: Vec<String> },
    Init { args: Vec<String> },
    Log { args: Vec<String> },
    LsFiles { args: Vec<String> },
    LsTree { args: Vec<String> },
    RevParse { args: Vec<String> },
    Rm { args: Vec<String> },
    ShowRef { args: Vec<String> },
    Status { args: Vec<String> },
    Tag { args: Vec<String> },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Add { args }) => cmd_add(args),
        Some(Commands::CatFile { args }) => cmd_cat_file(args),
        Some(Commands::CheckIgnore { args }) => cmd_check_ignore(args),
        Some(Commands::Checkout { args }) => cmd_checkout(args),
        Some(Commands::Commit { args }) => cmd_commit(args),
        Some(Commands::HashObject { args }) => cmd_hash_object(args),
        Some(Commands::Init { args }) => cmd_init(args),
        Some(Commands::Log { args }) => cmd_log(args),
        Some(Commands::LsFiles { args }) => cmd_ls_files(args),
        Some(Commands::LsTree { args }) => cmd_ls_tree(args),
        Some(Commands::RevParse { args }) => cmd_rev_parse(args),
        Some(Commands::Rm { args }) => cmd_rm(args),
        Some(Commands::ShowRef { args }) => cmd_show_ref(args),
        Some(Commands::Status { args }) => cmd_status(args),
        Some(Commands::Tag { args }) => cmd_tag(args),
        None => println!("No command provided. Run `gitiya --help` for usage."),
    }
}

fn cmd_add(args: Vec<String>) { println!("Executing add with args: {:?}", args); }
fn cmd_cat_file(args: Vec<String>) { println!("Executing cat-file with args: {:?}", args); }
fn cmd_check_ignore(args: Vec<String>) { println!("Executing check-ignore with args: {:?}", args); }
fn cmd_checkout(args: Vec<String>) { println!("Executing checkout with args: {:?}", args); }
fn cmd_commit(args: Vec<String>) { println!("Executing commit with args: {:?}", args); }
fn cmd_hash_object(args: Vec<String>) { println!("Executing hash-object with args: {:?}", args); }
fn cmd_init(args: Vec<String>) { println!("Executing init with args: {:?}", args); }
fn cmd_log(args: Vec<String>) { println!("Executing log with args: {:?}", args); }
fn cmd_ls_files(args: Vec<String>) { println!("Executing ls-files with args: {:?}", args); }
fn cmd_ls_tree(args: Vec<String>) { println!("Executing ls-tree with args: {:?}", args); }
fn cmd_rev_parse(args: Vec<String>) { println!("Executing rev-parse with args: {:?}", args); }
fn cmd_rm(args: Vec<String>) { println!("Executing rm with args: {:?}", args); }
fn cmd_show_ref(args: Vec<String>) { println!("Executing show-ref with args: {:?}", args); }
fn cmd_status(args: Vec<String>) { println!("Executing status with args: {:?}", args); }
fn cmd_tag(args: Vec<String>) { println!("Executing tag with args: {:?}", args); }
