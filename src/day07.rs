use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
struct Filesystem<'a> {
    files: BTreeMap<Vec<&'a str>, File>,
}

impl<'a> Filesystem<'a> {
    fn new(input: &'a str) -> Self {
        let mut lines = input.lines().peekable();

        let mut fs = Filesystem {
            files: BTreeMap::new(),
        };
        let mut cwd_path: Vec<&str> = vec![];

        while lines.peek().is_some() {
            let cmd = Command::new(lines.next().unwrap());
            match cmd {
                Command::Cd(p) => match p {
                    "/" => cwd_path = vec![],
                    ".." => {
                        cwd_path.pop();
                    }
                    _ => cwd_path.push(&p.clone()),
                },
                Command::Ls => {
                    while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
                        let line = lines.next().unwrap();
                        if !line.starts_with("dir") {
                            let mut words = line.split_whitespace();
                            let size: usize = words.next().unwrap().parse().unwrap();
                            let mut fp = cwd_path.clone();
                            fp.push(words.next().unwrap());
                            fs.files.insert(fp, File::new(size));
                        }
                    }
                }
            }
        }

        fs
    }

    fn get_folder_sizes(&self) -> Vec<(String, usize)> {
        let mut checked_folders = BTreeSet::new();
        let mut re = Vec::new();
        for path in self.files.keys() {
            let mut p_str = Vec::new();
            // ignore file part of path
            for folder in path.iter().take(path.len() - 1) {
                p_str.push(folder.clone());
                if !checked_folders.contains(&p_str) {
                    checked_folders.insert(p_str.clone());

                    let sum_size = self
                        .files
                        .iter()
                        .filter(|(k, _)| k.len() >= p_str.len() && k[0..p_str.len()] == p_str)
                        .map(|(_, v)| v.size)
                        .sum();

                    re.push((p_str.join("/"), sum_size));
                }
            }
        }
        re
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

enum Command<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> Command<'a> {
    fn new(line: &str) -> Command {
        if !line.starts_with("$") {
            panic!("Line is not a command");
        }

        let mut words = line.split_whitespace().skip(1);
        match words.next().unwrap() {
            "cd" => Command::Cd(words.next().unwrap()),
            "ls" => Command::Ls,
            _ => panic!("unknown command"),
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let fs = Filesystem::new(input);

    fs.get_folder_sizes()
        .iter()
        .map(|(n, s)| s)
        .filter(|s| **s < 100000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let fs = Filesystem::new(input);
    let folder_sizes = fs.get_folder_sizes();
    let used_space: usize = fs.files.values().map(|f| f.size).sum();

    let min_delete = used_space - (70000000 - 30000000);
    folder_sizes
        .iter()
        .map(|(_, s)| *s)
        .filter(|s| *s >= min_delete)
        .min()
        .unwrap()
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
        assert_eq!(part2(EXAMPLE), 24933642)
    }

    #[test]
    fn test_read_fs() {
        let fs = Filesystem::new(EXAMPLE);

        let exp = Filesystem {
            files: {
                let mut f = BTreeMap::new();
                f.insert(vec!["a", "e", "i"], File::new(584));
                f.insert(vec!["a", "f"], File::new(29116));
                f.insert(vec!["a", "g"], File::new(2557));
                f.insert(vec!["a", "h.lst"], File::new(62596));
                f.insert(vec!["b.txt"], File::new(14848514));
                f.insert(vec!["c.dat"], File::new(8504156));
                f.insert(vec!["d", "j"], File::new(4060174));
                f.insert(vec!["d", "d.log"], File::new(8033020));
                f.insert(vec!["d", "d.ext"], File::new(5626152));
                f.insert(vec!["d", "k"], File::new(7214296));
                f
            },
        };

        assert_eq!(fs, exp);
    }
    #[test]
    fn test_fs_dirsize() {
        let fs = Filesystem::new(EXAMPLE);

        assert!(fs.get_folder_sizes().contains(&("a/e".to_string(), 584)));
        assert!(fs
            .get_folder_sizes()
            .contains(&("d".to_string(), 4060174 + 8033020 + 5626152 + 7214296)));
    }
}
