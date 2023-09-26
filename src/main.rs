use std::env;
use std::fs;
use std::vec;

use content_elements::ContentElement;

mod markdown_parser;
mod content_elements;

fn main() {
    
    // read command line arguments:
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // read and print content of specified file:
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the specified file.");

    for val in file_content.lines() {
        println!("Got: {}", val);
    }

    // test simple content element implementation:
    let heading = content_elements::Heading {
        headline: String::from("First headline"),
        level: 1,
    };
    let text = content_elements::Text {
        text: String::from("This is a simple test\nto check if:")
    };
    let unordered_list = content_elements::UnorderedList {
        items: vec![
            String::from("simple markdown formatting works"),
            String::from("meta data work"),
            String::from("something unexpected happens")
        ]
    };
    let ordered_list = content_elements::OrderedList {
        items: vec![
            String::from("simple markdown formatting works"),
            String::from("meta data work"),
            String::from("something unexpected happens")
        ]
    };

    let content_elements: Vec<Box<dyn ContentElement>> = vec![
        Box::new(heading), 
        Box::new(text), 
        Box::new(unordered_list), 
        Box::new(ordered_list)
    ];

    for elem in content_elements {
        elem.print()
    }
}
