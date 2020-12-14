use anyhow::anyhow;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Hash, PartialEq, Debug)]
enum Tile {
    EMPTY,
    FULL,
    FLOOR,
}

#[derive(Clone)]
struct GameOfSeats {
    board: Vec<Vec<Tile>>,
}

impl GameOfSeats {
    fn hash(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        self.board.hash(&mut hash);
        hash.finish()
    }

    fn step(&mut self) {
        let old_state = self.clone();
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                let number_seated = old_state.number_seated_round(x as i32, y as i32);
                if old_state.board[y][x] == Tile::FULL && number_seated > 3 {
                    self.board[y][x] = Tile::EMPTY;
                }
                if old_state.board[y][x] == Tile::EMPTY && number_seated == 0 {
                    self.board[y][x] = Tile::FULL;
                }
            }
        }
    }

    fn step_line(&mut self) {
        let old_state = self.clone();
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                let number_seated = old_state.number_seated_round_line(x as i32, y as i32);
                if old_state.board[y][x] == Tile::FULL && number_seated > 4 {
                    self.board[y][x] = Tile::EMPTY;
                }
                if old_state.board[y][x] == Tile::EMPTY && number_seated == 0 {
                    self.board[y][x] = Tile::FULL;
                }
            }
        }
    }

    fn number_seated_round(&self, x: i32, y: i32) -> i32 {
        self.seated(x - 1, y - 1)
            + self.seated(x, y - 1)
            + self.seated(x + 1, y - 1)
            + self.seated(x - 1, y)
            + self.seated(x + 1, y)
            + self.seated(x - 1, y + 1)
            + self.seated(x, y + 1)
            + self.seated(x + 1, y + 1)
    }

    fn seated(&self, x: i32, y: i32) -> i32 {
        if y < 0 || y >= (self.board.len() as i32) || x < 0 || x >= (self.board[0].len() as i32) {
            0
        } else if self.board[y as usize][x as usize] == Tile::FULL {
            1
        } else {
            0
        }
    }

    fn number_seated_round_line(&self, x: i32, y: i32) -> i32 {
        self.seated_line(x, y, -1, -1)
            + self.seated_line(x, y, 0, -1)
            + self.seated_line(x, y, 1, -1)
            + self.seated_line(x, y, -1, 0)
            + self.seated_line(x, y, 1, 0)
            + self.seated_line(x, y, -1, 1)
            + self.seated_line(x, y, 0, 1)
            + self.seated_line(x, y, 1, 1)
    }

    fn seated_line(&self, mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
        loop {
            x += dx;
            y += dy;

            if y < 0
                || y >= (self.board.len() as i32)
                || x < 0
                || x >= (self.board[0].len() as i32)
                || self.board[y as usize][x as usize] == Tile::EMPTY
            {
                return 0;
            } else if self.board[y as usize][x as usize] == Tile::FULL {
                return 1;
            }
        }
    }

    fn number_of_full_seats(&self) -> usize {
        self.board
            .iter()
            .map(|r| r.iter().filter(|c| **c == Tile::FULL).count())
            .sum()
    }
}

pub fn run() -> Result<(String, String), anyhow::Error> {
    let input: String = std::fs::read_to_string("res/day11-input")?.parse()?;

    let board = parse(&input)?;
    let result_1 = iterate_until_stable(board.clone());
    let result_2 = iterate_until_stable_line(board);

    Ok((format!("{}", result_1), format!("{}", result_2)))
}

fn iterate_until_stable(mut board: GameOfSeats) -> usize {
    let mut old_hash = board.hash();
    board.step();
    while old_hash != board.hash() {
        old_hash = board.hash();
        board.step();
    }
    board.number_of_full_seats()
}

fn iterate_until_stable_line(mut board: GameOfSeats) -> usize {
    let mut old_hash = board.hash();
    board.step_line();
    while old_hash != board.hash() {
        old_hash = board.hash();
        board.step_line();
    }
    board.number_of_full_seats()
}

fn parse(input: &str) -> Result<GameOfSeats, anyhow::Error> {
    let mut board = vec![];
    for line in input.trim().split('\n') {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                'L' => Ok(Tile::EMPTY),
                '#' => Ok(Tile::FULL),
                '.' => Ok(Tile::FLOOR),
                _ => Err(anyhow!("not a map char")),
            }?)
        }
        board.push(row);
    }

    Ok(GameOfSeats { board })
}

#[test]
fn test_part_1() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let board = parse(&input).unwrap();
    let result_1 = iterate_until_stable(board);

    assert_eq!(37, result_1);
}

#[test]
fn test_part_2_single_empty() {
    let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

    let board = parse(&input).unwrap();
    let result_1 = board.number_seated_round_line(3, 4);

    assert_eq!(8, result_1);
}

#[test]
fn test_part_2() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let board = parse(&input).unwrap();
    let result_1 = iterate_until_stable_line(board);

    assert_eq!(26, result_1);
}
