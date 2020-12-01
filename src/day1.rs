use anyhow::anyhow;

pub fn run() -> Result<(), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day1-input")?.parse()?;

    let mut input_vec: Vec<i32> = input
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .filter(|i| i > &0)
        .collect();
    input_vec.sort_unstable();

    let result_1 = calculate_part_1(&input_vec)?;
    let result_2 = calculate_part_2(&input_vec)?;

    println!("result day 1 part 1 {}", result_1);
    println!("result day 1 part 2 {}", result_2);

    Ok(())
}

fn calculate_part_1(input: &[i32]) -> Result<i32, anyhow::Error> {
    for i in input {
        for j in input {
            let add = i + j;

            match add {
                2021..=std::i32::MAX => break,
                2020 => return Ok(i * j),
                _ => {}
            }
        }
    }

    Err(anyhow!("no two numbers add to 2020"))
}

fn calculate_part_2(input: &[i32]) -> Result<i32, anyhow::Error> {
    for i in input {
        for j in input {
            for k in input {
                let add = i + j + k;

                match add {
                    2021..=std::i32::MAX => break,
                    2020 => return Ok(i * j * k),
                    _ => {}
                }
            }
        }
    }

    Err(anyhow!("no three numbers add to 2020"))
}

#[test]
fn test_calculate_part_1() {
    let mut input: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];
    input.sort_unstable();
    assert_eq!(514579, calculate_part_1(&input).unwrap_or(0));
}

#[test]
fn test_calculate_part_2() {
    let mut input: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];
    input.sort_unstable();
    assert_eq!(241861950, calculate_part_2(&input).unwrap_or(0));
}
