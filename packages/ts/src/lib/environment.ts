import { RuntimeError } from './interpreter';
import { Token } from './types';

export class Environment {
	private readonly values = new Map<string, Object>();

	define(name: string, value: Object): void {
		this.values.set(name, value);
	}
	assign(name: Token, value: Object): void {
		if (!this.values.has(name.lexeme))
			throw new RuntimeError(
				name,
				`Undefined variable '${name.lexeme}'.`,
			);

		this.values.set(name.lexeme, value);
	}
	get(name: Token): Object {
		if (!this.values.has(name.lexeme))
			throw new RuntimeError(
				name,
				`Undefined variable '${name.lexeme}'.`,
			);

		return this.values.get(name.lexeme);
	}
}
