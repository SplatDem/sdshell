use crate::config::options::Options;

use std::env;
use std::fs;
use std::path::Path;

pub fn greet() {
    let home_dir_path = match env::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };

    let config_path_str = format!("{}/.config/sdshell/sdsh.toml", home_dir_path);
    let config_path = Path::new(&config_path_str);

    let contents = fs::read_to_string(config_path).unwrap();
    let data: Options = toml::from_str(&contents).unwrap();

    let greet = data.greet;

    println!("{}", greet);
}
