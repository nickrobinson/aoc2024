use anyhow::Result;
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = "inputs/4.txt";
    let results = read_chars(path);
    println!(
        "XMAS occunces: {}",
        count_word_occurrences(&results.as_ref().unwrap(), "XMAS")
    );
    println!("XMAS occunces: {}", count_xmas_patterns(&results.unwrap()));
}

fn count_word_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let ni = i as isize + k as isize * dx;
                    let nj = j as isize + k as isize * dy;

                    if ni < 0 || ni >= rows as isize || nj < 0 || nj >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[ni as usize][nj as usize] != word_chars[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let patterns = vec![
        vec!['M', 'A', 'S'], // "MAS"
        vec!['S', 'A', 'M'], // Reverse "MAS"
    ];

    let rows = grid.len();
    if rows < 3 {
        return 0; // Grid too small for an X-MAS
    }
    let cols = grid[0].len();
    if cols < 3 {
        return 0; // Grid too small for an X-MAS
    }

    let mut count = 0;

    // Iterate over all possible center cells of the X
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Extract diagonals radiating from (i, j)
            let top_left = [
                grid[i - 1][j - 1], // Top-left
                grid[i][j],         // Center
                grid[i + 1][j + 1], // Bottom-right
            ];
            let top_right = [
                grid[i - 1][j + 1], // Top-right
                grid[i][j],         // Center
                grid[i + 1][j - 1], // Bottom-left
            ];

            // Check if both diagonals match a valid pattern
            let matches_top_left = patterns.iter().any(|pattern| {
                pattern[0] == top_left[0] && pattern[1] == top_left[1] && pattern[2] == top_left[2]
            });
            let matches_top_right = patterns.iter().any(|pattern| {
                pattern[0] == top_right[0]
                    && pattern[1] == top_right[1]
                    && pattern[2] == top_right[2]
            });

            if matches_top_left && matches_top_right {
                count += 1;
            }
        }
    }

    count
}

fn read_chars<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<char>>> {
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
