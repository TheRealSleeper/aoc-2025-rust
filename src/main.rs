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
    let jbs = _input
        .lines()
        .map(|l| {
            let mut coords = l.split(',');
            (
                coords.next().unwrap().parse::<f64>().unwrap(),
                coords.next().unwrap().parse::<f64>().unwrap(),
                coords.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect_vec();

    let combos = jbs
        .iter()
        .enumerate()
        .combinations(2)
        .map(|combo| {
            let distance = ((combo[0].1.0 - combo[1].1.0).powi(2)
                + (combo[0].1.1 - combo[1].1.1).powi(2)
                + (combo[0].1.2 - combo[1].1.2).powi(2))
            .sqrt();
            (combo[0], combo[1], distance)
        })
        .sorted_unstable_by(|a, b| a.2.total_cmp(&b.2))
        .collect_vec();

    let mut circuits = vec![vec![]];
    'outer: for combo in &combos {
        for circuit in &mut circuits {
            if circuit.contains(&&combo.0) && circuit.contains(&&combo.1) {
                continue 'outer;
            }
            if circuit.contains(&&combo.0) && !circuit.contains(&&combo.1) {
                circuit.push(&combo.1);
                continue 'outer;
            } else if !circuit.contains(&&combo.0) && circuit.contains(&&combo.1) {
                circuit.push(&combo.0);
                continue 'outer;
            }
        }
        circuits.push(vec![&combo.0, &combo.1]);
    }

    circuits.sort_unstable_by_key(|c| c.len());
    circuits.reverse();

    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
