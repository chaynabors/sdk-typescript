import { run, ROOT } from "../run.js";

export interface CheckOptions {
  rs?: boolean;
  ts?: boolean;
  py?: boolean;
}

export async function check(opts?: CheckOptions): Promise<void> {
  const all = !opts?.rs && !opts?.ts && !opts?.py;

  if (all || opts?.rs) {
    run("cargo clippy --workspace -- -D warnings");
    run("cargo clippy -p strands --features pyo3 -- -D warnings");
  }

  if (all || opts?.py) {
    run(".venv/bin/ruff check strands/ tests_integ/", {
      cwd: `${ROOT}/strands-py`,
    });
  }

  if (all || opts?.ts) {
    run("npm run type-check --workspaces --if-present");
  }
}
