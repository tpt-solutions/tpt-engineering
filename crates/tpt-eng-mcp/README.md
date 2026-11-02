# tpt-eng-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) server that exposes
tpt-engineering calculation sheets as tools. An MCP client, such as an AI
assistant, can run a calculation and read the report without writing code.

## Tools

| Tool | Input | Output |
| --- | --- | --- |
| `run_sheet` | A calculation sheet as JSON. Its input schema is the sheet schema from `tpt-eng-sheet`. | The report as Markdown. Invalid sheets return `isError: true` with the reason, so the client can correct them. |
| `sheet_schema` | none | The JSON Schema for sheets. |

A sheet names one calculation (`water-state`, `beam-udl` or `pid-step`) and its
inputs. Field names carry the units, such as `pressure_kpa` and `span_m`.

## Build

```bash
cargo build --release -p tpt-eng-mcp
# binary: target/release/tpt-eng-mcp (tpt-eng-mcp.exe on Windows)
```

## Client configuration

Add the server to the client's MCP configuration. For example, in a
`claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tpt-eng": {
      "command": "/absolute/path/to/target/release/tpt-eng-mcp"
    }
  }
}
```

## Protocol notes

- Transport: newline-delimited JSON-RPC 2.0 on stdin and stdout. Diagnostics go
  to stderr, and nothing else is written to stdout.
- Protocol version: `2024-11-05`. The server offers the `tools` capability only.
- Messages without an `id` are notifications, and they get no reply.
- Protocol errors use the JSON-RPC codes: parse error (-32700), invalid request
  (-32600), method not found (-32601), invalid params (-32602).
