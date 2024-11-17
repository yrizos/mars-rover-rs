#[derive(Debug, PartialEq)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn as_char(&self) -> char {
        match self {
            Direction::NORTH => 'N',
            Direction::EAST => 'E',
            Direction::SOUTH => 'S',
            Direction::WEST => 'W',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_char() {
        assert_eq!(Direction::NORTH.as_char(), 'N');
        assert_eq!(Direction::EAST.as_char(), 'E');
        assert_eq!(Direction::SOUTH.as_char(), 'S');
        assert_eq!(Direction::WEST.as_char(), 'W');
    }
}
