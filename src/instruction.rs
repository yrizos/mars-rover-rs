#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    LEFT,
    RIGHT,
    MOVE,
}

impl Instruction {
    pub fn from_char(c: char) -> Option<Instruction> {
        match c {
            'L' => Some(Instruction::LEFT),
            'R' => Some(Instruction::RIGHT),
            'M' => Some(Instruction::MOVE),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_char() {
        assert_eq!(Instruction::from_char('L'), Some(Instruction::LEFT));
        assert_eq!(Instruction::from_char('R'), Some(Instruction::RIGHT));
        assert_eq!(Instruction::from_char('M'), Some(Instruction::MOVE));
        assert_eq!(Instruction::from_char('X'), None);
        assert_eq!(Instruction::from_char(' '), None);
    }
}
