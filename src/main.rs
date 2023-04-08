use cmder::Status;
use colored::Colorize;
use std::process::exit;

mod cliper;
mod cmder;
mod config;
mod gpt;

fn main() {
    // check options
    if let Some(key) = cmder::check_args() {
        match config::save_config(&key) {
            Ok(_) => {
                println!(
                    "{}\n{}",
                    "API key saved successfully.".green(),
                    "We assure that your key would only be stored in your own device."
                );
                exit(0)
            }
            Err(e) => {
                println!("fatal: {}", e.red());
                exit(1);
            }
        }
    }
    // read apikey
    let key = match config::read_config() {
        Ok(key) => key,
        Err(_) => {
            println!("{}", "fatal: API key not set.".red());
            println!(
                "\nusing `{}` to set your chatgpt api key.",
                "rustdiff --set-key=YOUR_API_KEY".green()
            );
            println!("visiting https://platform.openai.com/account/api-keys for more information.");
            exit(0)
        }
    };

    // run `git diff`
    let result = match cmder::run_diff() {
        Ok(diff) => gpt::summarize(&key, diff),
        Err(msg) => {
            println!("fatal: {}", msg.red());
            exit(1)
        }
    };

    // interactions
    print!(
        "Your commit would be:\n\n{}\n\n{}",
        result,
        "Commit? Y(Yes)/C(Copy)/N(No):".blue()
    );

    // actions
    match cmder::check_input() {
        Status::Yes => {
            println!("\n{}", cmder::run_add_commit(result));
        }
        Status::Copy => {
            let commit = format!("git add .;git commit -m \"{result}\"");
            cliper::clip(commit);
            println!(
                "\n{}",
                "Commit message has been copied to your clipboard.".green()
            );
        }
        Status::No => exit(0),
    }
}
