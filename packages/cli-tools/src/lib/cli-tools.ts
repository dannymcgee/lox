type TypeHint = typeof String | typeof Number | typeof Boolean;
// prettier-ignore
type Constructed<T extends TypeHint>
	= T extends typeof String ? string
	: T extends typeof Number ? number
	: T extends typeof Boolean ? boolean
	: never;

export function parse<T extends TypeHint>(
	typeHint: T,
	arg: string,
): Constructed<T> | undefined {
	if (arg === 'undefined') return undefined;
	return typeHint(arg) as Constructed<T>;
}
