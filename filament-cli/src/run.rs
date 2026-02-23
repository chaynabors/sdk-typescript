use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use filament::{
    bindings::Timeline,
    loader::WasmtimeLoader,
    pipeline::PipelineManifest,
    runtime::Runtime,
};
use tokio::signal;
use tracing::{error, info};

use crate::storage::VolatileTimeline;

pub async fn run_manifest(manifest_path: PathBuf) -> Result<()> {
    info!(path = ?manifest_path, "Loading manifest");
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| anyhow::anyhow!("Failed to read manifest: {}", e))?;

    info!("Initializing Loaders");
    let wasm_loader = Arc::new(WasmtimeLoader::new().map_err(|e| {
        error!(error = ?e, "Failed to create WasmtimeLoader");
        e
    })?);
    info!("Wasmtime loader created successfully");
    let loaders: Vec<Arc<dyn filament::loader::Loader>> = vec![wasm_loader];

    info!("Initializing Runtime");
    let runtime = Runtime::new(loaders);

    info!("Parsing Pipeline manifest");
    let manifest: PipelineManifest =
        toml::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to parse TOML: {}", e))?;

    info!("Initializing Pipeline Storage");
    let timeline: Arc<dyn Timeline> = Arc::new(VolatileTimeline::new());

    let pipeline_name = manifest.metadata.name.clone();
    let pipeline_version = manifest.metadata.version.clone();

    info!(
        pipeline = %pipeline_name,
        version = %pipeline_version,
        "Spawning Pipeline"
    );
    let handle = runtime.spawn(manifest, timeline).await?;
    info!(pid = handle.pid, "Pipeline Spawned");

    info!("Running... Press Ctrl+C to exit.");

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received");
        }
        Err(err) => {
            error!(error = %err, "Unable to listen for shutdown signal");
        }
    }

    info!("Stopping pipeline");
    handle.kill();

    Ok(())
}
