fn solve_part1(input: Vec<i32>) -> i32 {
    0
}

fn solve_part2(input: Vec<i32>) -> i32 {
    0
}

fn prepare_input() -> Vec<i32> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);
    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
