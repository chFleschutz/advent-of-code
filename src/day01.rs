// https://adventofcode.com/2023/day/1

pub fn solve(input: String) {
    println!("Day 01 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit = -1;
        let mut last_digit = -1;

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == -1 {
                    first_digit = c.to_digit(10).unwrap() as i32;
                }
                last_digit = c.to_digit(10).unwrap() as i32;
            }
        }

        if first_digit != -1 && last_digit != -1 {
            sum += first_digit * 10 + last_digit;
        }
    }

    println!("\t Part 1: {}", sum);
}

pub fn solve_part_2(input: &String) {
    let mut sum = 0;
    
    for line in input.lines() {
        let char_count = line.chars().count();
        let mut numbers: Vec<i32> = Vec::new();
        
        for i in 0..char_count {
            let char_i = line.chars().nth(i).unwrap();
            if char_i.is_digit(10) {
                numbers.push(char_i.to_digit(10).unwrap() as i32);
                continue;
            }

            if i + 3 > char_count {
                continue; 
            }

            match &line[i..i + 3] {
                "one" => numbers.push(1),
                "two" => numbers.push(2),
                "six" => numbers.push(6),
                _ => (),
            }
            
            if i + 4 > char_count {
                continue;
            }

            match &line[i..i + 4] {
                "four" => numbers.push(4),
                "five" => numbers.push(5),
                "nine" => numbers.push(9),
                _ => (),
            }

            if i + 5 > char_count {
                continue;
            }

            match &line[i..i + 5] {
                "three" => numbers.push(3),
                "seven" => numbers.push(7),
                "eight" => numbers.push(8),
                _ => (),
            }
        }

        if numbers.len() > 0 {
            sum += (numbers[0] * 10) + numbers[numbers.len() - 1];
        }
    }

    println!("\t Part 2: {}", sum);
}
