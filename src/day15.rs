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
    fn get_scanned_on_line(&self, l_num: i32) -> Option<(i32, i32)> {
        let dist = (self.pos.1 - l_num).abs();
        if dist > self.scan_range() {
            return None;
        } else {
            let lr = self.scan_range() - dist;
            return Some((self.pos.0 - lr, self.pos.0 + lr));
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

fn get_num_non_becon_spaces(sensors: &[Sensor], l_num: i32) -> i32 {
    if sensors.is_empty() {
        return 0;
    }

    let ranges: Vec<(i32, i32)> = sensors
        .iter()
        .filter_map(|s| s.get_scanned_on_line(l_num))
        .collect();

    let min = ranges.iter().map(|t| t.0).min().unwrap();
    let max = ranges.iter().map(|t| t.1).max().unwrap();

    let mut non_becon = 0;

    for i in min..=max {
        // in some range
        if ranges.iter().any(|t| t.0 <= i && t.1 >= i)
        // not a known beacon
            && !sensors.iter().any(|s| s.closest_beacon == Point(i, l_num))
        {
            non_becon += 1;
        }
    }

    return non_becon;
}

#[aoc(day15, part1)]
fn part1(input: &[Sensor]) -> i32 {
    get_num_non_becon_spaces(input, 2000000)
}

fn find_hole_in_square(sensors: &[Sensor], square_size: i32) -> i32 {
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
    fn test_nonb_empty() {
        assert_eq!(get_num_non_becon_spaces(&Vec::new()[..], 0), 0);
    }

    #[test]
    fn test_nonb_sensor() {
        let s = Sensor {
            pos: Point(0, 0),
            closest_beacon: Point(5, 0),
        };

        assert_eq!(s.get_scanned_on_line(6), None);
        assert_eq!(s.get_scanned_on_line(5), Some((0, 0)));
        assert_eq!(s.get_scanned_on_line(4), Some((-1, 1)));
        assert_eq!(s.get_scanned_on_line(3).unwrap(), (-2, 2));
        assert_eq!(s.get_scanned_on_line(2).unwrap(), (-3, 3));
        assert_eq!(s.get_scanned_on_line(1).unwrap(), (-4, 4));
        assert_eq!(s.get_scanned_on_line(0).unwrap(), (-5, 5));
        assert_eq!(s.get_scanned_on_line(-1).unwrap(), (-4, 4));
        assert_eq!(s.get_scanned_on_line(-2).unwrap(), (-3, 3));
        assert_eq!(s.get_scanned_on_line(-3).unwrap(), (-2, 2));
        assert_eq!(s.get_scanned_on_line(-4).unwrap(), (-1, 1));
        assert_eq!(s.get_scanned_on_line(-5).unwrap(), (-0, 0));
        assert_eq!(s.get_scanned_on_line(-6), None);
    }

    #[test]
    fn part1_test() {
        let input = &read(EXAMPLE)[..];
        assert_eq!(get_num_non_becon_spaces(&input, 10), 26);
    }

    #[test]
    fn part2_test() {
        assert_eq!(find_hole_in_square(&input, 20), 56000011);
    }
}
