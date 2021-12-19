mod grid;

use grid::{Direction, Grid};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeWithCost {
    x: usize,
    y: usize,
    cost: u16,
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn shortest_path(grid: &Grid<u8>) -> Option<u16> {
    let mut queue = BinaryHeap::new();
    queue.push(NodeWithCost {
        x: 0,
        y: 0,
        cost: 0,
    });

    let mut dist: Vec<_> = grid.items().map(|_| u16::MAX).collect();
    dist[0] = 0;

    while let Some(NodeWithCost { x, y, cost }) = queue.pop() {
        if (x, y) == (grid.width() - 1, grid.height() - 1) {
            return Some(cost);
        }

        for (dx, dy) in grid.neighbors(x, y, Direction::Cardinal) {
            let next = NodeWithCost {
                x: dx,
                y: dy,
                cost: cost + grid[(dx, dy)] as u16,
            };

            let idx = (dy * grid.width()) + dx;
            if next.cost < dist[idx] {
                queue.push(next);
                dist[idx] = next.cost;
            }
        }
    }
    None
}

fn solve_part1(input: &str) -> Option<u16> {
    let grid = Grid::single_ascii_number(input);
    shortest_path(&grid)
}

struct GridRowExpansion<'a> {
    s: &'a str,
    current: u8,
    offset: u8,
    iter: std::str::Bytes<'a>,
}

impl<'a> GridRowExpansion<'a> {
    fn new(s: &'a str, offset: u8) -> GridRowExpansion<'a> {
        Self {
            current: 0,
            offset,
            s,
            iter: s.bytes(),
        }
    }
}

impl<'a> Iterator for GridRowExpansion<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(((v - b'0' + self.offset + self.current - 1) % 9) + 1),
            None => {
                self.current += 1;
                if self.current >= 5 {
                    None
                } else {
                    self.iter = self.s.bytes();
                    self.next()
                }
            }
        }
    }
}

fn solve_part2(input: &str) -> Option<u16> {
    let data: Vec<_> = (0..5)
        .flat_map(|offset| {
            input
                .lines()
                .flat_map(move |s| GridRowExpansion::new(s, offset))
        })
        .collect();

    let height = input.lines().count() * 5;
    let width = input
        .find('\n')
        .map(|x| x * 5)
        .unwrap_or_else(|| data.len() / height);

    let grid = Grid::with_data(data, width, height);
    shortest_path(&grid)
}

fn main() {
    let input = include_str!("../input.txt").trim_end();

    let solution1 = solve_part1(input).unwrap();
    let solution2 = solve_part2(input).unwrap();
    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
