use dirs::home_dir;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, execvp, fork, ForkResult};
use std::env;
use std::ffi::{CStr, CString};
use std::io::Result;
use std::io::{self, Write};
use std::path::PathBuf;

// TODO: Remove .unwraps()

fn change_directory(args: Vec<&CStr>) -> i32 {
    if let 1 = args.len() {
        let home = match home_dir() {
            Some(value) => String::from(value.to_string_lossy()),
            None => return 1,
        };
        let _ = chdir(home.as_bytes());
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

fn read_line() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
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

    match args[0].to_str().unwrap() {
        "cd" => change_directory(args),
        _ => launch(args),
    }
}

fn launch(args: Vec<&CStr>) -> i32 {
    let fork_result = unsafe { fork() };

    if let Ok(ForkResult::Child) = fork_result {
        if let Err(_) = execvp(args[0], &args) {
            println!("'{}': command not found", args[0].to_string_lossy());
        };
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

fn shell_loop() {
    loop {
        let cwd = get_current_directory();
        let prompt = format!("{} $ ", cwd.to_string_lossy());

        print!("{}", prompt);
        let _ = io::stdout().flush();

        let line = read_line().unwrap();

        if line.is_empty() {
            continue;
        }

        let args = split_line(&line);
        let args = args.iter().map(|c| c.as_c_str()).collect();

        execute(args);
    }
}

fn main() {
    shell_loop();
}
