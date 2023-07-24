use crate::map::map::Map;
use crate::shared_kernel::position::Position;

pub struct Player {
    position: Position,
    angle: f32,
}

#[derive(Default, Clone)]
pub struct PlayerTransformation {
    pub position: Position,
    pub angle: f32,
}

impl Player {
    pub const DEFAULT_X: f32 = Map::SQUARE_SIZE * 1.5;
    pub const DEFAULT_Y: f32 = Map::SQUARE_SIZE * 1.5;
    const DEFAULT_ANGLE: f32 = 0.0;

    pub fn new(x: f32, y: f32) -> Self {
        Player {
            position: Position::new(x, y),
            angle: Self::DEFAULT_ANGLE,
        }
    }

    pub fn move_forward(&mut self, speed: f32, map: &Map) {
        let delta_x = speed * self.angle.cos();
        let delta_y = speed * self.angle.sin();

        if map.is_position_transitable(self.position.x + delta_x, self.position.y) {
            self.position.x += delta_x;
        }
        if map.is_position_transitable(self.position.x, self.position.y + delta_y) {
            self.position.y += delta_y;
        }
    }

    pub fn move_backward(&mut self, speed: f32, map: &Map) {
        let delta_x = -speed * self.angle.cos();
        let delta_y = -speed * self.angle.sin();

        if map.is_position_transitable(self.position.x + delta_x, self.position.y) {
            self.position.x += delta_x;
        }
        if map.is_position_transitable(self.position.x, self.position.y + delta_y) {
            self.position.y += delta_y;
        }
    }

    pub fn rotate_left(&mut self, rotation: f32) {
        self.angle -= rotation;
        if self.angle < 0.0 {
            self.angle += 2.0 * std::f32::consts::PI;
        }
    }

    pub fn rotate_right(&mut self, rotation: f32) {
        self.angle += rotation;
        if self.angle > 2.0 * std::f32::consts::PI {
            self.angle -= 2.0 * std::f32::consts::PI;
        }
    }

    pub fn player_transformation(&self) -> PlayerTransformation {
        PlayerTransformation {
            position: self.position.clone(),
            angle: self.angle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    const SPEED: f32 = 0.1;

    fn setup() -> Player {
        Player::new(Player::DEFAULT_X, Player::DEFAULT_Y)
    }

    #[test]
    fn test_player_new() {
        let sut = setup();

        assert_eq!(sut.position.x, Player::DEFAULT_X);
        assert_eq!(sut.position.y, Player::DEFAULT_Y);
    }

    #[test]
    fn test_player_move_forward() {
        let map = Map::new();
        let mut sut = setup();

        sut.move_forward(SPEED, &map);

        assert_eq!(sut.position.x, Player::DEFAULT_X + SPEED);
        assert_eq!(sut.position.y, Player::DEFAULT_Y);
    }

    #[test]
    fn test_player_moves_backward() {
        let map = Map::new();
        let mut sut = setup();

        sut.move_backward(SPEED, &map);

        assert_eq!(sut.position.x, Player::DEFAULT_X - SPEED);
        assert_eq!(sut.position.y, Player::DEFAULT_Y);
    }

    #[test]
    fn test_player_rotates_left() {
        let mut sut = setup();
        let initial_angle = sut.player_transformation().angle;
        let rotation = PI / 4.0; // Rotate 45 degrees

        sut.rotate_left(rotation);

        let expected_angle = (initial_angle - rotation).rem_euclid(2.0 * std::f32::consts::PI);
        assert_eq!(sut.player_transformation().angle, expected_angle);
    }

    #[test]
    fn test_player_rotates_right() {
        let mut sut = setup();
        let initial_angle = sut.player_transformation().angle;
        let rotation = std::f32::consts::PI / 4.0; // Rotate 45 degrees

        sut.rotate_right(rotation);

        let expected_angle = (initial_angle + rotation) % (2.0 * std::f32::consts::PI);
        assert_eq!(sut.player_transformation().angle, expected_angle);
    }
}
