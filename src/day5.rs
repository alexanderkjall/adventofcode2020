use nom::lib::std::collections::HashSet;

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day5-input")?.parse()?;

    let seats: Vec<u16> = input.split('\n').map(|s| parse_seat_id(s)).collect();

    let result_1 = seats.iter().max().unwrap();
    let result_2 = find_missing(&seats);

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn parse_seat_id(id: &str) -> u16 {
    if id.is_empty() {
        return 0;
    }

    let id = id.replace('F', "0");
    let id = id.replace('B', "1");
    let id = id.replace('R', "1");
    let id = id.replace('L', "0");

    u16::from_str_radix(&id, 2).unwrap()
}

fn find_missing(seats: &[u16]) -> u16 {
    let mut all: HashSet<u16> = (61..995).collect();

    for s in seats {
        all.remove(s);
    }

    return *all.iter().next().unwrap();
}

#[test]
fn test_parse() {
    assert_eq!(567, parse_seat_id("BFFFBBFRRR"));
    assert_eq!(119, parse_seat_id("FFFBBBFRRR"));
    assert_eq!(820, parse_seat_id("BBFFBBFRLL"));
}
