//! Counted S0 and d of the compact-orbit Koblitz producer
//! (`examples/koblitz_orbit_dlp_fast_online.rs`), the AGENTS.md section 5a
//! band, and the many-target gap G against batch rho.
//!
//!     descost <runs_dir> <n83_record_dir> <out_dir>
//!
//! Reads `runs/n*_T*.stdout` (summary line) and `.jsonl` (one record per
//! target) of this study, plus the recorded n = 83 K600/K3300 files, writes
//! `results.json` and `tables.md`.  Python in this directory only plots
//! `results.json`.
//!
//! Units.  One GAE = one batched affine addition on the same field, priced as
//! 5 field multiplications (3 for the shared inversion, 2 in the addition;
//! squaring is linear in characteristic 2 and the 1/lanes inversion share is
//! dropped).  A root call (one S3 solve) is priced from its multiplication
//! count in the producer's code:
//!   u64 path (n <= 63), unbatched: 8 M + one Itoh-Tsujii inversion,
//!   wide path (n >= 71), inversions batched over 2n inputs: 11 M + I / 2n
//!   (both roots always exist here, so the post-trace multiplication always runs).
//! Probes (canonical rotation + hash lookup) and table inserts are unpriced
//! native work at w in {0, 0.1, 1} GAE, like rho's canonicalisations.
//! Variant B prices w on both arms (the section 5a rule); variant A on rho
//! only (IC-favourable sensitivity).  A relation check is 3 unbatched
//! additions (I + 2 M each).

use serde_json::{json, Value};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const GAE_M: f64 = 5.0;
const WS: [f64; 3] = [0.0, 0.1, 1.0];
const BOOT: usize = 400;

fn subgroup_order(n: u32) -> f64 {
    // Largest prime factor of #E(F_2^n) (a = 0 for n <= 73, a = 1 at n = 83),
    // factored offline; see README.
    match n {
        41 => 549_756_390_943.0,
        53 => 21_044_858_204_113.0,
        61 => 162_888_033_982_417.0,
        73 => 86_020_738_150_056_119.0,
        83 => 8_569_786_107_849_059.0,
        _ => panic!("no subgroup order for n = {n}"),
    }
}

fn it_mults(n: u32) -> f64 {
    let e = n - 1;
    let bits = 32 - e.leading_zeros();
    let set: u32 = (0..bits - 1).map(|i| (e >> i) & 1).sum();
    ((bits - 1) + set) as f64
}

fn root_call_gae(n: u32) -> f64 {
    if n <= 63 {
        (8.0 + it_mults(n)) / GAE_M
    } else {
        batched_root_call_gae(n)
    }
}

fn batched_root_call_gae(n: u32) -> f64 {
    (11.0 + it_mults(n) / (2.0 * n as f64)) / GAE_M
}

fn lift_gae(n: u32) -> f64 {
    3.0 * (it_mults(n) + 2.0) / GAE_M
}

fn lgamma(x: f64) -> f64 {
    // Stirling series, x >= 1 (k >= 1 here).
    if x < 7.0 {
        return lgamma(x + 1.0) - x.ln();
    }
    let x2 = x * x;
    (x - 0.5) * x.ln() - x + 0.5 * (2.0 * std::f64::consts::PI).ln() + 1.0 / (12.0 * x)
        - 1.0 / (360.0 * x * x2)
        + 1.0 / (1260.0 * x2 * x2 * x)
}

/// Kuhn-Struik batch law K(k) = 2k C(2k,k) / 4^k.
fn batch_k(k: f64) -> f64 {
    2.0 * k * (lgamma(2.0 * k + 1.0) - 2.0 * lgamma(k + 1.0) - 2.0 * k * 2f64.ln()).exp()
}

/// min over k of (S0 + k d) / (rho1 K(k)), on a 5% geometric grid.
fn best_g(s0: f64, d: f64, rho1: f64) -> (f64, u64) {
    let mut best = (f64::INFINITY, 0u64);
    let mut k = 1.0f64;
    while k < 1e12 {
        let kk = k.floor();
        let g = (s0 + kk * d) / (rho1 * batch_k(kk));
        if g < best.0 {
            best = (g, kk as u64);
        }
        k *= 1.05;
    }
    best
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn f(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(Value::as_f64)
}

fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn median(xs: &[f64]) -> f64 {
    let mut s = xs.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = s.len() / 2;
    if s.len() % 2 == 1 { s[m] } else { 0.5 * (s[m - 1] + s[m]) }
}

struct Run {
    n: u32,
    recipe: String,
    summary: Value,
    records: Vec<Value>,
}

/// Counted S0 and per-target d (GAE) at probe/insert weight `w`, root-call price `cr`.
fn costs(run: &Run, cr: f64, w: f64) -> (f64, Vec<f64>, Value) {
    let s = &run.summary;
    let lg = lift_gae(run.n);
    let build = f(s, "index_pair_candidates").or_else(|| f(s, "regular_states")).unwrap();
    let inserts = f(s, "root_table_entries").unwrap_or(0.0);
    let relations = f(s, "rank_relations").unwrap();
    let rank_probes = f(s, "rank_probes_total").unwrap_or_else(|| f(s, "rank_probes_mean").unwrap() * relations);
    // The K3300 record has no root-call total: both roots of a target-side S3
    // always exist (U and Q are points), so probes = 2 root calls (checked on K600).
    let rank_rc = f(s, "rank_root_calls_total").unwrap_or(rank_probes / 2.0);
    let s0 = build * cr + inserts * w + rank_rc * cr + rank_probes * w + relations * lg;
    let rc = |r: &Value| f(r, "root_calls").unwrap_or_else(|| f(r, "probes").unwrap() / 2.0);
    let ds: Vec<f64> = run
        .records
        .iter()
        .map(|r| rc(r) * cr + f(r, "probes").unwrap() * w + f(r, "relation_checks").unwrap_or(1.0) * lg)
        .collect();
    let parts = json!({
        "build_root_calls": build, "inserts": inserts, "rank_root_calls": rank_rc, "rank_probes": rank_probes,
        "rank_relations": relations,
        "target_root_calls_mean": mean(&run.records.iter().map(rc).collect::<Vec<_>>()),
        "target_probes_mean": mean(&run.records.iter().map(|r| f(r, "probes").unwrap()).collect::<Vec<_>>()),
    });
    (s0, ds, parts)
}

/// Single-target rho: (additions, canonicalisations) per walk and its source.
fn rho_counts(n: u32, n83: &Path) -> (f64, f64, String) {
    if n == 83 {
        let dir = n83.join("k3300/rho_plain");
        let mut adds = Vec::new();
        let mut canon = Vec::new();
        for entry in fs::read_dir(&dir).unwrap() {
            let p = entry.unwrap().path();
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            if name.starts_with('w') && name.ends_with(".json") && !name.contains("point") {
                let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
                let c = &v["outcome"]["charges_setup_and_solve"];
                adds.push(c["group_additions"].as_f64().unwrap());
                canon.push(c["canonicalizations"].as_f64().unwrap());
            }
        }
        return (mean(&adds), mean(&canon), format!("measured, {} plain-rho walks (same records)", adds.len()));
    }
    let np = subgroup_order(n) / (2.0 * n as f64);
    let floor = (std::f64::consts::PI * np / 2.0).sqrt();
    (floor, floor, "floor sqrt(pi N'/2), N' = r/2n, one canonicalisation per addition (repo counted/floor 1.02-1.08)".into())
}

fn parse_tag(name: &str) -> Option<(u32, u32, u32, u32, u32)> {
    // n<n>_K<K>_N<N>_T<T>_s<sym>_r<seed>.stdout
    let stem = name.strip_suffix(".stdout")?;
    let mut it = stem.split('_');
    let n = it.next()?.strip_prefix('n')?.parse().ok()?;
    let k = it.next()?.strip_prefix('K')?.parse().ok()?;
    let nn = it.next()?.strip_prefix('N')?.parse().ok()?;
    let t = it.next()?.strip_prefix('T')?.parse().ok()?;
    let s = it.next()?.strip_prefix('s')?.parse().ok()?;
    Some((n, k, nn, t, s))
}

fn load_runs(runs_dir: &Path, n83: &Path) -> Vec<Run> {
    let mut names: Vec<String> = fs::read_dir(runs_dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".stdout"))
        .collect();
    names.sort();
    let mut runs = Vec::new();
    for name in names {
        let Some((n, k, nn, _t, sym)) = parse_tag(&name) else { continue };
        let text = fs::read_to_string(runs_dir.join(&name)).unwrap();
        let Some(line) = text.lines().filter(|l| l.starts_with('{')).last() else { continue };
        let summary: Value = serde_json::from_str(line).unwrap();
        let records: Vec<Value> = fs::read_to_string(runs_dir.join(name.replace(".stdout", ".jsonl")))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(summary["targets_solved"], summary["targets"], "{name}");
        assert_eq!(records.len() as u64, summary["targets"].as_u64().unwrap(), "{name}");
        assert!(records.iter().all(|r| r["recovered_matches_published"] != json!(false)), "{name}");
        let recipe = format!("K{k} norm{nn}{}", if sym == 1 { " sym" } else { "" });
        runs.push(Run { n, recipe, summary, records });
    }
    for (k, label) in [("k600", "K600 (recorded, a=1)"), ("k3300", "K3300 (recorded, a=1, sym)")] {
        let summary: Value = serde_json::from_str(fs::read_to_string(n83.join(k).join("ic/stdout.log")).unwrap().trim()).unwrap();
        let records = fs::read_to_string(n83.join(k).join("ic/records.jsonl"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        runs.push(Run { n: 83, recipe: label.into(), summary, records });
    }
    runs
}

fn analyse(run: &Run, n83: &Path, rng: &mut SplitMix) -> Value {
    let n = run.n;
    let cr = root_call_gae(n);
    let (ga, ca, rho_src) = rho_counts(n, n83);
    let np = subgroup_order(n) / (2.0 * n as f64);
    let mut bands = serde_json::Map::new();
    let mut parts = Value::Null;
    for variant in ["A", "B"] {
        for w in WS {
            let (s0, ds, p) = costs(run, cr, if variant == "B" { w } else { 0.0 });
            parts = p;
            let rho1 = ga + w * ca;
            let d = mean(&ds);
            let (g, kopt) = best_g(s0, d, rho1);
            let (mut lo, mut hi) = (g, g);
            if (variant == "A" && w == 0.0) || (variant == "B" && w == 1.0) {
                let mut boots: Vec<f64> = (0..BOOT)
                    .map(|_| {
                        let db = mean(&(0..ds.len()).map(|_| ds[(rng.next() % ds.len() as u64) as usize]).collect::<Vec<_>>());
                        best_g(s0, db, rho1).0
                    })
                    .collect();
                boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
                lo = boots[BOOT / 40];
                hi = boots[BOOT - 1 - BOOT / 40];
            }
            bands.insert(
                format!("{variant}{w:?}"),
                json!({"S0": s0, "d": d, "rho1": rho1, "sigma": s0 / rho1, "delta": d / rho1,
                       "sigma_delta": s0 * d / (rho1 * rho1), "G": g, "G_lo": lo, "G_hi": hi, "k_opt": kopt,
                       "S0d_over_Nprime": s0 * d / np, "cold_S": s0 + d}),
            );
        }
    }
    // IC-favourable sensitivity: root call = 1 GAE, w = 0.
    let (s0u, dsu, _) = costs(run, 1.0, 0.0);
    let unit_lb = json!({"S0": s0u, "d": mean(&dsu), "G": best_g(s0u, mean(&dsu), ga).0, "S0d_over_Nprime": s0u * mean(&dsu) / np});
    // Root-call price at which G = 1 under variant A, w = 1.
    let (mut lo, mut hi) = (0.01f64, 100.0f64);
    for _ in 0..60 {
        let mid = (lo * hi).sqrt();
        let (s0, ds, _) = costs(run, mid, 0.0);
        if best_g(s0, mean(&ds), ga + ca).0 < 1.0 { lo = mid } else { hi = mid }
    }
    let recs = &run.records;
    let col = |key: &str| recs.iter().map(|r| f(r, key).unwrap_or(0.0)).collect::<Vec<_>>();
    let probes = col("probes");
    let rcs: Vec<f64> = recs.iter().map(|r| f(r, "root_calls").unwrap_or_else(|| f(r, "probes").unwrap() / 2.0)).collect();
    let mut prof = json!({
        "root_calls": mean(&rcs), "probes": mean(&probes), "probes_median": median(&probes),
        "probes_max": probes.iter().cloned().fold(0.0, f64::max),
        "relation_checks": mean(&recs.iter().map(|r| f(r, "relation_checks").unwrap_or(1.0)).collect::<Vec<_>>()),
        "skipped_symmetric_states": mean(&col("skipped_symmetric_states")),
    });
    for key in ["target_query_ms", "target_pdp_ms", "target_relation_check_ms", "target_descent_ms", "target_recovery_check_ms", "target_ms"] {
        prof[key] = json!(mean(&col(key)));
    }
    json!({"n": n, "recipe": run.recipe, "root_call_gae": cr, "rho_source": rho_src, "targets": recs.len(),
           "N_prime": np, "bands": bands, "parts": parts, "unit_lb": unit_lb, "root_call_gae_for_G1_at_A1": hi,
           "timing_ms": run.summary.get("timing_ms").cloned().unwrap_or(Value::Null), "d_profile": prof})
}

fn baseline(n: u32) -> &'static str {
    // Cold (k = 1) tuned KIC_NORM recipes of research/ic_improve_20261023; n = 83 the recorded K600.
    match n {
        41 => "K12 norm16",
        53 => "K20 norm16",
        61 => "K18 norm32",
        73 => "K20 norm32",
        _ => "K600 (recorded, a=1)",
    }
}

fn tables(rows: &[Value]) -> String {
    let b = |r: &Value, t: &str, k: &str| r["bands"][t][k].as_f64().unwrap();
    let find = |n: u64, recipe: &str| rows.iter().find(|r| r["n"].as_u64() == Some(n) && r["recipe"] == recipe);
    let mut o = String::new();
    writeln!(o, "### Table 1: where d goes (per target, mean over the run's targets)\n").unwrap();
    writeln!(o, "| n | recipe | targets | root calls | probes (mean / median / max) | relation checks | skipped mate states | PDP ms | relation-check ms | log combination ms | recovery check ms | PDP share of target ms |").unwrap();
    writeln!(o, "|--:|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for r in rows {
        let p = &r["d_profile"];
        let g = |k: &str| p[k].as_f64().unwrap();
        writeln!(o, "| {} | {} | {} | {:.0} | {:.0} / {:.0} / {:.0} | {:.2} | {:.1} | {:.3} | {:.4} | {:.5} | {:.4} | {:.1}% |",
            r["n"], r["recipe"].as_str().unwrap(), r["targets"], g("root_calls"), g("probes"), g("probes_median"), g("probes_max"),
            g("relation_checks"), g("skipped_symmetric_states"), g("target_pdp_ms"), g("target_relation_check_ms"), g("target_descent_ms"),
            g("target_recovery_check_ms"), 100.0 * g("target_pdp_ms") / g("target_ms")).unwrap();
    }
    writeln!(o, "\n### Table 2: counted S0 and d, every run (GAE; root call priced in field multiplications; w = 0 unless named)\n").unwrap();
    writeln!(o, "| n | recipe | root call [GAE] | build root calls | rank root calls | S0 | d | S0*d/N' | sigma*delta | G, w = 0 [95%] | G, B w = 0.1 | G, B w = 1 | G, A w = 1 | G, root call = 1 GAE (IC-favourable) | root-call price for G = 1 at A w = 1 | build x d root calls / N' |").unwrap();
    writeln!(o, "|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|").unwrap();
    for r in rows {
        let p = &r["parts"];
        writeln!(o, "| {} | {} | {:.2} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.2} | {:.2} | {:.2} [{:.2}, {:.2}] | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.3} |",
            r["n"], r["recipe"].as_str().unwrap(), r["root_call_gae"].as_f64().unwrap(), p["build_root_calls"].as_f64().unwrap(),
            p["rank_root_calls"].as_f64().unwrap(), b(r, "B0.0", "S0"), b(r, "B0.0", "d"), b(r, "B0.0", "S0d_over_Nprime"),
            b(r, "B0.0", "sigma_delta"), b(r, "A0.0", "G"), b(r, "A0.0", "G_lo"), b(r, "A0.0", "G_hi"), b(r, "B0.1", "G"),
            b(r, "B1.0", "G"), b(r, "A1.0", "G"), r["unit_lb"]["G"].as_f64().unwrap(), r["root_call_gae_for_G1_at_A1"].as_f64().unwrap(),
            p["build_root_calls"].as_f64().unwrap() * p["target_root_calls_mean"].as_f64().unwrap() / r["N_prime"].as_f64().unwrap()).unwrap();
    }
    writeln!(o, "\n### Table 3: reductions of S0*d against the cold-tuned KIC_NORM recipe (same producer, same unit, w = 0)\n").unwrap();
    writeln!(o, "| n | baseline S0*d/N' | best retuned K, plain build | pair symmetry, same K | best retuned K + pair symmetry (measured) | with batched root calls (count projection, not run) | G: baseline -> best measured -> projected |").unwrap();
    writeln!(o, "|--:|--:|:--|:--|:--|:--|:--|").unwrap();
    let mut sizes: Vec<u64> = rows.iter().filter_map(|r| r["n"].as_u64()).filter(|&n| n < 83).collect();
    sizes.dedup();
    for n in sizes {
        let Some(base) = find(n, baseline(n as u32)) else { continue };
        let of = |r: &Value| b(r, "B0.0", "S0d_over_Nprime");
        let bs = of(base);
        let pick = |sym: bool| {
            rows.iter()
                .filter(|r| r["n"].as_u64() == Some(n) && r["recipe"].as_str().unwrap().contains(" sym") == sym && !r["recipe"].as_str().unwrap().contains("norm0"))
                .min_by(|x, y| of(x).partial_cmp(&of(y)).unwrap())
        };
        let bp = pick(false).unwrap();
        let same = find(n, &format!("{} sym", baseline(n as u32)));
        let same_s = same.map(|s| format!("{:.2} ({:.2}x)", of(s), bs / of(s))).unwrap_or("not run".into());
        match pick(true) {
            Some(bb) => {
                let cr = root_call_gae(n as u32);
                let crb = batched_root_call_gae(n as u32);
                let proj = of(bb) * (crb / cr).powi(2);
                writeln!(o, "| {n} | {bs:.2} | {}: {:.2} ({:.2}x) | {same_s} | {}: {:.2} ({:.2}x) | {proj:.2} ({:.2}x; root call {cr:.2} -> {crb:.2} GAE) | {:.2} -> {:.2} -> {:.2} |",
                    bp["recipe"].as_str().unwrap(), of(bp), bs / of(bp), bb["recipe"].as_str().unwrap(), of(bb), bs / of(bb), bs / proj,
                    b(base, "A0.0", "G"), b(bb, "A0.0", "G"), b(bb, "A0.0", "G") * crb / cr).unwrap();
            }
            None => {
                writeln!(o, "| {n} | {bs:.2} | {}: {:.2} ({:.2}x) | {same_s} | not run | n/a (wide path already batched) | {:.2} -> {:.2} |",
                    bp["recipe"].as_str().unwrap(), of(bp), bs / of(bp), b(base, "A0.0", "G"), b(bp, "A0.0", "G")).unwrap();
            }
        }
    }
    writeln!(o, "\n### Table 4: single-target (k = 1) cold cost S = S0 + d, GAE\n").unwrap();
    writeln!(o, "| n | baseline recipe | baseline S, w = 0 | best measured recipe | best S, w = 0 | gain, w = 0 | gain, B w = 1 |").unwrap();
    writeln!(o, "|--:|:--|--:|:--|--:|--:|--:|").unwrap();
    let mut sizes: Vec<u64> = rows.iter().filter_map(|r| r["n"].as_u64()).collect();
    sizes.dedup();
    for n in sizes {
        let Some(base) = find(n, baseline(n as u32)) else { continue };
        let best = rows.iter().filter(|r| r["n"].as_u64() == Some(n))
            .min_by(|x, y| b(x, "B0.0", "cold_S").partial_cmp(&b(y, "B0.0", "cold_S")).unwrap()).unwrap();
        writeln!(o, "| {n} | {} | {:.3e} | {} | {:.3e} | {:.2}x | {:.2}x |", base["recipe"].as_str().unwrap(), b(base, "B0.0", "cold_S"),
            best["recipe"].as_str().unwrap(), b(best, "B0.0", "cold_S"), b(base, "B0.0", "cold_S") / b(best, "B0.0", "cold_S"),
            b(base, "B1.0", "cold_S") / b(best, "B1.0", "cold_S")).unwrap();
    }
    o
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4, "usage: descost <runs_dir> <n83_record_dir> <out_dir>");
    let (runs_dir, n83, out) = (Path::new(&args[1]), Path::new(&args[2]), Path::new(&args[3]));
    let mut rng = SplitMix(20261030);
    let runs = load_runs(runs_dir, n83);
    let rows: Vec<Value> = runs.iter().map(|r| analyse(r, n83, &mut rng)).collect();
    fs::write(out.join("results.json"), serde_json::to_string_pretty(&rows).unwrap()).unwrap();
    let t = tables(&rows);
    fs::write(out.join("tables.md"), &t).unwrap();
    print!("{t}");
    for n in [41u32, 53, 61, 73, 83] {
        eprintln!("n = {n}: Itoh-Tsujii mults {}, root call {:.2} GAE (batched {:.2}), relation check {:.2} GAE",
            it_mults(n), root_call_gae(n), batched_root_call_gae(n), lift_gae(n));
    }
}
