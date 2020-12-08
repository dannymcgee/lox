import * as Util from 'util';
import * as Chalk from 'chalk';
import { Environment } from './environment';

import { ErrorReporter } from './error-reporter';
import { FnObject } from './fn-object';
import { TokenType, Token, isInvokable, Invokable } from './types';
import * as Expr from './types/expr';
import * as Stmt from './types/stmt';

export class Interpreter implements Expr.Visitor<Object>, Stmt.Visitor<void> {
	readonly globals = new Environment();
	private env = this.globals;

	constructor() {
		this.globals.define(
			'clock',
			new (class implements Invokable {
				arity(): number {
					return 0;
				}
				invoke(interpreter: Interpreter, ...args: Object[]): Object {
					return new Date().getTime();
				}
				toString(): string {
					return '<native fn>';
				}
			})(),
		);
		this.globals.define(
			'str',
			new (class implements Invokable {
				arity(): number {
					return 1;
				}
				invoke(interpreter: Interpreter, ...args: Object[]): string {
					let obj = args[0];
					if (typeof obj === 'number') return obj.toString(10);
					else if (
						'toString' in obj &&
						typeof obj.toString === 'function'
					)
						return obj.toString();
					return `${obj}`;
				}
				toString(): string {
					return '<native fn>';
				}
			})(),
		);
	}

	interpret(statements: readonly Stmt.Stmt[]): void {
		try {
			for (let statement of statements) this.execute(statement);
		} catch (err) {
			if (err instanceof RuntimeError) {
				ErrorReporter.error(err.token, err.message);
				return null;
			}
			throw err;
		}
	}

	visitVarStmt(stmt: Stmt.Var): void {
		// prettier-ignore
		let value = stmt.initializer != null
			? this.evaluate(stmt.initializer)
			: null;

		this.env.define(stmt.name.lexeme, value);
	}
	visitExpressionStmt(stmt: Stmt.Expression): void {
		this.evaluate(stmt.expression);
	}
	visitFnStmt(stmt: Stmt.Fn): void {
		let fn = new FnObject(stmt.func, this.env, stmt.name.lexeme);
		this.env.define(stmt.name.lexeme, fn);
	}
	visitIfStmt(stmt: Stmt.If): void {
		if (this.isTruthy(this.evaluate(stmt.condition)))
			this.execute(stmt.thenBranch);
		else if (stmt.elseBranch) this.execute(stmt.elseBranch);
	}
	visitPrintStmt(stmt: Stmt.Print): void {
		let value = this.evaluate(stmt.expression);
		console.log(formatValue(value));
	}
	visitReturnStmt(stmt: Stmt.Return): void {
		let value = null;
		if (stmt.value) value = this.evaluate(stmt.value);

		throw new Return(value);
	}
	visitWhileStmt(stmt: Stmt.While): void {
		while (this.isTruthy(this.evaluate(stmt.condition)))
			this.execute(stmt.body);
	}
	visitBlockStmt(stmt: Stmt.Block): void {
		this.executeBlock(stmt.statements, new Environment(this.env));
	}

	private execute(stmt: Stmt.Stmt): void {
		stmt.accept(this);
	}
	executeBlock(statements: Stmt.Stmt[], enclosing: Environment) {
		let previousEnv = this.env;
		try {
			this.env = enclosing;
			for (let statement of statements) this.execute(statement);
		} finally {
			this.env = previousEnv;
		}
	}

	visitAssignExpr(expr: Expr.Assign): Object {
		let value = this.evaluate(expr.value);
		this.env.assign(expr.name, value);
		return value;
	}
	visitLiteralExpr(expr: Expr.Literal): Object {
		return expr.value;
	}
	visitLogicalExpr(expr: Expr.Logical): Object {
		let left = this.evaluate(expr.left);
		if (expr.operator.type === TokenType.Or) {
			if (this.isTruthy(left)) return left;
		} else if (!this.isTruthy(left)) return left;
		return this.evaluate(expr.right);
	}
	visitFnExpr(expr: Expr.Fn): Object {
		return new FnObject(expr, this.env);
	}
	visitGroupingExpr(expr: Expr.Grouping): Object {
		return this.evaluate(expr.expression);
	}
	visitUnaryExpr(expr: Expr.Unary): Object {
		let right = this.evaluate(expr.right);
		// prettier-ignore
		switch (expr.operator.type) {
			case TokenType.Bang: return !this.isTruthy(right);
			case TokenType.Minus:
				this.checkNumberOperands(expr.operator, right);
				return -(right as number);
			}
		return null;
	}
	visitVariableExpr(expr: Expr.Variable): Object {
		return this.env.get(expr.name);
	}
	visitBinaryExpr(expr: Expr.Binary): Object {
		let left = this.evaluate(expr.left) as any;
		let right = this.evaluate(expr.right) as any;

		switch (expr.operator.type) {
			case TokenType.Minus:
			case TokenType.Slash:
			case TokenType.Star:
			case TokenType.Greater:
			case TokenType.GreaterEqual:
			case TokenType.Less:
			case TokenType.LessEqual:
				this.checkNumberOperands(expr.operator, left, right);
				break;
		}

		// prettier-ignore
		switch (expr.operator.type) {
			case TokenType.Minus: return left - right;
			case TokenType.Slash: return left / right;
			case TokenType.Star: return left * right;

			case TokenType.Greater: return left > right;
			case TokenType.GreaterEqual: return left >= right;
			case TokenType.Less: return left < right;
			case TokenType.LessEqual: return left <= right;

			case TokenType.Plus:
				this.checkStringOrNumber(expr.operator, left, right);
				if (typeof left === 'string') return `${left}${right}`;
				if (typeof left === 'number') return left + right;
				break;

			case TokenType.BangEqual: return !this.isEqual(left, right);
			case TokenType.EqualEqual: return this.isEqual(left, right);
		}
		return null;
	}
	visitCallExpr(expr: Expr.Call): Object {
		let callee = this.evaluate(expr.callee);
		let args: Object[] = expr.args.map((arg) => this.evaluate(arg));

		if (isInvokable(callee)) {
			let arity = callee.arity();
			if (args.length !== arity)
				throw new RuntimeError(
					expr.paren,
					`Expected ${arity} arguments but received ${args.length}.`,
				);
			return callee.invoke(this, ...args);
		}
		throw new RuntimeError(expr.paren, `Expression is not invokable.`);
	}

	private evaluate(expr: Expr.Expr): Object {
		return expr.accept(this);
	}

	private isTruthy(value: Object): boolean {
		if (typeof value === 'boolean') return value;
		return value != null;
	}
	private isEqual(a: Object, b: Object): boolean {
		return a === b || (a == null && b == null);
	}
	// prettier-ignore
	private checkNumberOperands(operator: Token, left: Object, right: Object): void;
	private checkNumberOperands(operator: Token, operand: Object): void;
	private checkNumberOperands(
		operator: Token,
		left: Object,
		right?: Object,
	): void {
		if (
			typeof left === 'number' &&
			(right == null || typeof right === 'number')
		)
			return;
		throw new RuntimeError(
			operator,
			right == null
				? 'Operand must be a number.'
				: 'Operands must be numbers.',
		);
	}
	private checkStringOrNumber(
		operator: Token,
		left: Object,
		right: Object,
	): void {
		this.checkSameType(operator, left, right);
		if (typeof left === 'string' || typeof left === 'number') return;
		throw new RuntimeError(
			operator,
			'Operands must be strings or numbers.',
		);
	}
	private checkSameType(operator: Token, left: Object, right: Object): void {
		if (typeof left === typeof right) return;
		throw new RuntimeError(operator, 'Operands must be the same type.');
	}
}

export class RuntimeError extends Error {
	readonly token: Token;
	constructor(token: Token, message: string) {
		super(message);
		this.token = token;
	}
}

export class Return extends Error {
	readonly value: Object;
	constructor(value: Object) {
		super();
		this.value = value;
	}
}

export function formatValue(result: Object): string {
	if (typeof result === 'string') return Chalk.bold.green(result);
	if (typeof result === 'number') return Chalk.bold.cyan(result.toString());
	if (typeof result === 'boolean') return Chalk.magenta(result.toString());
	if (result == null) {
		if (ErrorReporter.hadError) return Chalk.red('Error');
		return Chalk.magenta('nil');
	}
}
