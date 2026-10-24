//! Native analysis of N83 panels whose raw process records are in the
//! repository: PR #2282's padd panel and the h=3 panel
//! (`research/n83_mitm3_ic_20261024/PROTOCOL.md`).
//!
//! `validate-2282 <raw_dir> <answers.jsonl> <claimed analysis.json> <out.json>`
//!   recomputes PR #2282's point estimates from its raw records (usage,
//!   summaries, target rows, rho receipts), checks every scalar against the
//!   sealed answers and every arm's relation and rank identity, and writes a
//!   validation-gate validator report comparing them with the claimed values.
//!
//! `panel <panel_dir> <answers.jsonl> <first_point> <out.json>`
//!   checks and analyses the h=3 panel over points `first_point..=20`:
//!   scalars, identities, paired differences with a seeded bootstrap,
//!   IC/rho ratios and the §5a operation counts.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn jsonl(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn sha256_file(path: &Path) -> String {
    hex::encode(crypto_lib::hash::sha256(&fs::read(path).unwrap()))
}

fn f(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::String(s) => s.parse().unwrap(),
        _ => panic!("not a number: {v}"),
    }
}

fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// SplitMix64, the bootstrap's seeded generator.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

const BOOT: usize = 10_000;

fn quantiles(mut v: Vec<f64>) -> [f64; 2] {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    [v[(0.025 * v.len() as f64) as usize], v[(0.975 * v.len() as f64) as usize]]
}

/// Bootstrap of the mean of paired differences, resampling points.
fn boot_mean(d: &[f64], rng: &mut Rng) -> [f64; 2] {
    let k = d.len();
    quantiles((0..BOOT).map(|_| (0..k).map(|_| d[rng.below(k)]).sum::<f64>() / k as f64).collect())
}

/// Bootstrap of a ratio of means over paired points.
fn boot_ratio(num: &[f64], den: &[f64], rng: &mut Rng) -> [f64; 2] {
    let k = num.len();
    quantiles(
        (0..BOOT)
            .map(|_| {
                let idx: Vec<usize> = (0..k).map(|_| rng.below(k)).collect();
                idx.iter().map(|&i| num[i]).sum::<f64>() / idx.iter().map(|&i| den[i]).sum::<f64>()
            })
            .collect(),
    )
}

/// (mean IC cold) / (mean rho online), resampling points and, within each
/// drawn point, its rho draws (PR #2282's interval construction).
fn boot_ic_over_rho(ic: &[f64], rho: &[Vec<f64>], rng: &mut Rng) -> [f64; 2] {
    let k = ic.len();
    quantiles(
        (0..BOOT)
            .map(|_| {
                let idx: Vec<usize> = (0..k).map(|_| rng.below(k)).collect();
                let num = idx.iter().map(|&i| ic[i]).sum::<f64>() / k as f64;
                let mut s = 0.0;
                let mut c = 0usize;
                for &i in &idx {
                    for _ in 0..rho[i].len() {
                        s += rho[i][rng.below(rho[i].len())];
                        c += 1;
                    }
                }
                num / (s / c as f64)
            })
            .collect(),
    )
}

struct IcRun {
    cold_s: f64,
    rank_s: f64,
    setup_s: f64,
    scalar: String,
    identity: Value,
    rank_probes: f64,
    target_probes: f64,
    regular_states: f64,
    rank_solve: String,
}

fn ic_run(dir: &Path) -> IcRun {
    let usage = read(&dir.join("usage.json"));
    assert_eq!(usage["status"], "PASS_PRODUCER", "{}", dir.display());
    let summary = jsonl(&dir.join("stdout.jsonl"))
        .into_iter()
        .find(|r| r["kind"] == "compact_orbit_dlp_summary")
        .expect("summary row");
    let target = jsonl(&dir.join("targets_result.jsonl"))
        .into_iter()
        .find(|r| r["kind"] == "compact_orbit_dlp_target")
        .expect("target row");
    assert_eq!(target["group_verified"], true);
    let base = if dir.join("base_dump.jsonl").exists() {
        sha256_file(&dir.join("base_dump.jsonl"))
    } else {
        fs::read_to_string(dir.join("base_dump.sha256")).map(|s| s.trim().to_string()).unwrap_or_default()
    };
    let t = &summary["timing_ms"];
    IcRun {
        cold_s: f(&usage["wall_seconds"]),
        rank_s: f(&t["rank_stage"]) / 1000.0,
        setup_s: f(&t["setup_total"]) / 1000.0,
        scalar: target["recovered_scalar"].to_string().trim_matches('"').to_string(),
        identity: json!({
            "point_indices": target["point_indices"], "probes": target["probes"],
            "rank_attempts": summary["rank_attempts"], "rank_relations": summary["rank_relations"],
            "rank_probes_mean": summary["rank_probes_mean"], "rank": summary["rank"],
            "base_sha256": base,
        }),
        rank_probes: f(&summary["rank_probes_mean"]) * f(&summary["rank_relations"]),
        target_probes: f(&target["probes"]),
        regular_states: f(&summary["regular_states"]),
        rank_solve: summary["rank_solve"].as_str().unwrap_or("s3").to_string(),
    }
}

struct RhoRun {
    online_s: f64,
    wall_s: f64,
    scalar: String,
    charges: Value,
}

fn rho_run(dir: &Path) -> RhoRun {
    let usage = read(&dir.join("usage.json"));
    assert_eq!(usage["status"], "PASS_PRODUCER", "{}", dir.display());
    let r = read(&dir.join("receipt.json"));
    assert_eq!(r["status"], "PASS_VERIFIED_PUBLIC_TARGET");
    assert_eq!(r["generic_scalar_replay_verified"], true);
    fn find(v: &Value, key: &str) -> Option<Value> {
        match v {
            Value::Object(m) => m.get(key).cloned().or_else(|| m.values().find_map(|x| find(x, key))),
            _ => None,
        }
    }
    RhoRun {
        online_s: f(&r["online_ms"]) / 1000.0,
        wall_s: f(&usage["wall_seconds"]),
        scalar: r["recovered_scalar"].to_string().trim_matches('"').to_string(),
        charges: find(&r, "charges_setup_and_solve").unwrap_or(Value::Null),
    }
}

fn answers(path: &Path) -> Vec<String> {
    jsonl(path).iter().map(|a| a["scalar"].as_str().unwrap().to_string()).collect()
}

fn headline(name: &str, claimed: f64, recomputed: f64) -> Value {
    json!({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": 1e-9})
}

fn validate_2282(raw: &Path, answers_path: &Path, claimed_path: &Path, out: &Path) {
    let ans = answers(answers_path);
    let claimed = read(claimed_path);
    let runs = raw.join("runs");
    let (mut cur, mut padd, mut aa_a, mut aa_b) = (vec![], vec![], vec![], vec![]);
    let (mut rank_cur, mut rank_padd) = (vec![], vec![]);
    let mut rho_online: Vec<Vec<f64>> = vec![];
    let mut scalar_checks = 0usize;
    let mut identity_checks = 0usize;
    for i in 1..=20 {
        let arm = |a: &str| ic_run(&runs.join(format!("p{i}_{a}")));
        let (a, b, p) = (arm("cur_a"), arm("cur_b"), arm("padd"));
        for r in [&a, &b, &p] {
            assert_eq!(r.scalar, ans[i - 1], "point {i} scalar");
            scalar_checks += 1;
        }
        assert_eq!(a.identity, b.identity, "point {i} cur identity");
        assert_eq!(a.identity, p.identity, "point {i} padd identity");
        assert_eq!(p.rank_solve, "padd");
        identity_checks += 2;
        cur.push((a.cold_s + b.cold_s) / 2.0);
        padd.push(p.cold_s);
        aa_a.push(a.cold_s);
        aa_b.push(b.cold_s);
        rank_cur.push((a.rank_s + b.rank_s) / 2.0);
        rank_padd.push(p.rank_s);
        let mut draws = vec![];
        for d in 0..3 {
            let r = rho_run(&runs.join(format!("p{i}_rho{d}")));
            assert_eq!(r.scalar, ans[i - 1], "point {i} rho scalar");
            scalar_checks += 1;
            draws.push(r.online_s);
        }
        rho_online.push(draws);
    }
    let all_rho: Vec<f64> = rho_online.iter().flatten().copied().collect();
    let rho_mean = mean(&all_rho);
    let saving: Vec<f64> = cur.iter().zip(&padd).map(|(c, p)| c - p).collect();
    let m = &claimed["metrics"];
    let heads = vec![
        headline("cold_s.cur_mean", f(&m["cold_s"]["cur_mean"]), mean(&cur)),
        headline("cold_s.padd_mean", f(&m["cold_s"]["padd_mean"]), mean(&padd)),
        headline("cold_s.paired_saving_mean", f(&m["cold_s"]["paired_saving_mean"]), mean(&saving)),
        headline("cold_s.ratio_cur_over_padd", f(&m["cold_s"]["ratio_cur_over_padd"]), cur.iter().sum::<f64>() / padd.iter().sum::<f64>()),
        headline("cold_s.aa_ratio_a_over_b", f(&m["cold_s"]["aa_ratio_a_over_b"]), aa_a.iter().sum::<f64>() / aa_b.iter().sum::<f64>()),
        headline("rank_s.cur_mean", f(&m["rank_s"]["cur_mean"]), mean(&rank_cur)),
        headline("rank_s.padd_mean", f(&m["rank_s"]["padd_mean"]), mean(&rank_padd)),
        headline("rank_s.ratio_cur_over_padd", f(&m["rank_s"]["ratio_cur_over_padd"]), rank_cur.iter().sum::<f64>() / rank_padd.iter().sum::<f64>()),
        headline("rho.mean_online_s", f(&claimed["rho"]["mean_online_s"]), rho_mean),
        headline("rho.ratios.cur.ic_over_rho_online", f(&claimed["rho"]["ratios"]["cur"]["ic_over_rho_online"]), mean(&cur) / rho_mean),
        headline("rho.ratios.padd.ic_over_rho_online", f(&claimed["rho"]["ratios"]["padd"]["ic_over_rho_online"]), mean(&padd) / rho_mean),
    ];
    let agree = heads.iter().all(|h| {
        let (c, r) = (f(&h["claimed"]), f(&h["recomputed"]));
        (c - r).abs() <= 1e-9 * c.abs().max(r.abs()).max(1.0)
    });
    let mut raw_files: Vec<PathBuf> = vec![];
    for e in walk(raw) {
        raw_files.push(e);
    }
    raw_files.sort();
    let raw_data: Vec<Value> = raw_files
        .iter()
        .filter(|p| p.file_name().map(|n| n == "usage.json" || n == "receipt.json" || n == "stdout.jsonl" || n == "targets_result.jsonl").unwrap_or(false))
        .map(|p| json!({"path": p.to_string_lossy(), "sha256": sha256_file(p)}))
        .collect();
    let report = json!({
        "schema": "n83.padd-rank-validator-report/v1",
        "status": if agree { "pass" } else { "fail" },
        "independent": true,
        "independence": "native recomputation (examples/n83_panel_analyze.rs) written in a separate session from the producer's analyze.py; reads only the raw provider records",
        "provider_artifact": {"id": 12223498464u64, "run": 39296352594u64,
                              "zip_sha256": "41e1a7f4c7ee33c71902b39d21c52189abb0b5f2e7f9a15fbdf910b082bd9f19"},
        "scalar_checks": scalar_checks, "identity_checks": identity_checks,
        "headline": heads,
        "not_recomputed": "bootstrap intervals (the producer's Python Mersenne-Twister resampling); the point estimates above are exact recomputations",
        "raw_data": raw_data,
    });
    fs::write(out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    println!("validator status {} ({} scalar checks, {} identity checks)", report["status"], scalar_checks, identity_checks);
    assert!(agree, "headline mismatch");
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out
}

fn panel(dir: &Path, answers_path: &Path, first: usize, out: &Path) {
    let ans = answers(answers_path);
    let runs = dir.join("runs");
    let mut rng = Rng(20261024);
    let mut rows = vec![];
    let (mut cur, mut padd, mut h4, mut h1, mut aa_a, mut aa_b) = (vec![], vec![], vec![], vec![], vec![], vec![]);
    let mut rho_online: Vec<Vec<f64>> = vec![];
    let mut rho_wall: Vec<Vec<f64>> = vec![];
    let mut problems: Vec<String> = vec![];
    let mut rank_probe_set = std::collections::BTreeSet::new();
    let (mut h3_ops, mut base_ops, mut rho_adds, mut rho_unpriced) = (vec![], vec![], vec![], vec![]);
    let mut h3_split = vec![];
    for i in first..=20 {
        let arm = |a: &str| ic_run(&runs.join(format!("p{i}_{a}")));
        let (a, b, p) = (arm("cur_a"), arm("cur_b"), arm("padd"));
        for (name, r) in [("cur_a", &a), ("cur_b", &b), ("padd", &p)] {
            if r.scalar != ans[i - 1] {
                problems.push(format!("p{i} {name} scalar"));
            }
        }
        if a.identity != b.identity || a.identity != p.identity {
            problems.push(format!("p{i} baseline identity"));
        }
        let mut h3 = |t: usize| -> (f64, Value) {
            let d = runs.join(format!("p{i}_h3_t{t}"));
            let usage = read(&d.join("usage.json"));
            if usage["status"] != "PASS_PRODUCER" {
                problems.push(format!("p{i} h3_t{t} {}", usage["status"]));
            }
            let s = read(&d.join("summary.json"));
            let tr = &s["targets"][0];
            if tr["recovered_scalar"].as_str() != Some(ans[i - 1].as_str()) || tr["group_verified"] != true {
                problems.push(format!("p{i} h3_t{t} scalar"));
            }
            (f(&usage["wall_seconds"]), s)
        };
        let (c4, s4) = h3(4);
        let (c1, s1) = h3(1);
        rank_probe_set.insert(s4["rank_probes"].as_u64().unwrap());
        rank_probe_set.insert(s1["rank_probes"].as_u64().unwrap());
        let mut on = vec![];
        let mut wa = vec![];
        let mut adds = vec![];
        let mut unp = vec![];
        for dd in 0..3 {
            let r = rho_run(&runs.join(format!("p{i}_rho{dd}")));
            if r.scalar != ans[i - 1] {
                problems.push(format!("p{i} rho{dd} scalar"));
            }
            on.push(r.online_s);
            wa.push(r.wall_s);
            let ch = &r.charges;
            adds.push(f(&ch["group_additions"]));
            let mut u = 0.0;
            for key in ["canonicalizations", "partition_hashes", "table_queries", "table_inserts"] {
                if let Some(v) = ch.get(key) {
                    u += f(v);
                }
            }
            unp.push(u);
        }
        // h=3 counted operations: every stored key and every probe is one
        // x-only point addition (shared inversion); anchors are additions.
        let tp: f64 = f(&s4["targets"][0]["winning_stream_probes"]);
        let entries = f(&s4["table_inserted_entries"]);
        let probes = f(&s4["rank_probes"]);
        let anchors = f(&s4["rank_anchor_steps"]);
        let h3_add = entries + probes + anchors + tp;
        let h3_unp = 2.0 * (entries + probes + tp); // one canonicalisation and one table access each
        h3_ops.push((h3_add, h3_unp));
        h3_split.push(json!({"table_entries": entries, "rank_probes": probes, "rank_anchor_steps": anchors, "target_winning_stream_probes": tp}));
        // Baseline (padd arm): the rank and target probes and the index's
        // regular states are its native work; it emits no group-addition
        // count (PR #2282 section 5a), so its additions are bounded below by
        // the rank anchors (one scalar multiplication each) only.
        let b_unp = 2.5 * (p.rank_probes + p.target_probes) + 2.0 * p.regular_states;
        base_ops.push(b_unp);
        rho_adds.push(mean(&adds));
        rho_unpriced.push(mean(&unp));
        cur.push((a.cold_s + b.cold_s) / 2.0);
        padd.push(p.cold_s);
        aa_a.push(a.cold_s);
        aa_b.push(b.cold_s);
        h4.push(c4);
        h1.push(c1);
        rho_online.push(on.clone());
        rho_wall.push(wa.clone());
        rows.push(json!({"point": i, "cur_a_s": a.cold_s, "cur_b_s": b.cold_s, "padd_s": p.cold_s,
                         "padd_setup_s": p.setup_s, "padd_rank_s": p.rank_s,
                         "h3_t4_s": c4, "h3_t1_s": c1, "h3_t4_timing_ms": s4["timing_ms"], "h3_t1_timing_ms": s1["timing_ms"],
                         "rho_online_s": on, "rho_wall_s": wa}));
    }
    let d: Vec<f64> = padd.iter().zip(&h4).map(|(p, h)| p - h).collect();
    let all_rho: Vec<f64> = rho_online.iter().flatten().copied().collect();
    let rho_mean = mean(&all_rho);
    let rho_wall_mean = mean(&rho_wall.iter().flatten().copied().collect::<Vec<_>>());
    let ratio_block = |name: &str, v: &[f64], rng: &mut Rng| {
        json!({"arm": name, "mean_cold_s": mean(v), "ic_over_rho_online": mean(v) / rho_mean,
               "ic_over_rho_online_boot95": boot_ic_over_rho(v, &rho_online, rng),
               "ic_over_rho_child_wall": mean(v) / rho_wall_mean})
    };
    let lo = boot_mean(&d, &mut rng);
    let ratios = vec![
        ratio_block("padd (K1120 baseline, 4 rank threads)", &padd, &mut rng),
        ratio_block("h3_t4 (4 threads)", &h4, &mut rng),
        ratio_block("h3_t1 (1 thread, core-matched to rho)", &h1, &mut rng),
    ];
    let band: Vec<Value> = [0.0, 0.1, 1.0]
        .iter()
        .map(|&w| {
            let h3 = mean(&h3_ops.iter().map(|(a, u)| a + w * u).collect::<Vec<_>>());
            let rho = mean(&rho_adds.iter().zip(&rho_unpriced).map(|(a, u)| a + w * u).collect::<Vec<_>>());
            let base = mean(&base_ops.iter().map(|u| w * u).collect::<Vec<_>>());
            json!({"w_gae": w, "h3_gae": h3, "rho_gae": rho, "baseline_unpriced_gae": base, "h3_over_rho": h3 / rho})
        })
        .collect();
    let verdict = if !problems.is_empty() {
        "INVALID"
    } else if lo[0] > 0.0 {
        "IMPROVEMENT"
    } else if lo[1] < 0.0 {
        "REGRESSION"
    } else {
        "NOT_SEPARATED"
    };
    let out_v = json!({
        "schema": "n83.mitm3-panel-analysis/v1",
        "points": format!("{first}..=20"), "n_points": d.len(), "problems": problems, "verdict": verdict,
        "primary": {"padd_mean_s": mean(&padd), "h3_t4_mean_s": mean(&h4),
                    "paired_padd_minus_h3_t4_mean_s": mean(&d), "paired_boot95": lo,
                    "ratio_padd_over_h3_t4": padd.iter().sum::<f64>() / h4.iter().sum::<f64>(),
                    "ratio_boot95": boot_ratio(&padd, &h4, &mut rng)},
        "h3_t1_mean_s": mean(&h1),
        "h3_t1_over_h3_t4": mean(&h1) / mean(&h4),
        "aa_cur_a_over_cur_b": aa_a.iter().sum::<f64>() / aa_b.iter().sum::<f64>(),
        "baseline_cur_mean_s": mean(&cur),
        "rho": {"draws": all_rho.len(), "mean_online_s": rho_mean, "mean_child_wall_s": rho_wall_mean},
        "ic_over_rho": ratios,
        "h3_rank_probe_counts_distinct": rank_probe_set.len(),
        "section_5a": {"band": band, "h3_counts_per_point": h3_split,
                       "note": "h3 additions = table entries + rank probes + anchor steps + winning-stream target probes; h3 unpriced = one canonicalisation and one table access per entry and probe; rho additions and unpriced counters from its receipts; the baseline emits no group-addition count, so only its unpriced native work (2.5 per probe: canonicalisation, table query, half a root solve; 2 per regular index state) is priced"},
        "rows": rows,
    });
    fs::write(out, serde_json::to_string_pretty(&out_v).unwrap() + "\n").unwrap();
    println!("{}", serde_json::to_string_pretty(&json!({"verdict": verdict, "primary": out_v["primary"], "ic_over_rho": out_v["ic_over_rho"], "problems": out_v["problems"]})).unwrap());
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("validate-2282") if a.len() == 6 => {
            validate_2282(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4]), Path::new(&a[5]))
        }
        Some("panel") if a.len() == 6 => {
            panel(Path::new(&a[2]), Path::new(&a[3]), a[4].parse().unwrap(), Path::new(&a[5]))
        }
        _ => {
            eprintln!("usage: n83_panel_analyze validate-2282 <raw> <answers> <claimed> <out> | panel <dir> <answers> <first_point> <out>");
            std::process::exit(2);
        }
    }
}
