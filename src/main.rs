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

fn part1(_input: &str) -> AnswerType {
    let grid = _input
        .lines()
        .map(|l| l.bytes().collect_vec())
        .collect_vec();

    let mut beams = vec![false; grid[0].len()];
    beams[grid[0].iter().position(|&b| b == b'S').unwrap()] = true;
    let mut total = 0;

    for row in &grid {
        let mut new_beams = vec![];
        let mut remove_beams = vec![];
        for (pos, is_beam) in beams.iter().enumerate() {
            if !is_beam {
                continue;
            }
            let mut split = false;
            if row[pos] == b'^' {
                remove_beams.push(pos);
                if pos > 0 {
                    new_beams.push(pos - 1);
                    total += 1;
                    split = true;
                }
                if pos < row.len() - 1 {
                    new_beams.push(pos + 1);
                    if !split {
                        total += 1;
                    }
                }
            }
        }

        for &beam in &new_beams {
            beams[beam] = true;
        }
        new_beams.clear();

        for &beam in &remove_beams {
            beams[beam] = false;
        }
        remove_beams.clear();
        for i in 0..row.len() {
            if beams[i] {
                print!("|");
            } else {
                print!("{}", row[i] as char);
            }
        }
        println!();
    }

    total
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
