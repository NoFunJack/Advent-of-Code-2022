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

#[aoc(day15, part1)]
fn part1(input: &[Sensor]) -> u32 {
    println!("{:#?}", input);
    todo!()
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
    fn part1_test() {
        assert_eq!(part1(&read(EXAMPLE)[..]), 157)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&read(EXAMPLE)[..]), 70)
    }
}
