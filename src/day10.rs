// https://adventofcode.com/2023/day/10

pub fn solve(input: String) {
    println!("Day 10 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let pipe_grid = parse_input(input);
    let loop_pipe_count = calc_pipes_in_loop(&pipe_grid);

    let steps_to_furthest = loop_pipe_count / 2;

    println!("\t Part 1: {}", steps_to_furthest);
}

pub fn solve_part_2(input: &String) {
    println!("\t Part 2: {}", input.len());
}

struct Connections {
    a: (i32, i32),
    b: (i32, i32),
}

struct Pipe {
    connections: Connections,
}

struct PipeGrid {
    pipes: Vec<Pipe>,
    width: i32,
    height: i32,
    start: (i32, i32),
}

fn map_connections(character: char) -> Connections {
    match character {
        '|' => Connections {
            a: (0, -1),
            b: (0, 1),
        },
        '-' => Connections {
            a: (-1, 0),
            b: (1, 0),
        },
        'L' => Connections {
            a: (0, -1),
            b: (1, 0),
        },
        'J' => Connections {
            a: (0, -1),
            b: (-1, 0),
        },
        '7' => Connections {
            a: (0, 1),
            b: (-1, 0),
        },
        'F' => Connections {
            a: (0, 1),
            b: (1, 0),
        },
        '.' => Connections {
            a: (0, 0),
            b: (0, 0),
        },
        'S' => Connections {
            a: (0, 0),
            b: (0, 0),
        },
        _ => panic!("Invalid character: {}", character),
    }
}

fn add((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn pipe_at(pipe_grid: &PipeGrid, (x, y): (i32, i32)) -> Option<&Pipe> {
    if x < 0 || x >= pipe_grid.width || y < 0 || y >= pipe_grid.height {
        return None;
    }

    let index = (y * pipe_grid.width + x) as usize;
    Some(&pipe_grid.pipes[index])
}

fn find_start_connections(pipe_grid: &PipeGrid) -> Connections {
    let x_offsets = [0, -1, 1, 0];
    let y_offsets = [-1, 0, 0, 1];

    let mut start_directions = Vec::new();

    let start_pos = pipe_grid.start;
    println!("Start position: {:?}", start_pos);

    for i in 0..4 {
        let sample_pos = add(start_pos, (x_offsets[i], y_offsets[i]));
        let pipe = pipe_at(pipe_grid, sample_pos);

        if let Some(pipe) = pipe {
            let pos_a = add(sample_pos, pipe.connections.a);
            let pos_b = add(sample_pos, pipe.connections.b);

            if pos_a == start_pos || pos_b == start_pos {
                start_directions.push((x_offsets[i], y_offsets[i]));
            }
        }
    }

    if start_directions.len() != 2 {
        panic!("Invalid start position");
    }

    Connections {
        a: start_directions[0],
        b: start_directions[1],
    }
}

fn parse_input(input: &String) -> PipeGrid {
    let mut pipe_grid = PipeGrid {
        pipes: Vec::with_capacity(input.len()),
        width: input.lines().next().unwrap().len() as i32,
        height: input.lines().count() as i32,
        start: (0, 0),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let connections = map_connections(character);
            pipe_grid.pipes.push(Pipe { connections });

            if character == 'S' {
                pipe_grid.start = (x as i32, y as i32);
            }
        }
    }

    // Find and set the connections for the start pipe
    let start_pipe_index = (pipe_grid.start.1 * pipe_grid.width + pipe_grid.start.0) as usize;
    pipe_grid.pipes[start_pipe_index].connections = find_start_connections(&pipe_grid);
    
    pipe_grid
}

fn calc_pipes_in_loop(pipe_grid: &PipeGrid) -> i32 {
    let mut pipe_count = 0;
    let mut last_pos = pipe_grid.start;
    let mut current_pos = pipe_grid.start;
    let mut next_pos = (-1, -1);

    while next_pos != pipe_grid.start {
        let current_pipe = pipe_at(pipe_grid, current_pos).unwrap();

        next_pos = add(current_pos, current_pipe.connections.a);
        if next_pos == last_pos {
            next_pos = add(current_pos, current_pipe.connections.b);
        }

        last_pos = current_pos;
        current_pos = next_pos;
        pipe_count += 1;
    }

    pipe_count
}
