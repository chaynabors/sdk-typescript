//! Example using the Anthropic model provider directly (requires API key).
//!
//! Usage:
//!   ANTHROPIC_API_KEY=sk-... cargo run --example anthropic

use anyhow::Result;
use strands::{Agent, AnthropicConfig, ModelConfig, StreamEvent};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

    let mut agent = Agent::builder()
        .model(ModelConfig::Anthropic(AnthropicConfig {
            model_id: Some("claude-sonnet-4-20250514".into()),
            api_key: Some(api_key),
        }))
        .system_prompt("You are a helpful assistant. Be concise.")
        .build()
        .await?;

    let stream = agent.stream("Say hello in one sentence.").await?;
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result? {
            StreamEvent::TextDelta(text) => print!("{text}"),
            StreamEvent::Stop(data) => {
                println!();
                println!("[done: {:?}]", data.reason);
            }
            StreamEvent::Error(err) => eprintln!("[error: {err}]"),
            _ => {}
        }
    }

    Ok(())
}
