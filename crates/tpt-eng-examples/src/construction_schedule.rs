//! Construction-project scheduling: critical-path method over an
//! activity-on-node network, PERT expected durations for the uncertain
//! activities, resource leveling of a crew demand profile, and an Earned Value
//! check at a status date.

use tpt_eng_schedule::{
    Activity, Schedule, cpi, eac, leveling_peak, pert_expected, smooth_demands, spi,
};

/// Result of the construction-schedule scenario.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleReport {
    /// Project duration from the CPM forward pass, in working days.
    pub project_days: f64,
    /// Activities on the critical path, in order.
    pub critical_path: Vec<String>,
    /// PERT expected duration of the excavation activity (days).
    pub excavation_pert_days: f64,
    /// Peak crew demand before leveling.
    pub crew_peak_before: f64,
    /// Peak crew demand after a 3-day smoothing window.
    pub crew_peak_after: f64,
    /// Cost performance index at the status date.
    pub cpi: f64,
    /// Schedule performance index at the status date.
    pub spi: f64,
    /// Estimate at completion for a 1 000 k budget.
    pub eac: f64,
}

/// Schedule a small foundation-and-frame project and report the key metrics.
///
/// # Panics
///
/// Panics if the fixed activity network is malformed; it is valid by
/// construction.
pub fn run_construction_schedule() -> ScheduleReport {
    let activity = |id: &str, duration: f64, preds: &[&str]| Activity {
        id: id.to_string(),
        duration,
        predecessors: preds.iter().map(|p| p.to_string()).collect(),
    };
    // The excavation duration is uncertain: optimistic 4, most likely 6,
    // pessimistic 12 days. PERT gives the expected value used in the network.
    let excavation_pert_days = pert_expected(4.0, 6.0, 12.0);
    let schedule = Schedule::new(vec![
        activity("survey", 2.0, &[]),
        activity("excavate", excavation_pert_days, &["survey"]),
        activity("footings", 3.0, &["excavate"]),
        activity("frame", 10.0, &["footings"]),
        activity("services", 4.0, &["footings"]),
        activity("roof", 4.0, &["frame"]),
        activity("finish", 6.0, &["roof", "services"]),
    ])
    .expect("valid activity network");

    // Crew demand per working day for the critical activities.
    let crew = [4.0, 4.0, 4.0, 4.0, 1.0, 1.0, 1.0, 8.0, 8.0, 8.0];
    let smoothed = smooth_demands(&crew, 3);

    // Earned Value at day 20: planned 800 k, earned 700 k, actual 750 k.
    let (pv, ev, ac) = (800.0, 700.0, 750.0);

    ScheduleReport {
        project_days: schedule.project_duration(),
        critical_path: schedule.critical_path(),
        excavation_pert_days,
        crew_peak_before: leveling_peak(&crew),
        crew_peak_after: leveling_peak(&smoothed),
        cpi: cpi(ev, ac),
        spi: spi(ev, pv),
        eac: eac(1000.0, ev, ac),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_path_runs_through_frame_and_roof() {
        let r = run_construction_schedule();
        assert!(r.critical_path.iter().any(|a| a == "frame"));
        assert!(r.critical_path.iter().any(|a| a == "finish"));
        assert!(r.project_days > 0.0);
    }

    #[test]
    fn leveling_does_not_raise_the_peak() {
        let r = run_construction_schedule();
        assert!(r.crew_peak_after <= r.crew_peak_before + 1e-9);
    }
}
