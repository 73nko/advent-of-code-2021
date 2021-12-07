fn solve_part1(input: &Vec<i32>) -> i32 {
    let mut crabs = input.clone();
    let middle = input.len() / 2;

    crabs.select_nth_unstable(middle);
    crabs.iter().map(|crap| (*crap - crabs[middle]).abs()).sum()
}

fn calc(target: i32, crabs: &Vec<i32>) -> usize {
    crabs
        .iter()
        .map(|crab| {
            let crab_mean = (*crab - target).abs();
            ((crab_mean * (crab_mean + 1)) / 2) as usize
        })
        .sum()
}

fn solve_part2(input: &Vec<i32>) -> usize {
    let crabs = input.clone();
    let average: f64 = crabs.iter().map(|c| *c as f64).sum::<f64>() / crabs.len() as f64;

    let with_ceil = calc(average.ceil() as i32, &crabs);
    let with_floor = calc(average.floor() as i32, &crabs);

    with_ceil.min(with_floor)
}

fn prepare_input() -> Vec<i32> {
    include_str!("../input.txt")
        .split(",")
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
