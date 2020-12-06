import * as Util from 'util';
import * as Chalk from 'chalk';

export function formatAst(ast: any): string {
	return Util.inspect(ast, { depth: 20, colors: true })
		.replace(/[A-Z][a-zA-Z]+(?= {)/g, Chalk.bold.yellow('$&'))
		.replace(/[a-z][a-zA-Z]+(?=:)/g, Chalk.redBright('$&'));
}
