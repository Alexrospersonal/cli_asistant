use std::error::Error;
use std::path::PathBuf;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::FileHistory;
use crate::cli::args::Args;
use crate::cli::command_enum::Commands;
use crate::commands::dispatcher::{dispatch, dispatch_no_repl};

pub async fn run_loop() -> Result<(), Box<dyn Error>> {
    // TODO: рефактор коду
    println!("🔁 REPL is running...");

    let history_path = dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cli_assistant_history");
    
    let mut editor: Editor<(), FileHistory> = Editor::new()?;
    
    let _ = editor.load_history(&history_path);
    
    loop {
        let readline = editor.readline(">> ");
        
        match readline {
            Ok(line) => {
                editor.add_history_entry(&line)?;
                
                if line.trim() == "exit" || line.trim()  == "quit" {
                    break
                }
                let args: Vec<String> = shell_words::split(&line)?;
                
                let mut full_args = vec!["cli-assistant".to_string()];
                full_args.extend(args);
                // let args = Args::parse_from(full_args);
                
                match Args::try_parse_from(full_args) { 
                    Ok(args) => {
                        if let Some(command) = args.command {
                            if let Commands::Repl = command {
                                println!("🔁 You are in REPL");
                                continue;
                            }
                            if let Err(e) = dispatch_no_repl(command).await {
                                eprintln!("❌ Error: can't execute command: {e}");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("❗Parsing error command: {err}")
                    }
                }
                println!("Entered line: {line}");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    
    let _ = editor.save_history(&history_path);
    Ok(())
}