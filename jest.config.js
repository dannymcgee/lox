const { getJestProjects } = require('@nrwl/jest');

module.exports = {
	projects: [
		...getJestProjects(),
		'<rootDir>/packages/vscode-lox',
		'<rootDir>/packages/ts',
		'<rootDir>/packages/tools',
		'<rootDir>/packages/cli-tools',
	],
};
