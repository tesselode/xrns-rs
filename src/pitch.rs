use derive_more::derive::{Display, Error, FromStr};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromStr)]
pub enum NoteName {
	C,
	D,
	E,
	F,
	G,
	A,
	B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub struct Pitch {
	pub note_name: NoteName,
	pub is_sharp: bool,
	pub octave: u8,
}

impl TryFrom<&str> for Pitch {
	type Error = InvalidPitch;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let note_name = s[0..1].parse().map_err(|_| InvalidPitch(s.to_string()))?;
		let is_sharp = match &s[1..2] {
			"#" => true,
			"-" => false,
			_ => return Err(InvalidPitch(s.to_string())),
		};
		let octave = s[2..3].parse().map_err(|_| InvalidPitch(s.to_string()))?;
		Ok(Self {
			note_name,
			is_sharp,
			octave,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The pitch {} is invalid.", self.0)]
pub struct InvalidPitch(#[error(not(source))] pub String);
