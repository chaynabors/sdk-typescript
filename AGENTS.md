# AGENTS.md

<!-- tags: agents, navigation, architecture, development -->

## Overview

Strands is a multi-language AI agent SDK. A TypeScript agent runtime is compiled to a WASM component and hosted directly by Python via `wasmtime-py`. One TS implementation serves all languages through a shared WASM binary.

## Directory Map

| Directory      | Language   | What it is                                                          |
|----------------|------------|---------------------------------------------------------------------|
| `wit/`         | WIT        | Interface contract between WASM guest and host                      |
| `strands-ts/`  | TypeScript | Agent runtime: event loop, model providers, tools, hooks, streaming |
| `strands-wasm/` | TypeScript | Bridges TS SDK to WIT exports, compiles to WASM component          |
| `strands-py/`  | Python     | Python wrapper: Agent class, @tool decorator, direct WASM host      |
| `wasmtime-py/` | Python     | Wasmtime Python bindings (local fork with async component model)    |
| `strands-dev/` | TypeScript | Dev CLI (`src/cli.ts`) orchestrating build, test, lint, CI          |
| `docs/`        | Markdown   | Design proposal, team decisions, migration plan                     |

## Build Pipeline

```
wit/agent.wit → generate → strands-ts (npm build) → esbuild → strands-wasm (componentize-js)
→ agent.wasm → wasmtime-py (ctypes) → strands-py
```

Generated TS/WASM bindings are checked in (marked `// @generated`). Python types are hand-written in `_wasm_types.py`.

## Key Entry Points

| What you're looking for              | Where to look                                    |
|--------------------------------------|--------------------------------------------------|
| WIT contract (all types/resources)   | `wit/agent.wit`                                  |
| TS agent implementation              | `strands-ts/src/agent/agent.ts`                  |
| TS model providers                   | `strands-ts/src/models/`                         |
| TS multiagent (swarm)                | `strands-ts/src/multiagent/`                     |
| TS session management                | `strands-ts/src/session/`                        |
| TS public exports                    | `strands-ts/src/index.ts`                        |
| WASM bridge (TS ↔ WIT mapping)       | `strands-wasm/entry.ts`                          |
| Python WASM host                     | `strands-py/strands/_wasm_host.py`               |
| Python WIT type definitions          | `strands-py/strands/_wasm_types.py`              |
| Python type conversions (WIT ↔ dict) | `strands-py/strands/_conversions.py`             |
| Python Agent class                   | `strands-py/strands/agent/__init__.py`           |
| Python @tool decorator               | `strands-py/strands/tools/decorator.py`          |
| Python model wrappers                | `strands-py/strands/models/`                     |
| Python MCP client                    | `strands-py/strands/tools/mcp/mcp_client.py`    |
| Python multiagent (Graph, Swarm)     | `strands-py/strands/multiagent/`                 |
| Python hooks/lifecycle events        | `strands-py/strands/hooks.py`                    |
| Dev CLI source                       | `strands-dev/src/cli.ts`                         |
| Task tracking / work items           | `tasks.toml`                                     |
| wasmtime-py component model          | `wasmtime-py/wasmtime/component/`                |
| wasmtime-py FFI layer                | `wasmtime-py/wasmtime/_ffi.py`                   |

## Repo-Specific Patterns

- **WASM boundary:** All agent logic runs in the TS guest. Python is a thin host that marshals types across the WIT boundary. Tool execution is dispatched back to the host via the `tool-provider` import.
- **Process-wide cache:** `_wasm_host.py` caches Engine + Component in a `threading.Lock`-protected singleton. First agent construction pays JIT compilation cost; subsequent ones are fast.
- **Per-agent Linker:** Each `WasmAgent` gets its own Linker/Store because tool dispatch callbacks are instance-specific.
- **Async WASM calls:** All WASM entry points use `call_async`. Sync wrappers use `_run_sync()` which handles nested event loops by spawning a thread.
- **Record attribute naming:** wasmtime-py Records use kebab-case attributes matching WIT field names. Access via `getattr(obj, "field-name")`. The `_rec()` helper builds Records via `__dict__`.
- **Model config passthrough:** Python model wrappers (e.g., `BedrockModel`) separate known fields from extras. Known fields go through typed WIT fields; extras are JSON-serialized into `additional_config`.
- **Generated code:** `strands-ts/generated/` and `strands-wasm/generated/` are committed. Run `npm run dev -- generate --check` to verify freshness.
- **componentize-js fork:** Uses `@chaynabors/componentize-js` due to upstream WASI buffer reuse issues. See `strands-wasm/patches/getChunkedStream.js`.
- **wasmtime-py fork:** Local fork from `unshure/wasmtime` (v45.0.0-async) providing async component model support not available upstream.
- **AWS credential injection:** `_wasm_host.py` resolves credentials from env vars / `~/.aws/credentials` and injects into Bedrock model config before passing to WASM.

## Validation Commands

```bash
npm run dev -- validate wit         # WIT contract change (cascades to all layers)
npm run dev -- validate ts          # TS SDK internals only
npm run dev -- validate ts-api      # TS SDK public API (includes WASM + downstream)
npm run dev -- validate wasm        # WASM bridge
npm run dev -- validate py          # Pure Python
```

**TS internals vs public API:** If `strands-wasm/entry.ts` imports what you changed, it's a public API change → use `ts-api`. Otherwise use `ts`.

## Commit Convention

Scoped messages required: `[scope] message`

Scopes: `mono`, `meta`, `strands-ts`, `strands-wasm`, `strands-py`, `strands-dev`, `strands-metrics`

Enforced by `.husky/commit-msg` hook.

## Code Style

| Language   | Formatter     | Linter         |
|------------|---------------|----------------|
| TypeScript | `prettier`    | `tsc --noEmit` + eslint |
| Python     | `ruff format` | `ruff check`   |

## Testing

| Layer          | Framework | Location                                                          |
|----------------|-----------|-------------------------------------------------------------------|
| TypeScript SDK | vitest    | `strands-ts/src/**/__tests__/` (unit), `strands-ts/test/` (integ) |
| Python wrapper | pytest    | `strands-py/tests_integ/`                                         |
| wasmtime-py    | pytest    | `wasmtime-py/tests/`                                              |

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
