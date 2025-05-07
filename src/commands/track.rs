mod stop;
mod start;
mod status;
mod summary;
mod db;

use crate::cli::command_enum::TrackAction;

pub async fn execute(cmd: TrackAction) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        TrackAction::Start { label } => start::run(label).await,
        TrackAction::Stop => stop::run().await,
        TrackAction::Status => status::run().await,
        TrackAction::Summary => summary::run().await,
    }
}