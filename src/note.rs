use std::time::Duration;

use crate::{volume::Volume, Instrument, Panning, Semitones};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
	pub pitch: Semitones<f32>,
	pub start_time: Duration,
	pub duration: Duration,
	pub instrument: Instrument,
	pub volume: Volume<f32>,
	pub panning: Panning<f32>,
}

impl Note {
	pub fn end_time(self) -> Duration {
		self.start_time + self.duration
	}
}
