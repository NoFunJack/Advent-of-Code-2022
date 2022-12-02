pub struct Round1 {
    opponent: Shape,
    me: Shape,
}

impl Round1 {
    pub fn new(opponent: Shape, me: Shape) -> Self {
        Self { opponent, me }
    }
}

pub struct Round2 {
    opponent: Shape,
    outcome: Outcome,
}

impl Round2 {
    pub fn new(opponent: Shape, outcome: Outcome) -> Self {
        Self { opponent, outcome }
    }
}

#[derive(PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

pub enum Outcome {
    Lose,
    Draw,
    Win,
}

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

pub fn read_input_part1(input: &str) -> Vec<Round1> {
    return input
        .lines()
        .map(|l| {
            let mut inp = l.trim().split(' ').map(|d| d.parse().unwrap());
            Round1 {
                opponent: map_opp(inp.next().unwrap()),
                me: map_me(inp.next().unwrap()),
            }
        })
        .collect();

    fn map_me(c: char) -> Shape {
        match c {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("unkown shape {}", c),
        }
    }
}

pub fn read_input_part2(input: &str) -> Vec<Round2> {
    return input
        .lines()
        .map(|l| {
            let mut inp = l.trim().split(' ').map(|d| d.parse().unwrap());
            Round2 {
                opponent: map_opp(inp.next().unwrap()),
                outcome: map_outcome(inp.next().unwrap()),
            }
        })
        .collect();

    fn map_outcome(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("unkown Outcome {}", c),
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    read_input_part1(input)
        .iter()
        .map(|r| r.me.score() + match_points(&r.opponent, &r.me))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    read_input_part2(input)
        .iter()
        .map(|r| match r.outcome {
            Outcome::Lose => 0 + r.opponent.beats().score(),
            Outcome::Draw => 3 + r.opponent.score(),
            Outcome::Win => 6 + r.opponent.loses_to().score(),
        })
        .sum()
}

fn match_points(opp: &Shape, me: &Shape) -> i32 {
    if me.loses_to() == *opp {
        LOSE
    } else if me.beats() == *opp {
        WIN
    } else {
        DRAW
    }
}

fn map_opp(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("unkown shape {}", c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn example_part1() {
        let result = part1(INPUT);

        assert_eq!(result, 15)
    }

    #[test]
    fn example_part2() {
        let result = part2(INPUT);

        assert_eq!(result, 12)
    }
}
