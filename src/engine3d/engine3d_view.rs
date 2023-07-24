pub mod engine3d_view {
    use crate::constants;
    use crate::engine3d::{
        engine3d_collection::Engine3dCollection, engine3d_view_colors::engine3d_view_colors::*,
    };
    use crate::raycasting::ray::Ray;
    use ggez::graphics::{self, Canvas, Color};
    use ggez::{Context, GameResult};

    pub const HEIGHT_FACTOR: f32 = 50.0;

    pub fn draw(
        ctx: &mut Context,
        canvas: &mut Canvas,
        engine3d_collection: &Engine3dCollection,
    ) -> GameResult {
        draw_sky(ctx, canvas)?;

        draw_ground(ctx, canvas)?;

        let column_width = constants::WINDOW_WIDTH / engine3d_collection.rays.len() as f32;
        let player_angle = engine3d_collection.player_angle;

        for (column_index, ray) in engine3d_collection.rays.iter().enumerate() {
            if let Some(mesh) = create_rect(ctx, column_width, column_index, ray, player_angle)? {
                canvas.draw(&mesh, graphics::DrawParam::new());
            }
        }

        Ok(())
    }

    fn create_rect(
        ctx: &mut Context,
        column_width: f32,
        column_index: usize,
        ray: &Ray,
        player_angle: f32,
    ) -> GameResult<Option<graphics::Mesh>> {
        if !ray.hit_wall {
            return Ok(None);
        }

        let rect_x = column_index as f32 * column_width;
        let corrected_distance = ray.distance * (ray.angle - player_angle).cos();
        let rect_height = HEIGHT_FACTOR / (corrected_distance / Ray::MAX_RAY_DISTANCE);
        let rect_y = (constants::WINDOW_HEIGHT - rect_height) / 2.0;
        let rect = graphics::Rect::new(rect_x, rect_y, column_width, rect_height);

        let shading_factor = 0.5 - ray.distance / Ray::MAX_RAY_DISTANCE;
        let color = Color::new(
            WALL_R + shading_factor,
            WALL_G + shading_factor,
            WALL_B + shading_factor,
            1.0,
        );

        let mb = &mut graphics::MeshBuilder::new();
        mb.rectangle(graphics::DrawMode::fill(), rect, color)?;

        Ok(Some(graphics::Mesh::from_data(ctx, mb.build())))
    }

    fn draw_sky(ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let sky = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0,
                0.0,
                constants::WINDOW_WIDTH,
                constants::WINDOW_HEIGHT / 2.,
            ),
            Color::new(SKY_R, SKY_G, SKY_B, 1.0),
        )?;

        canvas.draw(&sky, graphics::DrawParam::new());

        Ok(())
    }

    fn draw_ground(ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let ground = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0,
                constants::WINDOW_HEIGHT / 2.,
                constants::WINDOW_WIDTH,
                constants::WINDOW_HEIGHT / 2.,
            ),
            Color::new(GROUND_R, GROUND_G, GROUND_B, 1.0),
        )?;

        canvas.draw(&ground, graphics::DrawParam::new());

        Ok(())
    }
}
