// https://adventofcode.com/2023/day/3

use std::collections::HashMap;

pub fn solve(input: String) {
    println!("Day 3 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut sum = 0;

    for (line_number, line) in input.lines().enumerate() {
        let mut part_number = false;
        let mut begin_index = Option::None;

        for (char_index, c) in line.chars().enumerate() {
            if !c.is_digit(10) {
                continue;
            }

            if begin_index.is_none() {
                begin_index = Some(char_index);
            }

            if !part_number {
                part_number = is_part_number(&input, line_number, char_index, is_part_symbol);
            }

            let number_finished = match line.chars().nth(char_index + 1) {
                Some(c) => !c.is_digit(10),
                None => true,
            };

            if number_finished {
                let num = line[begin_index.unwrap()..=char_index]
                    .parse::<i32>()
                    .unwrap();

                if part_number {
                    sum += num;
                }

                part_number = false;
                begin_index = Option::None;
            }
        }
    }

    println!("\t Part 1: {}", sum);
}

pub fn solve_part_2(input: &String) {
    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (line_number, line) in input.lines().enumerate() {
        let mut begin_index = Option::None;
        let mut part_coords = Option::None;

        for (char_index, c) in line.chars().enumerate() {
            if !c.is_digit(10) {
                continue;
            }

            if begin_index.is_none() {
                begin_index = Some(char_index);
            }

            if part_coords.is_none() {
                part_coords = part_location(&input, line_number, char_index, is_gear);
            }

            let number_finished = match line.chars().nth(char_index + 1) {
                Some(c) => !c.is_digit(10),
                None => true,
            };

            if number_finished {
                let num = line[begin_index.unwrap()..=char_index]
                    .parse::<i32>()
                    .unwrap();

                if let Some(coords) = part_coords {
                    match gear_map.get_mut(&coords) {
                        Some(vec) => vec.push(num),
                        None => {
                            gear_map.insert(coords, vec![num]);
                        }
                    }
                }

                part_coords = Option::None;
                begin_index = Option::None;
            }
        }
    }

    let mut sum = 0;
    for (_, value) in gear_map.iter() {
        if value.len() == 2 {
            sum += value[0] * value[1];
        }
    }

    println!("\t Part 2: {}", sum);
}

fn is_part_symbol(c: char) -> bool {
    match c {
        '*' | '/' | '+' | '-' | '%' | '=' | '@' | '#' | '&' | '$' => true,
        _ => false,
    }
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn is_part_number(
    schematic: &str,
    line_index: usize,
    char_index: usize,
    match_symbol: fn(char) -> bool,
) -> bool {
    match part_location(schematic, line_index, char_index, match_symbol) {
        Some(_) => true,
        None => false,
    }
}

fn part_location(
    schematic: &str,
    line_index: usize,
    char_index: usize,
    match_symbol: fn(char) -> bool,
) -> Option<(usize, usize)> {
    // Clamp line indices between 0 and the number of lines
    let max_line = std::cmp::min(line_index + 1, schematic.lines().count());
    let min_line = match line_index {
        0 => 0,
        _ => line_index - 1,
    };

    for i in min_line..=max_line {
        let line = match schematic.lines().nth(i) {
            Some(line) => line,
            None => continue,
        };

        // Clamp char indices between 0 and the number of chars
        let max_char = std::cmp::min(char_index + 1, line.chars().count());
        let min_char = match char_index {
            0 => 0,
            _ => char_index - 1,
        };

        for j in min_char..=max_char {
            if i == line_index && j == char_index {
                // Only check the adjacent chars
                continue;
            }

            let c = match line.chars().nth(j) {
                Some(c) => c,
                None => continue,
            };

            if match_symbol(c) {
                return Some((i, j));
            }
        }
    }

    return None;
}
