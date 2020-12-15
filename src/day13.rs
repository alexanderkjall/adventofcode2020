pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day13-input")?.parse()?;

    let (start_time, busses) = parse(&input)?;
    let result_1 = calc_start(&start_time, &busses);

    Ok((format!("{}", result_1), format!("")))
}

fn calc_start(start_time: &u64, busses: &[u64]) -> u64 {
    let mut lowest_wait: u64 = u64::MAX;
    let mut lowest_bus: u64 = u64::MAX;

    for bus in busses {
        if (bus - start_time % bus) < lowest_wait {
            lowest_wait = bus - start_time % bus;
            lowest_bus = *bus;
        }
    }

    lowest_wait * lowest_bus
}

fn parse(input: &str) -> Result<(u64, Vec<u64>), anyhow::Error> {
    let a: Vec<&str> = input.trim().split('\n').collect();

    let start_time = u64::from_str_radix(&a[0], 10)?;
    let busses: Vec<u64> = a[1]
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();

    Ok((start_time, busses))
}

#[test]
fn test_part_1() {
    let input = "939
7,13,x,x,59,x,31,19";

    let (start_time, busses) = parse(&input).unwrap();
    let result_1 = calc_start(&start_time, &busses);

    assert_eq!(295, result_1);
}
