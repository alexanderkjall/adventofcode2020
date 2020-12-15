pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day13-input")?.parse()?;

    let (start_time, busses) = parse(&input)?;
    let result_1 = calc_start(&start_time, &busses);
    let (modulii, residues) = parse_rem(&input)?;
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn parse_rem(input: &str) -> Result<(Vec<i64>, Vec<i64>), anyhow::Error> {
    let mut modulii = vec![];
    let mut residues = vec![];

    let a: Vec<&str> = input.trim().split('\n').collect();

    for (i, bus) in a[1].split(',').enumerate() {
        if bus == "x" {
            continue;
        }
        let busnr = i64::from_str_radix(bus, 10)?;
        modulii.push(busnr);
        residues.push(((i as i64 % busnr) - busnr).abs())
    }

    Ok((modulii, residues))
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

#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[test]
fn test_part_1() {
    let input = "939
7,13,x,x,59,x,31,19";

    let (start_time, busses) = parse(&input).unwrap();
    let result_1 = calc_start(&start_time, &busses);

    assert_eq!(295, result_1);
}

#[test]
fn test_part_2() {
    let input = "939
7,13,x,x,59,x,31,19";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(1068781, result_2);
}

#[test]
fn test_part_2_1() {
    let input = "939
17,x,13,19";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(3417, result_2);
}

#[test]
fn test_part_2_2() {
    let input = "939
67,7,59,61";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(754018, result_2);
}

#[test]
fn test_part_2_3() {
    let input = "939
67,x,7,59,61";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(779210, result_2);
}

#[test]
fn test_part_2_4() {
    let input = "939
67,7,x,59,61";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(1261476, result_2);
}

#[test]
fn test_part_2_5() {
    let input = "939
1789,37,47,1889";

    let (modulii, residues) = parse_rem(&input).unwrap();
    let result_2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(1202161486, result_2);
}
