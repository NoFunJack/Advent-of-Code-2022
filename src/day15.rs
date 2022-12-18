use std::collections::HashSet;

use crate::day09::Point;
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day15)]
pub fn read(input: &str) -> Vec<Sensor> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    return input
        .lines()
        .map(|line| {
            let cap_vec: Vec<i32> = RE
                .captures_iter(line)
                .flat_map(|caps| {
                    caps.iter()
                        .skip(1)
                        .map(|cap| cap.unwrap().as_str().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                        .clone()
                })
                .collect();

            let mut cap = cap_vec.into_iter();

            Sensor {
                pos: Point(cap.next().unwrap(), cap.next().unwrap()),
                closest_beacon: Point(cap.next().unwrap(), cap.next().unwrap()),
            }
        })
        .collect();
}

#[derive(Debug, PartialEq)]
pub struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn get_non_becon_spaces_on_line(&self, l_num: i32) -> HashSet<Point> {
        let dist = (self.pos.1 - l_num).abs();
        if dist > self.scan_range() {
            return HashSet::new();
        } else {
            let lr = self.scan_range() - dist;
            let mut re = HashSet::new();
            re.insert(Point(self.pos.0, l_num));
            for i in 1..=lr {
                re.insert(Point(self.pos.0 - i, l_num));
                re.insert(Point(self.pos.0 + i, l_num));
            }
            re.remove(&self.closest_beacon);
            return re;
        }
    }

    fn scan_range(&self) -> i32 {
        self.pos.man_dist(&self.closest_beacon)
    }
}

impl Point {
    fn man_dist(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn get_non_becon_spaces(sensors: &[Sensor], l_num: i32) -> HashSet<Point> {
    sensors
        .iter()
        .map(|s| s.get_non_becon_spaces_on_line(l_num))
        .fold(HashSet::new(), |a, b| a.into_iter().chain(b).collect())
}

#[aoc(day15, part1)]
fn part1(input: &[Sensor]) -> usize {
    get_non_becon_spaces(input, 2000000).len()
}

#[aoc(day15, part2)]
fn part2(input: &[Sensor]) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_reader() {
        let list = read(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        );

        assert_eq!(
            list,
            vec![
                Sensor {
                    pos: Point(2, 18),
                    closest_beacon: Point(-2, 15),
                },
                Sensor {
                    pos: Point(9, 16),
                    closest_beacon: Point(10, 16)
                }
            ]
        )
    }

    #[test]
    fn test_nonb_empty() {
        assert_eq!(get_non_becon_spaces(&Vec::new()[..], 0), HashSet::new());
    }

    #[test]
    fn test_nonb_sensor() {
        let s = Sensor {
            pos: Point(0, 0),
            closest_beacon: Point(5, 0),
        };

        assert_eq!(s.get_non_becon_spaces_on_line(6), HashSet::new());
        let mut exp = HashSet::new();
        exp.insert(Point(0, 5));
        assert_eq!(s.get_non_becon_spaces_on_line(5), exp);
        let mut exp = HashSet::new();
        exp.insert(Point(0, 4));
        exp.insert(Point(1, 4));
        exp.insert(Point(-1, 4));
        assert_eq!(s.get_non_becon_spaces_on_line(4), exp);
        assert_eq!(s.get_non_becon_spaces_on_line(3).len(), 5);
        assert_eq!(s.get_non_becon_spaces_on_line(2).len(), 7);
        assert_eq!(s.get_non_becon_spaces_on_line(1).len(), 9);
        // same pos as beacon
        assert_eq!(s.get_non_becon_spaces_on_line(0).len(), 10);
        assert_eq!(s.get_non_becon_spaces_on_line(-1).len(), 9);
        assert_eq!(s.get_non_becon_spaces_on_line(-2).len(), 7);
        assert_eq!(s.get_non_becon_spaces_on_line(-3).len(), 5);
        assert_eq!(s.get_non_becon_spaces_on_line(-4).len(), 3);
        assert_eq!(s.get_non_becon_spaces_on_line(-5).len(), 1);
        assert_eq!(s.get_non_becon_spaces_on_line(-6).len(), 0);
    }

    #[test]
    fn part1_test() {
        let input = &read(EXAMPLE)[..];
        assert_eq!(get_non_becon_spaces(&input, 10).len(), 26);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&read(EXAMPLE)[..]), 70)
    }
}
