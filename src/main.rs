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

#[derive(Debug, Clone, Copy)]
enum HomeworkEntry {
    Number(AnswerType),
    Operation(u8),
}

impl HomeworkEntry {
    fn parse(inp: &str) -> Result<Self, ()> {
        use HomeworkEntry::*;
        match inp {
            "+" | "*" => Ok(Operation(inp.as_bytes()[0])),
            &_ => inp
                .parse::<AnswerType>()
                .map_or_else(|_| Err(()), |n| Ok(Number(n))),
        }
    }
}

fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn part1(_input: &str) -> AnswerType {
    use HomeworkEntry::*;

    let mut problems = transpose(
        _input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|s| HomeworkEntry::parse(s).unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    );

    problems
        .iter_mut()
        .map(|p| {
            let operation = p.pop().unwrap();
            match operation {
                Operation(b'+') => p
                    .iter_mut()
                    .map(|&mut n| if let Number(n) = n { n } else { unreachable!() })
                    .sum::<AnswerType>(),
                Operation(b'*') => p
                    .iter_mut()
                    .map(|&mut n| if let Number(n) = n { n } else { unreachable!() })
                    .product::<AnswerType>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn part2(_input: &str) -> AnswerType {
    let grid = _input
        .lines()
        .map(|l| {
            let mut bytes = l.as_bytes().to_owned();
            bytes.reverse();
            bytes
        })
        .collect_vec();

    let mut sum = 0;
    let mut numbers = vec![];
    let mut operation = None;
    for i in 0..grid[0].len() {
        let mut empty_column = true;
        let mut n = 0;

        for ii in 0..grid.len() {
            let byte = grid[ii][i];
            match byte {
                b'0'..=b'9' => {
                    n *= 10;
                    n += (byte - b'0') as AnswerType;
                    empty_column = false;
                }
                b'+' => {
                    operation = Some(b'+');
                    empty_column = false;
                }
                b'*' => {
                    operation = Some(b'*');
                    empty_column = false;
                }
                _ => {}
            }
        }

        if empty_column {
            sum += match operation {
                Some(b'+') => numbers.iter().sum::<AnswerType>(),
                Some(b'*') => numbers.iter().product::<AnswerType>(),
                _ => unreachable!(),
            };
            operation = None;
            numbers.clear();
        } else {
            numbers.push(n);
        }
    }

    if !numbers.is_empty() && !operation.is_none() {
        sum += match operation {
            Some(b'+') => numbers.into_iter().sum::<AnswerType>(),
            Some(b'*') => numbers.into_iter().product::<AnswerType>(),
            _ => unreachable!(),
        };
    }

    sum
}
