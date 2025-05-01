mod cli;

use std::error::Error;
use clap::Parser;
use cli::args::{Args};
use crate::cli::command_enum::Commands;
use colored::Colorize;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    run(args).await.expect("Parsing failed");
}

async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let command = args.command.expect("Command not found");

    match command {
        Commands::Analyze { path, flag } => {
            println!("Analyze file: {}, {}", path.green(), flag.blue());
        }
        Commands::Fix => {
            let command = String::from("Run fix command").yellow();
            println!("{command}");
        }
    };

    Ok(())
}

#[test]
fn test_analyze_parsing() {
    let args = Args::parse_from([
        "test_bin", "analyze", "./main.rs", "--flag", "fast"
    ]);

    match args.command { 
        Some(Commands::Analyze { path, flag}) => {
            assert_eq!(path, "./main.rs");
            assert_eq!(flag, "fast");
        }
        _ => panic!("Analyze command not parsed")
    }
}
