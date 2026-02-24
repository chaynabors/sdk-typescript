import { execSync } from "node:child_process";
import { resolve } from "node:path";

/** Root of the monorepo (where the top-level package.json lives). */
export const ROOT = resolve(import.meta.dirname, "../..");

/** Run a shell command with inherited stdio. Throws on non-zero exit. */
export function run(cmd: string, opts?: { cwd?: string }): void {
  execSync(cmd, { stdio: "inherit", cwd: opts?.cwd ?? ROOT });
}
