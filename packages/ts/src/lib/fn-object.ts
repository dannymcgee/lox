import { Environment } from './environment';
import { Interpreter, Return } from './interpreter';
import { Invokable } from './types';
import { Fn } from './types/stmt';

export class FnObject implements Invokable {
	private readonly declaration: Fn;

	constructor(declaration: Fn) {
		this.declaration = declaration;
	}

	arity(): number {
		return this.declaration.params.length;
	}
	invoke(interpreter: Interpreter, ...args: Object[]): Object {
		let env = new Environment(interpreter.globals);
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
