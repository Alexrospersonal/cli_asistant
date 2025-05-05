use clap::Parser;
use crate::cli::command_enum::Commands;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[test]
fn test_search_command_parsing() {
    let args = Args::parse_from(["bin", "search", "--query", "rust", "borrow", "checker"]);
    
    match args.command { 
        Some(Commands::Search { query}) => {
            assert_eq!(query, vec!["rust", "borrow", "checker"]);
        }
        _ => panic!("Search command not parsed correctly"),
    }
}