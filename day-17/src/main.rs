fn main() {
    // target area: x=288..330, y=-96..-50
    let mut highest = i64::MIN;
    let mut num_in_target = 0;

    (0..400).for_each(|x| {
        (-100..100).for_each(|y| {
            let (mut x_vel, mut y_vel) = (x, y);
            let (mut x_pos, mut y_pos) = (0, 0);
            let mut inner_highest_y = i64::MIN;

            let in_target = loop {
                x_pos += x_vel;
                y_pos += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }

                if x_vel < 0 {
                    x_vel += 1;
                }

                y_vel -= 1;

                if y_pos > inner_highest_y {
                    inner_highest_y = y_pos;
                }

                if y_pos < -96 || x_pos > 330 {
                    break false;
                };

                if x_pos >= 288 && x_pos <= 330 && y_pos >= -96 && y_pos <= -50 {
                    num_in_target += 1;
                    break true;
                }
            };

            if in_target && inner_highest_y > highest {
                highest = inner_highest_y;
            }
        });
    });

    println!("Highest: {}", highest);
    println!("Num in Target: {}", num_in_target);
}
