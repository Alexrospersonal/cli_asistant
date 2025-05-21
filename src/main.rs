extern crate core;

mod cli;
mod commands;
mod services;

use clap::Parser;
use cli::args::Args;
use commands::dispatcher::dispatch;
use std::error::Error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .init();

    let args = Args::parse();
    run(args).await.expect("Parsing failed");
}

async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let command = args.command.expect("Command not found");
    dispatch(command).await
}

#[test]
fn test_analyze_parsing() {}
