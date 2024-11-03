use crate::config::options::Options;

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use toml;

pub fn user_prompt(config_path: &Path) -> String {
    match fs::read_to_string(config_path) {
        Ok(contents) => match toml::from_str::<Options>(&contents) {
            Ok(config) => config.user_prompt,
            Err(_) => String::from("$|> "),
        },
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                String::from("$|> ")
            } else {
                String::from("$|> ")
            }
        }
    }
}
