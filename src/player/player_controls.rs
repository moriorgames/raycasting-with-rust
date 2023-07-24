pub mod player_controls {
    use crate::map::map::Map;
    use crate::player::player::Player;
    use ggez::{input::keyboard::KeyCode, Context};

    const SPEED: f32 = 0.9;
    const ROTATION: f32 = 0.05;

    pub fn handle_input(ctx: &mut Context, player: &mut Player, map: &Map) {
        let pressed_keys = ctx.keyboard.pressed_keys();

        if pressed_keys.contains(&KeyCode::Right) {
            player.rotate_right(ROTATION);
        }

        if pressed_keys.contains(&KeyCode::Left) {
            player.rotate_left(ROTATION);
        }

        if pressed_keys.contains(&KeyCode::Up) {
            player.move_forward(SPEED, map);
        }

        if pressed_keys.contains(&KeyCode::Down) {
            player.move_backward(SPEED, map);
        }

        if pressed_keys.contains(&KeyCode::Escape) {
            ctx.request_quit();
        }
    }
}
