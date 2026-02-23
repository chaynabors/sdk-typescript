//! Simple streaming chat example using the default Bedrock model.
//!
//! Requires AWS credentials in the environment (e.g. via AWS_PROFILE or
//! AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY).
//!
//! Usage:
//!   cargo run --example chat

use anyhow::Result;
use strands::{Agent, StreamEvent};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = Agent::builder()
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
