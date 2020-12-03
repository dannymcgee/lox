import * as Path from 'path';
import * as FS from 'fs-extra';
import * as Chalk from 'chalk';
import { createInterface, ReadLine } from 'readline';

import { Scanner } from './lib/scanner';

class Lox {
	static hadError: boolean;

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
		if (this.hadError) process.exit(65);

		this.print(result);
	}

	private static runPrompt(previousResult?: string) {
		console.clear();
		console.log(Chalk.bold('Enter some Lox code to run it'));
		if (previousResult) {
			this.print(previousResult);
		}
		console.log('');

		this.rl.question(Chalk.bold.blue('> '), (input) => {
			let result = this.run(input);
			this.hadError = false;

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

		return tokens.join('\n');
	}

	private static error(line: number, message: string) {
		this.report(line, '', message);
	}

	private static report(line: number, where: string, message: string) {
		let err = Chalk.bold.red(`[line ${line}] Error${where}: ${message}`);
		console.log(err);

		this.hadError = true;
	}
}

Lox.main(process.argv);
