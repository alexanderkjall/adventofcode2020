use anyhow::anyhow;

pub fn run() -> Result<(), anyhow::Error> {
    let input:String = std::fs::read_to_string("res/day1-input")?.parse()?;

    let input_vec:Vec<i32> = input.split("\n").map(|s| s.parse::<i32>().unwrap_or(0)).filter(|i| i > &0).collect();
    let result_1 = calculate_part_1(input_vec.clone())?;
    let result_2 = calculate_part_2(input_vec)?;

    println!("result day 1 part 1 {}", result_1);
    println!("result day 1 part 2 {}", result_2);

    Ok(())
}

fn calculate_part_1(mut input: Vec<i32>) -> Result<i32, anyhow::Error> {
    input.sort();

    for i in &input {
        for j in &input {
            let add = i + j;

            if add > 2020 {
                break;
            }
            else if add == 2020 {
                return Ok(i * j);
            }
        }
    }

    Err(anyhow!("no two numbers add to 2020"))
}

fn calculate_part_2(mut input: Vec<i32>) -> Result<i32, anyhow::Error> {
    input.sort();

    for i in &input {
        for j in &input {
            for k in &input {
                let add = i + j + k;

                if add > 2020 {
                    break;
                }
                else if add == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }

    Err(anyhow!("no three numbers add to 2020"))
}

#[test]
fn test_calculate_part_1() {
    assert_eq!(514579, calculate_part_1(vec![1721, 979, 366, 299, 675, 1456]).unwrap_or(0));
}

#[test]
fn test_calculate_part_2() {
    assert_eq!(241861950, calculate_part_2(vec![1721, 979, 366, 299, 675, 1456]).unwrap_or(0));
}
