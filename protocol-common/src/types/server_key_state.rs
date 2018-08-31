/// All possible "keys" that a player can have activated.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct ServerKeyState {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub boost: bool,
	pub strafe: bool,
	pub stealth: bool,
	pub flagspeed: bool,
}
