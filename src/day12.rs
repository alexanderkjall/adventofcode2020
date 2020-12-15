use anyhow::anyhow;

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

struct Order {
    direction: Direction,
    distance: i32,
}

fn turn(direction: Direction, steps: u8) -> Result<Direction, anyhow::Error> {
    match direction {
        Direction::NORTH => match steps {
            1 => Ok(Direction::EAST),
            2 => Ok(Direction::SOUTH),
            3 => Ok(Direction::WEST),
            _ => Err(anyhow!("unknown direction")),
        },
        Direction::SOUTH => match steps {
            1 => Ok(Direction::WEST),
            2 => Ok(Direction::NORTH),
            3 => Ok(Direction::EAST),
            _ => Err(anyhow!("unknown direction")),
        },
        Direction::EAST => match steps {
            1 => Ok(Direction::SOUTH),
            2 => Ok(Direction::WEST),
            3 => Ok(Direction::NORTH),
            _ => Err(anyhow!("unknown direction")),
        },
        Direction::WEST => match steps {
            1 => Ok(Direction::NORTH),
            2 => Ok(Direction::EAST),
            3 => Ok(Direction::SOUTH),
            _ => Err(anyhow!("unknown direction")),
        },
        _ => Err(anyhow!("unknown direction")),
    }
}

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day12-input")?.parse()?;

    let orders = parse(&input)?;
    let result_1 = move_ship(&orders)?;
    let result_2 = move_waypoint(&orders)?;

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn move_ship(orders: &[Order]) -> Result<i32, anyhow::Error> {
    let mut dir = Direction::EAST;
    let mut x = 0;
    let mut y = 0;

    for o in orders {
        match o.direction {
            Direction::NORTH => y -= o.distance,
            Direction::SOUTH => y += o.distance,
            Direction::EAST => x += o.distance,
            Direction::WEST => x -= o.distance,
            Direction::LEFT => match o.distance {
                90 => dir = turn(dir, 3)?,
                180 => dir = turn(dir, 2)?,
                270 => dir = turn(dir, 1)?,
                _ => return Err(anyhow!("unknown direction")),
            },
            Direction::RIGHT => match o.distance {
                90 => dir = turn(dir, 1)?,
                180 => dir = turn(dir, 2)?,
                270 => dir = turn(dir, 3)?,
                _ => return Err(anyhow!("unknown direction")),
            },
            Direction::FORWARD => match dir {
                Direction::NORTH => y -= o.distance,
                Direction::SOUTH => y += o.distance,
                Direction::EAST => x += o.distance,
                Direction::WEST => x -= o.distance,
                _ => return Err(anyhow!("unknown direction")),
            },
        }
    }
    Ok(x.abs() + y.abs())
}

fn move_waypoint(orders: &[Order]) -> Result<i32, anyhow::Error> {
    let mut boat_x = 0;
    let mut boat_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    for o in orders {
        match o.direction {
            Direction::NORTH => waypoint_y -= o.distance,
            Direction::SOUTH => waypoint_y += o.distance,
            Direction::EAST => waypoint_x += o.distance,
            Direction::WEST => waypoint_x -= o.distance,
            Direction::LEFT => match o.distance {
                90 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 3);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                180 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 2);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                270 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 1);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                _ => return Err(anyhow!("unknown direction")),
            },
            Direction::RIGHT => match o.distance {
                90 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 1);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                180 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 2);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                270 => {
                    let (x, y) = transpose(waypoint_x, waypoint_y, 3);
                    waypoint_x = x;
                    waypoint_y = y;
                }
                _ => return Err(anyhow!("unknown direction")),
            },
            Direction::FORWARD => {
                boat_x += waypoint_x * o.distance;
                boat_y += waypoint_y * o.distance;
            }
        }
    }
    Ok(boat_x.abs() + boat_y.abs())
}

fn transpose(x: i32, y: i32, steps: i32) -> (i32, i32) {
    match steps {
        1 => (-y, x),
        2 => (-x, -y),
        3 => (y, -x),
        _ => (0, 0),
    }
}

fn parse(input: &str) -> Result<Vec<Order>, anyhow::Error> {
    let mut ret = vec![];
    for l in input.trim().split('\n') {
        let dir = match l.chars().next().unwrap() {
            'N' => Ok(Direction::NORTH),
            'S' => Ok(Direction::SOUTH),
            'E' => Ok(Direction::EAST),
            'W' => Ok(Direction::WEST),
            'L' => Ok(Direction::LEFT),
            'R' => Ok(Direction::RIGHT),
            'F' => Ok(Direction::FORWARD),
            _ => Err(anyhow!("unknown direction")),
        }?;

        let dist = i32::from_str_radix(&l[1..], 10)?;

        ret.push(Order {
            direction: dir,
            distance: dist,
        })
    }

    Ok(ret)
}

#[test]
fn test_part_1() {
    let input = "F10
N3
F7
R90
F11";

    let orders = parse(&input).unwrap();
    let result_1 = move_ship(&orders).unwrap();

    assert_eq!(25, result_1);
}

#[test]
fn test_part_2() {
    let input = "F10
N3
F7
R90
F11";

    let orders = parse(&input).unwrap();
    let result_2 = move_waypoint(&orders).unwrap();

    assert_eq!(286, result_2);
}
