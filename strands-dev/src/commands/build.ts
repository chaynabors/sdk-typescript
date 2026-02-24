import { run, ROOT } from "../run.js";

export interface BuildOptions {
  ts?: boolean;
  wasm?: boolean;
  rs?: boolean;
  py?: boolean;
  release?: boolean;
}

export async function build(opts?: BuildOptions): Promise<void> {
  const all = !opts?.ts && !opts?.wasm && !opts?.rs && !opts?.py;

  if (all || opts?.ts) {
    run("npm run build -w strands-ts");
  }

  if (all || opts?.wasm) {
    if (!all && !opts?.ts) {
      run("npm run build -w strands-ts");
    }
    run("npm run build -w strands-wasm");
  }

  if (all || opts?.rs) {
    const releaseFlag = opts?.release ? " --release" : "";
    run(`cargo build -p strands${releaseFlag}`);
  }

  if (all || opts?.py) {
    const maturinCmd = opts?.release
      ? ".venv/bin/maturin build --release"
      : ".venv/bin/maturin develop -E test";
    run(maturinCmd, { cwd: `${ROOT}/strands-py` });
    stubs();
  }
}

export function stubs(): void {
  run(
    `bash -c 'set -e; lib=$(find strands-py/strands -maxdepth 1 \\( -name "_strands*.so" -o -name "_strands*.dylib" \\) | head -1); if [ -n "$lib" ]; then cargo run -p strands --features stubs --bin strands-stubs -- "$lib" -m _strands -o strands-py/strands/; fi'`,
  );
}
