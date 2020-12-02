use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::alpha1,
    combinator::map_res,
    sequence::tuple,
    IResult,
};

struct Password {
    lower: u8,
    upper: u8,
    enforced_char: char,
    plain_text: String,
}

impl Password {
    fn validate_rule1(&self) -> bool {
        let mut count = 0;
        for c in self.plain_text.chars() {
            if c == self.enforced_char {
                count += 1;
            }
        }

        count >= self.lower && count <= self.upper
    }

    fn validate_rule2(&self) -> bool {
        let chars: Vec<char> = self.plain_text.chars().collect();

        let lower = if ((self.lower - 1) as usize) < chars.len() {
            chars[(self.lower - 1) as usize]
        } else {
            '!'
        };
        let upper = if ((self.upper - 1) as usize) < chars.len() {
            chars[(self.upper - 1) as usize]
        } else {
            '!'
        };

        (lower == self.enforced_char || upper == self.enforced_char) && lower != upper
    }
}

fn from_digit(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn str_to_char(input: &str) -> Result<char, std::num::ParseIntError> {
    let chars: Vec<char> = input.chars().collect();

    Ok(chars[0])
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn digit(input: &str) -> IResult<&str, u8> {
    map_res(take_while(is_digit), from_digit)(input)
}

fn take1(input: &str) -> IResult<&str, char> {
    map_res(take_while_m_n(1, 1, |_| true), str_to_char)(input)
}

fn password(input: &str) -> IResult<&str, Vec<Password>> {
    let mut passwords = vec![];

    for input in input.split('\n') {
        if input.is_empty() {
            break;
        }
        let (input, (lower, _, upper)) = tuple((digit, tag("-"), digit))(input)?;
        let (input, _) = tag(" ")(input)?;
        let (_input, (enforced_char, _, plain_text)) = tuple((take1, tag(": "), alpha1))(input)?;
        passwords.push(Password {
            lower,
            upper,
            enforced_char,
            plain_text: plain_text.to_string(),
        });
    }
    Ok((input, passwords))
}

pub fn run() -> Result<(), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day2-input")?.parse()?;

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_1(&input_vec)?;
    let result_2 = calculate_part_2(&input_vec)?;

    println!("result day 2 part 1 {}", result_1);
    println!("result day 2 part 2 {}", result_2);

    Ok(())
}

fn calculate_part_1(passwords: &[Password]) -> Result<i32, anyhow::Error> {
    let mut count = 0;
    for p in passwords {
        if p.validate_rule1() {
            count += 1;
        }
    }
    Ok(count)
}

fn calculate_part_2(passwords: &[Password]) -> Result<i32, anyhow::Error> {
    let mut count = 0;
    for p in passwords {
        if p.validate_rule2() {
            count += 1;
        }
    }
    Ok(count)
}

#[test]
fn part1() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_1(&input_vec).unwrap();

    assert_eq!(2, result_1);
}

#[test]
fn part2() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_2(&input_vec).unwrap();

    assert_eq!(1, result_1);
}

#[test]
fn part2_1() {
    let input = "1-3 a: abcde";

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_2(&input_vec).unwrap();

    assert_eq!(1, result_1);
}

#[test]
fn part2_2() {
    let input = "1-3 b: cdefg";

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_2(&input_vec).unwrap();

    assert_eq!(0, result_1);
}

#[test]
fn part2_3() {
    let input = "2-9 c: ccccccccc";

    let (_, input_vec) = password(&input).unwrap();

    let result_1 = calculate_part_2(&input_vec).unwrap();

    assert_eq!(0, result_1);
}
