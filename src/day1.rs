use anyhow::anyhow;

pub fn run() -> Result<(), anyhow::Error> {
    let input:String = std::fs::read_to_string("res/day1-input")?.parse()?;

    let result = calculate(input.split("\n").map(|s| s.parse::<i32>().unwrap_or(0)).collect())?;

    println!("result part 1 {}", result);

    Ok(())
}

fn calculate(mut input: Vec<i32>) -> Result<i32, anyhow::Error> {
    input.sort();

    for i in &input {
        for j in &input {
            let add = i + j;

            if add > 2020 {
                break;
            }
            if add == 2020 {
                return Ok(i * j);
            }
        }
    }
    Err(anyhow!("no numbers add to 2020"))
}

#[test]
fn test_calculate() {
    assert_eq!(514579, calculate(vec![1721, 979, 366, 299, 675, 1456]).unwrap_or(0));
}