use clap::Parser;
use crate::cli::command_enum::Commands;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}