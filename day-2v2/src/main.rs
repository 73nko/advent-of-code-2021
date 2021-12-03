use std::convert::From;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(' ').collect();
        let cmd = tokens[0];
        let val = tokens[1].parse::<i64>().unwrap();

        match cmd {
            "forward" => Command::Forward(val),
            "up" => Command::Up(val),
            "down" => Command::Down(val),
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}

pub trait Submarine {
    fn execute(&mut self, cmd: &Command);
    fn run(&mut self, input: &[Command]) {
        for cmd in input {
            self.execute(cmd);
        }
    }
}

#[derive(Debug)]
pub struct SubmarineV1 {
    x: i64,
    y: i64,
}

impl Submarine for SubmarineV1 {
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(delta) => self.x += delta,
            Command::Up(delta) => self.y -= delta,
            Command::Down(delta) => self.y += delta,
        }
    }
}

#[derive(Debug)]
pub struct SubmarineV2 {
    x: i64,
    y: i64,
    aim: i64,
}

impl Submarine for SubmarineV2 {
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Up(delta) => self.aim -= delta,
            Command::Down(delta) => self.aim += delta,
            Command::Forward(delta) => {
                self.x += delta;
                self.y += self.aim * delta;
            }
        }
    }
}

pub fn solve_part1(input: &[Command]) -> i64 {
    let mut submarine = SubmarineV1 { x: 0, y: 0 };
    submarine.run(input);

    submarine.x * submarine.y
}

pub fn solve_part2(input: &[Command]) -> i64 {
    let mut submarine = SubmarineV2 { x: 0, y: 0, aim: 0 };
    submarine.run(input);

    submarine.x * submarine.y
}
fn prepare_input() -> Vec<Command> {
    include_str!("../input.txt")
        .lines()
        .map(Command::from)
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);
    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
