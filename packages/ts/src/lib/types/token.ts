// prettier-ignore
export enum TokenType {
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

export class Token {
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
		let value = `${this.lexeme} | ${TokenType[this.type]}`;
		if (this.literal) {
			value += ` | [${this.literal}]`;
		}
		return value + ` | ${this.line}`;
	}
}
