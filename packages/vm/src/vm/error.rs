use std::fmt;

#[derive(Debug)]
pub enum Error {
	Compile,
	Runtime,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Compile => write!(f, "CompileError"),
			Error::Runtime => write!(f, "RuntimeError"),
		}
	}
}
