use crate::direction::Direction;
use crate::instruction::Instruction;
use crate::plateau::Plateau;

#[derive(Debug, PartialEq)]
pub struct Rover<'a> {
    x: i32,
    y: i32,
    direction: Direction,
    plateau: &'a Plateau,
}

impl<'a> Rover<'a> {
    pub fn new(x: i32, y: i32, direction: Direction, plateau: &'a Plateau) -> Self {
        Rover {
            x,
            y,
            direction,
            plateau,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };
    }

    pub fn move_forward(&mut self) {
        let (new_x, new_y) = match self.direction {
            Direction::NORTH => (self.x, self.y + 1),
            Direction::EAST => (self.x + 1, self.y),
            Direction::SOUTH => (self.x, self.y - 1),
            Direction::WEST => (self.x - 1, self.y),
        };

        if self.plateau.is_within_bounds(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn execute_instructions(&mut self, instructions: &[Instruction]) {
        for &instruction in instructions {
            match instruction {
                Instruction::LEFT => self.turn_left(),
                Instruction::RIGHT => self.turn_right(),
                Instruction::MOVE => self.move_forward(),
            }
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;
    use crate::instruction::Instruction;
    use crate::plateau::Plateau;

    #[test]
    fn test_turn_left() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::NORTH, &plateau);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::WEST);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::SOUTH);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::EAST);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::NORTH);
    }

    #[test]
    fn test_turn_right() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::NORTH, &plateau);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::EAST);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::SOUTH);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::WEST);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::NORTH);
    }

    #[test]
    fn test_move_forward() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::NORTH, &plateau);
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 1);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 1);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 0);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 0);
    }

    #[test]
    fn test_move_forward_within_bounds() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(5, 5, Direction::NORTH, &plateau);
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);
    }

    #[test]
    fn test_move_forward_out_of_bounds() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(5, 5, Direction::NORTH, &plateau);
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 4);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 4);
        assert_eq!(rover.y, 4);

        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 4);

        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 4);
    }

    #[test]
    fn test_execute_instructions() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(1, 2, Direction::NORTH, &plateau);
        let instructions = [
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::MOVE,
        ];
        rover.execute_instructions(&instructions);
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 3);
        assert_eq!(rover.direction, Direction::NORTH);

        let mut rover = Rover::new(3, 3, Direction::EAST, &plateau);
        let instructions = [
            Instruction::MOVE,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::MOVE,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::RIGHT,
            Instruction::MOVE,
        ];
        rover.execute_instructions(&instructions);
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 1);
        assert_eq!(rover.direction, Direction::EAST);
    }

    #[test]
    fn test_getters() {
        let plateau = Plateau::new(5, 5);
        let rover = Rover::new(1, 2, Direction::NORTH, &plateau);

        assert_eq!(rover.x(), 1);
        assert_eq!(rover.y(), 2);
        assert_eq!(rover.direction(), &Direction::NORTH);
    }
}
