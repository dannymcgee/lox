import { parse } from '@lox/cli-tools';
import { AstGenerator } from './lib/generate-ast';

let [path, dryRun] = process.argv.slice(2);
let normalized = {
	path: parse(String, path),
	dryRun: parse(Boolean, dryRun),
};

AstGenerator.main(normalized);
