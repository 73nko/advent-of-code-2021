use std::{collections::HashMap, convert::From};

struct Cave {
    data: HashMap<(usize, usize), u8>,
    width: usize,
    height: usize,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let data: HashMap<_, _> = input
            .lines()
            .enumerate()
            .flat_map(|(y, s)| s.bytes().enumerate().map(move |(x, v)| ((x, y), v - b'0')))
            .collect();

        let height = input.lines().count();
        let width = data
            .keys()
            .max_by_key(|x| x.0)
            .map(|x| x.0 + 1)
            .unwrap_or_default();

        Self {
            data,
            width,
            height,
        }
    }
}

impl Cave {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get(&(x, y)).copied()
    }

    fn neighbours(&self, x: usize, y: usize) -> [Option<u8>; 4] {
        let mut result = [None; 4];
        if y != 0 {
            result[0] = self.get(x, y - 1);
        }
        if y + 1 < self.height {
            result[1] = self.get(x, y + 1);
        }
        if x != 0 {
            result[2] = self.get(x - 1, y);
        }
        if x + 1 < self.width {
            result[3] = self.get(x + 1, y);
        }
        result
    }

    fn low_point(&self, x: usize, y: usize) -> Option<u8> {
        let current = self.get(x, y)?;
        let neighbours = self.neighbours(x, y);
        let low = neighbours.iter().all(|x| match x {
            None => true,
            Some(x) => current < *x,
        });

        if low {
            Some(current)
        } else {
            None
        }
    }
}
fn solve_part1(input: &Cave) -> usize {
    let mut points = vec![];
    for ((x, y), _) in input.data.iter() {
        if let Some(pt) = input.low_point(*x, *y) {
            points.push(pt);
        }
    }

    points.iter().map(|x| *x as usize + 1).sum()
}

fn prepare_input() -> Cave {
    Cave::from(include_str!("../input.txt").trim_end())
}

fn main() {
    let input = prepare_input();
    let solution1 = solve_part1(&input);

    // let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    // println!("PART 2: {}", solution2);
}
