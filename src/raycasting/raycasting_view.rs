pub mod raycasting_view {
    use crate::raycasting::ray::Ray;
    use ggez::context::Context;
    use ggez::graphics::{self, Canvas, Color};
    use ggez::mint::Point2;
    use ggez::GameResult;

    const LINE_THICKNESS: f32 = 1.0;

    pub fn draw(ctx: &mut Context, canvas: &mut Canvas, ray: &Ray) -> GameResult {
        draw_ray_line(ctx, canvas, ray);

        Ok(())
    }

    fn draw_ray_line(ctx: &mut Context, canvas: &mut Canvas, ray: &Ray) {
        let direction = create_ray_line(ctx, ray);
        for mesh in &direction {
            canvas.draw(mesh, graphics::DrawParam::new());
        }
    }

    fn create_ray_line(ctx: &mut Context, ray: &Ray) -> GameResult<graphics::Mesh> {
        let direction_line_end_x = ray.origin.x + ray.distance * ray.angle.cos();
        let direction_line_end_y = ray.origin.y + ray.distance * ray.angle.sin();

        let line_points = vec![
            Point2 {
                x: ray.origin.x,
                y: ray.origin.y,
            },
            Point2 {
                x: direction_line_end_x,
                y: direction_line_end_y,
            },
        ];
        let mesh = &mut graphics::MeshBuilder::new();
        mesh.line(&line_points, LINE_THICKNESS, Color::GREEN)?;

        Ok(graphics::Mesh::from_data(ctx, mesh.build()))
    }
}
