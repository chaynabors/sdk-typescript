//! Multi-turn conversation example.
//!
//! Demonstrates that the agent persists conversation history across
//! multiple stream() calls.
//!
//! Usage:
//!   cargo run --example multi_turn

use anyhow::Result;
use strands::{Agent, StreamEvent};
use tokio_stream::StreamExt;

async fn chat(agent: &mut Agent, input: &str) -> Result<()> {
    println!("User: {input}");
    print!("Agent: ");

    let stream = agent.stream(input).await?;
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result? {
            StreamEvent::TextDelta(text) => print!("{text}"),
            StreamEvent::Stop(_) => println!(),
            StreamEvent::Error(err) => eprintln!("[error: {err}]"),
            _ => {}
        }
    }
    println!();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = Agent::builder()
        .system_prompt("You are a helpful assistant. Be concise.")
        .build()
        .await?;

    chat(&mut agent, "My name is Alice.").await?;
    chat(&mut agent, "What is my name?").await?;

    Ok(())
}
