mod file_meta;
use file_meta::FileMetaData; 
// use std::fs::File;
// use std::io::{BufRead, BufReader};
use chrono::{DateTime, Utc};

fn main() {
    println!("Hello, world!");

    let last_mod: DateTime<Utc> = Utc::now();
    let file = FileMetaData::new("example.txt".to_string(), 1024, last_mod);
    file.print_details();

    //let source_dirs_file = File
}