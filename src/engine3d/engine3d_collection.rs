use crate::raycasting::ray::Ray;

pub struct Engine3dCollection {
    pub rays: Vec<Ray>,
    pub player_angle: f32,
}

impl Engine3dCollection {
    pub fn new() -> Self {
        Engine3dCollection {
            rays: vec![],
            player_angle: 0.,
        }
    }

    pub fn update(&mut self, rays: &[Ray], player_angle: f32) {
        self.rays.clear();
        self.rays.extend_from_slice(rays);
        self.player_angle = player_angle;
    }
}
