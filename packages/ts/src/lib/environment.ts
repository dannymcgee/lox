import { RuntimeError } from './interpreter';
import { Token } from './types';

export class Environment {
	readonly enclosing?: Environment;
	private readonly values = new Map<string, Object>();

	constructor(enclosing?: Environment) {
		this.enclosing = enclosing;
	}

	define(name: string, value: Object): void {
		this.values.set(name, value);
	}
	assign(name: Token, value: Object): void {
		if (this.values.has(name.lexeme)) this.values.set(name.lexeme, value);
		else if (this.enclosing) this.enclosing.assign(name, value);
		else
			throw new RuntimeError(
				name,
				`Undefined variable '${name.lexeme}'.`,
			);
	}
	get(name: Token): Object {
		if (this.values.has(name.lexeme)) return this.values.get(name.lexeme);
		if (this.enclosing) return this.enclosing.get(name);

		throw new RuntimeError(name, `Undefined variable '${name.lexeme}'.`);
	}
}
