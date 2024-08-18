use std::{fs::File, io::Read, num::ParseIntError, path::Path};

use derive_more::derive::{Display, Error, From};
use indexmap::IndexMap;
use serde::Deserialize;
use zip::{result::ZipError, ZipArchive};

use crate::{
	Effect, InvalidEffect, InvalidPanningColumnEffect, InvalidPitch, InvalidVolumeColumnEffect,
	PanningColumnEffect, Pitch, VolumeColumnEffect,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Song {
	pub global_song_data: GlobalSongData,
	pub pattern_pool: PatternPool,
}

impl Song {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Song, Error> {
		let path = path.as_ref();
		let file = File::open(path)?;
		let mut archive = ZipArchive::new(file)?;
		let mut song_xml_file = archive.by_name("Song.xml")?;
		let mut song_xml_string = String::new();
		song_xml_file.read_to_string(&mut song_xml_string)?;
		let song = quick_xml::de::from_str(&song_xml_string)?;
		Ok(song)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GlobalSongData {
	pub beats_per_min: u32,
	pub lines_per_beat: u32,
	pub ticks_per_line: u32,
	pub signature_numerator: u32,
	pub signature_denominator: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatternPool {
	#[serde(deserialize_with = "unwrap_pattern_list")]
	pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pattern {
	pub number_of_lines: u32,
	pub tracks: Tracks,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tracks {
	#[serde(rename = "PatternTrack")]
	pub pattern_tracks: Vec<PatternTrack>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "raw::PatternTrack")]
pub struct PatternTrack {
	pub lines: IndexMap<u32, Line>,
}

impl PatternTrack {
	pub fn line(&self, index: u32) -> Option<&Line> {
		self.lines.get(&index)
	}

	pub fn line_mut(&mut self, index: u32) -> Option<&mut Line> {
		self.lines.get_mut(&index)
	}
}

impl From<raw::PatternTrack> for PatternTrack {
	fn from(raw::PatternTrack { lines }: raw::PatternTrack) -> Self {
		Self {
			lines: lines
				.iter()
				.map(|raw_line| {
					(
						raw_line.index,
						Line {
							note_columns: raw_line.note_columns.clone(),
							effect_columns: raw_line.effect_columns.clone(),
						},
					)
				})
				.collect(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line {
	pub note_columns: Vec<NoteColumn>,
	pub effect_columns: Vec<EffectColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "raw::NoteColumn")]
pub struct NoteColumn {
	pub note_command: NoteCommand,
	pub instrument: Option<Instrument>,
	pub volume: Volume,
	pub panning: Panning,
	pub delay: Delay,
	pub effect: Option<Effect>,
}

impl TryFrom<raw::NoteColumn> for NoteColumn {
	type Error = InvalidEffect;

	fn try_from(
		raw::NoteColumn {
			note_command,
			instrument,
			volume,
			panning,
			delay,
			effect_number,
			effect_value,
		}: raw::NoteColumn,
	) -> Result<Self, Self::Error> {
		let effect = if let (Some(effect_number), Some(effect_value)) =
			(effect_number.as_ref(), effect_value.as_ref())
		{
			Some(Effect::try_from((
				effect_number.as_ref(),
				effect_value.as_ref(),
			))?)
		} else {
			None
		};
		Ok(Self {
			note_command,
			instrument,
			volume,
			panning,
			delay,
			effect,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "raw::EffectColumn")]
pub struct EffectColumn(pub Effect);

impl TryFrom<raw::EffectColumn> for EffectColumn {
	type Error = InvalidEffect;

	fn try_from(
		raw::EffectColumn { number, value }: raw::EffectColumn,
	) -> Result<Self, Self::Error> {
		Effect::try_from((number.as_ref(), value.as_ref())).map(Self)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub enum NoteCommand {
	On(Pitch),
	Off,
}

impl TryFrom<&str> for NoteCommand {
	type Error = InvalidPitch;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value == "OFF" {
			return Ok(Self::Off);
		}
		let pitch = Pitch::try_from(value)?;
		Ok(NoteCommand::On(pitch))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub struct Instrument(pub u8);

impl TryFrom<&str> for Instrument {
	type Error = ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(Self(u8::from_str_radix(value, 16)?))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub enum Volume {
	Volume(u8),
	Effect(VolumeColumnEffect),
}

impl TryFrom<&str> for Volume {
	type Error = ParseVolumeError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let volume = u8::from_str_radix(value, 16)?;
		if volume <= 0x80 {
			Ok(Self::Volume(volume))
		} else {
			Ok(Self::Effect(VolumeColumnEffect::try_from(value)?))
		}
	}
}

impl Default for Volume {
	fn default() -> Self {
		Self::Volume(0x80)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Display, From, Error)]
pub enum ParseVolumeError {
	ParseIntError(ParseIntError),
	InvalidEffect(
		#[from(ignore)]
		#[error(not(source))]
		String,
	),
}

impl From<InvalidVolumeColumnEffect> for ParseVolumeError {
	fn from(InvalidVolumeColumnEffect(s): InvalidVolumeColumnEffect) -> Self {
		Self::InvalidEffect(s)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "&str")]
pub enum Panning {
	Panning(u8),
	Effect(PanningColumnEffect),
}

impl TryFrom<&str> for Panning {
	type Error = ParsePanningError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let panning = u8::from_str_radix(value, 16)?;
		if panning <= 0x80 {
			Ok(Self::Panning(panning))
		} else {
			Ok(Self::Effect(PanningColumnEffect::try_from(value)?))
		}
	}
}

impl Default for Panning {
	fn default() -> Self {
		Self::Panning(0x40)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Display, From, Error)]
pub enum ParsePanningError {
	ParseIntError(ParseIntError),
	InvalidEffect(
		#[from(ignore)]
		#[error(not(source))]
		String,
	),
}

impl From<InvalidPanningColumnEffect> for ParsePanningError {
	fn from(InvalidPanningColumnEffect(s): InvalidPanningColumnEffect) -> Self {
		Self::InvalidEffect(s)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Default)]
#[serde(try_from = "&str")]
pub struct Delay(pub u8);

impl TryFrom<&str> for Delay {
	type Error = ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(Self(u8::from_str_radix(value, 16)?))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatternSequence {
	#[serde(deserialize_with = "unwrap_sequence_entry_list")]
	pub sequence_entries: Vec<SequenceEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SequenceEntry {
	pub pattern: usize,
}

#[derive(Debug, From, Error, Display)]
pub enum Error {
	IoError(std::io::Error),
	ZipError(ZipError),
	DeError(quick_xml::DeError),
}

fn unwrap_pattern_list<'de, D>(deserializer: D) -> Result<Vec<Pattern>, D::Error>
where
	D: serde::de::Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct List {
		#[serde(default)]
		pattern: Vec<Pattern>,
	}
	Ok(List::deserialize(deserializer)?.pattern)
}

fn unwrap_note_column_list<'de, D>(deserializer: D) -> Result<Vec<NoteColumn>, D::Error>
where
	D: serde::de::Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct List {
		#[serde(default)]
		note_column: Vec<NoteColumn>,
	}
	Ok(List::deserialize(deserializer)?.note_column)
}

fn unwrap_effect_column_list<'de, D>(deserializer: D) -> Result<Vec<EffectColumn>, D::Error>
where
	D: serde::de::Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct List {
		#[serde(default)]
		effect_column: Vec<EffectColumn>,
	}
	Ok(List::deserialize(deserializer)?.effect_column)
}

fn unwrap_sequence_entry_list<'de, D>(deserializer: D) -> Result<Vec<SequenceEntry>, D::Error>
where
	D: serde::de::Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	struct List {
		#[serde(default)]
		sequence_entry: Vec<SequenceEntry>,
	}
	Ok(List::deserialize(deserializer)?.sequence_entry)
}

mod raw {
	use serde::Deserialize;

	use super::{
		unwrap_effect_column_list, unwrap_note_column_list, Delay, Instrument, NoteCommand,
		Panning, Volume,
	};

	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	pub struct PatternTrack {
		#[serde(deserialize_with = "unwrap_line_list")]
		pub lines: Vec<Line>,
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "PascalCase")]
	pub struct Line {
		#[serde(rename = "@index")]
		pub index: u32,
		#[serde(deserialize_with = "unwrap_note_column_list")]
		pub note_columns: Vec<super::NoteColumn>,
		#[serde(deserialize_with = "unwrap_effect_column_list")]
		#[serde(default)]
		pub effect_columns: Vec<super::EffectColumn>,
	}

	#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
	#[serde(rename_all = "PascalCase")]
	pub struct NoteColumn {
		#[serde(rename = "Note")]
		pub note_command: NoteCommand,
		pub instrument: Option<Instrument>,
		#[serde(default)]
		pub volume: Volume,
		#[serde(default)]
		pub panning: Panning,
		#[serde(default)]
		pub delay: Delay,
		pub effect_number: Option<String>,
		pub effect_value: Option<String>,
	}

	#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
	#[serde(rename_all = "PascalCase")]
	pub struct EffectColumn {
		pub number: String,
		pub value: String,
	}

	fn unwrap_line_list<'de, D>(deserializer: D) -> Result<Vec<Line>, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(rename_all = "PascalCase")]
		struct List {
			#[serde(default)]
			line: Vec<Line>,
		}
		Ok(List::deserialize(deserializer)?.line)
	}
}
