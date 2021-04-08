use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

use std::collections::HashMap;

use crate::parse;

pub fn check_path(path: PathBuf) -> bool {
    Path::new(path.as_path()).exists()
}

fn find_config_file() -> Option<PathBuf> {
    let home_dir = home_dir()?;
    let config_file_name = ".shillrc";
    let paths = vec![home_dir.to_str().unwrap(), "~/.config"];

    for path in paths {
        let mut path = PathBuf::from(path);
        path.push(config_file_name);
        if let true = check_path(path.clone()) {
            return Some(path);
        }
    }

    None
}

fn read_config_file() -> Option<String> {
    let config_path = match find_config_file() {
        Some(value) => value,
        None => return None,
    };

    let content = match fs::read_to_string(config_path) {
        Ok(value) => value,
        Err(_) => return None,
    };

    Some(content)
}

pub fn load_config() -> HashMap<String, String> {
    let config_content = read_config_file();
    if let Some(content) = config_content {
        parse::parse_config(content).unwrap()
    } else {
        HashMap::new()
    }
}
