use super::*;

#[test]
fn it_works() {
	let mut chunk = Chunk::new();
	chunk.write_const(1.2.into(), 123);
	chunk.write_instr(OpCode::Return, 123);

	chunk.write_const(420.0.into(), 124);
	chunk.write_const(69.0.into(), 124);
	chunk.write_instr(OpCode::Return, 124);

	// eprintln!("{:?}", chunk);
	let expected = r#"
0000   123 CONSTANT          [0] '1.2'
0002     | RETURN
0003   124 CONSTANT          [1] '420'
0005     | CONSTANT          [2] '69'
0007     | RETURN
"#;
	assert_eq!(&format!("\n{:?}\n", chunk), expected);
}

#[test]
fn it_can_store_more_than_255_constants() {
	let mut chunk = Chunk::new();

	let mut line = 1;
	for i in 0..=265 {
		if i > 0 && i % 3 == 0 {
			line += 1;
		}
		chunk.write_const((i as f64).into(), line);
	}

	// eprintln!("{:?}", chunk);
	assert_eq!(chunk.constants.len(), 266);
	assert_eq!(chunk.constants[265], Value::Number(265.));
}

#[test]
fn it_can_store_more_than_65535_constants() {
	let mut chunk = Chunk::new();

	let mut line = 1;
	for i in 0..=(u16::MAX as usize) + 10 {
		if i > 0 && i % 100 == 0 {
			line += 1;
		}
		chunk.write_const((i as f64).into(), line);
	}

	// eprintln!("{:?}", chunk);
	assert_eq!(chunk.constants.len(), 65_546);
	assert_eq!(chunk.constants[65_545], Value::Number(65_545.));
}
