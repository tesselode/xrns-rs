mod from_song;

pub use from_song::*;

use std::time::Duration;

use crate::{volume::Volume, Instrument, Panning, Semitones};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
	pub pitch: Semitones<f32>,
	pub start_time: Duration,
	pub end_time: Duration,
	pub instrument: Instrument,
	pub volume: Volume<f32>,
	pub panning: Panning<f32>,
}

impl Note {
	pub fn duration(self) -> Duration {
		self.end_time - self.start_time
	}
}
