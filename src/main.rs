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
        println!(
            "{}",
            part1(content.as_deref().expect("No input file was opened"))
        )
    }

    if args.part2 {
        println!(
            "{}",
            part2(content.as_deref().expect("No input file was opened"))
        )
    }
}

#[inline]
/// Adds two numbers, wraps at upper and lower bounds and counts wraps, returns ```(sum, wraps)```
fn wrapping_add(n1: i32, n2: i32, upper: i32, lower: i32) -> (i32, i32) {
    let range = upper - lower + 1;
    let raw_sum = n1 + n2;
    let mut wraps = 0;
    let mut sum = if raw_sum > upper || raw_sum < lower {
        wraps = (raw_sum - lower) / range;
        raw_sum - wraps * range
    } else {
        raw_sum
    };

    wraps = wraps.abs();
    if sum < lower {
        wraps += 1;
        sum += range;
    }

    (sum, wraps)
}

#[inline]
fn get_turns(s: &str) -> Vec<i32> {
    s.lines()
        .map(|l| {
            let (dir, mag) = l.split_at(1);
            match dir {
                "L" => -1 * mag.parse::<i32>().expect("No magnitude present"),
                "R" => mag.parse::<i32>().expect("No magnitude present"),
                &_ => panic!("No direction present"),
            }
        })
        .collect_vec()
}

/// Solves part 1
fn part1(_input: &str) -> AnswerType {
    let turns = get_turns(_input);
    let mut count = 0;
    let mut position = 50;

    for movement in turns {
        (position, _) = wrapping_add(position, movement, 99, 0);
        if position == 0 {
            count += 1;
        }
    }

    count
}

/// Solves part 2
fn part2(_input: &str) -> AnswerType {
    let turns = get_turns(_input);
    let mut count = 0;
    let mut position = 50;

    for movement in turns {
        let (new_position, wraps) = wrapping_add(position, movement, 99, 0);
        if new_position == 0 && wraps == 0 {
            count += 1;
        }
        if position == 0 && wraps > 0 {
            count -= 1;
        }
        count += wraps;
        position = new_position;
    }

    count as AnswerType
}
