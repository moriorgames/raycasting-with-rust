use crate::map::map::Map;
use crate::shared_kernel::position::Position;

#[derive(Clone)]
pub struct Ray {
    pub origin: Position,
    pub angle: f32,
    pub distance: f32,
    pub hit_wall: bool,
}

impl Ray {
    pub const MAX_RAY_DISTANCE: f32 = 300.0;
    pub const DISTANCE_STEP: f32 = 1.0;

    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Ray {
            origin: Position::new(x, y),
            angle,
            distance: 0.0,
            hit_wall: false,
        }
    }

    pub fn cast(&mut self, map: &Map) -> f32 {
        let mut distance = 0.0;

        let direction = (self.angle.cos(), self.angle.sin());

        while distance < Self::MAX_RAY_DISTANCE {
            self.hit_wall = false;
            let new_position = self.calculate_new_position(distance, &direction);
            let map_position = self.convert_to_map_position(&new_position);

            if map.is_wall(map_position.0, map_position.1) {
                self.hit_wall = true;
                return distance;
            }

            distance += Self::DISTANCE_STEP;
        }

        distance
    }

    fn calculate_new_position(&self, distance: f32, direction: &(f32, f32)) -> (f32, f32) {
        (
            self.origin.x + direction.0 * distance,
            self.origin.y + direction.1 * distance,
        )
    }

    fn convert_to_map_position(&self, position: &(f32, f32)) -> (usize, usize) {
        (
            (position.0 / Map::SQUARE_SIZE) as usize % Map::MAP_SIZE,
            (position.1 / Map::SQUARE_SIZE) as usize % Map::MAP_SIZE,
        )
    }
}
