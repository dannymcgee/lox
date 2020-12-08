import { Environment } from './environment';
import { Interpreter, Return } from './interpreter';
import * as Expr from './types/expr';
import { Invokable } from './types';

export class FnObject implements Invokable {
	private readonly func: Expr.Fn;
	private readonly closure: Environment;
	private readonly name?: string;

	constructor(func: Expr.Fn, closure: Environment, name?: string) {
		this.func = func;
		this.closure = closure;
		this.name = name;
	}

	arity(): number {
		return this.func.params.length;
	}
	invoke(interpreter: Interpreter, ...args: Object[]): Object {
		let env = new Environment(this.closure);
		args.forEach((arg, i) => {
			env.define(this.func.params[i].lexeme, arg);
		});
		try {
			interpreter.executeBlock(this.func.body, env);
		} catch (err) {
			if (err instanceof Return) {
				return err.value;
			}
			throw err;
		}
		return null;
	}
	toString(): string {
		if (this.name) return `<fn ${this.name}>`;
		return `<fn>`;
	}
}
