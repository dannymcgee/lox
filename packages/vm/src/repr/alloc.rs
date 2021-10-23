use std::{
	alloc::{self, Allocator, GlobalAlloc, Layout, System},
	ptr::NonNull,
	sync::Mutex,
	thread::{self, JoinHandle},
	time::Duration,
};

use crate::cli;

lazy_static! {
	static ref STATE: Mutex<MemState> = Mutex::new(MemState::default());
}

pub struct Spy;

#[derive(Clone, Copy, Debug, Default)]
pub struct MemState {
	pub bytes: usize,
	pub allocs: usize,
	pub deallocs: usize,
	pub balance: usize,
}

impl Spy {
	pub fn enable_logging() -> JoinHandle<anyhow::Result<()>> {
		thread::spawn(|| loop {
			Self::report()?;
			thread::sleep(Duration::from_secs(1));
		})
	}

	fn report() -> anyhow::Result<()> {
		let state = *STATE.lock().unwrap();
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

		if let Ok(mut state) = STATE.lock() {
			state.bytes += layout.size();
			state.allocs += 1;
			state.balance = state.allocs - state.deallocs;
		}

		result
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let ptr = match NonNull::new(ptr) {
			Some(ptr) => ptr,
			None => alloc::handle_alloc_error(layout),
		};
		System.deallocate(ptr, layout);

		if let Ok(mut state) = STATE.lock() {
			state.bytes -= layout.size();
			state.deallocs += 1;
			state.balance = state.allocs - state.deallocs;
		}
	}
}
