// src/search.rs

use std::fs::{self, File};
use std::io::{BufReader, BufRead};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::thread;
use std::collections::VecDeque;

pub fn recurrsive_search(reader: BufReader<File>, target_file: &str) {
    println!("Recursively Searching directories:");
    for line in reader.lines() {
        match line {
            Ok(dir) => {
                let dir = dir.trim();
                println!("Dir: {}", dir);
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


struct SearchState {
    directory_stack: Arc<Mutex<VecDeque<PathBuf>>>,
    found_files: Arc<Mutex<Vec<PathBuf>>>,
    target_file: String,
}

impl SearchState {
    fn new(target_file: &str) -> Self {
        SearchState {
            directory_stack: Arc::new(Mutex::new(VecDeque::new())),
            found_files: Arc::new(Mutex::new(Vec::new())),
            target_file: target_file.to_string(),
        }
    }
}

fn process_directory(directory: &Path, state: &SearchState, thread_id: usize) {
    //println!("Thread {}: Processing directory: {:?}", thread_id, directory);
    
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let mut stack = state.directory_stack.lock().unwrap();
                stack.push_back(path.clone());
                // println!("Thread {}: Added directory to stack: {:?}", thread_id, path);
                // println!("Thread {}: Current stack size: {}", thread_id, stack.len());
            } else if path.is_file() {
                if path.file_name().unwrap().to_str().unwrap() == state.target_file {
                    state.found_files.lock().unwrap().push(path.clone());
                    //println!("Thread {}: Found target file: {:?}", thread_id, path);
                }
            }
        }
    }
}

pub fn concurrent_search(reader: BufReader<File>, target_file: &str) {
    let num_threads = 8;
    let state = Arc::new(SearchState::new(target_file));

    // Read initial directories from the file
    for line in reader.lines().flatten() {
        state.directory_stack.lock().unwrap().push_back(PathBuf::from(line));
    }

    //println!("Initial stack size: {}", state.directory_stack.lock().unwrap().len());

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let state = Arc::clone(&state);

        let handle = thread::spawn(move || {
            loop {
                let directory = {
                    let mut stack = state.directory_stack.lock().unwrap();
                    stack.pop_front()
                };

                match directory {
                    Some(dir) => process_directory(&dir, &state, thread_id),
                    None => {
                        //println!("Thread {}: No more directories to process", thread_id);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Print found files
    let found = state.found_files.lock().unwrap();
    println!("Total files found: {}", found.len());
    for file in found.iter() {
        println!("Found: {:?}", file);
    }
}
