fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut vert = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        vert.push(row);
    }

    vert
}

fn check_visible(map: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let height = map.len();
    let width = map.first().unwrap().len();
    let mut vis = vec![vec![false; height]; width];

    // lr
    for row in 0..height {
        println!("\n# row {:?}", row);
        set_by_sightline(&map, &mut vis, Iter2d::new((row, 0), (0, 1), (0, width)));
        set_by_sightline(
            &map,
            &mut vis,
            Iter2d::new((row, width - 1), (0, -1), (0, width)),
        );
    }
    // ud
    for column in 0..width {
        println!("\n# column {:?}", column);
        set_by_sightline(
            &map,
            &mut vis,
            Iter2d::new((0, column), (1, 0), (0, height)),
        );
        set_by_sightline(
            &map,
            &mut vis,
            Iter2d::new((height - 1, column), (-1, 0), (0, height)),
        );
    }

    vis
}

fn set_by_sightline(map: &Vec<Vec<u32>>, vis: &mut Vec<Vec<bool>>, iter: Iter2d) {
    let mut max = None;
    for (i, j) in iter {
        let h = get_height(map, i, j);
        if max.is_none() || h > max.unwrap() {
            set_vis(vis, i, j);
            max = Some(h);
            if max == Some(9) {
                break;
            }
        }
    }
}

fn get_height(map: &Vec<Vec<u32>>, h: usize, r: usize) -> u32 {
    *map.get(h).unwrap().get(r).unwrap()
}
fn set_vis(vis: &mut Vec<Vec<bool>>, h: usize, r: usize) {
    println!(">>setting {}/{}", h, r);
    *vis.get_mut(h).unwrap().get_mut(r).unwrap() = true;
}

struct Iter2d {
    pos: (usize, usize),
    dir: (i32, i32),
    range: (usize, usize),
}

impl Iter2d {
    fn new(pos: (usize, usize), dir: (i32, i32), range: (usize, usize)) -> Self {
        Self { pos, dir, range }
    }
}

impl Iterator for Iter2d {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.pos;
        match add(self.pos.0, self.dir.0) {
            Some(v) => self.pos.0 = v,
            None => return None,
        }
        match add(self.pos.1, self.dir.1) {
            Some(v) => self.pos.1 = v,
            None => return None,
        }

        if std::cmp::min(self.pos.0, self.pos.1) < self.range.0
            || std::cmp::max(self.pos.0, self.pos.1) > self.range.1
        {
            None
        } else {
            Some(curr)
        }
    }
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let vis_map = check_visible(&map);

    //print_debug(&map, &vis_map);

    vis_map
        .iter()
        .map(|line| line.iter().filter(|b| **b).count())
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    0
}

fn print_debug(map: &Vec<Vec<u32>>, vis: &Vec<Vec<bool>>) {
    for i in 0..map.first().unwrap().len() {
        for j in 0..map.len() {
            if *vis.get(i).unwrap().get(j).unwrap() {
                print!("{}", get_height(&map, i, j));
            } else {
                print!("\u{001B}[41m{}\u{001B}[0m", get_height(&map, i, j));
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 21)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 157)
    }

    #[test]
    fn test_parser() {
        let out = parse(EXAMPLE);

        assert_eq!(
            out,
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ]
        )
    }
    #[test]
    fn test_vis() {
        let out = check_visible(&parse(EXAMPLE));

        assert_eq!(
            out,
            vec![
                vec![true, true, true, true, true],
                vec![true, true, true, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true]
            ]
        )
    }
}
