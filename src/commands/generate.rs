use crate::services::openai::{fetch_suggestion_from_api, DEFAULT_OPENAI_API_URL};
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::error::Error;

pub async fn execute(prompt: String) -> Result<(), Box<dyn Error>> {
    let generate_prompt = format!(
        "Please generate a complete, idiomatic Rust code snippet based on the following description.\
    Return only the code block without explanations and ``` or another wrapper brackets:\n\n{}",
        &prompt
    );

    let code = fetch_suggestion_from_api(&generate_prompt, DEFAULT_OPENAI_API_URL).await?;

    println!("{}", code.yellow());

    let apply = Confirm::new()
        .with_prompt("Do you want to save this code to a file?")
        .default(false)
        .interact()?;

    if apply {
        let filename = Input::<String>::new()
            .with_prompt("Enter a file name, like: main.rs")
            .with_initial_text("new_file.rs")
            .interact_text()
            .unwrap();

        let temp_path = std::env::temp_dir().join(filename);

        tokio::fs::write(&temp_path, code).await?;
        println!("✅ Diff saved to: {}", &temp_path.display());
    }

    Ok(())
}
