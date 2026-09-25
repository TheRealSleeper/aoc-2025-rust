use std::{collections::HashMap, fs::read_to_string};

#[allow(dead_code)]
mod aoc_lib;

#[cfg(test)]
mod tests;

type AnswerType = usize;

fn main() {
    let args = aoc_lib::Args::get();

    let content = args
        .path
        .map(|p| read_to_string(&p).expect("input: Could not open file"));

    if args.part1 {
        let now = std::time::Instant::now();
        println!(
            "Part 1: {}, found in {}ms",
            part1(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }

    if args.part2 {
        let now = std::time::Instant::now();
        println!(
            "Part 2: {}, found in {}ms",
            part2(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let (dev, conns) = l.split_once(':').unwrap();
            (dev, conns.split_whitespace().collect())
        })
        .collect()
}

fn count_paths(node: &str, map: &HashMap<&str, Vec<&str>>, count: &mut AnswerType ) {
    let devs = &map[node]; 
    if devs.contains(&"out") {
        *count += 1; 
    } else {
        for dev in devs {
            count_paths(dev, map, count);
        }
    }
}

fn part1(_input: &str) -> AnswerType {
    let mut count = 0; 
    count_paths("you", &parse_input(_input), &mut count);
    count
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
