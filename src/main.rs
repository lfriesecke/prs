use std::env;
use std::fs;
use std::process;

mod prs_config;

fn main() {
    
    // read command line arguments:
    let args: Vec<String> = env::args().collect();
    let config = prs_config::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // read and print content of specified file:
    let file_content = fs::read_to_string(config.file_path)
        .expect("Error while reading the specified file.");

    for val in file_content.lines() {
        println!("Got: {}", val);
    }
}
