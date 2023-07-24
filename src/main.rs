mod debug;
mod engine3d;
mod map;
mod player;
mod raycasting;
mod shared_kernel;

use debug::debug_view;
use engine3d::engine3d_state::Engine3dState;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{ContextBuilder, GameResult};
use map::{map::Map, mini_map_state::MiniMapState};
use player::player_state::PlayerState;
use raycasting::raycasting_state::RaycastingState;

const DEBUG_VIEW: bool = false;
pub mod constants {
    pub const WINDOW_WIDTH: f32 = 1024.0;
    pub const WINDOW_HEIGHT: f32 = 510.0;
    pub const MAX_FPS: u32 = 60;
}

struct GameState {
    player_state: PlayerState,
    mini_map_state: MiniMapState,
    raycasting_state: RaycastingState,
    engine3d_state: Engine3dState,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let map = Map::new();
        let player_state = PlayerState::new(map.clone());
        let mini_map_state = MiniMapState::new(map.clone());
        let raycasting_state = RaycastingState::new(map.clone());
        let engine3d_state = Engine3dState::new();

        Ok(GameState {
            player_state,
            mini_map_state,
            raycasting_state,
            engine3d_state,
        })
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        while ctx.time.check_update_time(constants::MAX_FPS) {
            self.player_state.handle_input(ctx);
            let player_transformation = self.player_state.player.player_transformation();
            let rays = self.raycasting_state.update(player_transformation.clone());
            self.engine3d_state
                .update(rays, player_transformation.clone());
            self.mini_map_state.update(player_transformation.clone());
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let gray = Color::new(0.4, 0.4, 0.4, 1.0);
        let mut canvas = graphics::Canvas::from_frame(ctx, gray);

        self.engine3d_state.draw(ctx, &mut canvas)?;
        if DEBUG_VIEW {
            self.mini_map_state.draw(&mut canvas)?;
            self.raycasting_state.draw(ctx, &mut canvas)?;
        }

        let player_transformation = self.player_state.player.player_transformation();
        debug_view::debug_view::draw(ctx, &mut canvas, player_transformation.clone());

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("raycasting_game", "MoriorGames")
        .window_setup(ggez::conf::WindowSetup::default().title("Raycasting Game"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT),
        )
        .build()?;
    let game_state = GameState::new()?;

    event::run(ctx, event_loop, game_state)
}
