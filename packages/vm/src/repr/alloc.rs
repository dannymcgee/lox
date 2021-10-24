use std::{
	alloc::{self, Allocator, GlobalAlloc, Layout, System},
	ptr::NonNull,
	thread::{self, JoinHandle},
	time::Duration,
};

use parking_lot::Mutex;

use crate::cli;

lazy_static! {
	static ref STATE: Mutex<MemState> = Mutex::new(MemState::default());
}

pub struct Spy;

#[derive(Clone, Copy, Debug, Default)]
pub struct MemState {
	pub bytes: usize,
	pub allocs: usize,
}

impl Spy {
	pub fn enable_logging() -> JoinHandle<anyhow::Result<()>> {
		thread::spawn(|| loop {
			Self::report()?;
			thread::sleep(Duration::from_secs(1));
		})
	}

	fn report() -> anyhow::Result<()> {
		let state = *STATE.lock();
		cli::stdio().update_mem_readout(state)
	}
}

unsafe impl GlobalAlloc for Spy {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let result = match System.allocate(layout) {
			Ok(ptr) => ptr.as_ptr() as *mut u8,
			Err(err) => {
				eprintln!("{}", err);
				alloc::handle_alloc_error(layout);
			}
		};

		let mut state = STATE.lock();
		state.bytes += layout.size();
		state.allocs += 1;

		result
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let ptr = match NonNull::new(ptr) {
			Some(ptr) => ptr,
			None => alloc::handle_alloc_error(layout),
		};
		System.deallocate(ptr, layout);

		let mut state = STATE.lock();
		state.bytes -= layout.size();
		state.allocs -= 1;
	}
}
