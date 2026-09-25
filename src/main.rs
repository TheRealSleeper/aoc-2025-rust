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
    device: &'a str,
    end_device: &str,
    schematic: &HashMap<&str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, AnswerType>,
) -> AnswerType {
    if !schematic.contains_key(device) {
        return 0;
    }

    if cache.contains_key(device) {
        return cache[device];
    }

    let devs = &schematic[device];
    if devs.contains(&end_device) {
        cache.insert(device, 1);
        1
    } else {
        let mut paths = 0;
        for device in devs {
            paths += count_paths(device, end_device, schematic, cache);
        }
        cache.insert(device, paths);
        paths
    }
}

fn part1(_input: &str) -> AnswerType {
    count_paths("you", "out", &parse_input(_input), &mut HashMap::new())
}

fn part2(_input: &str) -> AnswerType {
    let schematic = parse_input(_input);

    let dac_paths = count_paths("dac", "fft", &schematic, &mut HashMap::new());
    let fft_paths = count_paths("fft", "dac", &schematic, &mut HashMap::new());

    let first = if dac_paths == 0 { "fft" } else { "dac" };
    let second = if dac_paths == 0 { "dac" } else { "fft" };
    let mid_paths = if dac_paths == 0 { fft_paths } else { dac_paths };

    let start_paths = count_paths("svr", first, &schematic, &mut HashMap::new());
    let end_paths = count_paths(second, "out", &schematic, &mut HashMap::new());

    start_paths * mid_paths * end_paths
}
