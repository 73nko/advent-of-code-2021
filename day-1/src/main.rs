fn solve_part1(measures: Vec<i32>) -> i32 {
    measures
        .into_iter()
        .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > prev { sum + 1 } else { sum }, n)
        })
        .0
}

fn solve_part2(measures: Vec<i32>) -> i32 {
    measures
        .windows(3)
        .map(|n| n.iter().sum())
        .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > prev { sum + 1 } else { sum }, n)
        })
        .0
}

fn prepare_input() -> Vec<i32> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(input.clone());

    let solution2 = solve_part2(input.clone());

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
