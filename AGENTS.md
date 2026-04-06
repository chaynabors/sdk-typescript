# AGENTS.md

<!-- tags: agents, navigation, architecture, development -->

## Overview

Strands is a multi-language AI agent SDK. A TypeScript agent runtime is compiled to a WASM component, hosted by Rust (Wasmtime), and exposed to Python and Kotlin/Java via UniFFI. One implementation serves all languages through a shared binary.

## Directory Map

| Directory         | Language   | What it is                                                          |
|-------------------|------------|---------------------------------------------------------------------|
| `wit/`            | WIT        | Interface contract between WASM guest and host                      |
| `strands-ts/`     | TypeScript | Agent runtime: event loop, model providers, tools, hooks, streaming |
| `strands-wasm/`   | TypeScript | Bridges TS SDK to WIT exports, compiles to WASM component          |
| `strands-rs/`     | Rust       | WASM host: Wasmtime, AOT compilation, WASI HTTP, UniFFI            |
| `strands-derive/` | Rust       | Proc macro: generates UniFFI wrapper types from WIT bindgen output  |
| `strands-py/`     | Python     | Python wrapper: Agent class, @tool decorator, structured output     |
| `strands-kt/`     | Kotlin     | Kotlin/Java wrapper via UniFFI bindings                             |
| `strands-dev/`    | TypeScript | Dev CLI (`src/cli.ts`) orchestrating build, test, lint, CI          |
| `uniffi-bindgen/` | Rust       | UniFFI binding generator utility                                    |
| `docs/`           | Markdown   | Design proposal and team decisions                                  |

## Build Pipeline

Changes flow through layers. Each depends on the one above:

```
wit/agent.wit → generate → strands-ts (npm build) → esbuild → strands-wasm (componentize-js)
→ agent.wasm → AOT compile (build.rs) → strands-rs (Cargo) → libstrands cdylib
→ maturin → strands-py | uniffi-bindgen → strands-kt
```

Generated TS/WASM bindings are checked in (marked `// @generated`). UniFFI Python/Kotlin bindings are not.

## Key Entry Points

| What you're looking for              | Where to look                                    |
|--------------------------------------|--------------------------------------------------|
| WIT contract (all types/resources)   | `wit/agent.wit`                                  |
| TS agent implementation              | `strands-ts/src/agent/agent.ts`                  |
| TS model providers                   | `strands-ts/src/models/`                         |
| TS public exports                    | `strands-ts/src/index.ts`                        |
| WASM bridge (TS ↔ WIT mapping)       | `strands-wasm/entry.ts`                          |
| Rust host (Wasmtime, streaming)      | `strands-rs/src/lib.rs`                          |
| Rust UniFFI bridge types             | `strands-rs/src/uniffi_bridge.rs`                |
| Rust AOT compilation                 | `strands-rs/build.rs`                            |
| Python Agent class                   | `strands-py/strands/agent/__init__.py`           |
| Python @tool decorator               | `strands-py/strands/tools/decorator.py`          |
| Python model wrappers                | `strands-py/strands/models/`                     |
| Python MCP client                    | `strands-py/strands/tools/mcp/mcp_client.py`    |
| Python multiagent (Graph, Swarm)     | `strands-py/strands/multiagent/`                 |
| Python hooks/lifecycle events        | `strands-py/strands/hooks.py`                    |
| Python type conversions (WIT ↔ dict) | `strands-py/strands/_conversions.py`             |
| Dev CLI source                       | `strands-dev/src/cli.ts`                         |
| Task tracking / work items           | `tasks.toml`                                     |
| Design proposal                      | `docs/design-proposal.md`                        |
| Team decisions                       | `docs/team-decisions.md`                         |

## Validation Commands

```bash
npm run dev -- validate wit         # WIT contract change (cascades to all layers)
npm run dev -- validate ts          # TS SDK internals only
npm run dev -- validate ts-api      # TS SDK public API (includes WASM + downstream)
npm run dev -- validate wasm        # WASM bridge
npm run dev -- validate rs          # Rust host
npm run dev -- validate py-bindings # Python bindings (Rust code)
npm run dev -- validate py          # Pure Python
```

**TS internals vs public API:** If `strands-wasm/entry.ts` imports what you changed, it's a public API change → use `ts-api`. Otherwise use `ts`.

**WIT changes** cascade to every layer. Fix compile errors in `strands-wasm/entry.ts`, `strands-rs/`, `strands-derive/`, and language wrappers.

## Repo-Specific Patterns

- **WASM boundary:** All agent logic runs in the TS guest. Language wrappers are thin layers that marshal types across the WIT boundary. Tool execution is dispatched back to the host via the `tool-provider` import.
- **AOT cache:** `strands-rs/src/lib.rs` uses a process-wide `OnceLock` to cache the Wasmtime Engine/Component/Linker. First agent construction pays ~100ms; subsequent ones ~7ms.
- **Model config passthrough:** Python model wrappers (e.g., `BedrockModel`) separate known fields from extras. Known fields go through typed WIT fields; extras are JSON-serialized into `additional_config`.
- **Generated code:** `strands-ts/generated/` and `strands-wasm/generated/` are committed. Run `npm run dev -- generate --check` to verify freshness. CI fails on stale generated code.
- **componentize-js fork:** Uses `@chaynabors/componentize-js` due to upstream WASI buffer reuse issues. See `strands-wasm/patches/getChunkedStream.js`.

## Commit Convention

Scoped messages required: `[scope] message`

Scopes: `mono`, `meta`, `strands-ts`, `strands-wasm`, `strands-rs`, `strands-py`, `strands-dev`, `strands-derive`, `strands-metrics`

Enforced by `.husky/commit-msg` hook.

## Code Style

| Language   | Formatter     | Linter         |
|------------|---------------|----------------|
| Rust       | `cargo fmt`   | `cargo clippy` |
| TypeScript | `prettier`    | `tsc --noEmit` + eslint |
| Python     | `ruff format` | `ruff check`   |

## Testing

| Layer          | Framework | Location                                                          |
|----------------|-----------|-------------------------------------------------------------------|
| TypeScript SDK | vitest    | `strands-ts/src/**/__tests__/` (unit), `strands-ts/test/` (integ) |
| Rust host      | cargo     | `strands-rs/src/` (doc-tests)                                     |
| Python wrapper | pytest    | `strands-py/tests_integ/`                                         |
| Kotlin wrapper | JUnit     | `strands-kt/lib/src/test/`                                        |

## Detailed Documentation

For deeper analysis, see `.agents/summary/index.md` which indexes:
- `architecture.md` — Build pipeline, design decisions, layer diagram
- `components.md` — Every package and subdirectory with responsibilities
- `interfaces.md` — WIT contract, Python/TS public APIs
- `data_models.md` — All types, stream events, lifecycle events
- `workflows.md` — Build, invocation, validation, tool dispatch flows
- `dependencies.md` — Complete dependency listing with versions

## Custom Instructions

<!-- This section is maintained by developers and agents during day-to-day work.
     It is NOT auto-generated by codebase-summary and MUST be preserved during refreshes.
     Add project-specific conventions, gotchas, and workflow requirements here. -->
