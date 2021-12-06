fn solve_parts(input: &[usize; 9], days: i32) -> usize {
    let mut count = input.clone();

    for _day in 0..days {
        count.rotate_left(1);
        count[6] += count[8];
    }
    count.iter().sum()
}

fn prepare_input() -> [usize; 9] {
    let mut counts = [0 as usize; 9];

    include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|c: usize| {
            counts[c] += 1;
        });

    counts
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_parts(&input, 80);
    let solution2 = solve_parts(&input, 256);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
