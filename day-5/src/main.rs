use std::collections::HashMap;
use std::fs;

type Point = (i32, i32);

#[derive(PartialEq, Debug)]
struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn get_points(&self) -> Vec<Point> {
        let x_inc = (self.x2 - self.x1).clamp(-1, 1);

        let y_inc = (self.y2 - self.y1).clamp(-1, 1);

        let mut result = Vec::new();

        let mut done = false;
        let (mut x, mut y) = (self.x1, self.y1);

        while !done {
            result.push((x, y));

            if (x, y) == (self.x2, self.y2) {
                done = true;
            }

            x += x_inc;
            y += y_inc;
        }

        result
    }
}

fn main() {
    let input = fs::read_to_string("day-5/input.txt").unwrap();

    let input = process_input(input);

    let overlapping_points = overlap_points(&input);

    let straight_lines: Vec<Line> = input.into_iter().filter(|x| x.is_straight()).collect();

    println!(
        "Overlapping points (straight): {}",
        overlap_points(&straight_lines)
    );

    println!("Overlapping points: {}", overlapping_points);
}

fn process_input(input: String) -> Vec<Line> {
    input.lines().fold(Vec::new(), |mut result, line| {
        let mut line_iter = line.split(" -> ");
        let x_elements: Vec<i32> = line_iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (x1, y1) = (x_elements[0], x_elements[1]);

        let y_elements: Vec<i32> = line_iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (x2, y2) = (y_elements[0], y_elements[1]);

        result.push(Line { x1, x2, y1, y2 });
        result
    })
}

fn overlap_points(lines: &Vec<Line>) -> usize {
    let map = lines.iter().fold(
        HashMap::new(),
        |mut result_map: HashMap<(i32, i32), i32>, line| {
            line.get_points().iter().for_each(|point| {
                let entry = result_map.entry(*point).or_insert(0);
                *entry += 1;
            });
            result_map
        },
    );

    map.values().filter(|x| **x > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        String::from("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2")
    }

    #[test]
    fn input_test() {
        assert_eq!(
            process_input(example())[0],
            Line {
                x1: 0,
                x2: 5,
                y1: 9,
                y2: 9
            }
        );
    }

    #[test]
    fn test_overlap_points() {
        assert_eq!(
            overlap_points(&vec![
                Line {
                    x1: 0,
                    x2: 5,
                    y1: 9,
                    y2: 9
                },
                Line {
                    x1: 0,
                    x2: 2,
                    y1: 9,
                    y2: 9
                },
                Line {
                    x1: 4,
                    x2: 0,
                    y1: 9,
                    y2: 9
                },
                Line {
                    x1: 4,
                    x2: 5,
                    y1: 8,
                    y2: 9
                }
            ]),
            6
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(overlap_points(&process_input(example())), 12)
    }
}
