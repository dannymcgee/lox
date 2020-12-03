import { TMGrammarScope } from '../types';

export const keywords: TMGrammarScope = {
	patterns: [
		{
			match: /\b(print|var|class|fn|and|or)\b/,
			name: 'keyword.$1.lox',
		},
		{
			match: /\b(if|else|while|for|return)\b/,
			name: 'keyword.control.$1.lox',
		},
		{
			match: /\b(this|super)\b/,
			name: 'variable.language.this.lox',
		},
		// Operators
		{
			match: /\+\+/,
			name: 'keyword.operator.increment.lox',
		},
		{
			match: /--/,
			name: 'keyword.operator.decrement.lox',
		},
		{
			match: /[-+*/]/,
			name: 'keyword.operator.arithmetic.lox',
		},
		{
			match: /=/,
			name: 'keyword.operator.assignment.lox',
		},
		{
			match: /[=!<>]{1,2}/,
			name: 'keyword.operator.comparison.lox',
		},
	],
};
