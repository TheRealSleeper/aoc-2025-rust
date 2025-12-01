use super::*;

fn test(func: fn(&str) -> AnswerType, answer: AnswerType, path: &str) {
    assert_eq!(
        func(&std::fs::read_to_string(path).expect("Unable to read file")),
        answer
    );
}

#[test]
fn test1() {
    test(super::part1, 3, "samples/sample1.txt");
}

#[test]
fn test2() {
    test(super::part2, 6, "samples/sample1.txt");
}
