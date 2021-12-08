use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day-8/input.txt").unwrap();

    println!(
        "Amount of easy numbers: {}",
        get_amount_of_easy_numbers(&input)
    );

    println!("Sum of numbers: {}", get_sum(&input));
}

fn get_amount_of_easy_numbers(input: &str) -> usize {
    input.lines().fold(0, |result, line| {
        result
            + line
                .split(" | ")
                .last()
                .unwrap()
                .split(" ")
                .filter(|x| match x.len() {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .count()
    })
}

fn get_sum(input: &str) -> i32 {
    input.lines().fold(0, |total_result, line| {
        let mut line_iter = line.split(" | ");

        let unique = line_iter.next().unwrap();
        let output = line_iter.next().unwrap();

        let letter_map: HashMap<char, Vec<usize>> =
            unique
                .trim()
                .split(" ")
                .fold(HashMap::new(), |result, letters| {
                    letters.chars().fold(result, |mut r, letter| {
                        let entry = r.entry(letter).or_insert(Vec::new());
                        entry.push(letters.len());
                        r
                    })
                });

        let output: i32 = output
            .trim()
            .split(" ")
            .fold(String::new(), |mut result, letters| {
                let (letters_in_1, letters_in_4) =
                    letters
                        .chars()
                        .fold((0, 0), |(mut in_1, mut in_4), letter| {
                            if letter_map.get(&letter).unwrap().contains(&2) {
                                in_1 += 1;
                            }
                            if letter_map.get(&letter).unwrap().contains(&4) {
                                in_4 += 1;
                            }
                            (in_1, in_4)
                        });

                match letters.len() {
                    2 => result.push('1'),
                    3 => result.push('7'),
                    4 => result.push('4'),
                    5 => {
                        match (letters_in_1, letters_in_4) {
                            (2, _) => result.push('3'),
                            (_, 2) => result.push('2'),
                            (_, 3) => result.push('5'),
                            _ => unreachable!(),
                        };
                    }
                    6 => {
                        match (letters_in_1, letters_in_4) {
                            (1, _) => result.push('6'),
                            (_, 4) => result.push('9'),
                            _ => result.push('0'),
                        };
                    }
                    7 => result.push('8'),
                    _ => unreachable!(),
                }

                result
            })
            .parse()
            .unwrap();

        total_result + output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn easy_numbers() {
        assert_eq!(get_amount_of_easy_numbers(example()), 26)
    }

    #[test]
    fn sum_numbers() {
        assert_eq!(get_sum(example()), 61229)
    }
}
