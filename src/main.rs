use std::env;
use std::fs;

fn main() {
    // read command line arguments:
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // read and print content of specified file:
    let file_content = fs::read_to_string(file_path)
        .expect("Something went wrong.");

    println!("File content:\n{file_content}")
}
