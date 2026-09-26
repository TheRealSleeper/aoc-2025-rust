use itertools::Itertools;
use std::fs::read_to_string;

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

fn parse_input(input: &str) -> (Vec<u8>, Vec<Vec<usize>>) {
    let shapes = input
        .split(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .take(6)
        .map(|s| s.chars().filter(|c| *c == '#').count() as u8)
        .collect();

    let zones = input
        .split(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .dropping(6)
        .map(|s| {
            s.lines()
                .map(|l| {
                    let (size, counts) = l.split_once(':').unwrap();
                    let size = size
                        .split('x')
                        .map(|n| n.parse::<usize>().unwrap())
                        .product::<usize>();
                    let mut counts = counts
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect_vec();
                    counts.insert(0, size);
                    counts
                })
                .collect_vec()
        })
        .last()
        .unwrap();

    (shapes, zones)
}

fn part1(_input: &str) -> AnswerType {
    let (shapes, zones) = parse_input(_input);
    zones
        .into_iter()
        .filter(|z| {
            let a_req = z[0];
            let a_act = z[1..]
                .iter()
                .enumerate()
                .map(|(i, n)| n * shapes[i] as usize)
                .sum();
            a_req >= a_act
        })
        .count()
}

fn part2(_input: &str) -> AnswerType {
    unimplemented!("Part 2 is solved automagically!")
}
