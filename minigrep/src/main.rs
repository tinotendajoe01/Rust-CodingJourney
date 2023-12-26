use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
// $ cargo run -- to poem.txt > output.txt
// $ IGNORE_CASE=1 cargo run -- to poem.txt
// PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
