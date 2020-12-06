import * as Chalk from 'chalk';

import { ErrorReporter } from './error-reporter';
import {
	Binary,
	Expr,
	Grouping,
	Literal,
	Token,
	TokenType,
	Unary,
	Visitor,
} from './types';

export class Interpreter implements Visitor<Object> {
	interpret(expression: Expr): Object {
		let result: Object;
		try {
			result = this.evaluate(expression);
		} catch (err) {
			if (err instanceof RuntimeError) {
				ErrorReporter.error(err.token, err.message);
				return null;
			}
			throw err;
		}
		return result;
	}
	visitLiteralExpr(expr: Literal): Object {
		return expr.value;
	}
	visitGroupingExpr(expr: Grouping): Object {
		return this.evaluate(expr.expression);
	}
	visitUnaryExpr(expr: Unary): Object {
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
	visitBinaryExpr(expr: Binary): Object {
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

	private evaluate(expr: Expr): Object {
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

class RuntimeError extends Error {
	readonly token: Token;
	constructor(token: Token, message: string) {
		super(message);
		this.token = token;
	}
}

export function formatResult(result: Object): string {
	if (typeof result === 'string') return Chalk.bold.green(result);
	if (typeof result === 'number') return Chalk.bold.cyan(result.toString());
	if (typeof result === 'boolean') return Chalk.magenta(result.toString());
	if (result == null) {
		if (ErrorReporter.hadError) return Chalk.red('Error');
		return Chalk.magenta('null');
	}
}
