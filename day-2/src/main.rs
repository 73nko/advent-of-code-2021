fn solve_part1(input: Vec<(&str, usize)>) -> usize {
    let (depth, horizontal) = input
        .into_iter()
        .fold((0, 0), |(depth, horizontal), (dir, val)| match dir {
            "forward" => (depth, horizontal + val),
            "down" => (depth + val, horizontal),
            "up" => (depth - val, horizontal),
            _ => (depth, horizontal),
        });
    depth * horizontal
}

fn solve_part2(input: Vec<(&str, usize)>) -> usize {
    let (depth, horizontal, _) = input.into_iter().fold(
        (0, 0, 0),
        |(depth, horizontal, aim), (dir, val)| match dir {
            "forward" => (depth + (aim * val), horizontal + val, aim),
            "down" => (depth, horizontal, aim + val),
            "up" => (depth, horizontal, aim - val),
            _ => (depth, horizontal, aim),
        },
    );
    depth * horizontal
}

fn prepare_input<'a>() -> Vec<(&'a str, usize)> {
    include_str!("../input.txt")
        .lines()
        .map(|line| -> (&str, usize) {
            let (order, value) = line.split_once(' ').unwrap();
            (order, value.parse::<usize>().unwrap())
        })
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(input.clone());
    let solution2 = solve_part2(input.clone());

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
