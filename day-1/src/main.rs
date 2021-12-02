use std::fs;
use std::iter::Iterator;

fn main() {
    let input = fs::read_to_string("day-1/input.txt").unwrap();

    let input = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("First answer: {}", get_number_of_increases(&input));
    println!(
        "Second answer: {}",
        get_number_of_increases(&get_three_pairs_sum(input))
    );
}

fn get_number_of_increases(input: &Vec<i32>) -> i32 {
    input
        .into_iter()
        .fold((0, &i32::MAX), |(counter, prev_value), val| {
            (
                if val > prev_value {
                    counter + 1
                } else {
                    counter
                },
                val,
            )
        })
        .0
}

fn get_three_pairs_sum(input: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    let mut iter1 = input.iter();
    let mut iter2 = input.iter().skip(1);
    let mut iter3 = input.iter().skip(2);

    loop {
        match (iter1.next(), iter2.next(), iter3.next()) {
            (Some(x), Some(y), Some(z)) => result.push(x + y + z),
            _ => break,
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_increases() {
        assert_eq!(
            get_number_of_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn three_pairs_sum() {
        assert_eq!(
            get_three_pairs_sum(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        );
    }
}
