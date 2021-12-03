use std::fs;

fn main() {
    let input = fs::read_to_string("day-3/input.txt").unwrap();

    let gamma = str_to_i32(most_common(&input));
    let epsilon = str_to_i32(least_common(&input));

    println!("Power consumption = {}", gamma * epsilon);

    let oxygen = str_to_i32(most_common_filtered(&input));
    let co2 = str_to_i32(least_common_filtered(&input));

    println!("Life supply = {}", oxygen * co2);
}

fn most_common(input: &str) -> String {
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

    generate_bin_str(occurrences)
}

fn most_common_at_position(inputs: &Vec<&str>, pos: usize) -> String {
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

    generate_bin_str(vec![total])
}

fn most_common_filtered(input: &str) -> String {
    let mut inputs: Vec<&str> = input.lines().collect();

    let mut counter = 0;

    while inputs.len() > 1 {
        let most_common = most_common_at_position(&inputs, counter);

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

fn least_common_at_position(inputs: &Vec<&str>, pos: usize) -> String {
    invert(most_common_at_position(&inputs, pos))
}

fn least_common(input: &str) -> String {
    invert(most_common(input))
}

fn least_common_filtered(input: &str) -> String {
    let mut inputs: Vec<&str> = input.lines().collect();

    let mut counter = 0;

    while inputs.len() > 1 {
        let most_common = least_common_at_position(&inputs, counter);

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

fn invert(input: String) -> String {
    input.chars().fold(String::new(), |mut result, c| {
        match c {
            '0' => result.push('1'),
            '1' => result.push('0'),
            _ => unreachable!(),
        };
        result
    })
}

fn generate_bin_str(occurrences: Vec<i32>) -> String {
    occurrences
        .iter()
        .fold(String::new(), |mut result, occurrence| {
            if *occurrence >= 0 {
                result.push('1')
            } else {
                result.push('0')
            }

            result
        })
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
        assert_eq!(most_common(example()), String::from("10110"));
    }

    #[test]
    fn least_common_test() {
        assert_eq!(least_common(example()), String::from("01001"));
    }

    #[test]
    fn example_1() {
        assert_eq!(
            str_to_i32(most_common(example())) * str_to_i32(least_common(example())),
            198
        );
    }

    #[test]
    fn most_common_at_position_test() {
        assert_eq!(
            most_common_at_position(&example().lines().collect(), 0),
            String::from("1")
        );
    }

    #[test]
    fn least_common_at_position_test() {
        assert_eq!(
            least_common_at_position(&example().lines().collect(), 0),
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
