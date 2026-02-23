#[cfg(feature = "web")]
pub mod browser;
#[cfg(feature = "web")]
pub use browser::BrowserLoader;

#[cfg(feature = "wasmtime")]
pub mod wasmtime;
#[cfg(feature = "wasmtime")]
pub use wasmtime::WasmtimeLoader;

use async_trait::async_trait;

use crate::{module::Module, types::FilamentError};

/// A factory that creates modules from URIs.
#[async_trait]
pub trait Loader: Send + Sync {
    fn supports(&self, uri: &str) -> bool;

    async fn load_module(&self, uri: &str) -> Result<Box<dyn Module>, FilamentError>;
}
