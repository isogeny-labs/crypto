use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct ArmConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    method_id: Option<String>,
    name: String,
    family: String,
    n: u64,
    factor_base: String,
    oracle: String,
    /// Further `ic.pipeline` parameters (for example `targets=stride`);
    /// empty for the factor-base and oracle families.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    params: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Incumbent {
    arm: ArmConfig,
    /// Mean S at w = 0, 0.1, 1 GAE per unpriced native operation.
    s: [f64; 3],
    #[serde(default)]
    rho_s: Option<[f64; 3]>,
    evidence: Value,
    since_round: u64,
}

fn fmt_distance(s: &[f64; 3], rho: Option<&[f64; 3]>) -> String {
    rho.and_then(|r| checked_distance(s, r))
        .map(|d| format!("{:.1}x / {:.1}x / {:.1}x", d[0], d[1], d[2]))
        .unwrap_or_else(|| "unavailable".into())
}

use std::collections::BTreeMap;

/// Return a cost mean only for a nonempty, completely verified cohort.
fn complete_means(rows: &BTreeMap<String, ([f64; 3], u64, u64)>, arm: &str) -> Option<[f64; 3]> {
    let (s, runs, verified) = rows.get(arm)?;
    (*runs > 0 && runs == verified && s.iter().all(|v| v.is_finite() && *v >= 0.0)).then_some(*s)
}

/// Missing/zero/corrupt reference costs cannot define a finite ratio.
fn checked_distance(s: &[f64; 3], rho: &[f64; 3]) -> Option<[f64; 3]> {
    if s.iter().any(|v| !v.is_finite() || *v < 0.0)
        || rho.iter().any(|v| !v.is_finite() || *v <= 0.0)
    {
        return None;
    }
    let ratio = [s[0] / rho[0], s[1] / rho[1], s[2] / rho[2]];
    ratio.iter().all(|v| v.is_finite()).then_some(ratio)
}

#[test]
fn partial_reference_does_not_supply_a_distance() {
    let rows = BTreeMap::from([
        ("sel".into(), ([1.0, 2.0, 3.0], 4, 4)),
        ("rho-neg".into(), ([2.0, 3.0, 4.0], 4, 1)),
    ]);
    assert!(complete_means(&rows, "sel").is_some());
    let ratio = complete_means(&rows, "sel")
        .zip(complete_means(&rows, "rho-neg"))
        .and_then(|(s, rho)| checked_distance(&s, &rho));
    assert_eq!(ratio, None);
}

#[test]
fn complete_cohorts_keep_the_original_ratios() {
    let rows = BTreeMap::from([
        ("sel".into(), ([1.0, 2.0, 3.0], 4, 4)),
        ("rho-neg".into(), ([2.0, 4.0, 6.0], 4, 4)),
    ]);
    assert_eq!(
        complete_means(&rows, "sel")
            .zip(complete_means(&rows, "rho-neg"))
            .and_then(|(s, rho)| checked_distance(&s, &rho)),
        Some([0.5; 3])
    );
}

#[test]
fn empty_incomplete_and_nonfinite_cohorts_are_not_eligible() {
    for (means, runs, verified) in [
        ([1.0; 3], 0, 0),
        ([1.0; 3], 4, 0),
        ([1.0; 3], 4, 3),
        ([f64::NAN; 3], 4, 4),
        ([-1.0; 3], 4, 4),
        ([f64::INFINITY; 3], 4, 4),
    ] {
        let rows = BTreeMap::from([("inc".into(), (means, runs, verified))]);
        assert!(complete_means(&rows, "inc").is_none());
    }
}

#[test]
fn invalid_denominators_and_ratio_overflow_are_not_distances() {
    assert_eq!(checked_distance(&[1.0; 3], &[0.0; 3]), None);
    assert_eq!(checked_distance(&[1.0; 3], &[-1.0; 3]), None);
    assert_eq!(checked_distance(&[1.0; 3], &[f64::NAN; 3]), None);
    assert_eq!(
        checked_distance(&[f64::MAX; 3], &[f64::MIN_POSITIVE; 3]),
        None
    );
    assert_eq!(checked_distance(&[0.0; 3], &[1.0; 3]), Some([0.0; 3]));
}

    #[test]
    fn optional_reference_keeps_legacy_array_encoding() {
        let mut value = serde_json::json!({"arm":{"name":"fixture","family":"fixture","n":1,
        "factor_base":"fixture","oracle":"fixture"},"s":[1.0,2.0,3.0],
        "rho_s":[2.0,3.0,4.0],"evidence":{},"since_round":0});
        let mut inc: Incumbent = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(inc.rho_s, Some([2.0, 3.0, 4.0]));
        assert_eq!(serde_json::to_value(&inc).unwrap(), value);
        inc.rho_s = None;
        value["rho_s"] = Value::Null;
        assert_eq!(serde_json::to_value(&inc).unwrap(), value);
        assert_eq!(fmt_distance(&inc.s, inc.rho_s.as_ref()), "unavailable");
    }