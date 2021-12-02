use std::fs;

type Instruction<'a> = (&'a str, i32);
type Instructions<'a> = Vec<Instruction<'a>>;

fn main() {
    let input = fs::read_to_string("day-2/input.txt").unwrap();

    let instructions: Instructions = process_input(&input[..]);

    let mut position = Position::new();
    position.process_instructions(&instructions);

    let mut position_with_aim = Position::new_aim();
    position_with_aim.process_instructions(&instructions);

    println!("Final position: {}", position.get_position());
    println!(
        "Final position advanced: {}",
        position_with_aim.get_position()
    );
}

fn process_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|x| {
            let mut input_iter = x.split_whitespace();
            (
                input_iter.next().unwrap(),
                input_iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Copy, Clone)]
enum PositionType {
    Naive,
    Aim,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
    position_type: PositionType,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
            position_type: PositionType::Naive,
        }
    }

    fn new_aim() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
            position_type: PositionType::Aim,
        }
    }

    fn process_position(&mut self, instruction: &Instruction) {
        match self.position_type {
            PositionType::Naive => match instruction {
                ("forward", x) => self.horizontal += x,
                ("down", x) => self.depth += x,
                ("up", x) => self.depth -= x,
                _ => unreachable!(),
            },
            PositionType::Aim => match instruction {
                ("forward", x) => {
                    self.horizontal += x;
                    self.depth += self.aim * x;
                }
                ("down", x) => self.aim += x,
                ("up", x) => self.aim -= x,
                _ => unreachable!(),
            },
        };
    }

    fn process_instructions(&mut self, instructions: &Instructions) {
        instructions
            .into_iter()
            .for_each(|instruction| self.process_position(instruction));
    }

    fn get_position(&self) -> i32 {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(
            process_input("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            vec![
                ("forward", 5),
                ("down", 5),
                ("forward", 8),
                ("up", 3),
                ("down", 8),
                ("forward", 2)
            ]
        );
    }

    #[test]
    fn position() {
        let mut pos = Position::new();
        pos.process_position(&("forward", 5));
        assert_eq!((pos.horizontal, pos.depth), (5, 0));
    }

    #[test]
    fn result() {
        let mut pos = Position::new();
        pos.process_instructions(&process_input(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
        ));
        assert_eq!(pos.get_position(), 150);
    }

    #[test]
    fn result2() {
        let mut pos = Position::new_aim();
        pos.process_instructions(&process_input(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
        ));
        assert_eq!(pos.get_position(), 900);
    }
}
