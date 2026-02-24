import { run, ROOT } from "../run.js";

export interface SetupOptions {
  rust?: boolean;
  node?: boolean;
  python?: boolean;
}

export async function setup(opts?: SetupOptions): Promise<void> {
  const all = !opts?.rust && !opts?.node && !opts?.python;

  if (all || opts?.rust) {
    run("rustup update stable");
    run("rustup target add wasm32-wasip2");
    run("cargo install cargo-machete cargo-upgrade");
  }

  if (all || opts?.node) {
    run("npm install");
    run(
      "rm -rf node_modules/@bytecodealliance/componentize-js && ln -s ../../../../bytecodealliance/ComponentizeJS node_modules/@bytecodealliance/componentize-js",
    );
  }

  if (all || opts?.python) {
    run("python3 -m venv .venv", { cwd: `${ROOT}/strands-py` });
    run(".venv/bin/pip install maturin ruff componentize-py", {
      cwd: `${ROOT}/strands-py`,
    });
  }
}
