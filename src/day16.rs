use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

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

#[derive(Debug)]
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
    map: Rc<Map>,
    pos: ValveId,
    open_valves: HashSet<ValveId>,
    relased: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Step {
    GoTo(ValveId),
    Open(ValveId),
}

impl Plan {
    fn new(map: &Rc<Map>) -> Self {
        Self {
            map: Rc::clone(map),
            pos: ValveId::new("AA"),
            open_valves: HashSet::new(),
            relased: 0,
        }
    }

    fn is_open(&self, v_id: &ValveId) -> bool {
        self.open_valves.contains(&v_id)
    }

    fn build_plan_with_step(&self, step: Step) -> Plan {
        let mut re = (*self).clone();
        re.relase_one_min();
        match step {
            GoTo(id) => re.pos = id.clone(),
            Open(id) => {
                re.open_valves.insert(id.clone());
            }
        };
        re
    }

    fn relase_one_min(&mut self) {
        for id in &self.open_valves {
            let rate = self.map.valves.get(&id).unwrap().rate;
            self.relased += rate;
        }
    }

    fn get_next_steps(&self) -> Vec<Step> {
        let curr_pos = self.pos.clone();
        let current_valve = self.map.valves.get(&curr_pos).unwrap();

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
}

fn find_best_plan(map: &Rc<Map>) -> Plan {
    let mut pot_plans = vec![Plan::new(&Rc::clone(&map))];

    for i in 0..30 {
        let mut next_plans = Vec::new();
        // all plans take one step
        for plan in &pot_plans {
            for step in plan.get_next_steps() {
                next_plans.push(plan.build_plan_with_step(step));
            }
        }
        println!("\nstep {},plans {}", i, next_plans.len());

        // only take the best paths at each position
        let mut next_best_plans = Vec::new();
        for v_id in map.valves.keys() {
            let best_plan_val = next_plans
                .iter()
                .filter(|p| p.pos == *v_id)
                .map(|p| p.relased)
                .max();

            if let Some(v) = best_plan_val {
                next_plans
                    .iter()
                    .filter(|p| p.pos == *v_id)
                    .filter(|p| p.relased == v)
                    .cloned()
                    .for_each(|p| next_best_plans.push(p));
            }
        }

        next_plans.iter().filter(|p| p.relased > 0).for_each(|p| {
            println!(
                "[{}/{}]\t{:?}",
                p.relased,
                p.pos,
                p.open_valves
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        });

        pot_plans = next_best_plans;
    }

    pot_plans.into_iter().max_by_key(|p| p.relased).unwrap()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let best = find_best_plan(&Rc::new(map));
    // 1488 to low
    best.relased
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
