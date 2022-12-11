use Operator::*;

fn read(input: &str) -> Vec<Monkey> {
    let mut mk_list = Vec::new();
    let mk_block_list = input.split("\n\n");

    for block in mk_block_list {
        let mut lines = block.lines().skip(1);
        // read items
        mk_list.push(Monkey {
            items: {
                let num_list_str = lines.next().unwrap().split(":").skip(1).next().unwrap();
                num_list_str
                    .split(",")
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect()
            },
            op: {
                let mut op_str = lines
                    .next()
                    .unwrap()
                    .splitn(2, "old")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split_whitespace();

                Op::new(op_str.next().unwrap(), op_str.next().unwrap())
            },
            test_div: lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            true_idx: lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            false_idx: lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            inspections_done: 0,
        })
    }

    mk_list
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<i32>,
    op: Op,
    test_div: i32,
    true_idx: usize,
    false_idx: usize,
    inspections_done: usize,
}

impl Monkey {
    fn throw(&mut self) -> Vec<FlyingItem> {
        let mut flying = Vec::new();
        for item in &self.items {
            let worry = self.op.apply(item) / 3;
            flying.push(FlyingItem {
                target: {
                    if worry % self.test_div == 0 {
                        self.true_idx
                    } else {
                        self.false_idx
                    }
                },
                worry,
            });
            self.inspections_done += 1;
        }

        self.items = Vec::new();

        flying
    }
}

#[derive(Debug, PartialEq)]
struct Op {
    op: Operator,
    number: Option<i32>,
}

impl Op {
    fn apply(&self, x: &i32) -> i32 {
        match self.op {
            Mult => x * self.number.unwrap(),
            Plus => x + self.number.unwrap(),
            Square => x * x,
        }
    }
}

struct FlyingItem {
    target: usize,
    worry: i32,
}

impl Op {
    fn new(op_str: &str, number: &str) -> Self {
        if number == "old" {
            Self {
                op: Square,
                number: None,
            }
        } else {
            match op_str {
                "*" => Self {
                    op: Mult,
                    number: Some(number.parse().unwrap()),
                },
                "+" => Self {
                    op: Plus,
                    number: Some(number.parse().unwrap()),
                },
                _ => panic!("invald op"),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Mult,
    Plus,
    Square,
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut mk_list = read(input);

    for _ in 0..20 {
        do_round(&mut mk_list);

        println!(
            "held items: {:#?}",
            mk_list
                .iter()
                .map(|m| m.items.clone())
                .collect::<Vec<Vec<i32>>>()
        );
    }

    monkey_fun(&mk_list)
}

fn do_round(mk_list: &mut [Monkey]) {
    for i in 0..mk_list.len() {
        let thrown_items: Vec<FlyingItem> = mk_list[i].throw();
        for item in thrown_items {
            mk_list.get_mut(item.target).unwrap().items.push(item.worry);
        }
    }
}

fn monkey_fun(mk_list: &[Monkey]) -> usize {
    let mut mkfun: Vec<usize> = mk_list.iter().map(|m| m.inspections_done).collect();
    mkfun.sort_by(|a, b| b.cmp(a));
    let mut iter = mkfun.iter();
    iter.next().unwrap() * iter.next().unwrap()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_read() {
        let ml = read(EXAMPLE);
        let mut ml = ml.iter();

        assert_eq!(
            ml.next(),
            Some(&Monkey {
                items: vec![79, 98],
                op: Op {
                    op: Mult,
                    number: Some(19),
                },
                test_div: 23,
                true_idx: 2,
                false_idx: 3,
                inspections_done: 0,
            })
        );

        assert_eq!(
            ml.next(),
            Some(&Monkey {
                items: vec![54, 65, 75, 74],
                op: Op {
                    op: Plus,
                    number: Some(6),
                },
                test_div: 19,
                true_idx: 2,
                false_idx: 0,
                inspections_done: 0,
            })
        );

        assert_eq!(
            ml.next(),
            Some(&Monkey {
                items: vec![79, 60, 97],
                op: Op {
                    op: Square,
                    number: None,
                },
                test_div: 13,
                true_idx: 1,
                false_idx: 3,
                inspections_done: 0,
            })
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 10605)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }
}
