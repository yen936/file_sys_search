// src/search.rs

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use walkdir::WalkDir;

pub fn recurrsive_search(reader: BufReader<File>, target_file: &str) {
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

pub fn search_files_recursively(dir: &str, target_file: &str) {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if filename == target_file {
                    println!("Found {} at this path {}", filename.to_str().unwrap(), path.display());
                }
            }
        }
    }
}

pub fn concurrent_search(reader: BufReader<File>, target_file: &str) {
    println!("Concurrently Searching directories:");

    let dirs: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let target_file = Arc::new(target_file.to_string());
    let found_files = Arc::new(Mutex::new(Vec::new()));

    // Customize ThreadPoolBuilder to set desired number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4) // Adjust the number of threads as needed
        .build()
        .unwrap();

    pool.install(|| {
        dirs.par_iter().for_each(|dir| {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
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
    });

    // Print out all found files (this could be written to a CSV or any other output)
    let found_files = found_files.lock().unwrap();
    for file in found_files.iter() {
        println!("{}", file);
    }
}
