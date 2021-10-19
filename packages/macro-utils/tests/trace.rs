#![allow(unused_imports, dead_code, clippy::eq_op)]

#[macro_use]
extern crate macro_utils;

mod fmt;

fn main() {
	if add(2, 2) == 4 {
		println!("Hooray!");
	} else {
		println!("Booooo.");
	}
}

#[trace(fmt::fn_call)]
fn add(a: isize, b: isize) -> isize {
	a + b
}
