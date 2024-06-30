mod helper;
mod search;
use std::io::{BufReader};
use std::time::{SystemTime};
use std::fs::File;


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
    search::recurrsive_search(reader, &target_file);
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Recursive search took {} milliseconds", duration.as_millis());

    // Re-open and read the search directories file for the concurrent search
    let file = File::open("search_dirs.txt").expect("Failed to open search_dirs.txt");
    let reader = BufReader::new(file);

    // Perform and time the concurrent search
    let start = SystemTime::now();
    search::concurrent_search(reader, &target_file);
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Concurrent search took {} milliseconds", duration.as_millis());
    
}


