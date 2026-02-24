import { run } from "../run.js";

export async function clean(): Promise<void> {
  run("cargo clean");
  try {
    run("npm run clean --workspaces");
  } catch {}
  run("rm -rf strands-py/target strands-py/.venv");
}
