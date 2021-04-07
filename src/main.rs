use dirs::home_dir;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, execvp, fork, ForkResult};
use std::env;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

mod parse;

use std::collections::HashMap;

fn change_directory(args: Vec<&CStr>) -> i32 {
    if let 1 = args.len() {
        let home = match home_dir() {
            Some(value) => String::from(value.to_string_lossy()),
            None => return 1,
        };
        let _ = chdir(home.as_bytes());
    } else if !check_path(PathBuf::from(args[1].to_string_lossy().into_owned())) {
        println!("'{}': No such file or directory", args[1].to_string_lossy());
    } else {
        let _ = chdir(args[1]);
    }
    1
}

fn get_current_directory() -> PathBuf {
    match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => PathBuf::new(),
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

fn split_line(line: &str) -> Vec<CString> {
    line.split(' ')
        .map(|s| CString::new(s.as_bytes()).unwrap())
        .collect()
}

fn execute(args: Vec<&CStr>) -> i32 {
    if let 0 = args.len() {
        return 1;
    }

    match args[0].to_string_lossy().as_ref() {
        "cd" => change_directory(args),
        _ => launch(args),
    }
}

fn launch(args: Vec<&CStr>) -> i32 {
    let fork_result = unsafe { fork() };

    if let Ok(ForkResult::Child) = fork_result {
        if execvp(args[0], &args).is_err() {
            println!("'{}': command not found", args[0].to_string_lossy());
        }
    };

    if let Ok(ForkResult::Parent { child, .. }) = fork_result {
        loop {
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(_, _)) => break,
                Ok(WaitStatus::Signaled(_, _, _)) => break,
                _ => {}
            };
        }
    };
    1
}

fn process_line(args: String, config: &HashMap<String, String>) -> String {
    let mut processed_line = args;
    for (key, value) in config {
        processed_line = processed_line.replace(key, value);
    }

    processed_line
}

fn shell_loop(config: &HashMap<String, String>) {
    loop {
        let current_directory = get_current_directory();
        let prompt = format!("{} $ ", current_directory.to_string_lossy());

        let processed_prompt = process_line(prompt, config);

        print!("{}", processed_prompt);
        let _ = io::stdout().flush();

        let line = read_line();

        if line.is_empty() {
            continue;
        }

        let processed_line = process_line(line, config);

        let args = split_line(&processed_line);
        let args = args.iter().map(|c| c.as_c_str()).collect();

        execute(args);
    }
}

fn check_path(path: PathBuf) -> bool {
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

fn load_config() -> HashMap<String, String> {
    let config_content = read_config_file();
    if let Some(content) = config_content {
        parse::parse_config(content).unwrap()
    } else {
        HashMap::new()
    }
}

fn main() {
    let config = load_config();
    shell_loop(&config);
}
