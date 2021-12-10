use std::fs;

fn main() {
    let input = fs::read_to_string("day-10/input.txt").unwrap();

    let (total_syntax_score, total_completion_score) = get_totals(&input);

    println!("Total syntax score: {}", total_syntax_score);
    println!("Total completion score: {}", total_completion_score);
}

fn get_totals(input: &str) -> (i32, i64) {
    let (total_syntax_score, mut total_completion_scores): (i32, Vec<i64>) =
        input.trim().lines().fold(
            (0, Vec::new()),
            |(syntax_result, mut completion_results), line| {
                let mut characters: Vec<char> = Vec::new();
                let first_illegal_character: Option<char> =
                    line.trim().chars().fold(None, |result, c| match result {
                        Some(x) => Some(x),
                        None => match c {
                            '{' | '(' | '<' | '[' => {
                                characters.push(c);
                                None
                            }
                            '}' => {
                                let matching = characters.pop().unwrap();
                                if matching == '{' {
                                    None
                                } else {
                                    Some(c)
                                }
                            }
                            ')' => {
                                let matching = characters.pop().unwrap();
                                if matching == '(' {
                                    None
                                } else {
                                    Some(c)
                                }
                            }
                            '>' => {
                                let matching = characters.pop().unwrap();
                                if matching == '<' {
                                    None
                                } else {
                                    Some(c)
                                }
                            }
                            ']' => {
                                let matching = characters.pop().unwrap();
                                if matching == '[' {
                                    None
                                } else {
                                    Some(c)
                                }
                            }
                            _ => unreachable!(),
                        },
                    });

                (
                    syntax_result
                        + match first_illegal_character {
                            None => 0,
                            Some(x) => match x {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            },
                        },
                    match first_illegal_character {
                        Some(_) => completion_results,
                        None => {
                            completion_results.push(characters.iter().rev().fold(0, |res, c| {
                                (res * 5)
                                    + match c {
                                        '(' => 1,
                                        '[' => 2,
                                        '{' => 3,
                                        '<' => 4,
                                        _ => unreachable!(),
                                    }
                            }));
                            completion_results
                        }
                    },
                )
            },
        );

    total_completion_scores.sort();

    let total_completion_score = total_completion_scores
        .get(total_completion_scores.len() / 2)
        .unwrap();

    (total_syntax_score, *total_completion_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn test_results() {
        assert_eq!(get_totals(example()), (26397, 288957))
    }
}
