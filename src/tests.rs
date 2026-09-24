use super::*;

fn test(func: fn(&str) -> AnswerType, answer: AnswerType, path: &str) {
    assert_eq!(
        func(&std::fs::read_to_string(path).expect("Unable to read file")),
        answer
    );
}

#[allow(unreachable_code)]
#[test]
fn test1() {
    test(part1, 7, "samples/sample1.txt");
}

#[allow(unreachable_code)]
#[test]
fn test2() {
    test(part2, 33, "samples/sample1.txt");
}
