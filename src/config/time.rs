use crate::config::options::Options;

use std::env;
use std::fs;
use std::path::Path;

use chrono::prelude::*;

fn get_time() -> String {
    let home_dir_path = match env::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };

    let config_paths_str = format!("{}/.config/sdshell/sdsh.toml", home_dir_path);
    let config_path = Path::new(&config_paths_str);

    let contents = fs::read_to_string(config_path).unwrap();
    let data: Options = toml::from_str(&contents).unwrap();

    Local::now().format("%H:%M:%S").to_string()
}

/*fn main() {
    loop {
        let time_str = get_time();
        println!("{}", time_str);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}*/
