import { generate } from "./generate.js";
import { fmt } from "./fmt.js";
import { check } from "./check.js";
import { build } from "./build.js";
import { test } from "./test.js";

export async function ci(): Promise<void> {
  await generate({ check: true });
  await fmt({ check: true });
  await check();
  await build();
  await test();
}
