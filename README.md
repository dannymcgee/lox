# Lox

Yet another Lox implementation (from the excellent book [Crafting Interpreters](https://craftinginterpreters.com/))

## Implementations

-   Tree-Walk interpreter in TypeScript (because it's my comfort zone) – **Complete**
-   Bytecode VM in Rust (started almost a year ago in a naive attempt to learn the language from scratch — ha! Revisiting now that I'm a battle-hardened Rustacean) **~25% done**

## Misc

-   Simple but accurate VS Code grammar for syntax highlighting available under `packages/vscode-lox`

## Running the Project

### Dependencies

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

### TypeScript AST-Walker

#### Start the REPL

```sh
# nx start ts
# Just kidding, this is broken
```

#### Run one of the example files

```sh
nx start ts --example <name> # e.g., nx start ts --example fizzbuzz2
```

#### Generate AST types

Modify the definitions in `packages/tools/src/lib/generate-ast.ts` as necessary, then run:

```sh
nx run tools:generate
```

### Rust VM

#### Start the REPL

```sh
# Only partially implemented so far
nx start vm  # --debug parse,codegen,exec
```

#### Run one of the example files

```sh
# Not yet implemented
nx start vm --example <nam>  # e.g., nx start vm --example fizzbuzz2
```

## Build the VS Code grammar

```sh
yarn run package:vscode-lox
```

The VSIX installer will be under `dist/packages/vscode-lox/`
