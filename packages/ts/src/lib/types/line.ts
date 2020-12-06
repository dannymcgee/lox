export class Line {
	readonly lineNumber: number;
	readonly start: number;
	end: number;
	content: string;

	constructor(
		lineNumber: number,
		start: number,
		end?: number,
		content?: string,
	) {
		this.lineNumber = lineNumber;
		this.start = start;
		this.end = end;
		this.content = content;
	}
}
