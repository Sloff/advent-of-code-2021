use std::fs;

type Heatmap = Vec<Vec<u32>>;

fn main() {
    let input = fs::read_to_string("day-9/input.txt").unwrap();

    let heatmap: Heatmap = parse_input(&input);

    let sum_of_low_points = get_sum_of_low_points(&heatmap);

    println!("Sum of the risk at the low points: {}", sum_of_low_points);

    println!(
        "Product of 3 largest basins: {}",
        get_product_of_largest_basins(&heatmap)
    );
}

fn parse_input(input: &str) -> Heatmap {
    input.trim().lines().fold(Vec::new(), |mut result, line| {
        result.push(
            line.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect(),
        );

        result
    })
}

fn get_sum_of_low_points(heatmap: &Heatmap) -> u32 {
    let low_points = get_low_points(heatmap);

    low_points.iter().fold(0, |result, (i_line, i_col)| {
        result + heatmap.get(*i_line).unwrap().get(*i_col).unwrap() + 1
    })
}

fn get_product_of_largest_basins(heatmap: &Heatmap) -> u32 {
    let low_points = get_low_points(heatmap);

    let mut basins: Vec<u32> = low_points
        .into_iter()
        .map(|(i_line, i_col)| {
            let mut result = 0;
            let mut entries_to_process = vec![(i_line, i_col)];
            let mut already_visited = vec![(i_line, i_col)];

            while entries_to_process.len() > 0 {
                let (i_line, i_col) = entries_to_process.pop().unwrap();
                result += 1;

                let above = if i_line == 0 {
                    None
                } else {
                    heatmap.get(i_line - 1)
                };
                let above = above.and_then(|x| x.get(i_col));

                if above.is_some()
                    && *above.unwrap() != 9
                    && !already_visited.contains(&(i_line - 1, i_col))
                {
                    entries_to_process.push((i_line - 1, i_col));
                    already_visited.push((i_line - 1, i_col));
                }

                let below = heatmap.get(i_line + 1);
                let below = below.and_then(|x| x.get(i_col));

                if below.is_some()
                    && *below.unwrap() != 9
                    && !already_visited.contains(&(i_line + 1, i_col))
                {
                    entries_to_process.push((i_line + 1, i_col));
                    already_visited.push((i_line + 1, i_col));
                }

                let left = heatmap.get(i_line);
                let left = if i_col == 0 {
                    None
                } else {
                    left.and_then(|x| x.get(i_col - 1))
                };

                if left.is_some()
                    && *left.unwrap() != 9
                    && !already_visited.contains(&(i_line, i_col - 1))
                {
                    entries_to_process.push((i_line, i_col - 1));
                    already_visited.push((i_line, i_col - 1));
                }

                let right = heatmap.get(i_line);
                let right = right.and_then(|x| x.get(i_col + 1));
                if right.is_some()
                    && *right.unwrap() != 9
                    && !already_visited.contains(&(i_line, i_col + 1))
                {
                    entries_to_process.push((i_line, i_col + 1));
                    already_visited.push((i_line, i_col + 1));
                }
            }

            result
        })
        .collect();

    basins.sort();

    basins.iter().rev().take(3).product()
}

fn get_low_points(heatmap: &Heatmap) -> Vec<(usize, usize)> {
    heatmap
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut result, (i_line, line)| {
            result.push(
                line.iter()
                    .enumerate()
                    .fold(Vec::new(), |mut res, (i_col, col)| {
                        let above = if i_line == 0 {
                            None
                        } else {
                            heatmap.get(i_line - 1)
                        };
                        let above = above.and_then(|x| x.get(i_col));

                        let below = heatmap.get(i_line + 1);
                        let below = below.and_then(|x| x.get(i_col));

                        let left = heatmap.get(i_line);
                        let left = if i_col == 0 {
                            None
                        } else {
                            left.and_then(|x| x.get(i_col - 1))
                        };

                        let right = heatmap.get(i_line);
                        let right = right.and_then(|x| x.get(i_col + 1));

                        let adjacent_locations = vec![above, below, left, right];

                        let is_lower = adjacent_locations
                            .iter()
                            .filter(|x| x.is_some())
                            .fold(true, |res, elem| res && elem.unwrap() > col);

                        if is_lower {
                            res.push((i_line, i_col));
                        }

                        res
                    }),
            );

            result
        })
        .into_iter()
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "2199943210\n3987894921\n9856789892\n8767896789\n9899965678"
    }

    #[test]
    fn sum_of_low_points() {
        assert_eq!(get_sum_of_low_points(&parse_input(example())), 15);
    }

    #[test]
    fn product_of_largest_basins() {
        assert_eq!(get_product_of_largest_basins(&parse_input(example())), 1134);
    }
}
