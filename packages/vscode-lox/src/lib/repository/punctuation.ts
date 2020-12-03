import { TMGrammarScope } from '../types';

export const punctuation: TMGrammarScope = {
	patterns: [
		{
			match: /[{}]/,
			name: 'punctuation.brace.curly.lox',
		},
		{
			match: /[()]/,
			name: 'punctuation.brace.round.lox',
		},
		{
			match: /,/,
			name: 'punctuation.separator.comma.lox',
		},
		{
			match: /;/,
			name: 'punctuation.terminator.semicolon.lox',
		},
		{
			match: /\./,
			name: 'punctuation.accessor.lox',
		},
	],
};
