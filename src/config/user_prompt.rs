use crate::config::options::Options;

use std::fs;
use std::path::Path;
use toml;

pub fn user_prompt(config_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config_path)?;
    let data: Options = toml::from_str(&contents)?;
    Ok(data.user_prompt)
}
