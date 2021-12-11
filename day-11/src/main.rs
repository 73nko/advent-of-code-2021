use std::{
    collections::{HashMap, HashSet},
    convert::From,
};

const GRID_SIZE: isize = 10;

#[derive(Debug)]
struct Grid {
    data: HashMap<(isize, isize), u8>,
    flashes: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data: HashMap<(isize, isize), u8> = input
            .lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.bytes()
                    .enumerate()
                    .map(move |(x, v)| ((x as isize, y as isize), v - b'0'))
            })
            .collect();
        Self { data, flashes: 0 }
    }
}

impl Grid {
    fn adjacent_indexes(&self, x: isize, y: isize) -> [(isize, isize); 8] {
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }

    fn run_flash(&mut self, flashed: &mut HashSet<(isize, isize)>, x: isize, y: isize) {
        if let Some(value) = self.data.get_mut(&(x, y)) {
            if flashed.contains(&(x, y)) {
                return;
            }

            *value += 1;
            if *value > 9 {
                *value = 0;
                flashed.insert((x, y));
                for (x, y) in self.adjacent_indexes(x, y) {
                    self.run_flash(flashed, x, y)
                }
            }
        }
    }

    fn step(&mut self) -> bool {
        let mut flashed = HashSet::new();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                self.run_flash(&mut flashed, x, y);
            }
        }
        self.flashes += flashed.len();

        flashed.len() == (GRID_SIZE * GRID_SIZE) as usize
    }
}

fn solve_part1(mut grid: Grid, steps: i32) -> usize {
    for _ in 0..steps {
        grid.step();
    }
    grid.flashes
}

fn solve_part2(mut grid: Grid) -> i32 {
    let mut steps = 1;
    loop {
        if grid.step() {
            return steps;
        }
        steps += 1;
    }
}

fn prepare_input() -> Grid {
    Grid::from(include_str!("../input.txt"))
}

fn main() {
    let grid = prepare_input();

    let solution1 = solve_part1(grid, 100);

    let grid = prepare_input();
    let solution2 = solve_part2(grid);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
