use anyhow::Result;
use std::char;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_chars<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<char>>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize the 2D vector for reports
    let mut reports: Vec<Vec<char>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Propagate I/O error

        // Split the line into words and parse them into integers
        let chars: Vec<char> = line.chars().collect();

        // Add the parsed row to the reports
        reports.push(chars);
    }

    Ok(reports)
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
