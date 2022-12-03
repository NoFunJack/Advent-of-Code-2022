#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| find_common(&l[0..l.len() / 2], &l[l.len() / 2..l.len()]))
        .map(|c| prio(c))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    0
}

fn prio(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 64 + 26,
        _ => panic!("Not a letter {}", c),
    }
}

fn find_common(p1: &str, p2: &str) -> char {
    p1.chars()
        .find(|c| p2.contains(|t| t == *c))
        .expect("No common letter found")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 157)
    }

    #[test]
    fn test_char_to_prio() {
        assert_eq!(prio('a'), 1);
        assert_eq!(prio('z'), 26);
        assert_eq!(prio('A'), 27);
        assert_eq!(prio('Z'), 52);
    }

    #[test]
    fn test_find_common() {
        assert_eq!(find_common("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
        assert_eq!(find_common("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(find_common("PmmdzqPrV", "vPwwTWBwg"), 'P');
    }
}
