use crate::{plugin::Plugin, types::*};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ModuleManifest {
    pub metadata: ModuleMetadata,
    pub plugins: Vec<ModulePluginDefinition>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ModulePluginDefinition {
    pub name: String,
    pub description: Option<String>,
    pub capabilities: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub optional_capabilities: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subscribes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub publishes: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum SchedulingPolicy {
    Shared,
    Dedicated,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct HostInfo {
    pub version: Version,
    pub mem_max: u64,
    pub time_limit: u64,
    pub bus_size: u64,
    pub cores: u32,
    pub policy: SchedulingPolicy,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct LoadArgs {
    pub host_info: HostInfo,
    pub entrypoint: String,
    pub version: Version,
}

/// A factory that creates Plugins from URIs.
#[async_trait::async_trait]
pub trait Module: Send + Sync {
    async fn manifest(&self) -> Result<ModuleManifest, FilamentError>;
    async fn load(&self, args: LoadArgs) -> Result<Box<dyn Plugin>, FilamentError>;
}
