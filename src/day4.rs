use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::map_res;
use nom::lib::std::collections::HashMap;
use nom::sequence::tuple;
use nom::IResult;

struct Passport {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    #[allow(dead_code)]
    cid: Option<String>,
    hgt: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.ecl.is_some()
            && self.pid.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.byr.is_some()
            && self.iyr.is_some()
            && self.hgt.is_some()
    }
}

pub fn run() -> Result<(), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day4-input")?.parse()?;

    let passports = parse_passports(&input)?;

    let result_1 = passports.iter().filter(|p| p.is_valid()).count();

    println!("result day 4 part 1 {}", result_1);

    Ok(())
}

fn not_space_or_newline(c: char) -> bool {
    c != '\n' && c != ' '
}

fn from_str(input: &str) -> Result<&str, std::num::ParseIntError> {
    Ok(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    map_res(take_while(not_space_or_newline), from_str)(input)
}

fn passport_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    alt((
        tuple((tag("ecl:"), string)),
        tuple((tag("pid:"), string)),
        tuple((tag("eyr:"), string)),
        tuple((tag("hcl:"), string)),
        tuple((tag("byr:"), string)),
        tuple((tag("iyr:"), string)),
        tuple((tag("cid:"), string)),
        tuple((tag("hgt:"), string)),
    ))(input)
}

fn end_or_space(input: &str) -> IResult<&str, &str> {
    alt((tag("\n\n"), tag("\n"), tag(" "), tag("")))(input)
}

fn parse_passports(input: &str) -> Result<Vec<Passport>, anyhow::Error> {
    let mut end = false;
    let mut passports: Vec<Passport> = vec![];
    let mut state = HashMap::new();
    let mut i = input;
    while !end {
        let (input, (key, val)) = passport_tuple(i).map_err(|e| anyhow!(format!("{:?}", e)))?;
        state.insert(key, val.to_owned());
        let (input, e_or_s) = end_or_space(input).map_err(|e| anyhow!(format!("{:?}", e)))?;
        i = input;
        if e_or_s == "\n\n" {
            passports.push(Passport {
                ecl: state.get("ecl:").cloned(),
                pid: state.get("pid:").cloned(),
                eyr: state.get("eyr:").cloned(),
                hcl: state.get("hcl:").cloned(),
                byr: state.get("byr:").cloned(),
                iyr: state.get("iyr:").cloned(),
                cid: state.get("cid:").cloned(),
                hgt: state.get("hgt:").cloned(),
            });

            state.clear();
        }
        if i.is_empty() {
            passports.push(Passport {
                ecl: state.get("ecl:").cloned(),
                pid: state.get("pid:").cloned(),
                eyr: state.get("eyr:").cloned(),
                hcl: state.get("hcl:").cloned(),
                byr: state.get("byr:").cloned(),
                iyr: state.get("iyr:").cloned(),
                cid: state.get("cid:").cloned(),
                hgt: state.get("hgt:").cloned(),
            });
            end = true;
        }
    }

    Ok(passports)
}

#[test]
fn test_passport_tuple() {
    assert_eq!(Ok(("", ("ecl:", "gry"))), passport_tuple("ecl:gry"));
}

#[test]
fn test_parse() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let passports = parse_passports(&input).unwrap();

    let result_1 = passports.iter().filter(|p| p.is_valid()).count();

    assert_eq!(2, result_1);
}
