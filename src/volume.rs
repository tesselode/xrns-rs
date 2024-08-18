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
pub struct Volume<T>(pub T);

impl Volume<u8> {
	pub const ZERO: Self = Self(0x0);
	pub const MAX: Self = Self(0x80);

	pub fn as_f32(self) -> Volume<f32> {
		Volume(self.0 as f32 / Self::MAX.0 as f32)
	}
}

impl Default for Volume<u8> {
	fn default() -> Self {
		Self::MAX
	}
}

impl Volume<f32> {
	pub const ZERO: Self = Self(0.0);
	pub const MAX: Self = Self(1.0);

	pub fn as_u8(self) -> Volume<u8> {
		Volume((self.0 * Volume::<u8>::MAX.0 as f32).round() as u8)
	}
}

impl Default for Volume<f32> {
	fn default() -> Self {
		Self::MAX
	}
}
