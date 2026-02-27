import { run } from "../run.js";

export async function clean(): Promise<void> {
  run("cargo clean");
  try {
    run("npm run clean --workspaces");
  } catch {}
  run("rm -rf strands-py/target strands-py/.venv");
  try {
    run("./strands-kt/gradlew -p strands-kt clean");
  } catch {}
  run("rm -f strands-kt/lib/src/main/kotlin/uniffi/strands/strands.kt");
}
