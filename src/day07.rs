// https://adventofcode.com/2023/day/7

use std::collections::HashMap;

pub fn solve(input: String) {
    println!("Day 7 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut hands = parse_input(input);
    hands.sort();

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i as i32 + 1);
    }

    println!("\t Part 1: {}", winnings);
}

pub fn solve_part_2(input: &String) {
    println!("\t Part 2: {}", input.len());
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    strength: HandType,
    bid: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Equal => {
                for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                    match card1.cmp(card2) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card: {}", c),
    }
}

fn hand_strength(cards: &str) -> HandType {
    let mut card_counts = HashMap::new();
    for card in cards.chars() {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    match card_counts.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if card_counts.values().any(|&x| x == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if card_counts.values().any(|&x| x == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPairs
            }
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn create_hand(cards: &str, bid: &str) -> Hand {
    Hand {
        cards: cards.chars().map(char_to_card).collect(),
        strength: hand_strength(cards),
        bid: bid.parse::<i32>().unwrap(),
    }
}

fn parse_input(input: &String) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        hands.push(create_hand(parts[0], parts[1]));
    }
    hands
}
