import * as cp from "child_process";
import * as fs from "fs";
import * as path from "path";

async function main() {
	let cwd = path.resolve(__dirname, "../../");
	let args = parseArgs();
	let debugPath = path.join(__dirname, "./debug.log");

	try {
		await fs.promises.writeFile(debugPath, "");
	} catch (err) {
		console.error(err);
		process.exit(1);
	}

	let debug = fs.createWriteStream(debugPath);
	await new Promise((resolve) => {
		debug.on("open", resolve);
	});

	let debugTerm = cp.spawn("start", [
		"wt", "nt", "--title", "Debug", "PowerShell", "-c",
		"Get-Content", debugPath, "-Wait",
	], {
		cwd,
		stdio: "pipe",
		shell: true,
	});

	let app = cp.spawn("cargo", ["run", "-p=vm", "--", ...args], {
		cwd,
		stdio: ["inherit", "inherit", debug],
		shell: true,
	});

	try {
		await processExit(app);
	} catch (err) {
		setTimeout(() => {
			if (typeof err === "number") {
				process.exit(err);
			} else {
				console.error(err);
				process.exit(1);
			}
		})
	} finally {
		debugTerm.kill(0);
	}
}

main();

function processExit(proc: cp.ChildProcess) {
	return new Promise<void>((resolve, reject) => {
		proc.on("error", reject);
		proc.on("exit", onExit);
		proc.on("close", onExit);

		function onExit(code?: number) {
			if (code) reject(code);
			else resolve();
		}
	});
}

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
