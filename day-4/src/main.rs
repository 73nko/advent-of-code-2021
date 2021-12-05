use std::convert::From;

#[derive(Debug, Clone)]
struct Cell {
    value: u32,
    marked: bool,
}

impl Cell {
    fn new(value: u32) -> Cell {
        Cell {
            value: value,
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Cell>,
}

impl Board {
    const LINE_SIZE: usize = 5;

    fn new_number(&mut self, value: u32) {
        if let Some(c) = self.numbers.iter_mut().find(|c| c.value == value) {
            c.marked = true;
        }
    }

    fn has_line(&self) -> bool {
        for row in self.numbers.chunks(Self::LINE_SIZE) {
            if row.iter().all(|cell| cell.marked) {
                return true;
            }
        }

        false
    }

    fn has_column(&self) -> bool {
        for column in 0..Self::LINE_SIZE {
            if self
                .numbers
                .iter()
                .skip(column)
                .step_by(Self::LINE_SIZE)
                .all(|f| f.marked)
            {
                return true;
            }
        }

        false
    }

    fn is_winner(&self) -> bool {
        self.has_line() || self.has_column()
    }

    fn sum_non_marked(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|c| !c.marked)
            .map(|c| c.value)
            .sum()
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let numbers: Vec<Cell> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .map(Cell::new)
            .collect();

        Board { numbers }
    }
}

fn solve_part1((values, boards): &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = boards.clone();
    for value in values {
        for board in &mut boards {
            board.new_number(*value);
            if board.is_winner() {
                return board.sum_non_marked() * value;
            }
        }
    }

    0
}

fn solve_part2((values, boards): &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = boards.clone();
    let n_boards = boards.len();
    let mut winners = Vec::new();
    for value in values {
        for (index, board) in boards.iter_mut().enumerate() {
            if winners.contains(&index) {
                continue;
            }
            board.new_number(*value);
            if board.is_winner() {
                winners.push(index);
                if winners.len() == n_boards {
                    return board.sum_non_marked() * value;
                }
            }
        }
    }

    0
}

fn prepare_input() -> (Vec<u32>, Vec<Board>) {
    let (first_line, boards) = include_str!("../input.txt").split_once('\n').unwrap();

    let values: Vec<u32> = first_line.split(',').map(|s| s.parse().unwrap()).collect();
    let boards: Vec<Board> = boards.trim_start().split("\n\n").map(Board::from).collect();

    (values, boards)
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);
    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
