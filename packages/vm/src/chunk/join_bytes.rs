use std::{iter::Enumerate, slice::Iter};

use crate::vector::IntoIter;

pub trait JoinBytes {
	fn join_bytes(&mut self, count: usize) -> Option<usize>;
}

macro_rules! impl_join_bytes {
	(by value : $target:ty) => {
		impl JoinBytes for $target {
			fn join_bytes(&mut self, count: usize) -> Option<usize> {
				match count {
					1 => {
						let idx = self.next()?;
						Some(idx as usize)
					}
					2 => {
						let a = self.next()?;
						let b = self.next()?;
						Some(u16::from_be_bytes([a, b]) as usize)
					}
					3 => {
						let b = self.next()?;
						let c = self.next()?;
						let d = self.next()?;
						Some(u32::from_be_bytes([0, b, c, d]) as usize)
					}
					_ => None,
				}
			}
		}
	};

	(by ref $(<$life:lifetime>)?: $target:ty) => {
		impl$(<$life>)? JoinBytes for $target {
			fn join_bytes(&mut self, count: usize) -> Option<usize> {
				match count {
					1 => {
						let idx = *self.next()?;
						Some(idx as usize)
					}
					2 => {
						let a = *self.next()?;
						let b = *self.next()?;
						Some(u16::from_be_bytes([a, b]) as usize)
					}
					3 => {
						let b = *self.next()?;
						let c = *self.next()?;
						let d = *self.next()?;
						Some(u32::from_be_bytes([0, b, c, d]) as usize)
					}
					_ => None,
				}
			}
		}
	};

	(enumerated ref $(<$life:lifetime>)?: $target:ty) => {
		impl$(<$life>)? JoinBytes for $target {
			fn join_bytes(&mut self, count: usize) -> Option<usize> {
				match count {
					1 => {
						let (_, idx) = self.next()?;
						Some(*idx as usize)
					}
					2 => {
						let (_, a) = self.next()?;
						let (_, b) = self.next()?;
						Some(u16::from_be_bytes([*a, *b]) as usize)
					}
					3 => {
						let (_, b) = self.next()?;
						let (_, c) = self.next()?;
						let (_, d) = self.next()?;
						Some(u32::from_be_bytes([0, *b, *c, *d]) as usize)
					}
					_ => None,
				}
			}
		}
	};

	(enumerated value : $target:ty) => {
		impl JoinBytes for $target {
			fn join_bytes(&mut self, count: usize) -> Option<usize> {
				match count {
					1 => {
						let (_, idx) = self.next()?;
						Some(idx as usize)
					}
					2 => {
						let (_, a) = self.next()?;
						let (_, b) = self.next()?;
						Some(u16::from_be_bytes([a, b]) as usize)
					}
					3 => {
						let (_, b) = self.next()?;
						let (_, c) = self.next()?;
						let (_, d) = self.next()?;
						Some(u32::from_be_bytes([0, b, c, d]) as usize)
					}
					_ => None,
				}
			}
		}
	}
}

impl_join_bytes!(by ref <'a> : Iter<'a, u8>);
impl_join_bytes!(by value : IntoIter<u8>);
impl_join_bytes!(enumerated ref <'a> : Enumerate<Iter<'a, u8>>);
impl_join_bytes!(enumerated value : super::Consumable);
