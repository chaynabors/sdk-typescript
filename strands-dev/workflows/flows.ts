/**
 * Validation flows for the strands-dev CLI.
 *
 * Each flow validates changes to a specific layer of the build pipeline:
 *   wit/agent.wit -> strands-ts -> strands-wasm -> strands-rs -> strands-py
 *
 * A change to any layer invalidates everything downstream.
 * Pure Python is editable-installed and never requires a rebuild.
 * Derive macro changes cascade into the Rust host.
 */

import { build } from "../src/commands/build.js";
import { check } from "../src/commands/check.js";
import { clean } from "../src/commands/clean.js";
import { fmt } from "../src/commands/fmt.js";
import { generate } from "../src/commands/generate.js";
import { setup } from "../src/commands/setup.js";
import { test } from "../src/commands/test.js";

/** Validate after changing wit/agent.wit. Regenerates all types, rebuilds
 *  every layer, and runs all tests. */
export async function testWitChanges(): Promise<void> {
  await generate();
  await build();
  await test();
}

/** Validate after changing strands-ts/ internals without touching the
 *  public API. */
export async function testTsChanges(): Promise<void> {
  await build({ ts: true });
  await test({ ts: true });
}

/** Validate after changing the strands-ts/ public API surface. WASM bundles
 *  the TS SDK, so it and its downstream consumers must be retested. */
export async function testTsApiChanges(): Promise<void> {
  await build({ wasm: true });
  await test({ rs: true });
  await test({ ts: true });
}

/** Validate after changing strands-wasm/ guest code. The Rust host
 *  AOT-compiles the .wasm, so Rust tests must re-run. */
export async function testWasmChanges(): Promise<void> {
  await build({ wasm: true });
  await test({ rs: true });
}

/** Validate after changing strands-rs/ host code (not PyO3 bindings). */
export async function testRustChanges(): Promise<void> {
  await check({ rs: true });
  await build({ rs: true });
  await test({ rs: true });
}

/** Validate after changing PyO3 bindings. build --py recompiles the
 *  extension via maturin and regenerates .pyi stubs. */
export async function testPyBindingChanges(): Promise<void> {
  await check({ rs: true });
  await build({ py: true });
  await test({ py: true });
}

/** Validate after changing pure Python. No rebuild required. */
export async function testPythonChanges(): Promise<void> {
  await check({ py: true });
  await test({ py: true });
}

/** Validate after changing the strands-derive/ proc macro. Changes cascade
 *  into the Rust host. */
export async function testDeriveChanges(): Promise<void> {
  await check({ rs: true });
  await build({ rs: true });
  await test({ rs: true });
}

/** First-time setup after cloning the repo. */
export async function bootstrap(): Promise<void> {
  await setup();
  await generate();
  await build();
  await test();
}

/** Validate everything before pushing. */
export async function prePush(): Promise<void> {
  await fmt({ check: true });
  await check();
}

/** Clean rebuild from scratch. */
export async function rebuild(): Promise<void> {
  await clean();
  await generate();
  await build();
}
