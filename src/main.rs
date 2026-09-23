use itertools::Itertools;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::{fs::read_to_string, sync::OnceLock};

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

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn from_str(string: &str) -> Self {
        static RE: OnceLock<Regex> = OnceLock::new();
        let captures = RE.get_or_init(||RegexBuilder::new(
            r"(?<lights>\[[.#]+\])\s*(?<buttons>(?:\s*\((?:[0-9]+,?)+\))+)\s*(?<joltages>\{(?:[0-9]+,?)+\})"
            )
            .build()
            .unwrap())
            .captures(string)
            .unwrap();

        let lights = captures
            .name("lights")
            .unwrap()
            .as_str()
            .strip_circumfix('[', ']')
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect_vec();

        let buttons = captures
            .name("buttons")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .map(|s| {
                s.strip_circumfix('(', ')')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let joltages = captures
            .name("joltages")
            .unwrap()
            .as_str()
            .strip_circumfix('{', '}')
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        Machine {
            lights,
            buttons,
            joltages,
        }
    }
}

fn part1(_input: &str) -> AnswerType {
    let machines = _input
        .par_lines()
        .map(Machine::from_str)
        .collect::<Vec<_>>(); 
        // .map(|cap| todo!())
    // .sum::<AnswerType>()
    // 
    todo!(); 
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
