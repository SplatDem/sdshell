use crate::config::{clock, user_prompt::*};

use rustyline::Editor;

use std::sync::{Arc, Mutex};
use std::{env, path::Path, process::Command};

pub fn command_line() -> Result<(), Box<dyn std::error::Error>> {
    let is_running = Arc::new(Mutex::new(true));
    let running = is_running.clone();

    let mut rl = Editor::<()>::new().unwrap();
    let mut command_history: Vec<String> = Vec::new();

    ctrlc::set_handler(move || {
        let mut running = running.lock().unwrap();
        *running = true;
    })
    .expect("> Failed to get signal");

    while *is_running.lock().unwrap() {
        let home_dir_path = match env::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => "/".to_string(),
        };

        let config_path_str = format!("{}/.config/sdshell/sdsh.toml", home_dir_path);
        let config_path = Path::new(&config_path_str);

        let user_prompt = user_prompt(config_path);

        let prompt = rl.readline(&user_prompt);

        match prompt {
            Ok(command) => {
                command_history.push(command.clone());
                rl.add_history_entry(command.clone());

                let command = command.trim(); // To destroy a new line symbol

                // Vector with command line arguments
                let args: Vec<&str> = command.split_whitespace().collect();

                if command.is_empty() {
                    continue;
                }

                if command.starts_with("cd") {
                    let path = if args.len() > 1 {
                        args[1]
                    } else {
                        &match env::home_dir() {
                            Some(path) => path.to_string_lossy().into_owned(),
                            None => "/".to_string(),
                        }
                    };

                    if let Err(e) = env::set_current_dir(&path) {
                        eprintln!("cd: {}", e);
                    }
                    continue;
                }

                if command == "exit" {
                    break;
                }

                // Aliases
                if command == "la" || command == "ll" || command == "l" {
                    let mut list = Command::new("ls").arg("-l").spawn().unwrap();

                    let _ = list.wait().expect("> Failed to execute command");
                    continue;
                }
                if command == "cls" || command == "c" {
                    let mut clear_screen = Command::new("clear").spawn().unwrap();

                    let _ = clear_screen.wait().expect("> Failed to execute command");
                    continue;
                }

                let status = Command::new(args[0]).args(&args[1..]).status();
                clock::show_clock();

                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("> Failed to execute command: {}", command);
                        }
                    }
                    Err(_) => {
                        eprintln!("> Failed to execute command: {}", command);
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok(())
}
