use std::process::exit;

mod cmder;
mod gpt;
mod cliper;

fn main() {
    let result = match cmder::run_diff() {
        Ok(info) => gpt::summarize(info),
        Err(msg) => panic!("{msg}")
    };

    println!("Your commit would be:\n\n{}\n\nCommit? Y(Yes)/C(Copy)/N(No):", result);
    let commit = format!("git commit -m \"{result}\"");
    match cmder::check_input() {
        cmder::Status::Yes => {},
        cmder::Status::Copy => cliper::clip(commit),
        cmder::Status::No => {exit(0)}
    }

}
