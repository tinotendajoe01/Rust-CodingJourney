use colored::*;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, Write},
    sync::mpsc,
    thread,
};

pub fn grep(
    pattern: &str,
    files: Vec<String>,
    ignore_case: bool,
    output_to_file: bool,
) -> Result<(), Box<dyn Error>> {
    //Regex Setup
    let re = if ignore_case {
        Regex::new(&format!("(?i){}", pattern))?
    } else {
        Regex::new(pattern)?
    };
    //channel and thread setup
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    //Loop to Spawn Threads

    for file in files {
        let tx = tx.clone();
        let re = re.clone();
        let file_clone = file.clone();

        // thread logic
        let handle = thread::spawn(move || {
            if let Ok(file) = File::open(&file_clone) {
                let reader = io::BufReader::new(file);
                let mut lines = vec![];

                // ... processing lines ...
                for line in reader.lines() {
                    let line = line.unwrap();
                    lines.push(line);
                    if lines.len() >= 1000 {
                        // Chunk size
                        let tx = tx.clone();
                        let re = re.clone();
                        let chunk = lines.clone();
                        lines.clear();
                        search_chunk(chunk, &re, &tx, &file_clone);
                    }
                }

                if !lines.is_empty() {
                    search_chunk(lines, &re, &tx, &file_clone);
                }
            }
        });
        handles.push(handle);
    }

    let mut output: Box<dyn Write> = if output_to_file {
        Box::new(File::create("output.txt")?) as Box<dyn Write>
    } else {
        Box::new(io::stdout()) as Box<dyn Write>
    };

    let mut any_match_found = false;
    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx); // Close the channel

    for received in rx {
        writeln!(output, "{}", received)?;
        any_match_found = true;
    }

    if !any_match_found {
        writeln!(
            output,
            "No matches found. Try other words, files, or select case sensitive."
        )?;
    }

    Ok(())
}

fn search_chunk(chunk: Vec<String>, re: &Regex, tx: &mpsc::Sender<String>, filename: &str) {
    for line in chunk {
        if re.is_match(&line) {
            let matched = re.find(&line).unwrap();
            let colored_match = &line[matched.start()..matched.end()].green().to_string();
            let result = format!(
                "{} (in {}): {}{}",
                &line[..matched.start()],
                filename,
                colored_match,
                &line[matched.end()..]
            );
            tx.send(result).unwrap();
        }
    }
}
