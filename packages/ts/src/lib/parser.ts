import * as Util from 'util';
import * as Chalk from 'chalk';

import { ErrorReporter } from './error-reporter';
import {
	Assign,
	Binary,
	Call,
	Expr,
	Grouping,
	Literal as _Literal,
	Logical,
	Token,
	TokenType,
	Unary,
	Variable,
} from './types';
import {
	Block,
	Expression,
	Fn,
	If,
	Print,
	Return,
	Stmt,
	Var,
	While,
} from './types/stmt';

namespace Operators {
	export const EQUALITY = [TokenType.BangEqual, TokenType.EqualEqual];
	export const COMPARISON = [
		TokenType.Greater,
		TokenType.GreaterEqual,
		TokenType.Less,
		TokenType.LessEqual,
	];
	export const TERM = [TokenType.Minus, TokenType.Plus];
	export const FACTOR = [TokenType.Slash, TokenType.Star];
	export const UNARY = [TokenType.Bang, TokenType.Minus];
}

class Literal extends _Literal {
	[Util.inspect.custom](): string {
		let color: Function;
		switch (typeof this.value) {
			case 'number':
				color = Chalk.cyan;
				break;
			case 'string':
				color = Chalk.green;
				break;
			case 'boolean':
				color = Chalk.magenta;
				break;
			default:
				if (this.value === null) color = Chalk.magenta;
				else color = Chalk.bold;
		}
		if (this.value == null) return color('nil');
		return color(this.value);
	}
}

export class Parser {
	private readonly tokens: readonly Token[];
	private current = 0;

	constructor(tokens: readonly Token[]) {
		this.tokens = tokens;
	}

	parse(): readonly Stmt[] | null {
		let statements: Stmt[] = [];
		while (!this.atEnd()) statements.push(this.declaration());

		return statements;
	}

	private declaration(): Stmt {
		try {
			if (this.match(TokenType.Fn)) return this.fnDeclaration('function');
			if (this.match(TokenType.Var)) return this.varDeclaration();
			return this.statement();
		} catch (err) {
			if (err instanceof ParseError) {
				this.synchronize();
				return;
			}
			throw err;
		}
	}
	private fnDeclaration(kind: string): Fn {
		let name = this.consume(TokenType.Identifier, `Expected ${kind} name.`);

		// Params
		this.consume(TokenType.LeftParen, `Expected '(' after ${kind} name.`);
		let params: Token[] = [];
		if (!this.check(TokenType.RightParen)) {
			do {
				if (params.length >= 255)
					this.error(
						this.peek(),
						`Cannot have more than 255 ${kind} parameters.`,
					);
				params.push(
					this.consume(
						TokenType.Identifier,
						`Expected parameter name.`,
					),
				);
			} while (this.match(TokenType.Comma));
		}
		this.consume(TokenType.RightParen, `Expected ')' after parameters.`);

		// Body
		this.consume(TokenType.LeftBrace, `Expected '{' before ${kind} body.`);
		let body: Stmt[] = this.block();

		return new Fn(name, params, body);
	}
	private varDeclaration(): Stmt {
		let name = this.consume(
			TokenType.Identifier,
			'Expected a variable name.',
		);
		let initializer = this.match(TokenType.Equal)
			? this.expression()
			: null;
		this.consume(
			TokenType.Semicolon,
			`Expected ';' after variable declaration.`,
		);

		return new Var(name, initializer);
	}

	private statement(): Stmt {
		if (this.match(TokenType.For)) return this.forStatement();
		if (this.match(TokenType.If)) return this.ifStatement();
		if (this.match(TokenType.Print)) return this.printStatement();
		if (this.match(TokenType.Return)) return this.returnStatement();
		if (this.match(TokenType.While)) return this.whileStatement();
		if (this.match(TokenType.LeftBrace)) return new Block(this.block());
		return this.expressionStatement();
	}
	private forStatement(): Stmt {
		this.consume(TokenType.LeftParen, `Expected '(' after 'for'.`);
		let initializer = this.match(TokenType.Semicolon)
			? null
			: this.match(TokenType.Var)
			? this.varDeclaration()
			: this.expressionStatement();
		let condition = this.check(TokenType.Semicolon)
			? new Literal(true)
			: this.expression();
		this.consume(TokenType.Semicolon, `Expected ';' after loop condition.`);
		let increment = this.check(TokenType.RightParen)
			? null
			: this.expression();
		this.consume(TokenType.RightParen, `Expected ')' after for clauses.`);

		let body = this.statement();
		if (increment) body = new Block([body, new Expression(increment)]);
		body = new While(condition, body);
		if (initializer) body = new Block([initializer, body]);

		return body;
	}
	private ifStatement(): Stmt {
		this.consume(TokenType.LeftParen, `Expected '(' after 'if'.`);
		let condition = this.expression();
		this.consume(TokenType.RightParen, `Expected ')' after if condition.`);

		let thenBranch = this.statement();
		let elseBranch = this.match(TokenType.Else) ? this.statement() : null;

		return new If(condition, thenBranch, elseBranch);
	}
	private block(): Stmt[] {
		let statements: Stmt[] = [];
		while (!this.check(TokenType.RightBrace) && !this.atEnd())
			statements.push(this.declaration());
		this.consume(TokenType.RightBrace, `Expected '}' after block.`);
		return statements;
	}
	private printStatement(): Stmt {
		let value = this.expression();
		this.consume(TokenType.Semicolon, `Expected ';' after value.`);
		return new Print(value);
	}
	private returnStatement(): Stmt {
		let keyword = this.previous();
		let value = null;
		if (!this.check(TokenType.Semicolon)) value = this.expression();
		this.consume(TokenType.Semicolon, `Expected ';' after return value.`);
		return new Return(keyword, value);
	}
	private whileStatement(): Stmt {
		this.consume(TokenType.LeftParen, `Expected '(' after 'while'.`);
		let condition = this.expression();
		this.consume(TokenType.RightParen, `Expected ')' after condition.`);
		let body = this.statement();
		return new While(condition, body);
	}
	private expressionStatement(): Stmt {
		let expr = this.expression();
		this.consume(TokenType.Semicolon, `Expected ';' after expression.`);
		return new Expression(expr);
	}

	private expression(): Expr {
		return this.assignment();
	}
	private assignment(): Expr {
		let expr = this.or();
		if (this.match(TokenType.Equal)) {
			let equals = this.previous();
			let value = this.assignment();

			if (expr instanceof Variable) {
				let name = expr.name;
				return new Assign(name, value);
			}
			this.error(equals, 'Invalid assignment target.');
		}
		return expr;
	}
	private or(): Expr {
		let expr = this.and();
		while (this.match(TokenType.Or)) {
			let operator = this.previous();
			let right = this.and();
			expr = new Logical(expr, operator, right);
		}
		return expr;
	}
	private and(): Expr {
		let expr = this.equality();
		while (this.match(TokenType.And)) {
			let operator = this.previous();
			let right = this.equality();
			expr = new Logical(expr, operator, right);
		}
		return expr;
	}
	private equality(): Expr {
		return this.binary(Operators.EQUALITY, this.comparison);
	}
	private comparison(): Expr {
		return this.binary(Operators.COMPARISON, this.term);
	}
	private term(): Expr {
		return this.binary(Operators.TERM, this.factor);
	}
	private factor(): Expr {
		return this.binary(Operators.FACTOR, this.unary);
	}
	private unary(): Expr {
		if (this.match(TokenType.Bang, TokenType.Minus)) {
			let operator = this.previous();
			let right = this.unary();

			return new Unary(operator, right);
		}
		return this.call();
	}
	private call(): Expr {
		let expr = this.primary();
		while (true) {
			if (this.match(TokenType.LeftParen)) expr = this.finishCall(expr);
			else break;
		}
		return expr;
	}
	finishCall(callee: Expr): Expr {
		let args: Expr[] = [];
		if (!this.check(TokenType.RightParen)) {
			do {
				if (args.length >= 255)
					this.error(
						this.peek(),
						`Cannot have more than 255 function arguments.`,
					);
				args.push(this.expression());
			} while (this.match(TokenType.Comma));
		}
		let paren = this.consume(
			TokenType.RightParen,
			`Expected ')' after arguments.`,
		);
		return new Call(callee, paren, args);
	}
	private primary(): Expr {
		// Language constants
		if (this.match(TokenType.False)) return new Literal(false);
		if (this.match(TokenType.True)) return new Literal(true);
		if (this.match(TokenType.Nil)) return new Literal(null);

		// Literal string / number
		if (this.match(TokenType.Number, TokenType.String))
			return new Literal(this.previous().literal);

		// Variable
		if (this.match(TokenType.Identifier))
			return new Variable(this.previous());

		// Paren group
		if (this.match(TokenType.LeftParen)) {
			let expr = this.expression();
			this.consume(
				TokenType.RightParen,
				`Expected ')' after expression.`,
			);
			return new Grouping(expr);
		}

		throw this.error(this.peek(), 'Expected expression.');
	}

	private synchronize(): void {
		this.advance();
		while (!this.atEnd()) {
			if (this.previous().type === TokenType.Semicolon) return;
			switch (this.peek().type) {
				case TokenType.Class:
				case TokenType.Fn:
				case TokenType.Var:
				case TokenType.For:
				case TokenType.If:
				case TokenType.While:
				case TokenType.Print:
				case TokenType.Return:
					return;
			}
			this.advance();
		}
	}

	private binary(operators: TokenType[], operandMethod: () => Expr) {
		let expr = operandMethod.call(this);
		while (this.match(...operators)) {
			let operator = this.previous();
			let right = operandMethod.call(this);
			expr = new Binary(expr, operator, right);
		}
		return expr;
	}

	private atEnd(): boolean {
		return this.peek().type === TokenType.EOF;
	}

	private advance(): Token {
		if (!this.atEnd()) this.current++;
		return this.previous();
	}
	private peek(): Token {
		return this.tokens[this.current];
	}
	private previous(): Token {
		return this.tokens[this.current - 1];
	}
	private consume(type: TokenType, errMessage: string): Token {
		if (this.check(type)) return this.advance();
		throw this.error(this.peek(), errMessage);
	}

	private check(type: TokenType): boolean {
		if (this.atEnd()) return false;
		return this.peek().type === type;
	}
	private match(...types: TokenType[]): boolean {
		if (types.some((type) => this.check(type))) {
			this.advance();
			return true;
		}
		return false;
	}

	private error(token: Token, message: string): Error {
		ErrorReporter.error(token, message);
		return new ParseError(token, message);
	}
}

class ParseError extends Error {
	readonly token: Token;
	constructor(token: Token, message: string) {
		super(message);
		this.token = token;
	}
}
