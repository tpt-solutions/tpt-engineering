//! A calculation laid out as a report: inputs, the formula for each step, the
//! results with pass/fail, and the references the formulas come from.
//!
//! [`Calculation::into_report`] builds an ordinary [`Report`] from existing
//! section types. The renderers need no changes to show it.

use serde::{Deserialize, Serialize};

use crate::model::{NamedValue, Report, ResultEntry, Table, ValidationStatus};

/// One step of a calculation: a named quantity, the formula that gives it, and
/// its value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// What the step computes, such as `"Max bending moment"`.
    pub name: String,
    /// The formula in plain text, such as `"M = w·L² / 8"`.
    pub formula: String,
    /// The computed value.
    pub value: f64,
    /// Unit of the value, if any.
    pub unit: Option<String>,
    /// Where the formula comes from, if it is cited.
    pub reference: Option<String>,
}

impl Step {
    /// A step with no unit or reference.
    pub fn new(name: impl Into<String>, formula: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            formula: formula.into(),
            value,
            unit: None,
            reference: None,
        }
    }

    /// Set the unit of the value.
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    /// Cite the source of the formula.
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }
}

/// A calculation, built up step by step and then laid out as a [`Report`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Calculation {
    /// Title of the report.
    pub title: String,
    /// Inputs to the calculation.
    pub inputs: Vec<NamedValue>,
    /// Steps, in the order they are computed.
    pub steps: Vec<Step>,
    /// Results, each with its pass/fail status.
    pub results: Vec<ResultEntry>,
    /// Full references cited by the steps.
    pub references: Vec<String>,
    /// Free-text notes, such as assumptions and limits of applicability.
    pub notes: Vec<String>,
}

impl Calculation {
    /// An empty calculation with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    /// Add an input.
    pub fn input(mut self, input: NamedValue) -> Self {
        self.inputs.push(input);
        self
    }

    /// Add a step.
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    /// Add a result.
    pub fn result(mut self, result: ResultEntry) -> Self {
        self.results.push(result);
        self
    }

    /// Add a reference. It is listed once even if it is added twice.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        let reference = reference.into();
        if !self.references.contains(&reference) {
            self.references.push(reference);
        }
        self
    }

    /// Add a note.
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Lay the calculation out as a report.
    ///
    /// Sections appear in this order: inputs, the step table, results, an
    /// overall status line (only when some result carries a pass/fail status),
    /// references, and notes. Empty sections are left out.
    pub fn into_report(self) -> Report {
        let mut report = Report::new(self.title);
        if !self.inputs.is_empty() {
            report = report.assumptions(self.inputs);
        }
        if !self.steps.is_empty() {
            let rows = self
                .steps
                .iter()
                .map(|s| {
                    vec![
                        s.name.clone(),
                        s.formula.clone(),
                        format_sig6(s.value),
                        s.unit.clone().unwrap_or_default(),
                        s.reference.clone().unwrap_or_default(),
                    ]
                })
                .collect();
            let headers = ["Step", "Formula", "Value", "Unit", "Reference"]
                .map(str::to_owned)
                .to_vec();
            report = report.table(Table::new(headers, rows).with_caption("Calculation steps"));
        }
        if !self.results.is_empty() {
            report = report.results(self.results);
        }
        let status = report.overall_status();
        if status != ValidationStatus::Info {
            report = report.paragraph(format!("Overall status: {}.", status.as_symbol()));
        }
        if !self.references.is_empty() {
            report = report.heading("References");
            for reference in self.references {
                report = report.paragraph(reference);
            }
        }
        for note in self.notes {
            report = report.paragraph(note);
        }
        report
    }
}

/// Text for a step value, rounded to six significant figures so floating-point
/// noise such as `5.999999999999998` is not printed.
fn format_sig6(value: f64) -> String {
    if value == 0.0 || !value.is_finite() {
        return value.to_string();
    }
    let scale = 10f64.powi(5 - value.abs().log10().floor() as i32);
    ((value * scale).round() / scale).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Section;

    #[test]
    fn step_values_print_with_six_significant_figures() {
        assert_eq!(format_sig6(5.999999999999998), "6");
        assert_eq!(format_sig6(0.00100215168), "0.00100215");
        assert_eq!(format_sig6(0.0), "0");
    }

    fn sample() -> Calculation {
        Calculation::new("Beam")
            .input(NamedValue::new("Span", 4.0).with_unit("m"))
            .step(
                Step::new("Max bending moment", "M = w·L² / 8", 4000.0)
                    .with_unit("N·m")
                    .with_reference("Elastic flexure"),
            )
            .result(ResultEntry::with_limits(
                "Utilisation",
                0.5,
                None,
                None,
                Some(1.0),
            ))
            .reference("Elastic flexure")
            .reference("Elastic flexure")
            .note("Uniform load over the full span.")
    }

    #[test]
    fn sections_appear_in_order_and_empty_ones_are_left_out() {
        let report = sample().into_report();
        let kinds: Vec<&str> = report
            .sections
            .iter()
            .map(|s| match s {
                Section::Assumptions(_) => "assumptions",
                Section::Table(_) => "table",
                Section::Results(_) => "results",
                Section::Heading(_) => "heading",
                Section::Paragraph(_) => "paragraph",
            })
            .collect();
        assert_eq!(
            kinds,
            [
                "assumptions",
                "table",
                "results",
                "paragraph",
                "heading",
                "paragraph",
                "paragraph"
            ]
        );
    }

    #[test]
    fn references_are_listed_once() {
        let report = sample().into_report();
        let refs = report
            .sections
            .iter()
            .filter(|s| matches!(s, Section::Paragraph(p) if p == "Elastic flexure"))
            .count();
        assert_eq!(refs, 1);
    }

    #[test]
    fn failing_result_adds_overall_status() {
        let report = Calculation::new("Overstressed")
            .result(ResultEntry::with_limits(
                "Utilisation",
                1.2,
                None,
                None,
                Some(1.0),
            ))
            .into_report();
        let has_status = report
            .sections
            .iter()
            .any(|s| matches!(s, Section::Paragraph(p) if p == "Overall status: FAIL."));
        assert!(has_status);
    }

    #[test]
    fn info_only_results_add_no_status_line() {
        let report = Calculation::new("Properties")
            .result(ResultEntry::new(
                "Density",
                997.0,
                None,
                ValidationStatus::Info,
                None,
            ))
            .into_report();
        assert!(
            !report
                .sections
                .iter()
                .any(|s| matches!(s, Section::Paragraph(p) if p.starts_with("Overall status")))
        );
    }
}
