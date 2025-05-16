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
        path: PathBuf,
        #[arg(long, help = "Review code without changing file")]
        dry_run:bool,
        #[arg(long, help = "Max cycling value")]
        max_loop: Option<u32>,
        #[arg(long, help = "Directory to save diffs")]
        output_dir: Option<PathBuf>,
        #[arg(long, help = "Apply all changes without confirmation")]
        no_confirm: bool
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