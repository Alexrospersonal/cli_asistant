use std::error::Error;
use crate::cli::command_enum::Commands;
use crate::commands::{analyze, fix, search, track, review, repl, generate};

pub async fn dispatch(cmd:Commands) -> Result<(), Box<dyn Error>> {
    match cmd {
        Commands::Repl => repl::run_loop().await,
        _ => dispatch_no_repl(cmd).await
    }
}

pub async fn dispatch_no_repl(cmd:Commands) -> Result<(), Box<dyn Error>> {
    match cmd {
        Commands::Analyze { path, flag } => analyze::execute(path, flag).await,
        Commands::Fix => fix::execute().await,
        Commands::Search {query} => search::execute(query).await,
        Commands::Track { action} => track::execute(action).await,
        Commands::Review {
            path,
            dry_run,
            max_loop,
            output_dir,
            no_confirm
        } => review::execute(path, dry_run, max_loop, output_dir, no_confirm).await,
        Commands::Repl => Err("Command repl already run".into()),
        Commands::Generate {prompt} => generate::execute(prompt).await,
    }
}