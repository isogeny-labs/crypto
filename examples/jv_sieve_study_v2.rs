//! Analysis of `research/jv_sieve_strong_rho_v2_20261020` (PROTOCOL.md, V2): reads the raw runs of
//! `jv_cover --exp sieve` and `--exp rhostrong` for seeds 17-32, checks the numeric acceptance rules
//! A1-A7 and, only if all pass, evaluates the numeric decision rules D1-D6.  Every threshold below
//! is the protocol's; none was chosen after data.  Computation lives here, in Rust.
//!
//! ```text
//! cargo run --release --example jv_sieve_study_v2 -- research/jv_sieve_strong_rho_v2_20261020
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const GAE: f64 = 331.0;
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
const SIZES: [u64; 5] = [251, 503, 1009, 1511, 1823];
const B: usize = 10_000;
const FIRST_SEED: u64 = 17;
const N_SEEDS: u64 = 16;

// ---- protocol thresholds (PROTOCOL.md, frozen before data) ----
const LEDGER_S: f64 = 1.3;
const A2_PLAIN: (f64, f64) = (1.98, 2.02);
const A2_NEG: (f64, f64) = (3.9, 4.1);
const A3_POINT: (f64, f64) = (1.0, 1.6);
const A4_RATIO: (f64, f64) = (0.85, 1.15);
const A7_POINT: (f64, f64) = (0.45, 0.55);
const FLAT: (f64, f64) = (-0.1, 0.1);

fn hash_scope_ok(neg: bool, hash: f64, steps: f64) -> bool {
    let (lo, hi) = if neg { A2_NEG } else { A2_PLAIN };
    steps > 0.0 && (lo..=hi).contains(&(hash / steps))
}
fn a3_ok(point: f64, lo: f64, hi: f64) -> bool {
    lo <= LEDGER_S && LEDGER_S <= hi && (A3_POINT.0..=A3_POINT.1).contains(&point)
}
fn a4_ok(ratio: f64, lo: f64, hi: f64) -> bool {
    (A4_RATIO.0..=A4_RATIO.1).contains(&ratio) && lo <= 1.0 && 1.0 <= hi
}
fn a7_ok(slope: f64, lo: f64, hi: f64) -> bool {
    (A7_POINT.0..=A7_POINT.1).contains(&slope) && lo <= 0.5 && 0.5 <= hi
}
/// D3 classification of a fitted exponent by its 95 % interval.
fn classify_fit(lo: f64, hi: f64) -> &'static str {
    if lo <= 1.0 / 3.0 && 1.0 / 3.0 <= hi {
        "l^(1/3) supported"
    } else if FLAT.0 <= lo && hi <= FLAT.1 {
        "flat"
    } else if hi < FLAT.0 {
        "falling"
    } else {
        "other"
    }
}
/// "IC<rho" when the whole interval of IC/rho lies below 1, "rho<IC" when above, else "unresolved".
fn verdict(lo: f64, hi: f64) -> &'static str {
    if !(lo.is_finite() && hi.is_finite()) {
        "unavailable"
    } else if hi < 1.0 {
        "IC<rho"
    } else if lo > 1.0 {
        "rho<IC"
    } else {
        "unresolved"
    }
}

/// The frozen sample of each cell: arm -> number of seeds from FIRST_SEED.
fn expected_n(p: u64, arm: &str) -> u64 {
    match (arm, p) {
        ("R3", _) => N_SEEDS,
        ("R0", 1511) => 8,
        ("R0", 1823) => 4,
        ("R0", _) => N_SEEDS,
        ("R1" | "R2", 251 | 503 | 1009) => N_SEEDS,
        ("R0_aa" | "R3_aa", 1009) => N_SEEDS,
        _ => 0,
    }
}
fn expected_seeds(n: u64) -> Vec<u64> {
    (FIRST_SEED..FIRST_SEED + n).collect()
}
/// Exactly the scheduled seeds: no missing, no duplicate, no extra.
fn inventory_ok(seeds: &[u64], expected: &[u64]) -> bool {
    let set: BTreeSet<u64> = seeds.iter().copied().collect();
    set.len() == seeds.len() && set == expected.iter().copied().collect()
}

fn load(dir: &str, name: &str) -> Option<Vec<Value>> {
    if let Ok(s) = fs::read_to_string(format!("{dir}/raw/{name}.json")) {
        return serde_json::from_str::<Vec<Value>>(&s)
            .ok()
            .filter(|r| !r.is_empty());
    }
    // per-instance rho jobs `<name>_s<seed>.json`, only those with a `.done` marker
    let mut out: Vec<(u64, Vec<Value>)> = vec![];
    for e in fs::read_dir(format!("{dir}/raw")).ok()?.flatten() {
        let fname = e.file_name().to_string_lossy().to_string();
        let Some(rest) = fname.strip_prefix(&format!("{name}_s")) else {
            continue;
        };
        let Some(seed) = rest
            .strip_suffix(".json")
            .and_then(|x| x.parse::<u64>().ok())
        else {
            continue;
        };
        if !e.path().with_extension("done").exists() {
            continue;
        }
        let v: Vec<Value> = serde_json::from_str(&fs::read_to_string(e.path()).ok()?).ok()?;
        out.push((seed, v));
    }
    out.sort_by_key(|x| x.0);
    let rows: Vec<Value> = out.into_iter().flat_map(|x| x.1).collect();
    (!rows.is_empty()).then_some(rows)
}
fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}
fn fo(v: &Value, k: &str) -> f64 {
    v["ops"][k].as_f64().unwrap_or(0.0)
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn sd(x: &[f64]) -> f64 {
    if x.len() < 2 {
        return 0.0;
    }
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
}
fn pct(mut v: Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn boot<F: Fn(&[usize]) -> f64>(n: usize, seed: u64, stat: F) -> [f64; 2] {
    if n == 0 {
        return [f64::NAN, f64::NAN];
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = Vec::with_capacity(B);
    for _ in 0..B {
        let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
        out.push(stat(&idx));
    }
    [pct(out.clone(), 0.025), pct(out, 0.975)]
}
fn mean_ci(x: &[f64], seed: u64) -> Value {
    let ci = boot(x.len(), seed, |i| {
        mean(&i.iter().map(|&j| x[j]).collect::<Vec<_>>())
    });
    json!({"n": x.len(), "mean": mean(x), "sd": sd(x), "ci95": ci})
}
/// Ratio of means over instances present in both, paired by seed identity.
fn ratio_by_seed(a: &[(u64, f64)], b: &[(u64, f64)], seed: u64) -> Value {
    let bm: BTreeMap<u64, f64> = b.iter().copied().collect();
    let mut seen = BTreeSet::new();
    let pairs: Vec<(f64, f64)> = a
        .iter()
        .filter(|(s, _)| seen.insert(*s))
        .filter_map(|(s, x)| bm.get(s).map(|y| (*x, *y)))
        .collect();
    let n = pairs.len();
    let r = |i: &[usize]| {
        mean(&i.iter().map(|&j| pairs[j].0).collect::<Vec<_>>())
            / mean(&i.iter().map(|&j| pairs[j].1).collect::<Vec<_>>())
    };
    let all: Vec<usize> = (0..n).collect();
    json!({"n": n, "ratio": r(&all), "ci95": boot(n, seed, r)})
}
fn rl(v: &Value) -> (f64, f64, f64) {
    (
        v["ratio"].as_f64().unwrap_or(f64::NAN),
        v["ci95"][0].as_f64().unwrap_or(f64::NAN),
        v["ci95"][1].as_f64().unwrap_or(f64::NAN),
    )
}

struct Ic {
    seed: u64,
    l: f64,
    muls: f64,
    s: f64,
    s_plus: f64,
    adds: f64,
    lookups: f64,
    enum_adds: f64,
    unemitted: f64,
    wall: f64,
    lin: f64,
    desc: f64,
    digest: u64,
    ok: bool,
}
fn ic_rows(v: &[Value]) -> Vec<Ic> {
    v.iter()
        .map(|r| {
            let muls = f(r, "total_muls");
            Ic {
                seed: r["seed"].as_u64().unwrap_or(0),
                l: f(r, "l"),
                muls,
                s: f(r, "s"),
                s_plus: f(r, "s_plus"),
                adds: f(r, "sieve_adds"),
                lookups: f(r, "sieve_lookups"),
                enum_adds: f(r, "enum_adds"),
                unemitted: muls - f(r, "sieve_muls") - f(r, "enum_muls"),
                wall: f(r, "wall_ms"),
                lin: f(r, "descent_lin_muls"),
                desc: f(r, "descent_muls"),
                digest: r["descent_digest"].as_u64().unwrap_or(0),
                ok: r["solved"].as_bool() == Some(true)
                    && r["correct"].as_bool() == Some(true)
                    && r["seed"].as_u64().is_some()
                    && muls.is_finite()
                    && muls > 0.0,
            }
        })
        .collect()
}
struct Rho {
    seed: u64,
    l: f64,
    mh: f64,
    mt: f64,
    steps: f64,
    fp_adds: f64,
    native: f64,
    hash_ok: bool,
    wall: f64,
    ok: bool,
}
fn rho_rows(v: &[Value]) -> Vec<Rho> {
    v.iter()
        .map(|r| {
            let neg = r["negation"].as_bool() == Some(true);
            let steps = f(r, "steps");
            Rho {
                seed: r["seed"].as_u64().unwrap_or(0),
                l: f(r, "l"),
                mh: f(r, "muls_harness"),
                mt: f(r, "muls_tuned"),
                steps,
                fp_adds: fo(r, "fp_adds"),
                native: [
                    "canon_cmp",
                    "hash",
                    "dp_check",
                    "scalar_ops",
                    "table_ops",
                    "cycle_cmp",
                ]
                .iter()
                .map(|k| fo(r, k))
                .sum(),
                hash_ok: hash_scope_ok(neg, fo(r, "hash"), steps),
                wall: f(r, "wall_ms"),
                ok: r["correct"].as_bool() == Some(true)
                    && r["capped"].as_bool() == Some(false)
                    && r["seed"].as_u64().is_some()
                    && f(r, "muls_harness") > 0.0
                    && f(r, "muls_tuned") > 0.0,
            }
        })
        .collect()
}

fn slope(x: &[f64], y: &[f64]) -> f64 {
    let (lx, ly): (Vec<f64>, Vec<f64>) = (
        x.iter().map(|v| v.ln()).collect(),
        y.iter().map(|v| v.ln()).collect(),
    );
    let (mx, my) = (mean(&lx), mean(&ly));
    lx.iter()
        .zip(&ly)
        .map(|(a, b)| (a - mx) * (b - my))
        .sum::<f64>()
        / lx.iter().map(|a| (a - mx).powi(2)).sum::<f64>()
}
/// Slope with a stratified bootstrap (instances resampled within each size).
fn fit(groups: &[(Vec<f64>, Vec<f64>)], seed: u64) -> Value {
    if groups.len() < 2 || groups.iter().any(|g| g.0.is_empty()) {
        return Value::Null;
    }
    let flat = |g: &[(Vec<f64>, Vec<f64>)]| {
        let (mut x, mut y) = (vec![], vec![]);
        for (gx, gy) in g {
            x.extend(gx);
            y.extend(gy);
        }
        slope(&x, &y)
    };
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = vec![];
    for _ in 0..B {
        let g: Vec<(Vec<f64>, Vec<f64>)> = groups
            .iter()
            .map(|(gx, gy)| {
                let idx: Vec<usize> = (0..gx.len()).map(|_| rng.gen_range(0..gx.len())).collect();
                (
                    idx.iter().map(|&i| gx[i]).collect(),
                    idx.iter().map(|&i| gy[i]).collect(),
                )
            })
            .collect();
        out.push(flat(&g));
    }
    json!({"slope": flat(groups), "ci95": [pct(out.clone(), 0.025), pct(out, 0.975)]})
}
fn fit_parts(v: &Value) -> Option<(f64, f64, f64)> {
    Some((
        v["slope"].as_f64()?,
        v["ci95"][0].as_f64()?,
        v["ci95"][1].as_f64()?,
    ))
}

fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let mut sizes_out: Vec<Value> = vec![];
    let mut fit_ic: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3t: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
    let mut failures: Vec<String> = vec![];
    // acceptance accumulators
    let (mut a1, mut a2, mut a6) = (true, true, true);
    let mut a3_rows = vec![];
    let mut a3 = true;
    let mut aa_out = serde_json::Map::new();
    let (mut a4, mut a5) = (false, false);
    for &p in &SIZES {
        let sd_ = p;
        let load_ic = |name: &str| load(&dir, name).map(|v| ic_rows(&v));
        let sparse = load_ic(&format!("ic_{p}_sparse"));
        let dense = load_ic(&format!("ic_{p}_dense"));
        let mut arms: BTreeMap<String, Vec<Rho>> = BTreeMap::new();
        for a in ["R0", "R1", "R2", "R3", "R0_aa", "R3_aa"] {
            if let Some(v) = load(&dir, &format!("rho_{p}_{a}")) {
                arms.insert(a.into(), rho_rows(&v));
            }
        }
        let mut e = serde_json::Map::new();
        e.insert("p".into(), json!(p));
        let exp16 = expected_seeds(N_SEEDS);
        // ---- A1 for the sieve
        for (nm, rows) in [("sparse", &sparse), ("dense", &dense)] {
            let ok = rows
                .as_ref()
                .map(|r| {
                    inventory_ok(&r.iter().map(|x| x.seed).collect::<Vec<_>>(), &exp16)
                        && r.iter().all(|x| x.ok)
                })
                .unwrap_or(false);
            e.insert(format!("ic_{nm}_inventory_and_validity_ok"), json!(ok));
            if !ok {
                a1 = false;
                failures.push(format!("A1: sieve {nm} sample at p={p}"));
            }
        }
        let Some(ic) = &sparse else {
            sizes_out.push(Value::Object(e));
            continue;
        };
        let l = ic[0].l;
        let sq = l.sqrt();
        e.insert("l".into(), json!(l));
        e.insert("log2_l".into(), json!(l.log2()));
        let seq = |fx: &dyn Fn(&Ic) -> f64| ic.iter().map(|r| (r.seed, fx(r))).collect::<Vec<_>>();
        let vals = |fx: &dyn Fn(&Ic) -> f64| ic.iter().map(fx).collect::<Vec<_>>();
        e.insert("ic_S".into(), mean_ci(&vals(&|r| r.s), sd_ + 1));
        e.insert("ic_S_plus".into(), mean_ci(&vals(&|r| r.s_plus), sd_ + 2));
        e.insert("ic_muls".into(), mean_ci(&vals(&|r| r.muls), sd_ + 3));
        e.insert(
            "ic_wall_s".into(),
            mean_ci(&vals(&|r| r.wall / 1e3), sd_ + 4),
        );
        // ---- A6 and D6: dense against sparse
        if let Some(dn) = &dense {
            let dseq =
                |fx: &dyn Fn(&Ic) -> f64| dn.iter().map(|r| (r.seed, fx(r))).collect::<Vec<_>>();
            let bm: BTreeMap<u64, &Ic> = dn.iter().map(|r| (r.seed, r)).collect();
            let same_digest = ic
                .iter()
                .filter(|r| bm.get(&r.seed).is_some_and(|d| d.digest == r.digest))
                .count();
            let same_out = ic
                .iter()
                .filter(|r| bm.get(&r.seed).is_some_and(|d| d.ok == r.ok))
                .count();
            if same_digest != ic.len() || same_out != ic.len() || ic.len() != 16 {
                a6 = false;
                failures.push(format!("A6: dense/sparse identity at p={p}"));
            }
            e.insert(
                "dense_vs_sparse".into(),
                json!({"pairs": ic.len(), "identical_digest": same_digest, "identical_outcome": same_out,
                       "S_dense_over_sparse": ratio_by_seed(&dseq(&|r| r.s), &seq(&|r| r.s), sd_ + 5),
                       "linear_stage_dense_over_sparse": ratio_by_seed(&dseq(&|r| r.lin), &seq(&|r| r.lin), sd_ + 6),
                       "descent_muls_dense_over_sparse": ratio_by_seed(&dseq(&|r| r.desc), &seq(&|r| r.desc), sd_ + 7)}),
            );
        }
        // ---- A5: sieve A/A at p = 1009
        if p == 1009 {
            let aa = load_ic("ic_1009_sparse_aa");
            let ok = aa
                .as_ref()
                .map(|aa| {
                    let bm: BTreeMap<u64, &Ic> = aa.iter().map(|r| (r.seed, r)).collect();
                    inventory_ok(&aa.iter().map(|x| x.seed).collect::<Vec<_>>(), &exp16)
                        && ic.iter().all(|r| {
                            bm.get(&r.seed)
                                .is_some_and(|d| d.muls == r.muls && d.digest == r.digest)
                        })
                })
                .unwrap_or(false);
            a5 = ok;
            if !ok {
                failures.push("A5: sieve A/A at p=1009".into());
            }
            aa_out.insert("sieve_repeat_identical".into(), json!(ok));
        }
        // ---- rho arms
        let mut rho_out = serde_json::Map::new();
        for (name, rs) in &arms {
            let want = expected_n(p, name);
            let inv = inventory_ok(
                &rs.iter().map(|x| x.seed).collect::<Vec<_>>(),
                &expected_seeds(want),
            );
            let valid = rs.iter().all(|x| x.ok);
            let hash = rs.iter().all(|x| x.hash_ok);
            if !(inv && valid) {
                a1 = false;
                failures.push(format!(
                    "A1: rho {name} at p={p} (inventory {inv}, all valid {valid})"
                ));
            }
            if !hash {
                a2 = false;
                failures.push(format!("A2: rho {name} at p={p} hash scope"));
            }
            let sx =
                |fx: &dyn Fn(&Rho) -> f64| rs.iter().map(|r| (r.seed, fx(r))).collect::<Vec<_>>();
            let sh = sx(&|r| r.mh / (GAE * sq));
            let st = sx(&|r| r.mt / (GAE * sq));
            let sph = sx(&|r| (r.mh + r.fp_adds + r.native) / (GAE * sq));
            let spt = sx(&|r| (r.mt + r.fp_adds + r.native) / (GAE * sq));
            let col = |v: &Vec<(u64, f64)>| v.iter().map(|x| x.1).collect::<Vec<_>>();
            let mut a = serde_json::Map::new();
            a.insert("n".into(), json!(rs.len()));
            a.insert("inventory_ok".into(), json!(inv));
            a.insert("all_runs_correct_uncapped".into(), json!(valid));
            a.insert("hash_scope_ok".into(), json!(hash));
            a.insert(
                "steps_over_sqrt_l".into(),
                mean_ci(
                    &rs.iter().map(|r| r.steps / sq).collect::<Vec<_>>(),
                    sd_ + 10,
                ),
            );
            a.insert("S_harness".into(), mean_ci(&col(&sh), sd_ + 11));
            a.insert("S_tuned".into(), mean_ci(&col(&st), sd_ + 12));
            a.insert("S_plus_harness".into(), mean_ci(&col(&sph), sd_ + 13));
            a.insert("S_plus_tuned".into(), mean_ci(&col(&spt), sd_ + 14));
            a.insert(
                "muls_harness".into(),
                mean_ci(&rs.iter().map(|r| r.mh).collect::<Vec<_>>(), sd_ + 15),
            );
            a.insert(
                "wall_s".into(),
                mean_ci(
                    &rs.iter().map(|r| r.wall / 1e3).collect::<Vec<_>>(),
                    sd_ + 16,
                ),
            );
            // sieve over this arm, paired by seed identity
            a.insert(
                "ic_over_rho".into(),
                json!({"S_harness": ratio_by_seed(&seq(&|r| r.s), &sh, sd_ + 20),
                       "S_tuned": ratio_by_seed(&seq(&|r| r.s), &st, sd_ + 21),
                       "S_plus_harness": ratio_by_seed(&seq(&|r| r.s_plus), &sph, sd_ + 22),
                       "S_plus_tuned": ratio_by_seed(&seq(&|r| r.s_plus), &spt, sd_ + 23)}),
            );
            let mut br = vec![];
            for &w in &BAND {
                let ia = seq(&|r| {
                    (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds + r.unemitted)) / sq
                });
                let ib = seq(&|r| (r.muls / GAE + w * r.lookups) / sq);
                let ra = sx(&|r| (r.mh / GAE + w * (r.native + r.fp_adds)) / sq);
                let rat = sx(&|r| (r.mt / GAE + w * (r.native + r.fp_adds)) / sq);
                let rb = sx(&|r| (r.mh / GAE + w * r.native) / sq);
                let rbt = sx(&|r| (r.mt / GAE + w * r.native) / sq);
                br.push(json!({"w": w,
                    "A_harness": ratio_by_seed(&ia, &ra, sd_ + 30), "A_tuned": ratio_by_seed(&ia, &rat, sd_ + 31),
                    "B_harness": ratio_by_seed(&ib, &rb, sd_ + 32), "B_tuned": ratio_by_seed(&ib, &rbt, sd_ + 33)}));
            }
            a.insert("band_ratio_ic_over_rho".into(), json!(br));
            // ---- A3 positive control
            if name == "R0" {
                let m = &a["S_harness"];
                let (pt, lo, hi) = (
                    m["mean"].as_f64().unwrap_or(f64::NAN),
                    m["ci95"][0].as_f64().unwrap_or(f64::NAN),
                    m["ci95"][1].as_f64().unwrap_or(f64::NAN),
                );
                let ok = a3_ok(pt, lo, hi);
                a3 &= ok;
                a3_rows.push(json!({"p": p, "mean": pt, "ci95": [lo, hi], "pass": ok}));
            }
            // fit data for the strong rho
            if name == "R3" && valid {
                fit_r3.push((
                    p,
                    rs.iter().map(|r| r.l).collect(),
                    rs.iter().map(|r| r.mh).collect(),
                ));
                fit_r3t.push((
                    p,
                    rs.iter().map(|r| r.l).collect(),
                    rs.iter().map(|r| r.mt).collect(),
                ));
            }
            rho_out.insert(name.clone(), Value::Object(a));
        }
        // ---- A4: rho A/A at p = 1009 (first run over repeat, paired by seed)
        if p == 1009 {
            let mut all = true;
            for (x, y) in [("R0", "R0_aa"), ("R3", "R3_aa")] {
                let (Some(rx), Some(ry)) = (arms.get(x), arms.get(y)) else {
                    all = false;
                    continue;
                };
                let a: Vec<(u64, f64)> = rx.iter().map(|r| (r.seed, r.mh / (GAE * sq))).collect();
                let b: Vec<(u64, f64)> = ry.iter().map(|r| (r.seed, r.mh / (GAE * sq))).collect();
                let rc = ratio_by_seed(&a, &b, sd_ + 40);
                let (r, lo, hi) = rl(&rc);
                let ok = rc["n"] == json!(16) && a4_ok(r, lo, hi);
                all &= ok;
                aa_out.insert(
                    format!("rho_{x}"),
                    json!({"ratio_first_over_repeat": rc, "pass": ok}),
                );
            }
            a4 = all;
            if !all {
                failures.push("A4: rho A/A at p=1009".into());
            }
        }
        e.insert("rho".into(), Value::Object(rho_out));
        fit_ic.push((
            p,
            ic.iter().map(|r| r.l).collect(),
            ic.iter().map(|r| r.muls).collect(),
        ));
        sizes_out.push(Value::Object(e));
    }
    // ---- fits and A7
    let groups = |g: &Vec<(u64, Vec<f64>, Vec<f64>)>, min_p: u64| -> Vec<(Vec<f64>, Vec<f64>)> {
        g.iter()
            .filter(|x| x.0 >= min_p)
            .map(|x| (x.1.clone(), x.2.clone()))
            .collect()
    };
    let f1 = fit(&groups(&fit_ic, 0), 77);
    let f2 = fit(&groups(&fit_ic, 1009), 80);
    let r3fit = fit(&groups(&fit_r3, 0), 78);
    let r3tfit = fit(&groups(&fit_r3t, 0), 79);
    let a7 = fit_parts(&r3fit).is_some_and(|(s, lo, hi)| a7_ok(s, lo, hi));
    if !a3 {
        failures.push("A3: positive control".into());
    }
    if !a7 {
        failures.push("A7: instrument exponent".into());
    }
    let acc = [
        ("A1", a1),
        ("A2", a2),
        ("A3", a3),
        ("A4", a4),
        ("A5", a5),
        ("A6", a6),
        ("A7", a7),
    ];
    let all_pass = acc.iter().all(|x| x.1);
    let acceptance = json!({
        "A1_samples_complete_and_valid": a1, "A2_hash_scope_complete": a2,
        "A3_positive_control": {"pass": a3, "rows": a3_rows},
        "A4_rho_aa_and_A5_sieve_aa": Value::Object(aa_out.clone()),
        "A4_pass": a4, "A5_pass": a5, "A6_dense_sparse_identical": a6,
        "A7_instrument_exponent": {"pass": a7, "fit": r3fit.clone()},
        "all_pass": all_pass, "failed": failures});

    // ---- decisions, only if every acceptance rule passed
    let decisions = if all_pass {
        let mut d1 = vec![];
        let mut d2 = vec![];
        let mut d4s = None;
        let mut d4sp = None;
        for e in &sizes_out {
            let p = e["p"].as_u64().unwrap();
            let r3 = &e["rho"]["R3"];
            let (_, _, sh_hi) = rl(&r3["ic_over_rho"]["S_harness"]);
            let (_, _, sph_hi) = rl(&r3["ic_over_rho"]["S_plus_harness"]);
            let (sh_lo, _) = (rl(&r3["ic_over_rho"]["S_harness"]).1, 0);
            if d4s.is_none() && sh_hi < 1.0 && sh_lo.is_finite() {
                d4s = Some(p);
            }
            if d4sp.is_none() && sph_hi < 1.0 {
                d4sp = Some(p);
            }
            if p >= 1009 {
                let (_, lo, hi) = rl(&r3["ic_over_rho"]["S_plus_harness"]);
                d1.push(json!({"p": p, "ci95": [lo, hi], "result": if hi < 1.0 {"sieve below strong rho"} else {"K1 fires"}}));
                let (_, tlo, thi) = rl(&r3["ic_over_rho"]["S_plus_tuned"]);
                let mut below = thi < 1.0;
                let mut cols = vec![verdict(tlo, thi)];
                for b in r3["band_ratio_ic_over_rho"].as_array().unwrap() {
                    for k in ["A_harness", "A_tuned", "B_harness", "B_tuned"] {
                        let (_, lo, hi) = rl(&b[k]);
                        below &= hi < 1.0;
                        cols.push(verdict(lo, hi));
                    }
                }
                d2.push(json!({"p": p, "ranked_below_at_every_weight": below,
                               "unresolved_columns": cols.iter().filter(|c| **c == "unresolved").count()}));
            }
        }
        let cls = |v: &Value| {
            fit_parts(v)
                .map(|(_, lo, hi)| classify_fit(lo, hi))
                .unwrap_or("unavailable")
        };
        let at1009 = |a: &str, b: &str| -> Value {
            let e = sizes_out.iter().find(|e| e["p"] == json!(1009)).unwrap();
            let seq = |arm: &str| -> Vec<(u64, f64)> {
                // reconstruct from the arm's raw rows
                load(&dir, &format!("rho_1009_{arm}"))
                    .map(|v| rho_rows(&v))
                    .unwrap_or_default()
                    .iter()
                    .map(|r| (r.seed, r.mh / (GAE * e["l"].as_f64().unwrap().sqrt())))
                    .collect()
            };
            ratio_by_seed(&seq(a), &seq(b), 1009 + 50)
        };
        let tuned1009 = |a: &str, b: &str| -> Value {
            let e = sizes_out.iter().find(|e| e["p"] == json!(1009)).unwrap();
            let seq = |arm: &str| -> Vec<(u64, f64)> {
                load(&dir, &format!("rho_1009_{arm}"))
                    .map(|v| rho_rows(&v))
                    .unwrap_or_default()
                    .iter()
                    .map(|r| (r.seed, r.mt / (GAE * e["l"].as_f64().unwrap().sqrt())))
                    .collect()
            };
            ratio_by_seed(&seq(a), &seq(b), 1009 + 51)
        };
        let (r02, r01, r23, t32) = (
            at1009("R0", "R2"),
            at1009("R0", "R1"),
            at1009("R2", "R3"),
            tuned1009("R3", "R2"),
        );
        let contains1 = |v: &Value| {
            let (_, lo, hi) = rl(v);
            lo <= 1.0 && 1.0 <= hi
        };
        let mut d6 = vec![];
        for e in &sizes_out {
            if let Some(d) = e.get("dense_vs_sparse") {
                let (r, lo, hi) = rl(&d["S_dense_over_sparse"]);
                d6.push(json!({"p": e["p"], "ratio": r, "ci95": [lo, hi], "gain": lo > 1.0}));
            }
        }
        json!({"status": "evaluated",
               "D1_K1": d1, "D2_K2": d2,
               "D3_exponent": {"F1_all_sizes": {"fit": f1.clone(), "class": cls(&f1), "supports_l13": cls(&f1) == "l^(1/3) supported"},
                                "F2_p_ge_1009": {"fit": f2.clone(), "class": cls(&f2), "claim_cost_grows_as_l13": cls(&f2) == "l^(1/3) supported"}},
               "D4_crossover_first_p_below_one": {"S_harness": d4s, "S_plus_harness": d4sp},
               "D5": {"negation_buys_R0_over_R2": {"ratio": r02.clone(), "buys": rl(&r02).1 > 1.0},
                       "batching_buys_nothing_harness": {"R0_over_R1": r01.clone(), "R2_over_R3": r23.clone(), "holds": contains1(&r01) && contains1(&r23)},
                       "batching_costs_tuned_R3_over_R2": {"ratio": t32.clone(), "costs": rl(&t32).1 > 1.0}},
               "D6_sparse_fglm": d6})
    } else {
        json!({"status": "withheld", "reason": "acceptance rules A1-A7 did not all pass", "failed": acceptance["failed"].clone()})
    };

    let mut summary = serde_json::Map::new();
    summary.insert("protocol".into(), json!("PROTOCOL.md (V2)"));
    summary.insert(
        "seeds".into(),
        json!([FIRST_SEED, FIRST_SEED + N_SEEDS - 1]),
    );
    summary.insert("acceptance".into(), acceptance);
    summary.insert("decisions".into(), decisions);
    summary.insert("sizes".into(), json!(sizes_out));
    summary.insert("exponent_F1_all_sizes_sieve".into(), f1);
    summary.insert("exponent_F2_p_ge_1009_sieve".into(), f2);
    summary.insert("exponent_rho_R3_harness".into(), r3fit);
    summary.insert("exponent_rho_R3_tuned".into(), r3tfit);
    summary.insert(
        "provenance".into(),
        fs::read_to_string(format!("{dir}/raw/PROVENANCE.json"))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(Value::Null),
    );
    summary.insert("gae_f_p_mults".into(), json!(GAE));
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&Value::Object(summary)).unwrap(),
    )
    .unwrap();
    println!("wrote {dir}/summary.json; acceptance all_pass = {all_pass}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_scope_uses_the_protocol_windows() {
        assert!(hash_scope_ok(false, 2.0e6, 1.0e6));
        assert!(!hash_scope_ok(false, 3.0e6, 1.0e6));
        assert!(hash_scope_ok(true, 4.0e6, 1.0e6));
        assert!(hash_scope_ok(true, 3.9e6, 1.0e6));
        assert!(!hash_scope_ok(true, 1.0e6, 1.0e6)); // the V1 producer's recorded scope
        assert!(!hash_scope_ok(true, 4.0, 0.0));
    }

    #[test]
    fn positive_control_needs_the_interval_and_the_point() {
        assert!(a3_ok(1.3, 1.0, 1.6));
        assert!(!a3_ok(1.3, 1.35, 1.6)); // interval misses the ledger value
        assert!(!a3_ok(1.7, 1.2, 2.0)); // point outside [1.0, 1.6]
        assert!(!a3_ok(0.9, 0.8, 1.4));
    }

    #[test]
    fn aa_needs_a_ratio_near_one_and_an_interval_containing_one() {
        assert!(a4_ok(0.97, 0.8, 1.2));
        assert!(!a4_ok(0.80, 0.7, 1.1));
        assert!(!a4_ok(1.10, 1.02, 1.2)); // interval excludes 1
        assert!(!a4_ok(1.20, 0.9, 1.4));
    }

    #[test]
    fn instrument_exponent_must_be_near_one_half() {
        assert!(a7_ok(0.508, 0.480, 0.538));
        assert!(!a7_ok(0.44, 0.40, 0.55));
        assert!(!a7_ok(0.52, 0.51, 0.56)); // interval misses 0.5
    }

    #[test]
    fn exponent_classes_follow_the_protocol() {
        assert_eq!(classify_fit(0.2, 0.5), "l^(1/3) supported");
        assert_eq!(classify_fit(-0.05, 0.08), "flat");
        assert_eq!(classify_fit(-0.3, -0.15), "falling");
        // an interval straddling the -0.1 edge is neither "flat" nor "falling": "other"
        assert_eq!(classify_fit(-0.165, -0.084), "other");
        assert_eq!(classify_fit(0.4, 0.7), "other");
        assert_eq!(classify_fit(-0.2, 0.05), "other");
    }

    #[test]
    fn verdicts_never_name_a_winner_inside_an_interval() {
        assert_eq!(verdict(0.2, 0.9), "IC<rho");
        assert_eq!(verdict(1.1, 3.0), "rho<IC");
        assert_eq!(verdict(0.8, 1.2), "unresolved");
        assert_eq!(verdict(f64::NAN, 1.0), "unavailable");
    }

    #[test]
    fn inventories_reject_missing_duplicate_and_extra_seeds() {
        let exp = expected_seeds(16);
        assert!(inventory_ok(&exp, &exp));
        let mut missing = exp.clone();
        missing.pop();
        assert!(!inventory_ok(&missing, &exp));
        let mut dup = exp.clone();
        dup[3] = dup[2];
        assert!(!inventory_ok(&dup, &exp));
        let mut extra = exp.clone();
        extra.push(99);
        assert!(!inventory_ok(&extra, &exp));
        assert_eq!(exp[0], 17);
        assert_eq!(*exp.last().unwrap(), 32);
    }

    #[test]
    fn pairing_is_by_seed_identity_not_position() {
        let a = vec![(17, 2.0), (18, 4.0), (19, 6.0)];
        let b = vec![(19, 3.0), (17, 1.0), (18, 2.0)];
        let r = ratio_by_seed(&a, &b, 1);
        assert_eq!(r["n"], 3);
        assert!((r["ratio"].as_f64().unwrap() - 2.0).abs() < 1e-12);
        // a seed present on one side only is not paired
        let c = vec![(17, 1.0), (40, 9.0)];
        assert_eq!(ratio_by_seed(&a, &c, 1)["n"], 1);
    }

    #[test]
    fn frozen_sample_sizes_match_the_protocol() {
        assert_eq!(expected_n(1823, "R0"), 4);
        assert_eq!(expected_n(1511, "R0"), 8);
        assert_eq!(expected_n(1823, "R3"), 16);
        assert_eq!(expected_n(1823, "R1"), 0);
        assert_eq!(expected_n(1009, "R3_aa"), 16);
        assert_eq!(expected_n(503, "R0_aa"), 0);
    }
}
