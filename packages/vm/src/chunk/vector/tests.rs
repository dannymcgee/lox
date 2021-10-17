use crate::chunk::OpCode;

use super::*;

#[test]
fn consuming_iterator() {
	let codes = vector![
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Return,
	];

	for (idx, code) in codes.into_iter().enumerate() {
		eprintln!("{:04} {:?}", idx, code);
	}
}

#[test]
fn by_ref_iterator() {
	let codes = vector![
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Return,
	];

	for (idx, code) in codes.iter().enumerate() {
		eprintln!("{:04} {:?}", idx, code);
	}

	assert_eq!(codes[3], OpCode::Return);
}

#[test]
fn by_ref_mutable_iterator() {
	let mut codes = vector![
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Constant,
		OpCode::Return,
	];

	eprintln!("{:?}", codes);

	for (idx, code) in codes.iter_mut().enumerate() {
		if idx % 2 == 0 {
			*code = OpCode::Return;
		} else {
			*code = OpCode::Constant;
		}
	}
	eprintln!();
	eprintln!("{:?}", codes);

	assert_eq!(codes[0], OpCode::Return);
	assert_eq!(codes[1], OpCode::Constant);
	assert_eq!(codes[2], OpCode::Return);
	assert_eq!(codes[3], OpCode::Constant);
}
