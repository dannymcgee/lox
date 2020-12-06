import * as Chalk from 'chalk';
import { Token, TokenType } from './types';

export class ErrorReporter {
	static hadError: boolean;

	static error(token: Token, message: string): void;
	static error(line: number, message: string): void;
	static error(lineOrToken: Token | number, message: string): void {
		if (lineOrToken instanceof Token) {
			if (lineOrToken.type === TokenType.EOF) {
				this.report(lineOrToken.line, ' at end', message);
			} else {
				this.report(
					lineOrToken.line,
					` at '${lineOrToken.lexeme}'`,
					message,
				);
			}
		} else {
			this.report(lineOrToken, '', message);
		}
	}

	private static report(line: number, where: string, message: string): void {
		let err = Chalk.bold.red(`[line ${line}] Error${where}: ${message}`);
		console.log(err);

		this.hadError = true;
	}
}
