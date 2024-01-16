// https://adventofcode.com/2023/day/6

pub fn solve(input: String) {
    println!("Day 6 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut result = 1;

    let races = parse_input(input);
    for race in races {
        result *= possible_wins(race);
    }

    println!("\t Part 1: {}", result);
}

pub fn solve_part_2(input: &String) {
    println!("\t Part 2: {}", input.len());
}

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<i32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<i32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn calc_distance(total_time: i32, hold_time: i32) -> i32 {
    hold_time * (total_time - hold_time)
}

fn possible_wins(race: Race) -> i32 {
    let mut wins = 0;
    for hold_time in 1..race.time {
        let distance = calc_distance(race.time, hold_time);
        if distance > race.distance {
            wins += 1;
        }
    }

    wins
}
