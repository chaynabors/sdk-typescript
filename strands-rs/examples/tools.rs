//! Tool dispatch example — calculator tool with the default Bedrock model.
//!
//! The model calls a `calculator` tool, the host evaluates the expression,
//! and the result flows back through the agent loop.
//!
//! Usage:
//!   cargo run --example tools

use anyhow::Result;
use strands::{Agent, StreamEvent};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = Agent::builder()
        .system_prompt("You have a calculator tool. Use it to answer math questions.")
        .tool(
            "calculator",
            "Evaluate a math expression and return the numeric result.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "A math expression to evaluate, e.g. '7 * 8'"
                    }
                },
                "required": ["expression"]
            }),
            |input: &str| -> Result<String, String> {
                let parsed: serde_json::Value =
                    serde_json::from_str(input).map_err(|e| e.to_string())?;
                let expr = parsed["expression"]
                    .as_str()
                    .ok_or("missing 'expression' field")?;

                // Trivial evaluator for demo purposes.
                let result: f64 = match expr {
                    "7 * 8" | "7*8" => 56.0,
                    "2 + 2" | "2+2" => 4.0,
                    _ => expr.parse::<f64>().map_err(|e| e.to_string())?,
                };

                Ok(serde_json::json!({ "result": result }).to_string())
            },
        )
        .build()
        .await?;

    let stream = agent.stream("What is 7 * 8?").await?;
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result? {
            StreamEvent::TextDelta(text) => print!("{text}"),
            StreamEvent::Stop(data) => {
                println!();
                println!("[done: {:?}]", data.reason);
            }
            StreamEvent::ToolUse(event) => println!("[tool-use: {}]", event.name),
            StreamEvent::ToolResult(event) => println!("[tool-result: {}]", event.status),
            StreamEvent::Metadata(_) => {}
            StreamEvent::Error(err) => eprintln!("[error: {err}]"),
            StreamEvent::Interrupt(_) => eprintln!("[interrupt]"),
        }
    }

    Ok(())
}
