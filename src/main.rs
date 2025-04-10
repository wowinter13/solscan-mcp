use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use solscan_mcp::SolscanApi;
use tracing_subscriber::{self, EnvFilter};

mod solscan_mcp;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    // Get API key from env
    let api_key =
        std::env::var("SOLSCAN_API_KEY").expect("SOLSCAN_API_KEY environment variable is required");

    tracing::info!("Starting Solscan MCP server");

    let service = SolscanApi::new(api_key)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("Serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
