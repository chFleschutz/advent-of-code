use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn load_input(day: i32) -> Result<String, std::io::Error> {
    let file_path = format!("input/day{:02}.txt", day);
    fs::read_to_string(file_path)
}

fn main() {
    // Get the day from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <day>", args[0]);
        return;
    }

    // Parse the day argument
    let day = match args[1].parse::<i32>() {
        Ok(num @ 1..=24) => num,
        Ok(num) => {
            println!("Day {} does not exist", num);
            return;
        }
        Err(_) => {
            println!("Invalid argument: {}", args[1]);
            return;
        }
    };

    // Load the input file of the given day
    let input = match load_input(day) {
        Ok(input_string) => input_string,
        Err(e) => {
            println!("Error reading input file: {}", e);
            return;
        }
    };

    // Solve the given day
    match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        9 => day09::solve(input),
        _ => println!("Day {} not implemented", day),
    }
}
