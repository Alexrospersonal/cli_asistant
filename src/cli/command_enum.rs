use clap::Subcommand;

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
}