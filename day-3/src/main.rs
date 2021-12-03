fn solve_part1(input: &Vec<i32>) -> isize {
    let gamma: String = input.iter().map(|x| x.to_string()).collect();
    let epsilon: String = input
        .iter()
        .map(|x| if *x == 1 { "0" } else { "1" })
        .collect();
    let gamma_dec = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon, 2).unwrap();

    gamma_dec * epsilon_dec
}

fn prepare_input1() -> Vec<i32> {
    let input = include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|c| *c as i32 - 48)
                .collect::<Vec<i32>>()
        })
        .fold(vec![0; 12], |mut acc, bytes| {
            for (i, x) in bytes.iter().enumerate() {
                if *x == 1 {
                    acc[i] = acc[i] + 1;
                } else {
                    acc[i] = acc[i] - 1;
                }
            }
            acc
        });

    input
        .into_iter()
        .map(|x| if x > 0 { 1 } else { 0 })
        .collect()
}

fn prepare_input2() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(String::from)
        .collect()
}

fn find_correct_val(input: &Vec<String>, one: u8, zero: u8) -> Vec<String> {
    let mut index = 0;
    let mut result = input.clone();

    while result.len() >= 2 {
        let most_common_byte = result.iter().fold([0u32; 2], |mut common_byte, line| {
            let byte = line.as_bytes()[index];
            common_byte[(byte - zero) as usize] += 1;
            common_byte
        });

        let b = if most_common_byte[0] <= most_common_byte[1] {
            one
        } else {
            zero
        };
        result.retain(|f| f.as_bytes()[index] == b);
        index += 1;
    }

    result
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let oxygen = find_correct_val(input, b'1', b'0');
    let co2 = find_correct_val(input, b'1', b'0');

    let oxygen = i32::from_str_radix(oxygen[0].as_str(), 2).unwrap();
    let co2 = i32::from_str_radix(co2[0].as_str(), 2).unwrap();

    oxygen * co2
}

fn main() {
    let input1 = prepare_input1();
    let solution1 = solve_part1(&input1);

    let input2 = prepare_input2();
    let solution2 = solve_part2(&input2);

    println!("PART 1: {}", solution1);
    println!("PART 2: {}", solution2);
}
