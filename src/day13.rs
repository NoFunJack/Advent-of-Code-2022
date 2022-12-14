use std::{cmp::Ordering, iter::Peekable, str::Chars};

use Paket::*;

#[derive(Debug, PartialEq, Clone)]
enum Paket {
    List(Vec<Paket>),
    Value(usize),
}

impl Paket {
    fn new(line: &str) -> Paket {
        let mut iter = line.chars().peekable();
        Paket::builder(&mut iter, ReaderState::Init)
    }

    fn builder(iter: &mut Peekable<Chars>, mut state: ReaderState) -> Paket {
        let mut root = Vec::new();

        loop {
            if let Some(c) = iter.next() {
                match c {
                    '[' => match state {
                        ReaderState::Init => {
                            state = ReaderState::InRoot;
                        }
                        ReaderState::InRoot => root.push(Paket::builder(iter, ReaderState::InRoot)),
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

impl PartialOrd for Paket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let (Value(left), Value(right)) = (self, other) {
            return Some(left.cmp(right));
        } else if let (List(left), List(right)) = (self, other) {
            let mut l_iter = left.iter();
            let mut r_iter = right.iter();
            loop {
                let l = l_iter.next();
                let r = r_iter.next();

                // check if one or both run out
                if l.is_none() && r.is_none() {
                    return Some(std::cmp::Ordering::Equal);
                } else if l.is_some() && r.is_none() {
                    return Some(std::cmp::Ordering::Greater);
                } else if l.is_none() && r.is_some() {
                    return Some(std::cmp::Ordering::Less);
                }

                // compare current
                if l != r {
                    return l.unwrap().partial_cmp(r.unwrap());
                }
            }
        } else {
            if let Value(v) = self {
                return List(vec![Value(*v)]).partial_cmp(other);
            } else if let Value(v) = other {
                return self.partial_cmp(&List(vec![Value(*v)]));
            }
        }

        todo!()
    }
}

impl Eq for Paket {}

impl Ord for Paket {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            Ordering::Equal
        } else {
            self.partial_cmp(other).unwrap()
        }
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            (
                Paket::new(lines.next().unwrap()),
                Paket::new(lines.next().unwrap()),
            )
        })
        .enumerate()
        .filter_map(|(i, pair)| if pair.0 < pair.1 { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    let div1 = Paket::new("[[2]]");
    let div2 = Paket::new("[[6]]");

    let mut msg_list: Vec<Paket> = input
        .split("\n\n")
        .flat_map(|pair| pair.lines().map(|l| Paket::new(l)))
        .collect();

    msg_list.push(div1.clone());
    msg_list.push(div2.clone());

    msg_list.sort();

    let check_pos: Vec<usize> = msg_list
        .iter()
        .enumerate()
        .filter_map(|(i, paket)| {
            if *paket == div1 || *paket == div2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", check_pos);

    check_pos.iter().product()
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
    fn test_cmp_values() {
        assert!(Paket::new("1") < Paket::new("2"));
        assert!(Paket::new("5") > Paket::new("2"));
        assert!(Paket::new("5") == Paket::new("5"));
    }

    #[test]
    fn test_cmp_list_neq_len() {
        assert!(Paket::new("[1]") < Paket::new("[1,2]"));
        assert!(Paket::new("[0,5,1]") > Paket::new("[0,5]"));
        assert!(Paket::new("[5,3,1]") == Paket::new("[5,3,1]"));
    }

    #[test]
    fn test_cmp_list_eq_len() {
        assert!(Paket::new("[1]") < Paket::new("[2]"));
        assert!(Paket::new("[0,5]") > Paket::new("[0,4]"));
        assert!(Paket::new("[5,3,1]") == Paket::new("[5,3,1]"));
    }

    #[test]
    fn test_compare_list_to_val() {
        assert_eq!(
            Paket::new("3").partial_cmp(&Paket::new("[3]")),
            Some(std::cmp::Ordering::Equal)
        );
        assert_eq!(
            Paket::new("[3]").partial_cmp(&Paket::new("3")),
            Some(std::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 13)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 140)
    }
}
