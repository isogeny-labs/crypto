//! `analysis/summary.json` for `research/postgb_linear_stage_20261026`: every
//! measure and prediction of its PROTOCOL.md (amendments 1–3), computed from
//! the run records alone. No number is typed in.
//!
//! * Wall time: host B's R2 cards (`runs/R2-L1..L6`, the portable ablation
//!   `runs/R2-A1..A2`, the linear stage's time in `runs/R2-D1..D2`).
//! * Counted multiplications: `runs/R3-C.json` (both arms in one run, the
//!   fixed counter of amendment 3) and `runs/R3-D1..D2` (S, S/rho, the
//!   negation-map rho reference).
//! * Host A: the first `runs/L1..L4`, a host-dependence check only.
//!
//! `cargo run --release --example postgb_la_summary [study_dir]`

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};

const FAMILIES: [&str; 5] = ["jv-nagao", "gaudry-s4", "gaudry-d4", "gaudry-s5", "dense"];
const SEED: u64 = 20261027;
const RESAMPLES: usize = 4000;

struct Row {
    card: String,
    v: Value,
}

impl Row {
    fn s(&self, k: &str) -> &str {
        self.v[k].as_str().unwrap_or_default()
    }
    fn f(&self, k: &str) -> f64 {
        self.v[k].as_f64().unwrap_or(f64::NAN)
    }
    fn key(&self) -> (String, String) {
        (self.s("family").to_string(), self.s("label").to_string())
    }
    fn arm(&self, a: &str) -> Option<&Value> {
        self.v["arms"].as_array()?.iter().find(|x| x["arm"] == a)
    }
    fn ms(&self, a: &str) -> Option<f64> {
        self.arm(a).and_then(|x| x["median_ms"].as_f64())
    }
    fn muls(&self, a: &str) -> Option<f64> {
        self.arm(a).and_then(|x| x["muls"].as_f64())
    }
}

fn read(dir: &Path, name: &str) -> Value {
    let p = dir.join("runs").join(format!("{name}.json"));
    let t = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    serde_json::from_str(&t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn load(dir: &Path, names: &[&str]) -> (Vec<Row>, Map<String, Value>) {
    let mut rows = Vec::new();
    let mut meta = Map::new();
    for n in names {
        let d = read(dir, n);
        let mut m = Map::new();
        for (k, v) in d.as_object().expect("a run object") {
            if k != "rows" {
                m.insert(k.clone(), v.clone());
            }
        }
        meta.insert(n.to_string(), Value::Object(m));
        for r in d["rows"].as_array().expect("rows") {
            rows.push(Row {
                card: n.to_string(),
                v: r.clone(),
            });
        }
    }
    (rows, meta)
}

fn geomean(xs: &[f64]) -> f64 {
    (xs.iter().map(|x| x.ln()).sum::<f64>() / xs.len() as f64).exp()
}

/// Geometric mean and bootstrap 95% interval over systems (log scale).
fn boot_geomean(logs: &[f64]) -> (f64, [f64; 2]) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let n = logs.len();
    let m = logs.iter().sum::<f64>() / n as f64;
    let mut bs: Vec<f64> = (0..RESAMPLES)
        .map(|_| (0..n).map(|_| logs[rng.gen_range(0..n)]).sum::<f64>() / n as f64)
        .collect();
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lo = bs[(0.025 * RESAMPLES as f64) as usize];
    let hi = bs[(0.975 * RESAMPLES as f64) as usize - 1];
    (m.exp(), [lo.exp(), hi.exp()])
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut o: Vec<usize> = (0..v.len()).collect();
    o.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < o.len() {
        let mut j = i;
        while j + 1 < o.len() && v[o[j + 1]] == v[o[i]] {
            j += 1;
        }
        for k in i..=j {
            r[o[k]] = (i + j) as f64 / 2.0;
        }
        i = j + 1;
    }
    r
}

fn spearman(xs: &[f64], ys: &[f64]) -> f64 {
    let (rx, ry) = (ranks(xs), ranks(ys));
    let n = rx.len() as f64;
    let (mx, my) = (rx.iter().sum::<f64>() / n, ry.iter().sum::<f64>() / n);
    let num: f64 = rx.iter().zip(&ry).map(|(a, b)| (a - mx) * (b - my)).sum();
    let den = (rx.iter().map(|a| (a - mx).powi(2)).sum::<f64>()
        * ry.iter().map(|b| (b - my).powi(2)).sum::<f64>())
    .sqrt();
    num / den
}

/// Counted sparse ÷ fast per system, from R3-C (amendment 3's fixed counter).
/// The R2 rows' own counts carry the old fast-arm counter, and R3-C runs
/// no dense arm, so a table without R3 counts reports no counted ratio.
type Counts = BTreeMap<(String, String), (f64, f64)>;

fn counted(r: &Row, counts: Option<&Counts>) -> Option<f64> {
    counts?.get(&r.key()).map(|(s, f)| s / f.max(1.0))
}

fn family_table(rows: &[Row], num: &str, den: &str, counts: Option<&Counts>) -> Value {
    let mut out = Map::new();
    for fam in FAMILIES {
        let rs: Vec<&Row> = rows
            .iter()
            .filter(|r| r.s("family") == fam && r.ms(num).is_some() && r.ms(den).is_some())
            .collect();
        if rs.is_empty() {
            continue;
        }
        let logs: Vec<f64> = rs
            .iter()
            .map(|r| (r.ms(num).unwrap() / r.ms(den).unwrap()).ln())
            .collect();
        let (g, ci) = boot_geomean(&logs);
        let mut ns: Vec<u64> = rs.iter().map(|r| r.v["n"].as_u64().unwrap()).collect();
        ns.sort();
        ns.dedup();
        let mut ps: Vec<u64> = rs.iter().map(|r| r.v["p"].as_u64().unwrap()).collect();
        ps.sort();
        ps.dedup();
        let deltas: Vec<u64> = rs.iter().map(|r| r.v["delta"].as_u64().unwrap()).collect();
        let median = |a: &str| {
            let mut v: Vec<f64> = rs.iter().map(|r| r.ms(a).unwrap()).collect();
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            v[v.len() / 2]
        };
        let cr: Vec<f64> = rs.iter().filter_map(|r| counted(r, counts)).collect();
        let mut e = Map::new();
        e.insert("systems".into(), json!(rs.len()));
        e.insert(
            "delta".into(),
            json!([deltas.iter().min(), deltas.iter().max()]),
        );
        e.insert("n".into(), json!(ns));
        e.insert("p".into(), json!(ps));
        e.insert("geomean".into(), json!(g));
        e.insert("ci95".into(), json!(ci));
        e.insert(
            "min".into(),
            json!(logs.iter().cloned().fold(f64::INFINITY, f64::min).exp()),
        );
        e.insert(
            "max".into(),
            json!(logs.iter().cloned().fold(f64::NEG_INFINITY, f64::max).exp()),
        );
        e.insert(
            "median_ms".into(),
            json!({ num: median(num), den: median(den) }),
        );
        if !cr.is_empty() {
            e.insert(
                "counted_ratio".into(),
                json!({
                    "min": cr.iter().cloned().fold(f64::INFINITY, f64::min),
                    "max": cr.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    "geomean": geomean(&cr),
                }),
            );
        }
        out.insert(fam.into(), Value::Object(e));
    }
    Value::Object(out)
}

fn dlp_rows(v: &Value) -> BTreeMap<(u64, u64), Value> {
    let rs = v
        .as_array()
        .cloned()
        .or_else(|| v["rows"].as_array().cloned())
        .expect("DLP rows");
    rs.into_iter()
        .map(|r| r.get("Dlp").cloned().unwrap_or(r))
        .filter(|r| r.get("rho").is_some())
        .map(|r| ((r["p"].as_u64().unwrap(), r["seed"].as_u64().unwrap()), r))
        .collect()
}

fn main() {
    let dir: PathBuf = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("research/postgb_linear_stage_20261026"));
    let corpus_cards = ["R2-L1", "R2-L2", "R2-L3", "R2-L4", "R2-L5", "R2-L6"];
    let (corpus, meta) = load(&dir, &corpus_cards);
    let (abl, meta_a) = load(&dir, &["R2-A1", "R2-A2"]);
    let (host_a, meta_ha) = load(&dir, &["L1", "L2", "L3", "L4"]);
    let (r3, meta_r3) = load(&dir, &["R3-C"]);

    // Amendment 3: counts from R3-C, whose solution sets must equal R2's.
    let mut counts = Counts::new();
    let mut r3_mismatch = Vec::new();
    let r3_by: BTreeMap<_, _> = r3.iter().map(|r| (r.key(), r)).collect();
    for r in &corpus {
        match r3_by.get(&r.key()) {
            Some(c) => {
                for a in ["sparse", "fast"] {
                    if c.arm(a).map(|x| &x["sols"]) != r.arm(a).map(|x| &x["sols"]) {
                        r3_mismatch.push(format!(
                            "{}:{}:{}:{a}",
                            r.card,
                            r.s("family"),
                            r.s("label")
                        ));
                    }
                }
                counts.insert(
                    r.key(),
                    (c.muls("sparse").unwrap(), c.muls("fast").unwrap()),
                );
            }
            None => r3_mismatch.push(format!(
                "{}:{}:{}:missing",
                r.card,
                r.s("family"),
                r.s("label")
            )),
        }
    }
    if r3.len() != corpus.len() {
        r3_mismatch.push(format!(
            "R3-C has {} systems, R2 {}",
            r3.len(),
            corpus.len()
        ));
    }

    let mut skipped = Map::new();
    for (k, v) in meta.iter().chain(meta_a.iter()).chain(meta_r3.iter()) {
        skipped.insert(k.clone(), v.get("skipped").cloned().unwrap_or(json!([])));
    }
    let lab = |r: &Row| format!("{}:{}", r.card, r.s("label"));
    let fast_gave_up: Vec<String> = corpus
        .iter()
        .filter_map(|r| {
            let g = &r.arm("fast")?["gave_up"];
            (!g.is_null()).then(|| format!("{}:{}", lab(r), g.as_str().unwrap_or("?")))
        })
        .collect();
    let disagree: Vec<String> = corpus
        .iter()
        .filter(|r| r.v["agree"] != true)
        .map(|r| format!("{}:{}:{}", r.card, r.s("family"), r.s("label")))
        .collect();
    let planted_lost: Vec<String> = corpus
        .iter()
        .filter(|r| r.v["planted_found"] == false)
        .map(lab)
        .collect();
    let sparse_fell_back: Vec<String> = corpus
        .iter()
        .filter(|r| r.arm("sparse").is_some_and(|a| a["fallback"] == true))
        .map(lab)
        .collect();
    let ablation_disagree: Vec<String> = abl
        .iter()
        .filter(|r| r.v["agree"] != true)
        .map(lab)
        .collect();
    let skipped_any = skipped
        .values()
        .any(|v| v.as_array().is_some_and(|a| !a.is_empty()));
    let pass = disagree.is_empty()
        && planted_lost.is_empty()
        && ablation_disagree.is_empty()
        && r3_mismatch.is_empty()
        && !skipped_any;
    let gates = json!({
        "systems": corpus.len(), "disagree": disagree, "planted_lost": planted_lost,
        "fast_gave_up": fast_gave_up, "sparse_fell_back": sparse_fell_back,
        "ablation_disagree": ablation_disagree, "r3_counts_mismatch": r3_mismatch, "pass": pass,
    });

    let so = family_table(&corpus, "sparse", "fast", Some(&counts));
    let dense_rows: Vec<&Row> = corpus.iter().filter(|r| r.s("family") == "dense").collect();
    let rho = spearman(
        &dense_rows.iter().map(|r| r.f("delta")).collect::<Vec<_>>(),
        &dense_rows
            .iter()
            .map(|r| r.ms("sparse").unwrap() / r.ms("fast").unwrap())
            .collect::<Vec<_>>(),
    );
    let simd: BTreeMap<_, _> = corpus
        .iter()
        .map(|r| (r.key(), r.ms("fast").unwrap()))
        .collect();
    let mut portable = Map::new();
    for fam in FAMILIES {
        let ratios: Vec<f64> = abl
            .iter()
            .filter(|r| r.s("family") == fam)
            .filter_map(|r| Some(r.ms("fast")? / simd.get(&r.key())?))
            .collect();
        if !ratios.is_empty() {
            portable.insert(fam.into(), json!(geomean(&ratios)));
        }
    }
    let systems: Vec<Value> = corpus
        .iter()
        .map(|r| {
            let fast = r.arm("fast").unwrap();
            let phases = fast.get("fast").filter(|x| !x.is_null()).map(|x| {
                json!({ "nf_ms": x["nf_ms"], "krylov_ms": x["krylov_ms"], "bm_ms": x["bm_ms"],
                        "roots_ms": x["roots_ms"], "readout_ms": x["readout_ms"] })
            });
            let arms = r.v["arms"].as_array().unwrap();
            let ms: Map<String, Value> = arms.iter().map(|a| (a["arm"].as_str().unwrap().into(), a["median_ms"].clone())).collect();
            let muls_r2: Map<String, Value> = arms.iter().map(|a| (a["arm"].as_str().unwrap().into(), a["muls"].clone())).collect();
            let (cs, cf) = counts[&r.key()];
            json!({
                "card": r.card, "family": r.s("family"), "label": r.s("label"), "n": r.v["n"], "p": r.v["p"],
                "delta": r.v["delta"], "f4_ms": r.v["f4_ms"], "sols": fast["sols"], "agree": r.v["agree"],
                "planted_found": r.v["planted_found"], "ms": ms,
                "muls": { "sparse": cs, "fast": cf }, "muls_r2": muls_r2, "fast_phases_ms": phases,
            })
        })
        .collect();

    // jv_cover DLP: wall time from R2-D, counts and the references from R3-D.
    let (t1, t2) = (
        dlp_rows(&read(&dir, "R2-D1")),
        dlp_rows(&read(&dir, "R2-D2")),
    );
    let (c1, c2) = (
        dlp_rows(&read(&dir, "R3-D1")),
        dlp_rows(&read(&dir, "R3-D2")),
    );
    let mut dlp = Vec::new();
    let mut dlp_mismatch = Vec::new();
    for (k, x) in &t1 {
        let (Some(y), Some(cx), Some(cy)) = (t2.get(k), c1.get(k), c2.get(k)) else {
            dlp_mismatch.push(format!("{k:?}: missing"));
            continue;
        };
        for (t, c) in [(x, cx), (y, cy)] {
            for f in ["relations", "correct", "solved", "unknowns"] {
                if t[f] != c[f] {
                    dlp_mismatch.push(format!("{k:?}: {f}"));
                }
            }
            let rt: Vec<_> = t["rho"]
                .as_array()
                .unwrap()
                .iter()
                .map(|z| (z["s"].clone(), z["correct"].clone()))
                .collect();
            let rc: Vec<_> = c["rho"]
                .as_array()
                .unwrap()
                .iter()
                .map(|z| (z["s"].clone(), z["correct"].clone()))
                .collect();
            if rt != rc {
                dlp_mismatch.push(format!("{k:?}: plain rho"));
            }
        }
        let num = |v: &Value, f: &str| v[f].as_f64().unwrap();
        let rho_all = [cx, cy]
            .iter()
            .flat_map(|c| {
                c["rho"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .chain(c["rho_neg"].as_array().into_iter().flatten())
            })
            .all(|z| z["correct"] == true);
        dlp.push(json!({
            "p": k.0, "seed": k.1,
            "correct": [cx["correct"], cy["correct"]],
            "s": [num(cx, "s"), num(cy, "s")],
            "s_ratio_sparse_over_fast": num(cx, "s") / num(cy, "s"),
            "lin_muls": [cx["lin_muls"], cy["lin_muls"]],
            "lin_muls_r2": [x["lin_muls"], y["lin_muls"]],
            "lin_ms": [num(x, "lin_ms"), num(y, "lin_ms")],
            "f4_ms": [x["f4_ms"], y["f4_ms"]],
            "lin_ms_ratio_sparse_over_fast": num(x, "lin_ms") / num(y, "lin_ms"),
            "wall_ms": [num(x, "wall_ms"), num(y, "wall_ms")],
            "rho_s_mean": [num(cx, "rho_s_mean"), num(cy, "rho_s_mean")],
            "s_over_rho": [num(cx, "s_over_rho"), num(cy, "s_over_rho")],
            "rho_neg_s_mean": [cx["rho_neg_s_mean"], cy["rho_neg_s_mean"]],
            "s_over_rho_neg": [cx["s_over_rho_neg"], cy["s_over_rho_neg"]],
            "relations": [cx["relations"], cy["relations"]],
            "rho_all_correct": rho_all,
        }));
    }

    // Predictions.
    let fams = so.as_object().unwrap();
    let p1 = fams
        .values()
        .all(|v| v["geomean"].as_f64().unwrap() >= 1.5 && v["ci95"][0].as_f64().unwrap() > 1.0);
    let by_family: Map<String, Value> = fams
        .iter()
        .map(|(k, v)| (k.clone(), json!([v["geomean"], v["ci95"]])))
        .collect();
    let gave_up_frac = gates["fast_gave_up"].as_array().unwrap().len() as f64 / corpus.len() as f64;
    let cr: Vec<f64> = counts.values().map(|(s, f)| s / f.max(1.0)).collect();
    let logs_correct = dlp
        .iter()
        .all(|r| r["correct"][0] == true && r["correct"][1] == true);
    // P4's wording: fast's counted multiplications within ±25% of sparse's.
    let fast_over_sparse: Vec<f64> = counts.values().map(|(s, f)| f / s.max(1.0)).collect();
    let within = fast_over_sparse.iter().all(|c| (0.75..=1.25).contains(c));
    let s10 = dlp
        .iter()
        .all(|r| (r["s_ratio_sparse_over_fast"].as_f64().unwrap() - 1.0).abs() <= 0.10);
    let lin_lower = dlp
        .iter()
        .all(|r| r["lin_ms_ratio_sparse_over_fast"].as_f64().unwrap() > 1.0);
    let ab = family_table(&abl, "sparse", "fast", None);
    let p5_by: Map<String, Value> = ab
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v["geomean"].clone()))
        .collect();
    let p5 = p5_by.values().all(|v| v.as_f64().unwrap() >= 1.2);
    let predictions = json!({
        "P1": { "held": p1, "by_family": by_family },
        "P2": { "held": rho > 0.0, "spearman_rho": rho, "systems": dense_rows.len() },
        "P3": { "held": gave_up_frac <= 0.05, "gave_up_fraction": gave_up_frac },
        "P4": {
            "logs_correct": logs_correct, "counted_within_25pct_every_system": within,
            "counted_ratio_range": [cr.iter().cloned().fold(f64::INFINITY, f64::min), cr.iter().cloned().fold(f64::NEG_INFINITY, f64::max)],
            "counted_outside_25pct": fast_over_sparse.iter().filter(|c| !(0.75..=1.25).contains(*c)).count(),
            "s_within_10pct": s10, "lin_ms_lower_every_run": lin_lower,
            "held": logs_correct && within && s10 && lin_lower,
        },
        "P5": { "held": p5, "by_family": p5_by },
    });

    let out = json!({
        "schema": "postgb_linear_stage.summary/v1",
        "tool": "examples/postgb_la_summary.rs",
        "host": "B (amendment 2); counts R3 (amendment 3)",
        "meta": meta, "meta_ablation": meta_a, "meta_host_a": meta_ha, "meta_counts": meta_r3,
        "skipped": skipped,
        "gates": gates,
        "sparse_over_fast": so,
        "dense_over_fast": family_table(&corpus, "dense", "fast", None),
        "ablation_sparse_over_portable_fast": ab,
        "host_a_sparse_over_fast": family_table(&host_a, "sparse", "fast", None),
        "host_a_dense_over_fast": family_table(&host_a, "dense", "fast", None),
        "portable_over_simd_fast": portable,
        "systems": systems,
        "dlp": dlp,
        "dlp_mismatch": dlp_mismatch,
        "predictions": predictions,
    });
    let a = dir.join("analysis");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::write(
        a.join("summary.json"),
        serde_json::to_string_pretty(&out).unwrap() + "\n",
    )
    .unwrap();
    for k in ["P1", "P2", "P3", "P4", "P5"] {
        println!("{k} {}", out["predictions"][k]["held"]);
    }
    println!(
        "gates {} dlp_mismatch {}",
        out["gates"]["pass"],
        out["dlp_mismatch"].as_array().unwrap().len()
    );
}
