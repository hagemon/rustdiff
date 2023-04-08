use cmder::Status;
use colored::Colorize;
use std::process::exit;

mod cliper;
mod cmder;
mod gpt;

fn main() {
    let result = match cmder::run_diff() {
        Ok(diff) => gpt::summarize(diff),
        Err(msg) => {
            println!("fatal: {}", msg.red());
            exit(1)
        }
    };

    print!(
        "Your commit would be:\n\n{}\n\n{}",
        result,
        "Commit? Y(Yes)/C(Copy)/N(No):".blue()
    );
    match cmder::check_input() {
        Status::Yes => {
            println!("{}", cmder::run_add_commit(result));
        }
        Status::Copy => {
            let commit = format!("git add .;git commit -m \"{result}\"");
            cliper::clip(commit);
            println!("{}", "Commit message has been copied to your clipboard.".green());
        }
        Status::No => exit(0),
    }
}
