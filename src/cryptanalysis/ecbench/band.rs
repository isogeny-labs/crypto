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
//!
//! **Table work** ([`Pricing::table_work`], on by default since 2026-11-08;
//! AGENTS.md §5a).  The index-calculus table oracles record their per-step
//! hash-table inserts as `work_native.table_insert_attempts` behind the
//! marker `counted:table_insert_attempts`, for the bitop unit (`work.rs`);
//! they never list the counter in `cost.unpriced`, while prime-field rho
//! lists its per-step canonicalisation (`canonicalisations_uncharged`) and
//! is charged for it.  Each arm does one native operation per step beside
//! its group addition, so a band that reads `cost.unpriced` alone prices
//! rho's and not the walk's.  The default reading therefore adds the
//! counters of [`TABLE_WORK_COUNTERS`] to `U` (phase sum of the
//! `work_native` counter, when and only when the matching `counted:` marker
//! is present in some phase; counted work without its marker is not
//! charged).  [`Pricing::pre_2026_11_08`] (`--no-price-table-work`) is the
//! reading every band file and comparison computed before that date, kept
//! so they can be reproduced; where one is cited again it is labelled the
//! pre-2026-11-08 reading.  The record format does not change: this is a
//! reading of committed records, and their replay audit stays identical.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

/// What the band charges beyond `cost.unpriced` (module comment, "Table
/// work").  The default, since 2026-11-08, prices the counted table work;
/// [`Pricing::pre_2026_11_08`] charges nothing extra, which is the band as
/// every band file and comparison before that date computed it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pricing {
    /// Also charge, at `w` per operation, the [`TABLE_WORK_COUNTERS`] a
    /// record counts in `work_native` behind a `counted:<name>` marker.
    /// On by default (owner, 2026-11-08).
    #[serde(default = "yes")]
    pub table_work: bool,
}

fn yes() -> bool {
    true
}

impl Default for Pricing {
    /// The band's default reading since 2026-11-08: table work priced.
    fn default() -> Self {
        Pricing { table_work: true }
    }
}

impl Pricing {
    /// The reading of every band file and comparison computed before
    /// 2026-11-08: `cost.unpriced` alone, no table work.  On the command
    /// line, `--no-price-table-work`.
    pub const fn pre_2026_11_08() -> Self {
        Pricing { table_work: false }
    }

    /// The counters [`Pricing::table_work`] adds to `U`, by name.
    pub fn priced_table_work(&self) -> Vec<String> {
        if self.table_work {
            TABLE_WORK_COUNTERS.iter().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }
}

/// Per-step table work the oracles count only for the bitop unit: one
/// insert attempt per walk step of `wwalk` and the pair tables
/// (`ic_framework/plugins.rs`, `table_counted()`).
pub const TABLE_WORK_COUNTERS: &[&str] = &["table_insert_attempts"];

/// The phase sum of each [`TABLE_WORK_COUNTERS`] counter in `r`'s
/// `work_native` maps, present only where some phase carries the marker
/// `counted:<name>` (the code that did the work counted it and said so).  A
/// record with the counter and no marker is not charged: without the marker
/// the count is not known to be complete.
pub fn table_work_counts(r: &Record) -> BTreeMap<String, u64> {
    let mut out = BTreeMap::new();
    for name in TABLE_WORK_COUNTERS {
        let marker = format!("counted:{name}");
        if !r.phases.iter().any(|p| p.work_native.contains_key(&marker)) {
            continue;
        }
        let total: u64 = r
            .phases
            .iter()
            .filter_map(|p| p.work_native.get(*name))
            .sum();
        out.insert(name.to_string(), total);
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
    priced_counts_with(r, Pricing::default())
}

/// [`priced_counts`] under `pricing`: with `table_work` on, the counted
/// table work of [`table_work_counts`] is added under its counter's name.
pub fn priced_counts_with(r: &Record, pricing: Pricing) -> BTreeMap<String, Option<u64>> {
    let mut counts = unpriced_counts(r);
    counts.retain(|name, _| !is_non_operation(name));
    if pricing.table_work {
        for (name, n) in table_work_counts(r) {
            let e = counts.entry(name).or_insert(Some(0));
            *e = e.map(|t| t + n);
        }
    }
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
    unpriced_total_with(r, Pricing::default())
}

/// [`unpriced_total`] under `pricing`.
pub fn unpriced_total_with(r: &Record, pricing: Pricing) -> (u64, bool) {
    let counts = priced_counts_with(r, pricing);
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
    // A record that carries its own counted ratios (`calibration_ratios`,
    // written by `ic.pipeline` + `bin-x-line` from the run's counted `F_q`
    // multiplications) is read from them when the pinned table has no row;
    // every record without that key is priced exactly as before.
    let own = || {
        r.detail
            .get("calibration_ratios")
            .and_then(|v| v.as_object())
            .cloned()
    };
    let table = pinned_calibration_ratios(regime_of(r), &r.workload.curve.slug)
        .or_else(own)
        .ok_or_else(|| {
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
    priced_gae_with(r, w, Pricing::default())
}

/// [`priced_gae`] under `pricing`.
pub fn priced_gae_with(r: &Record, w: f64, pricing: Pricing) -> Result<Option<f64>, String> {
    let Some(g) = r.cost.total_gae else {
        return Ok(None);
    };
    let (u, missing) = unpriced_total_with(r, pricing);
    if missing && w > 0.0 {
        return Ok(None);
    }
    let (c, n_c) = calibrated_native(r)?;
    Ok(Some(g - c + w * (u + n_c) as f64))
}

/// `priced_S(w)`.
pub fn priced_s(r: &Record, w: f64) -> Result<Option<f64>, String> {
    priced_s_with(r, w, Pricing::default())
}

/// [`priced_s`] under `pricing`.
pub fn priced_s_with(r: &Record, w: f64, pricing: Pricing) -> Result<Option<f64>, String> {
    let sq = (r.workload.curve.r as f64).sqrt();
    Ok(priced_gae_with(r, w, pricing)?.map(|g| g / sq))
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
    /// Mean count per verified run of each [`TABLE_WORK_COUNTERS`] counter
    /// the arm counts behind a `counted:` marker ([`table_work_counts`]).
    /// Always listed; in `U` when the report's `priced_table_work` names
    /// the counter (the default since 2026-11-08).
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub table_work_mean_counts: BTreeMap<String, f64>,
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
    /// The counted table-work counters this report priced in `U`
    /// ([`Pricing::table_work`]): present in the default reading since
    /// 2026-11-08; absent when none was, which marks a band file as the
    /// pre-2026-11-08 reading (`--no-price-table-work`), the key as
    /// those files were written.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub priced_table_work: Vec<String>,
    pub rows: Vec<BandRow>,
    pub ranks: Vec<BandRank>,
}

/// The band report over `records` (warm-ups ignored), default pricing.
pub fn band(records: &[Record], weights: &[f64]) -> Result<BandReport, String> {
    band_with(records, weights, Pricing::default())
}

/// [`band`] under `pricing`.
pub fn band_with(
    records: &[Record],
    weights: &[f64],
    pricing: Pricing,
) -> Result<BandReport, String> {
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
                match priced_s_with(r, w, pricing)? {
                    Some(v) => vals.push(v),
                    None => all = false,
                }
            }
            mean_s.push(all.then(|| vals.iter().sum::<f64>() / vals.len() as f64));
        }
        let mut uncounted: Vec<String> = rs
            .iter()
            .flat_map(|r| {
                priced_counts_with(r, pricing)
                    .into_iter()
                    .filter(|(_, v)| v.is_none())
                    .map(|(k, _)| k)
            })
            .collect();
        uncounted.sort();
        uncounted.dedup();
        let (mut charged, mut native, mut native_ok) = (0.0f64, 0u64, true);
        for r in &ok {
            let (u, missing) = unpriced_total_with(r, pricing);
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
        let mut table_work_mean_counts: BTreeMap<String, f64> = BTreeMap::new();
        for r in &ok {
            for (name, n) in table_work_counts(r) {
                *table_work_mean_counts.entry(name).or_insert(0.0) += n as f64;
            }
        }
        for v in table_work_mean_counts.values_mut() {
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
            table_work_mean_counts,
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
        exempt_counters: NON_OPERATION_COUNTERS
            .iter()
            .map(|s| s.to_string())
            .collect(),
        priced_table_work: pricing.priced_table_work(),
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
    if rep.priced_table_work.is_empty() {
        s.push_str(
            "\nTable work listed, not priced: the pre-2026-11-08 reading (`--no-price-table-work`); counted in `work_native` behind a `counted:` marker, not listed in `cost.unpriced`.\n",
        );
    } else {
        s.push_str(&format!(
            "\nTable work priced in U at w per operation (the default since 2026-11-08; `--no-price-table-work` for the earlier reading): {}; counted in `work_native` behind a `counted:` marker, not listed in `cost.unpriced`.\n",
            rep.priced_table_work.join(", ")
        ));
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
        assert_eq!(
            priced_gae(&r, 1.0).unwrap().unwrap(),
            g0,
            "not in the price"
        );

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

    /// F4 field multiply-adds of `pkm-semaev` are native work: the record
    /// lists the solver term and the two `pkm_*_fp_mults` counters as
    /// unpriced, `unpriced_counts` resolves the solver term to `solver.ops`,
    /// and the band charges them at `w` per operation with nothing missing
    /// (`research/curve25519_algebraic_ecbench_20261102`, a real record of
    /// `prime-isogeny:t=6` with `pkm-semaev:m=2,guess=4` on the k = 20
    /// Curve25519 miniature).
    #[test]
    fn pkm_f4_multiply_adds_are_priced_by_the_band() {
        let text = include_str!(
            "../../../research/curve25519_algebraic_ecbench_20261102/fixtures/pkm_iso_t6_g4_target3.record.json"
        );
        let r: Record = serde_json::from_str(text).unwrap();
        let solver_name = "solver_F_p_multiply-adds_(elimination_only)_uncharged";
        let ops = r.solver.as_ref().expect("solver block").ops;
        assert!(ops > 1_000_000_000);
        let counts = unpriced_counts(&r);
        assert_eq!(counts[solver_name], Some(ops));
        assert_eq!(counts["pkm_grid_fp_mults_uncharged"], Some(31_808));
        assert_eq!(counts["pkm_system_fp_mults_uncharged"], Some(915_534));
        let (u, missing) = unpriced_total(&r);
        assert!(!missing, "every unpriced counter has a count");
        assert!(u >= ops + 31_808 + 915_534);
        let g = r.cost.total_gae.unwrap();
        assert_eq!(priced_gae(&r, 0.0).unwrap().unwrap(), g);
        let g1 = priced_gae(&r, 1.0).unwrap().unwrap();
        assert!((g1 - g - u as f64).abs() < 1e-6);
        // At w = 0.1 the F4 work alone is at least ops / 10 GAE.
        assert!(priced_gae(&r, 0.1).unwrap().unwrap() >= g + ops as f64 / 10.0);
    }

    fn targetwalk_k20_s0() -> Vec<Record> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/curve25519_f1_targetwalk_20261108/sessions/k20-s0");
        crate::cryptanalysis::ecbench::runner::read_records(&dir).unwrap()
    }

    /// The pre-2026-11-08 reading is the committed one: `band_with(…,
    /// Pricing::pre_2026_11_08())` on the targetwalk study's k20-s0 session
    /// reproduces `data/band-k20-s0.json` (written by the study's frozen
    /// binary, before table work was priced) row for row, to 1e-9, and
    /// lists the table-work counts without pricing them.
    #[test]
    fn pre_2026_11_08_pricing_reproduces_the_committed_targetwalk_band_file() {
        let committed: serde_json::Value = serde_json::from_str(include_str!(
            "../../../research/curve25519_f1_targetwalk_20261108/data/band-k20-s0.json"
        ))
        .unwrap();
        let weights: Vec<f64> = committed["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.as_f64().unwrap())
            .collect();
        assert_eq!(weights, DEFAULT_WEIGHTS);
        let off = Pricing::pre_2026_11_08();
        assert_eq!(off, Pricing { table_work: false });
        assert_ne!(off, Pricing::default(), "the default prices table work");
        let report = band_with(&targetwalk_k20_s0(), &weights, off).unwrap();
        assert!(report.priced_table_work.is_empty(), "nothing extra priced");
        let rows = committed["rows"].as_array().unwrap();
        assert_eq!(rows.len(), report.rows.len());
        for (want, got) in rows.iter().zip(&report.rows) {
            assert_eq!(want["arm"].as_str().unwrap(), got.arm);
            assert_eq!(want["verified"].as_u64().unwrap(), got.verified);
            assert_eq!(
                want["native_sum"].as_u64().unwrap(),
                got.native_sum.unwrap()
            );
            for (k, m) in want["mean_s"].as_array().unwrap().iter().enumerate() {
                let (m, g) = (m.as_f64().unwrap(), got.mean_s[k].unwrap());
                assert!(
                    (g - m).abs() < 1e-9,
                    "{} w = {}: {g} vs {m}",
                    got.arm,
                    weights[k]
                );
            }
            let listed = got.table_work_mean_counts.get("table_insert_attempts");
            if got.arm == "rho-neg" {
                assert!(listed.is_none(), "rho-neg counts no table work");
            } else {
                assert!(listed.is_some_and(|&n| n > 100.0), "{}: listed", got.arm);
            }
        }
        // The pre-2026-11-08 JSON keeps its keys: no `priced_table_work`
        // key (its absence marks the reading), and the table-work column is
        // the one addition.
        let json = serde_json::to_value(&report).unwrap();
        assert!(json.get("priced_table_work").is_none());
        for (want, got) in rows.iter().zip(json["rows"].as_array().unwrap()) {
            for key in want.as_object().unwrap().keys() {
                assert!(got.get(key).is_some(), "key {key} kept");
            }
        }
        for rank in committed["ranks"].as_array().unwrap() {
            let got = report
                .ranks
                .iter()
                .find(|x| x.a == rank["a"].as_str().unwrap() && x.b == rank["b"].as_str().unwrap())
                .unwrap();
            assert_eq!(got.verdict, rank["verdict"].as_str().unwrap());
        }
    }

    /// The converse: the default band on the same session differs from the
    /// committed (pre-2026-11-08) file by exactly the insert term.  Per arm
    /// and weight, `S(w)` is the committed value plus
    /// `w · mean(table_insert_attempts) / √r` (zero for rho-neg, which
    /// counts no table work), `native_sum` is the committed sum plus the
    /// summed inserts, `charged_sum` is unchanged, and the report names
    /// the priced counter, in the JSON too.
    #[test]
    fn default_band_differs_from_the_committed_file_by_the_insert_term_only() {
        let committed: serde_json::Value = serde_json::from_str(include_str!(
            "../../../research/curve25519_f1_targetwalk_20261108/data/band-k20-s0.json"
        ))
        .unwrap();
        let records = targetwalk_k20_s0();
        let report = band(&records, &DEFAULT_WEIGHTS).unwrap();
        assert_eq!(
            report.priced_table_work,
            vec!["table_insert_attempts".to_string()],
            "priced by default"
        );
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(
            json["priced_table_work"],
            serde_json::json!(["table_insert_attempts"])
        );
        let rows = committed["rows"].as_array().unwrap();
        assert_eq!(rows.len(), report.rows.len());
        let mut changed = 0;
        for (want, got) in rows.iter().zip(&report.rows) {
            assert_eq!(want["arm"].as_str().unwrap(), got.arm);
            assert_eq!(want["verified"].as_u64().unwrap(), got.verified);
            let inserts = got
                .table_work_mean_counts
                .get("table_insert_attempts")
                .copied()
                .unwrap_or(0.0);
            if got.arm == "rho-neg" {
                assert_eq!(inserts, 0.0, "rho-neg counts no table work");
            } else {
                assert!(inserts > 100.0, "{}: counts table work", got.arm);
                changed += 1;
            }
            let want_native = want["native_sum"].as_u64().unwrap();
            let delta = got.native_sum.unwrap() - want_native;
            assert!(
                (delta as f64 - inserts * got.verified as f64).abs() < 1e-6,
                "{}: native_sum rises by the summed inserts",
                got.arm
            );
            assert!(
                (want["charged_sum"].as_f64().unwrap() - got.charged_sum.unwrap()).abs() < 1e-9,
                "{}: charged GAE unchanged",
                got.arm
            );
            let sqrt_r = 2f64.powf(got.log2_r / 2.0);
            for (k, m) in want["mean_s"].as_array().unwrap().iter().enumerate() {
                let (m, g, w) = (
                    m.as_f64().unwrap(),
                    got.mean_s[k].unwrap(),
                    DEFAULT_WEIGHTS[k],
                );
                assert!(
                    (g - m - w * inserts / sqrt_r).abs() < 1e-9,
                    "{} w = {w}: {g} vs {m} + {}",
                    got.arm,
                    w * inserts / sqrt_r
                );
            }
        }
        assert_eq!(changed, 3, "the three walk-table arms moved");
        // At w = 0 nothing moves, so every committed verdict that the band
        // changes is one that was unresolved inside the band.
        for rank in committed["ranks"].as_array().unwrap() {
            let got = report
                .ranks
                .iter()
                .find(|x| x.a == rank["a"].as_str().unwrap() && x.b == rank["b"].as_str().unwrap())
                .unwrap();
            let want = rank["verdict"].as_str().unwrap();
            if got.verdict != want {
                assert_eq!(want, "unresolved inside the band", "{} vs {}", got.a, got.b);
            }
        }
    }

    /// With table work priced (the default) against the pre-2026-11-08
    /// reading, rho-neg (no marker, no table) is unchanged, twcj-x15's
    /// `S(w)` rises by exactly `w · mean(table_insert_attempts) / √r`, and
    /// the pair's verdict goes from unresolved to `rho-neg < twcj-x15`.
    #[test]
    fn pricing_table_work_charges_the_walk_arms_and_resolves_the_rho_rank() {
        let records = targetwalk_k20_s0();
        let on = Pricing::default();
        let off = Pricing::pre_2026_11_08();
        assert!(on.table_work);
        let base = band_with(&records, &DEFAULT_WEIGHTS, off).unwrap();
        let priced = band(&records, &DEFAULT_WEIGHTS).unwrap();
        assert_eq!(
            serde_json::to_value(&priced).unwrap(),
            serde_json::to_value(band_with(&records, &DEFAULT_WEIGHTS, on).unwrap()).unwrap(),
            "`band` is `band_with` under the default"
        );
        assert_eq!(
            priced.priced_table_work,
            vec!["table_insert_attempts".to_string()]
        );
        let row =
            |rep: &BandReport, arm: &str| rep.rows.iter().find(|r| r.arm == arm).unwrap().clone();
        let (rho0, rho1) = (row(&base, "rho-neg"), row(&priced, "rho-neg"));
        assert_eq!(rho0.mean_s, rho1.mean_s, "rho-neg is unchanged");
        assert_eq!(rho0.native_sum, rho1.native_sum);
        let (tw0, tw1) = (row(&base, "twcj-x15"), row(&priced, "twcj-x15"));
        let inserts = tw1.table_work_mean_counts["table_insert_attempts"];
        assert_eq!(
            tw0.table_work_mean_counts, tw1.table_work_mean_counts,
            "listed either way"
        );
        let sqrt_r = 2f64.powf(tw0.log2_r / 2.0);
        for (k, &w) in DEFAULT_WEIGHTS.iter().enumerate() {
            let rise = tw1.mean_s[k].unwrap() - tw0.mean_s[k].unwrap();
            assert!(
                (rise - w * inserts / sqrt_r).abs() < 1e-9,
                "w = {w}: rise {rise} vs {}",
                w * inserts / sqrt_r
            );
        }
        // `native_sum` grows by the summed inserts, so the integer re-check holds.
        let n = tw1.native_sum.unwrap() - tw0.native_sum.unwrap();
        assert!((n as f64 - inserts * tw1.verified as f64).abs() < 1e-6);
        assert_eq!(tw0.charged_sum, tw1.charged_sum);
        // Per record: U rises by exactly the record's table work.
        for r in records.iter().filter(|r| r.arm == "twcj-x15" && r.counts()) {
            let t = table_work_counts(r)["table_insert_attempts"];
            assert!(t > 0);
            assert_eq!(unpriced_total(r).0, unpriced_total_with(r, off).0 + t);
            assert_eq!(priced_counts(r)["table_insert_attempts"], Some(t));
            assert!(!priced_counts_with(r, off).contains_key("table_insert_attempts"));
        }
        let verdict = |rep: &BandReport| {
            rep.ranks
                .iter()
                .find(|x| x.a == "rho-neg" && x.b == "twcj-x15")
                .unwrap()
                .verdict
                .clone()
        };
        assert_eq!(verdict(&base), "unresolved inside the band");
        assert_eq!(verdict(&priced), "rho-neg < twcj-x15");
        assert!(markdown(&priced).contains("Table work priced"));
        assert!(!markdown(&base).contains("Table work priced"));
        assert!(markdown(&base).contains("pre-2026-11-08 reading"));
    }

    /// A record that carries `work_native.table_insert_attempts` with no
    /// `counted:table_insert_attempts` marker is not charged: the count is
    /// not known to be complete.
    #[test]
    fn table_work_without_its_counted_marker_is_not_charged() {
        let records = targetwalk_k20_s0();
        let on = Pricing::default();
        let off = Pricing::pre_2026_11_08();
        let base = records
            .iter()
            .find(|r| r.arm == "rho-neg" && r.counts())
            .unwrap();
        let g0 = priced_gae(base, 1.0).unwrap().unwrap();
        assert_eq!(
            priced_gae_with(base, 1.0, off).unwrap().unwrap(),
            g0,
            "no table work either way"
        );
        let mut r = base.clone();
        r.phases[0]
            .work_native
            .insert("table_insert_attempts".to_string(), 5_000);
        assert!(table_work_counts(&r).is_empty(), "no marker, no count");
        assert_eq!(unpriced_total_with(&r, on), unpriced_total(base));
        assert_eq!(priced_gae_with(&r, 1.0, on).unwrap().unwrap(), g0);
        assert_eq!(
            priced_gae(&r, 1.0).unwrap().unwrap(),
            g0,
            "the default reads the marker too"
        );
        // The marker in any phase makes the counter count (phase sum).
        r.phases[1]
            .work_native
            .insert("counted:table_insert_attempts".to_string(), 1);
        r.phases[1]
            .work_native
            .insert("table_insert_attempts".to_string(), 7);
        assert_eq!(table_work_counts(&r)["table_insert_attempts"], 5_007);
        assert_eq!(
            unpriced_total_with(&r, on).0,
            unpriced_total(base).0 + 5_007
        );
        let g1 = priced_gae_with(&r, 1.0, on).unwrap().unwrap();
        assert!((g1 - g0 - 5_007.0).abs() < 1e-6);
        assert_eq!(
            priced_gae(&r, 1.0).unwrap().unwrap(),
            g1,
            "the default is `on`"
        );
        // The pre-2026-11-08 reading never reads it.
        assert_eq!(priced_gae_with(&r, 1.0, off).unwrap().unwrap(), g0);
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
