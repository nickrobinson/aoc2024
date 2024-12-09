use aoc2024::read_chars;
use std::collections::HashSet;

fn main() {
    let path = "inputs/8.txt";
    let city_map = read_chars(path).unwrap();
    let frequencies = find_frequencies(&city_map);
    println!("Frequencies: {:?}", frequencies);

    // Loop through the frequencies and find pairs
    for frequency in frequencies.iter() {
        let mut pair = 0;
        for i in city_map.iter() {
            for j in i.iter() {
                if *j == *frequency {
                    pair += 1;
                }
            }
        }
        println!("Frequency {}: {}", frequency, pair);
    }

    let mut found_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for frequency in frequencies.iter() {
        let antinodes = find_antinodes(&city_map, *frequency);
        antinodes.iter().for_each(|antinode| {
            found_antinodes.replace(*antinode);
        });
    }

    println!("Antinodes: {}", found_antinodes.len());

    // Part two
    let mut found_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for frequency in frequencies.iter() {
        let antinodes = find_antinodes_part_two(&city_map, *frequency);
        antinodes.iter().for_each(|antinode| {
            found_antinodes.replace(*antinode);
        });
    }
    println!("Antinodes Part Two: {}", found_antinodes.len());
}

fn find_antinodes(city_map: &[Vec<char>], frequency: char) -> HashSet<(i32, i32)> {
    // Find positions of frequency
    let mut positions: Vec<(i32, i32)> = Vec::new();
    for (i, row) in city_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == frequency {
                positions.push((i as i32, j as i32));
            }
        }
    }

    println!("Positions: {:?}", positions);
    let mut found_antinodes: HashSet<(i32, i32)> = HashSet::new();

    let mut cursor = 1;
    for (i, j) in positions.iter() {
        for (k, l) in positions.iter().skip(cursor) {
            println!("({}, {}) -> ({}, {})", i, j, k, l);
            // Find delta
            let (dx, dy) = ((k - i).abs(), (l - j));
            println!("Delta: ({}, {})", dx, dy);
            if dy > 0 {
                // Check up and to the left
                if (i - dx) >= 0 && (j - dy) >= 0 {
                    println!("1. Antinode found at ({}, {})", i - dx, j - dy);
                    found_antinodes.replace((i - dx, j - dy));
                }
                if (k + dx) < city_map.len() as i32 && (l + dy) < city_map[0].len() as i32 {
                    println!("2. Antinode found at ({}, {})", k + dx, l + dy);
                    found_antinodes.replace((k + dx, l + dy));
                }
            } else {
                // Check up and to the right
                if (i - dx) >= 0 && (j - dy) < city_map[0].len() as i32 {
                    println!("3. Antinode found at ({}, {})", i - dx, j - dy);
                    found_antinodes.replace((i - dx, j - dy));
                }
                if (k + dx) < city_map.len() as i32 && (l + dy) >= 0 {
                    println!("4. Antinode found at ({}, {})", k + dx, l + dy);
                    found_antinodes.replace((k + dx, l + dy));
                }
            }
        }
        cursor += 1;
    }

    found_antinodes
}

fn find_antinodes_part_two(city_map: &[Vec<char>], frequency: char) -> HashSet<(i32, i32)> {
    // Find positions of frequency
    let mut positions: Vec<(i32, i32)> = Vec::new();

    // Create new empty city map
    let mut new_city_map = city_map.to_vec();

    // Find all cells with frequency we care about
    for (i, row) in city_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == frequency {
                positions.push((i as i32, j as i32));
            }
        }
    }

    println!("Positions: {:?}", positions);
    let mut found_antinodes: HashSet<(i32, i32)> = HashSet::new();

    let mut cursor = 1;
    // Iterate through all pairs of positions
    for (i, j) in positions.iter() {
        found_antinodes.replace((*i, *j));

        for (k, l) in positions.iter().skip(cursor) {
            println!("({}, {}) -> ({}, {})", i, j, k, l);
            // Find delta
            let (dx, dy) = ((k - i).abs(), (l - j));
            println!("Delta: ({}, {})", dx, dy);
            if dy > 0 {
                // Check up and to the left
                let mut zx = i - dx;
                let mut zy = j - dy;
                while zx >= 0 && zy >= 0 {
                    found_antinodes.replace((zx, zy));
                    new_city_map[zx as usize][zy as usize] = '#';
                    zx -= dx;
                    zy -= dy;
                }
                // Check down and to the right
                let mut zx = k + dx;
                let mut zy = l + dy;
                while zx < city_map.len() as i32 && zy < city_map[0].len() as i32 {
                    found_antinodes.replace((zx, zy));
                    new_city_map[zx as usize][zy as usize] = '#';

                    zx += dx;
                    zy += dy;
                }
            } else {
                // Check up and to the right
                let mut zx = i - dx;
                let mut zy = j - dy;
                while zx >= 0 && zy < city_map[0].len() as i32 {
                    found_antinodes.replace((zx, zy));
                    new_city_map[zx as usize][zy as usize] = '#';

                    zx -= dx;
                    zy -= dy;
                }

                // Check down and to the left
                let mut zx = k + dx;
                let mut zy = l + dy;
                while zx < city_map.len() as i32 && zy >= 0 {
                    println!("Antinode found at ({}, {})", zx, zy);
                    found_antinodes.replace((zx, zy));
                    new_city_map[zx as usize][zy as usize] = '#';

                    zx += dx;
                    zy += dy;
                }
            }
        }
        cursor += 1;
    }

    found_antinodes
}

fn find_frequencies(city_map: &[Vec<char>]) -> HashSet<char> {
    let mut frequencies: HashSet<char> = HashSet::new();
    for i in city_map {
        for j in i {
            if *j != '.' {
                frequencies.insert(*j);
            }
        }
    }
    frequencies
}
