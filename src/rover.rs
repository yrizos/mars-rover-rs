use crate::direction::Direction;
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

    pub fn move_forward(&mut self) {
        let (new_x, new_y) = match self.direction {
            Direction::N => (self.x, self.y + 1),
            Direction::E => (self.x + 1, self.y),
            Direction::S => (self.x, self.y - 1),
            Direction::W => (self.x - 1, self.y),
        };

        if self.plateau.is_within_bounds(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn test_turn_left() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);

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
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::E);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::S);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::W);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::N);
    }

    #[test]
    fn test_move_forward() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);

        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 1);

        rover.direction = Direction::E;
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 1);

        rover.direction = Direction::S;
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 0);

        rover.direction = Direction::W;
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 0);
    }

    #[test]
    fn test_move_forward_within_bounds() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(5, 5, Direction::N, &plateau);
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.direction = Direction::E;
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);
    }
}
