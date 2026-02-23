//! AOT-compiles the WASM component at `cargo build` time so the runtime
//! can deserialize native code instead of JIT-compiling ~20MB of SpiderMonkey.

use anyhow::Result;
use std::path::PathBuf;
use wasmtime::{Config, Engine};

fn main() -> Result<()> {
    let wasm_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../strands-wasm/dist/strands-agent.wasm");

    println!("cargo::rerun-if-changed={}", wasm_path.display());

    // Engine config must match what lib.rs uses at runtime.
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.target(&std::env::var("TARGET")?)?;

    let engine = Engine::new(&config)?;
    let wasm_bytes = std::fs::read(&wasm_path)?;
    let precompiled = engine.precompile_component(&wasm_bytes)?;

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let out_path = out_dir.join("strands-agent.cwasm");
    std::fs::write(&out_path, &precompiled)?;

    Ok(())
}
