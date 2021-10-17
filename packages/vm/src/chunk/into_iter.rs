use std::{mem, ptr};

use crate::{
	value::Value,
	vector::{IntoIter, Vector},
};

use super::{lines::Lines, Chunk};

pub struct Consumable {
	data: IntoIter<u8>,
	offset: usize,
	constants: Vector<Value>,
	lines: Lines,
}

impl IntoIterator for Chunk {
	type Item = (usize, u8);
	type IntoIter = Consumable;

	fn into_iter(self) -> Self::IntoIter {
		unsafe {
			let chunk = ptr::read(&self);

			let data = chunk.data.into_iter();
			let constants = chunk.constants;
			let lines = chunk.lines;

			mem::forget(self);

			Consumable {
				data,
				offset: 0,
				constants,
				lines,
			}
		}
	}
}

impl Consumable {
	pub fn read_const(&self, handle: usize) -> Option<Value> {
		if handle >= self.constants.len() {
			None
		} else {
			Some(self.constants[handle])
		}
	}

	pub fn lines(&self) -> &Lines {
		&self.lines
	}
}

impl Iterator for Consumable {
	type Item = (usize, u8);

	fn next(&mut self) -> Option<Self::Item> {
		let byte = self.data.next()?;
		let offset = self.offset;
		self.offset += 1;

		Some((offset, byte))
	}
}

impl Drop for Consumable {
	fn drop(&mut self) {
		for _ in &mut *self {}
	}
}
