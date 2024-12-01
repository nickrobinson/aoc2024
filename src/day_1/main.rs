use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<()> {
    // Define the file path
    let path = "inputs/1.txt";

    // Call the function to read the columns into separate vectors
    let (left_column, right_column) = read_lists(path)?;

    println!(
        "Distance: {}",
        calculate_distance(&left_column, &right_column)
    );
    println!(
        "Similarity: {}",
        calculate_similarity(&left_column, &right_column)
    );

    Ok(())
}

fn calculate_similarity(left_column: &[i32], right_column: &[i32]) -> i32 {
    // Avoid duplicative sorting of right column
    let mut sorted_right: Vec<i32> = right_column.to_vec();
    sorted_right.sort();

    let mut similarity = 0;
    for left in left_column.iter().sorted() {
        let mut occurunces = 0;
        for right in sorted_right.iter() {
            match left.cmp(right) {
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Equal => occurunces += 1,
                std::cmp::Ordering::Greater => continue,
            }
        }
        similarity += left * occurunces;
    }
    similarity
}

fn calculate_distance(left_column: &[i32], right_column: &[i32]) -> i32 {
    let mut distance = 0;
    for (left, right) in left_column
        .iter()
        .sorted()
        .zip(right_column.iter().sorted())
    {
        distance += (left - right).abs();
    }
    distance
}

// Function to read numbers into two separate vectors
fn read_lists<P: AsRef<Path>>(file_path: P) -> Result<(Vec<i32>, Vec<i32>)> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize vectors for the two columns
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Propagate I/O error
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Parse and push values into respective vectors
        if let (Some(left), Some(right)) = (numbers.first(), numbers.get(1)) {
            left_column.push(left.parse::<i32>()?);
            right_column.push(right.parse::<i32>()?);
        }
    }

    Ok((left_column, right_column))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_distance(&left, &right), 11);
    }

    #[test]
    fn similarity() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_similarity(&left, &right), 31);
    }
}
