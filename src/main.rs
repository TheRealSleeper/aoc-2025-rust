use std::{collections::HashMap, fs::read_to_string};

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

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let (dev, conns) = l.split_once(':').unwrap();
            (dev, conns.split_whitespace().collect())
        })
        .collect()
}

fn count_paths<'a>(
    node: &'a str,
    end: &str,
    map: &HashMap<&str, Vec<&'a str>>,
    count: &mut AnswerType,
    cache: &mut HashMap<&'a str, AnswerType>,
) {
    if !map.contains_key(node) {
        return;
    }

    if cache.contains_key(node) {
        *count += cache[node];
        return;
    }

    let devs = &map[node];
    if devs.contains(&end) {
        *count += 1;
        cache.insert(node, 1);
    } else {
        let mut new_count = 0;
        for dev in devs {
            count_paths(dev, end, map, &mut new_count, cache);
        }
        cache.insert(node, new_count);
        *count += new_count;
    }
}

fn part1(_input: &str) -> AnswerType {
    let mut count = 0;
    count_paths(
        "you",
        "out",
        &parse_input(_input),
        &mut count,
        &mut HashMap::new(),
    );
    count
}

fn part2(_input: &str) -> AnswerType {
    let map = parse_input(_input);

    let mut dac_paths = 0;
    count_paths("dac", "fft", &map, &mut dac_paths, &mut HashMap::new());

    let mut fft_paths = 0;
    count_paths("fft", "dac", &map, &mut fft_paths, &mut HashMap::new());

    let first = if dac_paths == 0 { "fft" } else { "dac" };
    let second = if dac_paths == 0 { "dac" } else { "fft" };
    let mid_paths = if dac_paths == 0 { fft_paths } else { dac_paths };

    let mut start_paths = 0;
    count_paths("svr", first, &map, &mut start_paths, &mut HashMap::new());

    let mut end_paths = 0;
    count_paths(second, "out", &map, &mut end_paths, &mut HashMap::new());

    start_paths * mid_paths * end_paths
}
