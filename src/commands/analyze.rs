use std::error::Error;
use colored::Colorize;
use tokio::fs::read_to_string;

pub async fn execute(path: String, flag: String) -> Result<(), Box<dyn Error>> {
    println!("Analyze file: {}, {}", path.green(), flag.blue());
    
    let file = read_to_string(path).await?;
    
    for (idx, line) in file.lines().enumerate() {
        if idx > 5 {
            break;
        }
        println!("Index: {} {}",idx.to_string().yellow(), line.green());
    }
    
    Ok(())
}