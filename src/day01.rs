use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn solve() -> io::Result<()> {
    let file_path = "input/day01.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let mut first_digit = -1;
        let mut last_digit = -1;

        for c in line?.chars() {
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

    print!("Solution of Day 01 part 1: {}\n", sum);
    Ok(())
}
