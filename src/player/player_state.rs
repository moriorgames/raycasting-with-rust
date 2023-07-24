use crate::map::map::Map;
use crate::player::{player::Player, player_controls::player_controls};
use ggez::Context;

pub struct PlayerState {
    pub player: Player,
    map: Map,
}

impl PlayerState {
    pub fn new(map: Map) -> Self {
        PlayerState {
            player: Player::new(Player::DEFAULT_X, Player::DEFAULT_Y),
            map,
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) {
        player_controls::handle_input(ctx, &mut self.player, &self.map);
    }
}
