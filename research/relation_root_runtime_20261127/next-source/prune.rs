//! Sequential relation pruning preserving every declared bounded closed space.
//! Optional scalar labels preserve the full [R|a|b] row space. No curve claim,
//! solving-degree estimate, minimum-cardinality guarantee or timing is produced.
use super::{degree_space as ds, relation_labels as labels, Poly, Row};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub spaces: ds::Limits,
    pub labels: labels::Limits,
    pub max_trials: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            spaces: ds::Limits::default(),
            labels: labels::Limits::default(),
            max_trials: 256,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub removed: usize,
    pub spaces: Vec<ds::Snapshot>,
    pub labels: Option<labels::Audit>,
    pub accepted: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub caps: Vec<usize>,
    pub baseline: Vec<ds::Snapshot>,
    pub baseline_labels: Option<labels::Audit>,
    pub trials: Vec<Trial>,
    pub selected_original_indices: Vec<usize>,
    pub status: String,
}
fn validate(
    extra: &[Row],
    caps: &[usize],
    scalar: Option<&labels::LabelSystem>,
) -> Result<(), String> {
    if caps.is_empty()
        || caps.len() > 17
        || caps.iter().any(|&d| d > 16)
        || caps.windows(2).any(|x| x[0] >= x[1])
    {
        return Err("need a nonempty strictly increasing roster of degree caps<=16".into());
    }
    if scalar.is_some_and(|s| {
        s.rows.len() != extra.len() || s.a.len() != extra.len() || s.b.len() != extra.len()
    }) {
        return Err("scalar-label rows must correspond exactly to candidate rows".into());
    }
    Ok(())
}
fn order(extra: &[Row]) -> Vec<usize> {
    let mut ids: Vec<_> = (0..extra.len()).collect();
    // A deterministic sparsity preference, not a model of elimination fill.
    ids.sort_by_key(|&i| {
        (
            Reverse(extra[i].poly.len()),
            Reverse(extra[i].proof.len()),
            i,
        )
    });
    ids
}
fn subset(extra: &[Row], ids: &[usize]) -> Vec<Row> {
    ids.iter().map(|&i| extra[i].clone()).collect()
}
fn scalar_subset(s: &labels::LabelSystem, ids: &[usize]) -> labels::LabelSystem {
    labels::LabelSystem {
        scalar_prime: s.scalar_prime,
        columns: s.columns,
        rows: ids.iter().map(|&i| s.rows[i].clone()).collect(),
        a: ids.iter().map(|&i| s.a[i]).collect(),
        b: ids.iter().map(|&i| s.b[i]).collect(),
    }
}
fn audit(
    s: Option<&labels::LabelSystem>,
    ids: &[usize],
    limits: Limits,
) -> Result<Option<labels::Audit>, String> {
    s.map(|s| labels::audit(&scalar_subset(s, ids), limits.labels))
        .transpose()
}
fn snapshots(
    f: &[Poly],
    n: usize,
    p: u64,
    extra: &[Row],
    caps: &[usize],
    limits: Limits,
) -> Result<Vec<ds::Snapshot>, String> {
    caps.iter()
        .map(|&d| ds::snapshot(f, n, p, d, extra, ds::Mode::Closed, limits.spaces))
        .collect()
}
fn ranks(
    f: &[Poly],
    n: usize,
    p: u64,
    extra: &[Row],
    caps: &[usize],
    spaces: &[ds::Snapshot],
    limits: Limits,
) -> Result<Vec<Option<usize>>, String> {
    if spaces.len() != caps.len() {
        return Err("degree roster mismatch".into());
    }
    spaces
        .iter()
        .zip(caps)
        .map(|(s, d)| {
            if s.degree != *d || s.mode != ds::Mode::Closed {
                return Err("wrong degree or raw space".into());
            }
            Ok(ds::replay(f, n, p, extra, s, limits.spaces)?.exact_space_rank)
        })
        .collect()
}
fn label_rank(
    s: Option<&labels::LabelSystem>,
    ids: &[usize],
    a: Option<&labels::Audit>,
    limits: Limits,
) -> Result<Option<usize>, String> {
    match (s, a) {
        (None, None) => Ok(None),
        (Some(s), Some(a)) => {
            labels::replay(&scalar_subset(s, ids), a, limits.labels)?;
            Ok(Some(a.rank_augmented))
        }
        _ => Err("scalar-label audit presence mismatch".into()),
    }
}
fn preserves(
    base: &[Option<usize>],
    trial: &[Option<usize>],
    b: Option<usize>,
    t: Option<usize>,
) -> bool {
    // Each trial is structurally a subset of the full source roster. Complete
    // equal dimensions therefore prove equality, at EVERY cap, by containment.
    base.iter().all(Option::is_some) && base == trial && b == t
}
fn status(
    base: &[Option<usize>],
    trial_count: usize,
    candidates: usize,
    censored: bool,
) -> &'static str {
    if base.iter().any(Option::is_none) {
        "baseline_censored"
    } else if trial_count < candidates {
        "trial_budget"
    } else if censored {
        "complete_with_censored_trials"
    } else {
        "complete"
    }
}
/// Candidate membership is certified against the immutable original generators.
/// Labels, if supplied, are an independent scalar-field model of these candidate
/// rows; their scalar prime need not equal the polynomial coordinate prime.
/// Original input generators are never pruned. Every selection cost is caller work.
pub fn prune(
    f: &[Poly],
    n: usize,
    p: u64,
    extra: &[Row],
    caps: &[usize],
    scalar: Option<&labels::LabelSystem>,
    limits: Limits,
) -> Result<Report, String> {
    validate(extra, caps, scalar)?;
    let mut selected: Vec<_> = (0..extra.len()).collect();
    let baseline = snapshots(f, n, p, extra, caps, limits)?;
    let base = ranks(f, n, p, extra, caps, &baseline, limits)?;
    let baseline_labels = audit(scalar, &selected, limits)?;
    let b = label_rank(scalar, &selected, baseline_labels.as_ref(), limits)?;
    let mut trials = vec![];
    let mut censored = false;
    if base.iter().all(Option::is_some) {
        for i in order(extra).into_iter().take(limits.max_trials) {
            let ids: Vec<_> = selected.iter().copied().filter(|&j| j != i).collect();
            let rows = subset(extra, &ids);
            let spaces = snapshots(f, n, p, &rows, caps, limits)?;
            let rank = ranks(f, n, p, &rows, caps, &spaces, limits)?;
            let label = audit(scalar, &ids, limits)?;
            let t = label_rank(scalar, &ids, label.as_ref(), limits)?;
            let accepted = preserves(&base, &rank, b, t);
            censored |= rank.iter().any(Option::is_none);
            if accepted {
                selected = ids;
            }
            trials.push(Trial {
                removed: i,
                spaces,
                labels: label,
                accepted,
            });
        }
    }
    let state = status(&base, trials.len(), extra.len(), censored).to_string();
    Ok(Report {
        caps: caps.to_vec(),
        baseline,
        baseline_labels,
        trials,
        selected_original_indices: selected,
        status: state,
    })
}
/// Replays all bounded derivations, ideal witnesses and scalar-row certificates.
/// Trial rosters are reconstructed from prior ACCEPTED removals, preventing the
/// invalid union of individually redundant deletions. No producer booleans trusted.
pub fn replay(
    f: &[Poly],
    n: usize,
    p: u64,
    extra: &[Row],
    caps: &[usize],
    scalar: Option<&labels::LabelSystem>,
    r: &Report,
    limits: Limits,
) -> Result<(), String> {
    validate(extra, caps, scalar)?;
    if r.caps != caps || r.trials.len() > limits.max_trials {
        return Err("pruning scope/budget mismatch".into());
    }
    let mut selected: Vec<_> = (0..extra.len()).collect();
    let base = ranks(f, n, p, extra, caps, &r.baseline, limits)?;
    let b = label_rank(scalar, &selected, r.baseline_labels.as_ref(), limits)?;
    let expected_count = if base.iter().all(Option::is_some) {
        extra.len().min(limits.max_trials)
    } else {
        0
    };
    if r.trials.len() != expected_count {
        return Err("missing or extra pruning trials".into());
    }
    let mut censored = false;
    for (trial, i) in r.trials.iter().zip(order(extra)) {
        if trial.removed != i {
            return Err("trial order mismatch".into());
        }
        let ids: Vec<_> = selected.iter().copied().filter(|&j| j != i).collect();
        let rows = subset(extra, &ids);
        let rank = ranks(f, n, p, &rows, caps, &trial.spaces, limits)?;
        let t = label_rank(scalar, &ids, trial.labels.as_ref(), limits)?;
        let accepted = preserves(&base, &rank, b, t);
        if trial.accepted != accepted {
            return Err("unsupported pruning decision".into());
        }
        if accepted {
            selected = ids;
        }
        censored |= rank.iter().any(Option::is_none);
    }
    if r.selected_original_indices != selected
        || r.status != status(&base, r.trials.len(), extra.len(), censored)
    {
        return Err("pruning summary mismatch".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Vec<Poly>, Vec<Row>) {
        let f = vec![
            Poly::from([(vec![2], 1)]),
            Poly::from([(vec![2], 1), (vec![1], 1)]),
        ];
        let g = Row {
            poly: Poly::from([(vec![1], 1)]),
            proof: super::super::Proof::from([((1, vec![0]), 1), ((0, vec![0]), 100)]),
        };
        (f, vec![g.clone(), g])
    }
    #[test]
    fn sequential_removal_retains_low_degree_availability() {
        let (f, e) = fixture();
        let l = Limits::default();
        let r = prune(&f, 1, 101, &e, &[1, 2], None, l).unwrap();
        replay(&f, 1, 101, &e, &[1, 2], None, &r, l).unwrap();
        assert_eq!(r.selected_original_indices, vec![1]);
        assert!(r.trials[0].accepted);
        assert!(!r.trials[1].accepted);
        let high = prune(&f, 1, 101, &e, &[2], None, l).unwrap();
        assert!(high.selected_original_indices.is_empty());
        assert!(replay(&f, 1, 101, &e, &[1, 2], None, &high, l).is_err());
    }
    #[test]
    fn scalar_information_prevents_polynomial_only_deletion() {
        let (f, e) = fixture();
        let l = Limits::default();
        let s = labels::LabelSystem {
            scalar_prime: 37,
            columns: 1,
            rows: vec![vec![1], vec![1]],
            a: vec![0, 0],
            b: vec![0, 1],
        };
        let r = prune(&f, 1, 101, &e, &[1, 2], Some(&s), l).unwrap();
        assert_eq!(r.selected_original_indices, vec![0, 1]);
        assert!(!r.baseline_labels.as_ref().unwrap().curve_scalar_verified);
        replay(&f, 1, 101, &e, &[1, 2], Some(&s), &r, l).unwrap();
        let mut bad = r.clone();
        bad.baseline_labels.as_mut().unwrap().curve_scalar_verified = true;
        assert!(replay(&f, 1, 101, &e, &[1, 2], Some(&s), &bad, l).is_err());
    }
    #[test]
    fn censor_and_trial_budget_never_justify_unchecked_removal() {
        let (f, e) = fixture();
        let mut l = Limits::default();
        l.spaces.max_columns = 0;
        let r = prune(&f, 1, 101, &e, &[1, 2], None, l).unwrap();
        assert_eq!(r.status, "baseline_censored");
        assert_eq!(r.selected_original_indices, vec![0, 1]);
        replay(&f, 1, 101, &e, &[1, 2], None, &r, l).unwrap();
        l = Limits::default();
        l.max_trials = 1;
        let r = prune(&f, 1, 101, &e, &[1, 2], None, l).unwrap();
        assert_eq!(r.status, "trial_budget");
        replay(&f, 1, 101, &e, &[1, 2], None, &r, l).unwrap();
    }
    #[test]
    fn rejects_summary_roster_and_membership_tampering() {
        let (f, e) = fixture();
        let l = Limits::default();
        let r = prune(&f, 1, 101, &e, &[1, 2], None, l).unwrap();
        let mut bad = r.clone();
        bad.selected_original_indices.clear();
        assert!(replay(&f, 1, 101, &e, &[1, 2], None, &bad, l).is_err());
        let mut bad = r.clone();
        bad.trials[1].accepted = true;
        assert!(replay(&f, 1, 101, &e, &[1, 2], None, &bad, l).is_err());
        let mut bad = r;
        bad.trials.pop();
        assert!(replay(&f, 1, 101, &e, &[1, 2], None, &bad, l).is_err());
        let mut bad_e = e;
        bad_e[0].proof.clear();
        assert!(prune(&f, 1, 101, &bad_e, &[1, 2], None, l).is_err());
    }
}
