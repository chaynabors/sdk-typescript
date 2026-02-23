#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct PipelineMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct PipelinePluginDefinition {
    pub name: String,
    pub uri: String,
    pub entrypoint: String,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub enum PipelineEntry {
    Plugin(PipelinePluginDefinition),
    Barrier,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct PipelineManifest {
    pub metadata: PipelineMetadata,
    pub pipeline: Vec<PipelineEntry>,
}
