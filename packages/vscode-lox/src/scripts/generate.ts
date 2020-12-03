import * as fs from 'fs-extra';
import * as Path from 'path';
import * as chalk from 'chalk';
import { JsonObject, TMGrammar, TMGrammarScope } from '../lib/types';
import grammar from '../lib/vscode-lox';

const processGrammar = (
	grammar: TMGrammar | TMGrammarScope | TMGrammarScope[],
): JsonObject => {
	let processed: JsonObject = {};

	// prettier-ignore
	for (let [key, value] of Object.entries(grammar)) {
		if (typeof value === 'string')
			processed[key] = value;
		else if (value instanceof RegExp)
			processed[key] = value.toString().replace(/^\/|\/$/g, '');
		else if (value instanceof Array)
			processed[key] = value.map(processGrammar).filter(Boolean);
		else
			processed[key] = processGrammar(value) as any;
	}

	return processed;
};

async function generate(grammar: TMGrammar, name: string) {
	let processed = processGrammar(grammar);
	let content = JSON.stringify(processed, null, '\t');
	let dirname = Path.resolve(
		process.cwd(),
		'dist/packages/vscode-lox/grammars',
	);

	try {
		fs.ensureDirSync(dirname);
		fs.writeFileSync(
			Path.resolve(dirname, `${name}.tmLanguage.json`),
			content,
		);
		fs.copyFileSync(
			'packages/vscode-lox/src/lib/language-configuration.json',
			Path.resolve(dirname, 'language-configuration.json'),
		);
		console.log(chalk.bold.greenBright('Done!'));
	} catch (err) {
		console.log(chalk.bold.redBright(err.toString()));
		throw err;
	}
}

generate(grammar, 'lox');
