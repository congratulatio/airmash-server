//! Utility accessors for components
//! that are commonly used together.

mod clock;
pub(crate) mod fire_missiles;
mod isalive;
mod respawn;

pub use self::clock::ReadClock;
pub use self::fire_missiles::FireMissiles;
pub use self::isalive::IsAlive;
pub use self::respawn::PlayerRespawner;
