// https://adventofcode.com/2023/day/2

pub fn solve(input: String) {
    println!("Day 2 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;

    for line in input.lines() {
        let game = parse_game(line);
        let mut possible = true;

        for set in &game.sets {
            if set.red > max_red || set.green > max_green || set.blue > max_blue {
                //Game impossible
                possible = false;
            }
        }

        if possible {
            sum += game.id;
        }
    }

    println!("\t Part 1: {}", sum);
}

pub fn solve_part_2(input: &String) {
    let mut sum = 0;

    for line in input.lines() {
        let game = parse_game(line);
        let mut min_set = Set { red: 0, green: 0, blue: 0};
        
        for set in &game.sets {
            min_set.red = std::cmp::max(min_set.red, set.red);
            min_set.green = std::cmp::max(min_set.green, set.green);
            min_set.blue = std::cmp::max(min_set.blue, set.blue);
        }

        let power = min_set.red * min_set.green * min_set.blue;
        sum += power;
    }

    println!("\t Part 2: {}", sum);
}


struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

fn parse_game(input: &str) -> Game {
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
    };

    let parts: Vec<&str> = input.split(": ").collect();
    if parts.len() != 2 {
        panic!("Invalid game: {}", input);
    }

    let game_part = parts.first().unwrap();
    game.id = game_part[5..].parse::<i32>().unwrap();
    
    let sets_part = parts.last().unwrap();
    for set_str in sets_part.split("; ") {
        let mut set: Set = Set { red: 0, green: 0, blue: 0};

        let cubes: Vec<&str> = set_str.split(", ").collect();
        if cubes.len() > 3 {
            panic!("Invalid set: {}", set_str);
        }
        
        for cube in cubes {
            let pos = cube.find(" ").unwrap();
            let (count, color) = cube.split_at(pos);

            let count = count.parse::<i32>().unwrap();
            match color.trim() {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _ => panic!("Invalid color: {}", color),
            }
        }

        game.sets.push(set);
    }

    game
}