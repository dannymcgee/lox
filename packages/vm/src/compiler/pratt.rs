use std::hash::BuildHasherDefault;

use gramatika::{ParseStreamer, Result, Spanned, SpannedError};
use macro_utils::trace;
use once_cell::sync::OnceCell;
use rustc_hash::{FxHashMap, FxHasher};

use crate::{
	chunk::{Chunk, OpCode},
	*,
};

use super::{
	debug::{self, RuleType},
	lexer::{Stream, Token},
	prec::Prec,
};

static RULES: OnceCell<FxHashMap<HashToken, ParseRule>> = OnceCell::new();

pub(super) trait PrattParser<'a>
where 'static: 'a
{
	type ParseFn;

	fn expression(&mut self, input: &mut Stream<'a>) -> Result<'a, ()>;
	fn parse_precedence(&mut self, input: &mut Stream<'a>, prec: Prec) -> Result<'a, ()>;
	fn number(&mut self, input: &mut Stream<'a>) -> Result<'a, ()>;
	fn unary(&mut self, input: &mut Stream<'a>) -> Result<'a, ()>;
	fn binary(&mut self, input: &mut Stream<'a>) -> Result<'a, ()>;
	fn grouping(&mut self, input: &mut Stream<'a>) -> Result<'a, ()>;
}

#[derive(Clone, Copy)]
pub struct ParseRule<'a>
where 'a: 'static
{
	prefix: Option<<Chunk as PrattParser<'a>>::ParseFn>,
	infix: Option<<Chunk as PrattParser<'a>>::ParseFn>,
	prec: Prec,
}

impl<'a> PrattParser<'a> for Chunk
where 'a: 'static
{
	type ParseFn = fn(&mut Self, input: &mut Stream<'a>) -> Result<'a, ()>;

	#[trace(debug::entry)]
	fn expression(&mut self, input: &mut Stream<'a>) -> Result<'a, ()> {
		self.parse_precedence(input, Prec::Assignment)
	}

	#[trace(debug::precedence)]
	fn parse_precedence(&mut self, input: &mut Stream<'a>, prec: Prec) -> Result<'a, ()> {
		if input.is_empty() {
			return Err(SpannedError {
				message: "Expected expression.".into(),
				source: input.source(),
				span: input.prev().map(|token| token.span()),
			});
		} else {
			input.next().unwrap();
		}

		let prev = *input.prev().unwrap();
		let rule = get_rule(prev.into());

		debug::set_rule_type(RuleType::Prefix);
		match rule.prefix {
			None => Err(SpannedError {
				message: "Expected expression.".into(),
				source: input.source(),
				span: Some(prev.span()),
			}),
			Some(prefix_rule) => prefix_rule(self, input),
		}?;

		while let Some(current) = input.peek() {
			let prev = *current;
			let rule = get_rule(prev.into());

			if prec <= rule.prec {
				input.next().unwrap();

				debug::set_rule_type(RuleType::Infix);
				match rule.infix {
					None => Err(SpannedError {
						message: "Expected expression.".into(),
						source: input.source(),
						span: Some(prev.span()),
					}),
					Some(infix_rule) => infix_rule(self, input),
				}?;
			} else {
				break;
			}
		}

		Ok(())
	}

	#[trace(debug::parse_fn)]
	fn number(&mut self, input: &mut Stream<'a>) -> Result<'a, ()> {
		let token = input.prev().unwrap();
		let (lexeme, span) = token.as_inner();

		let value = lexeme
			.parse::<f64>()
			.map_err(|err| SpannedError {
				message: format!("{}", err),
				source: input.source(),
				span: Some(span),
			})?;

		self.emit_const(value, span);

		Ok(())
	}

	#[trace(debug::parse_fn)]
	fn grouping(&mut self, input: &mut Stream<'a>) -> Result<'a, ()> {
		self.expression(input)?;
		input.consume(brace![")"])?;

		Ok(())
	}

	#[trace(debug::parse_fn)]
	fn unary(&mut self, input: &mut Stream<'a>) -> Result<'a, ()> {
		let prev = *input.prev().unwrap();

		// Handle the operand
		self.parse_precedence(input, Prec::Unary)?;

		match prev {
			Token::Operator("-", span) => {
				self.emit_instr(OpCode::Negate, span);
				Ok(())
			}
			other => Err(SpannedError {
				message: format!("Expected Operator `-`, got {:?}", other),
				span: Some(other.span()),
				source: input.source(),
			}),
		}
	}

	#[trace(debug::parse_fn)]
	fn binary(&mut self, input: &mut Stream<'a>) -> Result<'a, ()> {
		let prev = *input.prev().unwrap();
		let rule = get_rule(prev.into());

		self.parse_precedence(input, rule.prec + 1)?;

		let (op, span) = match prev {
			Token::Operator("+", span) => (OpCode::Add, span),
			Token::Operator("-", span) => (OpCode::Subtract, span),
			Token::Operator("*", span) => (OpCode::Multiply, span),
			Token::Operator("/", span) => (OpCode::Divide, span),
			_ => unreachable!(),
		};
		self.emit_instr(op, span);

		Ok(())
	}
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashToken {
	None = 0x00,
	LeftParen,
	Minus,
	Plus,
	Factor,
	Number,
}

impl<'a> From<Token<'a>> for HashToken {
	fn from(token: Token<'a>) -> Self {
		match token {
			Token::Brace("(", _) => Self::LeftParen,
			Token::Operator("-", _) => Self::Minus,
			Token::Operator("+", _) => Self::Plus,
			Token::Operator("*" | "/", _) => Self::Factor,
			Token::NumLit(_, _) => Self::Number,
			_ => Self::None,
		}
	}
}

#[trace(debug::get_rule)]
fn get_rule(token: HashToken) -> ParseRule<'static> {
	*RULES
		.get_or_init(init_pratt_table)
		.get(&token)
		.unwrap()
}

macro_rules! parse_fn {
	(None) => {
		None
	};
	($fn:ident) => {
		Some(<Chunk as PrattParser>::$fn)
	};
}

macro_rules! pratt_table {
	($($key:ident => { $prefix:ident, $infix:ident, $prec:ident })+) => {{
		let mut table: FxHashMap<HashToken, ParseRule> = FxHashMap::with_capacity_and_hasher(
			6,
			BuildHasherDefault::<FxHasher>::default(),
		);
		$(table.insert(HashToken::$key, ParseRule {
			prefix: parse_fn!($prefix),
			infix: parse_fn!($infix),
			prec: Prec::$prec,
		});)+

		table
	}};
}

fn init_pratt_table<'a>() -> FxHashMap<HashToken, ParseRule<'a>> {
	pratt_table! {
		LeftParen => { grouping,  None,     None }
		Minus     => { unary,     binary,   Term }
		Plus      => { None,      binary,   Term }
		Factor    => { None,      binary,   Factor }
		Number    => { number,    None,     None }
		None      => { None,      None,     None }
	}
}
