use std::{
	alloc::{self, Allocator, GlobalAlloc, Layout, System},
	ptr::NonNull,
	sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
	thread::{self, JoinHandle},
	time::Duration,
};

use crate::cli;

static BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

static LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);

pub struct Spy;

pub struct MemState {
	pub bytes: usize,
	pub allocs: usize,
	pub deallocs: usize,
	pub balance: usize,
}

impl Spy {
	pub fn enable_logging() -> JoinHandle<()> {
		LOGGING_ENABLED.store(true, SeqCst);

		thread::spawn(|| loop {
			Self::report();
			thread::sleep(Duration::from_secs(1));
		})
	}

	fn report() {
		let bytes = BYTES.load(SeqCst);
		let allocs = ALLOCATIONS.load(SeqCst);
		let deallocs = DEALLOCATIONS.load(SeqCst);
		let balance = allocs - deallocs;

		cli::update_mem_readout(MemState {
			bytes,
			allocs,
			deallocs,
			balance,
		});
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

		BYTES.fetch_add(layout.size(), SeqCst);
		ALLOCATIONS.fetch_add(1, SeqCst);

		result
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let ptr = match NonNull::new(ptr) {
			Some(ptr) => ptr,
			None => alloc::handle_alloc_error(layout),
		};
		System.deallocate(ptr, layout);

		BYTES.fetch_sub(layout.size(), SeqCst);
		ALLOCATIONS.load(SeqCst);
		DEALLOCATIONS.fetch_add(1, SeqCst);
	}
}
