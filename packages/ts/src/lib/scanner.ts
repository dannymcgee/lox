import { ErrorReporter } from './error-reporter';

// prettier-ignore
enum TokenType {
	// Single-character tokens
	LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus,
	Semicolon, Slash, Star,
	// 1- or 2-char tokens
	Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,
	// Literals
	Identifier, String, Number,
	// Keywords
	And, Class, Else, False, Fn, For, If, Nil, Or, Print, Return, Super, This,
	True, Var, While,
	// Fin
	EOF,
}

class Token {
	readonly type: TokenType;
	readonly lexeme: string;
	readonly literal: any;
	readonly line: number;

	constructor(type: TokenType, lexeme: string, literal: any, line: number) {
		this.type = type;
		this.lexeme = lexeme;
		this.literal = literal;
		this.line = line;
	}

	toString(): string {
		if (this.literal) {
			return `${this.lexeme} : ${TokenType[this.type]} : [${
				this.literal
			}]`;
		}
		return `${this.lexeme} : ${TokenType[this.type]}`;
	}
}

export class Scanner {
	private readonly source: string;
	private readonly tokens: Token[] = [];
	private start = 0;
	private current = 0;
	private line = 1;

	constructor(source: Buffer | string) {
		this.source = source.toString();
	}

	scanTokens(): ReadonlyArray<Token> {
		while (!this.atEnd()) {
			this.start = this.current;
			this.scanToken();
		}
		this.tokens.push(new Token(TokenType.EOF, '', null, this.line));

		return this.tokens;
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
				this.line++;
				break;

			// String literals
			case '"': return this.string();

			default:
				if (this.isDigit(char)) return this.number();

				ErrorReporter.error(this.line, 'Unexpected character.');
				break;
		}
	}

	private isDigit(char: string): boolean {
		return /[0-9]/.test(char);
	}

	private string(): TokenType | undefined {
		while (this.peek() !== '"' && !this.atEnd()) {
			if (this.peek() === '\n') this.line++;
			this.advance();
		}
		if (this.atEnd()) {
			ErrorReporter.error(this.line, 'Unterminated string.');
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

	private advance(): string {
		return this.source.charAt(this.current++);
	}

	private peek(): string {
		if (this.atEnd()) return '\0';
		return this.source.charAt(this.current);
	}

	private match(expected: string): boolean {
		if (this.atEnd()) return false;
		if (this.source.charAt(this.current) !== expected) return false;

		this.current++;
		return true;
	}

	private addToken(type: TokenType, literal?: any) {
		let text = this.source.substring(this.start, this.current);
		this.tokens.push(new Token(type, text, literal, this.line));
	}
}
