use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::AddAssign,
};

use Direction::*;

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    length: i32,
}

impl Move {
    fn new(direction: Direction, length: i32) -> Self {
        Self { direction, length }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn get_tuple(&self) -> Point {
        match self {
            U => Point(-1, 0),
            D => Point(1, 0),
            L => Point(0, -1),
            R => Point(0, 1),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[aoc_generator(day9)]
fn read(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let dir = match words.next().unwrap() {
                "U" => U,
                "D" => D,
                "L" => L,
                "R" => R,
                _ => panic!("Unkown Direction"),
            };
            let size = words.next().unwrap().parse().unwrap();
            Move::new(dir, size)
        })
        .collect()
}

struct State {
    h_pos: Point,
    t_pos: Point,
    visited: HashSet<Point>,
}

impl State {
    fn new() -> Self {
        Self {
            h_pos: Point(0, 0),
            t_pos: Point(0, 0),
            visited: HashSet::new(),
        }
    }

    fn do_move(&mut self, mov: &Move) {
        //println!("Do_move {:#?}", mov);
        for _ in 0..mov.length {
            self.move_step(&mov.direction);
        }
    }

    // moves one step
    fn move_step(&mut self, dir: &Direction) {
        self.h_pos += dir.get_tuple();
        self.move_tail();
        //println!("done step {:?} \n{}", dir, self);
    }

    fn move_tail(&mut self) {
        // non diagonal
        if self.h_pos.0 == self.t_pos.0 || self.t_pos.1 == self.h_pos.1 {
            // ud
            match self.h_pos.0 - self.t_pos.0 {
                2 => self.t_pos.0 += 1,
                -1..=1 => (),
                -2 => self.t_pos.0 -= 1,
                _ => panic!("tail to far u/d"),
            }

            // lr
            match self.h_pos.1 - self.t_pos.1 {
                2 => self.t_pos.1 += 1,
                -1..=1 => (),
                -2 => self.t_pos.1 -= 1,
                _ => panic!("tail to far l/r"),
            }
        } else if (self.t_pos.0 - self.h_pos.0).abs() + (self.t_pos.1 - self.h_pos.1).abs() > 2 {
            // ud
            match self.h_pos.0 - self.t_pos.0 {
                1..=2 => self.t_pos.0 += 1,
                0 => (),
                -2..=-1 => self.t_pos.0 -= 1,
                _ => panic!("tail to far u/d"),
            }

            // lr
            match self.h_pos.1 - self.t_pos.1 {
                1..=2 => self.t_pos.1 += 1,
                0 => (),
                -2..=-1 => self.t_pos.1 -= 1,
                _ => panic!("tail to far l/r"),
            }
        }

        self.visited.insert(self.t_pos.clone());
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ub = min(
            *self.visited.iter().map(|Point(a, _)| a).min().unwrap_or(&0),
            min(self.t_pos.0, self.h_pos.0),
        );
        let db = max(
            *self
                .visited
                .iter()
                .map(|Point(a, _)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
            max(self.t_pos.0, self.h_pos.0),
        );
        let lb = min(
            *self.visited.iter().map(|Point(_, a)| a).min().unwrap_or(&0),
            min(self.t_pos.1, self.h_pos.1),
        );
        let rb = max(
            *self
                .visited
                .iter()
                .map(|Point(_, a)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
            max(self.t_pos.1, self.h_pos.1),
        );

        let mut chars: Vec<char> = Vec::new();

        for row in ub..=db {
            for p in lb..=rb {
                if self.h_pos == Point(row, p) {
                    chars.push('H');
                } else if self.t_pos == Point(row, p) {
                    chars.push('T');
                } else if self.visited.contains(&Point(row, p)) {
                    chars.push('#');
                } else {
                    chars.push('.');
                }
            }
            chars.push('\n')
        }

        write!(f, "{}", chars.iter().collect::<String>().trim())
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Move]) -> usize {
    let mut s = State::new();
    input.iter().for_each(|m| s.do_move(m));
    s.visited.len()
}

#[aoc(day9, part2)]
fn part2(input: &[Move]) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::Direction::*;
    use super::*;

    const EXAMPLE_STR: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    fn load() -> Vec<Move> {
        vec![
            Move::new(R, 4),
            Move::new(U, 4),
            Move::new(L, 3),
            Move::new(D, 1),
            Move::new(R, 4),
            Move::new(D, 1),
            Move::new(L, 5),
            Move::new(R, 2),
        ]
    }

    #[test]
    fn test_reader() {
        assert_eq!(read(EXAMPLE_STR), load())
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&load()[..]), 13)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&load()[..]), 70)
    }
}
