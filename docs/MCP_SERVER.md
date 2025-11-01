# MCP Server for minipg

minipg exposes its functionality via the Model Context Protocol (MCP) using the `rmcp` crate.

## Overview

The MCP server allows AI assistants and other tools to interact with minipg programmatically through the MCP protocol.

## Installation

The MCP server is available as a binary:

```bash
cargo build --release --bin minipg-mcp
```

## Usage

Start the MCP server:

```bash
minipg-mcp
```

The server communicates via stdio (standard input/output) following the MCP protocol.

## Available Tools

### 1. `parse_grammar`

Parse a grammar specification and return the AST.

**Parameters:**
- `grammar_text` (required): The grammar text to parse
- `filename` (optional): Filename for error reporting (default: "grammar.g4")

**Returns:**
- Success: Grammar AST with name, type, rules count, imports, and options
- Failure: Error message

### 2. `validate_grammar`

Validate a grammar specification and return diagnostics.

**Parameters:**
- `grammar_text` (required): The grammar text to validate

**Returns:**
- Success status
- Error and warning flags
- List of diagnostics with severity, message, and location

### 3. `generate_code`

Generate parser code from a grammar specification.

**Parameters:**
- `grammar_text` (required): The grammar text to generate code from
- `target_language` (required): Target language (rust, python, javascript, typescript, go, java, c, cpp)
- `package_name` (optional): Package name for generated code
- `generate_visitor` (optional): Generate visitor pattern (default: false)
- `generate_listener` (optional): Generate listener pattern (default: true)

**Returns:**
- Generated code
- Grammar name
- Target language

### 4. `get_grammar_info`

Get detailed information about a parsed grammar.

**Parameters:**
- `grammar_text` (required): The grammar text to analyze

**Returns:**
- Grammar name and type
- List of parser rules
- List of lexer rules
- Imports and options
- Rule counts

### 5. `list_target_languages`

List all supported target languages for code generation.

**Parameters:** None

**Returns:**
- List of supported languages with descriptions

## Integration

To integrate with MCP clients (like Claude Desktop or other AI assistants), configure the MCP server in your client settings.

Example configuration:

```json
{
  "mcpServers": {
    "minipg": {
      "command": "minipg-mcp",
      "args": []
    }
  }
}
```

## Implementation Status

✅ **COMPLETE**: The MCP server is fully implemented using rmcp v0.8 with the following features:

- ✅ Full rmcp integration with attribute macros (`#[tool]`, `#[tool_router]`, `#[tool_handler]`)
- ✅ All 5 tools implemented and working
- ✅ Structured input/output with JSON schemas
- ✅ Stdio transport for MCP protocol communication
- ✅ Proper error handling and diagnostics
- ✅ Support for all 8 target languages (Rust, Python, JavaScript, TypeScript, Go, Java, C, C++)

The server is production-ready and can be used with any MCP-compatible client (Claude Desktop, Cursor IDE, etc.).

