use specs::*;

use crate::systems::handlers::packet::LoginHandler;
use crate::SystemInfo;

use crate::component::counter::PlayersGame;
use crate::component::event::*;
use crate::consts::NUM_PLAYERS;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

use std::sync::atomic::Ordering::Relaxed;

#[derive(Default)]
pub struct UpdatePlayersGame;

#[derive(SystemData)]
pub struct UpdatePlayersGameData<'a> {
	playersgame: Write<'a, PlayersGame>,
}

impl EventHandlerTypeProvider for UpdatePlayersGame {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for UpdatePlayersGame {
	type SystemData = UpdatePlayersGameData<'a>;

	fn on_event(&mut self, _: &PlayerJoin, data: &mut Self::SystemData) {
		data.playersgame.0 += 1;
		NUM_PLAYERS.fetch_add(1, Relaxed);
	}
}

impl SystemInfo for UpdatePlayersGame {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
