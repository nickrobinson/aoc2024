use anyhow::{Error, Result};
use aoc2024::read_chars;

fn main() {
    let path = "inputs/6.txt";
    let guard_map = read_chars(path).unwrap();
    let simulated_guard_map = simulate_guard_movement(&guard_map);

    let mut visited_cells = 0;

    for i in simulated_guard_map.iter() {
        for j in i.iter() {
            if *j == 'X' {
                visited_cells += 1;
            }
        }
    }
    println!("Visited cells: {}", visited_cells);

    // Part 2

    for (x, i) in guard_map.iter().enumerate() {
        for (y, j) in i.iter().enumerate() {
            if *j != '#' && *j != '>' && *j != '<' && *j != '^' && *j != 'v' {
                let mut new_guard_map = guard_map.clone();
                new_guard_map[x][y] = '#';
                let simulated_guard_map = simulate_guard_movement(&new_guard_map);
            }
        }
    }
}

fn simulate_guard_movement(guard_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let step_limit = 1000;
    let mut steps = 0;
    // Clone mutable guard map
    let mut ret_guard_map = guard_map.clone();

    // Find initial guard position
    let mut guard_position = match find_guard_position(&guard_map) {
        Ok(pos) => pos,
        Err(error) => panic!("{error:?}"),
    };
    let mut guard_direction = guard_map[guard_position.0 as usize][guard_position.1 as usize];

    // Move guard
    loop {
        if steps >= step_limit {
            println!("Step limit reached");
            break;
        }

        ret_guard_map[guard_position.0 as usize][guard_position.1 as usize] = 'X';

        // Make sure we haven't hit a barrier

        match guard_direction {
            '^' => {
                if guard_position.0 == 0 {
                    break;
                }
                if ret_guard_map[guard_position.0 as usize - 1][guard_position.1 as usize] == '#' {
                    guard_direction = '>';
                    continue;
                }
                guard_position.0 -= 1
            }
            '>' => {
                if guard_position.1 == ret_guard_map[0].len() as u32 - 1 {
                    break;
                }
                if ret_guard_map[guard_position.0 as usize][guard_position.1 as usize + 1] == '#' {
                    guard_direction = 'v';
                    continue;
                }

                guard_position.1 += 1
            }
            'v' => {
                if guard_position.0 == ret_guard_map.len() as u32 - 1 {
                    break;
                }

                if ret_guard_map[guard_position.0 as usize + 1][guard_position.1 as usize] == '#' {
                    guard_direction = '<';
                    continue;
                }

                guard_position.0 += 1
            }
            '<' => {
                if guard_position.1 == 0 {
                    break;
                }

                if ret_guard_map[guard_position.0 as usize][guard_position.1 as usize - 1] == '#' {
                    guard_direction = '^';
                    continue;
                }

                guard_position.1 -= 1
            }
            _ => panic!("Invalid guard direction"),
        }

        // Check if guard is out of bounds
        if (guard_position.0 >= ret_guard_map.len() as u32
            || guard_position.1 >= ret_guard_map[0].len() as u32)
            || (guard_position.0 == 0 || guard_position.1 == 0)
        {
            break;
        }
        // Update guard direction
        ret_guard_map[guard_position.0 as usize][guard_position.1 as usize] = guard_direction;
        steps += 1;
    }

    ret_guard_map
}

fn find_guard_position(guard_map: &Vec<Vec<char>>) -> Result<(u32, u32)> {
    for (x, i) in guard_map.iter().enumerate() {
        for (y, j) in i.iter().enumerate() {
            if *j == '^' || *j == '>' || *j == '<' || *j == 'v' {
                // Dereference `j`
                return Ok((x as u32, y as u32));
            }
        }
    }
    Err(Error::msg("Could not find guard position"))
}
