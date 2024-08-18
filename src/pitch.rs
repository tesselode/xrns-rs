use derive_more::derive::{
	Add, AddAssign, Display, Div, DivAssign, Error, FromStr, Mul, MulAssign, Neg, Sub, SubAssign,
};
use serde::Deserialize;

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Neg,
)]
pub struct Semitones<T>(pub T);

impl Semitones<u32> {
	pub fn as_f32(self) -> Semitones<f32> {
		Semitones(self.0 as f32)
	}
}

impl Semitones<f32> {
	pub fn as_u32(self) -> Semitones<u32> {
		Semitones(self.0 as u32)
	}
}

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

impl NoteName {
	pub fn semitones_above_c(self) -> Semitones<u32> {
		Semitones(match self {
			NoteName::C => 0,
			NoteName::D => 2,
			NoteName::E => 4,
			NoteName::F => 5,
			NoteName::G => 7,
			NoteName::A => 9,
			NoteName::B => 11,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub struct Pitch {
	pub note_name: NoteName,
	pub is_sharp: bool,
	pub octave: u32,
}

impl Pitch {
	pub const LOWEST: Self = Self {
		note_name: NoteName::C,
		is_sharp: false,
		octave: 0,
	};
	pub const HIGHEST: Self = Self {
		note_name: NoteName::B,
		is_sharp: false,
		octave: 9,
	};
	pub const SEMITONES_PER_OCTAVE: Semitones<u32> = Semitones(12);

	pub fn semitones_above_c0(self) -> Semitones<u32> {
		Self::SEMITONES_PER_OCTAVE * self.octave
			+ self.note_name.semitones_above_c()
			+ Semitones(if self.is_sharp { 1 } else { 0 })
	}
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
