use aoc2024::read_lines;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

/// Parses a single line of input in the format "result: int1 int2 ..."
fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    // Parse the result value (integer before the colon) and the array of integers after it
    separated_pair(
        map(digit1, |s: &str| s.parse::<i64>().unwrap()), // Parse the result value
        tag(": "),                                        // Match and skip the colon with a space
        separated_list1(space1, map(digit1, |s: &str| s.parse::<i64>().unwrap())), // Parse the array of integers
    )(input)
}

fn generate_possible_equations(numbers: &Vec<i64>) -> Vec<String> {
    let mut equations = Vec::new();
    let num_operators = numbers.len() - 1;
    let total_combinations = 2_usize.pow(num_operators as u32);

    for i in 0..total_combinations {
        let mut equation = String::new();
        for (j, &num) in numbers.iter().enumerate() {
            if j > 0 {
                // Determine the operator based on the bit at position `j - 1` in `i`
                if (i & (1 << (j - 1))) != 0 {
                    equation.push('x');
                } else {
                    equation.push('+');
                }
            }
            equation.push_str(&num.to_string());
        }
        equations.push(equation);
    }
    equations
}

fn generate_possible_equations_part_two(numbers: &Vec<i64>) -> Vec<String> {
    let mut equations = Vec::new();
    let num_operators = numbers.len() - 1;
    let total_combinations = 3_usize.pow(num_operators as u32); // 3 operators: +, *, |

    for i in 0..total_combinations {
        let mut equation = String::new();
        let mut operator_index = i;

        for (j, &num) in numbers.iter().enumerate() {
            if j > 0 {
                // Determine the operator based on the current operator index (base 3)
                match operator_index % 3 {
                    0 => equation.push('+'),
                    1 => equation.push('x'),
                    2 => equation.push('|'),
                    _ => unreachable!(),
                }
                operator_index /= 3; // Move to the next operator
            }
            equation.push_str(&num.to_string());
        }

        equations.push(equation);
    }

    equations
}

fn evaluate_equation(equation: &str) -> i64 {
    let mut tokens = Vec::new();
    let mut num = String::new();

    // Split equation into tokens (numbers and operators)
    for ch in equation.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
        } else {
            if !num.is_empty() {
                //println!("Pushing number: {}", num);
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push(ch.to_string());
        }
    }
    if !num.is_empty() {
        //println!("Pushing number: {}", num);
        tokens.push(num);
    }

    //println!("Tokens: {:?}", tokens);

    // Evaluate the equation left-to-right
    let mut result = tokens[0].parse::<i64>().unwrap();
    //println!("Initial result: {}", result);
    let mut i = 1;
    while i < tokens.len() {
        let operator = &tokens[i];
        let next_number = tokens[i + 1].parse::<i64>().unwrap();
        //println!("Next number: {}", next_number);

        //println!("Operator: {}", operator);
        if operator == "+" {
            result += next_number;
        } else if operator == "x" {
            result *= next_number;
        } else if operator == "|" {
            let concatenated = format!("{}{}", result, next_number);
            result = concatenated.parse::<i64>().unwrap()
        }

        //println!("Result: {}", result);

        i += 2; // Move to the next operator
    }

    result
}

fn main() {
    let input = read_lines("inputs/7.txt");
    let mut calibration_result = 0;

    for line in input.iter() {
        let (result, numbers) = match parse_line(line) {
            Ok((_, (result, numbers))) => (result, numbers), // Bind the parsed values
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return; // Exit on error
            }
        };

        let equations = generate_possible_equations(&numbers);
        for eq in equations.iter() {
            if evaluate_equation(eq) == result {
                // println!("{}", eq);
                calibration_result += result;
                break;
            }
        }
    }
    println!("Calibration result: {}", calibration_result);

    // Part 2
    let mut calibration_result = 0;
    for line in input.iter() {
        let (result, numbers) = match parse_line(line) {
            Ok((_, (result, numbers))) => (result, numbers), // Bind the parsed values
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return; // Exit on error
            }
        };

        let equations = generate_possible_equations_part_two(&numbers);
        for eq in equations.iter() {
            if evaluate_equation(eq) == result {
                calibration_result += result;
                break;
            }
        }
    }
    println!("Calibration result part two: {}", calibration_result);
}
