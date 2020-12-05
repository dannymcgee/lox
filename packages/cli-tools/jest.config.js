module.exports = {
	displayName: 'cli-tools',
	preset: '../../jest.preset.js',
	globals: {
		'ts-jest': {
			tsConfig: '<rootDir>/tsconfig.spec.json',
		},
	},
	testEnvironment: 'node',
	transform: {
		'^.+\\.[tj]sx?$': 'ts-jest',
	},
	moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
	coverageDirectory: '../../coverage/packages/cli-tools',
};
