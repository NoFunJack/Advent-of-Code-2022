use std::iter::Cycle;

#[derive(Debug)]
struct Buffer {
    content: [char; 14],
    indx: Cycle<std::slice::Iter<'static, usize>>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        match size {
            4 => Self {
                content: ['\0'; 14],
                indx: [0, 1, 2, 3].iter().cycle(),
            },
            14 => Self {
                content: ['\0'; 14],
                indx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
                    .iter()
                    .cycle(),
            },
            _ => panic!("unknown size"),
        }
    }

    fn add(&mut self, c: char) {
        self.content[*self.indx.next().unwrap()] = c;
    }

    fn has_dublicate(&self) -> bool {
        self.content.iter().any(|c| {
            self.content
                .iter()
                .filter(|d| **d != '\0')
                .filter(|d| *d == c)
                .count()
                > 1
        })
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    find_idx(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    find_idx(input, 14)
}

fn find_idx(input: &str, scan_size: usize) -> usize {
    let mut buff = Buffer::new(scan_size);
    for (idx, ch) in input.char_indices() {
        buff.add(ch);
        if !buff.has_dublicate() && idx >= 3 {
            return idx + 1;
        }
    }
    panic!("No match found")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE1), 7);
        assert_eq!(part1(EXAMPLE2), 5);
        assert_eq!(part1(EXAMPLE3), 6);
        assert_eq!(part1(EXAMPLE4), 10);
        assert_eq!(part1(EXAMPLE5), 11);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE1), 19);
        assert_eq!(part2(EXAMPLE2), 23);
        assert_eq!(part2(EXAMPLE3), 23);
        assert_eq!(part2(EXAMPLE4), 29);
        assert_eq!(part2(EXAMPLE5), 26);
    }

    #[test]
    fn test_buffer_add() {
        let mut buf = Buffer::new(4);
        buf.add('x');
        buf.add('y');
        buf.add('z');
        buf.add('w');

        assert_eq!(buf.content[..4], ['x', 'y', 'z', 'w']);
        buf.add('o');
        assert_eq!(buf.content[..4], ['o', 'y', 'z', 'w']);
    }

    #[test]
    fn test_buffer_dublicate_chedk() {
        let mut buf = Buffer::new(4);
        assert_eq!(buf.has_dublicate(), false);
        buf.add('x');
        assert_eq!(buf.has_dublicate(), false);
        buf.add('y');
        assert_eq!(buf.has_dublicate(), false);
        buf.add('z');
        assert_eq!(buf.has_dublicate(), false);
        buf.add('z');
        assert_eq!(buf.has_dublicate(), true)
    }
}
