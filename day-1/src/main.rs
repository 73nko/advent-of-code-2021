fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > prev { sum + 1 } else { sum }, n)
        })
        .0
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse().ok())
        .collect::<Option<Vec<_>>>()
        .and_then(|nums| {
            Some(
                nums.windows(3)
                    .map(|n| n.iter().sum())
                    .fold((0, i32::MAX), |(sum, prev), n| {
                        (if n > prev { sum + 1 } else { sum }, n)
                    })
                    .0,
            )
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    let solution1 = solve_part1(input);

    let solution2 = solve_part2(input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
