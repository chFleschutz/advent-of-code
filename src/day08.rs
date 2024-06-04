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
        current = get_next_node(&directions, &map, current, steps);
        steps += 1;
    }

    println!("\t Part 1: {}", steps);
}

pub fn solve_part_2(input: &String) {
    let (directions, map) = parse_input(input);

    // list of all starting nodes ending with "xxA"
    let mut current_nodes: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).copied().collect();
    let mut step_count = vec![0; current_nodes.len()];

    // Get steps to reach node "xxZ" for each node
    for i in 0..current_nodes.len() {
        while !current_nodes[i].ends_with('Z'){
            current_nodes[i] = get_next_node(&directions, &map, current_nodes[i], step_count[i]);
            step_count[i] += 1;
        }
    }

    // Lowest common multiple of all steps is the total steps to set all nodes to "xxZ" (at the same time)
    let tota_steps = lowest_common_multiple(step_count);
    println!("\t Part 2: {}", tota_steps);
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

fn get_next_node<'a>(
    directions: &'a Vec<Direction>,
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    current: &'a str,
    steps: usize,
) -> &'a str {
    let (left, right) = match map.get(current) {
        Some((l, r)) => (l, r),
        None => panic!("Invalid current: {}", current),
    };

    match directions[steps % directions.len()] {
        Direction::Left => left,
        Direction::Right => right,
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lowest_common_multiple(numbers: Vec<usize>) -> usize {
    let mut lcm = 1;
    for i in 0..numbers.len() {
        lcm = lcm * numbers[i] / gcd(lcm, numbers[i]);
    }

    lcm
}