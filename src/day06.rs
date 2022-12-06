use std::iter::Cycle;

#[derive(Debug)]
struct Buffer {
    content: [char; 14],
    indx: Cycle<CountTill>,
    size: usize,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            content: ['\0'; 14],
            indx: CountTill::new(size).cycle(),
            size,
        }
    }

    fn add(&mut self, c: char) {
        self.content[self.indx.next().unwrap()] = c;
    }

    fn has_dublicate(&self) -> bool {
        self.content.iter().any(|c| {
            self.content[..self.size]
                .iter()
                .filter(|d| **d != '\0')
                .filter(|d| *d == c)
                .count()
                > 1
        })
    }
}

#[derive(Clone, Debug)]
struct CountTill {
    max: usize,
    pos: usize,
}

impl CountTill {
    fn new(max: usize) -> Self {
        Self { max, pos: 0 }
    }
}

impl Iterator for CountTill {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.max {
            None
        } else {
            let cur = self.pos;
            self.pos += 1;
            Some(cur)
        }
    }
}

#[aoc(day6, part1, ringbuff)]
fn part1(input: &str) -> usize {
    find_idx(input, 4)
}

#[aoc(day6, part2, ringbuff)]
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

#[aoc(day6, part1, range)]
fn part1_range(input: &str) -> usize {
    find_idx_inter(input, 4)
}

#[aoc(day6, part2, range)]
fn part2_range(input: &str) -> usize {
    find_idx_inter(input, 14)
}

fn find_idx_inter(input: &str, scan_size: usize) -> usize {
    let mut pos = 0;
    while pos < input.len() {
        let scan = &input[pos..pos + scan_size];
        if scan
            .chars()
            .all(|c| scan.chars().filter(|d| *d == c).count() == 1)
        {
            return pos + scan_size;
        }
        pos += 1;
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
    fn part1_range_test() {
        assert_eq!(part1_range(EXAMPLE1), 7);
        assert_eq!(part1_range(EXAMPLE2), 5);
        assert_eq!(part1_range(EXAMPLE3), 6);
        assert_eq!(part1_range(EXAMPLE4), 10);
        assert_eq!(part1_range(EXAMPLE5), 11);
    }

    #[test]
    fn part2_range_test() {
        assert_eq!(part2_range(EXAMPLE1), 19);
        assert_eq!(part2_range(EXAMPLE2), 23);
        assert_eq!(part2_range(EXAMPLE3), 23);
        assert_eq!(part2_range(EXAMPLE4), 29);
        assert_eq!(part2_range(EXAMPLE5), 26);
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
