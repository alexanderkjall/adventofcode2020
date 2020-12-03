use anyhow::anyhow;

struct Map {
    rows: Vec<Vec<bool>>,
}

impl Map {
    fn is_tree(&self, x: usize, y: usize) -> Result<bool, anyhow::Error> {
        if y > self.rows.len() {
            return Err(anyhow!("outside of map"));
        }

        let r = &self.rows[y];

        Ok(r[x % r.len()])
    }

    fn len(&self) -> usize {
        self.rows.len()
    }
}

pub fn run() -> Result<(), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day3-input")?.parse()?;

    let map = parse_map(&input)?;

    let result1 = calc_1(&map)?;

    println!("result day 3 part 1 {}", result1);

    Ok(())
}

fn parse_map(input: &str) -> Result<Map, anyhow::Error> {
    let mut rows: Vec<Vec<bool>> = vec![];

    for r in input.split('\n') {
        let mut row = vec![];
        for c in r.chars() {
            row.push(c == '#')
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }

    Ok(Map { rows })
}

fn calc_1(map: &Map) -> Result<i32, anyhow::Error> {
    let mut nr_of_trees = 0;
    for r in 0..map.len() {
        if map.is_tree(r * 3, r)? {
            nr_of_trees += 1;
        }
    }

    Ok(nr_of_trees)
}

#[test]
fn part1() {
    let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let map = parse_map(input).unwrap();

    let result1 = calc_1(&map).unwrap();

    assert_eq!(7, result1);
}
