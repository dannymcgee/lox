use std::fmt;

use nu_ansi_term::Color;

use crate::repr::Value;

pub trait FmtColored {
	fn fmt_(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.fmt_colored())
	}

	fn fmt_colored(&self) -> String;
}

impl FmtColored for Value {
	fn fmt_colored(&self) -> String {
		match self {
			Value::Number(n) => n.fmt_colored(),
			Value::Bool(b) => b.fmt_colored(),
			Value::Nil => Color::Cyan.italic().paint("nil").to_string(),
		}
	}
}

impl FmtColored for f64 {
	fn fmt_colored(&self) -> String {
		let prec = if self.abs() % 1. < f64::EPSILON {
			0
		} else if self.abs() * 10. % 1. < f64::EPSILON {
			1
		} else if self.abs() * 100. % 1. < f64::EPSILON {
			2
		} else {
			3
		};

		Color::Cyan
			.paint(format!("{1:.0$}", prec, self))
			.to_string()
	}
}

impl FmtColored for bool {
	fn fmt_colored(&self) -> String {
		Color::Cyan
			.italic()
			.paint(format!("{}", self))
			.to_string()
	}
}

impl FmtColored for &str {
	fn fmt_colored(&self) -> String {
		Color::Green
			.paint(format!(r#""{}""#, self))
			.to_string()
	}
}
