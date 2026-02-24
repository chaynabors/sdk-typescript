import { execSync } from "node:child_process";
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { run, ROOT } from "../run.js";

export interface GenerateOptions {
  check?: boolean;
}

export async function generate(opts?: GenerateOptions): Promise<void> {
  run("npm run generate -w strands-ts");
  run("npm run generate -w strands-wasm");
  annotateGenerated();
  generatePy();

  if (opts?.check) {
    try {
      execSync(
        "git diff --quiet -- strands-wasm/generated/ strands-ts/generated/ strands-py/strands/generated/",
        { cwd: ROOT },
      );
    } catch {
      console.error(
        "error: generated files are out of date -- run 'strands-dev generate' and commit",
      );
      run(
        "git diff --stat -- strands-wasm/generated/ strands-ts/generated/ strands-py/strands/generated/",
      );
      process.exit(1);
    }
  }
}

const GENERATED_HEADER = "// @generated from wit/agent.wit -- do not edit";

/** Prepend @generated headers to generated .d.ts files. */
function annotateGenerated(): void {
  const dirs = [
    join(ROOT, "strands-wasm/generated"),
    join(ROOT, "strands-ts/generated"),
  ];

  for (const dir of dirs) {
    for (const file of walkDts(dir)) {
      const content = readFileSync(file, "utf-8");
      if (!content.startsWith("// @generated")) {
        writeFileSync(file, `${GENERATED_HEADER}\n\n${content}`);
      }
    }
  }
}

/** Recursively find all .d.ts files under a directory. */
function walkDts(dir: string): string[] {
  const results: string[] = [];
  let entries: string[];
  try {
    entries = readdirSync(dir);
  } catch {
    return results;
  }
  for (const entry of entries) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      results.push(...walkDts(full));
    } else if (entry.endsWith(".d.ts")) {
      results.push(full);
    }
  }
  return results;
}

/** Regenerate Python type bindings from wit/agent.wit using componentize-py. */
function generatePy(): void {
  const script = resolve(import.meta.dirname, "../scripts/generate_py.py");
  run(`python3 ${script} ${ROOT}`);
}
