use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::map_res;
use nom::lib::std::collections::HashMap;
use nom::sequence::tuple;
use nom::{AsChar, IResult};

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

    fn validate_all_fields(&self) -> bool {
        validate_ecl(&self.ecl)
            && validate_pid(&self.pid)
            && validate_eyr(&self.eyr)
            && validate_hcl(&self.hcl)
            && validate_byr(&self.byr)
            && validate_iyr(&self.iyr)
            && validate_hgt(&self.hgt)
    }
}

fn validate_hgt(hgt: &Option<String>) -> bool {
    match hgt {
        Some(hgt) => {
            if hgt.ends_with("in") {
                let l = hgt[..hgt.len() - 2].parse::<i32>().unwrap();

                return (59..=76).contains(&l);
            }
            if hgt.ends_with("cm") {
                let l = hgt[..hgt.len() - 2].parse::<i32>().unwrap();

                return (150..=193).contains(&l);
            }
            false
        }
        None => false,
    }
}

fn validate_iyr(iyr: &Option<String>) -> bool {
    match iyr {
        Some(iyr) => {
            if iyr.len() != 4 || iyr.chars().take_while(|c| c.is_numeric()).count() != 4 {
                return false;
            }

            let year = iyr.parse::<i32>().unwrap();
            (2010..=2020).contains(&year)
        }
        None => false,
    }
}

fn validate_byr(byr: &Option<String>) -> bool {
    match byr {
        Some(byr) => {
            if byr.len() != 4 || byr.chars().take_while(|c| c.is_numeric()).count() != 4 {
                return false;
            }

            let year = byr.parse::<i32>().unwrap();
            (1920..=2002).contains(&year)
        }
        None => false,
    }
}

fn validate_hcl(hcl: &Option<String>) -> bool {
    match hcl {
        Some(hcl) => {
            if !hcl.starts_with('#') {
                return false;
            }

            hcl.chars().skip(1).take_while(|c| c.is_hex_digit()).count() == 6
        }
        None => false,
    }
}

fn validate_eyr(eyr: &Option<String>) -> bool {
    match eyr {
        Some(eyr) => {
            if eyr.len() != 4 || eyr.chars().take_while(|c| c.is_numeric()).count() != 4 {
                return false;
            }

            let year = eyr.parse::<i32>().unwrap();
            (2020..=2030).contains(&year)
        }
        None => false,
    }
}

fn validate_pid(pid: &Option<String>) -> bool {
    match pid {
        Some(pid) => pid.len() == 9 && pid.chars().take_while(|c| c.is_numeric()).count() == 9,
        None => false,
    }
}

fn validate_ecl(ecl: &Option<String>) -> bool {
    match ecl {
        Some(ecl) => {
            matches!(
                &ecl[..],
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
        }
        None => false,
    }
}

pub fn run() -> Result<(), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day4-input")?.parse()?;

    let passports = parse_passports(&input)?;

    let result_1 = passports.iter().filter(|p| p.is_valid()).count();
    let result_2 = passports.iter().filter(|p| p.validate_all_fields()).count();

    println!("result day 4 part 1 {}", result_1);
    println!("result day 4 part 2 {}", result_2);

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

#[test]
fn byr_valid() {
    assert_eq!(true, validate_byr(&Some("2002".to_string())))
}

#[test]
fn byr_invalid() {
    assert_eq!(false, validate_byr(&Some("2003".to_string())))
}

#[test]
fn hgt_valid1() {
    assert_eq!(true, validate_hgt(&Some("60in".to_string())))
}

#[test]
fn hgt_valid2() {
    assert_eq!(true, validate_hgt(&Some("190cm".to_string())))
}

#[test]
fn hgt_invalid1() {
    assert_eq!(false, validate_hgt(&Some("190in".to_string())))
}

#[test]
fn hgt_invalid2() {
    assert_eq!(false, validate_hgt(&Some("190".to_string())))
}

#[test]
fn hcl_valid() {
    assert_eq!(true, validate_hcl(&Some("#123abc".to_string())))
}

#[test]
fn hcl_invalid1() {
    assert_eq!(false, validate_byr(&Some("#123abz".to_string())))
}

#[test]
fn hcl_invalid2() {
    assert_eq!(false, validate_byr(&Some("123abc".to_string())))
}

#[test]
fn ecl_valid() {
    assert_eq!(true, validate_ecl(&Some("brn".to_string())))
}

#[test]
fn ecl_invalid() {
    assert_eq!(false, validate_ecl(&Some("wat".to_string())))
}

#[test]
fn pid_valid() {
    assert_eq!(true, validate_pid(&Some("000000001".to_string())))
}

#[test]
fn pid_invalid() {
    assert_eq!(false, validate_pid(&Some("0123456789".to_string())))
}

#[test]
fn test_strict_validation_all_invalid() {
    let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    let passports = parse_passports(&input).unwrap();

    let result_1 = passports.iter().filter(|p| p.validate_all_fields()).count();

    assert_eq!(4, passports.len());
    assert_eq!(0, result_1);
}

#[test]
fn test_strict_validation_all_valid() {
    let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    let passports = parse_passports(&input).unwrap();

    let result_1 = passports.iter().filter(|p| p.validate_all_fields()).count();

    assert_eq!(4, passports.len());
    assert_eq!(4, result_1);
}
