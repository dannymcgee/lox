import * as Path from 'path';
import * as FS from 'fs-extra';
import * as Chalk from 'chalk';
import { createInterface, ReadLine } from 'readline';

import { Scanner } from './lib/scanner';
import { ErrorReporter } from './lib/error-reporter';

class Lox {
	static main(args: string[]) {
		let normalized = args
			.slice(2)
			.map((arg) => arg?.trim())
			.filter(Boolean)
			.filter((arg) => arg !== 'undefined');

		if (normalized.length > 1) {
			console.log('Usage: tslox [script]');
			process.exit(64);
		} else if (normalized.length === 1) {
			console.log('args:', args);
			this.runFile(normalized[0]);
		} else {
			this.runPrompt();
		}
	}

	private static _rl?: ReadLine;
	private static get rl(): ReadLine {
		if (!this._rl) {
			this._rl = createInterface({
				input: process.stdin,
				output: process.stdout,
			});
		}
		return this._rl;
	}

	private static runFile(path: string) {
		let resolved = Path.resolve(process.cwd(), path);
		let content = FS.readFileSync(resolved);
		let prefix = Chalk.bold('Running file:');
		console.clear();
		console.log(`${prefix} ${resolved}`);

		let result = this.run(content);
		if (ErrorReporter.hadError) process.exit(65);

		this.print(result);
	}

	private static runPrompt(previousResult?: string) {
		ErrorReporter.hadError = false;

		if (!previousResult) console.clear();
		console.log(Chalk.bold('Enter some Lox code to run it'));
		if (previousResult) this.print(previousResult);
		console.log('');

		this.rl.question(Chalk.bold.blue('> '), (input) => {
			console.clear();
			let result = this.run(input);

			this.runPrompt(result);
		});
	}

	private static print(content: string) {
		let prefix = Chalk.bold.cyan('=>');

		console.log(`${prefix} ${Chalk.gray(content)}`);
	}

	private static run(source: Buffer | string): string {
		let scanner = new Scanner(source);
		let tokens = scanner.scanTokens();

		return tokens.join('\n   ');
	}
}

Lox.main(process.argv);
