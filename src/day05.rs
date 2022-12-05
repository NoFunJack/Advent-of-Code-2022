use regex::Regex;

pub struct Store {
    stacks: Vec<Vec<char>>,
}

impl Store {
    fn new(input: &str) -> Store {
        let mut lines: Vec<&str> = input.lines().collect();
        // every stack takes 4 chars the last 3
        let num_stacks = (lines.last().unwrap().len() / 4) + 1;
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
        // discard counting line
        lines.pop();

        for line in lines.iter().rev() {
            for pos in 0..num_stacks {
                if let Some(id) = line.chars().nth(1 + 4 * pos) {
                    if !id.is_whitespace() {
                        stacks.get_mut(pos).unwrap().push(id);
                    }
                }
            }
        }

        Store { stacks }
    }

    fn apply(&mut self, instr: Instr) {
        for _ in 0..instr.amount {
            //println!("{:?}\n step: {:?}", self.stacks, instr);

            let id = self
                .stacks
                .get_mut(instr.from - 1)
                .expect("no Stack")
                .pop()
                .expect("stack empty");

            self.stacks
                .get_mut(instr.to - 1)
                .expect("no target stack")
                .push(id);
        }
    }

    fn read_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct Instr {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instr {
    fn new(input: &str) -> Instr {
        let move_regex: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = move_regex.captures(input).unwrap();

        Instr {
            amount: caps.get(1).map(|s| s.as_str().parse().unwrap()).unwrap(),
            from: caps.get(2).map(|s| s.as_str().parse().unwrap()).unwrap(),
            to: caps.get(3).map(|s| s.as_str().parse().unwrap()).unwrap(),
        }
    }
}

pub fn load_data(input: &str) -> (Store, Vec<Instr>) {
    let mut parts = input.split("\n\n");
    let store_str = parts.next().unwrap();
    let intr_str = parts.next().unwrap();

    let store = Store::new(store_str);
    let prog = intr_str.lines().map(|l| Instr::new(l)).collect();

    (store, prog)
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let (mut store, prog) = load_data(input);

    for instr in prog {
        store.apply(instr);
    }

    store.read_top()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), "CMZ")
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }

    #[test]
    fn test_load_data_store() {
        let (store, _) = load_data(EXAMPLE);

        assert_eq!(store.stacks.len(), 3);
        assert_eq!(
            store.stacks,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        )
    }

    #[test]
    fn test_load_data_prog() {
        let (_, prog) = load_data(EXAMPLE);

        assert_eq!(prog.len(), 4);
        assert_eq!(
            prog,
            vec![
                Instr {
                    amount: 1,
                    from: 2,
                    to: 1,
                },
                Instr {
                    amount: 3,
                    from: 1,
                    to: 3,
                },
                Instr {
                    amount: 2,
                    from: 2,
                    to: 1,
                },
                Instr {
                    amount: 1,
                    from: 1,
                    to: 2,
                }
            ]
        )
    }
}
