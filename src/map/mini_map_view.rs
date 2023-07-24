pub mod mini_map_view {
    use crate::map::map::Map;
    use crate::player::player::PlayerTransformation;
    use ggez::graphics::{self, Canvas, Color};
    use ggez::GameResult;

    const PLAYER_SIZE: f32 = 6.0;
    const HEAD_SIZE: f32 = 3.0;
    const HEAD_DISTANCE: f32 = 7.0;
    const BORDER_SIZE: f32 = 1.0;

    pub fn draw(
        canvas: &mut Canvas,
        map: &Map,
        player_transformation: PlayerTransformation,
    ) -> GameResult {
        for (y, row) in map.data().iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let inner_rect = graphics::Rect::new_i32(
                    ((x as f32 * Map::SQUARE_SIZE) + BORDER_SIZE) as i32,
                    ((y as f32 * Map::SQUARE_SIZE) + BORDER_SIZE) as i32,
                    (Map::SQUARE_SIZE - 2.0 * BORDER_SIZE) as i32,
                    (Map::SQUARE_SIZE - 2.0 * BORDER_SIZE) as i32,
                );

                let outer_rect = graphics::Rect::new_i32(
                    (x as f32 * Map::SQUARE_SIZE) as i32,
                    (y as f32 * Map::SQUARE_SIZE) as i32,
                    Map::SQUARE_SIZE as i32,
                    Map::SQUARE_SIZE as i32,
                );

                let color = if cell == 1 {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                let border_color = Color::new(0.4, 0.4, 0.4, 1.0);

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest(outer_rect.point())
                        .scale(outer_rect.size())
                        .color(border_color),
                );

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest(inner_rect.point())
                        .scale(inner_rect.size())
                        .color(color),
                );
            }
        }

        draw_player(canvas, player_transformation);

        Ok(())
    }

    fn draw_player(canvas: &mut Canvas, player_transformation: PlayerTransformation) {
        let rect = graphics::Rect::new(
            player_transformation.position.x - PLAYER_SIZE / 2.0,
            player_transformation.position.y - PLAYER_SIZE / 2.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::YELLOW),
        );

        let rect = graphics::Rect::new(
            player_transformation.position.x + HEAD_DISTANCE * player_transformation.angle.cos(),
            player_transformation.position.y + HEAD_DISTANCE * player_transformation.angle.sin(),
            HEAD_SIZE,
            HEAD_SIZE,
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::YELLOW),
        );
    }
}
