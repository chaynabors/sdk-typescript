//! Basic agent example — calculator tool with the default Bedrock model.
//!
//! Usage:
//!   cargo run --example basic_agent

use anyhow::Result;
use strands::{Agent, StreamEvent};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = Agent::builder()
        .system_prompt("You are a helpful assistant with a calculator tool.")
        .tool(
            "calculator",
            "Evaluate a math expression and return the numeric result.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "A math expression to evaluate"
                    }
                },
                "required": ["expression"]
            }),
            |_input: &str| -> Result<String, String> {
                Ok(r#"{"status":"success","content":[{"text":"714"}]}"#.to_string())
            },
        )
        .build()
        .await?;

    let stream = agent.stream("What is 42 * 17?").await?;
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result? {
            StreamEvent::TextDelta(text) => print!("{text}"),
            StreamEvent::Stop(data) => println!("\n[{:?}]", data.reason),
            StreamEvent::ToolUse(event) => println!("[tool-use: {}]", event.name),
            StreamEvent::ToolResult(event) => println!("[tool-result: {} => {}]", event.status, event.content),
            _ => {}
        }
    }

    Ok(())
}
