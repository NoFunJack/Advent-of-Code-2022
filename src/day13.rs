use std::{iter::Peekable, str::Chars};

use Paket::*;

struct Signal {
    left: Paket,
    right: Paket,
}

#[derive(Debug, PartialEq)]
enum Paket {
    List(Vec<Paket>),
    Value(usize),
}

impl Paket {
    fn new(line: &str) -> Paket {
        println!("building packet");
        let mut iter = line.chars().peekable();
        Paket::builder(&mut iter)
    }

    fn builder(iter: &mut Peekable<Chars>) -> Paket {
        println!(">>Next Builder");
        let mut root = Vec::new();
        let mut state = ReaderState::Init;

        loop {
            if let Some(c) = iter.next() {
                println!("reading: {} in state {:?}", c, state);
                println!("Vec: {:?}", root);
                match c {
                    '[' => match state {
                        ReaderState::Init => {
                            state = ReaderState::InRoot;
                        }
                        ReaderState::InRoot => todo!(),
                    },
                    ']' => match state {
                        ReaderState::Init => panic!(),
                        ReaderState::InRoot => return List(root),
                    },
                    '0'..='9' => match state {
                        ReaderState::Init => return Value(collect_num(c, iter).parse().unwrap()),
                        ReaderState::InRoot => {
                            root.push(Paket::new(&collect_num(c, iter)));
                        }
                    },
                    ',' => (),
                    _ => panic!("unkown char: {}", c),
                }
            } else {
                break;
            }
        }

        fn collect_num(c: char, iter: &mut Peekable<Chars>) -> String {
            let mut rest_of_num = String::new();
            while iter.peek().map_or(false, |d| d.is_digit(10)) {
                rest_of_num.push(iter.next().unwrap());
                println!("{}", rest_of_num);
            }
            let re = format!("{}{}", c, rest_of_num);
            re
        }

        panic!("Paket is not is not closed");
    }
}

#[derive(Debug)]
enum ReaderState {
    Init,
    InRoot,
}

#[aoc(day13, part1)]
fn part1(input: &str) -> u32 {
    todo!()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_read_paket_empty() {
        assert_eq!(Paket::new("[]"), List(Vec::new()));
    }

    #[test]
    fn test_read_flat_packet() {
        assert_eq!(
            Paket::new("[1,2,32]"),
            List(vec![Value(1), Value(2), Value(32)])
        );
    }

    #[test]
    fn test_read_deep_packet() {
        assert_eq!(
            Paket::new("[1,[2],[3,4]]"),
            List(vec![
                Value(1),
                List(vec![Value(2)]),
                List(vec![Value(3), Value(4)])
            ])
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 13)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }
}
