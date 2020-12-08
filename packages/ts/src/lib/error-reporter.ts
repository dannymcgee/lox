import * as Chalk from 'chalk';
import { RuntimeError } from './interpreter';
import { Line, Token } from './types';

export class ErrorReporter {
	static hadError: boolean;
	static runtimeError: boolean;

	static error(err: RuntimeError): void;
	static error(token: Token, message: string): void;

	static error(err: RuntimeError | Token, message?: string): void {
		this.runtimeError = err instanceof RuntimeError;
		if (err instanceof RuntimeError) {
			this.report(err.token.line, err.token, err.message);
		} else {
			this.report(err.line, err, message);
		}
	}

	private static report(line: Line, where: Token, message: string): void {
		// Print the error message
		console.log(Chalk.bold.redBright(`ERROR: ${message}`));

		// Print the offending line with the error location highlighted
		let { content } = line;
		let index = where.start - line.start;
		let tokenLength = where?.lexeme?.length ?? 1;
		let annotated = [
			Chalk.grey(line.lineNumber),
			' ',
			content.substring(0, index),
			Chalk.bold.redBright(content.substr(index, tokenLength)),
			content.substring(index + tokenLength),
		]
			.join('')
			.replace(/\t/g, '  ')
			.replace(/\r\n/g, '');

		console.log(annotated);

		// Print a caret pointing to the error location
		let indents = content.match(/\t/g)?.length ?? 0;
		let indentsOffset = Array(indents).fill('  ').join('');
		let lineNumberOffset =
			line.lineNumber
				.toString(10)
				.split('')
				.map(() => ' ')
				.join('') + ' ';
		let pointerOffset = Array(index - indents)
			.fill(' ')
			.join('');

		// prettier-ignore
		console.log([
			lineNumberOffset,
			indentsOffset,
			pointerOffset,
			Chalk.bold.redBright('^'),
		].join(''));

		this.hadError = true;
	}
}
