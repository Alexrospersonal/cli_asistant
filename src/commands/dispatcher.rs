use std::error::Error;
use crate::cli::command_enum::Commands;
use crate::commands::{analyze, fix, search, track, review};

pub async fn dispatch(cmd:Commands) -> Result<(), Box<dyn Error>> {
    match cmd {
        Commands::Analyze { path, flag } => analyze::execute(path, flag).await,
        Commands::Fix => fix::execute().await,
        Commands::Search {query} => search::execute(query).await,
        Commands::Track { action} => track::execute(action).await,
        Commands::Review {path} => review::execute(path).await
    }
}