use std::{
	alloc::{self, Layout},
	mem,
	ptr::{self, NonNull},
};

mod debug;
mod into_iter;
mod iter;

pub use into_iter::IntoIter;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! vector {
	[] => {
		$crate::chunk::vector::Vector::new()
	};
	[$($elem:expr),*$(,)?] => {{
		let mut vec = $crate::chunk::vector::Vector::new();
		$(vec.push($elem);)*
		vec
	}};
}

pub use vector;

pub struct Vector<T> {
	pub(super) ptr: NonNull<T>,
	pub(super) cap: usize,
	len: usize,
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T> Vector<T> {
	pub fn new() -> Self {
		assert!(mem::size_of::<T>() != 0);

		Self {
			ptr: NonNull::dangling(),
			cap: 0,
			len: 0,
		}
	}

	pub(super) fn ptr(&self) -> *mut T {
		self.ptr.as_ptr()
	}

	pub fn push(&mut self, element: T) {
		if self.len >= self.cap {
			self.grow();
		}
		unsafe {
			ptr::write(self.ptr().add(self.len), element);
		}
		self.len += 1;
	}

	pub(super) fn grow(&mut self) {
		let (new_cap, new_layout) = if self.cap == 0 {
			(8, Layout::array::<T>(8).unwrap())
		} else {
			let new_cap = self.cap * 2;
			let new_layout = Layout::array::<T>(new_cap).unwrap();

			(new_cap, new_layout)
		};

		assert!(
			new_layout.size() <= isize::MAX as usize,
			"Allocation too large"
		);

		let new_ptr = if self.cap == 0 {
			unsafe { alloc::alloc(new_layout) }
		} else {
			let old_layout = Layout::array::<T>(self.cap).unwrap();
			let old_ptr = self.ptr() as *mut u8;

			unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
		};

		self.ptr = match NonNull::new(new_ptr as *mut T) {
			Some(ptr) => ptr,
			None => alloc::handle_alloc_error(new_layout),
		};
		self.cap = new_cap;
	}
}

impl<T> Drop for Vector<T> {
	fn drop(&mut self) {
		if self.cap != 0 {
			let layout = Layout::array::<T>(self.cap).unwrap();
			unsafe { alloc::dealloc(self.ptr() as *mut u8, layout) }
		}
	}
}
