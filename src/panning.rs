use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
pub struct Panning<T>(pub T);

impl Panning<u8> {
	pub const LEFT: Self = Self(0x0);
	pub const CENTER: Self = Self(0x40);
	pub const RIGHT: Self = Self(0x80);

	pub fn as_f32(self) -> Panning<f32> {
		Panning(self.0 as f32 / Self::RIGHT.0 as f32)
	}
}

impl Default for Panning<u8> {
	fn default() -> Self {
		Self::CENTER
	}
}

impl Panning<f32> {
	pub const LEFT: Self = Self(0.0);
	pub const CENTER: Self = Self(0.5);
	pub const RIGHT: Self = Self(1.0);

	pub fn as_u8(self) -> Panning<u8> {
		Panning((self.0 * Panning::<u8>::RIGHT.0 as f32).round() as u8)
	}
}

impl Default for Panning<f32> {
	fn default() -> Self {
		Self::CENTER
	}
}
