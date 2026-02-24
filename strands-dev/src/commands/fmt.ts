import { run, ROOT } from "../run.js";

export interface FmtOptions {
  check?: boolean;
}

export async function fmt(opts?: FmtOptions): Promise<void> {
  if (opts?.check) {
    run("cargo fmt --all --check");
    run(
      "npx prettier --check 'strands-wasm/**/*.ts' 'strands-ts/**/*.ts' --ignore-path .gitignore",
    );
    run(".venv/bin/ruff format --check strands/ tests_integ/", {
      cwd: `${ROOT}/strands-py`,
    });
  } else {
    run("cargo fmt --all");
    run(
      "npx prettier --write 'strands-wasm/**/*.ts' 'strands-ts/**/*.ts' --ignore-path .gitignore",
    );
    run(".venv/bin/ruff format strands/ tests_integ/", {
      cwd: `${ROOT}/strands-py`,
    });
  }
}
