use crate::engine3d::{engine3d_collection::Engine3dCollection, engine3d_view::engine3d_view};
use crate::player::player::PlayerTransformation;
use crate::raycasting::ray::Ray;
use ggez::context::Context;

pub struct Engine3dState {
    engine3d_collection: Engine3dCollection,
}

impl Engine3dState {
    pub fn new() -> Self {
        Engine3dState {
            engine3d_collection: Engine3dCollection::new(),
        }
    }

    pub fn update(&mut self, rays: &[Ray], player_transformation: PlayerTransformation) {
        self.engine3d_collection
            .update(rays, player_transformation.angle);
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        let _ = engine3d_view::draw(ctx, canvas, &self.engine3d_collection);

        Ok(())
    }
}
