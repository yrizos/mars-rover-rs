mod direction;
mod instruction;
mod plateau;
mod rover;

use direction::Direction;
use instruction::Instruction;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(plateau_dimensions)) = lines.next() {
        let plateau_parts: Vec<i32> = plateau_dimensions
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!("Plateau dimensions: {:?}", plateau_parts);
    }

    if let Some(Ok(rover_initial)) = lines.next() {
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
        println!(
            "Rover initial position: ({}, {}), Direction: {:?}",
            x, y, direction
        );
    }

    if let Some(Ok(instructions_line)) = lines.next() {
        let instructions: Vec<Instruction> = instructions_line
            .chars()
            .filter_map(Instruction::from_char)
            .collect();
        println!("Rover instructions: {:?}", instructions);
    }
}
