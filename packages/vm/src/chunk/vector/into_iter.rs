use std::{mem, ptr};

use super::Vector;

pub struct IntoIter<T> {
	_inner: Vector<T>,
	start: *const T,
	end: *const T,
}

impl<T> IntoIterator for Vector<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> IntoIter<T> {
		unsafe {
			let chunk = ptr::read(&self);
			let len = self.len;

			mem::forget(self);

			IntoIter {
				start: chunk.ptr(),
				end: chunk.ptr().add(len),
				_inner: chunk,
			}
		}
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.start == self.end {
			None
		} else {
			unsafe {
				let result = ptr::read(self.start);
				self.start = self.start.offset(1);

				Some(result)
			}
		}
	}
}

impl<T> Drop for IntoIter<T> {
	fn drop(&mut self) {
		for _ in &mut *self {}
	}
}
