#[derive(Clone)]
pub struct Map {
    map: [[u16; Self::MAP_SIZE]; Self::MAP_SIZE],
}

impl Map {
    pub const SQUARE_SIZE: f32 = 20.0;
    pub const MAP_SIZE: usize = 16;

    pub fn new() -> Self {
        let map = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
            [1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1],
            [1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1],
            [1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        Map { map }
    }

    pub fn data(&self) -> &[[u16; Self::MAP_SIZE]; Self::MAP_SIZE] {
        &self.map
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.map[y][x] == 1
    }

    pub fn is_position_transitable(&self, x: f32, y: f32) -> bool {
        let grid_x = (x / Self::SQUARE_SIZE).floor() as usize;
        let grid_y = (y / Self::SQUARE_SIZE).floor() as usize;

        if grid_x >= Self::MAP_SIZE || grid_y >= Self::MAP_SIZE {
            return false;
        }

        return self.map[grid_y][grid_x] != 1;
    }
}
