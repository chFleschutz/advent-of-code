// https://adventofcode.com/2023/day/4

use std::collections::{HashMap, HashSet};

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
    let mut result = 0;

    let mut card_matches = HashMap::new();
    let mut instances = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let card = parse_card(line);
        let matches = card.numbers.intersection(&card.winners).count();

        card_matches.insert(index, matches);
        instances.insert(index, 1);
    }

    if let Some(&max_num) = instances.keys().max() {
        for card_nr in 0..=max_num {
            let matches = card_matches.get(&card_nr).unwrap();
            let card_count = instances.get_mut(&card_nr).unwrap().clone();
            result += card_count;

            for n in (card_nr + 1)..=(card_nr + matches) {
                *instances.entry(n).or_insert(0) += card_count;
            }
        }
    }

    println!("\t Part 2: {}", result);
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
