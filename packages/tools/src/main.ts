import { AstGenerator } from './lib/generate-ast';

let normalized = process.argv
	.slice(2)
	.map((arg) => arg?.trim())
	.filter(Boolean)
	.filter((arg) => arg !== 'undefined');

AstGenerator.main(...normalized);
