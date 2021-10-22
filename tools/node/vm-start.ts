import * as cp_ from "child_process";
import * as path from "path";

namespace cp {
	export function spawn(
		cmd: string,
		args: string[],
		options: cp_.SpawnOptions
	) {
		return new Promise<void>((resolve, reject) => {
			let proc = cp_.spawn(cmd, args, options);

			proc.on("error", reject);
			proc.on("exit", onExit);
			proc.on("close", onExit);

			function onExit(code?: number) {
				if (code) reject(code);
				else resolve();
			}
		});
	}
}

async function main() {
	let cwd = path.resolve(__dirname, "../../");
	let args = parseArgs();

	try {
		await cp.spawn("cargo", ["run", "-p=vm", "--", ...args], {
			cwd,
			stdio: ["inherit", "inherit", "inherit"],
			shell: true,
		});
	} catch (err) {
		if (typeof err === "number") {
			process.exit(err);
		} else {
			console.error(err);
			process.exit(1);
		}
	}
}

main();

function parseArgs(): string[] {
	let args = process.argv
		.slice(2)
		.reduce((accum, arg) => {
			if (isKey(last(accum))) {
				if (isValue(arg)) {
					let key = accum.pop();
					return accum.concat(`${key}=${value(arg)}`);
				}
				if (isKey(arg)) {
					let prevKey = accum.pop();
					return accum.concat(`${prevKey}=true`, arg);
				}
			}
			return accum.concat(arg);
		}, [] as string[]);

	if (isKey(last(args))) {
		let key = args.pop();
		args.push(`${key}=true`);
	}

	return args;
}

function value(arg: string) {
	if (arg.includes(",")) {
		return `"${arg.replace(/,/g, " ")}"`;
	}
	return arg;
}

function last(arr: string[]) {
	return arr[arr.length - 1];
}

function isValue(arg?: string) {
	if (!arg) return false;
	return !arg.startsWith("--") && !arg.includes("=");
}

function isKey(arg?: string) {
	if (!arg) return false;
	return arg.startsWith("--") && !arg.includes("=");
}
