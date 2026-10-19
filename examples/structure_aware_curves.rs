//! Curve selection, spec generation and analysis for
//! `research/structure_aware_ic_20261019/` (Test A curve choice, Test B).
//!
//! Modes (`structure_aware_curves <mode> ...`):
//!
//! * `j0 <bits> <count> <seed>`, `rand <bits> <count> <seed>`: print `ecbench`
//!   curve entries.  `j0` is `y² = x³ + b`, `p ≡ 1 (mod 3)` of `bits` bits,
//!   `#E` prime (|Aut| = 6).  `rand` is `y² = x³ + a x + b`, `a, b ≠ 0`,
//!   same `p` rule, `#E` prime (Aut = {±1} unless j is special).
//! * `bin <n> <count> <seed0> [max_cofactor]`: the first seeds ≥ `seed0` for
//!   which `random_binary_instance` returns an `a = 1` curve.
//! * `spec-tune <bits> <outdir>`: tuning specs (size grid) on separate curves.
//! * `pick <bits> <dir>`: reads the tuning band reports, prints the chosen
//!   factor-base sizes (rule frozen in PROTOCOL.md).
//! * `spec-test <bits> <a6> <a2j> <a2r> <outdir>`: the test specs.
//! * `analyze <dir> <bits>...`: Test B statistics from the test band reports.
//!
//! No result is computed outside `analyze`, which only reads `ecbench band`
//! reports.

use crypto_lib::cryptanalysis::ic_boundary::{random_binary_instance, PrimeCurve, PrimePoint};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const GRID: [u64; 8] = [8, 16, 32, 64, 128, 256, 512, 1024];
const EXT_GRID: [u64; 2] = [2048, 4096];
const TUNE_SEED: u64 = 5001;
const TEST_J0_SEED: u64 = 101;
const TEST_RAND_A_SEED: u64 = 201;
const TEST_RAND_B_SEED: u64 = 202;
const TUNE_CURVES: usize = 2;
const TUNE_TARGETS: u64 = 4;
const TEST_CURVES: usize = 6;
const TEST_TARGETS: u64 = 8;
/// Band weight (GAE per unpriced native operation) used by the frozen tuning rule.
const TUNE_W_INDEX: usize = 1;

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn prime_with_bits(rng: &mut StdRng, bits: u32) -> u64 {
    loop {
        let cand = (rng.gen::<u64>() & ((1u64 << bits) - 1)) | (1u64 << (bits - 1)) | 1;
        if cand % 3 == 1 && is_prime_u64(cand) {
            return cand;
        }
    }
}

fn first_point(curve: &PrimeCurve) -> PrimePoint {
    (0..curve.p)
        .find_map(|x| curve.sqrt(curve.rhs(x)).map(|y| PrimePoint::affine(x, y)))
        .expect("a point exists")
}

fn curves(mode: &str, bits: u32, count: usize, seed: u64) -> Vec<Value> {
    let mut rng = StdRng::seed_from_u64(seed ^ (u64::from(bits) << 40) ^ 0x5A17);
    let mut out = Vec::new();
    while out.len() < count {
        let p = prime_with_bits(&mut rng, bits);
        for _ in 0..16 {
            let b = rng.gen_range(1..p);
            let a = if mode == "j0" { 0 } else { rng.gen_range(1..p) };
            let curve = PrimeCurve { p, a, b };
            let disc = (4 * mulmod(mulmod(a, a, p), a, p) % p + 27 * mulmod(b, b, p) % p) % p;
            if disc == 0 {
                continue;
            }
            if let Some(order) = curve.prime_order() {
                if order == p {
                    continue; // anomalous
                }
                let g = first_point(&curve);
                out.push(json!({"kind":"prime_explicit","p":p,"a":a,"b":b,
                    "group_order":order,"r":order,"gx":g.x,"gy":g.y}));
                break;
            }
        }
    }
    out
}

fn measurement(timeout: u64) -> Value {
    json!({"isolation_required":"L0","order":"alternate","rounds":1,"seed":1,
           "timeout_seconds":timeout,"warmup":0})
}

fn ic_arm(name: &str, role: &str, group: &str, size: u64) -> Value {
    json!({"name":name,"role":role,"method":{"id":"ic.pipeline","params":{
        "factor_base":format!("glv-orbit:size={size},group={group}"),
        "oracle":"mitm:negation_folded=1"}}})
}

fn rho_arm(name: &str, role: &str, id: &str) -> Value {
    json!({"name":name,"role":role,"method":{"id":id}})
}

fn write(dir: &str, name: &str, v: &Value) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{dir}/{name}"), serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
    println!("wrote {dir}/{name}");
}

fn spec(label: &str, desc: &str, arms: Vec<Value>, curves: Vec<Value>, tseed: u64, targets: u64) -> Value {
    json!({"schema":"ecbench.spec/v1","label":label,"description":desc,"arms":arms,
        "measurement":measurement(3600),
        "workloads":{"curves":curves,"target_seed":tseed,"targets_per_curve":targets}})
}

/// (curve, arm) -> mean_s[3] from an `ecbench.band/v1` report.
fn band(path: &str) -> BTreeMap<(String, String), [f64; 3]> {
    band_with(path, true)
}

/// `strict = false` drops arms with any unverified run (inadmissible in tuning: infinite cost).
fn band_with(path: &str, strict: bool) -> BTreeMap<(String, String), [f64; 3]> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))).unwrap();
    let mut m = BTreeMap::new();
    for r in v["rows"].as_array().unwrap() {
        let s = r["mean_s"].as_array().unwrap();
        if r["runs"] != r["verified"] {
            assert!(!strict, "unverified runs in {path}");
            eprintln!("  inadmissible (unverified runs): {} {}", r["curve"], r["arm"]);
            continue;
        }
        m.insert(
            (r["curve"].as_str().unwrap().to_string(), r["arm"].as_str().unwrap().to_string()),
            [s[0].as_f64().unwrap(), s[1].as_f64().unwrap(), s[2].as_f64().unwrap()],
        );
    }
    m
}

fn sums(b: &BTreeMap<(String, String), [f64; 3]>, arm: &str) -> Vec<[f64; 3]> {
    b.iter().filter(|((_, a), _)| a == arm).map(|(_, v)| *v).collect()
}

fn pick(bits: u32, dir: &str) {
    let mut j = band_with(&format!("{dir}/tune_j0_{bits}.band.json"), false);
    let mut r = band_with(&format!("{dir}/tune_rand_{bits}.band.json"), false);
    // Amendment 4: grid extensions (`tune_*_<bits>x`) are merged when present.
    for (m, tag) in [(&mut j, "j0"), (&mut r, "rand")] {
        let p = format!("{dir}/tune_{tag}_{bits}x.band.json");
        if std::path::Path::new(&p).exists() {
            m.extend(band_with(&p, false));
        }
    }
    let best = |b: &BTreeMap<(String, String), [f64; 3]>, prefix: &str| -> u64 {
        let mut best = (f64::INFINITY, 0u64);
        for &sz in GRID.iter().chain(EXT_GRID.iter()) {
            let rows = sums(b, &format!("{prefix}{sz}"));
            // an arm missing a curve (unverified run there) is inadmissible
            if rows.len() < TUNE_CURVES {
                continue;
            }
            let tot: f64 = rows.iter().map(|v| v[TUNE_W_INDEX]).sum();
            eprintln!("  {prefix}{sz}: sum S(w=0.1) over {} curves = {tot:.2}", rows.len());
            if !rows.is_empty() && tot < best.0 {
                best = (tot, sz);
            }
        }
        if best.1 == GRID[0] || best.1 == EXT_GRID[EXT_GRID.len() - 1] {
            eprintln!("  NOTE {prefix}: optimum at the edge of the grid");
        }
        best.1
    };
    let a6 = best(&j, "ic-a6-");
    let a2j = best(&j, "ic-a2-");
    let a2r = best(&r, "ic-a2-");
    println!("{a6} {a2j} {a2r}");
}

fn boot(rng: &mut StdRng, num: &[f64], den: &[f64]) -> (f64, f64, f64) {
    // ratio of sums over curves, bootstrap over curves
    let n = num.len();
    let point = num.iter().sum::<f64>() / den.iter().sum::<f64>();
    let mut v: Vec<f64> = (0..10000)
        .map(|_| {
            let (mut a, mut b) = (0.0, 0.0);
            for _ in 0..n {
                let i = rng.gen_range(0..n);
                a += num[i];
                b += den[i];
            }
            a / b
        })
        .collect();
    v.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (point, v[250], v[9750])
}

/// Bootstrap of (Σa/Σb) / (Σc/Σd) with independent curve resampling of the two families.
fn boot_q(rng: &mut StdRng, a: &[f64], b: &[f64], c: &[f64], d: &[f64]) -> (f64, f64, f64) {
    let point = (a.iter().sum::<f64>() / b.iter().sum::<f64>()) / (c.iter().sum::<f64>() / d.iter().sum::<f64>());
    let mut v: Vec<f64> = (0..10000)
        .map(|_| {
            let (mut x, mut y) = (0.0, 0.0);
            for _ in 0..a.len() {
                let i = rng.gen_range(0..a.len());
                x += a[i];
                y += b[i];
            }
            let (mut z, mut w) = (0.0, 0.0);
            for _ in 0..c.len() {
                let i = rng.gen_range(0..c.len());
                z += c[i];
                w += d[i];
            }
            (x / y) / (z / w)
        })
        .collect();
    v.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (point, v[250], v[9750])
}

fn col(rows: &[[f64; 3]], w: usize) -> Vec<f64> {
    rows.iter().map(|v| v[w]).collect()
}

/// Rows of one arm over curves, in curve (slug) order, so two arms line up.
fn arm_by_curve(b: &BTreeMap<(String, String), [f64; 3]>, arm: &str) -> BTreeMap<String, [f64; 3]> {
    b.iter().filter(|((_, a), _)| a == arm).map(|((c, _), v)| (c.clone(), *v)).collect()
}

fn aligned(b: &BTreeMap<(String, String), [f64; 3]>, arm: &str, order: &[String]) -> Vec<[f64; 3]> {
    let m = arm_by_curve(b, arm);
    order.iter().map(|c| m[c]).collect()
}

fn analyze(dir: &str, bits: &[u32]) {
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut out = Vec::new();
    for &n in bits {
        let j = band(&format!("{dir}/test_j0_{n}.band.json"));
        let ra = band(&format!("{dir}/test_randA_{n}.band.json"));
        let rb = band(&format!("{dir}/test_randB_{n}.band.json"));
        let sizes: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/sizes_{n}.json")).unwrap()).unwrap();
        let (a6, a2j, a2r) = (
            sizes["a6"].as_u64().unwrap(),
            sizes["a2j"].as_u64().unwrap(),
            sizes["a2r"].as_u64().unwrap(),
        );
        let jc: Vec<String> = arm_by_curve(&j, "rho-neg").keys().cloned().collect();
        let rac: Vec<String> = arm_by_curve(&ra, "rho-neg").keys().cloned().collect();
        let rbc: Vec<String> = arm_by_curve(&rb, "rho-neg").keys().cloned().collect();
        let ic6 = aligned(&j, &format!("ic-a6-{a6}"), &jc);
        let ic2j = aligned(&j, &format!("ic-a2-{a2j}"), &jc);
        let rho6 = aligned(&j, "rho-cubic", &jc);
        let rhonj = aligned(&j, "rho-neg", &jc);
        let ic2a = aligned(&ra, &format!("ic-a2-{a2r}"), &rac);
        let rhoa = aligned(&ra, "rho-neg", &rac);
        let plaina = aligned(&ra, "rho-plain", &rac);
        let ic2b = aligned(&rb, &format!("ic-a2-{a2r}"), &rbc);
        let rhob = aligned(&rb, "rho-neg", &rbc);
        let cat = |x: &[[f64; 3]], y: &[[f64; 3]]| [x, y].concat();
        let ic2r = cat(&ic2a, &ic2b);
        let rhor = cat(&rhoa, &rhob);
        let mut per_w = Vec::new();
        for w in 0..3 {
            let (q, ql, qh) = boot_q(&mut rng, &col(&ic6, w), &col(&rho6, w), &col(&ic2r, w), &col(&rhor, w));
            let (gic, gicl, gich) = boot(&mut rng, &col(&ic2j, w), &col(&ic6, w));
            let (grho, grhol, grhoh) = boot(&mut rng, &col(&rhonj, w), &col(&rho6, w));
            let (pop, popl, poph) = boot_q(&mut rng, &col(&ic2j, w), &col(&rhonj, w), &col(&ic2r, w), &col(&rhor, w));
            let (aa, aal, aah) = boot_q(&mut rng, &col(&ic2a, w), &col(&rhoa, w), &col(&ic2b, w), &col(&rhob, w));
            let (pos, posl, posh) = boot(&mut rng, &col(&plaina, w), &col(&rhoa, w));
            let (rr, rrl, rrh) = boot(&mut rng, &col(&ic6, w), &col(&rho6, w));
            let (rf, rfl, rfh) = boot(&mut rng, &col(&ic2r, w), &col(&rhor, w));
            per_w.push(json!({
                "w_index": w,
                "Q_j0_over_random": [q, ql, qh],
                "IC_over_rho_j0_A6": [rr, rrl, rrh],
                "IC_over_rho_random_A2": [rf, rfl, rfh],
                "IC_gain_A2_over_A6_on_j0": [gic, gicl, gich],
                "rho_gain_neg_over_six": [grho, grhol, grhoh],
                "population_term_IC2_over_rho_j0_vs_random": [pop, popl, poph],
                "AA_random_A_over_B": [aa, aal, aah],
                "positive_rho_plain_over_neg": [pos, posl, posh],
            }));
        }
        out.push(json!({"bits": n, "sizes": sizes, "curves_j0": jc.len(), "curves_random": rac.len() + rbc.len(), "band": per_w}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({"weights":[0.0,0.1,1.0],"results":out})).unwrap());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let num = |i: usize| -> u64 { args.get(i).and_then(|s| s.parse().ok()).unwrap_or_else(|| panic!("argument {i}")) };
    match mode {
        "j0" | "rand" => {
            for c in curves(mode, num(2) as u32, num(3) as usize, num(4)) {
                println!("{c}");
            }
        }
        "bin" => {
            let n = num(2) as u32;
            let max_cof = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(2);
            let (mut s, mut found) = (num(4), 0);
            while found < num(3) {
                if let Some(i) = random_binary_instance(n, s, max_cof) {
                    if i.a == 1 {
                        println!(
                            "{{\"kind\":\"binary_random\",\"n\":{n},\"seed\":{s},\"max_cofactor\":{max_cof}}}  // a={} b={:#x} #E={} r={} h={}",
                            i.a, i.b, i.group_order, i.r, i.cofactor
                        );
                        found += 1;
                    }
                }
                s += 1;
            }
        }
        "spec-tune" => {
            let (bits, dir) = (num(2) as u32, args[3].as_str());
            let mut aj = vec![rho_arm("rho-neg", "reference", "rho.negation")];
            let mut ar = aj.clone();
            for &sz in &GRID {
                aj.push(ic_arm(&format!("ic-a6-{sz}"), "candidate", "auto", sz));
                aj.push(ic_arm(&format!("ic-a2-{sz}"), "control", "negation", sz));
                ar.push(ic_arm(&format!("ic-a2-{sz}"), "control", "negation", sz));
            }
            write(dir, &format!("tune_j0_{bits}.json"), &spec(
                &format!("tuning, j=0, {bits} bits"), "structure_aware_ic_20261019 PROTOCOL.md B-tune: size grid on tuning curves, never used for the result",
                aj, curves("j0", bits, TUNE_CURVES, TUNE_SEED), TUNE_SEED, TUNE_TARGETS));
            write(dir, &format!("tune_rand_{bits}.json"), &spec(
                &format!("tuning, random, {bits} bits"), "structure_aware_ic_20261019 PROTOCOL.md B-tune",
                ar, curves("rand", bits, TUNE_CURVES, TUNE_SEED), TUNE_SEED, TUNE_TARGETS));
        }
        "spec-ext" => {
            // Amendment 4: the A = 2 grid extended past 1024 on the same tuning curves.
            let (bits, dir) = (num(2) as u32, args[3].as_str());
            let mut arms = vec![rho_arm("rho-neg", "reference", "rho.negation")];
            for &sz in &EXT_GRID {
                arms.push(ic_arm(&format!("ic-a2-{sz}"), "control", "negation", sz));
            }
            for (tag, mode) in [("j0", "j0"), ("rand", "rand")] {
                write(dir, &format!("tune_{tag}_{bits}x.json"), &spec(
                    &format!("tuning grid extension, {tag}, {bits} bits"), "structure_aware_ic_20261019 PROTOCOL.md Amendment 4: the A=2 optimum was on the grid edge",
                    arms.clone(), curves(mode, bits, TUNE_CURVES, TUNE_SEED), TUNE_SEED, TUNE_TARGETS));
            }
        }
        "pick" => pick(num(2) as u32, &args[3]),
        "spec-test" => {
            let (bits, a6, a2j, a2r, dir) = (num(2) as u32, num(3), num(4), num(5), args[6].as_str());
            write(dir, &format!("sizes_{bits}.json"), &json!({"a6":a6,"a2j":a2j,"a2r":a2r}));
            write(dir, &format!("test_j0_{bits}.json"), &spec(
                &format!("test, j=0, {bits} bits"), "structure_aware_ic_20261019 PROTOCOL.md B-test: j=0 curves, six-fold rho and A=6 / A=2 IC at tuned sizes",
                vec![rho_arm("rho-neg", "reference", "rho.negation"), rho_arm("rho-cubic", "control", "rho.cubic"),
                     ic_arm(&format!("ic-a6-{a6}"), "candidate", "auto", a6), ic_arm(&format!("ic-a2-{a2j}"), "control", "negation", a2j)],
                curves("j0", bits, TEST_CURVES, TEST_J0_SEED), TEST_J0_SEED, TEST_TARGETS));
            for (tag, sd) in [("randA", TEST_RAND_A_SEED), ("randB", TEST_RAND_B_SEED)] {
                write(dir, &format!("test_{tag}_{bits}.json"), &spec(
                    &format!("test, random {tag}, {bits} bits"), "structure_aware_ic_20261019 PROTOCOL.md B-test: random-curve control, rho.negation and A=2 IC; rho.plain is the positive control",
                    vec![rho_arm("rho-neg", "reference", "rho.negation"), rho_arm("rho-plain", "control", "rho.plain"),
                         ic_arm(&format!("ic-a2-{a2r}"), "control", "negation", a2r)],
                    curves("rand", bits, TEST_CURVES, sd), sd, TEST_TARGETS));
            }
        }
        "analyze" => {
            let bits: Vec<u32> = args[3..].iter().map(|s| s.parse().unwrap()).collect();
            analyze(&args[2], &bits);
        }
        _ => panic!("modes: j0 rand bin spec-tune pick spec-test analyze"),
    }
}
