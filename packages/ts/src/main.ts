import * as Path from 'path';
import * as FS from 'fs-extra';
import * as Chalk from 'chalk';
import { createInterface, ReadLine } from 'readline';

class Lox {
	public static main(args: string[]): void {
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

	private static runFile(path: string): void {
		let resolved = Path.resolve(process.cwd(), path);
		let content = FS.readFileSync(resolved);
		let result = this.run(content);
		let prefix = Chalk.bold('Running file:');
		console.clear();
		console.log(`${prefix} ${resolved}`);

		this.print(result);
	}

	private static print(content: string): void {
		let prefix = Chalk.bold.cyan('=>');

		console.log(`${prefix} ${Chalk.gray(content)}`);
	}

	private static runPrompt(previousResult?: string): void {
		console.clear();
		console.log(Chalk.bold('Enter some Lox code to run it'));
		if (previousResult) {
			this.print(previousResult);
		}
		console.log('');

		this.rl.question(Chalk.bold.blue('> '), (input) => {
			let result = this.run(input);

			this.runPrompt(result);
		});
	}

	private static run(script: Buffer | string): string {
		return script.toString();
	}
}

Lox.main(process.argv);
