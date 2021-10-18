use core::fmt;

use gramatika::{Parse, ParseStreamer, Span, Spanned, SpannedError, Token as _};

use super::Args;

pub(super) fn parse(raw_args: String) -> gramatika::Result<'static, Args> {
	ParseStream::from(raw_args).parse()
}

type ParseStream<'a> = gramatika::ParseStream<'a, Token<'a>, Lexer<'a>>;

#[derive(Clone, Copy, DebugLispToken, Token, Lexer)]
pub enum Token<'a> {
	#[pattern = "--"]
	ArgStart(&'a str, Span),

	#[pattern = "[0-9.]+"]
	Number(&'a str, Span),

	#[pattern = "(true|false)"]
	Boolean(&'a str, Span),

	#[pattern = "="]
	Equal(&'a str, Span),

	#[pattern = "[_a-zA-Z][-_a-zA-Z0-9]+"]
	Word(&'a str, Span),
}

impl<'a> Parse<'a> for Args {
	type Stream = ParseStream<'a>;

	fn parse(input: &mut Self::Stream) -> gramatika::Result<'a, Self> {
		use TokenKind::*;

		let mut result = Args { example: None };

		while !input.is_empty() {
			input.consume_kind(ArgStart)?;

			let key = input.consume_kind(Word)?;
			if key.lexeme() == "example" {
				input.consume_kind(Equal)?;
				result.example = Some(input.consume_kind(Word)?.lexeme().into());
			} else {
				return Err(SpannedError {
					source: input.source(),
					span: Some(key.span()),
					message: format!("Unknown argument '{}'", key.lexeme()),
				});
			}
		}

		Ok(result)
	}
}

impl<'a> fmt::Debug for Token<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		<Self as gramatika::DebugLisp>::fmt(self, f, 0)
	}
}
