use std::fmt::{Debug, Display, Write};
use std::{collections::HashSet, convert::From};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.x, &self.y)
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        let (x, y) = s.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}

impl From<&str> for Fold {
    fn from(s: &str) -> Fold {
        let (_, rest) = s.split_at(11);
        let (coord, value) = rest.split_once('=').unwrap();
        match coord {
            "x" => Self::X(value.parse().unwrap()),
            "y" => Self::Y(value.parse().unwrap()),
            _ => panic!("invalid fold"),
        }
    }
}

impl Fold {
    fn apply<'a, T>(&self, points: T) -> HashSet<Point>
    where
        T: Iterator<Item = &'a Point>,
    {
        match self {
            Fold::X(x) => points
                .map(|f| {
                    if f.x < *x {
                        *f
                    } else {
                        Point {
                            x: 2 * x - f.x,
                            y: f.y,
                        }
                    }
                })
                .collect(),
            Fold::Y(y) => points
                .map(|f| {
                    if f.y < *y {
                        *f
                    } else {
                        Point {
                            x: f.x,
                            y: 2 * y - f.y,
                        }
                    }
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Paper {
    coords: HashSet<Point>,
    folds: Vec<Fold>,
}

impl Paper {
    fn new(s: &str) -> Self {
        let (coords, folds) = s.split_once("\n\n").unwrap();
        let coords: Vec<Point> = coords.lines().map(Point::from).collect();
        let coords = HashSet::from_iter(coords);

        Self {
            coords,
            folds: folds.lines().map(Fold::from).collect(),
        }
    }

    fn fold(&mut self, all: bool) {
        let folds = if all {
            self.folds.iter()
        } else {
            self.folds[0..1].iter()
        };
        for fold in folds {
            self.coords = fold.apply(self.coords.iter());
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ugly
        let max_x = self
            .coords
            .iter()
            .max_by_key(|f| f.x)
            .map(|p| p.x)
            .unwrap_or_default();
        let min_x = self
            .coords
            .iter()
            .min_by_key(|f| f.x)
            .map(|p| p.x)
            .unwrap_or_default();

        let max_y = self
            .coords
            .iter()
            .max_by_key(|f| f.y)
            .map(|p| p.y)
            .unwrap_or_default();
        let min_y = self
            .coords
            .iter()
            .min_by_key(|f| f.y)
            .map(|p| p.y)
            .unwrap_or_default();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.coords.contains(&Point { x, y }) {
                    f.write_char('X').unwrap();
                } else {
                    f.write_char(' ').unwrap();
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn solve_part1(input: &str) -> usize {
    let mut paper = Paper::new(input);
    paper.fold(false);
    paper.coords.len()
}

fn solve_part2(input: &str) -> Paper {
    let mut paper = Paper::new(input);
    paper.fold(true);

    paper
}

fn main() {
    let solution1 = solve_part1(include_str!("../input.txt"));
    let solution2 = solve_part2(include_str!("../input.txt"));

    println!("PART 1: {}", solution1);
    println!("{}", solution2);
}
