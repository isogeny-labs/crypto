//! The pricing band of AGENTS.md §5a.
//!
//! Almost no ECDLP cost in a record is complete: rho, BSGS and the kangaroo
//! leave canonicalisations, partition hashes and table work unpriced, and an
//! index-calculus arm is complete only where `docs/ic/calibration.json` has a
//! pinned row for its curve.  Comparing a calibrated IC cost with an
//! uncalibrated rho cost is lopsided.  The band prices every arm alike: the
//! native work a record leaves unpriced *and* the native work a pinned
//! calibration priced are both charged at one weight `w` (group-addition
//! equivalents per native operation), and two arms are ranked only when the
//! order holds at every weight of the band.
//!
//! For a record, with `G` its charged GAE, `C` the GAE its calibration put on
//! native work, `n_C` the operations behind `C`, and `U` the count behind its
//! unpriced counters,
//!
//! ```text
//! priced_gae(w) = G − C + w · (U + n_C),   priced_S(w) = priced_gae(w) / √r.
//! ```
//!
//! A record that names an unpriced counter without a count has no priced
//! cost above `w = 0`; it stays a lower bound and cannot be ranked.
//!
//! `U` counts operations.  The memory-size counters of
//! [`NON_OPERATION_COUNTERS`] (table bytes) are listed with the unpriced
//! counters but are not in `U`, so they neither add to the price nor block a
//! ranking; a band row reports their mean count beside the price.

use std::collections::BTreeMap;

use serde::Serialize;

use super::methods::PRICED_NATIVE;
use super::record::Record;
use crate::cryptanalysis::ic_boundary::pinned_calibration_ratios;

/// The band every comparison uses unless it states a wider one.
pub const DEFAULT_WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

/// The count behind each unpriced counter of `r`, `None` where the record
/// names the counter but carries no count for it.  A counter `X_uncharged`
/// is looked up in the top-level counters, then in each phase's native map
/// as `X_uncharged` or `X`, and a `solver_*` counter in the solver block.
pub fn unpriced_counts(r: &Record) -> BTreeMap<String, Option<u64>> {
    let mut out = BTreeMap::new();
    for name in &r.cost.unpriced {
        let base = name.strip_suffix("_uncharged").unwrap_or(name);
        if let Some(&v) = r.counters.get(name) {
            out.insert(name.clone(), Some(v));
            continue;
        }
        let mut total = 0u64;
        let mut found = false;
        for p in &r.phases {
            if let Some(&v) = p.native.get(name).or_else(|| p.native.get(base)) {
                total += v;
                found = true;
            }
        }
        if !found && base.starts_with("solver_") {
            if let Some(s) = &r.solver {
                out.insert(name.clone(), Some(s.ops));
                continue;
            }
        }
        out.insert(name.clone(), found.then_some(total));
    }
    out
}

/// Unpriced counters that count bytes of memory, not operations (owner,
/// 2026-11-01, option B of `research/ic_byte_counter_pricing_20261101`).
///
/// The band charges `w` GAE per *operation*.  A byte count is a space
/// measure: at `w = 1` it would charge one GAE per byte, and for the residual
/// collector's table that was 83-99% of the arm's priced `S` (17-58x the
/// matched rho at 2^24..2^37, against 2.6-4.8x without it).  The time spent
/// on the table is already counted by its probes (`lookups`) and inserts.
/// These names are therefore kept in `cost.unpriced` and listed by
/// [`unpriced_counts`], but are not part of the price ([`unpriced_total`],
/// [`priced_gae`]) and cannot block a ranking; [`exempt_counts`] reports
/// them as their own column.  The classification is by name, so it applies
/// to committed records and replays unchanged.  `work.rs` already classes
/// `native_table_bytes_uncharged` as shape for the same reason.
///
/// `factor_base_vec_bytes_floor_uncharged` is priced separately, as one word
/// store per eight bytes, by the `*_full` methods (`full_price.rs`); a
/// record that charged it there does not list it as unpriced.
pub const NON_OPERATION_COUNTERS: &[&str] = &[
    "residual_table_bytes_uncharged",
    "factor_base_vec_bytes_floor_uncharged",
    "native_table_bytes_uncharged",
];

/// Whether `name` is a memory-size counter that the band does not price.
pub fn is_non_operation(name: &str) -> bool {
    NON_OPERATION_COUNTERS.contains(&name)
}

/// The unpriced counters of `r` that the band charges per operation: every
/// entry of [`unpriced_counts`] except [`NON_OPERATION_COUNTERS`].
pub fn priced_counts(r: &Record) -> BTreeMap<String, Option<u64>> {
    let mut counts = unpriced_counts(r);
    counts.retain(|name, _| !is_non_operation(name));
    counts
}

/// The memory-size counters of `r` that the band lists but does not price.
pub fn exempt_counts(r: &Record) -> BTreeMap<String, Option<u64>> {
    let mut counts = unpriced_counts(r);
    counts.retain(|name, _| is_non_operation(name));
    counts
}

/// Total unpriced operations of `r` (counted ones only) and whether any
/// unpriced operation counter has no count.  Memory-size counters
/// ([`NON_OPERATION_COUNTERS`]) are not operations and are left out.
pub fn unpriced_total(r: &Record) -> (u64, bool) {
    let counts = priced_counts(r);
    let missing = counts.values().any(Option::is_none);
    (counts.values().flatten().sum(), missing)
}

fn regime_of(r: &Record) -> &'static str {
    match r.workload.curve.family.as_str() {
        "koblitz" => "koblitz",
        "prime" => "prime",
        _ => "char2",
    }
}

/// The GAE a pinned calibration put on native work in `r`, and the number
/// of operations behind it.  `Err` when the record says it was priced by a
/// pinned calibration that this binary's table cannot reproduce.
pub fn calibrated_native(r: &Record) -> Result<(f64, u64), String> {
    let pinned: Vec<String> = r
        .detail
        .get("calibration_pinned_units")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if pinned.is_empty() {
        return Ok((0.0, 0));
    }
    let table =
        pinned_calibration_ratios(regime_of(r), &r.workload.curve.slug).ok_or_else(|| {
            format!(
                "{}: priced by pinned units {:?} but no calibration row for {}",
                r.record_id, pinned, r.workload.curve.slug
            )
        })?;
    let ratio = |unit: &str| table.get(unit).and_then(|v| v.as_f64());
    let mut gae = 0.0;
    let mut ops = 0u64;
    for p in &r.phases {
        for (counter, unit) in PRICED_NATIVE {
            let n = p.native.get(*counter).copied().unwrap_or(0);
            if n > 0 && pinned.iter().any(|u| u == unit) {
                let k = ratio(unit).ok_or_else(|| format!("{}: no ratio {unit}", r.record_id))?;
                gae += n as f64 * k;
                ops += n;
            }
        }
    }
    if let Some(s) = &r.solver {
        if s.ops > 0
            && s.op_unit.contains("word XOR")
            && pinned.iter().any(|u| u == "ns_per_word_xor")
            && !r.cost.unpriced.iter().any(|u| u.starts_with("solver_"))
        {
            let k = ratio("ns_per_word_xor")
                .ok_or_else(|| format!("{}: no ratio ns_per_word_xor", r.record_id))?;
            gae += s.ops as f64 * k;
            ops += s.ops;
        }
    }
    Ok((gae, ops))
}

/// `priced_gae(w)` of the module comment; `None` for a record with no
/// recorded cost, or one with an uncounted unpriced counter at `w > 0`.
pub fn priced_gae(r: &Record, w: f64) -> Result<Option<f64>, String> {
    let Some(g) = r.cost.total_gae else {
        return Ok(None);
    };
    let (u, missing) = unpriced_total(r);
    if missing && w > 0.0 {
        return Ok(None);
    }
    let (c, n_c) = calibrated_native(r)?;
    Ok(Some(g - c + w * (u + n_c) as f64))
}

/// `priced_S(w)`.
pub fn priced_s(r: &Record, w: f64) -> Result<Option<f64>, String> {
    let sq = (r.workload.curve.r as f64).sqrt();
    Ok(priced_gae(r, w)?.map(|g| g / sq))
}

/// One arm on one curve across the band.
#[derive(Clone, Debug, Serialize)]
pub struct BandRow {
    pub curve: String,
    pub log2_r: f64,
    pub arm: String,
    pub runs: u64,
    pub verified: u64,
    /// Mean priced `S` over verified runs at each weight, `None` when any
    /// verified run has no priced cost there.
    pub mean_s: Vec<Option<f64>>,
    /// `Σ (G − C)` over the verified runs: the charged GAE with any
    /// calibration-priced native work taken out. With `native_sum`,
    /// `Σ priced_gae(w) = charged_sum + w · native_sum` exactly, so a
    /// verdict can be re-checked in integers (`None` with no verified run).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charged_sum: Option<f64>,
    /// `Σ (U + n_C)` over the verified runs; `None` when a run names an
    /// unpriced counter without a count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_sum: Option<u64>,
    /// Unpriced counters that carry no count on some run.
    pub uncounted: Vec<String>,
    /// Mean count per verified run of each memory-size counter
    /// ([`NON_OPERATION_COUNTERS`]) the arm names: listed, not priced.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub exempt_mean_counts: BTreeMap<String, f64>,
}

/// The verdict for a pair of arms on one curve.
#[derive(Clone, Debug, Serialize)]
pub struct BandRank {
    pub curve: String,
    pub a: String,
    pub b: String,
    pub verdict: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct BandReport {
    pub schema: String,
    pub weights: Vec<f64>,
    /// The counters this report lists but does not price
    /// ([`NON_OPERATION_COUNTERS`]); a band file states its own rule.
    pub exempt_counters: Vec<String>,
    pub rows: Vec<BandRow>,
    pub ranks: Vec<BandRank>,
}

/// The band report over `records` (warm-ups ignored).
pub fn band(records: &[Record], weights: &[f64]) -> Result<BandReport, String> {
    let mut by: BTreeMap<(String, String), Vec<&Record>> = BTreeMap::new();
    for r in records.iter().filter(|r| !r.warmup) {
        by.entry((r.workload.curve.slug.clone(), r.arm.clone()))
            .or_default()
            .push(r);
    }
    let mut rows = Vec::new();
    for ((curve, arm), rs) in &by {
        let ok: Vec<&&Record> = rs.iter().filter(|r| r.counts()).collect();
        let mut mean_s = Vec::new();
        for &w in weights {
            let mut vals = Vec::new();
            let mut all = !ok.is_empty();
            for r in &ok {
                match priced_s(r, w)? {
                    Some(v) => vals.push(v),
                    None => all = false,
                }
            }
            mean_s.push(all.then(|| vals.iter().sum::<f64>() / vals.len() as f64));
        }
        let mut uncounted: Vec<String> = rs
            .iter()
            .flat_map(|r| {
                priced_counts(r)
                    .into_iter()
                    .filter(|(_, v)| v.is_none())
                    .map(|(k, _)| k)
            })
            .collect();
        uncounted.sort();
        uncounted.dedup();
        let (mut charged, mut native, mut native_ok) = (0.0f64, 0u64, true);
        for r in &ok {
            let (u, missing) = unpriced_total(r);
            let (c, n_c) = calibrated_native(r)?;
            charged += r.cost.total_gae.unwrap_or(0.0) - c;
            native += u + n_c;
            native_ok &= !missing;
        }
        let mut exempt_mean_counts: BTreeMap<String, f64> = BTreeMap::new();
        for r in &ok {
            for (name, n) in exempt_counts(r) {
                *exempt_mean_counts.entry(name).or_insert(0.0) += n.unwrap_or(0) as f64;
            }
        }
        for v in exempt_mean_counts.values_mut() {
            *v /= ok.len() as f64;
        }
        rows.push(BandRow {
            curve: curve.clone(),
            log2_r: (rs[0].workload.curve.r as f64).log2(),
            arm: arm.clone(),
            runs: rs.len() as u64,
            verified: ok.len() as u64,
            mean_s,
            charged_sum: (!ok.is_empty()).then_some(charged),
            native_sum: (!ok.is_empty() && native_ok).then_some(native),
            uncounted,
            exempt_mean_counts,
        });
    }
    rows.sort_by(|a, b| {
        a.log2_r
            .partial_cmp(&b.log2_r)
            .unwrap()
            .then(a.arm.cmp(&b.arm))
    });
    let mut ranks = Vec::new();
    for (i, a) in rows.iter().enumerate() {
        for b in rows[i + 1..].iter().filter(|b| b.curve == a.curve) {
            let verdict = if a.verified < a.runs || b.verified < b.runs {
                "incomplete".to_string()
            } else if a.mean_s.iter().chain(&b.mean_s).any(Option::is_none) {
                "unrankable (uncounted unpriced work)".to_string()
            } else {
                let lt = a
                    .mean_s
                    .iter()
                    .zip(&b.mean_s)
                    .all(|(x, y)| x.unwrap() < y.unwrap());
                let gt = a
                    .mean_s
                    .iter()
                    .zip(&b.mean_s)
                    .all(|(x, y)| x.unwrap() > y.unwrap());
                match (lt, gt) {
                    (true, _) => format!("{} < {}", a.arm, b.arm),
                    (_, true) => format!("{} < {}", b.arm, a.arm),
                    _ => "unresolved inside the band".to_string(),
                }
            };
            ranks.push(BandRank {
                curve: a.curve.clone(),
                a: a.arm.clone(),
                b: b.arm.clone(),
                verdict,
            });
        }
    }
    Ok(BandReport {
        schema: "ecbench.band/v1".into(),
        weights: weights.to_vec(),
        exempt_counters: NON_OPERATION_COUNTERS.iter().map(|s| s.to_string()).collect(),
        rows,
        ranks,
    })
}

/// The band as a Markdown table.
pub fn markdown(rep: &BandReport) -> String {
    let mut s = String::from("| curve | log2 r | arm | verified/runs |");
    for w in &rep.weights {
        s.push_str(&format!(" S(w={w}) |"));
    }
    s.push_str(" uncounted |\n|:--|--:|:--|--:|");
    for _ in &rep.weights {
        s.push_str("--:|");
    }
    s.push_str(":--|\n");
    for r in &rep.rows {
        s.push_str(&format!(
            "| {} | {:.2} | {} | {}/{} |",
            r.curve, r.log2_r, r.arm, r.verified, r.runs
        ));
        for v in &r.mean_s {
            match v {
                Some(v) => s.push_str(&format!(" {v:.3} |")),
                None => s.push_str(" — |"),
            }
        }
        let u = if r.uncounted.is_empty() {
            "—".to_string()
        } else {
            r.uncounted.join(", ")
        };
        s.push_str(&format!(" {u} |\n"));
    }
    s
}

// The read-only backend-leaderboard tool compiles this file without the runner
// or the repository-root session data (its build sets `ecbench_reader`).
#[cfg(all(test, not(ecbench_reader)))]
mod tests {
    use super::*;

    #[test]
    fn exact_sums_reproduce_the_band_means() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ic_planner_validation_20261020/sessions/r24");
        let records = crate::cryptanalysis::ecbench::runner::read_records(&dir).unwrap();
        let weights = [0.0, 0.1, 1.0];
        let report = band(&records, &weights).unwrap();
        for row in &report.rows {
            let (c, n) = (row.charged_sum.unwrap(), row.native_sum.unwrap());
            assert_eq!(c.fract(), 0.0, "{}: integral charged GAE", row.arm);
            let sqrt_r = 2f64.powf(row.log2_r / 2.0);
            for (k, &w) in weights.iter().enumerate() {
                let from_sums = (c + w * n as f64) / (row.verified as f64 * sqrt_r);
                let mean = row.mean_s[k].unwrap();
                assert!((from_sums / mean - 1.0).abs() < 1e-9, "{} w = {w}", row.arm);
            }
        }
    }

    /// A byte count in `cost.unpriced` is listed but neither priced nor a
    /// blocker, even when it carries no count; an operation counter is
    /// priced, and one without a count still blocks.
    #[test]
    fn memory_size_counters_are_listed_but_not_priced() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ic_planner_validation_20261020/sessions/r24");
        let records = crate::cryptanalysis::ecbench::runner::read_records(&dir).unwrap();
        let base = records
            .iter()
            .find(|r| r.counts() && r.cost.total_gae.is_some())
            .unwrap();
        let (u0, missing0) = unpriced_total(base);
        let g0 = priced_gae(base, 1.0).unwrap().unwrap();

        let mut r = base.clone();
        let name = "residual_table_bytes_uncharged";
        r.cost.unpriced.push(name.to_string());
        r.counters.insert(name.to_string(), 1_000_000);
        assert_eq!(unpriced_counts(&r)[name], Some(1_000_000), "still listed");
        assert_eq!(exempt_counts(&r)[name], Some(1_000_000));
        assert!(!priced_counts(&r).contains_key(name));
        assert_eq!(unpriced_total(&r), (u0, missing0), "not in U");
        assert_eq!(priced_gae(&r, 1.0).unwrap().unwrap(), g0, "not in the price");

        // Named without a count: an exempt counter does not block ranking.
        r.counters.remove(name);
        assert_eq!(unpriced_counts(&r)[name], None);
        assert_eq!(unpriced_total(&r), (u0, missing0));
        assert_eq!(priced_gae(&r, 1.0).unwrap().unwrap(), g0);

        // An operation counter of the same size is priced at w.
        let mut r = base.clone();
        r.cost.unpriced.push("test_probes_uncharged".to_string());
        r.counters
            .insert("test_probes_uncharged".to_string(), 1_000_000);
        assert_eq!(unpriced_total(&r).0, u0 + 1_000_000);
        let g1 = priced_gae(&r, 1.0).unwrap().unwrap();
        assert!((g1 - g0 - 1_000_000.0).abs() < 1e-6);
    }

    #[test]
    fn the_exempt_list_names_only_byte_counters() {
        for n in NON_OPERATION_COUNTERS {
            assert!(n.contains("bytes") && n.ends_with("_uncharged"), "{n}");
            assert!(is_non_operation(n));
        }
        assert!(!is_non_operation("hash_probes_uncharged"));
        assert!(!is_non_operation("residual_table_entries"));
    }
}
