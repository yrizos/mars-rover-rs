mod direction;
mod instruction;
mod plateau;
mod rover;

use direction::Direction;
use instruction::Instruction;
use plateau::Plateau;
use rover::Rover;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let plateau_dimensions = &lines[0];
    let plateau_parts: Vec<i32> = plateau_dimensions
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let plateau = Plateau::new(plateau_parts[0], plateau_parts[1]);

    let mut i = 1;
    while i < lines.len() {
        if i + 1 >= lines.len() {
            break;
        }

        let rover_initial = &lines[i];
        let rover_parts: Vec<&str> = rover_initial.split_whitespace().collect();
        let x: i32 = rover_parts[0].parse().unwrap();
        let y: i32 = rover_parts[1].parse().unwrap();
        let direction = match rover_parts[2] {
            "N" => Direction::NORTH,
            "E" => Direction::EAST,
            "S" => Direction::SOUTH,
            "W" => Direction::WEST,
            _ => panic!("Invalid direction"),
        };

        let instructions_line = &lines[i + 1];
        let instructions: Vec<Instruction> = instructions_line
            .chars()
            .filter_map(Instruction::from_char)
            .collect();

        let mut rover = Rover::new(x, y, direction, &plateau);
        rover.execute_instructions(&instructions);

        println!(
            "{} {} {}",
            rover.x(),
            rover.y(),
            rover.direction().as_char()
        );

        i += 2;
    }
}
