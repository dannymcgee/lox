import { Environment } from './environment';
import { Interpreter } from './interpreter';
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
		interpreter.executeBlock(this.declaration.body, env);

		return null;
	}
	toString(): string {
		return `<fn ${this.declaration.name.lexeme}>`;
	}
}
