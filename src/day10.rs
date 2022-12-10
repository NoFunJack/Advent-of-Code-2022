use Cmd::*;

#[aoc_generator(day10)]
fn read(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| {
            if l == "noop" {
                return Noop;
            } else if l.starts_with("addx") {
                return Addx(l.split_whitespace().nth(1).unwrap().parse().unwrap());
            } else {
                panic!("cannot read line");
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Cmd {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    clock: i32,
    signal_strength: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            clock: 0,
            signal_strength: 0,
        }
    }

    fn do_cmd(&mut self, cmd: &Cmd) {
        //println!("{:#?}", self);
        //        println!("Cmd: {:?}", cmd);
        match cmd {
            Noop => self.tick(1),
            Addx(x) => {
                self.tick(2);
                self.x += x;
            }
        }
    }

    fn tick(&mut self, t: usize) {
        for _ in 0..t {
            self.clock += 1;
            if self.is_step() {
                let a = self.x * self.clock;
                //              println!("Adding  {} in step {}", a, self.clock);
                self.signal_strength += a;
            }
        }
    }
    fn is_step(&self) -> bool {
        (self.clock + 20) % 40 == 0
    }
}

#[aoc(day10, part1)]
fn part1(input: &[Cmd]) -> i32 {
    let mut cpu = Cpu::new();
    for c in input {
        cpu.do_cmd(c);
    }
    cpu.signal_strength
}

#[aoc(day10, part2)]
fn part2(input: &[Cmd]) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reader() {
        let l = read("addx 20\nnoop\naddx -11");

        assert_eq!(l, vec![Addx(20), Noop, Addx(-11)]);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&read(EXAMPLE)[..]), 13140)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&read(EXAMPLE)[..]), 70)
    }

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
