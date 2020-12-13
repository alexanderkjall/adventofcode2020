use anyhow::anyhow;
use std::collections::HashSet;

enum Command {
    NOP,
    ACC,
    JMP,
}

struct Line {
    command: Command,
    val: i32,
}

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day8-input")?.parse()?;

    let program = parse_program(&input)?;

    let result_1 = execute_to_recursion(&program);

    Ok((format!("{}", result_1), format!("")))
}

fn execute_to_recursion(program: &[Line]) -> i32 {
    let mut acc = 0;
    let mut i = 0;
    let mut visited = HashSet::<i32>::new();

    loop {
        if visited.contains(&i) {
            break;
        }
        visited.insert(i);
        match program[i as usize].command {
            Command::NOP => i += 1,
            Command::ACC => {
                acc += program[i as usize].val;
                i += 1
            }
            Command::JMP => i += program[i as usize].val,
        }
    }
    acc
}

fn parse_program(input: &str) -> Result<Vec<Line>, anyhow::Error> {
    let mut ret = vec![];

    for l in input.trim().split('\n') {
        let c = match &l[0..3] {
            "nop" => Ok(Command::NOP),
            "acc" => Ok(Command::ACC),
            "jmp" => Ok(Command::JMP),
            _ => Err(anyhow!("unknown command")),
        }?;

        let amount = i32::from_str_radix(&l[4..], 10)?;

        ret.push(Line {
            command: c,
            val: amount,
        })
    }

    Ok(ret)
}

#[test]
fn test_part_1() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    let program = parse_program(&input).unwrap();

    let result_1 = execute_to_recursion(&program);

    assert_eq!(5, result_1);
}
