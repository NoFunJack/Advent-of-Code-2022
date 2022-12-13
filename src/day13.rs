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
        List(Vec::new())
    }
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
    fn test_read_paket() {
        assert_eq!(Paket::new("[]"), List(Vec::new()));
        assert_eq!(
            Paket::new("[1,2,3]"),
            List(vec![Value(1), Value(2), Value(3)])
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
