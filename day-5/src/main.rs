use std::{collections::HashMap, convert::From};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut coords = s.split(',');
        let x = coords.next().unwrap().parse::<i32>().unwrap();
        let y = coords.next().unwrap().parse::<i32>().unwrap();
        Point { x, y }
    }
}

pub enum Dir {
    Horizontal,
    Vertical,
    Diagonal,
    None,
}

#[derive(Debug, Clone)]
struct CoordsMap {
    coords: HashMap<(i32, i32), i32>,
}

impl CoordsMap {
    fn new() -> Self {
        Self {
            coords: HashMap::new(),
        }
    }

    pub fn direction(&self, min: Point, max: Point, calc_diagonal: bool) -> Dir {
        if min.x == max.x {
            Dir::Vertical
        } else if min.y == max.y {
            Dir::Horizontal
        } else if calc_diagonal {
            Dir::Diagonal
        } else {
            Dir::None
        }
    }

    pub fn add_point(&mut self, point: (i32, i32)) {
        self.coords
            .entry(point)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn add(&mut self, p1: Point, p2: Point, calc_diagonal: bool) {
        let (min, max) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };

        match self.direction(min, max, calc_diagonal) {
            Dir::Horizontal => {
                for x in min.x..=max.x {
                    self.add_point((x, min.y));
                }
            }
            Dir::Vertical => {
                for y in min.y..=max.y {
                    self.add_point((min.x, y));
                }
            }
            Dir::Diagonal => {
                let inc = if min.y >= max.y { -1 } else { 1 };
                let mut y = min.y;

                for x in min.x..=max.x {
                    self.add_point((x, y));
                    y += inc;
                }
            }
            Dir::None => (),
        }
    }

    fn count(&self) -> usize {
        self.coords.values().filter(|&v| *v >= 2).count()
    }
}

fn solve_part1(input: &Vec<(Point, Point)>) -> usize {
    let mut coords_map = CoordsMap::new();
    for (p1, p2) in input {
        coords_map.add(*p1, *p2, false)
    }
    coords_map.count()
}

fn solve_part2(input: &Vec<(Point, Point)>) -> usize {
    let mut coords_map = CoordsMap::new();
    for (p1, p2) in input {
        coords_map.add(*p1, *p2, true)
    }
    coords_map.count()
}

fn prepare_input() -> Vec<(Point, Point)> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once("->").unwrap();
            let (p1, p2) = (p1.trim(), p2.trim());
            (Point::from(p1), Point::from(p2))
        })
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);

    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
