//! Observed process-native windows, not isolated or complete solver costs.
use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Poly, VisitReport};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Counts {
    native_total: u64,
    stair_subset: u64,
}
impl Counts {
    fn read() -> Self {
        // Separate relaxed atomic loads, deliberately not a transaction.
        Self {
            native_total: f4_fp::native_ops_total(),
            stair_subset: f4_fp::native_stair_ops_total(),
        }
    }
    fn valid(self) -> Option<Self> {
        (self.stair_subset <= self.native_total).then_some(self)
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        Self {
            native_total: self.native_total.checked_add(other.native_total)?,
            stair_subset: self.stair_subset.checked_add(other.stair_subset)?,
        }
        .valid()
    }
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.valid()?;
        other.valid()?;
        Self {
            native_total: self.native_total.checked_sub(other.native_total)?,
            stair_subset: self.stair_subset.checked_sub(other.stair_subset)?,
        }
        .valid()
    }
}

#[derive(Debug, Serialize)]
pub(super) struct Work {
    schema: &'static str,
    attempted: bool,
    result_state: &'static str,
    inclusive: Option<Counts>,
    callback_windows: Option<Counts>,
    outside_callback_windows: Option<Counts>,
    callbacks_seen: Option<u64>,
    scope: &'static str,
    ownership: &'static str,
    provider_overflow_qualified: bool,
    solver_attribution_qualified: bool,
    coverage_complete: bool,
    priced_total: Option<u64>,
    comparison_admissible: bool,
}
impl Default for Work {
    fn default() -> Self {
        Self {
            schema: "pkm.f4-native-window/v1",
            attempted: false,
            result_state: "not_called",
            inclusive: None,
            callback_windows: Some(Counts::default()),
            outside_callback_windows: None,
            callbacks_seen: Some(0),
            scope: "process-wide incomplete native classes; concurrent work may contaminate a window; relaxed snapshots are not transactional",
            ownership: "inclusive aliases callbacks plus outside windows; stair is a subset of native_total, never add it; outside windows are not exclusive solver attribution",
            provider_overflow_qualified: false,
            solver_attribution_qualified: false,
            coverage_complete: false,
            priced_total: None,
            comparison_admissible: false,
        }
    }
}
impl Work {
    pub fn solve_visit(
        &mut self,
        input: &[Poly],
        variables: usize,
        p: u64,
        options: &F4Options,
        domains: &[Option<Vec<u64>>],
        mut visitor: impl FnMut(&[u64]) -> bool,
    ) -> Result<VisitReport, String> {
        if self.attempted {
            return Err("native window owner already used".into());
        }
        self.attempted = true;
        self.result_state = "in_progress";
        let before = Counts::read();
        let mut callbacks = Some(Counts::default());
        let mut seen = Some(0u64);
        let result = f4_fp::solve_visit(input, variables, p, options, domains, |assignment| {
            let before = Counts::read();
            let stop = visitor(assignment);
            let observed = Counts::read().checked_sub(before);
            callbacks = callbacks.zip(observed).and_then(|(a, b)| a.checked_add(b));
            seen = seen.and_then(|n| n.checked_add(1));
            if seen.is_none() {
                callbacks = None;
            }
            stop
        });
        // Retain the observed Result-returning call before the caller uses ?.
        self.inclusive = Counts::read().checked_sub(before);
        self.callback_windows = callbacks;
        self.outside_callback_windows = self
            .inclusive
            .zip(callbacks)
            .and_then(|(a, b)| a.checked_sub(b));
        self.callbacks_seen = seen;
        self.result_state = if result.is_err() {
            "rejected"
        } else {
            "returned"
        };
        result
    }
}

/// Check only metadata arithmetic. A producer can fabricate consistent counts;
/// no execution truth, complete coverage, isolation or price follows.
pub(super) fn check_report(record: &serde_json::Value) -> Result<(), String> {
    let bad = || "native window metadata mismatch".to_string();
    if record["schema"] != "pkm.f4-native-window/v1"
        || record["provider_overflow_qualified"] != false
        || record["solver_attribution_qualified"] != false
        || record["coverage_complete"] != false
        || record["comparison_admissible"] != false
        || !record["priced_total"].is_null()
    {
        return Err(bad());
    }
    let counts = |name: &str| -> Result<Option<Counts>, String> {
        let value = record.get(name).ok_or_else(bad)?;
        let parsed: Option<Counts> = serde_json::from_value(value.clone()).map_err(|_| bad())?;
        if parsed.is_some_and(|n| n.valid().is_none()) {
            return Err(bad());
        }
        Ok(parsed)
    };
    let inclusive = counts("inclusive")?;
    let callbacks = counts("callback_windows")?;
    let outside = counts("outside_callback_windows")?;
    let attempted = record["attempted"].as_bool().ok_or_else(bad)?;
    if !attempted {
        if record["result_state"] != "not_called"
            || inclusive.is_some()
            || outside.is_some()
            || callbacks != Some(Counts::default())
            || record["callbacks_seen"] != 0
        {
            return Err(bad());
        }
    } else {
        if record["result_state"] != "returned" && record["result_state"] != "rejected" {
            return Err(bad());
        }
        let seen: Option<u64> =
            serde_json::from_value(record.get("callbacks_seen").ok_or_else(bad)?.clone())
                .map_err(|_| bad())?;
        if seen.is_none() && callbacks.is_some() {
            return Err(bad());
        }
        if outside != inclusive.zip(callbacks).and_then(|(a, b)| a.checked_sub(b)) {
            return Err(bad());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Vec<Poly>, F4Options) {
        let ordering = f4_fp::Ordering::Grevlex;
        (
            vec![
                f4_fp::normalise(&[(vec![2, 0], 1), (vec![0, 1], 16)], 17, ordering),
                f4_fp::normalise(&[(vec![1, 1], 1), (vec![0, 0], 16)], 17, ordering),
            ],
            F4Options::new(ordering, 12),
        )
    }
    #[test]
    fn observed_callback_native_work_is_not_charged_again_outside_callbacks() {
        let (input, options) = fixture();
        let mut work = Work::default();
        let report = work
            .solve_visit(&input, 2, 17, &options, &[None, None], |_| {
                let nested = f4_fp::f4(&input, 2, 17, &options);
                assert!(!nested.timed_out);
                true
            })
            .unwrap();
        assert!(report.stopped_early);
        assert_eq!(work.callbacks_seen, Some(1));
        let inclusive = work.inclusive.unwrap();
        let callbacks = work.callback_windows.unwrap();
        let outside = work.outside_callback_windows.unwrap();
        assert!(callbacks.native_total > 0);
        assert_eq!(outside.checked_add(callbacks), Some(inclusive));
        let value = serde_json::to_value(&work).unwrap();
        check_report(&value).unwrap();
        let mut bad = value;
        bad["outside_callback_windows"]["native_total"] = serde_json::json!(u64::MAX);
        assert!(check_report(&bad).is_err());
    }
    #[test]
    fn refusal_and_zero_call_states_are_retained_without_inventing_completion() {
        let (input, options) = fixture();
        let mut work = Work::default();
        check_report(&serde_json::to_value(&work).unwrap()).unwrap();
        assert!(work
            .solve_visit(&input, 2, 4, &options, &[None, None], |_| panic!(
                "invalidinput must not visit"
            ))
            .is_err());
        assert_eq!(work.result_state, "rejected");
        assert_eq!(work.callbacks_seen, Some(0));
        check_report(&serde_json::to_value(&work).unwrap()).unwrap();
        let before = serde_json::to_value(&work).unwrap();
        assert!(work
            .solve_visit(&input, 2, 17, &options, &[None, None], |_| false)
            .is_err());
        assert_eq!(serde_json::to_value(&work).unwrap(), before);
    }
    #[test]
    fn observed_backwards_incoherent_and_overflowing_differences_stay_unavailable() {
        let a = Counts {
            native_total: 10,
            stair_subset: 2,
        };
        assert!(Counts {
            native_total: 9,
            stair_subset: 2
        }
        .checked_sub(a)
        .is_none());
        assert!(Counts {
            native_total: 11,
            stair_subset: 9
        }
        .checked_sub(a)
        .is_none());
        assert!(Counts {
            native_total: 3,
            stair_subset: 4
        }
        .valid()
        .is_none());
        assert!(Counts {
            native_total: u64::MAX,
            stair_subset: 0
        }
        .checked_add(Counts {
            native_total: 1,
            stair_subset: 0
        })
        .is_none());
        assert!(a
            .checked_sub(Counts {
                native_total: 11,
                stair_subset: 2
            })
            .is_none());
    }
}
