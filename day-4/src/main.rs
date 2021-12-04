use std::fs;

#[derive(Debug)]
struct Board {
    layout: Vec<Vec<(bool, i32)>>,
}

impl Board {
    fn new(lines: Vec<&str>) -> Board {
        Board {
            layout: lines.iter().fold(Vec::new(), |mut result, line| {
                result.push(
                    line.split_whitespace()
                        .map(|x| (false, x.parse::<i32>().unwrap()))
                        .collect(),
                );
                result
            }),
        }
    }

    fn play(&mut self, num: i32) {
        self.layout.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| {
                if col.1 == num {
                    col.0 = true
                }
            })
        })
    }

    fn is_win(&self) -> bool {
        let mut rows = [true, true, true, true, true];
        let mut cols = [true, true, true, true, true];

        self.layout.iter().enumerate().for_each(|(i_row, row)| {
            row.iter().enumerate().for_each(|(i_col, col)| {
                rows[i_row] = rows[i_row] && col.0;
                cols[i_col] = cols[i_col] && col.0;
            })
        });

        rows.iter().any(|x| *x) || cols.iter().any(|x| *x)
    }

    fn unmarked_score(&self) -> i32 {
        self.layout
            .iter()
            .map(|row| row.iter().filter(|x| !x.0).map(|x| x.1).sum::<i32>())
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("day-4/input.txt").unwrap();

    let (winning_score, last_winning_score) = play_game(input);

    println!("First board to win score: {}", winning_score);
    println!("Last board to win score: {}", last_winning_score);
}

fn play_game(input: String) -> (i32, i32) {
    let mut lines = input.lines();

    let mut input_values = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap());

    let mut boards: Vec<Board> = Vec::new();

    while lines.next().is_some() {
        boards.push(Board::new((0..5).fold(Vec::new(), |mut result, _| {
            result.push(lines.next().unwrap());
            result
        })));
    }

    let mut winning_score: Option<i32> = None;
    let mut last_winning_score = 0;

    while let Some(input) = input_values.next() {
        boards
            .iter_mut()
            .filter(|board| !board.is_win())
            .for_each(|board| {
                board.play(input);
                if board.is_win() {
                    if winning_score.is_none() {
                        winning_score = Some(board.unmarked_score() * input);
                    }

                    last_winning_score = board.unmarked_score() * input;
                }
            })
    }

    let winning_score = winning_score.unwrap();

    (winning_score, last_winning_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n 22 13 17 11  0\n 8  2 23  4 24\n 21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n 19  8  7 25 23\n 20 11 10 24  4\n 14 21 16 12  6\n\n 14 21 17 24  4\n 10 16 15  9 19\n 18  8 23 26 20\n 22 11 13  6  5\n 2  0 12  3  7"
    }

    fn example_board() -> Board {
        Board::new(vec![
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
        ])
    }

    #[test]
    fn test_board_creation() {
        assert_eq!(
            Board::new(vec![
                "22 13 17 11  0",
                "8  2 23  4 24",
                "21  9 14 16  7",
                "6 10  3 18  5",
                "1 12 20 15 19"
            ])
            .layout,
            vec![
                vec![
                    (false, 22),
                    (false, 13),
                    (false, 17),
                    (false, 11),
                    (false, 0)
                ],
                vec![(false, 8), (false, 2), (false, 23), (false, 4), (false, 24)],
                vec![
                    (false, 21),
                    (false, 9),
                    (false, 14),
                    (false, 16),
                    (false, 7)
                ],
                vec![(false, 6), (false, 10), (false, 3), (false, 18), (false, 5)],
                vec![
                    (false, 1),
                    (false, 12),
                    (false, 20),
                    (false, 15),
                    (false, 19)
                ]
            ]
        );
    }

    #[test]
    fn test_play() {
        let mut board = example_board();
        board.play(21);
        assert_eq!(
            board.layout,
            vec![
                vec![
                    (false, 22),
                    (false, 13),
                    (false, 17),
                    (false, 11),
                    (false, 0)
                ],
                vec![(false, 8), (false, 2), (false, 23), (false, 4), (false, 24)],
                vec![(true, 21), (false, 9), (false, 14), (false, 16), (false, 7)],
                vec![(false, 6), (false, 10), (false, 3), (false, 18), (false, 5)],
                vec![
                    (false, 1),
                    (false, 12),
                    (false, 20),
                    (false, 15),
                    (false, 19)
                ]
            ]
        )
    }

    #[test]
    fn test_win() {
        assert_eq!(
            (Board {
                layout: vec![
                    vec![
                        (false, 22),
                        (false, 13),
                        (false, 17),
                        (false, 11),
                        (false, 0)
                    ],
                    vec![(false, 8), (false, 2), (false, 23), (false, 4), (false, 24)],
                    vec![(true, 21), (false, 9), (false, 14), (false, 16), (false, 7)],
                    vec![(false, 6), (false, 10), (false, 3), (false, 18), (false, 5)],
                    vec![
                        (false, 1),
                        (false, 12),
                        (false, 20),
                        (false, 15),
                        (false, 19)
                    ]
                ]
            })
            .is_win(),
            false
        );

        assert_eq!(
            (Board {
                layout: vec![
                    vec![
                        (false, 22),
                        (false, 13),
                        (false, 17),
                        (false, 11),
                        (false, 0)
                    ],
                    vec![(true, 8), (true, 2), (true, 23), (true, 4), (true, 24)],
                    vec![(true, 21), (false, 9), (false, 14), (false, 16), (false, 7)],
                    vec![(false, 6), (false, 10), (false, 3), (false, 18), (false, 5)],
                    vec![
                        (false, 1),
                        (false, 12),
                        (false, 20),
                        (false, 15),
                        (false, 19)
                    ]
                ]
            })
            .is_win(),
            true
        );

        assert_eq!(
            (Board {
                layout: vec![
                    vec![
                        (true, 22),
                        (false, 13),
                        (false, 17),
                        (false, 11),
                        (false, 0)
                    ],
                    vec![(true, 8), (false, 2), (true, 23), (true, 4), (true, 24)],
                    vec![(true, 21), (false, 9), (false, 14), (false, 16), (false, 7)],
                    vec![(true, 6), (false, 10), (false, 3), (false, 18), (false, 5)],
                    vec![
                        (true, 1),
                        (false, 12),
                        (false, 20),
                        (false, 15),
                        (false, 19)
                    ]
                ]
            })
            .is_win(),
            true
        );
    }

    #[test]
    fn test_game() {
        assert_eq!(play_game(String::from(example())), (4512, 1924))
    }
}
