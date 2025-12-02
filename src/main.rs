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

fn part1(_input: &str) -> AnswerType {
    _input
        .split(',')
        .flat_map(|r| {
            let (lower, upper) = r.split_once('-').expect("Range missing '-'");
            (lower.trim().parse::<AnswerType>().unwrap_or_else(|_| {
                eprintln!("Error: '{lower}' is not a valid number");
                panic!()
            })..=upper.trim().parse::<AnswerType>().unwrap_or_else(|_| {
                eprintln!("Error: '{upper}' is not a valid number");
                panic!()
            }))
                .filter(|v| {
                    let s = v.to_string();
                    let (left, right) = s.split_at(s.len() / 2);
                    left == right
                })
        })
        .sum()
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
