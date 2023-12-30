use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Remove the first argument, which is the path to the executable
    if let Some(string) = args.get_mut(0) {
        *string = string.replace("file_reader", "");
    }
    let mut input = String::new();
    println!("Enter a file name in this directory: {}", args[0]);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    let path = format!("{}{}", args[0],input.trim());

    println!("Path: {}", &path);
    let file = File::open(&path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
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
