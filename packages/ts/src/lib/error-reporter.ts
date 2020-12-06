import * as Chalk from 'chalk';
import { Token, TokenType } from './types';

export class ErrorReporter {
	static hadError: boolean;
	static runtimeError: boolean;

	static error(err: Error): void;
	static error(token: Token, message?: string): void;
	static error(line: number, message?: string): void;

	static error(err: Error | Token | number, message?: string): void {
		this.runtimeError = err instanceof Error;
		if (err instanceof Error) {
			this.report(
				(err as any).token.line,
				` at '${(err as any).token.lexeme}'`,
				err.message,
			);
		} else if (err instanceof Token) {
			if (err.type === TokenType.EOF) {
				this.report(err.line, ' at end', message);
			} else {
				this.report(err.line, ` at '${err.lexeme}'`, message);
			}
		} else {
			this.report(err, '', message);
		}
	}

	private static report(line: number, where: string, message: string): void {
		let err = Chalk.bold.red(`[line ${line}] Error${where}: ${message}`);
		console.log(err);

		this.hadError = true;
	}
}
