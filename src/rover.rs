use crate::direction::Direction;

#[derive(Debug, PartialEq)]
pub struct Rover {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Rover {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Rover { x, y, direction }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn test_turn_left() {
        let mut rover = Rover::new(0, 0, Direction::N);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::W);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::S);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::E);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::N);
    }

    #[test]
    fn test_turn_right() {
        let mut rover = Rover::new(0, 0, Direction::N);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::E);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::S);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::W);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::N);
    }
}
