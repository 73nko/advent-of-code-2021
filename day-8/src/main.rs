use std::collections::HashSet;

fn solve_part1(input: &Vec<(&str, &str)>) -> usize {
    input
        .into_iter()
        .flat_map(|(_, numbers)| numbers.split(" "))
        .filter(|&s| match s.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn get_sixes(result: &mut Vec<HashSet<u8>>, sixes: Vec<HashSet<u8>>) {
    for digit in sixes {
        if !digit.is_superset(&result[1]) {
            result[6] = digit;
        } else if !digit.is_superset(&result[4]) {
            result[0] = digit;
        } else {
            result[9] = digit;
        }
    }
}

fn get_fives(result: &mut Vec<HashSet<u8>>, fives: Vec<HashSet<u8>>) {
    for digit in fives {
        if result[6].is_superset(&digit) {
            result[5] = digit;
        } else if result[9].is_superset(&digit) {
            result[3] = digit;
        } else {
            result[2] = digit;
        }
    }
}

fn find_digits<'a>(input: Vec<&'a str>) -> Vec<HashSet<u8>> {
    let mut result = vec![HashSet::new(); 10];
    let mut sixes: Vec<HashSet<u8>> = Vec::new();
    let mut fives: Vec<HashSet<u8>> = Vec::new();

    for code in input.iter() {
        let digits: HashSet<u8> = code.bytes().collect();
        match code.len() {
            2 => result[1] = digits,
            3 => result[7] = digits,
            4 => result[4] = digits,
            7 => result[8] = digits,

            5 => fives.push(digits),
            6 => sixes.push(digits),
            _ => (),
        };
    }

    get_sixes(&mut result, sixes);
    get_fives(&mut result, fives);

    result
}

fn solve_part2(input: &Vec<(&str, &str)>) -> usize {
    input
        .into_iter()
        .map(|(clue, numbers)| {
            let decoded: Vec<HashSet<u8>> = find_digits(clue.split(" ").collect());
            numbers.split(" ").fold(0, |total, s| {
                let digits: HashSet<u8> = s.bytes().collect();
                let digit = decoded
                    .iter()
                    .position(|d| *d == digits)
                    .unwrap_or_default();
                total * 10 + digit
            })
        })
        .sum()
}

fn prepare_input<'a>() -> Vec<(&'a str, &'a str)> {
    include_str!("../input.txt")
        .trim_end()
        .lines()
        .filter_map(|f| f.split_once('|'))
        .collect()
}

fn main() {
    let input = prepare_input();

    let solution1 = solve_part1(&input);

    let solution2 = solve_part2(&input);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
