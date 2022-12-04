use std::cmp;

#[derive(Debug)]
pub struct Work {
    e1: Intervall,
    e2: Intervall,
}

impl Work {
    pub fn new(e1: Intervall, e2: Intervall) -> Self {
        Self { e1, e2 }
    }

    fn common_work(&self) -> Option<Intervall> {
        self.e1.inter(&self.e2)
    }
}

#[derive(Debug, PartialEq)]
pub struct Intervall {
    start: i32,
    end: i32,
}

impl Intervall {
    fn new(start: i32, end: i32) -> Self {
        if end < start {
            panic!("unorderd input")
        }
        Self { start, end }
    }

    fn size(&self) -> i32 {
        (self.end - self.start).abs()
    }

    fn inter(&self, other: &Intervall) -> Option<Intervall> {
        let new = Intervall {
            start: cmp::max(self.start, other.start),
            end: cmp::min(self.end, other.end),
        };

        if new.start <= new.end {
            Some(new)
        } else {
            None
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Work> {
    input
        .lines()
        .map(|l| {
            let mut ranges = l.trim().split(',').map(|r| {
                let mut nums = r.split('-').map(|n| n.parse().unwrap());
                Intervall::new(nums.next().unwrap(), nums.next().unwrap())
            });
            Work::new(ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Work]) -> usize {
    input
        .iter()
        .filter(|w| {
            let min_int = cmp::min(w.e1.size(), w.e2.size());
            if let Some(common) = w.common_work() {
                common.size() == min_int
            } else {
                false
            }
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[Work]) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::{input_generator, part1, Intervall};

    static EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 2)
    }

    #[test]
    fn test_size() {
        assert_eq!(Intervall::new(0, 0).size(), 0);
        assert_eq!(Intervall::new(5, 15).size(), 10);
    }

    #[test]
    fn test_inter() {
        assert_eq!(Intervall::new(0, 0).inter(&Intervall::new(1, 1)), None);
        assert_eq!(
            Intervall::new(0, 5).inter(&Intervall::new(1, 1)),
            Some(Intervall::new(1, 1))
        );
        assert_eq!(
            Intervall::new(0, 5).inter(&Intervall::new(3, 7)),
            Some(Intervall::new(3, 5))
        );
    }
}
