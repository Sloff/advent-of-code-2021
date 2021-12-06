use std::fs;

fn main() {
    let input = fs::read_to_string("day-6/input.txt").unwrap();

    let initial_fish = parse_input(&input);

    println!(
        "After 80 days there would be {} fish",
        determine_number_of_fish(&initial_fish, 80)
    );

    println!(
        "After 256 days there would be {} fish",
        determine_number_of_fish(&initial_fish, 256)
    );
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn determine_number_of_fish(input: &Vec<usize>, days: i32) -> usize {
    let initial_fish = input
        .iter()
        .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], |mut result, fish| {
            result[*fish] += 1;
            result
        });

    (0..days)
        .fold(initial_fish, |mut result, _| {
            let zero_fish = result[0];
            (0..8).for_each(|i| {
                result[i] = result[i + 1];
            });
            result[6] += zero_fish;
            result[8] = zero_fish;
            result
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "3,4,3,1,2"
    }

    #[test]
    fn parse() {
        assert_eq!(parse_input(example()), vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_18() {
        assert_eq!(determine_number_of_fish(&parse_input(example()), 18), 26)
    }

    #[test]
    fn test_80() {
        assert_eq!(determine_number_of_fish(&parse_input(example()), 80), 5934)
    }
}
