use anyhow::anyhow;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Command {
    NOP,
    ACC,
    JMP,
}

#[derive(Clone, PartialEq)]
struct Line {
    command: Command,
    val: i32,
}

struct Program {
    program: Vec<Line>,
}

struct ProgramMutationIter {
    program: Program,
    pos: usize,
}

impl Iterator for ProgramMutationIter {
    type Item = Program;

    fn next(&mut self) -> Option<Program> {
        if self.pos > self.program.program.len() {
            return None;
        }

        let mut prog = self.program.program.to_vec();
        loop {
            if self.pos >= prog.len() {
                return None;
            }
            if prog[self.pos].command == Command::NOP {
                prog[self.pos] = Line {
                    command: Command::JMP,
                    val: prog[self.pos].val,
                };
                self.pos += 1;
                return Some(Program { program: prog });
            } else if prog[self.pos].command == Command::JMP {
                prog[self.pos] = Line {
                    command: Command::NOP,
                    val: prog[self.pos].val,
                };
                self.pos += 1;
                return Some(Program { program: prog });
            } else {
                self.pos += 1;
            }
        }
    }
}

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day8-input")?.parse()?;

    let program = parse_program(&input)?;

    let result_1 = execute_to_recursion(&program);

    let it = ProgramMutationIter { program, pos: 0 };
    let result_2: i32 = it
        .map(|p| match execute_to_end(&p) {
            Ok(acc) => acc,
            Err(_) => 0,
        })
        .sum();

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn execute_to_recursion(program: &Program) -> i32 {
    let mut acc = 0;
    let mut i = 0;
    let mut visited = HashSet::<i32>::new();

    loop {
        if visited.contains(&i) {
            break;
        }
        visited.insert(i);
        match program.program[i as usize].command {
            Command::NOP => i += 1,
            Command::ACC => {
                acc += program.program[i as usize].val;
                i += 1
            }
            Command::JMP => i += program.program[i as usize].val,
        }
    }
    acc
}

fn execute_to_end(program: &Program) -> Result<i32, anyhow::Error> {
    let mut acc = 0;
    let mut i = 0;
    let mut visited = HashSet::<i32>::new();

    loop {
        if visited.contains(&i) {
            return Err(anyhow!("recursion detected"));
        }
        if i as usize >= program.program.len() {
            break;
        }
        visited.insert(i);
        match program.program[i as usize].command {
            Command::NOP => i += 1,
            Command::ACC => {
                acc += program.program[i as usize].val;
                i += 1
            }
            Command::JMP => i += program.program[i as usize].val,
        }
    }
    Ok(acc)
}

fn parse_program(input: &str) -> Result<Program, anyhow::Error> {
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

    Ok(Program { program: ret })
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

#[test]
fn test_part_2() {
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

    let it = ProgramMutationIter { program, pos: 0 };
    let result_2 = it
        .map(|p| match execute_to_end(&p) {
            Ok(acc) => acc,
            Err(_) => 0,
        })
        .sum();

    assert_eq!(8, result_2);
}
