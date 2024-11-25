use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn is_file_empty(filename: &str) -> bool {
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(_) => return true, // File doesn't exist or can't be opened
    };

    let mut buffer = [0; 1024];
    match file.read(&mut buffer) {
        Ok(0) => true, // File is empty
        Ok(_) => false, // File has content
        Err(_) => true, // Error reading file, assume it's empty
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("\nUse format as: cargo run filename.extension.\n");
        return;
    }

    let filename = &args[1];

    if Path::new(filename).exists() {
        if is_file_empty(filename) {
            println!("\nFilename '{}' is empty. Deleting...\n", filename);
        } else {
            println!("\nFilename '{}' contains data. Are you sure you want to delete it? (y/n)", filename);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("\nFailed to read input\n");
            if input.trim().to_lowercase() != "y" {
                println!("\nDeletion canceled.\n");
                return;
            }
        }

        match fs::remove_file(filename) {
            Ok(_) => println!("\nFilename '{}' removed successfully.\n", filename),
            Err(e) => println!("\nError removing filename: {}\n", e),
        }
    } else {
        println!("\nFilename '{}' does not find.\n", filename);
    }
}