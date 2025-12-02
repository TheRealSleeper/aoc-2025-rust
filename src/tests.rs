use super::*;

fn test(func: fn(&str) -> AnswerType, answer: AnswerType, path: &str) {
    assert_eq!(
        func(&std::fs::read_to_string(path).expect("Unable to read file")),
        answer
    );
}

#[test]
fn test1() {
    test(part1, 3, "samples/sample1.txt");
}

#[test]
fn test2() {
    test(part2, 6, "samples/sample1.txt");
    test(part2, 5, "samples/full-rotations.txt");
    test(part2, 1, "samples/p2-11");
    test(part2, 1, "samples/p2-12");
    test(part2, 1, "samples/p2-13");
    test(part2, 1, "samples/p2-14");
    test(part2, 2, "samples/p2-21");
    test(part2, 2, "samples/p2-22");
    test(part2, 2, "samples/p2-23");
    test(part2, 2, "samples/p2-24");
}
