use std::iter::Cycle;

#[derive(Debug)]
struct Buffer {
    content: [char; 4],
    indx: Cycle<std::array::IntoIter<usize, 4_usize>>,
}

impl Buffer {
    fn new() -> Self {
        Self {
            content: ['\0'; 4],
            indx: [0, 1, 2, 3].into_iter().cycle(),
        }
    }

    fn add(&mut self, c: char) {
        self.content[self.indx.next().unwrap()] = c;
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
    let mut buff = Buffer::new();
    for (idx, ch) in input.char_indices() {
        buff.add(ch);
        if !buff.has_dublicate() && idx >= 3 {
            println!("{} -> {:?}", idx, buff);
            return idx + 1;
        }
    }
    panic!("No match found")
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    0
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
        assert_eq!(part2(EXAMPLE1), 70)
    }

    #[test]
    fn test_buffer_add() {
        let mut buf = Buffer::new();
        buf.add('x');
        buf.add('y');
        buf.add('z');
        buf.add('w');

        assert_eq!(buf.content, ['x', 'y', 'z', 'w']);
        buf.add('o');
        assert_eq!(buf.content, ['o', 'y', 'z', 'w']);
    }

    #[test]
    fn test_buffer_dublicate_chedk() {
        let mut buf = Buffer::new();
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
