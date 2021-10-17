use std::{
	alloc::{self, Layout},
	fmt, mem, ptr,
};

use crate::value::Value;

#[cfg(test)]
mod tests;

pub struct Stack<T> {
	begin: *mut T,
	end: *mut T,
	size: usize,
}

impl<T> Stack<T> {
	pub const MAX: usize = 256;

	pub fn new() -> Self {
		assert!(mem::size_of::<T>() != 0);
		let layout = Self::layout();
		let ptr = unsafe { alloc::alloc(layout) };

		Stack {
			begin: ptr as _,
			end: ptr as _,
			size: 0,
		}
	}

	fn layout() -> Layout {
		Layout::array::<T>(Self::MAX).unwrap()
	}

	pub fn push(&mut self, elem: T) {
		if self.size == Self::MAX {
			panic!("Stack overflow");
		}
		unsafe {
			ptr::write(self.end, elem);
			self.end = self.end.add(1)
		}
		self.size += 1;
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.is_empty() {
			None
		} else {
			self.size -= 1;
			Some(unsafe {
				let ptr = self.end.sub(1);
				self.end = ptr;

				ptr::read(ptr)
			})
		}
	}

	pub fn empty(&mut self) {
		while self.pop().is_some() {}
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	#[allow(dead_code)]
	pub fn size(&self) -> usize {
		self.size
	}
}

impl<T> Drop for Stack<T> {
	fn drop(&mut self) {
		self.empty();
		unsafe {
			alloc::dealloc(self.begin as _, Self::layout());
		}
	}
}

pub trait FmtStackElement {
	fn fmt_value(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.fmt_to_string())
	}

	fn fmt_to_string(&self) -> String;
}

impl<T> fmt::Debug for Stack<T>
where T: FmtStackElement
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut ptr = self.begin;
		let end = self.end;

		write!(f, "[")?;
		while ptr != end {
			if ptr != self.begin {
				write!(f, ", ")?;
			}

			let value = unsafe { &*ptr };
			value.fmt_value(f)?;

			ptr = unsafe { ptr.add(1) };
		}
		write!(f, "]")
	}
}

impl FmtStackElement for Value {
	fn fmt_to_string(&self) -> String {
		let prec = if self.abs() % 1. < f64::EPSILON {
			0
		} else if self.abs() * 10. % 1. < f64::EPSILON {
			1
		} else if self.abs() * 100. % 1. < f64::EPSILON {
			2
		} else {
			3
		};

		format!("{1:.0$}", prec, self)
	}
}

impl FmtStackElement for &str {
	fn fmt_to_string(&self) -> String {
		format!(r#""{}""#, self)
	}
}
