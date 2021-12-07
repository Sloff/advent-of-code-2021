use std::fs;

fn main() {
    let input = fs::read_to_string("day-7/input.txt").unwrap();

    println!(
        "Min fuel to get all aligned: {}",
        get_min_fuel(&parse_input(input.trim()))
    );

    println!(
        "Min fuel to get all aligned (advanced): {}",
        get_min_fuel_2(&parse_input(input.trim()))
    );
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn get_min_max(positions: &Vec<i32>) -> (i32, i32) {
    positions
        .iter()
        .skip(1)
        .fold((positions[0], positions[0]), |(min, max), position| {
            if *position < min {
                (*position, max)
            } else if *position > max {
                (min, *position)
            } else {
                (min, max)
            }
        })
}

fn get_min_fuel(positions: &Vec<i32>) -> i32 {
    let (min, max) = get_min_max(&positions);

    (min..=max).fold(i32::MAX, |fuel, position| {
        let fuel_to_position = positions.iter().map(|x| (*x - position).abs()).sum();
        if fuel_to_position < fuel {
            fuel_to_position
        } else {
            fuel
        }
    })
}

fn get_min_fuel_2(positions: &Vec<i32>) -> i32 {
    let (min, max) = get_min_max(&positions);

    (min..=max).fold(i32::MAX, |fuel, position| {
        let fuel_to_position = positions
            .iter()
            .map(|x| {
                let x = (*x - position).abs();
                (x * (x + 1)) / 2
            })
            .sum();
        if fuel_to_position < fuel {
            fuel_to_position
        } else {
            fuel
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn input() {
        assert_eq!(parse_input(example()), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
    }

    #[test]
    fn min_max() {
        assert_eq!(get_min_max(&parse_input(example())), (0, 16));
    }

    #[test]
    fn min_fuel() {
        assert_eq!(get_min_fuel(&parse_input(example())), 37)
    }

    #[test]
    fn min_fuel_2() {
        assert_eq!(get_min_fuel_2(&parse_input(example())), 168)
    }
}
