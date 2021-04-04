use std::io::Result;
use std::io::{self, Read, Write};

fn read_line() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn split_line(line: String) -> Vec<String> {
    Vec::new()
}

fn execute(args: Vec<String>) -> i32 {
    0
}

fn shell_loop() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let line = read_line().unwrap();
        println!("{:?}", line);
        let args = split_line(line);
        let _status = execute(args);
    }
}

fn main() {
    shell_loop();
}
