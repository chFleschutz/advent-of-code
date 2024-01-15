// https://adventofcode.com/2023/day/5

pub fn solve(input: String) {
    println!("Day 5 solutions: ");
    solve_part_1(&input);
    solve_part_2(&input);
}

pub fn solve_part_1(input: &String) {
    let mut min_loc = u32::MAX;

    let (
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_ligth,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    ) = parse_input(input);

    for seed in seeds {
        let soil = map_value(seed, &seed_to_soil);
        let fert = map_value(soil, &soil_to_fert);
        let water = map_value(fert, &fert_to_water);
        let light = map_value(water, &water_to_ligth);
        let temp = map_value(light, &light_to_temp);
        let humid = map_value(temp, &temp_to_humid);
        let loc = map_value(humid, &humid_to_loc);

        if loc < min_loc {
            min_loc = loc;
        }
    }

    println!("\t Part 1: {}", min_loc);
}

pub fn solve_part_2(input: &String) {
    let (
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_ligth,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    ) = parse_input(input);

    let seed_ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            range: chunk[1],
        })
        .collect();

    let soil_ranges = map_ranges(seed_ranges, &seed_to_soil);
    let fert_ranges = map_ranges(soil_ranges, &soil_to_fert);
    let water_ranges = map_ranges(fert_ranges, &fert_to_water);
    let light_ranges = map_ranges(water_ranges, &water_to_ligth);
    let temp_ranges = map_ranges(light_ranges, &light_to_temp);
    let humid_ranges = map_ranges(temp_ranges, &temp_to_humid);
    let loc_ranges = map_ranges(humid_ranges, &humid_to_loc);

    let mut result = u32::MAX;
    for range in loc_ranges {
        result = std::cmp::min(result, range.start);
    }

    println!("\t Part 2: {}", result);
}

struct Mapping {
    destination: u32,
    source: u32,
    range: u32,
}

#[derive(Clone)]
struct Range {
    start: u32,
    range: u32,
}

fn parse_input(
    input: &String,
) -> (
    Vec<u32>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
) {
    let maps: Vec<&str> = input.split("\r\n\r\n").collect();
    if maps.len() != 8 {
        panic!("Invalid input");
    }

    let seeds = parse_seeds(maps[0]);

    let seed_to_soil = parse_map(maps[1]);
    let soil_to_fert = parse_map(maps[2]);
    let fert_to_water = parse_map(maps[3]);
    let water_to_ligth = parse_map(maps[4]);
    let light_to_temp = parse_map(maps[5]);
    let temp_to_humid = parse_map(maps[6]);
    let humid_to_loc = parse_map(maps[7]);

    (
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_ligth,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    )
}

fn parse_seeds(input: &str) -> Vec<u32> {
    let seed_str: Vec<&str> = input.split_whitespace().collect();
    let seeds: Result<Vec<u32>, _> = seed_str.iter().skip(1).map(|s| s.parse::<u32>()).collect();

    seeds.expect("Seed parsing failed")
}

fn parse_map(input: &str) -> Vec<Mapping> {
    let map_str: Vec<&str> = input.split("\r\n").collect();
    let maps: Result<Vec<Mapping>, Box<dyn std::error::Error>> = map_str
        .iter()
        .skip(1)
        .map(|s| {
            let map_str: Vec<&str> = s.split_whitespace().collect();
            let destination = map_str[0]
                .parse::<u32>()
                .expect("Destination parsing failed");
            let source = map_str[1].parse::<u32>().expect("Source parsing failed");
            let range = map_str[2].parse::<u32>().expect("Range parsing failed");
            Ok(Mapping {
                destination,
                source,
                range,
            })
        })
        .collect();

    maps.expect("Map parsing failed")
}

fn map_value(value: u32, map: &Vec<Mapping>) -> u32 {
    let mut result = value;
    for m in map {
        let min_value = m.source;
        let max_value = match m.source.checked_add(m.range) {
            Some(max) => max,
            None => u32::MAX,
        };

        if value >= min_value && value < max_value {
            result = m.destination + (value - m.source);
            break;
        }
    }
    result
}

fn ranges_mapping(ranges: &Vec<Range>, mapping: &Mapping) -> (Vec<Range>, Vec<Range>) {
    let mut map_ranges = Vec::new();
    let mut new_ranges = Vec::new();

    let mapping_max = match mapping.source.checked_add(mapping.range) {
        Some(max) => max,
        None => u32::MAX,
    };

    for range in ranges {
        let range_max = match range.start.checked_add(range.range) {
            Some(max) => max,
            None => u32::MAX,
        };

        // No overlap
        if range.start >= mapping_max || range_max <= mapping.source {
            new_ranges.push(range.clone());
            continue;
        }

        // Range is fully contained in mapping
        if range.start >= mapping.source && range_max <= mapping_max {
            map_ranges.push(Range {
                start: mapping.destination + (range.start - mapping.source),
                range: range.range,
            });
            continue;
        }

        let mut range_remaining = range.range;

        // Range until mapping
        if range.start < mapping.source {
            new_ranges.push(Range {
                start: range.start,
                range: mapping.source - range.start,
            });

            range_remaining -= mapping.source - range.start;
        }

        // Range in mapping
        let offset = match range.start > mapping.source {
            true => range.start - mapping.source,
            false => 0,
        };
        map_ranges.push(Range {
            start: mapping.destination + offset,
            range: std::cmp::min(std::cmp::min(mapping_max - range.start, range_remaining), mapping.range), 
        });
        
        // Range after mapping
        if range_max > mapping_max {
            new_ranges.push(Range {
                start: mapping_max,
                range: range_max - mapping_max,
            });
        }
    }

    (new_ranges, map_ranges)
}

fn map_ranges(ranges: Vec<Range>, map: &Vec<Mapping>) -> Vec<Range> {
    let mut result = Vec::new();
    let mut new_ranges = ranges;

    for mapping in map {
        let (unmapped_ranges, mapped_ranges) = ranges_mapping(&new_ranges, mapping);
        new_ranges = unmapped_ranges;
        result.extend(mapped_ranges);
    }

    result.extend(new_ranges);
    result
}