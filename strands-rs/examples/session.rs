//! Session save/restore example.
//!
//! Demonstrates getting conversation history from one agent, creating
//! a new agent, and restoring that history so it continues the conversation.
//!
//! Usage:
//!   cargo run --example session

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
    // First agent — have a conversation.
    let mut agent1 = Agent::builder()
        .system_prompt("You are a helpful assistant. Be concise.")
        .build()
        .await?;

    chat(&mut agent1, "My name is Alice.").await?;

    // Save the conversation history.
    let messages_json = agent1.get_messages().await?;
    println!(
        "[saved {} bytes of conversation history]",
        messages_json.len()
    );

    // Create a new agent and restore the history.
    let mut agent2 = Agent::builder()
        .system_prompt("You are a helpful assistant. Be concise.")
        .build()
        .await?;

    agent2.set_messages(&messages_json).await?;
    println!("[restored conversation history to new agent]");

    // The new agent should remember the conversation.
    chat(&mut agent2, "What is my name?").await?;

    Ok(())
}
