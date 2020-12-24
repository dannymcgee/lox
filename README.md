# Lox

Yet another Lox implementation (from the excellent book [Crafting Interpreters](https://craftinginterpreters.com/))

## Implementations

-   Tree-Walk interpreter in TypeScript (because it's my comfort zone) – **Complete**
-   Bytecode VM in Rust (because C is annoying and I've been itching to learn a modern systems language) **~6% done**

## Misc

-   Simple but accurate VS Code grammar for syntax highlighting available under `packages/vscode-lox`

## Running the project

The workspace is managed through [Nx](https://nx.dev/). Globally install the `nx` Node package for the best experience:

```sh
yarn global add nx
# or
npm i -g nx
```

Then install dependencies:

```sh
yarn
# or
npm i
```

### Run some Lox code from terminal input

```sh
nx run ts:start
```

### Run one of the example files

```sh
nx run ts:start --example <name> # e.g., nx start ts --example fizzbuzz2
```

### Generate AST types

Modify the definitions in `packages/tools/src/lib/generate-ast.ts` as necessary, then run:

```sh
nx run tools:generate
```

### Build the VS Code grammar

```sh
yarn run package:vscode-lox
```

The VSIX installer will be under `dist/packages/vscode-lox/`
