import { Environment } from './environment';
import { Interpreter, Return } from './interpreter';
import { Invokable } from './types';
import { Fn } from './types/stmt';

export class FnObject implements Invokable {
	private readonly declaration: Fn;
	private readonly closure: Environment;

	constructor(declaration: Fn, closure: Environment) {
		this.declaration = declaration;
		this.closure = closure;
	}

	arity(): number {
		return this.declaration.params.length;
	}
	invoke(interpreter: Interpreter, ...args: Object[]): Object {
		let env = new Environment(this.closure);
		args.forEach((arg, i) => {
			env.define(this.declaration.params[i].lexeme, arg);
		});
		try {
			interpreter.executeBlock(this.declaration.body, env);
		} catch (err) {
			if (err instanceof Return) {
				return err.value;
			}
			throw err;
		}
		return null;
	}
	toString(): string {
		return `<fn ${this.declaration.name.lexeme}>`;
	}
}
