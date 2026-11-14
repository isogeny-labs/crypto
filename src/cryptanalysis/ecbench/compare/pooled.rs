//! One paired ratio pooled over several sessions.
//!
//! A study run as one session per size (one chunk spec per curve size, as
//! `research/cheon_ecbench_wide_20261106/`) has no single session to
//! compare in.  [`compare_pooled`] reads every session, pairs runs *within*
//! each session exactly as [`super::compare_loaded_weighted`] does (by
//! `(workload, round)`, with the same censoring, `--weight` pricing and
//! table-work reading), never across sessions, and pools the pairs.
//!
//! *Statistic.*  Sessions of different sizes have different `√r`, so the
//! sum of `S` would weight a 36-bit pair like a 56-bit one.  The headline
//! is the ratio of total cost, `Σ S_B·√r / Σ S_A·√r` (total GAE, priced at
//! `--weight` when given) over every pair of every session: the same ratio
//! of sums as a single session over one curve size, and the pooled figure
//! a study's validator computes.  The ratio of summed `S` (every pair
//! weighted alike) is reported beside it as `s_pooled`.
//!
//! *Interval.*  Workloads of every session are the clusters (one cluster
//! per `(session, workload)`), passed to [`super::ratio_report`] with the
//! same resamples and seed: the linearised `t(K−1)` interval and the
//! two-stage cluster bootstrap of the single-session path.
//!
//! *Refusals.*  Fewer than two sessions, a repeated session, a session that
//! is not `complete` (unless `allow_interrupted`, recorded and then not
//! admissible), an arm missing from a session, arms whose `method_id` or
//! unit differ between sessions, and, within a session, the single-session
//! refusals (different workloads, different units).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    band_report, fmt_ci, match_pairs, measured, ratio_report, summarize, validate_pricing_weight,
    validate_weighted_totals, CompareOptions, OpsBand, Pair, RatioReport,
};
use crate::cryptanalysis::ecbench::canonical::short_id;
use crate::cryptanalysis::ecbench::record::Record;
use crate::cryptanalysis::ecbench::runner::{read_records, read_session, Session};

/// The pooled comparison's schema.
pub const POOLED_COMPARISON_SCHEMA: &str = "ecbench.pooled_comparison/v1";

/// One session's share of a pooled comparison.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PooledSession {
    pub dir: String,
    pub session_id: String,
    pub status: String,
    /// Matched pairs this session contributes (censored ones included).
    pub pairs: u64,
    pub workloads: u64,
    pub unpaired_runs: u64,
    pub censored_pairs: u64,
    /// This session's own ratio of total cost (`Σ S_B·√r / Σ S_A·√r`).
    pub ratio_b_over_a: Option<f64>,
}

/// One ratio with its intervals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PooledRatio {
    pub ratio_b_over_a: Option<f64>,
    /// Linearised `t(K−1)` interval, read on the log scale.
    pub ci95: Option<(f64, f64)>,
    pub ci_method: String,
    pub dof: Option<f64>,
    /// Two-stage (workload, then pair) percentile bootstrap.
    pub ci95_bootstrap: Option<(f64, f64)>,
    pub ci_bootstrap_method: String,
}

impl From<&RatioReport> for PooledRatio {
    fn from(r: &RatioReport) -> Self {
        Self {
            ratio_b_over_a: r.ratio,
            ci95: r.ci95,
            ci_method: r.ci_method.clone(),
            dof: r.dof,
            ci95_bootstrap: r.ci95_bootstrap,
            ci_bootstrap_method: r.bootstrap_method.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PooledComparison {
    pub schema: String,
    pub comparison_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priced_table_work: Vec<String>,
    pub resamples: usize,
    pub resample_seed: u64,
    /// `[arm, method_id]` of the baseline (denominator).
    pub a: (String, String),
    /// `[arm, method_id]` of the candidate (numerator).
    pub b: (String, String),
    pub unit: String,
    pub sessions: Vec<PooledSession>,
    pub pairs: u64,
    /// `(session, workload)` clusters behind the interval.
    pub workloads: u64,
    pub unpaired_runs: u64,
    pub censored_pairs: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    /// Either arm left work unpriced in some session.
    pub bounded: bool,
    /// The headline: ratio of total cost (`Σ S·√r`, GAE) over every pair.
    pub ratio_b_over_a: Option<f64>,
    pub ci95: Option<(f64, f64)>,
    pub ci_method: String,
    pub dof: Option<f64>,
    pub ci95_bootstrap: Option<(f64, f64)>,
    pub ci_bootstrap_method: String,
    /// Over the pairs where both sides verified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_only_ratio: Option<f64>,
    /// The ratio of summed `S`: every pair weighted alike, whatever its size.
    pub s_pooled: PooledRatio,
    /// AGENTS.md §5a over the exact pairs, in total cost.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band: Option<OpsBand>,
    pub allow_interrupted: bool,
    /// Every session is `complete`.
    pub admissible: bool,
    pub verdict: String,
}

/// Read every session in `dirs` and pool the paired comparison of `b_arm`
/// against `a_arm`.
pub fn compare_pooled(
    dirs: &[PathBuf],
    a_arm: &str,
    b_arm: &str,
    opts: &CompareOptions,
    weight: Option<f64>,
) -> Result<PooledComparison, String> {
    let mut loaded = Vec::with_capacity(dirs.len());
    for d in dirs {
        let s = read_session(d).map_err(|e| format!("{}: {e}", d.display()))?;
        let r = read_records(d).map_err(|e| format!("{}: {e}", d.display()))?;
        loaded.push((d.display().to_string(), s, r));
    }
    compare_pooled_loaded(&loaded, a_arm, b_arm, opts, weight)
}

/// [`compare_pooled`] on sessions already read: `(dir, session, records)`.
pub fn compare_pooled_loaded(
    sessions: &[(String, Session, Vec<Record>)],
    a_arm: &str,
    b_arm: &str,
    opts: &CompareOptions,
    weight: Option<f64>,
) -> Result<PooledComparison, String> {
    validate_pricing_weight(weight)?;
    if sessions.len() < 2 {
        return Err("a pooled comparison needs two or more sessions".into());
    }
    if a_arm == b_arm {
        return Err("compare two different arms".into());
    }
    let mut seen = BTreeSet::new();
    for (dir, s, _) in sessions {
        if !seen.insert(s.session_id.as_str()) {
            return Err(format!(
                "session {} ({dir}) is given twice: its pairs would be counted twice",
                s.session_id
            ));
        }
    }
    let unfinished: Vec<String> = sessions
        .iter()
        .filter(|(_, s, _)| s.status != "complete")
        .map(|(_, s, _)| format!("{} is `{}`", s.session_id, s.status))
        .collect();
    if !unfinished.is_empty() && !opts.allow_interrupted {
        return Err(format!(
            "session not complete ({}): refusing to compare; pass --allow-interrupted to compare it as descriptive evidence (the comparison is then recorded as not admissible)",
            unfinished.join("; ")
        ));
    }

    let mut a_method: Option<String> = None;
    let mut b_method: Option<String> = None;
    let mut unit: Option<String> = None;
    let mut bounded = false;
    let mut rows = Vec::with_capacity(sessions.len());
    // Total-cost strata and S strata, one per (session, workload).
    let mut gae_strata: Vec<Vec<Pair>> = Vec::new();
    let mut s_strata: Vec<Vec<Pair>> = Vec::new();
    for (dir, s, recs) in sessions {
        let ctx = |e: String| format!("{dir} ({}): {e}", s.session_id);
        let ma = measured(recs, a_arm);
        let mb = measured(recs, b_arm);
        let a = summarize(&s.session_id, a_arm, &ma).map_err(ctx)?;
        let b = summarize(&s.session_id, b_arm, &mb).map_err(ctx)?;
        for (want, got, arm) in [
            (&mut a_method, &a.method_id, a_arm),
            (&mut b_method, &b.method_id, b_arm),
        ] {
            match want {
                None => *want = Some(got.clone()),
                Some(m) if m != got => {
                    return Err(format!(
                        "the sessions disagree on arm `{arm}`: method {m} in an earlier session, {got} in {dir} ({}); a pooled comparison pools one method per arm",
                        s.session_id
                    ))
                }
                Some(_) => {}
            }
        }
        bounded |= a.lower_bound || b.lower_bound;

        let wa: BTreeSet<&str> = ma.iter().map(|r| r.workload.workload_id.as_str()).collect();
        let wb: BTreeSet<&str> = mb.iter().map(|r| r.workload.workload_id.as_str()).collect();
        if wa != wb {
            return Err(ctx(format!(
                "the arms ran different workloads ({} vs {}); a comparison pairs identical workloads",
                wa.len(),
                wb.len()
            )));
        }
        let u = &ma[0].cost.unit;
        if ma.iter().chain(mb.iter()).any(|r| &r.cost.unit != u) {
            return Err(ctx("the arms report different units".into()));
        }
        match &unit {
            None => unit = Some(u.clone()),
            Some(x) if x != u => {
                return Err(ctx(format!(
                    "unit {u} differs from an earlier session's {x}"
                )))
            }
            Some(_) => {}
        }

        let matched = match_pairs(&ma, &mb, weight, opts.pricing, false)?;
        if weight.is_some() {
            validate_weighted_totals(&matched)?;
        }
        let sqrt_r: BTreeMap<&str, f64> = ma
            .iter()
            .map(|r| {
                (
                    r.workload.workload_id.as_str(),
                    (r.workload.curve.r as f64).sqrt(),
                )
            })
            .collect();
        let mut mine: Vec<Vec<Pair>> = Vec::new();
        for (wid, pairs) in matched.by_workload.iter().filter(|(_, v)| !v.is_empty()) {
            let k = sqrt_r[wid.as_str()];
            mine.push(pairs.iter().map(|p| scaled(p, k)).collect());
            s_strata.push(pairs.clone());
        }
        let own = ratio_of_totals(&mine);
        let n: usize = mine.iter().map(Vec::len).sum();
        rows.push(PooledSession {
            dir: dir.clone(),
            session_id: s.session_id.clone(),
            status: s.status.clone(),
            pairs: n as u64,
            workloads: mine.len() as u64,
            unpaired_runs: matched.unpaired,
            censored_pairs: mine
                .iter()
                .flatten()
                .filter(|p| p.a_censored || p.b_censored)
                .count() as u64,
            ratio_b_over_a: own,
        });
        gae_strata.extend(mine);
    }

    let (resamples, seed) = (opts.resamples, opts.seed);
    let rep = ratio_report(&gae_strata, resamples, seed);
    let s_rep = ratio_report(&s_strata, resamples, seed);
    let priced_table_work = opts.pricing.priced_table_work();
    let band = band_report(&gae_strata, a_arm, b_arm, rep.censored_pairs).map(|mut b| {
        if !priced_table_work.is_empty() {
            b.note.push_str(&format!(
                "; counted table work priced in U with the unpriced counters: {}",
                priced_table_work.join(", ")
            ));
            b.priced_table_work = priced_table_work.clone();
        }
        b.note.push_str("; pooled in total cost (S·√r)");
        b
    });
    let (a_method, b_method) = (a_method.unwrap_or_default(), b_method.unwrap_or_default());
    let unpaired: u64 = rows.iter().map(|r| r.unpaired_runs).sum();

    let mut identity = json!({
        "schema": POOLED_COMPARISON_SCHEMA,
        "sessions": rows.iter().map(|r| r.session_id.clone()).collect::<Vec<_>>(),
        "a": [a_arm, a_method],
        "b": [b_arm, b_method],
        "resamples": resamples,
        "seed": seed.to_string(),
        "allow_interrupted": opts.allow_interrupted,
    });
    if let Some(w) = weight {
        identity["pricing_weight"] = json!(w.to_string());
    }
    if !priced_table_work.is_empty() {
        identity["priced_table_work"] = json!(priced_table_work);
    }
    let (comparison_id, _) = short_id("ECCP", &identity)?;

    let mut verdict = format!(
        "{b_arm} / {a_arm} = {}{} (ratio of total cost, {} pooled over {} sessions, {} pairs, {} workloads, {}); ratio of summed S {}{}",
        rep.ratio
            .map(|r| format!("{r:.4}"))
            .unwrap_or_else(|| "n/a".into()),
        fmt_ci(rep.ci95),
        unit.as_deref().unwrap_or("?"),
        rows.len(),
        rep.pairs,
        rep.workloads,
        rep.ci_method,
        s_rep
            .ratio
            .map(|r| format!("{r:.4}"))
            .unwrap_or_else(|| "n/a".into()),
        fmt_ci(s_rep.ci95),
    );
    if let Some(b) = &rep.bound {
        verdict.push_str(&format!(
            "; {} censored pair(s): the ratio is a {b} bound",
            rep.censored_pairs
        ));
    }
    if bounded {
        verdict.push_str("; an arm left work unpriced (lower bounds)");
    }
    if unpaired > 0 {
        verdict.push_str(&format!("; {unpaired} run(s) without a partner"));
    }
    if !unfinished.is_empty() {
        verdict.push_str(&format!(
            "; NOT ADMISSIBLE: {} (compared with --allow-interrupted)",
            unfinished.join("; ")
        ));
    }

    Ok(PooledComparison {
        schema: POOLED_COMPARISON_SCHEMA.into(),
        comparison_id,
        pricing_weight: weight,
        priced_table_work,
        resamples,
        resample_seed: seed,
        a: (a_arm.into(), a_method),
        b: (b_arm.into(), b_method),
        unit: unit.unwrap_or_default(),
        sessions: rows,
        pairs: rep.pairs as u64,
        workloads: rep.workloads as u64,
        unpaired_runs: unpaired,
        censored_pairs: rep.censored_pairs as u64,
        bound: rep.bound.clone(),
        bounded,
        ratio_b_over_a: rep.ratio,
        ci95: rep.ci95,
        ci_method: rep.ci_method.clone(),
        dof: rep.dof,
        ci95_bootstrap: rep.ci95_bootstrap,
        ci_bootstrap_method: rep.bootstrap_method.clone(),
        verified_only_ratio: rep.verified_only_ratio,
        s_pooled: PooledRatio::from(&s_rep),
        band,
        allow_interrupted: opts.allow_interrupted,
        admissible: unfinished.is_empty(),
        verdict,
    })
}

/// A pair in total cost: every `S` (and every band entry) times `√r`.
fn scaled(p: &Pair, k: f64) -> Pair {
    Pair {
        a: p.a * k,
        b: p.b * k,
        a_censored: p.a_censored,
        b_censored: p.b_censored,
        band: p
            .band
            .iter()
            .map(|(x, y)| (x.map(|x| x * k), y.map(|y| y * k)))
            .collect(),
    }
}

fn ratio_of_totals(strata: &[Vec<Pair>]) -> Option<f64> {
    super::pair_ratio(strata.iter().flatten().cloned())
}

#[cfg(test)]
mod tests {
    use super::super::compare_loaded_weighted;
    use super::*;
    use crate::cryptanalysis::ecbench::band::Pricing;
    use crate::cryptanalysis::ecbench::spec::Level;
    use serde_json::Value;

    const SESSION: &str = include_str!(
        "../../../../research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/session.json"
    );
    const RECORDS: &str = include_str!(
        "../../../../research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/records.jsonl"
    );

    fn session(id: &str, status: &str) -> Session {
        let mut v: Value = serde_json::from_str(SESSION).unwrap();
        v["session_id"] = json!(id);
        v["status"] = json!(status);
        serde_json::from_value(v).unwrap()
    }

    /// A verified measured run of `arm` on workload `wid` of a curve with
    /// order `r`, costing `s`.
    fn rec(arm: &str, method: &str, wid: &str, r: u64, round: u32, s: f64) -> Record {
        let mut v: Value = serde_json::from_str(RECORDS.lines().next().unwrap()).unwrap();
        v["arm"] = json!(arm);
        v["warmup"] = json!(false);
        v["round"] = json!(round);
        v["method"]["method_id"] = json!(method);
        v["workload"]["workload_id"] = json!(wid);
        v["workload"]["curve"]["r"] = json!(r);
        v["outcome"]["status"] = json!("verified");
        v["cost"]["lower_bound"] = json!(false);
        v["cost"]["unpriced"] = json!([]);
        v["counters"] = json!({});
        v["phases"] = json!([]);
        v["algorithm_seed"] = json!(format!("{round}"));
        let mut rec: Record = serde_json::from_value(v).unwrap();
        rec.cost.s = Some(s);
        rec.cost.total_gae = Some(s * (r as f64).sqrt());
        rec
    }

    /// A session of `rows` = `(workload, a_s, b_s)`, one round each, on a
    /// curve of order `r`; arm A runs `M1`, arm B runs `b_method`.
    fn sess(
        id: &str,
        r: u64,
        rows: &[(&str, f64, f64)],
        b_method: &str,
    ) -> (String, Session, Vec<Record>) {
        let mut recs = Vec::new();
        for (w, a, b) in rows {
            let wid = format!("{id}-{w}");
            recs.push(rec("A", "M1", &wid, r, 1, *a));
            recs.push(rec("B", b_method, &wid, r, 1, *b));
        }
        (format!("dir-{id}"), session(id, "complete"), recs)
    }

    fn opts(seed: u64) -> CompareOptions {
        CompareOptions {
            resamples: 400,
            seed,
            allow_interrupted: false,
            pricing: Pricing::default(),
            bitop: false,
            w_cal: None,
        }
    }

    const R1: u64 = 1 << 36;
    const R2: u64 = 1 << 44;

    fn fixture() -> Vec<(String, Session, Vec<Record>)> {
        vec![
            sess(
                "S1",
                R1,
                &[("w1", 1.0, 0.5), ("w2", 2.0, 1.5), ("w3", 0.8, 0.6)],
                "M2",
            ),
            sess(
                "S2",
                R2,
                &[("w1", 1.2, 0.4), ("w2", 0.9, 0.5), ("w3", 1.5, 0.7)],
                "M2",
            ),
        ]
    }

    #[test]
    fn pooled_ratio_is_the_hand_computed_ratio_of_total_cost() {
        let c = compare_pooled_loaded(&fixture(), "A", "B", &opts(7), None).unwrap();
        let (k1, k2) = ((R1 as f64).sqrt(), (R2 as f64).sqrt());
        let num = (0.5 + 1.5 + 0.6) * k1 + (0.4 + 0.5 + 0.7) * k2;
        let den = (1.0 + 2.0 + 0.8) * k1 + (1.2 + 0.9 + 1.5) * k2;
        let got = c.ratio_b_over_a.unwrap();
        assert!((got - num / den).abs() < 1e-12, "{got} vs {}", num / den);
        let s = c.s_pooled.ratio_b_over_a.unwrap();
        let s_want = (0.5 + 1.5 + 0.6 + 0.4 + 0.5 + 0.7) / (1.0 + 2.0 + 0.8 + 1.2 + 0.9 + 1.5);
        assert!((s - s_want).abs() < 1e-12);
        assert_eq!((c.pairs, c.workloads, c.sessions.len()), (6, 6, 2));
        assert_eq!(c.sessions[0].pairs, 3);
        assert_eq!(c.sessions[1].session_id, "S2");
        assert!(c.admissible);
        assert_eq!(c.schema, POOLED_COMPARISON_SCHEMA);
        // The band's w = 0 entry is the same ratio of totals.
        let band = c.band.unwrap();
        assert!((band.ratio_b_over_a[0].unwrap() - num / den).abs() < 1e-12);
    }

    #[test]
    fn each_session_matches_its_single_session_comparison_at_every_weight() {
        let f = fixture();
        for w in [None, Some(0.0), Some(0.1), Some(1.0)] {
            let c = compare_pooled_loaded(&f, "A", "B", &opts(7), w).unwrap();
            assert_eq!(c.pricing_weight, w);
            for ((_, s, r), row) in f.iter().zip(&c.sessions) {
                let single =
                    compare_loaded_weighted(s, r, s, r, "A", "B", Level::L2, &opts(7), w).unwrap();
                let (x, y) = (
                    single.ops.ratio_b_over_a.unwrap(),
                    row.ratio_b_over_a.unwrap(),
                );
                assert!((x - y).abs() < 1e-12, "{w:?}: {x} vs {y}");
                assert_eq!(single.ops.pairs, row.pairs);
            }
        }
    }

    #[test]
    fn interval_contains_the_point_and_is_deterministic_for_a_seed() {
        let c = compare_pooled_loaded(&fixture(), "A", "B", &opts(11), None).unwrap();
        let p = c.ratio_b_over_a.unwrap();
        let (lo, hi) = c.ci95.unwrap();
        assert!(lo <= p && p <= hi, "{lo} {p} {hi}");
        let (blo, bhi) = c.ci95_bootstrap.unwrap();
        assert!(blo <= p && p <= bhi, "{blo} {p} {bhi}");
        assert_eq!(c.ci_method, "linearised_t");
        assert_eq!(c.ci_bootstrap_method, "cluster");
        let again = compare_pooled_loaded(&fixture(), "A", "B", &opts(11), None).unwrap();
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            serde_json::to_string(&again).unwrap()
        );
        let other = compare_pooled_loaded(&fixture(), "A", "B", &opts(12), None).unwrap();
        assert_eq!(other.ratio_b_over_a, c.ratio_b_over_a);
        assert_ne!(other.ci95_bootstrap, c.ci95_bootstrap);
        assert_ne!(other.comparison_id, c.comparison_id);
    }

    #[test]
    fn refuses_mismatched_methods_missing_arms_repeats_and_incomplete_sessions() {
        let mut f = fixture();
        f[1] = sess("S2", R2, &[("w1", 1.0, 0.5)], "M3");
        let e = compare_pooled_loaded(&f, "A", "B", &opts(1), None).unwrap_err();
        assert!(e.contains("disagree on arm `B`"), "{e}");

        let mut f = fixture();
        f[1].2.retain(|r| r.arm != "B");
        let e = compare_pooled_loaded(&f, "A", "B", &opts(1), None).unwrap_err();
        assert!(e.contains("arm `B` has no measured runs"), "{e}");

        let f = vec![fixture().remove(0), fixture().remove(0)];
        let e = compare_pooled_loaded(&f, "A", "B", &opts(1), None).unwrap_err();
        assert!(e.contains("given twice"), "{e}");

        let e = compare_pooled_loaded(&fixture()[..1], "A", "B", &opts(1), None).unwrap_err();
        assert!(e.contains("two or more sessions"), "{e}");

        let mut f = fixture();
        f[0].1 = session("S1", "interrupted");
        let e = compare_pooled_loaded(&f, "A", "B", &opts(1), None).unwrap_err();
        assert!(e.contains("--allow-interrupted"), "{e}");
        let mut o = opts(1);
        o.allow_interrupted = true;
        let c = compare_pooled_loaded(&f, "A", "B", &o, None).unwrap();
        assert!(!c.admissible && c.verdict.contains("NOT ADMISSIBLE"));
    }
}
