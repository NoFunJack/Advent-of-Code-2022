use std::collections::{HashMap, HashSet};

use Tile::*;

use crate::day09::Point;

#[aoc_generator(day12)]
fn read(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Height(usize),
    Start,
    End,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            'S' => Start,
            'E' => End,
            _ => Height((c as u32 - 48 - 49).try_into().unwrap()),
        }
    }

    fn height(&self) -> usize {
        match self {
            Height(h) => *h,
            Start => 0,
            End => 25,
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Self { tiles }
    }
    fn find_path(&self) -> Vec<Point> {
        let mut min_steps = HashMap::new();
        let mut q = HashSet::new();
        q.insert(vec![self.find_start()]);

        while !q.is_empty() {
            let path_ref = q
                .iter()
                .min_by_key(|p| p.len() + self.get_tile(p.last().unwrap()).unwrap().height())
                .cloned()
                .unwrap();
            let path = q.take(&path_ref).unwrap();
            let node = path.last().unwrap().clone();
            if self.get_tile(&node) == Some(End) {
                println!("found path {:#?}", path);
                return path;
            } else {
                for next_pos in self.adj(&node) {
                    let new_cost = path.len() + 1 + self.get_tile(&next_pos).unwrap().height();
                    if let Some(min_cost) = min_steps.get(&next_pos) {
                        if *min_cost > new_cost {
                            let mut new_path = path.clone();
                            new_path.push(next_pos.clone());
                            q.insert(new_path);
                            min_steps.insert(next_pos.clone(), new_cost);
                        }
                    } else {
                        let mut new_path = path.clone();
                        new_path.push(next_pos.clone());
                        q.insert(new_path);
                        min_steps.insert(next_pos.clone(), new_cost);
                    }
                }
            }
        }

        panic!("no path found")
    }

    fn find_start(&self) -> Point {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, t) in row.iter().enumerate() {
                if let Start = t {
                    return Point(x.try_into().unwrap(), y.try_into().unwrap());
                }
            }
        }
        panic!("No start found")
    }

    fn adj(&self, pos: &Point) -> Vec<Point> {
        let curr_height = self.get_tile(pos).expect("start point not found").height();

        vec![Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1)]
            .into_iter()
            .map(|m| pos.clone() + m)
            .filter(|p| {
                if let Some(t) = self.get_tile(p) {
                    t.height() <= curr_height + 1
                } else {
                    false
                }
            })
            .collect()
    }

    fn get_tile(&self, pos: &Point) -> Option<Tile> {
        let Point(x, y) = pos.clone();

        if x < 0 || y < 0 {
            return None;
        }

        if let Some(row) = self.tiles.iter().nth(x.try_into().unwrap()) {
            if let Some(tile) = row.iter().nth(y.try_into().unwrap()) {
                return Some(tile.clone());
            }
        }
        None
    }
}

#[aoc(day12, part1)]
fn part1(input: &[Vec<Tile>]) -> usize {
    Map::new(input.to_vec()).find_path().len() - 1
}

#[aoc(day12, part2)]
fn part2(input: &[Vec<Tile>]) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_read_input() {
        let map = read("Sab\nyzE");

        assert_eq!(
            map,
            vec![
                vec![Start, Height(0), Height(1)],
                vec![Height(24), Height(25), End],
            ]
        )
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&read(EXAMPLE)[..]), 31)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&read(EXAMPLE)[..]), 70)
    }
}
