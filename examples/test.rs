use std::error::Error;

use xrns_rs::Song;

fn main() -> Result<(), Box<dyn Error>> {
	let song = Song::from_file("test.xrns")?;
	dbg!(song);
	Ok(())
}
