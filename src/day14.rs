use std::{collections::HashSet, fmt::Display};

use crate::day09::Point;

fn read(input: &str) -> Scan {
    let mut rocks = HashSet::new();

    for line in input.lines() {
        let mut coords = line.split(" -> ").map(|str| {
            let mut nums = str.split(",").map(|s| s.parse().unwrap());
            Point(nums.next().unwrap(), nums.next().unwrap())
        });

        let mut from = coords.next().expect("No start point");
        let mut to_opt = coords.next();

        while to_opt.is_some() {
            let to = to_opt.unwrap();

            if from.0 == to.0 {
                for y in from.1..=to.1 {
                    rocks.insert(Point(y, from.0));
                }
            } else if from.1 == to.1 {
                for y in from.0..=to.0 {
                    rocks.insert(Point(from.1, y));
                }
            } else {
                panic!("Diagonal found");
            }

            from = to;
            to_opt = coords.next();
        }
    }

    println!("{:?}", rocks);

    Scan { rocks }
}

struct Scan {
    rocks: HashSet<Point>,
}

impl Display for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_y = self.rocks.iter().map(|p| p.1).min().unwrap_or(0);
        let max_y = self.rocks.iter().map(|p| p.1).max().unwrap_or(0);
        let mut re = String::new();

        for row in 0..=self.rocks.iter().map(|p| p.0).max().unwrap_or(0) {
            for pos in min_y..=max_y {
                match self.rocks.contains(&Point(row, pos)) {
                    true => re.push('#'),
                    false => re.push('.'),
                }
            }
            re.push('\n');
        }
        write!(f, "{}", re)
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u32 {
    todo!()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_reader() {
        assert_eq!(
            read(EXAMPLE).to_string(),
            "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."
        )
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 157)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }
}
