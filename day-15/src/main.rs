use std::collections::HashMap;
use std::fs;

type Grid = HashMap<(usize, usize), (usize, usize)>;

fn get_grid(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (x, line)| {
            line.trim().chars().enumerate().for_each(|(y, letter)| {
                map.insert((x, y), (letter.to_digit(10).unwrap() as usize, usize::MAX));
            });

            map
        })
}

fn get_full_grid(input: &str) -> Grid {
    let wrap = |x: usize, i: usize| {
        if x + i >= 10 {
            x + i - 9
        } else {
            x + i
        }
    };

    let max_size = input.trim().lines().nth(0).unwrap().trim().len();

    input
        .trim()
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (x, line)| {
            line.trim().chars().enumerate().for_each(|(y, letter)| {
                let num = letter.to_digit(10).unwrap() as usize;
                (0..5).for_each(|r| {
                    (0..5).for_each(|c| {
                        map.insert(
                            (x + (r * max_size), y + (c * max_size)),
                            (wrap(num, r + c), usize::MAX),
                        );
                    })
                })
            });

            map
        })
}

fn get_shortest_path(grid: &mut Grid) -> usize {
    let mut current_node_key = (0, 0);
    grid.get_mut(&current_node_key).unwrap().1 = 0;

    let grid_max = (grid.len() as f64).sqrt() as usize;

    let final_node_key = (grid_max - 1, grid_max - 1);

    while current_node_key != final_node_key {
        let current_node = grid.remove(&current_node_key).unwrap();

        let x = current_node_key.0 as i64;
        let y = current_node_key.1 as i64;

        let surrounding_node_keys = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        surrounding_node_keys.into_iter().for_each(|(x, y)| {
            if let Some(node) = grid.get_mut(&(x as usize, y as usize)) {
                let new_total = current_node.1 + node.0;

                if new_total < node.1 {
                    node.1 = new_total;
                }
            }
        });

        println!("Progress: {}", grid.len());

        current_node_key = *grid.into_iter().min_by_key(|(_, val)| val.1).unwrap().0
    }

    grid.get(&final_node_key).unwrap().1
}

fn main() {
    let input = fs::read_to_string("day-15/input.txt").unwrap();

    println!(
        "Shortest path: {}",
        get_shortest_path(&mut get_grid(&input))
    );

    println!(
        "Shortest full path: {}",
        get_shortest_path(&mut get_full_grid(&input))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581"
    }

    #[test]
    fn shortest_path() {
        assert_eq!(get_shortest_path(&mut get_grid(example())), 40);
    }

    #[test]
    fn shortest_path_on_full_grid() {
        assert_eq!(get_shortest_path(&mut get_full_grid(example())), 315);
    }
}
