use std::{
	ops::{Deref, DerefMut},
	slice,
};

use super::Vector;

impl<T> Deref for Vector<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		unsafe { slice::from_raw_parts(self.ptr(), self.len) }
	}
}

impl<T> DerefMut for Vector<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) }
	}
}
