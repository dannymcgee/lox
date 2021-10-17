use std::{
	alloc::{self, Layout},
	fmt, mem, ptr,
};

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

impl<T> fmt::Debug for Stack<T>
where T: fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let begin = self.begin;
		let mut end = self.end;
		let mut first = true;

		write!(f, "[")?;
		while end != begin {
			if !first {
				write!(f, ", ")?;
			}
			first = false;

			end = unsafe { end.sub(1) };
			write!(f, "{:?}", unsafe { &*end })?;
		}
		write!(f, "]")
	}
}
