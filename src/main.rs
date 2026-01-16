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

    let x_points = [0]
        .iter()
        .chain(points.iter().map(|(x, _)| x).sorted().dedup())
        .collect_vec();
    let y_points = [0]
        .iter()
        .chain(points.iter().map(|(_, y)| y).sorted().dedup())
        .collect_vec();
    let points_compressed = points
        .iter()
        .map(|(x, y)| {
            (
                x_points.binary_search(&x).unwrap(),
                y_points.binary_search(&y).unwrap(),
            )
        })
        .collect_vec();

    let mut grid = vec![
        vec![false; points_compressed.iter().map(|(x, _)| x).max().unwrap() + 2];
        points_compressed.iter().map(|(_, y)| y).max().unwrap() + 2
    ];

    for &(x, y) in &points_compressed {
        grid[y][x] = true;
    }

    let mut verticals = points_compressed
        .windows(2)
        .filter_map(|w| {
            if w[0].0 == w[1].0 {
                let max = w[0].1.max(w[1].1);
                let min = w[0].1.min(w[1].1);
                Some((w[0].0, min..=max))
            } else {
                None
            }
        })
        .collect_vec();
    if points_compressed.first().unwrap().0 == points_compressed.last().unwrap().0 {
        let max = points_compressed
            .first()
            .unwrap()
            .1
            .max(points_compressed.last().unwrap().1);
        let min = points_compressed
            .first()
            .unwrap()
            .1
            .min(points_compressed.last().unwrap().1);
        verticals.push((points_compressed.first().unwrap().0, min..=max));
    }

    for r in (0..grid.len()).rev() {
        let mut count = 0;
        let mut top = false;
        let mut bottom = false;
        for c in 0..grid[0].len() {
            let mut on_vert = false;
            for (x, y) in &verticals {
                if *x == c && y.contains(&r) {
                    on_vert = true;
                    if top {
                        if r == *y.start() {
                            top = false;
                            count += 1;
                        } else if r == *y.end() {
                            top = false;
                        }
                    } else if bottom {
                        if r == *y.end() {
                            bottom = false;
                            count += 1;
                        } else if r == *y.start() {
                            bottom = false;
                        }
                    } else {
                        top = r == *y.end();
                        bottom = r == *y.start();
                    }
                    count += 1;
                    break;
                }
            }
            grid[r][c] = grid[r][c] || count & 1 == 1 || on_vert;
        }
    }
    
    points_compressed
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|combo| {
            (
                (combo[0].0).min(combo[1].0),
                (combo[0].0).max(combo[1].0),
                (combo[0].1).min(combo[1].1),
                (combo[0].1).max(combo[1].1),
            )
        })
        .filter(|(x_min, x_max, y_min, y_max)| {
            grid.iter()
                .take(*y_max + 1)
                .skip(*y_min)
                .flat_map(|r| r.iter().take(*x_max + 1).skip(*x_min))
                .all(|&x| x)
        })
        .map(|(x_min, x_max, y_min, y_max)| {
            (x_points[x_max] - x_points[x_min] + 1) * (y_points[y_max] - y_points[y_min] + 1)
        })
        .max()
        .unwrap()
}
