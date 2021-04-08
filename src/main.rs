use dirs::home_dir;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, execvp, fork, ForkResult};
use std::collections::HashMap;
use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::path::PathBuf;

mod load_config;
mod parse;

use load_config::check_path;
use load_config::load_config;

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

fn process_prompt(prompt: String) -> String {
    let mut processed_prompt = prompt;

    let home = home_dir().unwrap();
    let home = match home.to_str() {
        Some(directory) => directory,
        None => "",
    };

    let prompt_aliases = vec![(home, "~")];

    for (value_to_replace, alias) in prompt_aliases {
        processed_prompt = processed_prompt.replace(value_to_replace, alias);
    }

    processed_prompt
}

fn shell_loop(config: &HashMap<String, String>) {
    loop {
        let current_directory = get_current_directory();
        let prompt = format!("{} $ ", current_directory.to_string_lossy());

        let processed_prompt = process_prompt(prompt);

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

fn main() {
    let config = load_config();
    shell_loop(&config);
}
