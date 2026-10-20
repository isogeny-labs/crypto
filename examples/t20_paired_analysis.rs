//! T20 analysis: paired old-vs-new producer comparison on the frozen n = 73,
//! K = 600 index-calculus row, with an A/A arm, matched rho draws and the
//! section 5a price band.  Reads the records a run left in `<runs_dir>`
//! and writes `analysis.json`, `tables.md` and `graph.svg` to `<out_dir>`.
//!
//! Usage: t20_paired_analysis <runs_dir> <out_dir>
//!
//! Computation only; it launches nothing and times nothing.  Every number it
//! prints is derived from the records on disk.

use serde_json::{json, Value};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const BOOTSTRAP: usize = 10_000;
const THREADS: usize = 3;
const SUBGROUP_ORDER: f64 = 86_020_738_150_056_119.0;
const KNOWN_ANSWER: u64 = 12_345_678_901_234_567;
const BAND: [f64; 3] = [0.0, 0.1, 1.0];

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 11
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let line = text
        .lines()
        .rfind(|l| l.trim_start().starts_with('{'))
        .unwrap_or_else(|| panic!("no json line in {path:?}"));
    serde_json::from_str(line).unwrap_or_else(|e| panic!("parse {path:?}: {e}"))
}

#[derive(Clone, Copy, Debug)]
struct Time {
    real: f64,
    user: f64,
    sys: f64,
}

fn read_time(path: &Path) -> Time {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut t = Time {
        real: 0.0,
        user: 0.0,
        sys: 0.0,
    };
    for tok in text.split_whitespace() {
        if let Some((k, v)) = tok.split_once('=') {
            let v: f64 = v.parse().unwrap_or(0.0);
            match k {
                "real" => t.real = v,
                "user" => t.user = v,
                "sys" => t.sys = v,
                _ => {}
            }
        }
    }
    t
}

fn pct(sorted: &[f64], q: f64) -> f64 {
    let i = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[i]
}

fn ci(mut v: Vec<f64>) -> (f64, f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (pct(&v, 0.025), pct(&v, 0.5), pct(&v, 0.975))
}

struct IcRun {
    tag: String,
    time: Time,
    summary: Value,
    row: Value,
    /// per column (probes, attempts, ns), indexed by column
    columns: Vec<(u64, u64, f64)>,
}

fn load_ic(dir: &Path, tag: &str) -> IcRun {
    let summary = read_json(&dir.join(format!("ic_{tag}.summary.json")));
    let row = read_json(&dir.join(format!("ic_{tag}.row.jsonl")));
    let time = read_time(&dir.join(format!("ic_{tag}.time")));
    let trace = summary["rank_column_trace"]
        .as_array()
        .expect("rank_column_trace");
    let mut columns = vec![(0u64, 0u64, 0f64); trace.len()];
    for t in trace {
        let c = t[0].as_u64().unwrap() as usize;
        columns[c] = (
            t[1].as_u64().unwrap(),
            t[2].as_u64().unwrap(),
            t[3].as_u64().unwrap() as f64,
        );
    }
    IcRun {
        tag: tag.into(),
        time,
        summary,
        row,
        columns,
    }
}

fn phase(run: &IcRun, key: &str) -> f64 {
    run.summary["timing_ms"][key].as_f64().unwrap_or(0.0) / 1000.0
}

/// Wall seconds of everything that is not the rank stage, from the run.
fn other_wall(run: &IcRun) -> f64 {
    run.time.real - phase(run, "rank_stage")
}

/// Point estimate of the parallel rank wall from per-column traces, using the
/// same thread strata as the paired bootstrap.
fn rank_wall_from_columns(columns: &[(u64, u64, f64)]) -> f64 {
    let mut sums = [0.0; THREADS];
    for (column, (_, _, ns)) in columns.iter().enumerate() {
        sums[column % THREADS] += ns;
    }
    sums.into_iter().fold(0.0, f64::max) / 1e9
}

/// Stratified (by column mod THREADS) paired bootstrap of the end-to-end wall
/// ratio base/cand: the same resampled columns in both arms; rank wall is the
/// maximum over thread strata of the summed per-column nanoseconds; the
/// non-rank phases are the measured ones of each arm.
struct Ratio {
    point_wall: f64,
    wall_ci: (f64, f64, f64),
    point_rank_cpu: f64,
    rank_cpu_ci: (f64, f64, f64),
    point_rank_wall: f64,
    rank_wall_ci: (f64, f64, f64),
}

fn paired(
    base: &[(u64, u64, f64)],
    cand: &[(u64, u64, f64)],
    base_other: f64,
    cand_other: f64,
    base_wall: f64,
    cand_wall: f64,
    base_rank: f64,
    cand_rank: f64,
    seed: u64,
) -> Ratio {
    let n = base.len();
    assert_eq!(n, cand.len());
    let strata: Vec<Vec<usize>> = (0..THREADS)
        .map(|s| (s..n).step_by(THREADS).collect())
        .collect();
    let mut rng = Lcg(seed);
    let (mut wall, mut cpu, mut rankw) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..BOOTSTRAP {
        let (mut bmax, mut cmax, mut bsum, mut csum) = (0f64, 0f64, 0f64, 0f64);
        for st in &strata {
            let (mut b, mut c) = (0f64, 0f64);
            for _ in 0..st.len() {
                let j = st[rng.below(st.len())];
                b += base[j].2;
                c += cand[j].2;
            }
            bmax = bmax.max(b);
            cmax = cmax.max(c);
            bsum += b;
            csum += c;
        }
        wall.push((base_other + bmax / 1e9) / (cand_other + cmax / 1e9));
        cpu.push(bsum / csum);
        rankw.push(bmax / cmax);
    }
    Ratio {
        point_wall: base_wall / cand_wall,
        wall_ci: ci(wall),
        point_rank_cpu: base.iter().map(|c| c.2).sum::<f64>()
            / cand.iter().map(|c| c.2).sum::<f64>(),
        rank_cpu_ci: ci(cpu),
        point_rank_wall: base_rank / cand_rank,
        rank_wall_ci: ci(rankw),
    }
}

fn ci_json(c: (f64, f64, f64)) -> Value {
    json!({"lo": c.0, "median": c.1, "hi": c.2})
}

fn ratio_json(r: &Ratio) -> Value {
    json!({
        "end_to_end_wall": {"point": r.point_wall, "bootstrap_95": ci_json(r.wall_ci)},
        "rank_stage_wall": {"point": r.point_rank_wall, "bootstrap_95": ci_json(r.rank_wall_ci)},
        "rank_stage_cpu_sum_over_columns": {"point": r.point_rank_cpu, "bootstrap_95": ci_json(r.rank_cpu_ci)},
    })
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn boot_mean(v: &[f64], seed: u64) -> (f64, f64, f64) {
    let mut rng = Lcg(seed);
    let out: Vec<f64> = (0..BOOTSTRAP)
        .map(|_| {
            mean(
                &(0..v.len())
                    .map(|_| v[rng.below(v.len())])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    ci(out)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let runs = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir_all(out).unwrap();

    let n1 = load_ic(runs, "N1");
    let o1 = load_ic(runs, "O1");
    let n2 = load_ic(runs, "N2");
    let arms = [&n1, &o1, &n2];

    // ---- identity and verification checks --------------------------------
    let mut checks = Vec::new();
    for a in &arms {
        let solved = a.summary["targets_solved"].as_u64() == Some(1);
        let rel = a.summary["rank_relations"].as_u64() == Some(600);
        let rank = a.summary["rank"].as_u64() == Some(600);
        let verified = a.row["group_verified"].as_bool() == Some(true);
        let known = a.row["recovered_scalar"].as_u64() == Some(KNOWN_ANSWER);
        let exit0 = a.row["exit_code"].as_u64() == Some(0);
        checks.push(
            json!({"arm": a.tag, "targets_solved_1": solved, "rank_relations_600": rel,
            "rank_600": rank, "group_verified": verified, "recovered_equals_sidecar_answer": known,
            "exit_code_0": exit0}),
        );
    }
    let probes_equal = |x: &IcRun, y: &IcRun| {
        x.columns
            .iter()
            .zip(&y.columns)
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
    };
    let target_probes_equal =
        n1.row["probes"] == o1.row["probes"] && n1.row["probes"] == n2.row["probes"];
    let identical = probes_equal(&n1, &o1)
        && probes_equal(&n1, &n2)
        && n1.row["point_indices"] == o1.row["point_indices"]
        && n1.row["x_codes"] == o1.row["x_codes"]
        && n1.row["recovered_scalar"] == o1.row["recovered_scalar"]
        && target_probes_equal;
    let flag = |k: &str, a: &IcRun| a.summary[k].clone();
    let switches = json!({
        "N1": {"lazy": flag("lazy_canonical_enabled", &n1), "prefetch": flag("probe_prefetch_enabled", &n1)},
        "O1": {"lazy": flag("lazy_canonical_enabled", &o1), "prefetch": flag("probe_prefetch_enabled", &o1)},
        "N2": {"lazy": flag("lazy_canonical_enabled", &n2), "prefetch": flag("probe_prefetch_enabled", &n2)},
    });

    // ---- paired comparisons ----------------------------------------------
    let n_mean_cols: Vec<(u64, u64, f64)> = n1
        .columns
        .iter()
        .zip(&n2.columns)
        .map(|(a, b)| (a.0, a.1, (a.2 + b.2) / 2.0))
        .collect();
    let n_other = (other_wall(&n1) + other_wall(&n2)) / 2.0;
    let n_wall = (n1.time.real + n2.time.real) / 2.0;
    let n_rank = (phase(&n1, "rank_stage") + phase(&n2, "rank_stage")) / 2.0;
    let ab = paired(
        &o1.columns,
        &n_mean_cols,
        other_wall(&o1),
        n_other,
        o1.time.real,
        n_wall,
        phase(&o1, "rank_stage"),
        n_rank,
        11,
    );
    // pairing against each new run separately (the bracket)
    let ab1 = paired(
        &o1.columns,
        &n1.columns,
        other_wall(&o1),
        other_wall(&n1),
        o1.time.real,
        n1.time.real,
        phase(&o1, "rank_stage"),
        phase(&n1, "rank_stage"),
        12,
    );
    let ab2 = paired(
        &o1.columns,
        &n2.columns,
        other_wall(&o1),
        other_wall(&n2),
        o1.time.real,
        n2.time.real,
        phase(&o1, "rank_stage"),
        phase(&n2, "rank_stage"),
        13,
    );
    // A/A: the two new runs against each other, same estimator
    let aa = paired(
        &n1.columns,
        &n2.columns,
        other_wall(&n1),
        other_wall(&n2),
        n1.time.real,
        n2.time.real,
        phase(&n1, "rank_stage"),
        phase(&n2, "rank_stage"),
        14,
    );
    // Analyzer positive control: scale new-run columns by 1.10 and re-detect.
    let scaled: Vec<(u64, u64, f64)> = n1.columns.iter().map(|c| (c.0, c.1, c.2 * 1.10)).collect();
    let pc_scaled_wall = other_wall(&n1) + rank_wall_from_columns(&scaled);
    let pc_control_wall = other_wall(&n2) + rank_wall_from_columns(&n2.columns);
    let pc = paired(
        &scaled,
        &n2.columns,
        other_wall(&n1),
        other_wall(&n2),
        pc_scaled_wall,
        pc_control_wall,
        phase(&n1, "rank_stage") * 1.10,
        phase(&n2, "rank_stage"),
        15,
    );

    // ---- phases ------------------------------------------------------------
    let phase_keys = [
        "base_load",
        "normal_basis_and_selftest",
        "index_build",
        "rank_stage",
        "linear_algebra",
        "targets_total",
        "process_total",
    ];
    let mut phases = serde_json::Map::new();
    for a in &arms {
        let mut m = serde_json::Map::new();
        for k in phase_keys {
            m.insert(k.into(), json!(phase(a, k)));
        }
        m.insert("process_wall_bash_real_s".into(), json!(a.time.real));
        m.insert("process_cpu_user_s".into(), json!(a.time.user));
        m.insert("process_cpu_sys_s".into(), json!(a.time.sys));
        m.insert(
            "pre_main_and_teardown_s".into(),
            json!(a.time.real - phase(a, "process_total")),
        );
        m.insert("peak_rss_bytes".into(), a.summary["peak_rss_bytes"].clone());
        phases.insert(a.tag.clone(), Value::Object(m));
    }

    // ---- rho draws ---------------------------------------------------------
    let mut rho_wall = Vec::new();
    let mut rho_cpu = Vec::new();
    let mut rho_steps = Vec::new();
    let mut rho_verified = 0;
    let mut rho_counters = [0f64; 7];
    let counter_keys = [
        "group_additions",
        "canonicalizations",
        "frobenius_maps",
        "partition_hashes",
        "table_inserts",
        "table_queries",
        "negations_examined",
    ];
    let mut rho_n = 0;
    for i in 1..=99 {
        let jp = runs.join(format!("rho_{i:02}.jsonl"));
        if !jp.exists() {
            break;
        }
        let d = read_json(&jp);
        let t = read_time(&runs.join(format!("rho_{i:02}.time")));
        rho_wall.push(t.real);
        rho_cpu.push(t.user + t.sys);
        rho_steps.push(d["walk_steps"].as_f64().unwrap());
        if d["verified"].as_bool() == Some(true) {
            rho_verified += 1;
        }
        for (k, key) in counter_keys.iter().enumerate() {
            rho_counters[k] += d["charges"][*key].as_f64().unwrap_or(0.0);
        }
        rho_n += 1;
    }
    let rho_ok = rho_n > 0;
    let mut rho_json = json!({"draws": rho_n, "verified": rho_verified});
    let sqrt_r = SUBGROUP_ORDER.sqrt();
    let mut table_md = String::new();
    let ic_wall_new = n_wall;
    let ic_cpu_new = (n1.time.user + n1.time.sys + n2.time.user + n2.time.sys) / 2.0;
    let ic_cpu_old = o1.time.user + o1.time.sys;
    let mut ic_over_rho = json!(null);
    if rho_ok {
        let mw = mean(&rho_wall);
        let mc = mean(&rho_cpu);
        let wci = boot_mean(&rho_wall, 21);
        let cci = boot_mean(&rho_cpu, 22);
        let sci = boot_mean(&rho_steps, 23);
        let mut sorted = rho_wall.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        rho_json = json!({
            "draws": rho_n, "verified": rho_verified,
            "wall_s": {"mean": mw, "mean_bootstrap_95": ci_json(wci), "median": pct(&sorted, 0.5), "min": sorted[0], "max": sorted[sorted.len()-1]},
            "cpu_s": {"mean": mc, "mean_bootstrap_95": ci_json(cci)},
            "walk_steps": {"mean": mean(&rho_steps), "mean_bootstrap_95": ci_json(sci)},
            "ledger_reference_total_ms": 390184.49379199994,
        });
        // IC/rho: IC value fixed per arm, interval from the rho draws.
        let ratio_ci = |ic: f64, v: &[f64], seed: u64| {
            let mut rng = Lcg(seed);
            let r: Vec<f64> = (0..BOOTSTRAP)
                .map(|_| {
                    ic / mean(
                        &(0..v.len())
                            .map(|_| v[rng.below(v.len())])
                            .collect::<Vec<_>>(),
                    )
                })
                .collect();
            ci(r)
        };
        ic_over_rho = json!({
            "new_wall_over_rho_wall": {"point": ic_wall_new / mw, "bootstrap_95_over_rho_draws": ci_json(ratio_ci(ic_wall_new, &rho_wall, 31))},
            "old_wall_over_rho_wall": {"point": o1.time.real / mw, "bootstrap_95_over_rho_draws": ci_json(ratio_ci(o1.time.real, &rho_wall, 32))},
            "new_cpu_over_rho_cpu": {"point": ic_cpu_new / mc, "bootstrap_95_over_rho_draws": ci_json(ratio_ci(ic_cpu_new, &rho_cpu, 33))},
            "old_cpu_over_rho_cpu": {"point": ic_cpu_old / mc, "bootstrap_95_over_rho_draws": ci_json(ratio_ci(ic_cpu_old, &rho_cpu, 34))},
            "note": "IC used 3 pinned cores (wall) and rho 1 core; the cpu_s ratio is the matched-resource one. IC run-to-run noise is the A/A spread, not in this interval.",
        });
    }

    // ---- section 5a band --------------------------------------------------
    // IC counters: probes (rank + target). Each probe = one S3 root solve,
    // one canonical-key computation and one Bloom query (derived, not emitted
    // per kind). Group-addition counter is NOT emitted by the example: the
    // IC side is a lower bound at every weight.
    let rank_probes: f64 = n1.summary["rank_probes_mean"].as_f64().unwrap() * 600.0;
    let target_probes = n1.row["probes"].as_f64().unwrap_or(0.0);
    let ic_probes = rank_probes + target_probes;
    let ic_native = 3.0 * ic_probes;
    let mut band = Vec::new();
    let rho_mean = |k: usize| {
        if rho_ok {
            rho_counters[k] / rho_n as f64
        } else {
            f64::NAN
        }
    };
    let rho_priced = rho_mean(0);
    let rho_native: f64 = (1..7).map(rho_mean).sum();
    for w in BAND {
        let ic_s = (0.0 + w * ic_native) / sqrt_r;
        let rho_s = (rho_priced + w * rho_native) / sqrt_r;
        band.push(
            json!({"weight_gae_per_native_op": w, "ic_S_lower_bound": ic_s, "rho_S": rho_s,
            "ic_over_rho": ic_s / rho_s, "old_over_new_ic": 1.0}),
        );
    }

    // ---- scan positive control --------------------------------------------
    let mut scan = Vec::new();
    for arm in ["old", "new"] {
        let mut v = Vec::new();
        for i in 1..=9 {
            let p = runs.join(format!("scan_{arm}_{i}.txt"));
            if let Ok(t) = fs::read_to_string(&p) {
                for l in t.lines() {
                    if l.starts_with("PROF") {
                        if let Some(x) = l.split("ns_per_probe=").nth(1) {
                            v.push(x.trim().parse::<f64>().unwrap());
                        }
                    }
                }
            }
        }
        scan.push((arm, v));
    }
    let scan_json = json!({"old_ns_per_probe": scan[0].1, "new_ns_per_probe": scan[1].1,
        "ratio_new_over_old_of_means": if scan[0].1.is_empty() || scan[1].1.is_empty() { Value::Null } else { json!(mean(&scan[1].1) / mean(&scan[0].1)) }});

    // ---- kill condition -------------------------------------------------
    let kill = ab.point_wall < 1.1;
    let result = json!({
        "protocol_kill_condition": "end-to-end wall ratio old/new < 1.1 => component gains do not reach the row",
        "kill_triggered": kill,
        "checks": checks,
        "outputs_identical_across_arms": identical,
        "switches_as_recorded": switches,
        "phases_s": Value::Object(phases),
        "old_over_new": {
            "vs_mean_of_N1_N2": ratio_json(&ab),
            "vs_N1": ratio_json(&ab1),
            "vs_N2": ratio_json(&ab2),
        },
        "A_A_N1_over_N2": ratio_json(&aa),
        "analyzer_positive_control_scale_1.10": ratio_json(&pc),
        "scan_positive_control": scan_json,
        "rho": rho_json,
        "ic_over_rho": ic_over_rho,
        "counted_units": {
            "ic_rank_probes": rank_probes, "ic_target_probes": target_probes,
            "ic_native_ops_derived_3_per_probe": ic_native,
            "ic_group_additions": "not emitted by the example (implementation gap); IC S is a lower bound",
            "rho_mean_counters": counter_keys.iter().enumerate().map(|(k, key)| (key.to_string(), json!(rho_mean(k)))).collect::<serde_json::Map<_, _>>(),
            "sqrt_subgroup_order": sqrt_r,
            "band": band,
        },
    });
    fs::write(
        out.join("analysis.json"),
        serde_json::to_string_pretty(&result).unwrap(),
    )
    .unwrap();

    // ---- tables ------------------------------------------------------------
    let _ = writeln!(table_md, "| arm | wall s (bash real) | cpu s (user+sys) | base load | basis | index build | rank stage | LA | target | pre/post-main |");
    let _ = writeln!(table_md, "|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    for a in &arms {
        let _ = writeln!(
            table_md,
            "| {} | {:.1} | {:.1} | {:.2} | {:.2} | {:.1} | {:.1} | {:.3} | {:.3} | {:.1} |",
            a.tag,
            a.time.real,
            a.time.user + a.time.sys,
            phase(a, "base_load"),
            phase(a, "normal_basis_and_selftest"),
            phase(a, "index_build"),
            phase(a, "rank_stage"),
            phase(a, "linear_algebra"),
            phase(a, "targets_total"),
            a.time.real - phase(a, "process_total")
        );
    }
    let row = |name: &str, r: &Ratio| {
        format!(
            "| {name} | {:.3} ({:.3}–{:.3}) | {:.3} ({:.3}–{:.3}) | {:.3} ({:.3}–{:.3}) |\n",
            r.point_wall,
            r.wall_ci.0,
            r.wall_ci.2,
            r.point_rank_wall,
            r.rank_wall_ci.0,
            r.rank_wall_ci.2,
            r.point_rank_cpu,
            r.rank_cpu_ci.0,
            r.rank_cpu_ci.2
        )
    };
    let _ = writeln!(table_md, "\n| comparison | end-to-end wall | rank-stage wall | rank-stage cpu (column sum) |\n|:--|--:|--:|--:|");
    table_md.push_str(&row("O1 / mean(N1,N2)", &ab));
    table_md.push_str(&row("O1 / N1", &ab1));
    table_md.push_str(&row("O1 / N2", &ab2));
    table_md.push_str(&row("A/A N1 / N2", &aa));
    table_md.push_str(&row("analyzer positive control (N1 x1.10 / N2)", &pc));
    fs::write(out.join("tables.md"), &table_md).unwrap();

    // ---- graph (SVG, recorded values only) ---------------------------------
    fs::write(
        out.join("graph.svg"),
        svg(&arms, &rho_wall, &ab, &aa, ic_wall_new),
    )
    .unwrap();
    println!("{}", table_md);
    println!("kill_triggered={kill} identical={identical}");
}

fn svg(arms: &[&IcRun; 3], rho_wall: &[f64], ab: &Ratio, aa: &Ratio, ic_wall_new: f64) -> String {
    let (w, h) = (980.0, 560.0);
    let mut s = String::new();
    let _ = writeln!(
        s,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" font-family="Helvetica,Arial,sans-serif" font-size="12"><rect width="100%" height="100%" fill="white"/>"##
    );
    let _ = writeln!(
        s,
        r##"<text x="20" y="26" font-size="16" font-weight="bold">T20: n=73 K=600 cold end-to-end wall by phase (3 pinned cores), old vs landed producer</text>"##
    );
    let colors = [
        ("pre/post-main + base load + basis", "#9aa5b1"),
        ("index build", "#e8a33d"),
        ("rank stage", "#3b7dd8"),
        ("LA + target", "#2a9d6f"),
    ];
    let maxv = arms.iter().map(|a| a.time.real).fold(0.0, f64::max);
    let x0 = 150.0;
    let scale = 780.0 / maxv;
    for (i, a) in arms.iter().enumerate() {
        let y = 60.0 + i as f64 * 46.0;
        let other = a.time.real
            - phase(a, "index_build")
            - phase(a, "rank_stage")
            - phase(a, "linear_algebra")
            - phase(a, "targets_total");
        let parts = [
            other,
            phase(a, "index_build"),
            phase(a, "rank_stage"),
            phase(a, "linear_algebra") + phase(a, "targets_total"),
        ];
        let mut x = x0;
        let label = match a.tag.as_str() {
            "O1" => "O1 old producer",
            "N1" => "N1 landed producer",
            _ => "N2 landed (A/A)",
        };
        let _ = writeln!(s, r##"<text x="20" y="{}" >{label}</text>"##, y + 20.0);
        for (k, p) in parts.iter().enumerate() {
            let wd = (p * scale).max(0.0);
            let _ = writeln!(
                s,
                r##"<rect x="{x}" y="{y}" width="{wd}" height="28" fill="{}"/>"##,
                colors[k].1
            );
            x += wd;
        }
        let _ = writeln!(
            s,
            r##"<text x="{}" y="{}">{:.0} s</text>"##,
            x + 6.0,
            y + 19.0,
            a.time.real
        );
    }
    for (k, (name, c)) in colors.iter().enumerate() {
        let _ = writeln!(
            s,
            r##"<rect x="{}" y="205" width="12" height="12" fill="{c}"/><text x="{}" y="216">{name}</text>"##,
            20.0 + k as f64 * 230.0,
            38.0 + k as f64 * 230.0
        );
    }
    let _ = writeln!(
        s,
        r##"<text x="20" y="250" font-weight="bold">Paired ratios (95% bootstrap over columns, stratified by thread)</text>"##
    );
    let items = [
        (
            "O1 / mean(N1,N2) end-to-end wall",
            ab.point_wall,
            ab.wall_ci,
        ),
        (
            "O1 / mean(N1,N2) rank-stage wall",
            ab.point_rank_wall,
            ab.rank_wall_ci,
        ),
        ("A/A N1 / N2 end-to-end wall", aa.point_wall, aa.wall_ci),
    ];
    let (lo, hi) = (0.8, 2.6);
    let xr = |v: f64| 330.0 + (v - lo) / (hi - lo) * 560.0;
    for (i, (name, pt, c)) in items.iter().enumerate() {
        let y = 280.0 + i as f64 * 34.0;
        let _ = writeln!(
            s,
            r##"<text x="20" y="{}">{name}</text><line x1="{}" y1="{y}" x2="{}" y2="{y}" stroke="#333" stroke-width="2"/><circle cx="{}" cy="{y}" r="5" fill="#d62728"/><text x="{}" y="{}">{:.3} [{:.3}, {:.3}]</text>"##,
            y + 4.0,
            xr(c.0),
            xr(c.2),
            xr(*pt),
            xr(hi) + 6.0 - 40.0,
            y - 8.0,
            pt,
            c.0,
            c.2
        );
    }
    let _ = writeln!(
        s,
        r##"<line x1="{0}" y1="268" x2="{0}" y2="380" stroke="#888" stroke-dasharray="4"/><text x="{0}" y="392" text-anchor="middle">1.0 (no change)</text><line x1="{1}" y1="268" x2="{1}" y2="380" stroke="#c33" stroke-dasharray="4"/><text x="{1}" y="392" text-anchor="middle">1.1 kill</text>"##,
        xr(1.0),
        xr(1.1)
    );
    if !rho_wall.is_empty() {
        let _ = writeln!(
            s,
            r##"<text x="20" y="425" font-weight="bold">Matched rho draws (1 core, wall s) vs IC new wall ({:.0} s, 3 cores)</text>"##,
            ic_wall_new
        );
        let rmax = rho_wall.iter().cloned().fold(ic_wall_new, f64::max) * 1.05;
        let xs = |v: f64| 40.0 + v / rmax * 880.0;
        let _ = writeln!(
            s,
            r##"<line x1="40" y1="480" x2="920" y2="480" stroke="#333"/>"##
        );
        for v in rho_wall {
            let _ = writeln!(
                s,
                r##"<circle cx="{}" cy="480" r="4" fill="#3b7dd8" fill-opacity="0.6"/>"##,
                xs(*v)
            );
        }
        let _ = writeln!(
            s,
            r##"<line x1="{0}" y1="455" x2="{0}" y2="500" stroke="#d62728" stroke-width="2"/><text x="{0}" y="515" text-anchor="middle">IC new</text>"##,
            xs(ic_wall_new)
        );
        let _ = writeln!(
            s,
            r##"<text x="40" y="530">0 s</text><text x="920" y="530" text-anchor="end">{:.0} s</text>"##,
            rmax
        );
    }
    let _ = writeln!(
        s,
        r##"<text x="20" y="552" font-size="10" fill="#555">Recorded values only; host: unisolated-class 4 vCPU KVM guest, IC pinned to cores 1-3. See analysis.json for the sources.</text></svg>"##
    );
    s
}
