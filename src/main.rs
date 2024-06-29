mod file_meta;
//use file_meta::FileMetaData; 
//use chrono::{DateTime, Utc};
use std::time::{SystemTime};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use std::io::{self, Write};

// TODO:
// Edgecases: permission issues, write out location (josn?)

fn main() {
    println!("Welcome to the efficient share_drive open search: ESOD!");
    // Search 
    let tgt_file = get_user_input("Enter the file to search for: ");
    let start = SystemTime::now();

    let file = File::open("search_dirs.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    recurrsive_search(reader, &tgt_file);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} milliseconds", duration.as_millis());
    
}

fn recurrsive_search(reader: BufReader<File>, target_file: &str) {
    println!("Recursively Searching directories:");
    for line in reader.lines() {
        match line {
            Ok(dir) => {
                let dir = dir.trim();
                search_files_recursively(dir, &target_file);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn search_files_recursively(dir: &str, target_file: &str) {
    // Start a for loop that iterates over the directory entries
    for entry in WalkDir::new(dir)  // Create a new WalkDir instance for the given directory
        .into_iter()  // Convert WalkDir into an iterator
        .filter_map(|e| e.ok()) {  // Filter out errors and unwrap Ok values
        
        let path = entry.path();
        if path.is_file() {
            let path = Path::new(path);
            let filename = path.file_name().unwrap();
            if filename == target_file {
                println!("Found {} at this path {}", filename.to_str().unwrap(), path.display());
            }
            //println!("  {}", path.display());
        }
    }

}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}