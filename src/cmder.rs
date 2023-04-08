use std::io;
use std::process::Command;

pub enum Status {
    Yes,
    Copy,
    No,
}

pub fn run_diff() -> Result<String, String> {
    let output = Command::new("git")
        .args(&["diff"])
        .output()
        .expect("Git not installed or git respository not found.");
    let result = String::from_utf8_lossy(&output.stdout);
    if result.len() == 0 {
        Err(String::from("No change found in respository."))
    } else {
        Ok(result.to_string())
    }
}

pub fn check_input() -> Status {
    let flag: Status;
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
