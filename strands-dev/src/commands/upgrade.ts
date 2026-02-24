import { run } from "../run.js";

export interface UpgradeOptions {
  incompatible?: boolean;
}

export async function upgrade(opts?: UpgradeOptions): Promise<void> {
  if (opts?.incompatible) {
    run("cargo upgrade --incompatible");
  } else {
    run("cargo upgrade");
  }
}
