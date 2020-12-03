import { expressions, keywords } from './repository';
import { identifiers } from './repository/identifiers';
import { punctuation } from './repository/punctuation';
import { TMGrammar } from './types';

const grammar: TMGrammar = {
	name: 'Lox',
	scopeName: 'source.lox',
	patterns: [{ include: '#statements' }],
	repository: {
		expressions: { ...expressions },
		keywords: { ...keywords },
		punctuation: { ...punctuation },
		identifiers: { ...identifiers },
		statements: {
			patterns: [{ include: '#expressions' }],
		},
	},
};

export default grammar;
