use crate::services::openai::{fetch_suggestion_from_api, DEFAULT_OPENAI_API_URL};
use colored::Colorize;
use std::error::Error;
use tokio::fs::read_to_string;

pub async fn execute(path: String, flag: String) -> Result<(), Box<dyn Error>> {
    println!("Analyze file: {}, {}", path.green(), flag.blue());

    let file = read_to_string(path).await?;

    for (idx, line) in file.lines().enumerate() {
        if idx > 5 {
            break;
        }
        println!("Index: {} {}", idx.to_string().yellow(), line.green());
    }

    let prompt = "Please analyze the following code and provide detailed suggestions for improvements,\
    optimizations, or best practices. Do not modify the code itself, just give recommendations.: \n\n";

    let message_to_ai = format!("{prompt} {file}");

    let suggestion = fetch_suggestion_from_api(&message_to_ai, DEFAULT_OPENAI_API_URL).await?;
    println!("Purpose from AI: {}", suggestion.blue());

    Ok(())
}
