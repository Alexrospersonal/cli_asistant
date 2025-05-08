use std::error::Error;
use std::path::PathBuf;
use colored::{Color, Colorize};
use dialoguer::Confirm;
use similar::{ChangeTag, TextDiff};

pub async fn execute(path_buf: PathBuf) -> Result<(), Box<dyn Error>> {
    let original_file = tokio::fs::read_to_string(&path_buf).await?;
    let suggestion = mock_ai_suggestion(&original_file);

    print_diff_to_cli(&original_file, &suggestion);
    
    let review_name = format!("review_{}.diff", chrono::Utc::now().timestamp());
    let diff_as_string = create_diff_string(&original_file, &suggestion);

    let apply = Confirm::new()
        .with_prompt(format!("Apply changes to {}?", &path_buf.display()))
        .default(false)
        .interact()?;

    if apply {
        tokio::fs::write(&path_buf, suggestion).await?;
        save_diff_to_temp_file(review_name, diff_as_string).await?;
        println!("✅ Changes applied to: {}", path_buf.display());
    } else {
        println!("❌ Changes were not applied.");
    }
    

    Ok(())
}

fn print_diff_to_cli(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);
    
    for change in diff.iter_all_changes(){
        let (sign, color) = match change.tag() {
            ChangeTag::Delete => ("-", Color::Red),
            ChangeTag::Insert => ("+", Color::Green),
            ChangeTag::Equal => (" ", Color::White),
        };
        let line = format!("{sign } {change}");

        println!("{}",line.color(color));
    }

    let changes_count_string = format!(
        "Changes: {}", diff.iter_all_changes().count()
    );

    println!("{}", changes_count_string.yellow());
}

fn create_diff_string(old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    
    let diff_as_string: String = diff.iter_all_changes().map(|change| {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " "
        };
        format!("{sign} {change}\n")
    }).collect();
    
    diff_as_string
}

async fn save_diff_to_temp_file(review_name: String, file: String) -> Result<(), Box<dyn Error>> {
    let temp_path = std::env::temp_dir().join(review_name);
    
    tokio::fs::write(&temp_path, file).await?;
    println!("✅ Diff saved to: {}", &temp_path.display());
    
    Ok(())
}

fn mock_ai_suggestion(file: &str) -> String {
    file
        .lines()
        .map(|row| format!("{row} // suggested"))
        .collect::<Vec<_>>()
        .join("\n")
}