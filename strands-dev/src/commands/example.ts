import { run, ROOT } from "../run.js";

export interface ExampleOptions {
  py?: boolean;
}

export async function example(
  name: string,
  opts?: ExampleOptions,
): Promise<void> {
  if (opts?.py) {
    run(`.venv/bin/python examples/${name}.py`, {
      cwd: `${ROOT}/strands-py`,
    });
  } else {
    run(`cargo run -p strands --example ${name}`);
  }
}
