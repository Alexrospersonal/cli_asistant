use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
     #[command(about = "Analyzing code in file")]
    Analyze {
         #[arg(help = "File path which need analyzing")]
        path: String,
        
        #[arg(short, long, help = "Analyzing mode (some commands)")]
        flag: String
    },
    #[command(about = "Fix code in file")]
    Fix,
    Search {
        #[arg(short, long, help = "Search answer in StackOverflow", num_args = 1..)]
        query: Vec<String>
    }, 
    Track {
        #[command(subcommand)]
        action: TrackAction
    },
    Review {
        #[arg(help = "Path to file for review")]
        path: PathBuf
    },
    #[command(about = "Start interactive assistant mode")]
    Repl,
    #[command(about = "Send prompt to OpenAI")]
    Generate {
        #[arg(help = "Prompt to generate file")]
        prompt: String
    }
}

#[derive(Subcommand, Debug)]
pub enum TrackAction {
    Start {
        #[arg(short, long)]
        label: Option<String>
    },
    Stop,
    Status,
    Summary(SummaryArgs),
}

#[derive(Debug, Args)]
pub struct SummaryArgs {
    #[arg(long)]
    pub today: bool,
}