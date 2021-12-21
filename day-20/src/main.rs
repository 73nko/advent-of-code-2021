use itertools::Itertools;
use std::{collections::HashSet, fmt::Write};

const ADJACENT: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rect {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl Rect {
    #[inline]
    fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x && x <= self.width && y >= self.y && y <= self.height
    }
}

#[derive(Debug)]
struct Image {
    algorithm: Vec<bool>,
    input: HashSet<(isize, isize)>,
    bounds: Rect,
    default_pixel: bool,
}

impl Image {
    fn from_input(s: &str) -> Self {
        let (algorithm, s) = s.split_once("\n\n").unwrap();
        let algorithm: Vec<bool> = algorithm.chars().map(|c| c == '#').collect();
        let input: HashSet<(isize, isize)> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, v)| (v == '#').then(|| (x as isize, y as isize)))
            })
            .collect();

        let (x, width) = input.iter().map(|p| p.0).minmax().into_option().unwrap();
        let (y, height) = input.iter().map(|p| p.1).minmax().into_option().unwrap();

        Self {
            default_pixel: false,
            algorithm,
            input,
            bounds: Rect {
                x,
                y,
                width,
                height,
            },
        }
    }

    fn get(&self, x: isize, y: isize) -> usize {
        if !self.bounds.contains(x, y) {
            self.default_pixel as usize
        } else {
            self.input.contains(&(x, y)) as usize
        }
    }

    fn binary_at(&self, x: isize, y: isize) -> usize {
        ADJACENT
            .iter()
            .enumerate()
            .fold(0, |acc, (index, (dx, dy))| {
                acc | (self.get(x + dx, y + dy) << (8 - index))
            })
    }

    fn pixels(&self) -> usize {
        self.input.len()
    }

    fn step(&mut self) {
        let mut new_image = HashSet::with_capacity(
            self.input.len() + self.bounds.width as usize * 2 + self.bounds.height as usize * 2,
        );

        for x in self.bounds.x - 1..=self.bounds.width + 1 {
            for y in self.bounds.y - 1..=self.bounds.height + 1 {
                let index = self.binary_at(x, y);
                if self.algorithm[index] {
                    new_image.insert((x, y));
                }
            }
        }

        if self.default_pixel {
            self.default_pixel = self.algorithm.last().copied().unwrap_or(false);
        } else {
            self.default_pixel = self.algorithm.first().copied().unwrap_or(true);
        }

        self.input = new_image;
        self.bounds.x -= 1;
        self.bounds.y -= 1;
        self.bounds.width += 1;
        self.bounds.height += 1;
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default = if self.default_pixel { '#' } else { '.' };
        for y in self.bounds.y - 1..=self.bounds.height + 1 {
            for x in self.bounds.x - 1..=self.bounds.width + 1 {
                if self.input.contains(&(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char(default)?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn solve_part1(image: &mut Image) -> usize {
    for _ in 0..2 {
        image.step();
    }
    image.pixels()
}

fn solve_part2(image: &mut Image) -> usize {
    for _ in 0..50 {
        image.step();
    }
    image.pixels()
}

fn prepare_input() -> Image {
    Image::from_input(include_str!("../input.txt"))
}

fn main() {
    let mut input = prepare_input();
    let solution1 = solve_part1(&mut input);

    let mut input = prepare_input();
    let solution2 = solve_part2(&mut input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
