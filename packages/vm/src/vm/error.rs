use std::fmt;

#[derive(Debug)]
pub enum Error {
	Compile,
	Runtime(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Compile => write!(f, "CompileError"),
			Error::Runtime(msg) => write!(f, "RuntimeError: {}", msg),
		}
	}
}
