use std::collections::HashMap;
use std::fs;

type MapType = HashMap<String, Vec<String>>;

fn main() {
    let input = fs::read_to_string("day-12/input.txt").unwrap();

    println!(
        "Paths passing through at most 1 small cave: {}",
        get_paths(&process_input(&input)).len()
    );

    println!(
        "Paths passing through at most 1 small cave twice: {}",
        get_paths_2(&process_input(&input)).len()
    );
}

fn process_input(input: &str) -> MapType {
    input.trim().lines().fold(HashMap::new(), |mut map, line| {
        let mut line_iter = line.trim().split("-");
        let left = line_iter.next().unwrap();
        let right = line_iter.next().unwrap();

        let left_entry = map.entry(left.to_string()).or_insert(Vec::new());
        left_entry.push(right.to_string());

        if left != "start" && right != "end" {
            let right_entry = map.entry(right.to_string()).or_insert(Vec::new());
            right_entry.push(left.to_string());
        }

        map
    })
}

fn get_paths(map: &MapType) -> Vec<Vec<String>> {
    let mut paths_sofar = vec![vec!["start".to_string()]];
    let mut result: Vec<Vec<String>> = Vec::new();

    while paths_sofar.len() > 0 {
        let path_sofar = paths_sofar.pop().unwrap();
        let mut paths_to_explore = map
            .get(&path_sofar[path_sofar.len() - 1])
            .unwrap_or(&vec![])
            .clone();

        while paths_to_explore.len() > 0 {
            let path = paths_to_explore.pop().unwrap();

            if path.len() > 2 || *path != path.to_lowercase() || !path_sofar.contains(&path) {
                let mut new_path = path_sofar.clone();
                new_path.push(path);

                paths_sofar.push(new_path);
            }
        }

        if path_sofar.iter().last().unwrap() == "end" {
            result.push(path_sofar);
        }
    }

    result
}

fn get_paths_2(map: &MapType) -> Vec<Vec<String>> {
    let mut paths_sofar = vec![vec!["start".to_string()]];
    let mut result: Vec<Vec<String>> = Vec::new();

    while paths_sofar.len() > 0 {
        let path_sofar = paths_sofar.pop().unwrap();
        let mut paths_to_explore = map
            .get(&path_sofar[path_sofar.len() - 1])
            .unwrap_or(&vec![])
            .clone();

        while paths_to_explore.len() > 0 {
            let path = paths_to_explore.pop().unwrap();

            let number_of_double_lower = path_sofar.iter().fold(0, |res, x| {
                if x.len() <= 2
                    && *x == x.to_lowercase()
                    && path_sofar.iter().filter(|y| *y == x).count() > 1
                {
                    res + 1
                } else {
                    res
                }
            });

            if path.len() > 2
                || *path != path.to_lowercase()
                || !path_sofar.contains(&path)
                || number_of_double_lower < 1
            {
                let mut new_path = path_sofar.clone();
                new_path.push(path);

                paths_sofar.push(new_path);
            }
        }

        if path_sofar.iter().last().unwrap() == "end" {
            result.push(path_sofar);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end"
    }

    fn example_2() -> &'static str {
        "dc-end\nstart-HN\nstart-kj\nstart-dc\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
    }

    #[test]
    fn test_process_input() {
        let mut map = HashMap::new();
        map.insert("start".to_string(), vec!["A".to_string(), "b".to_string()]);
        map.insert("c".to_string(), vec!["A".to_string()]);
        map.insert("d".to_string(), vec!["b".to_string()]);
        map.insert(
            "A".to_string(),
            vec!["c".to_string(), "b".to_string(), "end".to_string()],
        );
        map.insert(
            "b".to_string(),
            vec!["A".to_string(), "d".to_string(), "end".to_string()],
        );

        assert_eq!(process_input(example()), map);
    }

    #[test]
    fn test_1() {
        assert_eq!(get_paths(&process_input(example_2())).len(), 19);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_paths_2(&process_input(example())).len(), 36);
    }
}
