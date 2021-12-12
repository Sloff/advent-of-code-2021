use std::fs;

type Grid = Vec<Vec<(u32, bool)>>;

struct GridIter {
    grid: Grid,
}

impl GridIter {
    fn new(input: &str) -> GridIter {
        GridIter {
            grid: input.trim().lines().fold(Vec::new(), |mut result, line| {
                result.push(
                    line.trim()
                        .chars()
                        .map(|x| (x.to_digit(10).unwrap(), false))
                        .collect(),
                );

                result
            }),
        }
    }
}

impl Iterator for GridIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut flashes = 0;
        let mut first = true;
        let mut flash_happened = false;

        while first || flash_happened {
            flash_happened = false;
            (0..10).for_each(|row| {
                (0..10).for_each(|col| {
                    let elem = self.grid.get_mut(row).unwrap().get_mut(col).unwrap();
                    if first {
                        (*elem).0 += 1;
                    }

                    if elem.0 > 9 && !elem.1 {
                        flash_happened = true;
                        flashes += 1;
                        elem.1 = true;

                        let adjacent = vec![
                            if row == 0 || col == 0 {
                                None
                            } else {
                                Some((row - 1, col - 1))
                            },
                            if col == 0 { None } else { Some((row, col - 1)) },
                            if col == 0 {
                                None
                            } else {
                                Some((row + 1, col - 1))
                            },
                            if row == 0 { None } else { Some((row - 1, col)) },
                            Some((row + 1, col)),
                            if row == 0 {
                                None
                            } else {
                                Some((row - 1, col + 1))
                            },
                            Some((row, col + 1)),
                            Some((row + 1, col + 1)),
                        ];

                        adjacent
                            .into_iter()
                            .filter(|x| x.is_some())
                            .map(|x| x.unwrap())
                            .for_each(|(r, c)| {
                                let e = self.grid.get_mut(r).and_then(|x| x.get_mut(c));

                                if let Some(e) = e {
                                    e.0 += 1;
                                }
                            })
                    }
                })
            });
            first = false;
        }

        (0..10).for_each(|row| {
            (0..10).for_each(|col| {
                let elem = self.grid.get_mut(row).unwrap().get_mut(col).unwrap();
                if elem.1 {
                    elem.1 = false;
                    elem.0 = 0;
                }
            })
        });

        Some(flashes)
    }
}

fn main() {
    let input = fs::read_to_string("day-11/input.txt").unwrap();

    println!(
        "By step 100 the amount of flashes would have been: {}",
        get_total_flashed_by_100(&input)
    );

    println!(
        "All would flash by step: {}",
        get_step_where_all_flashed(&input)
    );
}

fn get_total_flashed_by_100(input: &str) -> i32 {
    let iter = GridIter::new(input);
    iter.take(100).sum::<i32>()
}

fn get_step_where_all_flashed(input: &str) -> usize {
    let iter = GridIter::new(input);
    iter.enumerate()
        .skip_while(|(_, x)| *x != 100)
        .next()
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526"
    }

    #[test]
    fn test_flashed_at_100() {
        assert_eq!(get_total_flashed_by_100(example()), 1656);
    }

    #[test]
    fn test_all_flashed() {
        assert_eq!(get_step_where_all_flashed(example()), 195);
    }
}
