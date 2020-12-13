pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day10-input")?.parse()?;

    let data = parse(&input);

    let result_1 = calc_diffs(&data);
    let result_2 = calc_combinations(&data);

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn calc_combinations(data: &[u64]) -> u64 {
    let mut init = data.to_vec();
    init.push(0);
    init.sort_unstable();

    let mut sums: Vec<u64> = vec![0; init.len()];

    for i in (0..init.len()).rev() {
        let mut sum = 1;
        if i + 2 < init.len() {
            sum = sums[i + 1];
        }
        if i + 2 < init.len() && init[i + 2] - init[i] < 4 {
            sum += sums[i + 2];
        }
        if i + 3 < init.len() && init[i + 3] - init[i] < 4 {
            sum += sums[i + 3];
        }
        sums[i] = sum;
    }

    sums[0]
}

fn calc_diffs(data: &[u64]) -> u64 {
    let mut diff1 = 0;
    let mut diff3 = 0;

    data.iter().fold(0, |l, d| {
        match *d - l {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        }

        *d
    });

    diff1 * diff3
}

fn parse(input: &str) -> Vec<u64> {
    let mut data: Vec<u64> = input
        .trim()
        .split('\n')
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    data.push(data.iter().max().unwrap() + 3);
    data.sort_unstable();

    data
}

#[test]
fn test_small() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";

    let data = parse(&input);

    let result_1 = calc_diffs(&data);

    assert_eq!(35, result_1);
}

#[test]
fn test_larger() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    let data = parse(&input);

    let result_1 = calc_diffs(&data);

    assert_eq!(220, result_1);
}

#[test]
fn test_small_combinations() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";

    let data = parse(&input);

    let result_2 = calc_combinations(&data);

    assert_eq!(8, result_2);
}

#[test]
fn test_larger_combinations() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    let data = parse(&input);

    let result_2 = calc_combinations(&data);

    assert_eq!(19208, result_2);
}
