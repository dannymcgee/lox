import { ErrorReporter } from './error-reporter';
import { Line, Token, TokenType } from './types';

const KEYWORDS: ReadonlyMap<string, TokenType> = new Map([
	['and', TokenType.And],
	['class', TokenType.Class],
	['else', TokenType.Else],
	['false', TokenType.False],
	['for', TokenType.For],
	['fun', TokenType.Fun],
	['if', TokenType.If],
	['nil', TokenType.Nil],
	['or', TokenType.Or],
	['print', TokenType.Print],
	['return', TokenType.Return],
	['super', TokenType.Super],
	['this', TokenType.This],
	['true', TokenType.True],
	['var', TokenType.Var],
	['while', TokenType.While],
]);

export class Scanner {
	private readonly source: string;
	private readonly tokens: Token[] = [];
	private readonly lines: Line[] = [new Line(1, 0)];
	private start = 0;
	private lineStart = 0;
	private current = 0;
	private line = 1;

	constructor(source: Buffer | string) {
		this.source = source.toString();
	}

	scan(): { tokens: readonly Token[]; lines: readonly Line[] } {
		while (!this.atEnd()) {
			this.start = this.current;
			this.scanToken();
		}
		this.addToken(TokenType.EOF);

		return {
			tokens: this.tokens,
			lines: this.lines,
		};
	}

	private atEnd(): boolean {
		return this.current >= this.source.length;
	}

	private scanToken() {
		let c = this.advance();
		let type = this.identify(c);
		if (type != null) {
			if (type === TokenType.String) {
				let value = this.source.substring(
					this.start + 1,
					this.current - 1,
				);
				this.addToken(type, value);
			} else if (type === TokenType.Number) {
				let value = this.source.substring(this.start, this.current);
				this.addToken(type, parseFloat(value));
			} else if (type === TokenType.Identifier) {
				let value = this.source.substring(this.start, this.current);
				if (KEYWORDS.has(value)) {
					this.addToken(KEYWORDS.get(value));
				} else {
					this.addToken(type);
				}
			} else {
				this.addToken(type);
			}
		}
	}

	private identify(char: string): TokenType | undefined {
		// prettier-ignore
		switch (char) {
			// Single-character tokens
			case '(': return TokenType.LeftParen;
			case ')': return TokenType.RightParen;
			case '{': return TokenType.LeftBrace;
			case '}': return TokenType.RightBrace;
			case ',': return TokenType.Comma;
			case '.': return TokenType.Dot;
			case '-': return TokenType.Minus;
			case '+': return TokenType.Plus;
			case ';': return TokenType.Semicolon;
			case '*': return TokenType.Star;

			// 1- or 2-char tokens
			case '!': return this.match('=') ? TokenType.BangEqual : TokenType.Bang;
			case '=': return this.match('=') ? TokenType.EqualEqual : TokenType.Equal;
			case '<': return this.match('=') ? TokenType.LessEqual : TokenType.Less;
			case '>': return this.match('=') ? TokenType.GreaterEqual : TokenType.Greater;

			// Maybe comment
			case '/':
				if (!this.match('/'))
					return TokenType.Slash;
				while (this.peek() !== '\n' && !this.atEnd())
					this.advance();
				break; // Ignore comment

			case ' ':
			case '\r':
			case '\t':
				break; // Ignore whitespace

			case '\n':
				this.advanceLine();
				break;

			// String literals
			case '"': return this.string();

			default: {
				if (this.isDigit(char)) return this.number();
				if (this.isAlpha(char)) return this.identifier();

				this.finalizeLine();
				let token = this.addToken(TokenType.Unknown);
				ErrorReporter.error(
					token,
					'Unexpected character.'
				);
				break;
			}
		}
	}

	private isDigit(char: string): boolean {
		return /[0-9]/.test(char);
	}
	private isAlpha(char: string): boolean {
		return /[_a-zA-Z]/.test(char);
	}
	private isAlphaNumeric(char: string): boolean {
		return this.isAlpha(char) || this.isDigit(char);
	}

	private string(): TokenType | undefined {
		while (this.peek() !== '"' && !this.atEnd()) {
			if (this.peek() === '\n') this.advanceLine();
			this.advance();
		}
		if (this.atEnd()) {
			this.finalizeLine();
			let token = this.addToken(TokenType.String);
			ErrorReporter.error(token, 'Unterminated string');
			return;
		}
		// Consume the closing "
		this.advance();

		return TokenType.String;
	}

	private number(): TokenType {
		while (this.isDigit(this.peek())) this.advance();
		if (this.peek() === '.' && this.isDigit(this.peekNext())) {
			this.advance();
			while (this.isDigit(this.peek())) this.advance();
		}
		return TokenType.Number;
	}

	private identifier(): TokenType {
		while (this.isAlphaNumeric(this.peek())) this.advance();

		return TokenType.Identifier;
	}

	private advance(): string {
		return this.source.charAt(this.current++);
	}

	private peek(): string {
		if (this.atEnd()) return '\0';
		return this.source.charAt(this.current);
	}

	private peekNext(): string {
		if (this.current + 1 >= this.source.length) return '\0';
		return this.source.charAt(this.current + 1);
	}

	private match(expected: string): boolean {
		if (this.atEnd()) return false;
		if (this.source.charAt(this.current) !== expected) return false;

		this.current++;
		return true;
	}

	private finalizeLine(): Line {
		let line = this.lines[this.line - 1];
		let lineEnd = this.source.indexOf('\n', this.lineStart);
		if (lineEnd === -1) lineEnd = this.source.length - 1;
		let content = this.source.substring(this.lineStart, lineEnd);
		line.end = lineEnd;
		line.content = content;

		return line;
	}

	private advanceLine(): void {
		this.finalizeLine();
		this.line++;
		this.lineStart = this.current;
		this.lines.push(new Line(this.line, this.current));
	}

	private addToken(type: TokenType, literal?: any): Token {
		let text = this.source.substring(this.start, this.current);
		let line = this.lines[this.line - 1];

		let token = new Token(type, text, literal, line, this.start);
		this.tokens.push(token);

		return token;
	}
}
