use super::*;

#[test]
fn it_works() {
	let mut chunk = Chunk::new();
	let handle = chunk.add_constant(1.2);
	chunk.write(OpCode::Constant as u8, 123);
	chunk.write(handle as u8, 123);
	chunk.write(4, 123);
	chunk.write(OpCode::Return as u8, 123);

	eprintln!("{:?}", chunk);
}
