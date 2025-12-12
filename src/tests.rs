use super::*;

fn test(func: fn(&str) -> AnswerType, answer: AnswerType, path: &str) {
    assert_eq!(
        func(&std::fs::read_to_string(path).expect("Unable to read file")),
        answer
    );
}

#[test]
fn test1() {
    test(part1, 50, "samples/sample1.txt");
}

#[test]
fn test2() {
    test(part2, 24, "samples/sample1.txt");
}
