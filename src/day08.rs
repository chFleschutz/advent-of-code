// https://adventofcode.com/2023/day/8

use std::collections::HashMap;

pub fn solve(input: String) {
    println!("Day 8 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let (directions, map) = parse_input(input);
    
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let (left, right) = match map.get(current) {
            Some((l, r)) => (l, r),
            None => panic!("Invalid current: {}", current),
        };

        current = match directions[steps % directions.len()] {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps += 1;
    }

    println!("\t Part 1: {}", steps);
}

pub fn solve_part_2(input: &String) {
 
    println!("\t Part 2: {}", input.len());
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &String) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {

    let mut parts = input.split("\r\n\r\n");

    let directio_str = parts.next().unwrap();
    let map_str = parts.next().unwrap();

    // Parse the direction
    let mut directions = Vec::new();
    for c in directio_str.chars() {
        let dir = match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        };

        directions.push(dir);
    }

    // Parse the map
    let mut map = HashMap::new();
    for line in map_str.lines() {

        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(key, (left, right));
    }

    (directions, map)
}