use std::{
	fmt::Write as _,
	io::Write as _,
	sync::{
		atomic::{AtomicBool, AtomicUsize, Ordering},
		Mutex, MutexGuard,
	},
};

use gramatika::{ParseStreamer, Span, Token as _};
use nu_ansi_term::Color;
use once_cell::sync::OnceCell;

use crate::{
	chunk::OpCode,
	cli::{self, DebugFlags},
	compiler::lexer::TokenKind,
	debug::Repeat,
	repr::Value,
};

use super::{
	lexer::{Stream, Token},
	pratt::HashToken,
	prec::Prec,
};

#[derive(Clone, Copy, Debug)]
pub enum RuleType {
	Prefix,
	Infix,
}

static BUFFER: OnceCell<Mutex<String>> = OnceCell::new();
lazy_static! {
	static ref ENABLED: AtomicBool = AtomicBool::new(false);
	static ref INDENT: AtomicUsize = AtomicUsize::new(0);
	static ref RULE_TYPE: Mutex<RuleType> = Mutex::new(RuleType::Prefix);
}

pub(super) fn write_header(name: &str) {
	if cli::debug_flags().intersects(DebugFlags::COMPILE) {
		cli::print_header("compile", name);
	}
}

pub(super) fn entry(name: &'static str, _: &mut Stream) {
	if should_print(DebugFlags::PARSE) {
		reset_indent(0);
		write_label("parse");
		write_fn_call(name);
		flush();
	}
}

pub(super) fn precedence(name: &'static str, _: &mut Stream, prec: Prec) {
	if should_print(DebugFlags::PARSE) {
		reset_indent(1);
		write_indent(1);
		write_fn_call(name);
		write_prec(prec);
		flush();
	}
}

pub(super) fn get_rule(name: &'static str, token: HashToken) {
	if should_print(DebugFlags::PARSE) {
		write_indent(get_indent());
		write_fn_call(name);
		write_enum_variant(&format!("{:?}", token));
		flush();
	}
}

pub(super) fn set_rule_type(rule_type: RuleType) {
	let mut guard = RULE_TYPE.lock().unwrap();
	*guard = rule_type;
}

fn get_rule_type() -> RuleType {
	*RULE_TYPE.lock().unwrap()
}

pub(super) fn parse_fn(name: &'static str, input: &mut Stream) {
	if should_print(DebugFlags::PARSE) {
		write_indent(inc_indent());
		write_rule_type(get_rule_type());
		write_fn_call(name);
		write_token(*input.prev().unwrap());
		flush();
	}
}

pub(super) fn codegen_instr(name: &'static str, op: OpCode, _: Span) {
	if should_print(DebugFlags::CODEGEN) {
		write_label("codegen");
		write_fn_call(name);
		write_byte(op as u8);
		write_opcode(op);
		flush();
	}
}

pub(super) fn codegen_pair(name: &'static str, pair: (OpCode, OpCode), _: Span) {
	if should_print(DebugFlags::CODEGEN) {
		write_label("codegen");
		write_fn_call(name);

		let (a, b) = pair;
		write_byte(a as u8);
		write_opcode(a);
		write_byte(b as u8);
		write_opcode(b);

		flush();
	}
}

pub(super) fn codegen_const(name: &'static str, value: Value, _: Span) {
	if should_print(DebugFlags::CODEGEN) {
		write_label("codegen");
		write_fn_call(name);
		write_number(&format!("{}", value));
		flush();
	}
}

fn should_print(flag: DebugFlags) -> bool {
	cli::debug_flags().contains(flag)
}

fn reset_indent(value: usize) {
	INDENT.store(value, Ordering::Release);
}

fn inc_indent() -> usize {
	INDENT.fetch_add(1, Ordering::AcqRel) + 1
}

fn get_indent() -> usize {
	INDENT.load(Ordering::Acquire)
}

fn write_label(label: &str) {
	let mut buf = buf();
	write!(
		&mut buf,
		"{} {} ",
		Color::Yellow.paint(label),
		cli::prompt_char()
	)
	.unwrap()
}

fn write_indent(depth: usize) {
	let mut buf = buf();
	let indent = ' '.repeat(3 * depth);

	write!(&mut buf, "{}", indent).unwrap();
}

fn write_fn_call(name: &'static str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::DarkGray.paint(name)).unwrap();
}

fn write_prec(prec: Prec) {
	let mut buf = buf();
	let byte = prec as u8;
	let name = format!("{:?}", prec);

	write!(
		&mut buf,
		"{} {} ",
		Color::DarkGray.paint(format!("{}", byte)),
		Color::DarkGray.paint(name),
	)
	.unwrap();
}

fn write_token(token: Token) {
	use TokenKind::*;

	match token.kind() {
		NumLit => {
			write_enum_variant("NumLit");
			write_number(token.lexeme());
		}
		Operator => {
			write_enum_variant("Operator");
			write_operator_bright(token.lexeme());
		}
		Punct => {
			write_enum_variant("Punct");
			write_operator(token.lexeme());
		}
		Brace => {
			write_enum_variant("Brace");
			write_operator_bright(token.lexeme());
		}
		Keyword => {
			write_enum_variant("Keyword");
			let lex = token.lexeme();
			match lex {
				"true" | "false" | "nil" => {
					write_language_constant(lex);
				}
				_ => write_keyword(lex),
			}
		}
		_ => unimplemented!(),
	};
}

fn write_rule_type(rule_type: RuleType) {
	let mut buf = buf();
	let name = format!("[{:?}]", rule_type);

	write!(&mut buf, "{} ", Color::DarkGray.paint(name)).unwrap();
}

fn write_opcode(op: OpCode) {
	let mut buf = buf();
	let name = format!("{:?}", op);

	write!(&mut buf, "{} ", Color::Fixed(5).bold().paint(name)).unwrap();
}

fn write_byte(byte: u8) {
	let mut buf = buf();
	write!(
		&mut buf,
		"{}",
		Color::DarkGray.paint(format!("{:#04x} ", byte))
	)
	.unwrap();
}

fn write_number(lex: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::Cyan.paint(lex)).unwrap();
}

fn write_operator(lex: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::DarkGray.paint(lex)).unwrap();
}

fn write_operator_bright(lex: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::LightGray.paint(lex)).unwrap();
}

fn write_enum_variant(name: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::DarkGray.italic().paint(name)).unwrap();
}

fn write_language_constant(lex: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::Cyan.italic().paint(lex)).unwrap();
}

fn write_keyword(lex: &str) {
	let mut buf = buf();
	write!(&mut buf, "{} ", Color::Magenta.italic().paint(lex)).unwrap();
}

fn flush() {
	let mut buf = buf();
	let mut stdout = cli::stdout();

	writeln!(stdout, "{}", buf).unwrap();
	stdout.flush().unwrap();
	buf.clear();
}

fn buf<'a>() -> MutexGuard<'a, String> {
	BUFFER
		.get_or_init(|| Mutex::new(String::new()))
		.lock()
		.unwrap()
}
