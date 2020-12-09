import { ErrorReporter } from './error-reporter';
import { Interpreter } from './interpreter';
import { Token } from './types';
import * as Expr from './types/expr';
import * as Stmt from './types/stmt';

enum FunctionType {
	None,
	Function,
	Method,
	Constructor,
}
enum ClassType {
	None,
	Class,
	Subclass,
}

export class Resolver implements Stmt.Visitor<void>, Expr.Visitor<void> {
	private readonly interpreter: Interpreter;
	private readonly scopes: Stack<Map<string, boolean>> = new Stack();
	private currentFunction: FunctionType = FunctionType.None;
	private currentClass: ClassType = ClassType.None;

	constructor(interpreter: Interpreter) {
		this.interpreter = interpreter;
	}

	resolve(expression: Expr.Expr): void;
	resolve(statement: Stmt.Stmt): void;
	resolve(statements: readonly Stmt.Stmt[]): void;

	resolve(statements: readonly Stmt.Stmt[] | Stmt.Stmt | Expr.Expr): void {
		if (statements instanceof Array)
			for (let stmt of statements) this.resolve(stmt);
		else (statements as any).accept(this);
	}
	private resolveLocal(expr: Expr.Expr, name: Token): void {
		for (let i = this.scopes.size - 1; i >= 0; i--) {
			if (this.scopes.get(i).has(name.lexeme)) {
				this.interpreter.resolve(expr, this.scopes.size - 1 - i);
			}
		}
	}
	private resolveFunction(fn: Expr.Fn, type: FunctionType): void {
		let enclosing = this.currentFunction;
		this.currentFunction = type;

		this.beginScope();
		for (let param of fn.params) {
			this.declare(param);
			this.define(param);
		}
		this.resolve(fn.body);
		this.endScope();

		this.currentFunction = enclosing;
	}

	visitBlockStmt(stmt: Stmt.Block): void {
		this.beginScope();
		this.resolve(stmt.statements);
		this.endScope();
	}
	visitClassStmt(stmt: Stmt.Class): void {
		let enclosing = this.currentClass;
		this.currentClass = ClassType.Class;

		this.declare(stmt.name);
		this.define(stmt.name);

		if (stmt.superclass) {
			if (stmt.superclass.name.lexeme === stmt.name.lexeme)
				ErrorReporter.error(
					stmt.superclass.name,
					`A class cannot inherit from itself.`,
				);
			else {
				this.currentClass = ClassType.Subclass;
				this.resolve(stmt.superclass);
				this.beginScope();
				this.scopes.peek().set('super', true);
			}
		}

		this.beginScope();
		this.scopes.peek().set('this', true);
		for (let method of stmt.methods) {
			let declaration =
				method.name.lexeme === 'init'
					? FunctionType.Constructor
					: FunctionType.Method;
			this.resolveFunction(method.func, declaration);
		}
		this.endScope();
		if (stmt.superclass) this.endScope();

		this.currentClass = enclosing;
	}
	visitExpressionStmt(stmt: Stmt.Expression): void {
		this.resolve(stmt.expression);
	}
	visitFnStmt(stmt: Stmt.Fn): void {
		this.declare(stmt.name);
		this.define(stmt.name);
		this.resolveFunction(stmt.func, FunctionType.Function);
	}
	visitIfStmt(stmt: Stmt.If): void {
		this.resolve(stmt.condition);
		this.resolve(stmt.thenBranch);
		if (stmt.elseBranch) this.resolve(stmt.elseBranch);
	}
	visitPrintStmt(stmt: Stmt.Print): void {
		this.resolve(stmt.expression);
	}
	visitReturnStmt(stmt: Stmt.Return): void {
		if (this.currentFunction === FunctionType.None)
			ErrorReporter.error(
				stmt.keyword,
				`Cannot return from outside of a function.`,
			);
		if (stmt.value) {
			if (this.currentFunction === FunctionType.Constructor)
				ErrorReporter.error(
					stmt.keyword,
					`Class constructors cannot return a value.`,
				);
			this.resolve(stmt.value);
		}
	}
	visitVarStmt(stmt: Stmt.Var): void {
		this.declare(stmt.name);
		if (stmt.initializer) {
			this.resolve(stmt.initializer);
		}
		this.define(stmt.name);
	}
	visitWhileStmt(stmt: Stmt.While): void {
		this.resolve(stmt.condition);
		this.resolve(stmt.body);
	}

	visitAssignExpr(expr: Expr.Assign): void {
		this.resolve(expr.value);
		this.resolveLocal(expr, expr.name);
	}
	visitBinaryExpr(expr: Expr.Binary): void {
		this.resolve(expr.left);
		this.resolve(expr.right);
	}
	visitFnExpr(expr: Expr.Fn): void {
		this.resolveFunction(expr, FunctionType.Function);
	}
	visitCallExpr(expr: Expr.Call): void {
		this.resolve(expr.callee);
		for (let arg of expr.args) this.resolve(arg);
	}
	visitGetExpr(expr: Expr.Get): void {
		this.resolve(expr.obj);
	}
	visitGroupingExpr(expr: Expr.Grouping): void {
		this.resolve(expr.expression);
	}
	visitLiteralExpr(expr: Expr.Literal): void {}
	visitLogicalExpr(expr: Expr.Logical): void {
		this.resolve(expr.left);
		this.resolve(expr.right);
	}
	visitSetExpr(expr: Expr.Set): void {
		this.resolve(expr.value);
		this.resolve(expr.obj);
	}
	visitSuperExpr(expr: Expr.Super): void {
		if (this.currentClass === ClassType.None)
			ErrorReporter.error(
				expr.keyword,
				`Cannot use 'super' outside of a class.`,
			);
		else if (this.currentClass !== ClassType.Subclass)
			ErrorReporter.error(
				expr.keyword,
				`Cannot use 'super' in a base class`,
			);
		this.resolveLocal(expr, expr.keyword);
	}
	visitThisExpr(expr: Expr.This): void {
		if (this.currentClass === ClassType.None) {
			ErrorReporter.error(
				expr.keyword,
				`Cannot use 'this' outside of a class.`,
			);
			return;
		}
		this.resolveLocal(expr, expr.keyword);
	}
	visitUnaryExpr(expr: Expr.Unary): void {
		this.resolve(expr.right);
	}
	visitVariableExpr(expr: Expr.Variable): void {
		let scope = this.scopes.peek();
		if (scope?.has(expr.name.lexeme) && !scope.get(expr.name.lexeme)) {
			ErrorReporter.error(
				expr.name,
				`Cannot read local variable in its own initializer.`,
			);
		}
		this.resolveLocal(expr, expr.name);
	}

	private beginScope(): void {
		this.scopes.push(new Map());
	}
	private endScope(): void {
		this.scopes.pop();
	}
	private declare(token: Token): void {
		let scope = this.scopes.peek();
		if (scope?.has(token.lexeme))
			ErrorReporter.error(
				token,
				`Cannot redeclare identifier '${token.lexeme}'`,
			);
		scope?.set(token.lexeme, false);
	}
	private define(token: Token): void {
		this.scopes.peek()?.set(token.lexeme, true);
	}
}

class Stack<T> {
	get isEmpty(): boolean {
		return this.data.length === 0;
	}
	get size(): number {
		return this.data.length;
	}
	private data = [];

	get(index: number) {
		return this.data[index];
	}
	peek(): T {
		if (this.isEmpty) return;
		return this.data[this.data.length - 1];
	}
	push(item: T): void {
		this.data.push(item);
	}
	pop(): T {
		return this.data.pop();
	}
}
