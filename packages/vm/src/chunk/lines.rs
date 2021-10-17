use crate::vector::{vector, Vector};

#[derive(Clone, Copy)]
pub struct LineStart {
	pub line: usize,
	pub offset: usize,
}

pub struct Lines {
	inner: Vector<LineStart>,
}

impl Lines {
	pub fn new() -> Self {
		Self { inner: vector![] }
	}

	pub fn add_byte(&mut self, line: usize, offset: usize) {
		let tail = self.tail();
		if tail.is_none() || tail.unwrap().line != line {
			self.inner.push(LineStart { line, offset });
		}
	}

	pub fn find_line(&self, offset: usize) -> usize {
		// Binary search for the last LineStart whose offset <= the given param
		let mut start = 0;
		let mut end = self.last_idx();

		loop {
			let mid = (start + end) / 2;
			let line_start = &self.inner[mid];

			if offset < line_start.offset {
				// Needle is in first half of the haystack
				end = mid - 1;
			} else if mid == self.last_idx() || offset < self.inner[mid + 1].offset {
				// Found it
				break line_start.line;
			} else {
				// Needle is in second half of the haystack
				start = mid + 1;
			}
		}
	}

	fn last_idx(&self) -> usize {
		if self.inner.is_empty() {
			0
		} else {
			self.inner.len() - 1
		}
	}

	fn tail(&self) -> Option<&LineStart> {
		if self.inner.is_empty() {
			None
		} else {
			Some(&self.inner[self.last_idx()])
		}
	}
}
