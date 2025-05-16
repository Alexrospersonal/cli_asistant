use crate::services::openai::fetch_suggestion_from_api;
use colored::{Color, Colorize};
use dialoguer::Confirm;
use similar::{ChangeTag, TextDiff};
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub async fn execute(
    path_buf: PathBuf,
    dry_run: bool,
    max_loop: Option<u32>,
    output_dir: Option<PathBuf>,
    no_confirm: bool,
) -> Result<(), Box<dyn Error>> {
    let mut loop_counter = 0;
    // TODO: refactor

    loop {
        if loop_counter >= max_loop.unwrap_or(5) {
            let max_loop_message = format!("Reached max loops ({:?}). Review stopped.", max_loop);
            println!("{}", max_loop_message.blue());
            break;
        }

        let original_file = tokio::fs::read_to_string(&path_buf).await?;

        let prompt = "Please review the following code and propose actual improvements by fully rewriting the code if necessary.\
        Return only the improved version of the code, without explanations.: \n\n";

        let message_to_ai = format!("{prompt} {original_file}");
        let suggestion = fetch_suggestion_from_api(&message_to_ai).await?;

        let diff = TextDiff::from_lines(&original_file, &suggestion);

        if diff.ratio() == 1.0 {
            println!("{}", "Ai assistant not changing your code".yellow());
            break;
        }

        print_diff_to_cli(&original_file, &suggestion);

        if dry_run {
            println!(
                "{}",
                "Dry run mode active — code will NOT be changed".yellow()
            );
            continue;
        } else if no_confirm {
            apply_changing(&original_file, &suggestion, &path_buf, &output_dir).await?;
            println!("Auto-applying changes (--no-confirm)");
            break;
        } else {
            let apply = Confirm::new()
                .with_prompt(format!("Apply changes to {}?", &path_buf.display()))
                .default(false)
                .interact()?;

            if apply {
                apply_changing(&original_file, &suggestion, &path_buf, &output_dir).await?;
            } else {
                break;
            }
        }

        loop_counter += 1;
    }

    Ok(())
}

async fn apply_changing(
    original_file: &String,
    suggestion: &String,
    path_buf: &PathBuf,
    output_dir: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let review_name = format!("review_{}.diff", chrono::Utc::now().timestamp());
    let diff_as_string = create_diff_string(&original_file, &suggestion);

    tokio::fs::write(&path_buf, suggestion).await?;

    let save_dir = output_dir.clone().unwrap_or_else(|| env::temp_dir());

    save_diff_to_temp_file(review_name, diff_as_string, save_dir).await?;

    println!("✅ Changes applied to: {}", path_buf.display());

    Ok(())
}

fn print_diff_to_cli(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);

    for change in diff.iter_all_changes() {
        let (sign, color) = match change.tag() {
            ChangeTag::Delete => ("-", Color::Red),
            ChangeTag::Insert => ("+", Color::Green),
            ChangeTag::Equal => (" ", Color::White),
        };
        let line = format!("{sign } {change}");

        println!("{}", line.color(color));
    }

    let changes_count_string = format!("Changes: {}", diff.iter_all_changes().count());

    println!("{}", changes_count_string.yellow());
}

fn create_diff_string(old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);

    let diff_as_string: String = diff
        .iter_all_changes()
        .map(|change| {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            format!("{sign} {change}\n")
        })
        .collect();

    diff_as_string
}

async fn save_diff_to_temp_file(
    review_name: String,
    file: String,
    output_dir: PathBuf,
) -> Result<(), Box<dyn Error>> {
    if !output_dir.exists() {
        tokio::fs::create_dir_all(&output_dir).await?;
    }

    let temp_path = output_dir.join(review_name);

    tokio::fs::write(&temp_path, file).await?;
    println!("✅ Diff saved to: {}", &temp_path.display());

    Ok(())
}
