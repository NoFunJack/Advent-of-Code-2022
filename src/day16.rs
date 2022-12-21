use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ValveId([char; 2]);

impl ValveId {
    fn new(code: &str) -> ValveId {
        if code.len() != 2 {
            panic!("Codes have to be length 2");
        }

        let mut chars = code.chars();

        ValveId([chars.next().unwrap(), chars.next().unwrap()])
    }
}

struct Map {
    valves: HashMap<ValveId, Valve>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut valves = HashMap::new();
        for line in input.lines() {
            let mut words = line.split_whitespace().skip(1);
            let valveid = ValveId::new(&words.next().unwrap());
            let rate: usize = words
                .nth(2)
                .unwrap()
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(";")
                .unwrap()
                .parse()
                .unwrap();

            let connections: Vec<ValveId> = words
                .skip(4)
                .map(|s| s.strip_suffix(",").unwrap_or(s))
                .map(|s| ValveId::new(s))
                .collect();

            valves.insert(valveid, Valve::new(rate, connections));
        }

        Map { valves }
    }
}

#[derive(Debug, PartialEq)]
struct Valve {
    rate: usize,
    paths_to: Vec<ValveId>,
}

impl Valve {
    fn new(rate: usize, paths_to: Vec<ValveId>) -> Self {
        Self { rate, paths_to }
    }
}

struct Plan {
    steps: [Option<Step>; 30],
}

enum Step {
    GoTo(ValveId),
    Open(ValveId),
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u32 {
    todo!()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_build_map() {
        let m = Map::new(EXAMPLE);

        assert_eq!(
            m.valves.get(&ValveId::new("AA")),
            Some(&Valve::new(
                0,
                vec![ValveId::new("DD"), ValveId::new("II"), ValveId::new("BB")]
            ))
        );
        assert_eq!(
            m.valves.get(&ValveId::new("HH")),
            Some(&Valve::new(22, vec![ValveId::new("GG")]))
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 1651)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }
}
