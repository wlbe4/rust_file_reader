use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        panic!("Usage: {} <file_name>", args[0]);
    }
    let file_name = args[1].clone();
    let file = File::open(&file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File {} not found: {}", file_name, error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
