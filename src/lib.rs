use std::{fs::File, io::Read, path::Path};

use derive_more::derive::{Display, Error, From};
use serde::{Deserialize, Deserializer};
use zip::{result::ZipError, ZipArchive};

pub fn load(path: impl AsRef<Path>) -> Result<Song, Error> {
	let path = path.as_ref();
	let file = File::open(path)?;
	let mut archive = ZipArchive::new(file)?;
	let mut song_xml_file = archive.by_name("Song.xml")?;
	let mut song_xml_string = String::new();
	song_xml_file.read_to_string(&mut song_xml_string)?;
	let song = quick_xml::de::from_str(&song_xml_string)?;
	Ok(song)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Song {
	pub global_song_data: GlobalSongData,
	pub pattern_pool: PatternPool,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatternPool {
	#[serde(deserialize_with = "unwrap_pattern_list")]
	pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pattern {
	pub number_of_lines: u32,
	pub tracks: Tracks,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tracks {
	#[serde(rename = "PatternTrack")]
	pub pattern_tracks: Vec<PatternTrack>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatternTrack {
	#[serde(deserialize_with = "unwrap_line_list")]
	pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
	#[serde(rename = "@index")]
	pub index: u32,
	#[serde(deserialize_with = "unwrap_note_column_list")]
	pub note_columns: Vec<NoteColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoteColumn {
	pub note: String,
	pub instrument: Option<String>,
	pub volume: Option<String>,
	pub panning: Option<String>,
	pub delay: Option<String>,
	pub effect_number: Option<String>,
	pub effect_value: Option<String>,
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

macro_rules! unwrap_list_fns {
    ($($element_name:ident),*$(,)?) => {
		$(
			paste::paste! {
				fn [<unwrap_ $element_name:snake _list>]<'de, D>(deserializer: D) -> Result<Vec<$element_name>, D::Error>
				where
					D: Deserializer<'de>,
				{
					#[derive(Deserialize)]
					#[serde(rename_all = "PascalCase")]
					struct List {
						#[serde(default)]
						[<$element_name:snake>]: Vec<$element_name>,
					}
					Ok(List::deserialize(deserializer)?.[<$element_name:snake>])
				}
			}
		)*
	};
}

unwrap_list_fns!(Pattern, Line, NoteColumn, SequenceEntry);
