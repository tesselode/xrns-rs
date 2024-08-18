use derive_more::derive::{Display, Error};
use xrns_derive::Effect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Effect)]
pub enum VolumeColumnEffect {
	#[effect("Ix")]
	FadeIn { speed: u8 },
	#[effect("Ox")]
	FadeOut { speed: u8 },
	#[effect("Ux")]
	SlideUp { semitones: u8 },
	#[effect("Dx")]
	SlideDown { semitones: u8 },
	#[effect("Gx")]
	Glide { semitones: u8 },
	#[effect("Cx")]
	Cut { ticks: u8 },
	#[effect("Bx")]
	Backwards { backwards: bool },
	#[effect("Qx")]
	Delay { ticks: u8 },
	#[effect("Yx")]
	Maybe { probability: u8 },
	#[effect("Rx")]
	Retrigger { ticks: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanningColumnEffect {
	SlideLeft { speed: u8 },
	SlideRight { speed: u8 },
	SlideUp { semitones: u8 },
	SlideDown { semitones: u8 },
	Glide { semitones: u8 },
	Cut { ticks: u8 },
	Backwards { backwards: bool },
	Delay { ticks: u8 },
	Maybe { probability: u8 },
	Retrigger { ticks: u8 },
}

impl TryFrom<&str> for PanningColumnEffect {
	type Error = InvalidPanningColumnEffect;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let kind = &value[0..1];
		let value = value[1..2]
			.parse::<u8>()
			.map_err(|_| InvalidPanningColumnEffect(value.to_string()))?;
		Ok(match kind {
			"J" => Self::SlideLeft { speed: value },
			"K" => Self::SlideRight { speed: value },
			"U" => Self::SlideUp { semitones: value },
			"D" => Self::SlideDown { semitones: value },
			"G" => Self::Glide { semitones: value },
			"C" => Self::Cut { ticks: value },
			"B" => Self::Backwards {
				backwards: match value {
					0 => true,
					1 => false,
					_ => return Err(InvalidPanningColumnEffect(value.to_string())),
				},
			},
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
