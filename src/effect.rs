use derive_more::derive::{Display, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeColumnEffect {}

impl TryFrom<&str> for VolumeColumnEffect {
	type Error = InvalidVolumeColumnEffect;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		todo!()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The volume column effect {} is invalid.", self.0)]
pub struct InvalidVolumeColumnEffect(#[error(not(source))] pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanningColumnEffect {}

impl TryFrom<&str> for PanningColumnEffect {
	type Error = InvalidPanningColumnEffect;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		todo!()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
#[display("The panning column effect {} is invalid.", self.0)]
pub struct InvalidPanningColumnEffect(#[error(not(source))] pub String);
