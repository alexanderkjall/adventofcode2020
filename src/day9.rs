use anyhow::anyhow;

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day9-input")?.parse()?;

    let data = parse(&input);
    let result_1 = find_pattern_breaker(&data, 25)?;
    let result_2 = find_range_that_sums_to(&data, result_1);
    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn find_range_that_sums_to(data: &[u64], target: u64) -> u64 {
    for i in 0..data.len() {
        for j in i..data.len() {
            let sum: u64 = data[i..j].iter().sum();
            if sum == target {
                return data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap();
            }
            if sum > target {
                break;
            }
        }
    }
    0
}

fn find_pattern_breaker(data: &[u64], lookback: usize) -> Result<u64, anyhow::Error> {
    for (i, d) in data[lookback..].iter().enumerate() {
        let mut found_match = false;
        for e in 0..lookback {
            for f in 0..lookback {
                if data[i + e] + data[i + f] == *d {
                    found_match = true;
                }
            }
        }
        if !found_match {
            return Ok(*d);
        }
    }
    Err(anyhow!("no pattern breaker"))
}

fn parse(input: &str) -> Vec<u64> {
    return input
        .trim()
        .split('\n')
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
}

#[test]
fn test_part_1() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    let data = parse(&input);
    let result_1 = find_pattern_breaker(&data, 5).unwrap();

    assert_eq!(127, result_1);
}

#[test]
fn test_part_2() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    let data = parse(&input);
    let result_2 = find_range_that_sums_to(&data, 127);

    assert_eq!(62, result_2);
}
