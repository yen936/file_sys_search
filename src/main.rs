mod helper;

use std::time::{SystemTime};
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use std::io::{self, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::path::PathBuf;
//use rayon;
use rayon::prelude::*;

// TODO:
// Edgecases: permission issues, write out location (josn?)

fn main() {
    println!("Welcome to the efficient share_drive open search: ESOD!");

    // Get user input for the target file
    let target_file = helper::get_user_input("Enter the file to search for: ");

    // Open and read the search directories file
    let file = File::open("search_dirs.txt").expect("Failed to open search_dirs.txt");
    let reader = BufReader::new(file);

    // Perform and time the recursive search
    let start = SystemTime::now();
    recurrsive_search(reader, &target_file);
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Recursive search took {} milliseconds", duration.as_millis());

    // Re-open and read the search directories file for the concurrent search
    let file = File::open("search_dirs.txt").expect("Failed to open search_dirs.txt");
    let reader = BufReader::new(file);

    // Perform and time the concurrent search
    let start = SystemTime::now();
    concurrent_search(reader, &target_file);
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Concurrent search took {} milliseconds", duration.as_millis());
    
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
        }
    }

}


fn concurrent_search(reader: BufReader<File>, target_file: &str) {
    println!("Concurrently Searching directories:");

    let dirs: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let target_file = Arc::new(target_file.to_string());
    let found_files = Arc::new(Mutex::new(Vec::new()));

    dirs.par_iter().for_each(|dir| {
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .par_bridge()
            .for_each(|entry| {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if filename == target_file.as_str() {
                            let mut found_files = found_files.lock().unwrap();
                            found_files.push(path.display().to_string());
                        }
                    }
                }
            });
    });

    // Print out all found files (this could be written to a CSV or any other output)
    let found_files = found_files.lock().unwrap();
    for file in found_files.iter() {
        println!("{}", file);
    }
}