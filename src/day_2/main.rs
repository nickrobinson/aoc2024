use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<()> {
    // Define the file path
    let path = "inputs/2.txt";
    let results = read_reports(path);

    results.map(|reports| {
        let mut safe_count = 0;
        for report in reports {
            if is_safe_dampener(&report) {
                safe_count += 1;
                println!("Safe report: {:?}", report);
            } else {
                println!("Unsafe report: {:?}", report);
            }
        }
        println!("Safe reports: {}", safe_count);
    })?;

    Ok(())
}

fn is_safe_dampener(result: &[i32]) -> bool {
    if is_safe(result) {
        return true;
    }

    // Remove one element at a time and check if the result is safe
    // If it is, return true
    for i in 0..result.len() {
        if remove_and_check(result, i) {
            return true;
        }
    }

    false
}

fn remove_and_check(result: &[i32], index: usize) -> bool {
    let mut result = result.to_vec();
    println!("Removing: {}", result[index]);
    result.remove(index);
    if is_safe(&result) {
        return true;
    }
    false
}

fn is_safe(result: &[i32]) -> bool {
    let mut prev = result.first();
    let mut direction: Option<i32> = None;

    for el in result.iter().skip(1) {
        let change = el - prev.unwrap();
        if (1..4).contains(&change.abs()) {
            if direction.is_none() {
                direction = Some(change.signum());
            }

            // Check if the direction is consistent
            if direction != Some(change.signum()) {
                return false;
            }

            prev = Some(el);
        } else {
            return false;
        }
    }
    true
}

fn read_reports<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<i32>>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize the 2D vector for reports
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Propagate I/O error

        // Split the line into words and parse them into integers
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|num| num.parse()) // Parse each part into i32
            .collect::<Result<Vec<_>, _>>()?; // Collect and propagate any parse error

        // Add the parsed row to the reports
        reports.push(numbers);
    }

    Ok(reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_test() {
        let result = vec![48, 51, 54, 56];
        assert!(is_safe(&result));
    }

    #[test]
    fn is_safe_dampener_test() {
        let result = vec![48, 51, 54, 56, 60];
        assert!(is_safe_dampener(&result));
    }
}
