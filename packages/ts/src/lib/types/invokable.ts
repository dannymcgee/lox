import { Interpreter } from '../interpreter';

export interface Invokable {
	invoke(interpreter: Interpreter, ...args: Object[]): Object;
	arity(): number;
}

export function isInvokable(obj: unknown): obj is Invokable {
	return (
		typeof obj === 'object' &&
		'invoke' in obj &&
		typeof (obj as any).invoke === 'function' &&
		'arity' in obj &&
		typeof (obj as any).arity === 'function'
	);
}
