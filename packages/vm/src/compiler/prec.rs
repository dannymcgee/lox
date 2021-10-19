use std::ops;

#[repr(u8)]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Prec {
	None       = 0x00,
	Assignment = 0x01,
	Or         = 0x02,
	And        = 0x03,
	Equality   = 0x04,
	Comparison = 0x05,
	Term       = 0x06,
	Factor     = 0x07,
	Unary      = 0x08,
	Call       = 0x09,
	Primary    = 0x0A,
}

impl From<u8> for Prec {
	fn from(value: u8) -> Self {
		match value {
			0x00 => Self::None,
			0x01 => Self::Assignment,
			0x02 => Self::Or,
			0x03 => Self::And,
			0x04 => Self::Equality,
			0x05 => Self::Comparison,
			0x06 => Self::Term,
			0x07 => Self::Factor,
			0x08 => Self::Unary,
			0x09 => Self::Call,
			_ => Self::Primary,
		}
	}
}

impl ops::Add<u8> for Prec {
	type Output = Self;

	fn add(self, rhs: u8) -> Self::Output {
		(self as u8 + rhs).into()
	}
}
