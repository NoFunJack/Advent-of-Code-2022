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

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Point(x, y)
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
    links: Vec<Point>,
    visited: HashSet<Point>,
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            links: vec![Point(0, 0); size + 1],
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
        *self.links.get_mut(0).unwrap() += dir.get_tuple();

        for i in 1..self.links.len() {
            let pos = State::get_pos(self.links.get(i - 1).unwrap(), self.links.get(i).unwrap());
            *self.links.get_mut(i).unwrap() = pos;
        }

        self.visited.insert(self.links.last().unwrap().clone());

        //println!("done step {:?} \n{}", dir, self);
    }

    fn get_pos(h_pos: &Point, t_pos: &Point) -> Point {
        let mut new_pos = t_pos.clone();
        // non diagonal
        if h_pos.0 == t_pos.0 || t_pos.1 == h_pos.1 {
            // ud
            match h_pos.0 - t_pos.0 {
                2 => new_pos.0 += 1,
                -1..=1 => (),
                -2 => new_pos.0 -= 1,
                _ => panic!("tail to far u/d"),
            }

            // lr
            match h_pos.1 - t_pos.1 {
                2 => new_pos.1 += 1,
                -1..=1 => (),
                -2 => new_pos.1 -= 1,
                _ => panic!("tail to far l/r"),
            }
        } else if (t_pos.0 - h_pos.0).abs() + (t_pos.1 - h_pos.1).abs() > 2 {
            // ud
            match h_pos.0 - t_pos.0 {
                1..=2 => new_pos.0 += 1,
                0 => (),
                -2..=-1 => new_pos.0 -= 1,
                _ => panic!("tail to far u/d"),
            }

            // lr
            match h_pos.1 - t_pos.1 {
                1..=2 => new_pos.1 += 1,
                0 => (),
                -2..=-1 => new_pos.1 -= 1,
                _ => panic!("tail to far l/r"),
            }
        }

        new_pos
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ub = min(
            *self.visited.iter().map(|Point(a, _)| a).min().unwrap_or(&0),
            *self.links.iter().map(|Point(a, _)| a).min().unwrap_or(&0),
        );
        let db = max(
            *self
                .visited
                .iter()
                .map(|Point(a, _)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
            *self
                .links
                .iter()
                .map(|Point(a, _)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
        );
        let lb = min(
            *self.visited.iter().map(|Point(_, a)| a).min().unwrap_or(&0),
            *self.links.iter().map(|Point(_, a)| a).min().unwrap_or(&0),
        );
        let rb = max(
            *self
                .visited
                .iter()
                .map(|Point(_, a)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
            *self
                .links
                .iter()
                .map(|Point(_, a)| a)
                .max()
                .unwrap_or(&std::i32::MAX),
        );

        let mut chars: Vec<char> = Vec::new();

        for row in ub..=db {
            for p in lb..=rb {
                if *self.links.first().unwrap() == Point(row, p) {
                    chars.push('H');
                } else if *self.links.last().unwrap() == Point(row, p) {
                    chars.push('T');
                } else if self.links.contains(&Point(row, p)) {
                    chars.push('*');
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
    let mut s = State::new(1);
    input.iter().for_each(|m| s.do_move(m));
    s.visited.len()
}

#[aoc(day9, part2)]
fn part2(input: &[Move]) -> usize {
    let mut s = State::new(9);
    input.iter().for_each(|m| s.do_move(m));
    s.visited.len()
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

    const EXAMPLE_STR_BIG: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

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
        assert_eq!(part2(&load()[..]), 1)
    }

    #[test]
    fn part2_test_big() {
        assert_eq!(part2(&read(EXAMPLE_STR_BIG)[..]), 36)
    }
}
