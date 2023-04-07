use std::process::Command;

pub fn run_diff() {
    let output = Command::new("git")
        .args(&["diff"])
        .output()
        .expect("Failed to execut `git diff`\n Make sure git is installed.");
    println!("{:?}", output);
}
