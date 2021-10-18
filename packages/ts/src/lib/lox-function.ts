import { Environment } from './environment';
import { Interpreter, Return } from './interpreter';
import * as Expr from './types/expr';
import { Invokable } from './types';
import { LoxInstance } from './lox-class';

export class LoxFunction implements Invokable {
	readonly name?: string;
	private readonly func: Expr.Fun;
	private readonly closure: Environment;

	constructor(func: Expr.Fun, closure: Environment, name?: string) {
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

		if (this.name === 'init') return this.closure.getAt(0, 'this');
		return null;
	}

	bind(instance: LoxInstance): LoxFunction {
		let env = new Environment(this.closure);
		env.define('this', instance);
		return new LoxFunction(this.func, env, this.name);
	}

	toString(): string {
		if (this.name) return `<fun ${this.name}>`;
		return `<fun>`;
	}
}
