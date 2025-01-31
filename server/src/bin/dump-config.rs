use std::error::Error;
use std::fs::File;
use std::io::Write;

use airmash_server::types::Config;

fn main() -> Result<(), Box<dyn Error>> {
	let config = Config::default();
	let json = serde_json::to_string_pretty(&config)?;

	let mut file = File::create("default-config.json")?;

	writeln!(file, "{}", json)?;

	Ok(())
}
