import { run, ROOT } from "../run.js";

export interface TestOptions {
  rs?: boolean;
  py?: boolean;
  ts?: boolean;
  kt?: boolean;
  file?: string;
}

export async function test(opts?: TestOptions): Promise<void> {
  const all = !opts?.rs && !opts?.py && !opts?.ts && !opts?.kt;

  if (all || opts?.rs) {
    run("cargo test -p strands");
  }

  if (all || opts?.py) {
    if (opts?.file) {
      run(`.venv/bin/pytest tests_integ/${opts.file} -v`, {
        cwd: `${ROOT}/strands-py`,
      });
    } else {
      run(".venv/bin/pytest", { cwd: `${ROOT}/strands-py` });
    }
  }

  if (all || opts?.ts) {
    run("npm test -w strands-ts");
  }

  if (all || opts?.kt) {
    run(`./strands-kt/gradlew -p strands-kt :lib:test`);
  }
}
