// https://adventofcode.com/2023/day/9

pub fn solve(input: String) {
    println!("Day 9 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let value_histories = parse_input(input);

    let mut result = 0;
    for value_history in value_histories.iter() {
        result += predict_next_value(value_history);
    }

    println!("\t Part 1: {}", result);
}

pub fn solve_part_2(input: &String) {
    let value_histories = parse_input(input);

    let mut result = 0;
    for value_history in value_histories.iter() {
        result += predict_previous_value(value_history);
    }

    println!("\t Part 2: {}", result);
}

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse::<i32>().unwrap());
        }
        result.push(row);
    }
    result
}

fn calc_derivative(values: &Vec<i32>) -> Vec<i32> {
    let mut derivative = Vec::new();
    for i in 0..values.len() - 1 {
        derivative.push(values[i + 1] - values[i]);
    }
    derivative
}

fn is_zero_derivative(derivative: &Vec<i32>) -> bool {
    for i in 0..derivative.len() {
        if derivative[i] != 0 {
            return false;
        }
    }
    true
}

fn calc_all_derivatives(values: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut derivatives = Vec::new();
    let mut derivative_all_zero = false;

    derivatives.push(values.clone());

    while !derivative_all_zero {
        let last_derivate = derivatives.last().unwrap();
        let next_derivative = calc_derivative(last_derivate);

        derivative_all_zero = is_zero_derivative(&next_derivative);
        derivatives.push(next_derivative);
    }
    derivatives
}

fn predict_next_value(value_history: &Vec<i32>) -> i32 {

    let derivatives = calc_all_derivatives(value_history);

    // Calculate the next value
    let mut predicted_value = 0;
    for derivative in derivatives.iter() {
        predicted_value += derivative.last().unwrap();
    }
    predicted_value
}

fn predict_previous_value(value_history: &Vec<i32>) -> i32 {

    let derivatives = calc_all_derivatives(value_history);

    // Calculate the previous value
    let mut predicted_value = 0;
    for derivative in derivatives.iter().rev() {
        predicted_value = derivative.first().unwrap() - predicted_value;
    }

    predicted_value
}