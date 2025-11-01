//! MCP server binary for minipg
//!
//! This binary starts an MCP server that exposes minipg functionality
//! via the Model Context Protocol.
//!
//! Usage:
//!   minipg-mcp

use minipg::mcp::create_server;
use rmcp::service::serve_server;
use rmcp::transport::stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_server();
    serve_server(server, stdio()).await?;
    Ok(())
}
