mod file_meta;
use file_meta::FileMetaData; 
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

// TODO:
// Edgecases: permission issues, write out location (josn?)

fn main() {
    println!("Welcome to the efficient share_drive open search: ESOD!");

    let last_mod: DateTime<Utc> = Utc::now();
    let file = FileMetaData::new("example.txt".to_string(), 1024, last_mod);
    file.print_details();

    let file = File::open("search_dirs.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    println!("Searching directories:");
    for line in reader.lines() {
        match line {
            Ok(dir) => {
                let dir = dir.trim();
                println!("Directory: {}", dir);
                search_files_recursively(dir);
                println!();
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

// Define a function that takes a reference to a string slice as an argument
fn search_files_recursively(dir: &str) {
    // Start a for loop that iterates over the directory entries
    for entry in WalkDir::new(dir)  // Create a new WalkDir instance for the given directory
        .into_iter()  // Convert WalkDir into an iterator
        .filter_map(|e| e.ok()) {  // Filter out errors and unwrap Ok values
        
        let path = entry.path();
        if path.is_file() {
            println!("  {}", path.display());
        }
    }
    
}