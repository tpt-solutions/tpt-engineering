//! `tpt-eng-mcp`: serve calculation sheets to MCP clients over stdio.

use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    tpt_eng_mcp::serve(BufReader::new(stdin.lock()), stdout.lock())
}
