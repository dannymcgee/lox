import { TMGrammarScope } from '../types';

export const identifiers: TMGrammarScope = {
	patterns: [
		// class
		{
			match: /\b(?<=class)\s+([a-zA-Z]+)(?:\s+(<)\s+([a-zA-Z]+))?\b/,
			captures: {
				1: { name: 'entity.name.class.lox' },
				2: { name: 'keyword.operator.inheritance.lox' },
				3: { name: 'entity.name.class.lox' },
			},
		},
		// function
		{
			match: /\b(?<=fn)\s+([a-zA-Z]+)\b/,
			captures: {
				1: { name: 'entity.name.function' },
			},
		},
		{
			match: /\b[a-zA-Z]+(?=\s*\()\b/,
			name: 'entity.name.function',
		},
		// property
		{
			match: /(?<=\.)[a-zA-Z]+\b/,
			name: 'variable.object.property.lox',
		},
		// generic variable
		{
			match: /\b[a-zA-Z]+\b/,
			name: 'variable.lox',
		},
	],
};
