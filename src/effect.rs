use derive_more::derive::{Display, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
	Arpeggio { note_offsets: [u8; 2] },
	SlideUp { amount: u8 },
	SlideDown { amount: u8 },
	Glide { amount: u8 },
	Vibrato { speed: u8, depth: u8 },
	FadeIn { amount: u8 },
	FadeOut { amount: u8 },
	Tremolo { speed: u8, depth: u8 },
	Cut { volume_factor: u8, delay: u8 },
	SliceOrOffset(u8),
	Backwards(bool),
	EnvelopePosition(u8),
	Autopan { speed: u8, depth: u8 },
	ChannelVolume(u8),
	Phrase(u8),
	Delay { ticks: u8 },
	Maybe { probability: u8 },
	Retrigger { volume_factor: u8, ticks: u8 },
	PreMixerVolume(u8),
	PreMixerPanning(u8),
	PreMixerSurround(u8),
	Stop { effect: u8 },
	OutputRouting(u8),
	Tempo(u8),
	LinesPerBeat(u8),
	TicksPerLine(u8),
	GrooveEnabled(bool),
	BreakPattern { next_pattern_start_line: u8 },
	Pause { lines: u8 },
}

impl TryFrom<(&str, &str)> for Effect {
	type Error = InvalidEffect;

	fn try_from((kind, value): (&str, &str)) -> Result<Self, Self::Error> {
		let xx = u8::from_str_radix(value, 16).map_err(|_| InvalidEffect {
			kind: kind.to_string(),
			value: value.to_string(),
		})?;
		let x = xx >> 0x1;
		let y = xx & 0x1;
		Ok(match kind {
			"0A" => Self::Arpeggio {
				note_offsets: [x, y],
			},
			"0U" => Self::SlideUp { amount: xx },
			"0D" => Self::SlideDown { amount: xx },
			"0G" => Self::Glide { amount: xx },
			"0V" => Self::Vibrato { speed: x, depth: y },
			"0I" => Self::FadeIn { amount: xx },
			"0O" => Self::FadeOut { amount: xx },
			"0T" => Self::Tremolo { speed: x, depth: y },
			"0C" => Self::Cut {
				volume_factor: x,
				delay: y,
			},
			"0S" => Self::SliceOrOffset(xx),
			"0B" => Self::Backwards(match xx {
				0 => true,
				1 => false,
				_ => {
					return Err(InvalidEffect {
						kind: kind.to_string(),
						value: value.to_string(),
					})
				}
			}),
			"0E" => Self::EnvelopePosition(xx),
			"0N" => Self::Autopan { speed: x, depth: y },
			"0M" => Self::ChannelVolume(xx),
			"0Z" => Self::Phrase(xx),
			"0Q" => Self::Delay { ticks: xx },
			"0Y" => Self::Maybe { probability: xx },
			"0R" => Self::Retrigger {
				volume_factor: x,
				ticks: y,
			},
			"0L" => Self::PreMixerVolume(xx),
			"0P" => Self::PreMixerPanning(xx),
			"0W" => Self::PreMixerSurround(xx),
			"0X" => Self::Stop { effect: xx },
			"0J" => Self::OutputRouting(xx),
			"ZT" => Self::Tempo(xx),
			"ZL" => Self::LinesPerBeat(xx),
			"ZK" => Self::TicksPerLine(xx),
			"ZG" => Self::GrooveEnabled(match xx {
				0 => true,
				1 => false,
				_ => {
					return Err(InvalidEffect {
						kind: kind.to_string(),
						value: value.to_string(),
					})
				}
			}),
			"ZB" => Self::BreakPattern {
				next_pattern_start_line: xx,
			},
			"ZD" => Self::Pause { lines: xx },
			_ => {
				return Err(InvalidEffect {
					kind: kind.to_string(),
					value: value.to_string(),
				})
			}
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The effect {}{} is invalid.", self.kind, self.value)]
pub struct InvalidEffect {
	#[error(not(source))]
	kind: String,
	#[error(not(source))]
	value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeColumnEffect {
	FadeIn { speed: u8 },
	FadeOut { speed: u8 },
	SlideUp { semitones: u8 },
	SlideDown { semitones: u8 },
	Glide { semitones: u8 },
	Cut { ticks: u8 },
	Backwards(bool),
	Delay { ticks: u8 },
	Maybe { probability: u8 },
	Retrigger { ticks: u8 },
}

impl TryFrom<&str> for VolumeColumnEffect {
	type Error = InvalidVolumeColumnEffect;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let kind = &value[0..1];
		let value = u8::from_str_radix(&value[1..2], 16)
			.map_err(|_| InvalidVolumeColumnEffect(value.to_string()))?;
		Ok(match kind {
			"I" => Self::FadeIn { speed: value },
			"O" => Self::FadeOut { speed: value },
			"U" => Self::SlideUp { semitones: value },
			"D" => Self::SlideDown { semitones: value },
			"G" => Self::Glide { semitones: value },
			"C" => Self::Cut { ticks: value },
			"B" => Self::Backwards(match value {
				0 => true,
				1 => false,
				_ => return Err(InvalidVolumeColumnEffect(value.to_string())),
			}),
			"Q" => Self::Delay { ticks: value },
			"Y" => Self::Maybe { probability: value },
			"R" => Self::Retrigger { ticks: value },
			_ => return Err(InvalidVolumeColumnEffect(value.to_string())),
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The volume column effect {} is invalid.", self.0)]
pub struct InvalidVolumeColumnEffect(#[error(not(source))] pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanningColumnEffect {
	SlideLeft { speed: u8 },
	SlideRight { speed: u8 },
	SlideUp { semitones: u8 },
	SlideDown { semitones: u8 },
	Glide { semitones: u8 },
	Cut { ticks: u8 },
	Backwards(bool),
	Delay { ticks: u8 },
	Maybe { probability: u8 },
	Retrigger { ticks: u8 },
}

impl TryFrom<&str> for PanningColumnEffect {
	type Error = InvalidPanningColumnEffect;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let kind = &value[0..1];
		let value = u8::from_str_radix(&value[1..2], 16)
			.map_err(|_| InvalidPanningColumnEffect(value.to_string()))?;
		Ok(match kind {
			"J" => Self::SlideLeft { speed: value },
			"K" => Self::SlideRight { speed: value },
			"U" => Self::SlideUp { semitones: value },
			"D" => Self::SlideDown { semitones: value },
			"G" => Self::Glide { semitones: value },
			"C" => Self::Cut { ticks: value },
			"B" => Self::Backwards(match value {
				0 => true,
				1 => false,
				_ => return Err(InvalidPanningColumnEffect(value.to_string())),
			}),
			"Q" => Self::Delay { ticks: value },
			"Y" => Self::Maybe { probability: value },
			"R" => Self::Retrigger { ticks: value },
			_ => return Err(InvalidPanningColumnEffect(value.to_string())),
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The panning column effect {} is invalid.", self.0)]
pub struct InvalidPanningColumnEffect(#[error(not(source))] pub String);
