import * as Util from 'util';
import * as Chalk from 'chalk';
import { Line } from './line';

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
	And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This,
	True, Var, While,
	// Error
	Unknown,
	// Fin
	EOF,
}

export class Token {
	readonly type: TokenType;
	readonly lexeme: string;
	readonly literal: any;
	readonly line: Line;
	readonly start: number;

	constructor(
		type: TokenType,
		lexeme: string,
		literal: any,
		line: Line,
		start: number,
	) {
		this.type = type;
		this.lexeme = lexeme;
		this.literal = literal;
		this.line = line;
		this.start = start;
	}

	toString(): string {
		let value = `${this.lexeme} | ${TokenType[this.type]}`;
		if (this.literal) {
			value += ` | [${this.literal}]`;
		}
		return value + ` | ${this.line}`;
	}

	[Util.inspect.custom](): string {
		return Chalk.bold.white(this.lexeme);
	}
}
