use std::fs;

type MaxMin = (String, String);

fn main() {
    let input = fs::read_to_string("day-3/input.txt").unwrap();

    let (gamma, epsilon) = get_gamma_and_epsilon(&input);
    let gamma = str_to_i32(gamma);
    let epsilon = str_to_i32(epsilon);

    println!("Power consumption = {}", gamma * epsilon);

    let oxygen = str_to_i32(most_common_filtered(&input));
    let co2 = str_to_i32(least_common_filtered(&input));

    println!("Life supply = {}", oxygen * co2);
}

fn get_gamma_and_epsilon(input: &str) -> MaxMin {
    let mut occurrences = Vec::new();
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            let x = match c {
                '0' => -1,
                '1' => 1,
                _ => unreachable!(),
            };

            match occurrences.get_mut(i) {
                None => occurrences.push(x),
                Some(elem) => *elem += x,
            }
        })
    });

    generate_bin_strings(occurrences)
}

fn get_max_min_at_position(inputs: &Vec<&str>, pos: usize) -> MaxMin {
    let total = inputs.iter().fold(0, |mut total, line| {
        let c = line.chars().skip(pos).take(1).last().unwrap();
        let x = match c {
            '0' => -1,
            '1' => 1,
            _ => unreachable!(),
        };

        total += x;
        total
    });

    generate_bin_strings(vec![total])
}

fn most_common_filtered(input: &str) -> String {
    let mut inputs: Vec<&str> = input.lines().collect();

    let mut counter = 0;

    while inputs.len() > 1 {
        let most_common = get_max_min_at_position(&inputs, counter).0;

        inputs = inputs
            .into_iter()
            .filter(|input| {
                input.chars().nth(counter).unwrap() == most_common.chars().nth(0).unwrap()
            })
            .collect();

        counter += 1;
    }

    String::from(inputs[0])
}

fn least_common_filtered(input: &str) -> String {
    let mut inputs: Vec<&str> = input.lines().collect();

    let mut counter = 0;

    while inputs.len() > 1 {
        let most_common = get_max_min_at_position(&inputs, counter).1;

        inputs = inputs
            .into_iter()
            .filter(|input| {
                input.chars().nth(counter).unwrap() == most_common.chars().nth(0).unwrap()
            })
            .collect();

        counter += 1;
    }

    String::from(inputs[0])
}

fn generate_bin_strings(occurrences: Vec<i32>) -> MaxMin {
    occurrences.iter().fold(
        (String::new(), String::new()),
        |(mut most_common, mut least_common), occurrence| {
            if *occurrence >= 0 {
                most_common.push('1');
                least_common.push('0');
            } else {
                most_common.push('0');
                least_common.push('1');
            }

            (most_common, least_common)
        },
    )
}

fn str_to_i32(bin_string: String) -> i32 {
    i32::from_str_radix(&bin_string[..], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
    }

    #[test]
    fn most_common_test() {
        assert_eq!(get_gamma_and_epsilon(example()).0, String::from("10110"));
    }

    #[test]
    fn least_common_test() {
        assert_eq!(get_gamma_and_epsilon(example()).1, String::from("01001"));
    }

    #[test]
    fn example_1() {
        let (gamma, epsilon) = get_gamma_and_epsilon(example());
        assert_eq!(str_to_i32(gamma) * str_to_i32(epsilon), 198);
    }

    #[test]
    fn most_common_at_position_test() {
        assert_eq!(
            get_max_min_at_position(&example().lines().collect(), 0).0,
            String::from("1")
        );
    }

    #[test]
    fn least_common_at_position_test() {
        assert_eq!(
            get_max_min_at_position(&example().lines().collect(), 0).1,
            String::from("0")
        );
    }

    #[test]
    fn most_common_filtered_test() {
        assert_eq!(most_common_filtered(example()), String::from("10111"));
    }

    #[test]
    fn least_common_filtered_test() {
        assert_eq!(least_common_filtered(example()), String::from("01010"));
    }
}
