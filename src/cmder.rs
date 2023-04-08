use std::env;
use std::io;
use std::io::Write;
use std::process::Command;

pub enum Status {
    Yes,
    Copy,
    No,
}

pub fn run_diff() -> Result<String, String> {
    let output = Command::new("git").args(&["diff"]).output();
    // println!("{:?}", output);
    match output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if result.len() == 0 {
                if stderr.len() > 0 {
                    Err(String::from("not a git respository."))
                } else {
                    Err(String::from("no diff found in respository."))
                }
            } else {
                Ok(result.to_string())
            }
        }
        Err(_) => Err(String::from("git not installed")),
    }
}

pub fn run_add_commit(msg: String) -> String {
    Command::new("git").args(&["add", "."]);
    let output = Command::new("git")
        .args(&["commit", "-l", &msg])
        .output()
        .expect("Failed to execute `git commit`");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn check_input() -> Status {
    let flag: Status;
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "Y" | "y" | "" => {
                flag = Status::Yes;
                break;
            }
            "N" | "n" => {
                flag = Status::No;
                break;
            }
            "C" | "c" => {
                flag = Status::Copy;
                break;
            }
            _ => println!("Invalid command, press again"),
        }
    }
    flag
}

pub fn check_args() -> Option<String> {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(arg) => {
            if let Some(key) = arg.split('=').last() {
                return Some(String::from(key));
            } else {
                return None;
            }
        }
        None => None,
    }
}
