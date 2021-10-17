use super::Stack;

#[test]
fn it_works() {
	let mut stack = Stack::new();
	stack.push(1);
	stack.push(2);
	stack.push(3);

	assert_eq!(stack.size(), 3);

	let three = stack.pop().unwrap();
	let two = stack.pop().unwrap();

	stack.push(4);
	stack.push(5);

	assert_eq!(three, 3);
	assert_eq!(two, 2);

	assert_eq!(stack.pop().unwrap(), 5);
	assert_eq!(stack.pop().unwrap(), 4);

	stack.empty();
	assert!(stack.is_empty());

	for i in 0..=255 {
		stack.push(i);
	}

	let mut i = 255;
	while let Some(val) = stack.pop() {
		assert_eq!(val, i);
		i -= 1;
	}
	assert!(stack.is_empty());
}

#[test]
fn it_supports_fmt_debug() {
	let mut stack = Stack::new();
	stack.push("foo");
	stack.push("bar");
	stack.push("baz");

	let debug = format!("{:?}", stack);
	assert_eq!(&debug, r#"["foo", "bar", "baz"]"#);

	stack.pop().unwrap();

	let debug = format!("{:?}", stack);
	assert_eq!(&debug, r#"["foo", "bar"]"#);

	stack.push("Lorem Ipsum");

	let debug = format!("{:?}", stack);
	assert_eq!(&debug, r#"["foo", "bar", "Lorem Ipsum"]"#);

	stack.empty();

	let debug = format!("{:?}", stack);
	assert_eq!(&debug, "[]");
}
