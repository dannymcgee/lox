use std::{fmt, mem, str::FromStr};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub enum Value {
	Number(f64),
	Bool(bool),
	Nil,
}

impl Value {
	pub fn is_falsy(&self) -> bool {
		match self {
			Value::Number(_) => false,
			Value::Bool(b) => !b,
			Value::Nil => true,
		}
	}
}

impl FromStr for Value {
	type Err = <f64 as FromStr>::Err; // TODO

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"true" | "false" => Ok(Value::Bool(bool::from_str(s).unwrap())),
			"nil" => Ok(Value::Nil),
			_ => f64::from_str(s).map(Value::Number),
		}
	}
}

impl From<f64> for Value {
	fn from(n: f64) -> Self {
		Value::Number(n)
	}
}

impl From<bool> for Value {
	fn from(b: bool) -> Self {
		Value::Bool(b)
	}
}

impl<T> From<Option<T>> for Value
where T: Into<Value>
{
	fn from(opt: Option<T>) -> Self {
		match opt {
			Some(value) => value.into(),
			None => Value::Nil,
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Number(lhs), Self::Number(rhs)) => (lhs - rhs).abs() < f64::EPSILON,
			(Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
			_ => mem::discriminant(self) == mem::discriminant(other),
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Number(n) => n.fmt(f),
			Value::Bool(b) => b.fmt(f),
			Value::Nil => write!(f, "nil"),
		}
	}
}
