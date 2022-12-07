use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Dir {
    subdirs: BTreeMap<String, Rc<RefCell<Dir>>>,
    files: Vec<File>,
    partent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(input: &str) -> Rc<RefCell<Dir>> {
        let mut root = Dir {
            subdirs: BTreeMap::new(),
            files: Vec::new(),
            partent: None,
        };

        let root_rc = Rc::new(RefCell::new(root));
        {
            let mut cwd = Rc::clone(&root_rc);
            let mut stdIn = input.lines().peekable();
            while stdIn.peek().is_some() {
                let cmd = Command::new(stdIn.next().unwrap());
                match cmd {
                    Command::Cd(p) => match p.as_str() {
                        "/" => cwd = Rc::clone(&root_rc),
                        ".." => cwd = (*cwd).borrow_mut().partent.unwrap(),
                        _ => cwd = todo!(),
                    },
                    Command::Ls => todo!(),
                }
            }
        }

        root_rc
    }
}

#[derive(Debug, PartialEq)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}

enum Command {
    Cd(String),
    Ls,
}

impl Command {
    fn new(line: &str) -> Command {
        println!("building cmd from line  {}", line);
        if !line.starts_with("$") {
            panic!("Line is not a command");
        }

        let mut words = line.split_whitespace().skip(1);
        match words.next().unwrap() {
            "cd" => Command::Cd(words.next().unwrap().to_string()),
            "ls" => Command::Ls,
            _ => panic!("unknown command"),
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u32 {
    0
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 95437)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 70)
    }

    #[test]
    fn test_read_fs() {
        let fs = Dir::new(EXAMPLE);

        let ref_e = Rc::new(RefCell::new(Dir {
            subdirs: BTreeMap::new(),
            files: vec![File::new(584)],
            partent: None,
        }));

        let ref_a = Rc::new(RefCell::new(Dir {
            subdirs: {
                let mut d = BTreeMap::new();
                d.insert("e".to_string(), Rc::clone(&ref_e));
                d
            },
            files: vec![File::new(29116), File::new(2557), File::new(62596)],
            partent: None,
        }));

        (*ref_e).borrow_mut().partent = Some(Rc::clone(&ref_e));

        let ref_d = Rc::new(RefCell::new(Dir {
            subdirs: BTreeMap::new(),
            files: vec![
                File::new(4060174),
                File::new(8033020),
                File::new(5626152),
                File::new(7214296),
            ],
            partent: None,
        }));

        let ref_ex = Rc::new(RefCell::new(Dir {
            subdirs: {
                let mut subs = BTreeMap::new();
                subs.insert("a".to_string(), Rc::clone(&ref_a));
                subs.insert("d".to_string(), Rc::clone(&ref_d));
                subs
            },
            files: vec![File::new(14848514), File::new(8504156)],
            partent: None,
        }));

        (*ref_a).borrow_mut().partent = Some(Rc::clone(&ref_ex));
        (*ref_d).borrow_mut().partent = Some(Rc::clone(&ref_ex));

        assert_eq!(fs, ref_ex);
    }
}
