import { Binary, Expr, Grouping, Literal, Unary, Visitor } from './types';

export class AstDebugger implements Partial<Visitor<string>> {
	print(expr: Expr): string {
		return expr.accept(this);
	}
	visitBinaryExpr(expr: Binary): string {
		return this.parenthesize(expr.operator.lexeme, expr.left, expr.right);
	}
	visitGroupingExpr(expr: Grouping): string {
		return this.parenthesize('group', expr.expression);
	}
	visitLiteralExpr(expr: Literal): string {
		if (expr.value == null) return 'nil';
		return expr.value.toString();
	}
	visitUnaryExpr(expr: Unary): string {
		return this.parenthesize(expr.operator.lexeme, expr.right);
	}
	private parenthesize(name: string, ...exprs: Expr[]): string {
		return `(${name} ${exprs.map((expr) => expr.accept(this))})`;
	}
}
