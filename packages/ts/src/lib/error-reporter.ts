import * as Chalk from 'chalk';

export class ErrorReporter {
	static hadError: boolean;

	static error(line: number, message: string) {
		this.report(line, '', message);
	}

	private static report(line: number, where: string, message: string) {
		let err = Chalk.bold.red(`[line ${line}] Error${where}: ${message}`);
		console.log(err);

		this.hadError = true;
	}
}
