//! Null distribution of ω(D_K) (class-group 2-rank + 1) for prime-order curves.
//!
//! Protocol: `research/p224_two_rank_null_20261022/PROTOCOL.md`.
//!
//! ```bash
//! CARGO_TARGET_DIR=target cargo run --release --example p224_two_rank_null -- \
//!     run --out research/p224_two_rank_null_20261022/run --n 1000
//! cargo run --release --example p224_two_rank_null -- \
//!     analyze --dir research/p224_two_rank_null_20261022/run
//! ```
//!
//! `run` writes `rows.jsonl` (one row per sample, the raw data) and
//! `config.json`.  `analyze` recomputes everything else from `rows.jsonl`:
//! `summary.json`, `summary.md` and `omega_null.svg`.

use crypto_lib::cryptanalysis::p224_hypotheses::trace;
use crypto_lib::cryptanalysis::two_rank_null::*;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::{BigInt, BigUint};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::io::Write;
use std::time::Instant;

/// Significance level and size of the pre-registered tested family.
const ALPHA: f64 = 0.05;
const FAMILY: usize = 10;

struct Arm {
    name: &'static str,
    id: u64,
    bits: u64,
    fixed_p: bool,
    scheme: TraceScheme,
    big: bool,
}

fn p224() -> BigUint {
    (BigUint::from(1u8) << 224u32) - (BigUint::from(1u8) << 96u32) + 1u8
}

fn arms() -> Vec<Arm> {
    let a = |name, id, bits, fixed_p, scheme, big| Arm {
        name,
        id,
        bits,
        fixed_p,
        scheme,
        big,
    };
    use TraceScheme::*;
    vec![
        a("R224u", 1, 224, false, Uniform, false),
        a("R224s", 2, 224, false, Semicircle, false),
        a("F224u", 3, 224, true, Uniform, false),
        a("F224s", 4, 224, true, Semicircle, false),
        a("R256u", 5, 256, false, Uniform, false),
        a("R256s", 6, 256, false, Semicircle, false),
        a("R384u", 7, 384, false, Uniform, true),
        a("R521u", 8, 521, false, Uniform, true),
    ]
}

fn targets() -> Vec<CurveParams> {
    vec![
        CurveParams::p224(),
        CurveParams::p256(),
        CurveParams::p384(),
        CurveParams::p521(),
        CurveParams::secp224k1(),
        CurveParams::secp256k1(),
        CurveParams::brainpool_p224r1(),
        CurveParams::brainpool_p256r1(),
        CurveParams::brainpool_p384r1(),
    ]
}

struct Cfg {
    trial: u32,
    rho: u64,
    seed: u64,
}

#[allow(clippy::too_many_arguments)]
fn row(
    kind: &str,
    arm: &str,
    idx: u64,
    seed: u64,
    p: &BigUint,
    t: &BigInt,
    abs: &BigUint,
    cfg: &Cfg,
    extra: Value,
) -> Value {
    let t0 = Instant::now();
    let (f, b) = omega_of(abs, cfg.trial, cfg.rho);
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    let small_primes: Vec<String> = f
        .primes
        .iter()
        .filter(|(_, &(e, _))| e % 2 == 1)
        .map(|(q, _)| q.to_string())
        .collect();
    json!({
        "kind": kind, "arm": arm, "idx": idx, "seed": seed,
        "p": p.to_string(), "p_bits": p.bits(), "t": t.to_string(),
        "delta_bits": abs.bits(),
        "omega_lo": b.lo, "omega_mid": b.mid, "omega_hi": b.hi,
        "omega_small": b.small, "n_cofactors": b.cofactor_bits.len(),
        "cofactor_bits": b.cofactor_bits, "overlap": b.overlap,
        "rho_iterations": f.rho_iterations, "ms": ms,
        "odd_exponent_primes": small_primes, "extra": extra,
    })
}

fn null_row(arm: &Arm, idx: u64, cfg: &Cfg, plant: usize, kind: &str, idx_offset: u64) -> Value {
    let mut rng = sample_rng(cfg.seed, arm.id, idx + idx_offset);
    let p = if arm.fixed_p {
        p224()
    } else {
        random_prime(arm.bits, &mut rng)
    };
    let (_n, t) = sample_prime_order_trace(&p, arm.scheme, &mut rng);
    let mut abs = abs_delta(&p, &t);
    if plant > 0 {
        abs = plant_small_primes(&abs, plant);
    }
    row(
        kind,
        arm.name,
        idx + idx_offset,
        cfg.seed,
        &p,
        &t,
        &abs,
        cfg,
        json!({"planted": plant}),
    )
}

fn run(args: &[String]) {
    let get = |k: &str, d: &str| -> String {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
            .unwrap_or_else(|| d.to_string())
    };
    let out = get("--out", "run");
    let n: u64 = get("--n", "1000").parse().unwrap();
    let n_big: u64 = get("--n-big", "100").parse().unwrap();
    let n_pc: u64 = get("--n-pc", "300").parse().unwrap();
    let cfg = Cfg {
        trial: get("--trial", "1000000").parse().unwrap(),
        rho: get("--rho", "2097152").parse().unwrap(),
        seed: get("--seed", "20261022").parse().unwrap(),
    };
    let only = get("--only", "");
    std::fs::create_dir_all(&out).unwrap();
    let mut rows: Vec<Value> = Vec::new();
    let want = |s: &str| only.is_empty() || only.split(',').any(|o| o == s);
    let t_all = Instant::now();
    let stage = |label: &str, new: Vec<Value>, rows: &mut Vec<Value>| {
        let ms: f64 = new.iter().map(|r| r["ms"].as_f64().unwrap()).sum();
        eprintln!(
            "{label}: {} rows, {:.1} s cpu, {:.1} s wall elapsed",
            new.len(),
            ms / 1e3,
            t_all.elapsed().as_secs_f64()
        );
        rows.extend(new);
    };
    // 1. Positive control first (protocol section 4).
    if want("pc") {
        let a = &arms()[0];
        for k in [2usize, 4] {
            let new: Vec<Value> = (0..n_pc)
                .into_par_iter()
                .map(|i| null_row(a, i, &cfg, k, "pc_pop", 1_000_000))
                .collect();
            stage(&format!("pc_pop k={k}"), new, &mut rows);
        }
        let c = CurveParams::p224();
        let t = trace(&c);
        let abs0 = abs_delta(&c.p, &t);
        let new: Vec<Value> = (0..=12usize)
            .into_par_iter()
            .map(|k| {
                let abs = plant_small_primes(&abs0, k);
                row(
                    "pc_target",
                    &format!("P-224+{k}"),
                    k as u64,
                    cfg.seed,
                    &c.p,
                    &t,
                    &abs,
                    &cfg,
                    json!({"planted": k}),
                )
            })
            .collect();
        stage("pc_target", new, &mut rows);
    }
    // 2. Targets.
    if want("targets") {
        let new: Vec<Value> = targets()
            .par_iter()
            .map(|c| {
                let t = trace(c);
                let abs = abs_delta(&c.p, &t);
                row(
                    "target",
                    c.name,
                    0,
                    cfg.seed,
                    &c.p,
                    &t,
                    &abs,
                    &cfg,
                    json!({}),
                )
            })
            .collect();
        stage("targets", new, &mut rows);
    }
    // 3. Null arms.
    for a in arms() {
        if !want(a.name) {
            continue;
        }
        let m = if a.big { n_big } else { n };
        let new: Vec<Value> = (0..m)
            .into_par_iter()
            .map(|i| null_row(&a, i, &cfg, 0, "null", 0))
            .collect();
        stage(a.name, new, &mut rows);
    }
    let mut f = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{out}/rows.jsonl"))
            .unwrap(),
    );
    for r in &rows {
        writeln!(f, "{r}").unwrap();
    }
    std::fs::write(
        format!("{out}/config.json"),
        serde_json::to_string_pretty(&json!({
            "trial_bound": cfg.trial, "rho_budget": cfg.rho, "seed": cfg.seed,
            "n": n, "n_big": n_big, "n_pc": n_pc, "only": only,
        }))
        .unwrap(),
    )
    .unwrap();
}

// ── Analysis ────────────────────────────────────────────────────────────────

fn load(dir: &str) -> Vec<Value> {
    std::fs::read_to_string(format!("{dir}/rows.jsonl"))
        .expect("rows.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn col(rows: &[&Value], key: &str) -> Vec<u32> {
    rows.iter()
        .map(|r| r[key].as_u64().unwrap() as u32)
        .collect()
}

struct Test {
    p_min: f64,
    p_mid: f64,
    p_max: f64,
    verdict: String,
    rank: String,
    up: Tail,
    lo: Tail,
    midrank: f64,
}

/// The pre-registered censoring rule (PROTOCOL.md, "Censoring").
fn tail_test(x: (u32, u32, u32), c: (&[u32], &[u32], &[u32]), thr: f64) -> Test {
    let (xlo, xmid, xhi) = x;
    let (clo, cmid, chi) = c;
    // Upper-tail evidence "target high": target at its lowest, controls at their highest.
    let up_max = Tail::new(chi, xlo).ge;
    let up_min = Tail::new(clo, xhi).ge;
    // Lower-tail evidence "target low": target at its highest, controls at their lowest.
    let lo_max = Tail::new(clo, xhi).le;
    let lo_min = Tail::new(chi, xlo).le;
    let n = clo.len();
    let d = (n + 1) as f64;
    let p = |u: usize, l: usize| (2.0 * ((1 + u) as f64 / d).min((1 + l) as f64 / d)).min(1.0);
    let p_max = p(up_max, lo_max);
    let p_min = p(up_min, lo_min);
    let mid = Tail::new(cmid, xmid);
    let p_mid = mid.p_two_sided();
    let (verdict, rank) = if p_max < thr {
        let high = mid.ge <= mid.le;
        (
            format!(
                "more extreme than {:.1}% of controls ({})",
                100.0
                    * if high {
                        mid.lt as f64
                    } else {
                        (n - mid.le) as f64
                    }
                    / n as f64,
                if high { "high" } else { "low" }
            ),
            format!("{}/{}", mid.lt + 1, n),
        )
    } else if p_min >= thr {
        (
            "within the control distribution".to_string(),
            format!("{:.0}/{}", mid.midrank() * n as f64, n),
        )
    } else {
        (
            "inconclusive: censoring".to_string(),
            format!("{:.0}/{}", mid.midrank() * n as f64, n),
        )
    };
    Test {
        p_min,
        p_mid,
        p_max,
        verdict,
        rank,
        up: mid,
        lo: mid,
        midrank: mid.midrank(),
    }
}

fn vers(rows: &[&Value]) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    (
        col(rows, "omega_lo"),
        col(rows, "omega_mid"),
        col(rows, "omega_hi"),
        col(rows, "omega_small"),
    )
}

fn pct(x: f64) -> String {
    format!("{:.1}%", 100.0 * x)
}

fn quantiles(v: &[u32]) -> (u32, u32, u32) {
    let mut s = v.to_vec();
    s.sort();
    let q = |f: f64| s[((s.len() - 1) as f64 * f).round() as usize];
    (q(0.5), q(0.025), q(0.975))
}

fn analyze(dir: &str) {
    let rows = load(dir);
    let cfg: Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{dir}/config.json")).unwrap())
            .unwrap();
    let thr = ALPHA / FAMILY as f64;
    let by = |kind: &str, arm: &str| -> Vec<&Value> {
        rows.iter()
            .filter(|r| r["kind"] == kind && r["arm"] == arm)
            .collect()
    };
    let target = |name: &str| -> Option<&Value> {
        rows.iter()
            .find(|r| r["kind"] == "target" && r["arm"] == name)
    };
    let tv = |r: &Value| {
        (
            r["omega_lo"].as_u64().unwrap() as u32,
            r["omega_mid"].as_u64().unwrap() as u32,
            r["omega_hi"].as_u64().unwrap() as u32,
            r["omega_small"].as_u64().unwrap() as u32,
        )
    };
    let mut md = String::new();
    let mut js = serde_json::Map::new();
    // Null summaries.
    writeln!(md, "## Null arms\n").unwrap();
    writeln!(md, "| arm | N | censored (any cofactor) | exact ω | ω_lo median [2.5%,97.5%] | ω_mid median [2.5%,97.5%] | ω_hi median [2.5%,97.5%] | ω_small median [2.5%,97.5%] | overlap flags |").unwrap();
    writeln!(md, "|:--|--:|--:|--:|:--|:--|:--|:--|--:|").unwrap();
    let mut null_js = serde_json::Map::new();
    for a in arms() {
        let r = by("null", a.name);
        if r.is_empty() {
            continue;
        }
        let (lo, mid, hi, small) = vers(&r);
        let cens = r
            .iter()
            .filter(|x| x["n_cofactors"].as_u64().unwrap() > 0)
            .count();
        let (cl, ch) = clopper_pearson(cens, r.len(), 0.05);
        let q = |v: &[u32]| {
            let (m, a, b) = quantiles(v);
            format!("{m} [{a},{b}]")
        };
        let ov = r.iter().filter(|x| x["overlap"] == true).count();
        writeln!(
            md,
            "| {} | {} | {} ({}; CP95 {}..{}) | {} | {} | {} | {} | {} | {} |",
            a.name,
            r.len(),
            cens,
            pct(cens as f64 / r.len() as f64),
            pct(cl),
            pct(ch),
            r.len() - cens,
            q(&lo),
            q(&mid),
            q(&hi),
            q(&small),
            ov
        )
        .unwrap();
        null_js.insert(
            a.name.into(),
            json!({"n": r.len(), "censored": cens, "censored_cp95": [cl, ch], "overlap": ov,
                   "mean_lo": lo.iter().sum::<u32>() as f64 / lo.len() as f64,
                   "mean_mid": mid.iter().sum::<u32>() as f64 / mid.len() as f64,
                   "mean_hi": hi.iter().sum::<u32>() as f64 / hi.len() as f64,
                   "mean_small": small.iter().sum::<u32>() as f64 / small.len() as f64}),
        );
    }
    js.insert("null".into(), Value::Object(null_js));
    // Targets.
    writeln!(md, "\n## Target discriminants\n").unwrap();
    writeln!(md, "| target | |Δ| bits | ω_lo | ω_mid | ω_hi | ω_small | 2-rank = ω−1 | unsplit cofactor bits |").unwrap();
    writeln!(md, "|:--|--:|--:|--:|--:|--:|--:|:--|").unwrap();
    for c in targets() {
        if let Some(r) = target(c.name) {
            let (lo, mid, hi, sm) = tv(r);
            writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {}..{} | {} |",
                c.name,
                r["delta_bits"],
                lo,
                mid,
                hi,
                sm,
                lo.saturating_sub(1),
                hi.saturating_sub(1),
                r["cofactor_bits"]
            )
            .unwrap();
        }
    }
    // Pre-registered family.
    let family: [(&str, &str); 5] = [
        ("P-224", "R224u"),
        ("P-224", "F224u"),
        ("P-256", "R256u"),
        ("brainpoolP224r1", "R224u"),
        ("brainpoolP256r1", "R256u"),
    ];
    let mut tests_js = Vec::new();
    for (title, suffix_s, scale) in [
        (
            "Pre-registered family (uniform trace; threshold alpha/m = 0.005, m = 10)",
            false,
            true,
        ),
        (
            "Sensitivity: semicircle-weighted trace (not in the family; same rule and threshold)",
            true,
            true,
        ),
    ] {
        let _ = scale;
        writeln!(md, "\n## {title}\n").unwrap();
        writeln!(md, "| target | null arm | metric | target value | control median [2.5%,97.5%] | rank r/N | mid-rank percentile | upper tail (ω ≥ x) frac, CP95 | lower tail (ω ≤ x) frac, CP95 | p (min / mid / max over censoring) | verdict |").unwrap();
        writeln!(md, "|:--|:--|:--|--:|:--|--:|--:|:--|:--|:--|:--|").unwrap();
        for (tn, an) in family {
            let an = if suffix_s {
                an.replace('u', "s")
            } else {
                an.to_string()
            };
            let (Some(tr), ctrl) = (target(tn), by("null", &an)) else {
                continue;
            };
            if ctrl.is_empty() {
                continue;
            }
            let (clo, cmid, chi, csm) = vers(&ctrl);
            let (xlo, xmid, xhi, xsm) = tv(tr);
            for metric in ["omega", "omega_small"] {
                let t = if metric == "omega" {
                    tail_test((xlo, xmid, xhi), (&clo, &cmid, &chi), thr)
                } else {
                    tail_test((xsm, xsm, xsm), (&csm, &csm, &csm), thr)
                };
                let (cm, c1, c2) = if metric == "omega" {
                    quantiles(&cmid)
                } else {
                    quantiles(&csm)
                };
                let n = ctrl.len();
                let (ul, uh) = clopper_pearson(t.up.ge, n, 0.05);
                let (ll, lh) = clopper_pearson(t.lo.le, n, 0.05);
                let xs = if metric == "omega" {
                    if xlo == xhi {
                        format!("{xlo}")
                    } else {
                        format!("{xlo}..{xhi}")
                    }
                } else {
                    format!("{xsm}")
                };
                writeln!(
                    md,
                    "| {tn} | {an} | {metric} | {xs} | {cm} [{c1},{c2}] | {} | {} | {} ({}..{}) | {} ({}..{}) | {:.4} / {:.4} / {:.4} | {} |",
                    t.rank,
                    pct(t.midrank),
                    pct(t.up.ge as f64 / n as f64),
                    pct(ul),
                    pct(uh),
                    pct(t.lo.le as f64 / n as f64),
                    pct(ll),
                    pct(lh),
                    t.p_min,
                    t.p_mid,
                    t.p_max,
                    t.verdict
                )
                .unwrap();
                tests_js.push(
                    json!({"family": if suffix_s {"sensitivity_semicircle"} else {"primary"},
                    "target": tn, "null": an, "metric": metric, "x": [xlo, xmid, xhi, xsm],
                    "n": n, "upper_ge": t.up.ge, "lower_le": t.lo.le,
                    "midrank": t.midrank, "p_min": t.p_min, "p_mid": t.p_mid, "p_max": t.p_max,
                    "verdict": t.verdict}),
                );
            }
        }
    }
    js.insert("tests".into(), Value::Array(tests_js));
    // Descriptive: other targets.
    writeln!(
        md,
        "\n## Descriptive comparison points (no verdict; outside the tested family)\n"
    )
    .unwrap();
    writeln!(md, "| target | null arm | N | target ω (lo..hi) | control ω_mid median [2.5%,97.5%] | fraction of controls with ω_hi ≥ x_lo / ω_lo ≤ x_hi |").unwrap();
    writeln!(md, "|:--|:--|--:|--:|:--|:--|").unwrap();
    for (tn, an) in [
        ("P-384", "R384u"),
        ("brainpoolP384r1", "R384u"),
        ("P-521", "R521u"),
        ("secp224k1", "R224u"),
        ("secp256k1", "R256u"),
    ] {
        let (Some(tr), ctrl) = (target(tn), by("null", an)) else {
            continue;
        };
        if ctrl.is_empty() {
            continue;
        }
        let (clo, cmid, chi, _) = vers(&ctrl);
        let (xlo, _, xhi, _) = tv(tr);
        let (m, a, b) = quantiles(&cmid);
        writeln!(
            md,
            "| {tn} | {an} | {} | {xlo}..{xhi} | {m} [{a},{b}] | {} / {} |",
            ctrl.len(),
            pct(Tail::new(&chi, xlo).ge as f64 / ctrl.len() as f64),
            pct(Tail::new(&clo, xhi).le as f64 / ctrl.len() as f64)
        )
        .unwrap();
    }
    // Positive control.
    writeln!(md, "\n## Positive control\n").unwrap();
    let base = by("null", "R224u");
    let (_, bmid, _, _) = vers(&base);
    let mut pc_js = serde_json::Map::new();
    writeln!(md, "(a) Population shift: fresh R224u samples with k planted small primes vs the R224u null (Mann-Whitney on ω_mid, two-sided).\n").unwrap();
    writeln!(md, "| planted k | N | mean ω_mid planted | mean ω_mid null | shift | MW z | MW p | detected (p < 0.001 and |shift − k| ≤ 0.5) |").unwrap();
    writeln!(md, "|--:|--:|--:|--:|--:|--:|--:|:--|").unwrap();
    let mean = |v: &[u32]| v.iter().sum::<u32>() as f64 / v.len().max(1) as f64;
    let mut pop_ok = true;
    for k in [2u64, 4] {
        let r: Vec<&Value> = rows
            .iter()
            .filter(|r| r["kind"] == "pc_pop" && r["extra"]["planted"] == k)
            .collect();
        if r.is_empty() {
            continue;
        }
        let (_, pm, _, _) = vers(&r);
        let (z, p) = mann_whitney(&bmid, &pm);
        let shift = mean(&pm) - mean(&bmid);
        let ok = p < 0.001 && (shift - k as f64).abs() <= 0.5;
        pop_ok &= ok;
        writeln!(
            md,
            "| {k} | {} | {:.2} | {:.2} | {:+.2} | {:.1} | {:.2e} | {} |",
            pm.len(),
            mean(&pm),
            mean(&bmid),
            shift,
            z,
            p,
            if ok { "yes" } else { "NO" }
        )
        .unwrap();
        pc_js.insert(
            format!("pop_k{k}"),
            json!({"n": pm.len(), "shift": shift, "z": z, "p": p, "detected": ok}),
        );
    }
    writeln!(md, "\n(b) Target sweep: P-224's discriminant times the first k primes >= 5 not dividing it, ranked in R224u (the P-224 vs R224u rule, threshold {thr}).\n").unwrap();
    writeln!(md, "| k | planted ω (lo..hi) | midrank percentile | p min / mid / max | verdict | ω_small | p (ω_small) | flagged (ω_small) |").unwrap();
    writeln!(md, "|--:|--:|--:|:--|:--|--:|--:|:--|").unwrap();
    let (clo, cmid, chi, csm) = vers(&base);
    let mut first_flag_small: Option<u64> = None;
    let mut first_flag: Option<u64> = None;
    let mut sweep = rows
        .iter()
        .filter(|r| r["kind"] == "pc_target")
        .collect::<Vec<_>>();
    sweep.sort_by_key(|r| r["idx"].as_u64().unwrap());
    for r in sweep {
        let (lo, mid, hi, sm) = tv(r);
        let ts = tail_test((sm, sm, sm), (&csm, &csm, &csm), thr);
        if ts.p_max < thr && first_flag_small.is_none() {
            first_flag_small = Some(r["idx"].as_u64().unwrap());
        }
        let t = tail_test((lo, mid, hi), (&clo, &cmid, &chi), thr);
        let k = r["idx"].as_u64().unwrap();
        if t.p_max < thr && first_flag.is_none() {
            first_flag = Some(k);
        }
        writeln!(
            md,
            "| {k} | {lo}..{hi} | {} | {:.4} / {:.4} / {:.4} | {} | {sm} | {:.4} | {} |",
            pct(t.midrank),
            t.p_min,
            t.p_mid,
            t.p_max,
            t.verdict,
            ts.p_max,
            if ts.p_max < thr { "yes" } else { "no" }
        )
        .unwrap();
    }
    let pass =
        pop_ok && first_flag.is_some_and(|k| k <= 10) && first_flag_small.is_some_and(|k| k <= 10);
    writeln!(
        md,
        "\nPre-registered pass rule: both population shifts detected and the target sweep flagged (p_max < alpha/m) at some k <= 10 on both omega and omega_small. First flagged k: omega {:?}, omega_small {:?}. **Positive control {}.**",
        first_flag,
        first_flag_small,
        if pass { "PASSED" } else { "FAILED" }
    )
    .unwrap();
    pc_js.insert("first_flagged_k".into(), json!(first_flag));
    pc_js.insert("passed".into(), json!(pass));
    js.insert("positive_control".into(), Value::Object(pc_js));
    js.insert("config".into(), cfg);
    std::fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&Value::Object(js)).unwrap(),
    )
    .unwrap();
    std::fs::write(format!("{dir}/summary.md"), &md).unwrap();
    std::fs::write(format!("{dir}/omega_null.svg"), svg(&rows)).unwrap();
    println!("{md}");
}

/// ω histograms of the three uniform-trace nulls with target markers.
fn svg(rows: &[Value]) -> String {
    let panels = [
        (
            "R224u",
            "random 224-bit p",
            vec!["P-224", "brainpoolP224r1"],
        ),
        ("F224u", "fixed P-224 prime", vec!["P-224"]),
        (
            "R256u",
            "random 256-bit p",
            vec!["P-256", "brainpoolP256r1"],
        ),
    ];
    let (w, ph, left, bottom) = (900.0, 230.0, 70.0, 40.0);
    let h = 84.0 + panels.len() as f64 * (ph + bottom);
    let mut s = String::new();
    write!(s, "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'>").unwrap();
    write!(s, "<rect width='{w}' height='{h}' fill='white'/>").unwrap();
    write!(s, "<text x='{left}' y='24' font-size='16' font-weight='bold'>Distinct primes dividing the CM discriminant of prime-order curves, omega(D_K)</text>").unwrap();
    write!(s, "<text x='{left}' y='44' fill='#444'>bars: omega_mid fraction of null; thin lines: omega_lo (blue) and omega_hi (red) fractions; dashed: target.</text>").unwrap();
    write!(s, "<text x='{left}' y='60' fill='#444'>2-rank = omega - 1. Trace uniform on the Hasse interval, p+1-t prime. Run seed 20261022.</text>").unwrap();
    let kmax = 16usize;
    let bw = (w - left - 30.0) / (kmax + 1) as f64;
    for (pi, (arm, label, tg)) in panels.iter().enumerate() {
        let y0 = 84.0 + pi as f64 * (ph + bottom);
        let r: Vec<&Value> = rows
            .iter()
            .filter(|r| r["kind"] == "null" && r["arm"] == *arm)
            .collect();
        if r.is_empty() {
            continue;
        }
        let n = r.len() as f64;
        let frac = |key: &str| {
            let mut f = vec![0.0; kmax + 1];
            for x in &r {
                let k = (x[key].as_u64().unwrap() as usize).min(kmax);
                f[k] += 1.0 / n;
            }
            f
        };
        let (flo, fmid, fhi) = (frac("omega_lo"), frac("omega_mid"), frac("omega_hi"));
        let ymax = fmid
            .iter()
            .chain(&flo)
            .chain(&fhi)
            .cloned()
            .fold(0.0, f64::max)
            * 1.15;
        let ypx = |v: f64| y0 + ph - v / ymax * ph;
        write!(
            s,
            "<text x='{left}' y='{}' font-weight='bold'>{label} ({arm}, N = {})</text>",
            y0 - 6.0,
            r.len()
        )
        .unwrap();
        write!(
            s,
            "<line x1='{left}' y1='{}' x2='{}' y2='{}' stroke='#888'/>",
            y0 + ph,
            w - 30.0,
            y0 + ph
        )
        .unwrap();
        write!(
            s,
            "<line x1='{left}' y1='{y0}' x2='{left}' y2='{}' stroke='#888'/>",
            y0 + ph
        )
        .unwrap();
        for k in 0..=kmax {
            let x = left + k as f64 * bw;
            write!(
                s,
                "<rect x='{:.1}' y='{:.1}' width='{:.1}' height='{:.1}' fill='#9aa5b1'/>",
                x + 2.0,
                ypx(fmid[k]),
                bw - 4.0,
                y0 + ph - ypx(fmid[k])
            )
            .unwrap();
            for (f, c) in [(&flo, "#1f5fbf"), (&fhi, "#c0392b")] {
                write!(s, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{c}' stroke-width='2'/>", x + 2.0, ypx(f[k]), x + bw - 2.0, ypx(f[k])).unwrap();
            }
            write!(
                s,
                "<text x='{:.1}' y='{:.1}' text-anchor='middle' fill='#333'>{}</text>",
                x + bw / 2.0,
                y0 + ph + 16.0,
                if k == kmax {
                    format!("{k}+")
                } else {
                    k.to_string()
                }
            )
            .unwrap();
        }
        write!(
            s,
            "<text x='{}' y='{}' text-anchor='middle' fill='#333'>omega(D_K)</text>",
            (left + w - 30.0) / 2.0,
            y0 + ph + 32.0
        )
        .unwrap();
        write!(s, "<text x='14' y='{}' transform='rotate(-90 14 {})' text-anchor='middle' fill='#333'>fraction of null</text>", y0 + ph / 2.0, y0 + ph / 2.0).unwrap();
        for (ti, name) in tg.iter().enumerate() {
            if let Some(t) = rows
                .iter()
                .find(|r| r["kind"] == "target" && r["arm"] == *name)
            {
                let (lo, hi) = (
                    t["omega_lo"].as_u64().unwrap() as f64,
                    t["omega_hi"].as_u64().unwrap() as f64,
                );
                let xm = left + ((lo + hi) / 2.0).min(kmax as f64) * bw + bw / 2.0;
                let col = if ti == 0 { "#111" } else { "#b8860b" };
                write!(s, "<line x1='{xm:.1}' y1='{y0}' x2='{xm:.1}' y2='{}' stroke='{col}' stroke-width='2' stroke-dasharray='6 4'/>", y0 + ph).unwrap();
                write!(
                    s,
                    "<text x='{:.1}' y='{:.1}' fill='{col}'>{name}: omega {lo}{}</text>",
                    xm + 4.0,
                    y0 + 14.0 + ti as f64 * 14.0,
                    if hi > lo {
                        format!("..{hi}")
                    } else {
                        String::new()
                    }
                )
                .unwrap();
            }
        }
    }
    s.push_str("</svg>");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
        Some("analyze") => {
            let dir = args
                .iter()
                .position(|a| a == "--dir")
                .map(|i| args[i + 1].clone())
                .expect("--dir");
            analyze(&dir)
        }
        _ => eprintln!("usage: p224_two_rank_null run --out DIR [--n N] | analyze --dir DIR"),
    }
}
