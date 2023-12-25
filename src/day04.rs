// https://adventofcode.com/2023/day/4

use std::collections::HashSet;

pub fn solve(input: String) {
    println!("Day 4 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut points = 0;

    for line in input.lines() {
        let card = parse_card(line);
        let matches = card.numbers.intersection(&card.winners).count();
        if matches > 0 {
            points += 2_i32.pow((matches - 1) as u32);
        }
    }
 
    println!("\t Part 1: {}", points);
}

pub fn solve_part_2(input: &String) {
    
    println!("\t Part 2: {}", 0);
}

#[derive(Debug)]
struct Card {
    winners: HashSet<i32>,
    numbers: HashSet<i32>,
}

fn parse_card(line: &str) -> Card {
    let mut all_numbers = line.split(": ").last().unwrap().split(" | ");
    let winners = all_numbers.next().unwrap();
    let numbers = all_numbers.next().unwrap();

    let parse = |x: &str| -> HashSet<i32> {
        x.split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
    };

    Card {
        winners: parse(winners),
        numbers: parse(numbers),
    }
}