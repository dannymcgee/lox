import { Interpreter, RuntimeError } from './interpreter';
import { LoxFunction } from './lox-function';
import { Invokable, Token } from './types';

export class LoxClass implements Invokable {
	readonly name: string;
	readonly methods: Map<string, LoxFunction>;
	readonly superclass?: LoxClass;

	constructor(
		name: string,
		methods: Map<string, LoxFunction>,
		superclass?: LoxClass,
	) {
		this.name = name;
		this.methods = methods;
		this.superclass = superclass;
	}

	method(name: string): LoxFunction | null {
		if (this.methods.has(name)) return this.methods.get(name);
		return this.superclass?.method(name);
	}

	invoke(interpreter: Interpreter, ...args: Object[]): Object {
		let instance: LoxInstance = new LoxInstance(this);
		this.method('init')
			?.bind(instance)
			.invoke(interpreter, ...args);

		return instance;
	}

	arity(): number {
		let init = this.method('init');
		return init?.arity() ?? 0;
	}

	toString(): string {
		return `<class ${this.name}>`;
	}
}

export class LoxInstance {
	readonly proto: LoxClass;
	private readonly fields = new Map<string, Object>();

	constructor(proto: LoxClass) {
		this.proto = proto;
	}

	get(prop: Token): Object {
		if (this.fields.has(prop.lexeme)) return this.fields.get(prop.lexeme);

		let method = this.proto.method(prop.lexeme);
		if (method) return method.bind(this);

		throw new RuntimeError(prop, `Undefined property '${prop.lexeme}'.`);
	}
	set(prop: Token, value: Object): void {
		this.fields.set(prop.lexeme, value);
	}

	toString(): string {
		return `<class ${this.proto.name} instance>`;
	}
}
