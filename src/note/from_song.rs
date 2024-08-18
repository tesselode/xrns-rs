use std::time::Duration;

use indexmap::IndexMap;

use crate::{Instrument, NoteCommand, Panning, Semitones, SequenceEntry, Song, Volume};

use super::Note;

pub fn notes_in_song(song: &Song) -> Vec<Note> {
	let mut notes = vec![];
	let bpm = song.global_song_data.beats_per_min;
	let lines_per_beat = song.global_song_data.lines_per_beat;
	let beat_duration = Duration::from_secs_f64(60.0 / bpm as f64);
	let line_duration = beat_duration / lines_per_beat;
	let mut current_time = Duration::ZERO;
	let mut in_progress_notes: IndexMap<NoteLocation, InProgressNote> = IndexMap::new();
	// for each pattern
	for SequenceEntry { pattern_index } in &song.pattern_sequence.sequence_entries {
		let pattern = &song.pattern_pool.patterns[*pattern_index];
		// for each line
		for line_index in 0..pattern.number_of_lines {
			// for each track
			for (track_index, pattern_track) in pattern.tracks.pattern_tracks.iter().enumerate() {
				let Some(line) = pattern_track.line(line_index) else {
					continue;
				};
				// for each note column
				for (note_column_index, note_column) in line.note_columns.iter().enumerate() {
					let Some(note_command) = note_column.note_command else {
						continue;
					};
					let note_location = NoteLocation {
						track_index,
						note_column_index,
					};
					let just_finished_note = in_progress_notes
						.swap_remove(&note_location)
						.map(|in_progress_note| in_progress_note.finish(current_time));
					notes.extend(just_finished_note);
					let (NoteCommand::On(pitch), Some(instrument)) =
						(note_command, note_column.instrument)
					else {
						continue;
					};
					in_progress_notes.insert(
						note_location,
						InProgressNote {
							pitch: pitch.semitones_above_c0().as_f32(),
							start_time: current_time,
							instrument,
							volume: note_column.volume_column.volume().as_f32(),
							panning: note_column.panning_column.panning().as_f32(),
						},
					);
				}
			}
			current_time += line_duration;
		}
	}
	// finish all notes that never got a note off
	notes.extend(
		in_progress_notes
			.drain(..)
			.map(|(_, in_progress_note)| in_progress_note.finish(current_time)),
	);
	notes
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct InProgressNote {
	pitch: Semitones<f32>,
	start_time: Duration,
	instrument: Instrument,
	volume: Volume<f32>,
	panning: Panning<f32>,
}

impl InProgressNote {
	fn finish(self, end_time: Duration) -> Note {
		Note {
			pitch: self.pitch,
			start_time: self.start_time,
			end_time,
			instrument: self.instrument,
			volume: self.volume,
			panning: self.panning,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NoteLocation {
	track_index: usize,
	note_column_index: usize,
}
