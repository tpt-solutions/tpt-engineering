//! A Model Context Protocol (MCP) server that exposes calculation sheets as tools.
//!
//! The server speaks JSON-RPC 2.0 over newline-delimited messages, as MCP
//! stdio transport requires. It offers two tools:
//!
//! * `run_sheet` takes a calculation sheet as JSON and returns its report as
//!   Markdown. Its input schema is the sheet schema from `tpt-eng-sheet`, so a
//!   client sees the same units-in-field-names contract the CLI enforces.
//! * `sheet_schema` returns that schema as text.
//!
//! Only JSON-RPC goes to stdout. Diagnostics belong on stderr.

use std::io::{self, BufRead, Write};

use serde_json::{Value, json};
use tpt_eng_sheet::{Sheet, json_schema, run};

/// The MCP protocol revision this server implements.
pub const PROTOCOL_VERSION: &str = "2024-11-05";

/// Name reported to clients in `serverInfo`.
pub const SERVER_NAME: &str = "tpt-eng-mcp";

/// JSON-RPC error codes used by this server.
const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;

/// An error that becomes a JSON-RPC error response.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RpcError {
    code: i64,
    message: String,
}

impl RpcError {
    fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Handle one incoming line. Returns the response to send, or `None` for a
/// notification (a message without an `id`), which gets no reply.
pub fn handle_message(line: &str) -> Option<Value> {
    let message: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(err) => {
            return Some(error_response(
                Value::Null,
                RpcError::new(PARSE_ERROR, format!("parse error: {err}")),
            ));
        }
    };
    let Some(object) = message.as_object() else {
        return Some(error_response(
            Value::Null,
            RpcError::new(INVALID_REQUEST, "message must be a JSON object"),
        ));
    };
    // A notification has no id and is never answered.
    let id = object.get("id")?.clone();
    let Some(method) = object.get("method").and_then(Value::as_str) else {
        return Some(error_response(
            id,
            RpcError::new(INVALID_REQUEST, "request has no method"),
        ));
    };
    let params = object.get("params").cloned().unwrap_or(Value::Null);
    Some(match dispatch(method, &params) {
        Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
        Err(err) => error_response(id, err),
    })
}

fn error_response(id: Value, err: RpcError) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": err.code, "message": err.message },
    })
}

fn dispatch(method: &str, params: &Value) -> Result<Value, RpcError> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": { "tools": {} },
            "serverInfo": { "name": SERVER_NAME, "version": env!("CARGO_PKG_VERSION") },
        })),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools()? })),
        "tools/call" => call_tool(params),
        other => Err(RpcError::new(
            METHOD_NOT_FOUND,
            format!("method not found: {other}"),
        )),
    }
}

/// The tool descriptions returned by `tools/list`.
fn tools() -> Result<Vec<Value>, RpcError> {
    let sheet_schema: Value = serde_json::from_str(&schema_text()?)
        .map_err(|e| RpcError::new(-32603, format!("schema is not JSON: {e}")))?;
    Ok(vec![
        json!({
            "name": "run_sheet",
            "description": "Run a calculation sheet and return its report as Markdown. \
                A sheet names one calculation (kind: water-state, beam-udl or pid-step) \
                and its inputs. Field names carry the units, such as pressure_kpa and span_m. \
                Use sheet_schema for the full format.",
            "inputSchema": sheet_schema,
            "annotations": { "readOnlyHint": true },
        }),
        json!({
            "name": "sheet_schema",
            "description": "Return the JSON Schema that calculation sheets must satisfy.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false },
            "annotations": { "readOnlyHint": true },
        }),
    ])
}

fn schema_text() -> Result<String, RpcError> {
    json_schema().map_err(|e| RpcError::new(-32603, format!("could not build schema: {e}")))
}

fn call_tool(params: &Value) -> Result<Value, RpcError> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| RpcError::new(INVALID_PARAMS, "tools/call needs a tool name"))?;
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    match name {
        "run_sheet" => Ok(run_sheet_result(&arguments)),
        "sheet_schema" => Ok(text_result(&schema_text()?, false)),
        other => Err(RpcError::new(
            INVALID_PARAMS,
            format!("unknown tool: {other}"),
        )),
    }
}

/// Run a sheet given as a JSON object. Sheet errors are tool errors
/// (`isError: true`), not protocol errors, so the model can read and fix them.
fn run_sheet_result(arguments: &Value) -> Value {
    let mut sheet_value = arguments.clone();
    // Clients often send the schema's own "$schema" key along with the sheet.
    if let Some(object) = sheet_value.as_object_mut() {
        object.remove("$schema");
    }
    let outcome = Sheet::from_json(&sheet_value.to_string())
        .map_err(|e| e.to_string())
        .and_then(|sheet| run(&sheet).map_err(|e| e.to_string()));
    match outcome {
        Ok(report) => text_result(&tpt_eng_report::to_markdown(&report), false),
        Err(message) => text_result(&message, true),
    }
}

fn text_result(text: &str, is_error: bool) -> Value {
    json!({
        "content": [{ "type": "text", "text": text }],
        "isError": is_error,
    })
}

/// Serve MCP over newline-delimited JSON-RPC until the input ends.
///
/// # Errors
///
/// Returns an I/O error if reading the input or writing the output fails.
pub fn serve<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = handle_message(&line) {
            writeln!(output, "{response}")?;
            output.flush()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BEAM: &str = r#"{
        "title": "Lintel check",
        "calc": {
            "kind": "beam-udl",
            "span_m": 4.0,
            "load_n_per_m": 2000.0,
            "width_m": 0.1,
            "height_m": 0.2,
            "allowable_stress_mpa": 12.0
        }
    }"#;

    fn call(name: &str, arguments: Value) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "tools/call",
            "params": { "name": name, "arguments": arguments },
        });
        handle_message(&request.to_string()).unwrap()
    }

    fn text_of(response: &Value) -> &str {
        response["result"]["content"][0]["text"].as_str().unwrap()
    }

    #[test]
    fn initialize_reports_protocol_and_tools_capability() {
        let r = handle_message(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#)
            .unwrap();
        assert_eq!(r["id"], 1);
        assert_eq!(r["result"]["protocolVersion"], PROTOCOL_VERSION);
        assert!(r["result"]["capabilities"]["tools"].is_object());
        assert_eq!(r["result"]["serverInfo"]["name"], SERVER_NAME);
    }

    #[test]
    fn notifications_get_no_reply() {
        let r = handle_message(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
        assert!(r.is_none());
    }

    #[test]
    fn tools_list_offers_run_sheet_with_the_sheet_schema() {
        let r = handle_message(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#).unwrap();
        let tools = r["result"]["tools"].as_array().unwrap();
        let run_sheet = tools.iter().find(|t| t["name"] == "run_sheet").unwrap();
        assert!(run_sheet["inputSchema"]["$defs"]["Calc"].is_object());
        assert!(tools.iter().any(|t| t["name"] == "sheet_schema"));
    }

    #[test]
    fn run_sheet_returns_markdown_report() {
        let response = call("run_sheet", serde_json::from_str(BEAM).unwrap());
        assert_eq!(response["result"]["isError"], false);
        assert!(text_of(&response).contains("Overall status: PASS."));
    }

    #[test]
    fn run_sheet_accepts_the_schema_reference_key() {
        let mut sheet: Value = serde_json::from_str(BEAM).unwrap();
        sheet["$schema"] = json!("https://json-schema.org/draft/2020-12/schema");
        let response = call("run_sheet", sheet);
        assert_eq!(response["result"]["isError"], false);
    }

    #[test]
    fn invalid_sheet_is_a_tool_error_not_a_protocol_error() {
        let mut sheet: Value = serde_json::from_str(BEAM).unwrap();
        sheet["calc"]["span_m"] = json!(-4.0);
        let response = call("run_sheet", sheet);
        assert!(response.get("error").is_none());
        assert_eq!(response["result"]["isError"], true);
        assert!(text_of(&response).contains("span_m"));
    }

    #[test]
    fn misspelt_field_is_reported_to_the_client() {
        let mut sheet: Value = serde_json::from_str(BEAM).unwrap();
        sheet["calc"]["span_mm"] = json!(4.0);
        let response = call("run_sheet", sheet);
        assert_eq!(response["result"]["isError"], true);
        assert!(text_of(&response).contains("span_mm"));
    }

    #[test]
    fn sheet_schema_tool_returns_the_schema() {
        let response = call("sheet_schema", json!({}));
        let schema: Value = serde_json::from_str(text_of(&response)).unwrap();
        assert_eq!(schema["title"], "Sheet");
    }

    #[test]
    fn unknown_method_tool_and_parse_errors_use_jsonrpc_codes() {
        let unknown = handle_message(r#"{"jsonrpc":"2.0","id":3,"method":"nope"}"#).unwrap();
        assert_eq!(unknown["error"]["code"], METHOD_NOT_FOUND);

        let bad_tool = call("does_not_exist", json!({}));
        assert_eq!(bad_tool["error"]["code"], INVALID_PARAMS);

        let parse = handle_message("{not json").unwrap();
        assert_eq!(parse["error"]["code"], PARSE_ERROR);
        assert!(parse["id"].is_null());
    }

    #[test]
    fn serve_answers_each_request_on_its_own_line() {
        let input = concat!(
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            "\n",
            r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
            "\n\n",
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
            "\n",
        );
        let mut output = Vec::new();
        serve(input.as_bytes(), &mut output).unwrap();
        let lines: Vec<&str> = std::str::from_utf8(&output).unwrap().lines().collect();
        assert_eq!(lines.len(), 2);
        let first: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(first["id"], 1);
    }
}
