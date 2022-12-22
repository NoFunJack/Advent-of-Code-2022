use std::collections::HashMap;

use Step::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

#[derive(Clone, Debug)]
struct Plan {
    steps: Vec<Step>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Step {
    GoTo(ValveId),
    Open(ValveId),
}

impl Plan {
    fn new() -> Self {
        Self { steps: Vec::new() }
    }

    fn get_total_released(&self, map: &Map) -> usize {
        self.steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                if let Open(v_id) = step {
                    let rate = map.valves.get(&v_id).unwrap().rate;
                    rate * (30 - i)
                } else {
                    0
                }
            })
            .sum()
    }

    fn is_open(&self, v_id: &ValveId) -> bool {
        self.steps.contains(&Open(v_id.clone()))
    }

    fn build_plan_with_step(&self, step: Step) -> Plan {
        let mut re = (*self).clone();
        re.steps.push(step);
        re
    }
}

fn find_best_plan(map: &Map) -> Plan {
    find_best_plan_int(map, Plan::new(), &ValveId::new("AA"))
}

fn find_best_plan_int(map: &Map, plan: Plan, current_pos: &ValveId) -> Plan {
    println!("{:?}", plan);
    // ancor
    if plan.steps.len() >= 30 {
        return plan;
    }
    let current_valve = &map.valves.get(&current_pos.clone()).unwrap();
    let mut plans_from_here = Vec::new();

    // try to open valve
    if current_valve.rate > 0 && !plan.is_open(&current_pos) {
        let p = plan.build_plan_with_step(Open(current_pos.clone()));
        plans_from_here.push(find_best_plan_int(map, p, current_pos))
    }

    // continue in cave
    for next_path in &current_valve.paths_to {
        let p = plan.build_plan_with_step(GoTo(next_path.clone()));
        plans_from_here.push(find_best_plan_int(map, p, &next_path));
    }

    plans_from_here
        .iter()
        .max_by_key(|p| p.get_total_released(map))
        .expect("reached a dead end")
        .clone()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let best = find_best_plan(&map);
    println!("### BEST: {:?}", best);
    best.get_total_released(&map)
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
