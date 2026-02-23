use anyhow::Result;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::core::{Event, Signal};
use crate::traits::{Loader, Plugin};

#[wasm_bindgen(inline_js = "export function import_module(url) { return import(url); }")]
extern "C" {
    fn import_module(url: &str) -> js_sys::Promise;
}

pub struct BrowserLoader;

#[async_trait]
impl Loader for BrowserLoader {
    fn supports(&self, uri: &str) -> bool {
        // Support HTTP(S) or relative paths to .js modules (JCO transpiled components)
        uri.starts_with("http") || uri.starts_with("/") || uri.starts_with("./")
    }

    async fn load(&self, uri: &str) -> Result<Box<dyn Plugin>> {
        let promise = import_module(uri);
        let module_value = JsFuture::from(promise)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to import plugin module: {:?}", e))?;

        Ok(Box::new(BrowserPlugin {
            js_module: module_value,
        }))
    }
}

struct BrowserPlugin {
    js_module: JsValue,
}

#[async_trait]
impl Plugin for BrowserPlugin {
    async fn weave(&mut self, inputs: Vec<Event>) -> Result<(Vec<Event>, Signal)> {
        let js_inputs = serde_wasm_bindgen::to_value(&inputs)
            .map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;

        let func_name = JsValue::from_str("weave");

        let weave_fn = js_sys::Reflect::get(&self.js_module, &func_name)
            .map_err(|_| anyhow::anyhow!("Plugin does not export 'weave' function"))?;

        if !weave_fn.is_function() {
            return Err(anyhow::anyhow!("'weave' export is not a function"));
        }
        let weave_fn = js_sys::Function::from(weave_fn);

        let promise = weave_fn
            .call1(&JsValue::NULL, &js_inputs)
            .map_err(|e| anyhow::anyhow!("Plugin execution failed: {:?}", e))?;

        let result_val = JsFuture::from(js_sys::Promise::from(promise))
            .await
            .map_err(|e| anyhow::anyhow!("Plugin promise rejected: {:?}", e))?;

        let output_tuple: (Vec<Event>, Signal) = serde_wasm_bindgen::from_value(result_val)
            .map_err(|e| anyhow::anyhow!("Deserialization error (Schema Mismatch): {}", e))?;

        Ok(output_tuple)
    }
}
