import { Operation, type DiffArray, charDiff } from './diff';

export interface Token {
	text: string;
	className: string;
}

export interface Row {
	originalLineNumber: number;
	currentLineNumber: number;
	tokens: Token[];
	type: RowType;
}

export const enum RowType {
	Deletion = 'deletion',
	Addition = 'addition',
	Equal = 'equal',
	Spacer = 'spacer'
}

export function buildDiffRows(diff: DiffArray, paddingLines = 10000): {
	originalLines: readonly string[];
	currentLines: readonly string[];
	rows: readonly Row[];
} {
	let currentLineNumber = 0;
	let originalLineNumber = 0;

	const originalLines: string[] = [];
	const currentLines: string[] = [];
	const rows: Row[] = [];

	for (let i = 0; i < diff.length; ++i) {
		const token = diff[i];
		switch (token[0]) {
			case Operation.Equal:
				rows.push(...createEqualRows(token[1], i === 0, i === diff.length - 1));
				originalLines.push(...token[1]);
				currentLines.push(...token[1]);
				break;
			case Operation.Insert:
				for (const line of token[1]) {
					rows.push(createRow(line, RowType.Addition));
				}
				currentLines.push(...token[1]);
				break;
			case Operation.Delete:
				originalLines.push(...token[1]);
				if (diff[i + 1] && diff[i + 1][0] === Operation.Insert) {
					i++;
					rows.push(...createModifyRows(token[1].join('\n'), diff[i][1].join('\n')));
					currentLines.push(...diff[i][1]);
				} else {
					for (const line of token[1]) {
						rows.push(createRow(line, RowType.Deletion));
					}
				}
				break;
		}
	}

	return { originalLines, currentLines, rows };

	function createEqualRows(lines: string[], atStart: boolean, atEnd: boolean): Row[] {
		const equalRows = [];
		if (!atStart) {
			for (let i = 0; i < paddingLines && i < lines.length; i++) {
				equalRows.push(createRow(lines[i], RowType.Equal));
			}
			if (lines.length > paddingLines * 2 + 1 && !atEnd) {
				equalRows.push(
					createRow(`skipping ${lines.length - paddingLines * 2} matching lines`, RowType.Spacer)
				);
			}
		}
		if (!atEnd) {
			const start = Math.max(lines.length - paddingLines - 1, atStart ? 0 : paddingLines);
			let skip = lines.length - paddingLines - 1;
			if (!atStart) {
				skip -= paddingLines;
			}
			if (skip > 0) {
				originalLineNumber += skip;
				currentLineNumber += skip;
			}

			for (let i = start; i < lines.length; i++) {
				equalRows.push(createRow(lines[i], RowType.Equal));
			}
		}
		return equalRows;
	}

	function createModifyRows(before: string, after: string): Row[] {
		const internalDiff = charDiff(before, after, true /* cleanup diff */);
		const deletionRows = [createRow('', RowType.Deletion)];
		const insertionRows = [createRow('', RowType.Addition)];

		for (const token of internalDiff) {
			const text = token[1];
			const type = token[0];
			const className = type === Operation.Equal ? '' : 'inner-diff';
			const lines = text.split('\n');
			for (let i = 0; i < lines.length; i++) {
				if (i > 0 && type !== Operation.Insert) {
					deletionRows.push(createRow('', RowType.Deletion));
				}
				if (i > 0 && type !== Operation.Delete) {
					insertionRows.push(createRow('', RowType.Addition));
				}
				if (!lines[i]) {
					continue;
				}
				if (type !== Operation.Insert) {
					deletionRows[deletionRows.length - 1].tokens.push({ text: lines[i], className });
				}
				if (type !== Operation.Delete) {
					insertionRows[insertionRows.length - 1].tokens.push({ text: lines[i], className });
				}
			}
		}
		return deletionRows.concat(insertionRows);
	}

	function createRow(text: string, type: RowType): Row {
		if (type === RowType.Addition) {
			currentLineNumber++;
		}
		if (type === RowType.Deletion) {
			originalLineNumber++;
		}
		if (type === RowType.Equal) {
			originalLineNumber++;
			currentLineNumber++;
		}

		return {
			originalLineNumber,
			currentLineNumber,
			tokens: text ? [{ text, className: 'inner-diff' }] : [],
			type
		};
	}
}

export function documentMap(lines: readonly string[]): Map<number, number> {
	const map = new Map<number, number>();
	for (let pos = 0, lineNo = 0; lineNo < lines.length; lineNo++) {
		map.set(lineNo + 1, pos);
		pos += lines[lineNo].length + 1;
	}
	return map;
}
