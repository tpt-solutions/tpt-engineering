//! JSON calculation-trail exporter for [`Report`](crate::Report).

use crate::error::Result;
use crate::model::Report;

/// Render a report to a pretty-printed JSON calculation-trail string.
pub fn render(report: &Report) -> Result<String> {
    Ok(serde_json::to_string_pretty(report)?)
}
