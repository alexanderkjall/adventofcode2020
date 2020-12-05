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

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day3-input")?.parse()?;

    let map = parse_map(&input)?;

    let result1 = calc_1(&map)?;

    let mut result2: u64 = 1;
    for (d_x, d_y) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result2 *= calc_2(&map, d_x, d_y).unwrap();
    }

    Ok((format!("{}", result1), format!("{}", result2)))
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

fn calc_1(map: &Map) -> Result<u64, anyhow::Error> {
    let mut nr_of_trees = 0;
    for r in 0..map.len() {
        if map.is_tree(r * 3, r)? {
            nr_of_trees += 1;
        }
    }

    Ok(nr_of_trees)
}

fn calc_2(map: &Map, d_x: &usize, d_y: &usize) -> Result<u64, anyhow::Error> {
    let mut nr_of_trees = 0;
    for (i, r) in (0..map.len()).step_by(*d_y).enumerate() {
        if map.is_tree(i * d_x, r)? {
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

#[test]
fn part2() {
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

    let mut result2 = 1;
    for (d_x, d_y) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result2 *= calc_2(&map, d_x, d_y).unwrap();
    }

    assert_eq!(336, result2);
}
