use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<()> {
    // Part 1
    let rules_path = "inputs/day_5_rules.txt";
    let rules = read_rules(rules_path)?;
    println!("Rules: {:?}", rules);

    let updates_path = "inputs/day_5_updates.txt";
    let updates = read_updates(updates_path)?;
    println!("Updates: {:?}", updates);

    let mut total = 0;
    for update in &updates {
        if is_valid_update(&rules, &update) {
            // println!("Valid update: {:?}", update);
            // Add the value of the middle element to the total
            total += update[update.len() / 2];
        } else {
            // println!("Invalid update: {:?}", update);
        }
    }
    println!("Total: {}", total);

    // Part 2
    let invalid_updates = &updates
        .iter()
        .filter(|update| !is_valid_update(&rules, update))
        .collect::<Vec<&Vec<i32>>>();

    let mut total = 0;
    for update in invalid_updates {
        let mut valid = false;
        let mut new_update = update.to_vec();

        while !valid {
            let mut prev = HashMap::new();
            for i in 0..new_update.len() {
                if prev.is_empty() {
                    prev.insert(new_update[i], 0);
                    continue;
                }

                for rule in &rules {
                    if rule.0 == new_update[i] && prev.contains_key(&rule.1) {
                        println!("Rule: {:?}", rule);
                        println!("Prev: {:?}", prev);

                        new_update[i] = rule.1;
                        new_update[prev[&rule.1] as usize] = rule.0;
                        println!("New update: {:?}", new_update);
                        break;
                    }
                }
                prev.insert(new_update[i], i as i32);
            }
            println!("New update: {:?}", new_update);
            valid = is_valid_update(&rules, &new_update);
            if valid {
                println!("Valid update: {:?}", new_update);
                total += new_update[new_update.len() / 2];
                break;
            }
        }
    }
    println!("Part 2 Total: {}", total);

    Ok(())
}

fn is_valid_update(rules: &[(i32, i32)], updates: &[i32]) -> bool {
    let mut prev: Vec<i32> = Vec::new();
    for &num in updates {
        if prev.is_empty() {
            prev.push(num);
            continue;
        }

        for r in rules {
            if r.0 == num && prev.contains(&r.1) {
                return false;
            }
        }
        prev.push(num);
    }
    true
}

fn read_rules<P: AsRef<Path>>(file_path: P) -> Result<Vec<(i32, i32)>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize the 2D vector for reports
    let mut reports: Vec<(i32, i32)> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Propagate I/O error

        // Split the line into words and parse them into integers
        let mut parts = line.split('|');
        let first = parts.next().unwrap().parse::<i32>().unwrap();
        let second = parts.next().unwrap().parse::<i32>().unwrap();

        // Add the parsed row to the reports
        reports.push((first, second));
    }

    Ok(reports)
}

fn read_updates<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<i32>>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize the 2D vector for reports
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        reports.push(
            line.unwrap()
                .trim() // Remove leading/trailing whitespace
                .split(',') // Split by commas
                .filter_map(|num| num.parse::<i32>().ok()) // Parse to i32, ignore invalid numbers
                .collect::<Vec<i32>>(),
        )
    }

    Ok(reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_update() {
        let update = [61, 13, 29];
        let rules = [(29, 13)];
        assert!(!is_valid_update(&rules, &update))
    }
}
