use std::sync::LazyLock;

use derive_more::derive::{Display, Error, FromStr};
use regex_lite::Regex;
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

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		static PARSE_PITCH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
			Regex::new(r"([ABCDEFG])([-#])(\d)").expect("error compiling parse pitch regex")
		});
		let captures = PARSE_PITCH_REGEX
			.captures(value)
			.ok_or_else(|| InvalidPitch(value.to_string()))?;
		let note_name = captures[1].parse().unwrap();
		let is_sharp = match &captures[2] {
			"#" => true,
			"-" => false,
			_ => unreachable!(),
		};
		let octave = captures[3].parse().unwrap();
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
