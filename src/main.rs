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
        .enumerate()
        .map(|(i, l)| {
            let mut coords = l.split(',');
            (
                coords.next().unwrap().trim().parse::<f64>().unwrap(),
                coords.next().unwrap().trim().parse::<f64>().unwrap(),
                coords.next().unwrap().trim().parse::<f64>().unwrap(),
                i,
            )
        })
        .collect_vec();

    let combos = jbs
        .iter()
        .combinations(2)
        .map(|combo| {
            let distance = ((combo[0].0 - combo[1].0).powi(2)
                + (combo[0].1 - combo[1].1).powi(2)
                + (combo[0].2 - combo[1].2).powi(2))
            .sqrt();
            (combo[0].3, combo[1].3, distance)
        })
        .sorted_unstable_by(|a, b| a.2.total_cmp(&b.2))
        .collect_vec();

    const COMBOS: usize = if cfg!(test) { 10 } else { 1000 };
    let mut circuits: Vec<Vec<_>> = Vec::new();
    'outer: for combo in &combos[0..COMBOS] {
        for circuit in &mut circuits {
            let a = circuit.contains(&combo.0);
            let b = circuit.contains(&combo.1);
            if a && !b {
                circuit.push(combo.1);
                continue 'outer;
            }
            if !a && b {
                circuit.push(combo.0);
                continue 'outer;
            }
            if a && b {
                continue 'outer;
            }
        }
        circuits.push(vec![combo.0, combo.1]);
    }

    let mut removed = true;
    let mut remove_list = vec![];
    while removed {
        removed = false;
        'outer: for i in 0..circuits.len() {
            'inner: for ii in 0..circuits.len() {
                if i == ii {
                    continue 'outer;
                }
                if remove_list.contains(&ii) {
                    continue
                }
                
                for iii in 0..circuits[i].len() {
                    if circuits[ii].contains(&circuits[i][iii]) {
                        let mut circuit = circuits[ii].clone();
                        circuits[i].append(&mut circuit);
                        circuits[i].sort();
                        circuits[i].dedup();
                        remove_list.push(ii);
                        continue 'inner;
                    }
                }
            }
        }
        
        remove_list.sort();
        while let Some(i) = remove_list.pop() {
            removed = true;
            circuits.remove(i);
        }
    }

    'outer: for jb in &jbs {
        for circuit in &circuits {
            if circuit.contains(&jb.3) {
                continue 'outer;
            }
        }
        circuits.push(vec![jb.3])
    }

    circuits.sort_unstable_by_key(|c| c.len());
    circuits.reverse();
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn part2(_input: &str) -> AnswerType {
    todo!()
}
