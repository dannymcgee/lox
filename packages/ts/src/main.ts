import * as Path from 'path';
import * as Util from 'util';
import * as FS from 'fs-extra';
import * as Chalk from 'chalk';
import { createInterface, ReadLine } from 'readline';

Util.inspect.styles = {
	bigint: 'cyan',
	boolean: 'magenta',
	date: 'orange',
	module: 'orange',
	null: 'magenta',
	number: 'cyan',
	regexp: 'orange',
	special: 'magenta',
	string: 'green',
	symbol: 'magenta',
	undefined: 'magenta',
};

import { Scanner } from './lib/scanner';
import { ErrorReporter } from './lib/error-reporter';
import { Parser } from './lib/parser';
import { Interpreter, Return, RuntimeError } from './lib/interpreter';
import { Resolver } from './lib/resolver';
import { formatAst } from './lib/debug';

class Lox {
	static main(args: string[]): void {
		let normalized = args
			.slice(2)
			.map((arg) => arg?.trim())
			.filter(Boolean)
			.filter((arg) => arg !== 'undefined');

		if (normalized.length > 1) {
			console.log('Usage: tslox [script]');
			process.exit(64);
		} else if (normalized.length === 1) {
			// console.log('args:', args);
			this.runFile(normalized[0]);
		} else {
			this.runPrompt();
		}
	}

	private static interpreter = new Interpreter();

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
		let prefix = Chalk.bold('Running file:');
		console.clear();
		console.log(`${prefix} ${resolved}`);

		let result = this.run(content);
		if (ErrorReporter.hadError) {
			if (ErrorReporter.runtimeError) process.exit(70);
			else process.exit(65);
		}

		this.print(result);
	}

	private static runPrompt(previousResult?: string): void {
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

	private static print(content: string): void {
		let prefix = Chalk.bold.cyan('=>');

		console.log(`${prefix} ${Chalk.gray(content)}`);
	}

	private static run(source: Buffer | string): string {
		let output: string;
		try {
			let { tokens } = new Scanner(source).scan();
			let ast = new Parser(tokens).parse();
			if (ErrorReporter.hadError) return; // Parse error

			new Resolver(this.interpreter).resolve(ast);
			if (ErrorReporter.hadError) return; // Resolve error

			// return formatAst(ast).replace(/\n/g, '\n   ');
			this.interpreter.interpret(ast);
			output = Chalk.bold.inverse.greenBright(' DONE ');
		} catch (err) {
			if (err instanceof RuntimeError || err instanceof Return)
				return Chalk.bold.redBright(' ERROR ');
			throw err;
		}
		return output;
	}
}

Lox.main(process.argv);
