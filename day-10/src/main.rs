fn score_1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_2(c: &char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn braces_match(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('<', '>') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}

fn illegal_score(l: &str) -> usize {
    let mut stack = vec![];
    for ch in l.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(top) = stack.pop() {
                    if !braces_match(top, ch) {
                        return score_1(ch);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    0
}

fn solve_part1(input: &str) -> usize {
    input.lines().map(illegal_score).sum()
}

fn sum_incomplete_scores(s: Vec<char>) -> usize {
    s.iter().rev().fold(0, |acc, ch| 5 * acc + score_2(ch))
}

fn fix_score(line: &str) -> Option<Vec<char>> {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(top) = stack.pop() {
                    if !braces_match(top, ch) {
                        return None;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    Some(stack)
}
fn solve_part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(fix_score)
        .map(sum_incomplete_scores)
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

fn prepare_input() -> &'static str {
    include_str!("../input.txt").trim_end()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);

    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
