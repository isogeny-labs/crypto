//! From raw harness rows to the performance index: per-kernel paired
//! log-ratios, the A/A spread, the verdict, and the weighted geometric mean.
//! The formula and its limits are in `docs/perf/PERFORMANCE_INDEX.md`.

use crate::stats::{
    bootstrap_median_ci, hodges_lehmann, median, median_ci_exact, sign_test_p, BOOTSTRAP,
};
use crate::util::{num, SplitMix64};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// A kernel must move by more than 2% to be called faster or slower.
pub fn ln_floor() -> f64 {
    1.02f64.ln()
}
/// Fewer usable paired rounds than this is "insufficient".
pub const MIN_ROUNDS_FOR_VERDICT: usize = 3;
/// AGENTS.md section 10: at least five interleaved rounds.
pub const MIN_ROUNDS_QUALIFIED: usize = 5;
/// A/A pairs needed before a spread means anything.
pub const MIN_AA_FOR_NOISE: usize = 3;
/// At one thread, CPU time / wall time below this means the process was descheduled.
pub const DEFAULT_MIN_CPU_RATIO: f64 = 0.90;
/// Hypervisor steal, as a fraction of an invocation's wall time.
pub const DEFAULT_MAX_STEAL: f64 = 0.05;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KernelInfo {
    pub id: String,
    pub area: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arm {
    A,
    B,
    A2,
}

impl Arm {
    pub fn name(self) -> &'static str {
        match self {
            Arm::A => "A",
            Arm::B => "B",
            Arm::A2 => "A2",
        }
    }
    pub fn parse(s: &str) -> Option<Arm> {
        match s {
            "A" => Some(Arm::A),
            "B" => Some(Arm::B),
            "A2" => Some(Arm::A2),
            _ => None,
        }
    }
}

/// Rows of one kernel, by arm, each keyed by the round that produced it.
#[derive(Default, Clone, Debug)]
pub struct ArmRows {
    pub a: BTreeMap<usize, Value>,
    pub b: BTreeMap<usize, Value>,
    pub a2: BTreeMap<usize, Value>,
}

impl ArmRows {
    pub fn arm(&self, arm: Arm) -> &BTreeMap<usize, Value> {
        match arm {
            Arm::A => &self.a,
            Arm::B => &self.b,
            Arm::A2 => &self.a2,
        }
    }
    pub fn arm_mut(&mut self, arm: Arm) -> &mut BTreeMap<usize, Value> {
        match arm {
            Arm::A => &mut self.a,
            Arm::B => &mut self.b,
            Arm::A2 => &mut self.a2,
        }
    }
}

pub type Per = BTreeMap<String, ArmRows>;

/// Groups the `invocation` records of a raw file by kernel, arm and round.
pub fn group_raw(kernels: &[KernelInfo], raw: &[Value]) -> Per {
    let mut per: Per = kernels
        .iter()
        .map(|k| (k.id.clone(), ArmRows::default()))
        .collect();
    for rec in raw {
        if rec.get("type").and_then(Value::as_str) != Some("invocation") {
            continue;
        }
        let (Some(kid), Some(arm), Some(round), Some(row)) = (
            rec.get("kernel").and_then(Value::as_str),
            rec.get("arm").and_then(Value::as_str).and_then(Arm::parse),
            rec.get("round").and_then(Value::as_u64),
            rec.get("row"),
        ) else {
            continue;
        };
        if let Some(rows) = per.get_mut(kid) {
            rows.arm_mut(arm).insert(round as usize, row.clone());
        }
    }
    per
}

#[derive(Clone, Debug)]
pub struct Params {
    pub threads: usize,
    pub exclude_contended: bool,
    pub min_cpu_ratio: f64,
    pub max_invol_per_s: Option<f64>,
    pub max_steal: f64,
    pub rounds_planned: Option<usize>,
    /// Each arm has one exact observation (instruction counts): one is
    /// conclusive, there is no spread to measure, and no rounds to count.
    pub deterministic: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            threads: 1,
            exclude_contended: true,
            min_cpu_ratio: DEFAULT_MIN_CPU_RATIO,
            max_invol_per_s: None,
            max_steal: DEFAULT_MAX_STEAL,
            rounds_planned: None,
            deterministic: false,
        }
    }
}

/// Reasons a harness row cannot be trusted at all (not contention: that is a
/// property of the host; these are properties of the row).
pub fn row_problems(row: &Value, kid: &str, threads: Option<usize>) -> Vec<String> {
    let mut bad = Vec::new();
    if row.get("id").and_then(Value::as_str) != Some(kid) {
        bad.push(format!(
            "row id {:?} is not the requested kernel",
            row.get("id")
        ));
    }
    let fp_ok = row
        .get("fingerprint")
        .and_then(Value::as_str)
        .is_some_and(|s| {
            s.len() == 16
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        });
    if !fp_ok {
        bad.push(format!(
            "fingerprint {:?} is not 16 hex digits",
            row.get("fingerprint")
        ));
    }
    if !num(row, "median_ns").is_some_and(|m| m > 0.0) {
        bad.push(format!(
            "median_ns {:?} is not positive",
            row.get("median_ns")
        ));
    }
    if let (Some(want), Some(got)) = (threads, row.get("threads").and_then(Value::as_u64)) {
        if got as usize != want {
            bad.push(format!(
                "the harness ran {got} threads, {want} were requested"
            ));
        }
    }
    bad
}

/// Why this invocation looks descheduled, from what the process reported
/// about itself.  Empty means no sign of it (or no data: a row from an old
/// harness has no accounting fields and cannot be classified).
pub fn contention(
    row: &Value,
    threads: usize,
    min_cpu_ratio: f64,
    max_invol_per_s: Option<f64>,
    max_steal: f64,
) -> Vec<String> {
    let mut why = Vec::new();
    if threads == 1 {
        if let (Some(cpu), Some(wall)) = (num(row, "median_cpu_ns"), num(row, "median_ns")) {
            if wall > 0.0 {
                let ratio = cpu / wall;
                if ratio < min_cpu_ratio {
                    why.push(format!("cpu/wall {ratio:.2} < {min_cpu_ratio:.2}"));
                }
            }
        }
    }
    // Involuntary context switches count every thread of the process.  A kernel
    // with a private thread pool, pinned to one CPU, preempts itself hundreds of
    // times a second with nothing else running (measured: 441 of 490 flags in a
    // 2040-invocation null run, at cpu/wall 1.00 and no steal), so this test is
    // off unless asked for: the figure is recorded in every row either way.
    if let (Some(limit), Some(invol)) = (max_invol_per_s, num(row, "invol_csw")) {
        let secs: f64 = row
            .get("samples_ns")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_f64).sum::<f64>() / 1e9)
            .unwrap_or(0.0);
        if secs > 0.0 && invol / secs > limit {
            why.push(format!(
                "{:.0} involuntary context switches/s",
                invol / secs
            ));
        }
    }
    // Ticks are 1/CLK_TCK s (10 ms): one tick is within the clock's own jitter
    // and, on a short invocation, a large fraction of it, so two are required.
    if let (Some(steal), Some(tck), Some(inv)) = (
        num(row, "steal_ticks"),
        num(row, "clk_tck"),
        num(row, "invocation_s"),
    ) {
        if steal >= 2.0 && tck > 0.0 && inv > 0.0 {
            let frac = (steal / tck) / inv;
            if frac > max_steal {
                why.push(format!(
                    "hypervisor steal {:.1}% of the invocation",
                    100.0 * frac
                ));
            }
        }
    }
    why
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelSummary {
    pub id: String,
    pub area: String,
    pub fingerprints: Vec<String>,
    pub valid: bool,
    pub base_median_ns: Option<f64>,
    pub cand_median_ns: Option<f64>,
    pub speedup: f64,
    pub speedup_ci95: [f64; 2],
    pub speedup_min: Option<f64>,
    pub corroborated: bool,
    pub speedup_hl: Option<f64>,
    pub sign_test_p: Option<f64>,
    pub speedup_ci_exact: Option<[f64; 2]>,
    pub ci_exact_coverage: Option<f64>,
    pub aa_noise_ln: Option<f64>,
    pub aa_pairs: usize,
    pub verdict: String,
    pub rounds_used: usize,
    pub rounds_contended: Vec<usize>,
    pub contended_why: BTreeMap<String, Vec<String>>,
    pub rounds_total: usize,
    pub budget_hit_rows: usize,
    pub maxrss_kb: Option<u64>,
    pub d: Vec<f64>,
    pub n: Vec<f64>,
    pub d_by_round: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AreaSummary {
    pub index: Option<f64>,
    pub weight: f64,
    pub kernels: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    pub valid: bool,
    pub qualified: bool,
    pub performance_index: Option<f64>,
    pub performance_index_ci95: Option<[f64; 2]>,
    pub areas: BTreeMap<String, AreaSummary>,
    pub aa_noise_median_pct: Option<f64>,
    pub kernels: Vec<KernelSummary>,
    pub rounds: usize,
    pub rounds_planned: usize,
    pub exclude_contended: bool,
    pub warnings: Vec<String>,
    /// What the command that produced this adds: mode, unit, hosts, cost, ...
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

/// A list of strings as Python would print it, which is how the reference
/// implementation words its warnings.
fn py_list(xs: &[String]) -> String {
    format!(
        "[{}]",
        xs.iter()
            .map(|x| format!("'{x}'"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn median_of_rows(rows: &BTreeMap<usize, Value>, rounds: &[usize]) -> Option<f64> {
    let v: Vec<f64> = rounds
        .iter()
        .filter_map(|i| rows.get(i).and_then(|r| num(r, "median_ns")))
        .collect();
    (!v.is_empty()).then(|| median(&v))
}

pub fn summarize(
    kernels: &[KernelInfo],
    per: &Per,
    weights: &BTreeMap<String, f64>,
    rng: &mut SplitMix64,
    p: &Params,
) -> Summary {
    let mut rows: Vec<KernelSummary> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let floor = ln_floor();
    let empty = ArmRows::default();
    for k in kernels {
        let rec = per.get(&k.id).unwrap_or(&empty);
        let mut fps: BTreeSet<String> = BTreeSet::new();
        for arm in [Arm::A, Arm::B, Arm::A2] {
            for r in rec.arm(arm).values() {
                if let Some(f) = r.get("fingerprint").and_then(Value::as_str) {
                    fps.insert(f.to_string());
                }
            }
        }
        // (arm, round) -> reasons
        let mut why: BTreeMap<(usize, Arm), Vec<String>> = BTreeMap::new();
        for arm in [Arm::A, Arm::B, Arm::A2] {
            for (&rnd, row) in rec.arm(arm) {
                let w = contention(
                    row,
                    p.threads,
                    p.min_cpu_ratio,
                    p.max_invol_per_s,
                    p.max_steal,
                );
                if !w.is_empty() {
                    why.insert((rnd, arm), w);
                }
            }
        }
        let contended_rounds: Vec<usize> = why
            .keys()
            .map(|&(r, _)| r)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let usable = |arms: &[Arm]| -> Vec<usize> {
            let mut rs: BTreeSet<usize> = rec.arm(arms[0]).keys().copied().collect();
            for &a in &arms[1..] {
                let other: BTreeSet<usize> = rec.arm(a).keys().copied().collect();
                rs = rs.intersection(&other).copied().collect();
            }
            if p.exclude_contended {
                rs.retain(|rnd| !arms.iter().any(|&a| why.contains_key(&(*rnd, a))));
            }
            rs.into_iter().collect()
        };
        let rounds_d = usable(&[Arm::A, Arm::B]);
        let rounds_n = usable(&[Arm::A, Arm::A2]);
        let med_ns = |arm: Arm, i: usize| num(&rec.arm(arm)[&i], "median_ns").unwrap_or(f64::NAN);
        let d_by_round: BTreeMap<usize, f64> = rounds_d
            .iter()
            .map(|&i| (i, (med_ns(Arm::A, i) / med_ns(Arm::B, i)).ln()))
            .collect();
        let n_vals: Vec<f64> = rounds_n
            .iter()
            .map(|&i| (med_ns(Arm::A, i) / med_ns(Arm::A2, i)).ln())
            .collect();
        let d: Vec<f64> = rounds_d.iter().map(|i| d_by_round[i]).collect();
        let valid = fps.len() == 1;
        let (med, lo, hi) = if d.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            let (lo, hi) = bootstrap_median_ci(&d, rng);
            (median(&d), lo, hi)
        };
        let noise = (n_vals.len() >= MIN_AA_FOR_NOISE)
            .then(|| median(&n_vals.iter().map(|x| x.abs()).collect::<Vec<_>>()) * 1.4826);
        let thresh = (2.0 * noise.unwrap_or(0.0)).max(floor);
        let min_rounds = if p.deterministic {
            1
        } else {
            MIN_ROUNDS_FOR_VERDICT
        };
        let mut verdict = if !valid {
            "INVALID"
        } else if d.len() < min_rounds {
            "insufficient"
        } else if p.deterministic {
            // An exact count that moved by more than the build-noise floor.
            if med > floor {
                "faster"
            } else if -med > floor {
                "slower"
            } else {
                "neutral"
            }
        } else if lo > 0.0 && med > thresh {
            "faster"
        } else if hi < 0.0 && -med > thresh {
            "slower"
        } else {
            "neutral"
        }
        .to_string();
        if !p.deterministic && (verdict == "faster" || verdict == "slower") && noise.is_none() {
            verdict = format!("unqualified-{verdict}"); // no A/A spread: not a result under AGENTS.md section 10
        }
        let mins = |arm: Arm| -> Vec<f64> {
            rec.arm(arm)
                .iter()
                .filter(|(i, _)| d_by_round.contains_key(i))
                .filter_map(|(_, r)| num(r, "min_ns"))
                .collect()
        };
        let (min_a, min_b) = (mins(Arm::A), mins(Arm::B));
        let exact = median_ci_exact(&d, 0.95);
        // The best-of ratio (fastest sample of each arm, over the used rounds)
        // is not moved by a few slow samples.  A verdict whose median says
        // "slower" while best-of says "no change" is a verdict about interference.
        let fmin = |v: &[f64]| v.iter().copied().fold(f64::INFINITY, f64::min);
        let best = (!min_a.is_empty() && !min_b.is_empty()).then(|| fmin(&min_a) / fmin(&min_b));
        let corroborated = if verdict.ends_with("faster") {
            best.is_none_or(|b| b.ln() > floor)
        } else if verdict.ends_with("slower") {
            best.is_none_or(|b| b.ln() < -floor)
        } else {
            true
        };
        let all_rows = || rec.a.values().chain(rec.b.values()).chain(rec.a2.values());
        let mut contended_why = BTreeMap::new();
        for ((rnd, arm), w) in &why {
            contended_why.insert(format!("{}@{}", arm.name(), rnd), w.clone());
        }
        rows.push(KernelSummary {
            id: k.id.clone(),
            area: k.area.clone(),
            fingerprints: fps.iter().cloned().collect(),
            valid,
            base_median_ns: median_of_rows(&rec.a, &rounds_d),
            cand_median_ns: median_of_rows(&rec.b, &rounds_d),
            speedup: med.exp(),
            speedup_ci95: [lo.exp(), hi.exp()],
            speedup_min: best,
            corroborated,
            speedup_hl: hodges_lehmann(&d).map(f64::exp),
            sign_test_p: sign_test_p(&d),
            speedup_ci_exact: exact.map(|e| [e.0.exp(), e.1.exp()]),
            ci_exact_coverage: exact.map(|e| e.2),
            aa_noise_ln: noise,
            aa_pairs: n_vals.len(),
            verdict,
            rounds_used: d.len(),
            rounds_contended: contended_rounds,
            contended_why,
            rounds_total: rec.a.keys().filter(|i| rec.b.contains_key(i)).count(),
            budget_hit_rows: all_rows()
                .filter(|r| r.get("budget_hit").and_then(Value::as_bool) == Some(true))
                .count(),
            maxrss_kb: all_rows()
                .filter_map(|r| r.get("maxrss_kb").and_then(Value::as_u64))
                .max(),
            d,
            n: n_vals,
            d_by_round: d_by_round
                .iter()
                .map(|(i, v)| (i.to_string(), *v))
                .collect(),
        });
    }
    let mut by_area: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        by_area.entry(r.area.clone()).or_default().push(i);
    }
    let areas: Vec<String> = by_area.keys().cloned().collect();
    let defaulted: Vec<String> = areas
        .iter()
        .filter(|a| !weights.contains_key(*a))
        .cloned()
        .collect();
    if !defaulted.is_empty() {
        warnings.push(format!(
            "areas without a weight in the weights file counted as 1.0: {}",
            py_list(&defaulted)
        ));
    }
    let w_raw: Vec<f64> = areas
        .iter()
        .map(|a| weights.get(a).copied().unwrap_or(1.0))
        .collect();
    let mut wsum: f64 = w_raw.iter().sum();
    if wsum == 0.0 {
        wsum = 1.0;
    }
    let w: BTreeMap<String, f64> = areas
        .iter()
        .zip(&w_raw)
        .map(|(a, v)| (a.clone(), v / wsum))
        .collect();

    let index_for = |rounds_idx: &[usize]| -> (f64, BTreeMap<String, f64>) {
        let mut area_ln: BTreeMap<String, f64> = BTreeMap::new();
        for ar in &areas {
            let mut lns = Vec::new();
            for &ki in &by_area[ar] {
                let ds: Vec<f64> = rounds_idx
                    .iter()
                    .filter_map(|i| rows[ki].d_by_round.get(&i.to_string()).copied())
                    .collect();
                if !ds.is_empty() {
                    lns.push(median(&ds));
                }
            }
            if !lns.is_empty() {
                area_ln.insert(ar.clone(), lns.iter().sum::<f64>() / lns.len() as f64);
            }
        }
        let mut wsum_present: f64 = area_ln.keys().map(|a| w[a]).sum();
        if wsum_present == 0.0 {
            wsum_present = 1.0;
        }
        let total: f64 = area_ln.iter().map(|(a, v)| w[a] * v).sum();
        (total / wsum_present, area_ln)
    };

    let all_rounds: Vec<usize> = rows
        .iter()
        .flat_map(|r| r.d_by_round.keys().filter_map(|k| k.parse::<usize>().ok()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let n_rounds = all_rounds.last().map_or(0, |m| m + 1);
    let (pi_ln, area_ln) = index_for(&all_rounds);
    let mut boots: Vec<f64> = Vec::new();
    if all_rounds.len() > 1 {
        for _ in 0..BOOTSTRAP {
            let idx: Vec<usize> = all_rounds
                .iter()
                .map(|_| all_rounds[rng.below(all_rounds.len())])
                .collect();
            boots.push(index_for(&idx).0);
        }
    }
    boots.sort_by(|a, b| a.total_cmp(b));
    let ci: [f64; 2] = if boots.is_empty() {
        [pi_ln.exp(); 2]
    } else {
        [
            boots[(0.025 * boots.len() as f64) as usize].exp(),
            boots[(0.975 * boots.len() as f64) as usize - 1].exp(),
        ]
    };
    let valid = rows.iter().all(|r| r.valid);
    let missing_areas: Vec<String> = areas
        .iter()
        .filter(|a| !area_ln.contains_key(*a))
        .cloned()
        .collect();
    if !missing_areas.is_empty() {
        warnings.push(format!(
            "no usable round for any kernel of {}: left out of the index",
            py_list(&missing_areas)
        ));
    }
    let thin: Vec<String> = rows
        .iter()
        .filter(|r| r.valid && r.verdict == "insufficient")
        .map(|r| r.id.clone())
        .collect();
    if !thin.is_empty() {
        warnings.push(format!(
            "{} kernel(s) have fewer than {MIN_ROUNDS_FOR_VERDICT} usable rounds: {}",
            thin.len(),
            py_list(&thin[..thin.len().min(5)])
        ));
    }
    let soft: Vec<String> = rows
        .iter()
        .filter(|r| !r.corroborated)
        .map(|r| r.id.clone())
        .collect();
    if !soft.is_empty() {
        warnings.push(format!(
            "{} verdict(s) are not corroborated by the best-of ratio (marked †): {}",
            soft.len(),
            py_list(&soft[..soft.len().min(5)])
        ));
    }
    let n_cont = rows
        .iter()
        .filter(|r| !r.rounds_contended.is_empty())
        .count();
    if n_cont > 0 {
        warnings.push(format!(
            "{n_cont} kernel(s) had contended rounds{}",
            if p.exclude_contended {
                " (left out of their medians)"
            } else {
                " (kept: --keep-contended)"
            }
        ));
    }
    if !p.deterministic && n_rounds > 0 && n_rounds < MIN_ROUNDS_QUALIFIED {
        warnings.push(format!("{n_rounds} round(s): fewer than the {MIN_ROUNDS_QUALIFIED} AGENTS.md section 10 asks for"));
    }
    let no_aa: Vec<&KernelSummary> = rows.iter().filter(|r| r.aa_noise_ln.is_none()).collect();
    if !p.deterministic && !no_aa.is_empty() {
        warnings.push(format!(
            "{} kernel(s) have no A/A spread (fewer than {MIN_AA_FOR_NOISE} A/A pairs)",
            no_aa.len()
        ));
    }
    let qualified = valid
        && !rows.is_empty()
        && missing_areas.is_empty()
        && (p.deterministic
            || (no_aa.is_empty()
                && thin.is_empty()
                && n_rounds >= MIN_ROUNDS_QUALIFIED
                && rows.iter().all(|r| r.rounds_used >= MIN_ROUNDS_QUALIFIED)));
    let noises: Vec<f64> = rows.iter().filter_map(|r| r.aa_noise_ln).collect();
    let index_ok = valid && !area_ln.is_empty();
    Summary {
        valid,
        qualified,
        performance_index: index_ok.then(|| pi_ln.exp()),
        performance_index_ci95: index_ok.then_some(ci),
        areas: areas
            .iter()
            .map(|ar| {
                (
                    ar.clone(),
                    AreaSummary {
                        index: area_ln.get(ar).map(|v| v.exp()),
                        weight: w[ar],
                        kernels: by_area[ar].len(),
                    },
                )
            })
            .collect(),
        aa_noise_median_pct: (!noises.is_empty()).then(|| 100.0 * (median(&noises).exp() - 1.0)),
        kernels: rows,
        rounds: n_rounds,
        rounds_planned: p.rounds_planned.unwrap_or(n_rounds),
        exclude_contended: p.exclude_contended,
        warnings,
        extra: serde_json::Map::new(),
    }
}

fn fmt_pct(x: Option<f64>) -> String {
    x.map_or("n/a".into(), |v| {
        format!("±{:.1}%", 100.0 * (v.exp() - 1.0))
    })
}

/// The table `compare.md` holds.
pub fn markdown(s: &Summary, title: &str) -> String {
    let mut out: Vec<String> = vec![format!("# {title}"), String::new()];
    let mode = s
        .extra
        .get("mode")
        .and_then(Value::as_str)
        .unwrap_or("wall");
    let unit = s.extra.get("unit").and_then(Value::as_str).unwrap_or("ms");
    match (s.valid, s.performance_index, s.performance_index_ci95) {
        (true, Some(pi), Some([lo, hi])) => {
            let tag = if s.qualified {
                ""
            } else {
                " — **not qualified** (see warnings)"
            };
            out.push(format!("**Performance index PI = {pi:.3}x** (95% CI {lo:.3}–{hi:.3}, {} paired rounds){tag}", s.rounds));
        }
        (true, _, _) => out.push("**No usable round: no index.**".into()),
        _ => out.push(
            "**INVALID: at least one kernel's output fingerprint changed. No index.**".into(),
        ),
    }
    if let Some(n) = s.aa_noise_median_pct {
        out.push(format!("\nMedian A/A noise across kernels: ±{n:.1}%."));
    }
    for w in &s.warnings {
        out.push(format!("\n> warning: {w}"));
    }
    out.extend([
        "".into(),
        "| area | weight | kernels | area index |".into(),
        "|:--|--:|--:|--:|".into(),
    ]);
    for (ar, v) in &s.areas {
        let idx = v.index.map_or("n/a".into(), |x| format!("{x:.3}x"));
        out.push(format!(
            "| {ar} | {:.3} | {} | {idx} |",
            v.weight, v.kernels
        ));
    }
    let rich = mode == "wall";
    out.push(String::new());
    if rich {
        out.push(format!("| kernel | base {unit} | cand {unit} | speedup | 95% CI | best-of | A/A noise | rounds | verdict |"));
        out.push("|:--|--:|--:|--:|:--|--:|--:|--:|:--|".into());
    } else {
        out.push(format!(
            "| kernel | base {unit} | cand {unit} | speedup | 95% CI | A/A noise | verdict |"
        ));
        out.push("|:--|--:|--:|--:|:--|--:|:--|".into());
    }
    let scale = if unit == "ms" { 1e6 } else { 1.0 };
    for r in &s.kernels {
        let [lo, hi] = r.speedup_ci95;
        let cell = |x: Option<f64>| {
            x.map_or("n/a".to_string(), |v| {
                if unit == "Ir" {
                    format!("{v:.0}")
                } else {
                    format!("{:.3}", v / scale)
                }
            })
        };
        let dag = if r.corroborated { "" } else { " †" };
        if rich {
            let mut used = format!("{}/{}", r.rounds_used, r.rounds_total);
            if !r.rounds_contended.is_empty() {
                used.push_str(&format!(" ({} contended)", r.rounds_contended.len()));
            }
            let best = r
                .speedup_min
                .map_or("n/a".to_string(), |b| format!("{b:.3}x"));
            out.push(format!(
                "| `{}` | {} | {} | {:.3}x | {lo:.3}–{hi:.3} | {best} | {} | {used} | {}{dag} |",
                r.id,
                cell(r.base_median_ns),
                cell(r.cand_median_ns),
                r.speedup,
                fmt_pct(r.aa_noise_ln),
                r.verdict
            ));
        } else {
            out.push(format!(
                "| `{}` | {} | {} | {:.3}x | {lo:.3}–{hi:.3} | {} | {}{dag} |",
                r.id,
                cell(r.base_median_ns),
                cell(r.cand_median_ns),
                r.speedup,
                fmt_pct(r.aa_noise_ln),
                r.verdict
            ));
        }
    }
    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn kernel(id: &str, area: &str) -> KernelInfo {
        KernelInfo {
            id: id.into(),
            area: area.into(),
        }
    }

    /// Rows for kernels with the given (A, B, A2) medians in ns, one per round.
    fn per(items: &[(&str, &[f64], &[f64], &[f64])]) -> Per {
        let mut out = Per::new();
        for (id, a, b, a2) in items {
            let mut rows = ArmRows::default();
            for (arm, xs) in [(Arm::A, a), (Arm::B, b), (Arm::A2, a2)] {
                for (i, x) in xs.iter().enumerate() {
                    rows.arm_mut(arm).insert(
                        i,
                        json!({"fingerprint": "00000000000000aa", "median_ns": x}),
                    );
                }
            }
            out.insert(id.to_string(), rows);
        }
        out
    }

    fn run(items: &[(&str, &[f64], &[f64], &[f64])], weights: &[(&str, f64)]) -> Summary {
        let ks: Vec<KernelInfo> = items
            .iter()
            .map(|(id, ..)| kernel(id, id.split('/').next().unwrap()))
            .collect();
        let w = weights.iter().map(|(a, v)| (a.to_string(), *v)).collect();
        summarize(
            &ks,
            &per(items),
            &w,
            &mut SplitMix64::new(1),
            &Params::default(),
        )
    }

    #[test]
    fn a_difference_inside_the_aa_spread_is_neutral() {
        // candidate 3% faster, but the baseline disagrees with itself by ~8%
        let s = run(
            &[(
                "g/x",
                &[100.0; 5],
                &[97.0; 5],
                &[108.0, 93.0, 106.0, 94.0, 107.0],
            )],
            &[],
        );
        assert_eq!(s.kernels[0].verdict, "neutral");
    }

    #[test]
    fn the_two_percent_floor() {
        let a = [100.0; 5];
        assert_eq!(
            run(&[("g/x", &a, &[99.0; 5], &a)], &[]).kernels[0].verdict,
            "neutral"
        );
        assert_eq!(
            run(&[("g/x", &a, &[90.0; 5], &a)], &[]).kernels[0].verdict,
            "faster"
        );
        assert_eq!(
            run(&[("g/x", &a, &[110.0; 5], &a)], &[]).kernels[0].verdict,
            "slower"
        );
    }

    #[test]
    fn the_index_is_the_weighted_geometric_mean() {
        let items: [(&str, &[f64], &[f64], &[f64]); 2] = [
            ("g/x", &[100.0; 5], &[50.0; 5], &[100.0; 5]),
            ("h/y", &[100.0; 5], &[25.0; 5], &[100.0; 5]),
        ];
        let s = run(&items, &[("g", 1.0), ("h", 3.0)]);
        let want = ((1.0 * 2f64.ln() + 3.0 * 4f64.ln()) / 4.0).exp();
        assert!((s.performance_index.unwrap() - want).abs() < 1e-12);
        let s = run(&items, &[("g", 1.0), ("h", 0.0)]); // weight 0 never moves the index
        assert!((s.performance_index.unwrap() - 2.0).abs() < 1e-12);
        assert!(s.qualified, "{:?}", s.warnings);
    }

    #[test]
    fn unweighted_areas_and_few_rounds_are_warned_about() {
        let a = [100.0; 3];
        let s = run(&[("g/x", &a, &[50.0; 3], &a)], &[("other", 1.0)]);
        assert!(s.warnings.iter().any(|w| w.contains("without a weight")));
        assert!(s.warnings.iter().any(|w| w.contains("round(s)")));
        assert!(!s.qualified);
    }

    #[test]
    fn no_aa_means_no_verdict() {
        let s = run(&[("g/x", &[100.0; 5], &[50.0; 5], &[])], &[]);
        assert_eq!(s.kernels[0].verdict, "unqualified-faster");
        assert!(!s.qualified);
    }

    #[test]
    fn a_changed_fingerprint_refuses_the_index() {
        let mut p = per(&[("g/x", &[100.0; 5], &[50.0; 5], &[100.0; 5])]);
        p.get_mut("g/x")
            .unwrap()
            .arm_mut(Arm::B)
            .get_mut(&2)
            .unwrap()["fingerprint"] = json!("00000000000000bb");
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &Params::default(),
        );
        assert!(!s.valid);
        assert_eq!(s.kernels[0].verdict, "INVALID");
        assert!(s.performance_index.is_none() && s.performance_index_ci95.is_none());
    }

    #[test]
    fn exact_interval_sign_test_and_hl_are_reported() {
        let a = [100.0; 5];
        let k = &run(&[("g/x", &a, &[50.0; 5], &a)], &[]).kernels[0];
        assert!((k.ci_exact_coverage.unwrap() - 0.9375).abs() < 1e-12);
        assert!((k.sign_test_p.unwrap() - 2.0 / 32.0).abs() < 1e-12);
        assert!((k.speedup_hl.unwrap() - 2.0).abs() < 1e-12);
    }

    fn with_min(p: &mut Per, id: &str, a_min: f64, b_min: f64) {
        for (arm, m) in [(Arm::A, a_min), (Arm::B, b_min)] {
            for row in p.get_mut(id).unwrap().arm_mut(arm).values_mut() {
                row["min_ns"] = json!(m);
            }
        }
    }

    #[test]
    fn a_slow_median_with_equal_best_of_is_not_corroborated() {
        // the candidate arm is hit by interference in most rounds; its fastest samples are as fast as the baseline's
        let mut p = per(&[(
            "g/x",
            &[100.0; 5],
            &[140.0, 150.0, 125.0, 145.0, 135.0],
            &[100.0, 101.0, 99.0, 100.0, 100.0],
        )]);
        with_min(&mut p, "g/x", 98.0, 98.0);
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &Params::default(),
        );
        assert_eq!(s.kernels[0].verdict, "slower");
        assert!(!s.kernels[0].corroborated);
        assert!(s.warnings.iter().any(|w| w.contains("not corroborated")));
        assert!(markdown(&s, "t").contains("slower †"));
        // a real slowdown moves the best-of too
        let mut p = per(&[(
            "g/x",
            &[100.0; 5],
            &[140.0; 5],
            &[100.0, 101.0, 99.0, 100.0, 100.0],
        )]);
        with_min(&mut p, "g/x", 98.0, 138.0);
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &Params::default(),
        );
        assert!(s.kernels[0].corroborated);
    }

    fn contended_row(cpu_ratio: f64) -> Value {
        json!({"fingerprint": "00000000000000aa", "median_ns": 100.0, "median_cpu_ns": 100.0 * cpu_ratio})
    }

    #[test]
    fn contended_rounds_are_left_out_counted_and_optionally_kept() {
        let mut p = per(&[("g/x", &[100.0; 5], &[50.0; 5], &[100.0; 5])]);
        p.get_mut("g/x")
            .unwrap()
            .arm_mut(Arm::B)
            .insert(2, contended_row(0.5));
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &Params::default(),
        );
        let k = &s.kernels[0];
        assert_eq!((k.rounds_used, k.rounds_total), (4, 5));
        assert_eq!(k.rounds_contended, vec![2]);
        assert!(k.contended_why.keys().any(|w| w == "B@2"));
        assert!(!s.qualified); // four usable rounds are fewer than five
        let keep = Params {
            exclude_contended: false,
            ..Params::default()
        };
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &keep,
        );
        assert_eq!(s.kernels[0].rounds_used, 5);
        assert_eq!(s.kernels[0].rounds_contended, vec![2]); // still reported
    }

    #[test]
    fn a_kernel_with_too_few_quiet_rounds_is_insufficient() {
        let mut p = per(&[("g/x", &[100.0; 5], &[50.0; 5], &[100.0; 5])]);
        for r in 0..4 {
            p.get_mut("g/x")
                .unwrap()
                .arm_mut(Arm::B)
                .insert(r, contended_row(0.5));
        }
        let s = summarize(
            &[kernel("g/x", "g")],
            &p,
            &BTreeMap::new(),
            &mut SplitMix64::new(1),
            &Params::default(),
        );
        assert_eq!(s.kernels[0].rounds_used, 1);
        assert_eq!(s.kernels[0].verdict, "insufficient");
    }

    #[test]
    fn deterministic_counts_need_one_observation_and_the_floor() {
        let det = Params {
            deterministic: true,
            exclude_contended: false,
            ..Params::default()
        };
        let one = |a: f64, b: f64| {
            let mut rows = ArmRows::default();
            rows.arm_mut(Arm::A).insert(
                0,
                json!({"fingerprint": "00000000000000aa", "median_ns": a}),
            );
            rows.arm_mut(Arm::B).insert(
                0,
                json!({"fingerprint": "00000000000000aa", "median_ns": b}),
            );
            let mut p = Per::new();
            p.insert("g/x".into(), rows);
            summarize(
                &[kernel("g/x", "g")],
                &p,
                &BTreeMap::from([("g".to_string(), 1.0)]),
                &mut SplitMix64::new(0),
                &det,
            )
        };
        let s = one(1000.0, 500.0);
        assert_eq!(s.kernels[0].verdict, "faster");
        assert!(s.qualified, "{:?}", s.warnings);
        assert!(s.warnings.is_empty(), "{:?}", s.warnings);
        assert_eq!(one(1000.0, 1010.0).kernels[0].verdict, "neutral"); // 1% is build noise
        assert_eq!(one(1000.0, 1100.0).kernels[0].verdict, "slower");
    }

    #[test]
    fn row_problems_and_contention() {
        let good = json!({"id": "x/y", "fingerprint": "00000000000000aa", "median_ns": 1.0e7, "threads": 1});
        assert!(row_problems(&good, "x/y", Some(1)).is_empty());
        assert!(!row_problems(&good, "other", Some(1)).is_empty());
        assert!(!row_problems(
            &json!({"id": "x/y", "fingerprint": "xyz", "median_ns": 1.0}),
            "x/y",
            None
        )
        .is_empty());
        assert!(!row_problems(
            &json!({"id": "x/y", "fingerprint": "00000000000000aa", "median_ns": 0}),
            "x/y",
            None
        )
        .is_empty());
        let wrong_threads =
            json!({"id": "x/y", "fingerprint": "00000000000000aa", "median_ns": 1.0, "threads": 4});
        assert!(!row_problems(&wrong_threads, "x/y", Some(1)).is_empty()); // RAYON_NUM_THREADS silently ignored
        assert!(row_problems(
            &json!({"id": "x/y", "fingerprint": "00000000000000aa", "median_ns": 1.0}),
            "x/y",
            Some(1)
        )
        .is_empty());

        let row = |extra: Value| {
            let mut r = json!({"median_ns": 1.0e7, "median_cpu_ns": 9.9e6, "samples_ns": [1.0e7, 1.0e7, 1.0e7]});
            r.as_object_mut()
                .unwrap()
                .extend(extra.as_object().unwrap().clone());
            r
        };
        assert!(contention(&row(json!({})), 1, 0.9, None, 0.05).is_empty());
        assert!(!contention(&row(json!({"median_cpu_ns": 5.0e6})), 1, 0.9, None, 0.05).is_empty());
        // CPU time over wall time says nothing at many threads
        assert!(contention(&row(json!({"median_cpu_ns": 5.0e6})), 4, 0.9, None, 0.05).is_empty());
        // involuntary switches: off unless asked for
        let busy = row(json!({"invol_csw": 10000}));
        assert!(contention(&busy, 4, 0.9, None, 0.05).is_empty());
        assert!(!contention(&busy, 4, 0.9, Some(100.0), 0.05).is_empty());
        // steal needs two ticks and a share of the invocation
        let steal = |ticks: u32, inv: f64| {
            row(json!({"steal_ticks": ticks, "clk_tck": 100, "invocation_s": inv}))
        };
        assert!(!contention(&steal(5, 0.5), 1, 0.9, None, 0.05).is_empty()); // 50 ms of 500 ms
        assert!(contention(&steal(5, 5.0), 1, 0.9, None, 0.05).is_empty()); // 1%
        assert!(contention(&steal(1, 0.02), 1, 0.9, None, 0.05).is_empty()); // one tick is the clock's jitter
                                                                             // a row from an old harness has no accounting fields and cannot be classified
        assert!(contention(
            &json!({"median_ns": 10.0, "samples_ns": [10.0]}),
            1,
            0.9,
            Some(1.0),
            0.05
        )
        .is_empty());
    }

    #[test]
    fn markdown_renders_warnings_and_the_invalid_banner() {
        let s = Summary {
            valid: false,
            qualified: false,
            performance_index: None,
            performance_index_ci95: None,
            areas: BTreeMap::new(),
            aa_noise_median_pct: None,
            kernels: vec![],
            rounds: 0,
            rounds_planned: 0,
            exclude_contended: true,
            warnings: vec!["w1".into()],
            extra: serde_json::Map::new(),
        };
        let md = markdown(&s, "t");
        assert!(md.contains("INVALID") && md.contains("warning: w1"));
    }
}
