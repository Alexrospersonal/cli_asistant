use std::error::Error;
use colored::Colorize;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let command = String::from("Run fix command").yellow();
    println!("{command}");
    Ok(())
}