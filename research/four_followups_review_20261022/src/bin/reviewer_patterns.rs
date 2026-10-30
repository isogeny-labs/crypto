//! Deterministic, reporting-only reviewer-pattern appendix.
//!
//! This binary reads the already-normalized review CSVs. It never invokes a
//! collector, solver, replay, matrix builder, nullspace routine, or recovery
//! routine.

use serde_json::json;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

type Row = BTreeMap<String, String>;
type TargetKey = (String, String, String, String);
type ModelClusters = BTreeMap<(String, String), BTreeMap<String, Vec<(f64, f64)>>>;

const COHORT: usize = 1260;
const REPS: usize = 10_000;
const DELTA_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "bits",
    "p",
    "target_index",
    "workload_id",
    "candidate_GAE",
    "rho_GAE",
    "candidate_minus_rho_GAE",
    "candidate_over_rho",
    "log2_candidate_over_rho",
    "comparison",
    "included_in_cost_analysis",
    "null_reason",
    "source_record",
];
const ECDF_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "bits",
    "p",
    "target_index",
    "workload_id",
    "ratio",
    "observed_rank",
    "observed_count",
    "cohort_count",
    "missing_count",
    "conditional_ecdf",
    "cohort_ecdf_lower",
    "cohort_ecdf_upper",
    "status",
];
const TAIL_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "slice",
    "cohort_count",
    "observed_count",
    "null_count",
    "wins",
    "ties",
    "losses",
    "min",
    "p01",
    "p05",
    "p25",
    "median",
    "p75",
    "p95",
    "p99",
    "max",
    "mean",
];
const BOOT_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "observed_models",
    "observed_targets",
    "cohort_count",
    "missing_targets",
    "replicates",
    "seed",
    "statistic",
    "point_estimate",
    "q025",
    "q975",
    "status",
];
const HEAT_FIELDS: &[&str] = &[
    "panel",
    "lead",
    "arm",
    "slice",
    "targets",
    "S_per_M",
    "I_per_M",
    "native_GAE_per_logical",
    "candidate_GAE",
    "rho_GAE",
    "candidate_over_rho",
    "derivation",
    "source_grid",
];
const BREAK_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "slice",
    "targets",
    "dM",
    "dS",
    "dI",
    "dL",
    "equation",
    "cells_checked",
    "max_abs_reconstruction_error_GAE",
    "max_relative_reconstruction_error",
    "source_grid",
];

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
fn val<'a>(r: &'a Row, k: &str) -> &'a str {
    r.get(k).map(String::as_str).unwrap_or("")
}
fn put(r: &mut Row, k: &str, v: impl ToString) {
    r.insert(k.into(), v.to_string());
}
fn num(x: &str) -> f64 {
    x.parse().unwrap_or_else(|_| panic!("invalid number: {x}"))
}
fn fmt(x: f64) -> String {
    format!("{x:.12}")
}
fn csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut value = String::new();
    let mut quoted = false;
    let mut chars = line.trim_end_matches('\r').chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                value.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => fields.push(std::mem::take(&mut value)),
            _ => value.push(ch),
        }
    }
    assert!(!quoted);
    fields.push(value);
    fields
}
fn read_csv(path: &Path) -> Vec<Row> {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();
    let head = csv_line(&lines.next().unwrap().unwrap());
    lines
        .map(|line| {
            let fields = csv_line(&line.unwrap());
            assert_eq!(head.len(), fields.len(), "{}", path.display());
            head.iter().cloned().zip(fields).collect()
        })
        .collect()
}
fn escape_csv(x: &str) -> String {
    if x.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", x.replace('"', "\"\""))
    } else {
        x.into()
    }
}
fn write_csv(path: &Path, fields: &[&str], rows: &[Row]) {
    let mut out = File::create(path).unwrap();
    writeln!(out, "{}", fields.join(",")).unwrap();
    for row in rows {
        writeln!(
            out,
            "{}",
            fields
                .iter()
                .map(|f| escape_csv(val(row, f)))
                .collect::<Vec<_>>()
                .join(",")
        )
        .unwrap();
    }
}
fn key(r: &Row) -> TargetKey {
    (
        val(r, "bits").into(),
        val(r, "p").into(),
        val(r, "target_index").into(),
        val(r, "workload_id").into(),
    )
}
fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap()
}
fn quantile(sorted: &[f64], q: f64) -> f64 {
    assert!(!sorted.is_empty() && (0.0..=1.0).contains(&q));
    let rank = ((q * sorted.len() as f64).ceil() as usize).max(1) - 1;
    sorted[rank.min(sorted.len() - 1)]
}

fn paired_deltas(targets: &[Row]) -> Vec<Row> {
    let mut out = Vec::new();
    for source in targets {
        let mut r = Row::new();
        for f in [
            "lead",
            "arm",
            "bits",
            "p",
            "target_index",
            "workload_id",
            "candidate_GAE",
            "rho_GAE",
            "candidate_over_rho",
            "source_record",
        ] {
            put(&mut r, f, val(source, f));
        }
        let coordinate = val(source, "arm") == "coordinate_identity_gate";
        let valid = !coordinate
            && val(source, "candidate_dlp_verified") == "True"
            && val(source, "rho_verified") == "True"
            && !val(source, "candidate_GAE").is_empty()
            && !val(source, "rho_GAE").is_empty()
            && !val(source, "candidate_over_rho").is_empty();
        if valid {
            let c = num(val(source, "candidate_GAE"));
            let rho = num(val(source, "rho_GAE"));
            let ratio = num(val(source, "candidate_over_rho"));
            assert!((c / rho - ratio).abs() <= 1e-9 * ratio.abs().max(1.0));
            put(&mut r, "candidate_minus_rho_GAE", fmt(c - rho));
            put(&mut r, "log2_candidate_over_rho", fmt(ratio.log2()));
            put(
                &mut r,
                "comparison",
                if ratio < 1.0 {
                    "win"
                } else if ratio > 1.0 {
                    "loss"
                } else {
                    "tie"
                },
            );
            put(&mut r, "included_in_cost_analysis", "true");
        } else {
            put(&mut r, "included_in_cost_analysis", "false");
            put(
                &mut r,
                "null_reason",
                if coordinate {
                    "coordinate_only_kummer_no_dlp_cost"
                } else if val(source, "failure").is_empty() {
                    "candidate_cost_or_verification_missing"
                } else {
                    val(source, "failure")
                },
            );
        }
        out.push(r);
    }
    assert_eq!(out.len(), 12_850);
    assert_eq!(
        out.iter()
            .filter(|r| val(r, "included_in_cost_analysis") == "true")
            .count(),
        11_586
    );
    assert_eq!(
        out.iter()
            .filter(|r| val(r, "null_reason") == "sparse_matrix_cap")
            .count(),
        4
    );
    assert_eq!(
        out.iter()
            .filter(|r| val(r, "null_reason") == "coordinate_only_kummer_no_dlp_cost")
            .count(),
        1_260
    );
    out
}

fn master_keys(targets: &[Row]) -> Vec<TargetKey> {
    let keys: BTreeSet<_> = targets
        .iter()
        .filter(|r| val(r, "lead") == "factor_base" && val(r, "arm") == "full_16")
        .map(key)
        .collect();
    assert_eq!(keys.len(), COHORT);
    keys.into_iter().collect()
}

fn ecdf_rows(targets: &[Row], master: &[TargetKey]) -> Vec<Row> {
    let arms: BTreeSet<_> = targets
        .iter()
        .filter(|r| val(r, "arm") != "coordinate_identity_gate")
        .map(|r| (val(r, "lead").to_string(), val(r, "arm").to_string()))
        .collect();
    let mut out = Vec::new();
    for (lead, arm) in arms {
        let mut observed: BTreeMap<TargetKey, f64> = BTreeMap::new();
        for r in targets.iter().filter(|r| {
            val(r, "lead") == lead
                && val(r, "arm") == arm
                && val(r, "candidate_dlp_verified") == "True"
                && val(r, "rho_verified") == "True"
                && !val(r, "candidate_over_rho").is_empty()
        }) {
            let old = observed.insert(key(r), num(val(r, "candidate_over_rho")));
            assert!(old.is_none(), "duplicate priced target for {lead}/{arm}");
        }
        let n = observed.len();
        assert!(n > 0 && n <= COHORT);
        let missing = COHORT - n;
        let mut sorted: Vec<_> = observed.values().copied().collect();
        sorted.sort_by(cmp_f64);
        for k in master {
            let mut r = Row::new();
            put(&mut r, "lead", &lead);
            put(&mut r, "arm", &arm);
            put(&mut r, "bits", &k.0);
            put(&mut r, "p", &k.1);
            put(&mut r, "target_index", &k.2);
            put(&mut r, "workload_id", &k.3);
            put(&mut r, "observed_count", n);
            put(&mut r, "cohort_count", COHORT);
            put(&mut r, "missing_count", missing);
            if let Some(&ratio) = observed.get(k) {
                let rank = sorted.partition_point(|x| *x <= ratio);
                put(&mut r, "ratio", fmt(ratio));
                put(&mut r, "observed_rank", rank);
                put(&mut r, "conditional_ecdf", fmt(rank as f64 / n as f64));
                put(
                    &mut r,
                    "cohort_ecdf_lower",
                    fmt(rank as f64 / COHORT as f64),
                );
                put(
                    &mut r,
                    "cohort_ecdf_upper",
                    fmt((rank + missing) as f64 / COHORT as f64),
                );
                put(&mut r, "status", "observed_verified_ratio");
            } else {
                put(&mut r, "status", "explicit_missing_ratio");
            }
            out.push(r);
        }
    }
    assert_eq!(out.len(), 13 * COHORT);
    out
}

fn tails(ecdf: &[Row]) -> Vec<Row> {
    let mut groups: BTreeMap<(String, String, String), Vec<f64>> = BTreeMap::new();
    let mut counts: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for r in ecdf {
        for slice in ["all", val(r, "bits")] {
            let k = (val(r, "lead").into(), val(r, "arm").into(), slice.into());
            *counts.entry(k.clone()).or_default() += 1;
            if !val(r, "ratio").is_empty() {
                groups.entry(k).or_default().push(num(val(r, "ratio")));
            }
        }
    }
    let mut out = Vec::new();
    for (k, total) in counts {
        let mut xs = groups.remove(&k).unwrap_or_default();
        xs.sort_by(cmp_f64);
        let n = xs.len();
        if n == 0 {
            continue;
        }
        let mut r = Row::new();
        put(&mut r, "lead", k.0);
        put(&mut r, "arm", k.1);
        put(&mut r, "slice", k.2);
        put(&mut r, "cohort_count", total);
        put(&mut r, "observed_count", n);
        put(&mut r, "null_count", total - n);
        put(&mut r, "wins", xs.iter().filter(|x| **x < 1.0).count());
        put(&mut r, "ties", xs.iter().filter(|x| **x == 1.0).count());
        put(&mut r, "losses", xs.iter().filter(|x| **x > 1.0).count());
        for (f, q) in [
            ("min", 0.0),
            ("p01", 0.01),
            ("p05", 0.05),
            ("p25", 0.25),
            ("median", 0.5),
            ("p75", 0.75),
            ("p95", 0.95),
            ("p99", 0.99),
            ("max", 1.0),
        ] {
            put(&mut r, f, fmt(quantile(&xs, q)));
        }
        put(&mut r, "mean", fmt(xs.iter().sum::<f64>() / n as f64));
        out.push(r);
    }
    out
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn pick(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn bootstrap(targets: &[Row]) -> Vec<Row> {
    let mut arms: ModelClusters = BTreeMap::new();
    for r in targets.iter().filter(|r| {
        val(r, "arm") != "coordinate_identity_gate"
            && val(r, "candidate_dlp_verified") == "True"
            && val(r, "rho_verified") == "True"
            && !val(r, "candidate_GAE").is_empty()
    }) {
        arms.entry((val(r, "lead").into(), val(r, "arm").into()))
            .or_default()
            .entry(val(r, "p").into())
            .or_default()
            .push((num(val(r, "candidate_GAE")), num(val(r, "rho_GAE"))));
    }
    let mut out = Vec::new();
    for ((lead, arm), models) in arms {
        let clusters: Vec<Vec<(f64, f64)>> = models.into_values().collect();
        let targets: usize = clusters.iter().map(Vec::len).sum();
        let sums = |sets: &[Vec<(f64, f64)>]| {
            let c: f64 = sets.iter().flatten().map(|x| x.0).sum();
            let r: f64 = sets.iter().flatten().map(|x| x.1).sum();
            c / r
        };
        let point = sums(&clusters);
        let mut seed = 0x9e3779b97f4a7c15u64;
        for b in format!("{lead}/{arm}").bytes() {
            seed = seed.rotate_left(5) ^ (b as u64);
        }
        let fixed_seed = seed;
        let mut rng = Rng(seed);
        let mut reps = Vec::with_capacity(REPS);
        for _ in 0..REPS {
            let picked: Vec<_> = (0..clusters.len())
                .map(|_| clusters[rng.pick(clusters.len())].clone())
                .collect();
            reps.push(sums(&picked));
        }
        reps.sort_by(cmp_f64);
        let mut r = Row::new();
        put(&mut r, "lead", lead);
        put(&mut r, "arm", arm);
        put(&mut r, "observed_models", clusters.len());
        put(&mut r, "observed_targets", targets);
        put(&mut r, "cohort_count", COHORT);
        put(&mut r, "missing_targets", COHORT - targets);
        put(&mut r, "replicates", REPS);
        put(&mut r, "seed", fixed_seed);
        put(
            &mut r,
            "statistic",
            "ratio_of_summed_candidate_GAE_to_summed_rho_GAE",
        );
        put(&mut r, "point_estimate", fmt(point));
        put(&mut r, "q025", fmt(quantile(&reps, 0.025)));
        put(&mut r, "q975", fmt(quantile(&reps, 0.975)));
        put(
            &mut r,
            "status",
            if targets == COHORT {
                "descriptive_frozen_21_model_resampling"
            } else {
                "descriptive_observed_model_resampling_with_missing_cohort"
            },
        );
        out.push(r);
    }
    out
}

fn cell_key(r: &Row) -> (String, String, String) {
    (
        val(r, "S_per_M").into(),
        val(r, "I_per_M").into(),
        val(r, "native_GAE_per_logical").into(),
    )
}
fn heatmap(grids: &[Row]) -> Vec<Row> {
    let specs = [
        ("Factor base full-16", "factor_base", "full_16", "all"),
        (
            "Projective retained state",
            "projective_law",
            "retained_state",
            "all",
        ),
        (
            "Pencil degree peel (observed pool)",
            "pencil_collector",
            "degree_peel",
            "pooled",
        ),
        (
            "Kummer differential uncached",
            "kummer_walk",
            "differential_uncached",
            "all",
        ),
    ];
    let mut out = Vec::new();
    for (panel, lead, arm, slice) in specs {
        if slice != "pooled" {
            for x in grids.iter().filter(|r| {
                val(r, "lead") == lead && val(r, "arm") == arm && val(r, "slice") == slice
            }) {
                let mut r = x.clone();
                put(&mut r, "panel", panel);
                put(&mut r, "derivation", "copied_complete_36_cell_aggregate");
                out.push(r);
            }
        } else {
            let source: Vec<_> = grids
                .iter()
                .filter(|r| val(r, "lead") == lead && val(r, "arm") == arm)
                .collect();
            let mut by: BTreeMap<_, Vec<&Row>> = BTreeMap::new();
            for x in source {
                by.entry(cell_key(x)).or_default().push(x);
            }
            assert_eq!(by.len(), 36);
            for ((s, i, n), xs) in by {
                assert_eq!(xs.len(), 10);
                let c: f64 = xs.iter().map(|r| num(val(r, "candidate_GAE"))).sum();
                let rho: f64 = xs.iter().map(|r| num(val(r, "rho_GAE"))).sum();
                let targets: usize = xs.iter().map(|r| num(val(r, "targets")) as usize).sum();
                let mut r = Row::new();
                put(&mut r, "panel", panel);
                put(&mut r, "lead", lead);
                put(&mut r, "arm", arm);
                put(&mut r, "slice", "observed_10_curve_pool");
                put(&mut r, "targets", targets);
                put(&mut r, "S_per_M", s);
                put(&mut r, "I_per_M", i);
                put(&mut r, "native_GAE_per_logical", n);
                put(&mut r, "candidate_GAE", fmt(c));
                put(&mut r, "rho_GAE", fmt(rho));
                put(&mut r, "candidate_over_rho", fmt(c / rho));
                put(
                    &mut r,
                    "derivation",
                    "sum_10_verified_curve_aggregates_then_divide",
                );
                put(
                    &mut r,
                    "source_grid",
                    xs.iter()
                        .map(|r| val(r, "source_grid"))
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(";"),
                );
                out.push(r);
            }
        }
    }
    assert_eq!(out.len(), 144);
    assert!(out
        .iter()
        .all(|r| val(r, "arm") != "coordinate_identity_gate"));
    out
}

fn break_even(grids: &[Row]) -> Vec<Row> {
    let mut groups: BTreeMap<(String, String, String), Vec<&Row>> = BTreeMap::new();
    for r in grids
        .iter()
        .filter(|r| !val(r, "candidate_GAE").is_empty() && !val(r, "rho_GAE").is_empty())
    {
        groups
            .entry((
                val(r, "lead").into(),
                val(r, "arm").into(),
                val(r, "slice").into(),
            ))
            .or_default()
            .push(r);
    }
    let mut out = Vec::new();
    for ((lead, arm, slice), rows) in groups {
        assert_eq!(rows.len(), 36);
        let mut d = BTreeMap::new();
        for r in &rows {
            d.insert(
                cell_key(r),
                num(val(r, "candidate_GAE")) - num(val(r, "rho_GAE")),
            );
        }
        let get = |s: &str, i: &str, n: &str| d[&(s.into(), i.into(), n.into())];
        let unit = |s: f64, i: f64| 9.0 + 4.0 * s + i;
        let raw = |s: &str, i: &str| get(s, i, "0") * unit(num(s), num(i));
        let di = ((raw("0", "10") - raw("0", "1")) / 9.0).round() as i128;
        let dm = (raw("0", "1") - di as f64).round() as i128;
        let ds = (raw("1", "1") - raw("0", "1")).round() as i128;
        let dl = (get("0", "1", "1") - get("0", "1", "0")).round() as i128;
        let mut abs: f64 = 0.0;
        let mut rel: f64 = 0.0;
        for ((s, i, n), actual) in &d {
            let (sf, iv, nv) = (num(s), num(i), num(n));
            let u = unit(sf, iv);
            let pred = (dm as f64 + ds as f64 * sf + di as f64 * iv + dl as f64 * nv * u) / u;
            let e = (pred - actual).abs();
            abs = abs.max(e);
            rel = rel.max(e / actual.abs().max(1.0));
        }
        assert!(
            rel < 1e-8,
            "break-even reconstruction {lead}/{arm}/{slice}: {rel}"
        );
        let mut r = Row::new();
        put(&mut r, "lead", &lead);
        put(&mut r, "arm", &arm);
        put(&mut r, "slice", &slice);
        put(&mut r, "targets", val(rows[0], "targets"));
        put(&mut r, "dM", dm);
        put(&mut r, "dS", ds);
        put(&mut r, "dI", di);
        put(&mut r, "dL", dl);
        put(
            &mut r,
            "equation",
            format!("{dm} + ({ds})*S + ({di})*I + ({dl})*N*(9+4*S+I) = 0"),
        );
        put(&mut r, "cells_checked", 36);
        put(
            &mut r,
            "max_abs_reconstruction_error_GAE",
            format!("{abs:.9}"),
        );
        put(
            &mut r,
            "max_relative_reconstruction_error",
            format!("{rel:.12e}"),
        );
        put(
            &mut r,
            "source_grid",
            rows.iter()
                .map(|r| val(r, "source_grid"))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(";"),
        );
        out.push(r);
    }
    assert_eq!(out.len(), 85);
    out
}

fn esc(x: &str) -> String {
    x.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn ecdf_x(v: f64) -> f64 {
    80.0 + ((v.log2() + 3.0) / 20.0).clamp(0.0, 1.0) * 760.0
}
fn ecdf_y(v: f64) -> f64 {
    435.0 - v * 370.0
}
fn right_continuous_step_path(data: &[(f64, f64, f64, f64)], slot: usize) -> String {
    assert!(!data.is_empty() && slot < 3);
    let mut unique = Vec::new();
    for point in data {
        if unique
            .last()
            .is_some_and(|last: &(f64, f64, f64, f64)| last.0 == point.0)
        {
            *unique.last_mut().unwrap() = *point;
        } else {
            unique.push(*point);
        }
    }
    let initial = if slot == 2 {
        unique[0].3 - unique[0].2
    } else {
        0.0
    };
    let mut path = format!("M {:.1} {:.1}", ecdf_x(2f64.powi(-3)), ecdf_y(initial));
    let mut previous = initial;
    for point in &unique {
        let level = [point.1, point.2, point.3][slot];
        assert!(level >= previous);
        path.push_str(&format!(" H {:.1} V {:.1}", ecdf_x(point.0), ecdf_y(level)));
        previous = level;
    }
    path.push_str(&format!(" H {:.1}", ecdf_x(2f64.powi(17))));
    path
}
fn draw_ecdf(path: &Path, ecdf: &[Row]) {
    let specs = [
        ("factor_base", "full_16", "#1764aa"),
        ("projective_law", "retained_state", "#8647a3"),
        ("pencil_collector", "peel_identity", "#c5761b"),
        ("kummer_walk", "differential_uncached", "#202020"),
    ];
    let mut svg=vec![r#"<svg xmlns="http://www.w3.org/2000/svg" width="1180" height="540" viewBox="0 0 1180 540"><rect width="100%" height="100%" fill="white"/>"#.to_string(),r#"<text x="80" y="28" font-family="sans-serif" font-size="17">Candidate / matched rho empirical distributions</text>"#.into()];
    for q in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let yy = ecdf_y(q);
        svg.push(format!(r##"<line x1="80" y1="{yy:.1}" x2="840" y2="{yy:.1}" stroke="#dde3e8"/><text x="69" y="{:.1}" text-anchor="end" font-family="sans-serif" font-size="11">{q:.2}</text>"##,yy+4.0));
    }
    for e in [-3, 0, 3, 6, 9, 12, 15, 17] {
        let ratio = 2f64.powi(e);
        let xx = ecdf_x(ratio);
        svg.push(format!(r##"<line x1="{xx:.1}" y1="65" x2="{xx:.1}" y2="435" stroke="#edf0f2"/><text x="{xx:.1}" y="455" text-anchor="middle" font-family="sans-serif" font-size="10">2^{e}</text>"##));
    }
    for (idx, (lead, arm, color)) in specs.iter().enumerate() {
        let mut data: Vec<(f64, f64, f64, f64)> = ecdf
            .iter()
            .filter(|r| {
                val(r, "lead") == *lead && val(r, "arm") == *arm && !val(r, "ratio").is_empty()
            })
            .map(|r| {
                (
                    num(val(r, "ratio")),
                    num(val(r, "conditional_ecdf")),
                    num(val(r, "cohort_ecdf_lower")),
                    num(val(r, "cohort_ecdf_upper")),
                )
            })
            .collect();
        data.sort_by(|a, b| cmp_f64(&a.0, &b.0));
        if *lead == "pencil_collector" {
            svg.push(format!(r#"<path data-ecdf="cohort-lower-right-continuous" d="{}" fill="none" stroke="{color}" stroke-width="1.2" stroke-dasharray="5 4" opacity="0.7"/><path data-ecdf="cohort-upper-right-continuous" d="{}" fill="none" stroke="{color}" stroke-width="1.2" stroke-dasharray="5 4" opacity="0.7"/>"#,right_continuous_step_path(&data,1),right_continuous_step_path(&data,2)));
        }
        svg.push(format!(
            r#"<path data-ecdf="conditional-right-continuous" d="{}" fill="none" stroke="{color}" stroke-width="2.3"/>"#,
            right_continuous_step_path(&data, 0)
        ));
        let label = format!("{}/{} (n={})", lead, arm, data.len());
        svg.push(format!(r#"<line x1="865" y1="{}" x2="892" y2="{}" stroke="{color}" stroke-width="3"/><text x="900" y="{}" font-family="sans-serif" font-size="11">{}</text>"#,80+idx*34,80+idx*34,84+idx*34,esc(&label)));
    }
    svg.push(r#"<text x="460" y="486" text-anchor="middle" font-family="sans-serif" font-size="12">Candidate / rho ratio (log₂ scale)</text><text x="80" y="516" font-family="sans-serif" font-size="10">Right-continuous steps. Solid: conditional ECDF of observed verified rows. Pencil dashed: 1,260-cohort bounds under arbitrary missing ratios.</text></svg>"#.into());
    fs::write(path, format!("{}\n", svg.join("\n"))).unwrap();
}

fn heat_color(v: f64, min: f64, max: f64) -> String {
    let t = ((v.log2() - min.log2()) / (max.log2() - min.log2())).clamp(0.0, 1.0);
    let r = (35.0 + 190.0 * t) as u8;
    let g = (110.0 - 55.0 * t) as u8;
    let b = (170.0 - 120.0 * t) as u8;
    format!("#{r:02x}{g:02x}{b:02x}")
}
fn draw_heatmap(path: &Path, rows: &[Row]) {
    let min = rows
        .iter()
        .map(|r| num(val(r, "candidate_over_rho")))
        .fold(f64::INFINITY, f64::min);
    let max = rows
        .iter()
        .map(|r| num(val(r, "candidate_over_rho")))
        .fold(f64::NEG_INFINITY, f64::max);
    let panels: Vec<_> = rows
        .iter()
        .map(|r| val(r, "panel"))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut svg=vec![r#"<svg xmlns="http://www.w3.org/2000/svg" width="1060" height="740" viewBox="0 0 1060 740"><rect width="100%" height="100%" fill="white"/><text x="65" y="28" font-family="sans-serif" font-size="17">36-cell price sensitivity: candidate / matched rho</text>"#.to_string()];
    let ss = ["0", "0.5", "1"];
    let ns = ["0", "0.1", "1"];
    let inv = ["1", "10", "100", "1000"];
    for (pi, panel) in panels.iter().enumerate() {
        let ox = 65 + (pi % 2) * 500;
        let oy = 65 + (pi / 2) * 320;
        svg.push(format!(
            r#"<text x="{ox}" y="{}" font-family="sans-serif" font-size="13">{}</text>"#,
            oy - 12,
            esc(panel)
        ));
        for (iy, i) in inv.iter().enumerate() {
            svg.push(format!(r#"<text x="{}" y="{}" text-anchor="end" font-family="sans-serif" font-size="10">I={i}</text>"#,ox-8,oy+iy*48+29));
            for (is, s) in ss.iter().enumerate() {
                for (inn, n) in ns.iter().enumerate() {
                    let r = rows
                        .iter()
                        .find(|r| {
                            val(r, "panel") == *panel
                                && val(r, "S_per_M") == *s
                                && val(r, "I_per_M") == *i
                                && val(r, "native_GAE_per_logical") == *n
                        })
                        .unwrap();
                    let ratio = num(val(r, "candidate_over_rho"));
                    let xx = ox + (is * 3 + inn) * 47;
                    let yy = oy + iy * 48;
                    svg.push(format!(r#"<rect x="{xx}" y="{yy}" width="44" height="44" fill="{}"/><text x="{}" y="{}" text-anchor="middle" font-family="sans-serif" font-size="8" fill="white">{ratio:.2}</text>"#,heat_color(ratio,min,max),xx+22,yy+26));
                }
            }
        }
        for (is, s) in ss.iter().enumerate() {
            svg.push(format!(r#"<text x="{}" y="{}" text-anchor="middle" font-family="sans-serif" font-size="9">S={s}</text>"#,ox+is*141+68,oy+214));
            for (inn, n) in ns.iter().enumerate() {
                svg.push(format!(r#"<text x="{}" y="{}" text-anchor="middle" font-family="sans-serif" font-size="8">N={n}</text>"#,ox+(is*3+inn)*47+22,oy+229));
            }
        }
    }
    svg.push(format!(r#"<text x="65" y="718" font-family="sans-serif" font-size="10">Color scale is log₂ ratio across these 144 cells; numeric labels are exact derived grid ratios. Range {min:.3}–{max:.3}.</text></svg>"#));
    fs::write(path, format!("{}\n", svg.join("\n"))).unwrap();
}

#[cfg(test)]
mod tests {
    use super::right_continuous_step_path;

    #[test]
    fn ecdf_paths_use_horizontal_segments_then_vertical_jumps() {
        let data = [(1.0, 0.5, 0.25, 0.75), (2.0, 1.0, 0.5, 1.0)];
        let conditional = right_continuous_step_path(&data, 0);
        assert!(conditional.starts_with("M 80.0 435.0 H "));
        assert_eq!(conditional.matches(" H ").count(), 3);
        assert_eq!(conditional.matches(" V ").count(), 2);
        assert!(!conditional.contains(" L "));

        let upper = right_continuous_step_path(&data, 2);
        assert!(upper.starts_with("M 80.0 250.0 H "));
        assert_eq!(upper.matches(" H ").count(), 3);
        assert_eq!(upper.matches(" V ").count(), 2);
    }
}

fn main() {
    let base = here();
    let out = base.join("reviewer-patterns");
    fs::create_dir_all(&out).unwrap();
    let targets = read_csv(&base.join("results/standard-targets.csv"));
    let grids = read_csv(&base.join("results/standard-cost-grid.csv"));
    let master = master_keys(&targets);
    let deltas = paired_deltas(&targets);
    let ecdf = ecdf_rows(&targets, &master);
    let tails = tails(&ecdf);
    let boot = bootstrap(&targets);
    let heat = heatmap(&grids);
    let breaks = break_even(&grids);
    write_csv(&out.join("paired-deltas.csv"), DELTA_FIELDS, &deltas);
    write_csv(&out.join("ecdf.csv"), ECDF_FIELDS, &ecdf);
    write_csv(&out.join("descriptive-tails.csv"), TAIL_FIELDS, &tails);
    write_csv(&out.join("cluster-bootstrap.csv"), BOOT_FIELDS, &boot);
    write_csv(&out.join("price-cell-heatmap.csv"), HEAT_FIELDS, &heat);
    write_csv(&out.join("break-even.csv"), BREAK_FIELDS, &breaks);
    draw_ecdf(&out.join("ecdf.svg"), &ecdf);
    draw_heatmap(&out.join("price-cell-heatmap.svg"), &heat);
    let summary = json!({"schema":"reviewer-patterns/v1","generator_package_version":env!("CARGO_PKG_VERSION"),"inputs":["results/standard-targets.csv","results/standard-cost-grid.csv"],"reporting_only":true,"n5_status":"partial_missing_phase_fractions","standard_target_rows":targets.len(),"paired_cost_rows":deltas.iter().filter(|r|val(r,"included_in_cost_analysis")=="true").count(),"explicit_null_rows":deltas.iter().filter(|r|val(r,"included_in_cost_analysis")=="false").count(),"coordinate_only_kummer_rows_excluded_from_cost_plots":1260,"pencil_cap_null_rows":4,"ecdf_rows":ecdf.len(),"ecdf_rendering":"right_continuous_empirical_steps","tail_rows":tails.len(),"bootstrap_rows":boot.len(),"bootstrap_replicates_per_arm":REPS,"heatmap_rows":heat.len(),"break_even_groups":breaks.len(),"phase_fraction_output":{"status":"not_generated","reason":"the cross-lead standardized target schema contains total representative costs but no common phase-counter columns; source-specific phase normalization was not added in this reporting pass"}});
    fs::write(
        out.join("summary.json"),
        format!("{}\n", serde_json::to_string_pretty(&summary).unwrap()),
    )
    .unwrap();
    println!("wrote reviewer patterns: {} paired rows, {} ECDF rows, {} heatmap cells, {} break-even surfaces",deltas.len(),ecdf.len(),heat.len(),breaks.len());
}
