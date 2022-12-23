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
impl std::fmt::Display for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.0[0], self.0[1])
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
                    rate * (29 - i)
                } else {
                    0
                }
            })
            .sum()
    }

    fn potential(&self, map: &Map) -> usize {
        let used_steps = self.steps.len();

        let pot: usize = map
            .valves
            .iter()
            .filter(|(_, v)| v.rate > 0)
            .filter(|(id, _)| self.is_open(&id))
            .map(|(_, v)| v.rate * (30 - used_steps))
            .sum();

        pot + self.get_total_released(map)
    }

    fn is_open(&self, v_id: &ValveId) -> bool {
        self.steps.contains(&Open(v_id.clone()))
    }

    fn build_plan_with_step(&self, step: Step) -> Plan {
        let mut re = (*self).clone();
        re.steps.push(step);
        re
    }

    fn get_next_steps(&self, map: &Map) -> Vec<Step> {
        let curr_pos = &self.curr_pos();
        let current_valve = &map.valves.get(&curr_pos).unwrap();

        let mut re = Vec::new();
        // try to open valve
        if current_valve.rate > 0 && !self.is_open(&curr_pos) {
            re.push(Open(curr_pos.clone()))
        }

        // continue in cave
        for next_path in &current_valve.paths_to {
            re.push(GoTo(next_path.clone()));
        }
        re
    }

    fn curr_pos(&self) -> ValveId {
        if self.steps.is_empty() {
            return ValveId::new("AA");
        }

        match self.steps.last().unwrap() {
            GoTo(id) => id.clone(),
            Open(id) => id.clone(),
        }
    }
}

impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut plan_str = Vec::new();

        for step in &self.steps {
            match step {
                GoTo(id) => plan_str.push(format!(">{}", id)),
                Open(id) => plan_str.push(format!("x{}", id)),
            }
        }

        write!(f, "[{}]", &plan_str.join(","))
    }
}

fn find_best_plan(map: &Map) -> Plan {
    let mut pot_plans = vec![Plan::new()];

    for _ in 0..30 {
        let mut next_plans = Vec::new();
        // all plans take one step
        for plan in &pot_plans {
            for step in plan.get_next_steps(map) {
                next_plans.push(plan.build_plan_with_step(step));
            }
        }

        // only take the best paths at each position
        let mut next_best_plans = Vec::new();
        for v_id in map.valves.keys() {
            let best_plan_val = next_plans
                .iter()
                .filter(|p| p.curr_pos() == *v_id)
                .map(|p| p.get_total_released(map))
                .max();

            if let Some(v) = best_plan_val {
                next_plans.append(
                    next_plans
                        .iter()
                        .filter(|p| p.get_total_released(map) == best_plan_val)
                        .collect(),
                );
                next_best_plans.push(p.clone());
            }
        }

        pot_plans = next_best_plans;
    }

    pot_plans
        .into_iter()
        .max_by_key(|p| p.get_total_released(&map))
        .unwrap()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let best = find_best_plan(&map);
    best.steps
        .iter()
        .enumerate()
        .for_each(|(i, s)| println!("{}:\t{:?}", i, s));
    println!("### BEST: {}", best);
    // 1488 to low
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
