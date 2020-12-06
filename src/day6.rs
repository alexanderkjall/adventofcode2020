use std::collections::HashSet;

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day6-input")?.parse()?;

    let result_1 = sum_unique(&input);
    let result_2 = sum_and(&input);

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn sum_unique(input: &str) -> usize {
    let groups: Vec<&str> = input.split("\n\n").collect();

    groups
        .iter()
        .map(|s| {
            let chars: HashSet<char> = s.chars().filter(|c| *c != ' ' && *c != '\n').collect();
            chars.len()
        })
        .sum()
}

fn sum_and(input: &str) -> usize {
    let groups: Vec<&str> = input.trim().split("\n\n").collect();

    groups
        .iter()
        .map(|s| {
            let lines = s.split('\n');
            let initial_state: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
            let intersect = lines
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold(initial_state, |i, s| i.intersection(&s).copied().collect());
            intersect.len()
        })
        .sum()
}

#[test]
fn test_sum_unique() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(11, sum_unique(input));
}

#[test]
fn test_sum_and() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";

    assert_eq!(6, sum_and(input));
}

#[test]
fn test_sum_and_1() {
    let input = "abc";

    assert_eq!(3, sum_and(input));
}

#[test]
fn test_sum_and_2() {
    let input = "a
b
c";

    assert_eq!(0, sum_and(input));
}

#[test]
fn test_sum_and_3() {
    let input = "ab
ac";

    assert_eq!(1, sum_and(input));
}
