use crate::map::map::Map;
use crate::player::player::PlayerTransformation;
use crate::raycasting::{ray::Ray, raycasting_view::raycasting_view};
use ggez::context::Context;

const RAYS_COUNT: usize = 90;
const RAYS_ANGLE_SUB: f32 = 30.;
const DEGREE_PER_RAY: f32 = 60. / RAYS_COUNT as f32;

pub struct RaycastingState {
    rays: Vec<Ray>,
    map: Map,
}

impl RaycastingState {
    pub fn new(map: Map) -> Self {
        let mut rays = Vec::with_capacity(RAYS_COUNT);
        for i in 0..RAYS_COUNT {
            let angle = (i as f32 * DEGREE_PER_RAY - RAYS_ANGLE_SUB).to_radians();
            rays.push(Ray::new(0., 0., angle));
        }

        RaycastingState { rays, map }
    }

    pub fn update(&mut self, player_transformation: PlayerTransformation) -> &[Ray] {
        for (i, ray) in self.rays.iter_mut().enumerate() {
            let angle = (i as f32 * DEGREE_PER_RAY - RAYS_ANGLE_SUB).to_radians();
            ray.origin
                .update_from(player_transformation.position.clone());
            ray.angle = player_transformation.angle + angle;

            ray.distance = ray.cast(&self.map);
        }

        &self.rays
    }

    #[allow(dead_code)]
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        for ray in &self.rays {
            let _ = raycasting_view::draw(ctx, canvas, ray);
        }

        Ok(())
    }
}
