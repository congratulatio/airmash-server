use crate::types::*;
use shrev::*;
use specs::*;

use crate::dispatch::SystemInfo;
use crate::types::event::ConnectionOpen;

use crate::systems::PacketHandler;

pub struct OnOpenHandler {
	reader: Option<ReaderId<ConnectionOpen>>,
}

impl OnOpenHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for OnOpenHandler {
	type SystemData = (
		Read<'a, EventChannel<ConnectionOpen>>,
		Write<'a, Connections>,
	);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<ConnectionOpen>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (channel, mut connections): Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in channel.read(reader) {
				connections.add(evt.conn, evt.sink.clone(), evt.addr, evt.origin.clone());
			}
		}
	}
}

impl SystemInfo for OnOpenHandler {
	type Dependencies = PacketHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		module_path!()
	}
}
