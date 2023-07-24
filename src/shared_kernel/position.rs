#[derive(Default, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }

    pub fn update_from(&mut self, position: Position) {
        self.x = position.x;
        self.y = position.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let sut = Position::new(10.0, 20.0);
        assert_eq!(sut.x, 10.0);
        assert_eq!(sut.y, 20.0);
    }
}
