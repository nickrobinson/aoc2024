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
        if (1..=3).contains(&change.abs()) {
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
    fn test_is_safe() {
        assert!(is_safe(&[7, 6, 4, 2, 1])); // Decreasing
        assert!(!is_safe(&[1, 2, 7, 8, 9])); // Invalid step size
        assert!(!is_safe(&[1, 3, 2, 4, 5])); // Direction change
        assert!(is_safe(&[1, 3, 6, 7, 9])); // Increasing
    }

    #[test]
    fn test_is_safe_dampener() {
        assert!(is_safe_dampener(&[1, 3, 2, 4, 5])); // Remove 2
        assert!(!is_safe_dampener(&[1, 2, 7, 8, 9])); // No valid removal
        assert!(is_safe_dampener(&[8, 6, 4, 4, 1])); // Remove 4
    }

    #[test]
    fn test_read_reports() {
        let input = "7 6 4 2 1\n1 3 2 4 5\n";
        let path = "test_input.txt";

        // Create a test file
        std::fs::write(path, input).unwrap();

        let reports = read_reports(path).unwrap();
        assert_eq!(reports, vec![vec![7, 6, 4, 2, 1], vec![1, 3, 2, 4, 5]]);

        // Clean up
        std::fs::remove_file(path).unwrap();
    }
}
