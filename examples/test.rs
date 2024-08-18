use std::error::Error;

use xrns_rs::{notes_in_song, Song};

fn main() -> Result<(), Box<dyn Error>> {
	let song = Song::from_file("test.xrns")?;
	dbg!(notes_in_song(&song));
	Ok(())
}
