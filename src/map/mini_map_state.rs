use crate::map::{map::Map, mini_map_view::mini_map_view};
use crate::player::player::PlayerTransformation;

pub struct MiniMapState {
    map: Map,
    player_transformation: PlayerTransformation,
}

impl MiniMapState {
    pub fn new(map: Map) -> Self {
        MiniMapState {
            map,
            player_transformation: PlayerTransformation::default(),
        }
    }

    pub fn update(&mut self, player_transformation: PlayerTransformation) {
        self.player_transformation = player_transformation;
    }

    pub fn draw(&mut self, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        mini_map_view::draw(canvas, &self.map, self.player_transformation.clone())
    }
}
