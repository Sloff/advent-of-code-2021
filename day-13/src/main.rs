use std::fs;

fn main() {
    let input = fs::read_to_string("day-13/input.txt").unwrap();
    let folds = fs::read_to_string("day-13/input2.txt").unwrap();

    let input: Vec<(i32, i32)> = input.trim().lines().fold(Vec::new(), |mut result, line| {
        let mut line_iter = line.trim().split(',');

        result.push((
            line_iter.next().unwrap().parse().unwrap(),
            line_iter.next().unwrap().parse().unwrap(),
        ));
        result
    });

    let mut count = 0;

    let dots = folds.lines().fold(input, |result, line| {
        if count == 1 {
            println!("Dots after 1 fold: {}", result.len());
        }
        count += 1;

        let mut line_iter = line.trim().split('=');
        let (axis, num) = (
            line_iter.next().unwrap(),
            line_iter.next().unwrap().parse::<i32>().unwrap(),
        );

        match axis {
            "x" => fold_vertical(result, num),
            "y" => fold_horizontal(result, num),
            _ => unreachable!(),
        }
    });

    let (max_x, max_y) = dots.iter().fold((0, 0), |mut result, (x, y)| {
        if *x > result.0 {
            result.0 = *x
        }

        if *y > result.1 {
            result.1 = *y
        }

        result
    });

    (0..=max_y).for_each(|x| {
        (0..=max_x).for_each(|y| {
            if dots.contains(&(y, x)) {
                print!("█");
            } else {
                print!(" ");
            }
        });

        print!("\n");
    })
}

fn fold_horizontal(dots: Vec<(i32, i32)>, y_fold: i32) -> Vec<(i32, i32)> {
    dots.into_iter().fold(Vec::new(), |mut result, (x, mut y)| {
        if y > y_fold {
            let distance = y - y_fold;
            y = y - (distance * 2);
        }

        if !result.contains(&(x, y)) {
            result.push((x, y));
        }

        result
    })
}

fn fold_vertical(dots: Vec<(i32, i32)>, x_fold: i32) -> Vec<(i32, i32)> {
    dots.into_iter().fold(Vec::new(), |mut result, (mut x, y)| {
        if x > x_fold {
            let distance = x - x_fold;
            x = x - (distance * 2);
        }

        if !result.contains(&(x, y)) {
            result.push((x, y));
        }

        result
    })
}
