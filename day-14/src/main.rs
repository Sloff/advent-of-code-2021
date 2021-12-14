use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day-14/input.txt").unwrap();

    let (initial_str, map) = parse_input(&input);

    println!("Result after 10 steps: {}", polymer(initial_str, &map, 10));
    println!("Result after 40 steps: {}", polymer(initial_str, &map, 40));
}

fn parse_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let mut line_iter = input.trim().lines();
    let initial_str = line_iter.next().unwrap();
    line_iter.next();

    let map = line_iter
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|(c, v)| {
            (
                (c.as_bytes()[0] as char, c.as_bytes()[1] as char),
                v.as_bytes()[0] as char,
            )
        })
        .collect();

    (initial_str, map)
}

fn polymer(input: &str, map: &HashMap<(char, char), char>, step: usize) -> usize {
    let pair_occurrences: HashMap<(char, char), usize> = input
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .fold(HashMap::new(), |mut result, x| {
            let entry = result.entry((x[0], x[1])).or_insert(0);
            *entry += 1;

            result
        });

    let pair_occurrences = (0..step).fold(pair_occurrences, |result, _| {
        result.iter().fold(HashMap::new(), |mut res, (key, val)| {
            let new_letter = map.get(key).unwrap();

            *res.entry((key.0, *new_letter)).or_insert(0) += val;
            *res.entry((*new_letter, key.1)).or_insert(0) += val;

            res
        })
    });

    let mut letter_occurrences =
        pair_occurrences
            .iter()
            .fold(HashMap::new(), |mut result, (key, val)| {
                *result.entry(key.1).or_insert(0) += val;
                result
            });

    *letter_occurrences
        .entry(input.as_bytes()[0] as char)
        .or_insert(0) += 1;

    let max = letter_occurrences.iter().map(|(_, x)| *x).max().unwrap();
    let min = letter_occurrences.iter().map(|(_, x)| *x).min().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C"
    }

    #[test]
    fn test_parse() {
        let (input, map) = parse_input(example());
        assert_eq!(input, "NNCB");
        assert_eq!(map.get(&('H', 'N')).unwrap(), &'C');
        assert_eq!(map.get(&('H', 'H')).unwrap(), &'N');
    }

    #[test]
    fn test_10() {
        let (input, map) = parse_input(example());
        assert_eq!(polymer(input, &map, 10), 1588);
    }

    #[test]
    fn test_40() {
        let (input, map) = parse_input(example());
        assert_eq!(polymer(input, &map, 40), 2188189693529);
    }
}
