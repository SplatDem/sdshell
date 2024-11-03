use crate::config::options::Options;

use chrono::prelude::*;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

fn check_clock() -> bool {
    let home_dir_path = match env::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };

    let config_path_str = format!("{}/.config/sdshell/sdsh.toml", home_dir_path);
    let config_path = Path::new(&config_path_str);

    match fs::read_to_string(config_path) {
        Ok(contents) => match toml::from_str::<Options>(&contents) {
            Ok(config) => config.clock,
            Err(_) => false,
        },
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                false
            } else {
                false
            }
        }
    }
}

pub fn show_clock() {
    match check_clock() {
        true => {
            let home_dir_path = match env::home_dir() {
                Some(path) => path.to_string_lossy().into_owned(),
                None => "/".to_string(),
            };

            let config_path_str = format!("{}/.config/sdshell/sdsh.toml", home_dir_path);
            let config_path = Path::new(&config_path_str);

            let contents = fs::read_to_string(config_path).unwrap();
            let config: Options = toml::from_str(&contents).unwrap();

            if config.clock {
                let local_time = Local::now();
                println!("{}", local_time.format("| %H:%M:%S | %Y-%m-%d | "));
            }
        }
        false => {}
    }
}
