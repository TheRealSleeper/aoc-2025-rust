use itertools::Itertools;
use rayon::prelude::*;
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
    _input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (
                x.parse::<AnswerType>().unwrap(),
                y.parse::<AnswerType>().unwrap(),
            )
        })
        .combinations(2)
        .map(|combo| (combo[0].0.abs_diff(combo[1].0) + 1) * (combo[0].1.abs_diff(combo[1].1) + 1))
        .max()
        .unwrap()
}

fn part2(_input: &str) -> AnswerType {
    let points = _input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (
                x.parse::<AnswerType>().unwrap(),
                y.parse::<AnswerType>().unwrap(),
            )
        })
        .collect_vec();
    let mut grid = vec![
        vec![false; points.iter().map(|p| p.1).max().unwrap() + 1];
        points.iter().map(|p| p.0).max().unwrap() + 1
    ];

    for line in points.windows(2) {
        for i in line[0].0.min(line[1].0)..=line[0].0.max(line[1].0) {
            for ii in line[0].1.min(line[1].1)..=line[0].1.max(line[1].1) {
                grid[i][ii] = true;
            }
        }
    }
    for i in points[0].0.min(points.last().unwrap().0)..=points[0].0.max(points.last().unwrap().0) {
        for ii in
            points[0].1.min(points.last().unwrap().1)..=points[0].1.max(points.last().unwrap().1)
        {
            grid[i][ii] = true;
        }
    }

    grid.insert(0, vec![false; grid[0].len()]);
    grid.push(vec![false; grid[0].len()]);

    for row in &mut grid {
        row.push(false);
        row.insert(0, false);
    }

    for i in 1..grid.len() - 1 {
        let mut edge_bot = false;
        let mut edge_top = false;
        let mut edge_count = 0;
        for ii in 1..grid[i].len() - 1 {
            let space = grid[i][ii];
            let space_up = grid[i - 1][ii];
            let space_down = grid[i + 1][ii];
            let space_fwd = grid[i][ii + 1];
            let space_back = grid[i][ii - 1];

            if space_up && space && space_fwd && !space_down && !edge_top && !edge_bot {
                edge_top = true;
            }
            if !space_up && space && space_fwd && space_down && !edge_top && !edge_bot {
                edge_bot = true;
            }
            if !edge_top && !edge_bot {
                if space {
                    edge_count += 1;
                } else {
                    grid[i][ii] = edge_count & 1 == 1;
                }
            }
            if edge_top && space_down && space && space_back && !space_up {
                edge_count += 1;
                edge_top = false;
            } else if edge_bot && !space_down && space && space_back && space_up {
                edge_count += 1;
                edge_bot = false;
            }
            if edge_top && !space_down && space && space_back && space_up {
                edge_top = false;
            } else if edge_bot && space_down && space && space_back && !space_up {
                edge_bot = false;
            }
        }
    }

    // for row in &grid {
    //     for &space in row {
    //         print!("{}", if space {'#'} else {'.'});
    //     }
    //     println!();
    // }

    points
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|combo| {
            (
                (combo[0].0 + 1).min(combo[1].0 + 1),
                (combo[0].0 + 1).max(combo[1].0 + 1),
                (combo[0].1 + 1).min(combo[1].1 + 1),
                (combo[0].1 + 1).max(combo[1].1 + 1),
            )
        })
        .filter(|(x_min, x_max, y_min, y_max)| {
            for i in *x_min..=*x_max {
                for ii in *y_min..=*y_max {
                    if !grid[i][ii] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(x_min, x_max, y_min, y_max)| (x_max - x_min + 1) * (y_max - y_min + 1))
        .max()
        .unwrap()
}
