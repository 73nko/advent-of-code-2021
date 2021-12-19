mod counter;

use counter::Counter;
use std::{collections::HashMap, fmt::Write};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Element(char, char);

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)?;
        f.write_char(self.1)
    }
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        Self(chars.next().unwrap(), chars.next().unwrap())
    }
}

impl From<&[u8]> for Element {
    fn from(x: &[u8]) -> Self {
        Self(x[0] as char, x[1] as char)
    }
}

struct Polymer {
    initial: String,
    counter: Counter<Element>,
    rules: HashMap<Element, char>,
}

impl Polymer {
    fn from_input(s: &str) -> Self {
        let (start, rules) = s.split_once("\n\n").unwrap();
        Self {
            initial: start.to_owned(),
            counter: start.as_bytes().windows(2).map(Element::from).collect(),
            rules: rules
                .lines()
                .filter_map(|s| s.split_once(" -> "))
                .map(|(from, to)| (Element::from(from), to.chars().next().unwrap()))
                .collect(),
        }
    }

    fn step(&mut self) {
        self.counter = self
            .counter
            .iter()
            .flat_map(|(el, count)| {
                let middle = self.rules[el];
                [
                    (Element(el.0, middle), *count),
                    (Element(middle, el.1), *count),
                ]
            })
            .collect();
    }

    fn apply(&mut self, count: usize) -> Counter<char> {
        for _ in 0..count {
            self.step();
        }

        let mut result: Counter<char> = self
            .counter
            .iter()
            .map(|(el, count)| (el.0, *count))
            .collect();
        let last = self.initial.as_bytes().last().copied().unwrap() as char;
        *result.entry(last) += 1;
        result
    }
}

fn solve(input: &str, count: usize) -> i64 {
    let mut polymer = Polymer::from_input(input);
    let counts = polymer.apply(count);
    let top = counts.top().unwrap();
    let bottom = counts.bottom().unwrap();
    top.1 - bottom.1
}

fn part1(input: &str) -> i64 {
    solve(input, 10)
}

fn part2(input: &str) -> i64 {
    solve(input, 40)
}

fn main() {
    let input = include_str!("../input.txt").trim_end();
    println!("PART 1: {:?}", part1(input));
    println!("PART 2: {:?}", part2(input));
}
