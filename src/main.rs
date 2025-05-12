extern crate core;

mod cli;
mod commands;
mod services;

use std::error::Error;
use clap::Parser;
use cli::args::{Args};
use commands::dispatcher::dispatch;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    run(args).await.expect("Parsing failed");
}

async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let command = args.command.expect("Command not found");
    dispatch(command).await
}

#[test]
fn test_analyze_parsing() {
    // let args = Args::parse_from([
    //     "test_bin", "analyze", "./main.rs", "--flag", "fast"
    // ]);

    // match args.command {
    //     Some(Commands::Analyze { path, flag}) => {
    //         assert_eq!(path, "./main.rs");
    //         assert_eq!(flag, "fast");
    //     }
    //     _ => panic!("Analyze command not parsed")
    // }
}
