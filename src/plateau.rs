// plateau.rs
#[derive(Debug, PartialEq)]
pub struct Plateau {
    width: i32,
    height: i32,
}

impl Plateau {
    pub fn new(width: i32, height: i32) -> Self {
        Plateau { width, height }
    }

    pub fn is_within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x <= self.width && y >= 0 && y <= self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_within_bounds() {
        let plateau = Plateau::new(5, 5);

        assert!(plateau.is_within_bounds(0, 0));
        assert!(plateau.is_within_bounds(5, 5));
        assert!(plateau.is_within_bounds(3, 4));

        assert!(!plateau.is_within_bounds(-1, 0));
        assert!(!plateau.is_within_bounds(0, -1));
        assert!(!plateau.is_within_bounds(6, 5));
        assert!(!plateau.is_within_bounds(5, 6));
    }
}
