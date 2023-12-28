use clap::{App, Arg};
use std::{io, process};

mod grep;

fn main() {
    let matches = App::new("clui")
        .version("1.0")
        .author("Tinotenda Joe")
        .about("Grep app")
        .arg(
            Arg::with_name("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("Sets the pattern to search for")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the files to search")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap_or_else(|| {
        eprintln!("No pattern provided");
        process::exit(1);
    });

    let files: Vec<String> = matches
        .values_of("file")
        .unwrap_or_else(|| {
            eprintln!("No file provided");
            process::exit(1);
        })
        .map(|s| s.to_string())
        .collect();

    println!("Do you want to perform a case-insensitive search? (yes/no)");
    let mut ignore_case_input = String::new();
    io::stdin()
        .read_line(&mut ignore_case_input)
        .expect("Failed to read line");
    let ignore_case = ignore_case_input.trim().eq_ignore_ascii_case("yes");

    println!("Do you want to output results to a file (output.txt)? (yes/no)");
    let mut output_to_file_input = String::new();
    io::stdin()
        .read_line(&mut output_to_file_input)
        .expect("Failed to read line");
    let output_to_file = output_to_file_input.trim().eq_ignore_ascii_case("yes");

    if let Err(e) = grep::grep(&pattern, files, ignore_case, output_to_file) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
