import { TMGrammarScope } from '../types';

export const expressions: TMGrammarScope = {
	patterns: [
		{
			// Comments
			begin: /\/\//,
			beginCaptures: {
				0: { name: 'punctuation.definition.comment.lox' },
			},
			end: /\n|\r/,
			name: 'comment.lox',
		},
		{
			// String literal
			begin: /"/,
			beginCaptures: {
				0: { name: 'punctuation.definition.string.begin.lox' },
			},
			end: /(?<!\\)"/,
			endCaptures: {
				0: { name: 'punctuation.definition.string.end.lox' },
			},
			name: 'string.lox',
		},
		{
			// Number literal
			match: /\b(\d+)(?:(\.)(\d+))*\b/,
			captures: {
				1: { name: 'constant.numeric.lox' },
				3: { name: 'constant.numeric.lox' },
				2: { name: 'punctuation.separator.decimal.lox' },
			},
		},
		// Language constants
		{
			match: /\bnil\b/,
			name: 'constant.language.null.lox',
		},
		{
			match: /\b(true|false)\b/,
			name: 'constant.language.boolean.$1.lox',
		},
		{ include: '#keywords' },
		{ include: '#identifiers' },
		{ include: '#punctuation' },
	],
};
