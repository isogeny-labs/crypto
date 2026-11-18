//! Fall degrees of the binary Weil descent of Semaev's `S₃` at `m = 2`:
//! the instrument of the fall-degree paper's data request
//! (`research/fd_paper_data_request_20261029/`, protocols
//! FD-BINARY-CLOSURE-EXT and FD-F4-STEP-DEGREE).
//!
//! For each instance `(F_{2^n}, b, z, V)` it builds the descent of
//! `S₃(x₁, x₂, z)` with `x₁, x₂ ∈ V` ([`build_decomposition_system`],
//! `m = 2`, `2k` Boolean unknowns, `n` equations), counts its Boolean
//! solutions exactly, and then, as asked:
//!
//! * `--closure` scans `D = d-min ..= d-max` with [`closure_refutation_scan`]:
//!   XL (plain Macaulay) and the full MutantXL closure, whose degree-`D`
//!   row space is `W_D`, the closure of the Macaulay space under
//!   multiplying any element of degree `< D` by a variable.  The last
//!   fall degree is the least `D` whose closure contains `1`; a degree
//!   not built (over `--max-cols`) is unmeasured, never a refutation.
//! * `--f4` runs Boolean-ring F4 ([`groebner_basis_f4`], normal selection)
//!   and reports its highest step degree (`D_reg`), its solving degree
//!   and whether the basis is `{1}`.
//!
//! Instances come from a recorded file or are drawn:
//!
//! ```sh
//! # recorded (crypto-autoresearcher analysis/semaev-m2-last-fall-degree/results/instances.json)
//! cargo run --release --example fd_m2_degrees -- --instances instances.json \
//!     [--names P17a,R35a] --closure --d-min 3 --d-max 4 [--f4] [--max-cols 200000]
//! # drawn: family P (Semaev's <1,t,..,t^{k-1}>) or R (random), z on the curve
//! cargo run --release --example fd_m2_degrees -- --draw 41:R:3 --seed S --closure --d-max 4
//! # planted satisfiable controls (target x(P1+P2) with P_i abscissae in V)
//! cargo run --release --example fd_m2_degrees -- --draw 41:R:1 --seed S --planted --closure
//! ```
//!
//! A drawn target whose system has a Boolean solution is reported with
//! `"kind":"satisfiable"` and its draw is replaced by the next one (each
//! replacement is printed), as the protocol requires.  One JSON line per
//! instance on stdout; the field modulus, `b`, `z` and `V` are printed so
//! any line can be rebuilt.  `--checkpoint-dir DIR` makes the dense
//! closure scan resumable exactly as in `dreg_closure`.

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::closure_checkpoint::{
    closure_refutation_scan_checkpointed, ScanEnd, Store,
};
use crypto_lib::cryptanalysis::koblitz_bench::{closure_refutation_scan, ClosureDegree};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::pq_groebner_f2::mask_low64;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

/// One descent instance, in the words of `F_{2^n}` under `modulus`.
#[derive(Clone)]
struct Instance {
    name: String,
    n: u32,
    /// Bits of the modulus, leading term included.
    modulus: u64,
    a2: u64,
    b: u64,
    z: u64,
    basis: Vec<u64>,
    family: String,
}

fn irreducible(n: u32, modulus: u64) -> IrreduciblePoly {
    assert!(modulus >> n == 1, "modulus degree");
    IrreduciblePoly {
        degree: n,
        low_terms: (0..n).filter(|i| modulus >> i & 1 == 1).collect(),
    }
}

fn modulus_bits(irr: &IrreduciblePoly) -> u64 {
    irr.low_terms
        .iter()
        .fold(1u64 << irr.degree, |acc, &i| acc | 1 << i)
}

fn hex(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).expect("hex word")
}

/// Row echelon of a set of words by leading bit: `reduce` maps a word to
/// zero exactly when it lies in their F₂-span.
struct Echelon {
    rows: Vec<u64>,
}

impl Echelon {
    fn new(words: &[u64]) -> Self {
        let mut rows: Vec<u64> = Vec::new();
        for &w in words {
            let r = rows.iter().fold(w, |acc, &p| acc.min(acc ^ p));
            if r != 0 {
                rows.push(r);
                rows.sort_unstable_by(|a, b| b.cmp(a));
            }
        }
        Echelon { rows }
    }
    fn rank(&self) -> usize {
        self.rows.len()
    }
    fn reduce(&self, w: u64) -> u64 {
        self.rows.iter().fold(w, |acc, &p| acc.min(acc ^ p))
    }
}

/// Solver for `y² + y = t` over `F_{2^n}`: the map is F₂-linear with
/// kernel `{0, 1}`; `solve` returns one root or `None` (when `Tr t = 1`).
struct ArtinSchreier {
    /// For each pivot bit, the combination of `L(e_i)` images and the
    /// preimage that produced it.
    rows: Vec<(u64, u64)>,
}

impl ArtinSchreier {
    fn new(gf: &Gf2, n: u32) -> Self {
        let mut rows: Vec<(u64, u64)> = Vec::new();
        for i in 0..n {
            let e = 1u64 << i;
            let (mut img, mut pre) = (gf.sqr(e) ^ e, e);
            for &(pi, pp) in &rows {
                if img ^ pi < img {
                    img ^= pi;
                    pre ^= pp;
                }
            }
            if img != 0 {
                rows.push((img, pre));
                rows.sort_unstable_by_key(|r| std::cmp::Reverse(r.0));
            }
        }
        ArtinSchreier { rows }
    }
    fn solve(&self, t: u64) -> Option<u64> {
        let (mut r, mut y) = (t, 0u64);
        for &(pi, pp) in &self.rows {
            if r ^ pi < r {
                r ^= pi;
                y ^= pp;
            }
        }
        (r == 0).then_some(y)
    }
}

fn sqrt(gf: &Gf2, a: u64, n: u32) -> u64 {
    gf.sqr_k(a, n - 1)
}

/// Roots `c` of `S₃(a, c, z) = (a+c)²z² + acz + (ac)² + b`, a quadratic in `c`.
fn s3_roots(gf: &Gf2, asr: &ArtinSchreier, n: u32, a: u64, z: u64, b: u64) -> Vec<u64> {
    let coef_a = gf.sqr(z) ^ gf.sqr(a);
    let coef_b = gf.mul(a, z);
    let coef_c = gf.mul(gf.sqr(a), gf.sqr(z)) ^ b;
    match (coef_a, coef_b) {
        (0, 0) => {
            if coef_c == 0 {
                panic!("S3 vanishes identically in c");
            }
            Vec::new()
        }
        (0, _) => vec![gf.mul(coef_c, gf.inv(coef_b))],
        (_, 0) => vec![sqrt(gf, gf.mul(coef_c, gf.inv(coef_a)), n)],
        _ => {
            // c = (B/A) y with y² + y = A C / B².
            let t = gf.mul(gf.mul(coef_a, coef_c), gf.inv(gf.sqr(coef_b)));
            match asr.solve(t) {
                None => Vec::new(),
                Some(y) => {
                    let s = gf.mul(coef_b, gf.inv(coef_a));
                    vec![gf.mul(s, y), gf.mul(s, y ^ 1)]
                }
            }
        }
    }
}

fn span(words: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64];
    for &w in words {
        let more: Vec<u64> = out.iter().map(|x| x ^ w).collect();
        out.extend(more);
    }
    out
}

/// Ordered pairs `(a, c) ∈ V²` with `S₃(a, c, z) = 0`: the Boolean solutions.
fn solution_count(inst: &Instance, gf: &Gf2, asr: &ArtinSchreier) -> u64 {
    let ech = Echelon::new(&inst.basis);
    span(&inst.basis)
        .par_iter()
        .map(|&a| {
            s3_roots(gf, asr, inst.n, a, inst.z, inst.b)
                .into_iter()
                .filter(|&c| ech.reduce(c) == 0)
                .count() as u64
        })
        .sum()
}

fn trace(gf: &Gf2, n: u32, x: u64) -> u64 {
    let (mut t, mut s) = (0u64, x);
    for _ in 0..n {
        t ^= s;
        s = gf.sqr(s);
    }
    t & 1
}

/// `x` is the abscissa of an `F_{2^n}`-point of `y² + xy = x³ + a₂x² + b`.
fn on_curve(gf: &Gf2, n: u32, x: u64, a2: u64, b: u64) -> bool {
    x != 0 && trace(gf, n, x ^ a2 ^ gf.mul(b, gf.inv(gf.sqr(x)))) == 0
}

fn random_word(rng: &mut StdRng, n: u32) -> u64 {
    rng.gen::<u64>() & ((1u64 << n) - 1)
}

fn cell_rng(seed: u64, n: u32, family: &str, planted: bool) -> StdRng {
    let mut h = Sha256::new();
    h.update(format!("fd_m2_degrees|{seed}|{n}|{family}|{planted}").as_bytes());
    let d = h.finalize();
    StdRng::seed_from_u64(u64::from_be_bytes(d[..8].try_into().unwrap()))
}

/// Draw instances for one cell; an unplanted draw with solutions is
/// reported and replaced.
fn draw_cell(
    n: u32,
    family: &str,
    count: usize,
    seed: u64,
    planted: bool,
    report: &mut dyn FnMut(&Instance, u64),
) -> Vec<Instance> {
    let k = n.div_ceil(2) as usize;
    let irr = find_irreducible_sparse(n).expect("field");
    let gf = Gf2::new(&irr);
    let asr = ArtinSchreier::new(&gf, n);
    let mut rng = cell_rng(seed, n, family, planted);
    let mut out = Vec::new();
    let mut index = 0usize;
    while out.len() < count {
        let a2 = rng.gen::<u64>() & 1;
        let b = loop {
            let w = random_word(&mut rng, n);
            if w != 0 {
                break w;
            }
        };
        let basis: Vec<u64> = match family {
            "P" => (0..k).map(|i| 1u64 << i).collect(),
            "R" => loop {
                let v: Vec<u64> = (0..k).map(|_| random_word(&mut rng, n)).collect();
                if Echelon::new(&v).rank() == k {
                    break v;
                }
            },
            other => panic!("family {other}: P or R"),
        };
        let z = if planted {
            // A root z of S₃(a, c, z) for random a, c ∈ V: (a, c) is a solution.
            let words = span(&basis);
            loop {
                let a = words[rng.gen_range(0..words.len())];
                let c = words[rng.gen_range(0..words.len())];
                // S₃ is symmetric, so the roots in z of S₃(a, c, z) are
                // s3_roots with c in the third slot.
                let zs = s3_roots(&gf, &asr, n, a, c, b);
                if let Some(&z) = zs.iter().find(|&&z| z != 0) {
                    break z;
                }
            }
        } else {
            loop {
                let x = random_word(&mut rng, n);
                if on_curve(&gf, n, x, a2, b) {
                    break x;
                }
            }
        };
        let inst = Instance {
            name: format!(
                "{family}{n}-s{seed}-{index}{}",
                if planted { "-sat" } else { "" }
            ),
            n,
            modulus: modulus_bits(&irr),
            a2,
            b,
            z,
            basis,
            family: family.to_string(),
        };
        index += 1;
        let sols = solution_count(&inst, &gf, &asr);
        if planted {
            assert!(sols > 0, "planted instance has no solution");
            out.push(inst);
        } else if sols > 0 {
            report(&inst, sols);
        } else {
            out.push(inst);
        }
    }
    out
}

fn recorded(file: &str, names: &Option<Vec<String>>) -> Vec<Instance> {
    let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
    let v: Value = serde_json::from_str(&text).expect("instances json");
    v.as_array()
        .expect("array of instances")
        .iter()
        .filter(|e| {
            names
                .as_ref()
                .is_none_or(|ns| ns.iter().any(|n| e["name"].as_str() == Some(n)))
        })
        .filter(|e| e.get("t").is_none())
        .map(|e| Instance {
            name: e["name"].as_str().unwrap().to_string(),
            n: e["n"].as_u64().unwrap() as u32,
            modulus: hex(e["modulus_hex"].as_str().unwrap()),
            a2: e["a2"].as_u64().unwrap_or(0),
            b: hex(e["a6_hex"].as_str().unwrap()),
            z: hex(e["z_hex"].as_str().unwrap()),
            basis: e["V_basis_hex"]
                .as_array()
                .unwrap()
                .iter()
                .map(|w| hex(w.as_str().unwrap()))
                .collect(),
            family: e["basis"].as_str().unwrap_or("").to_string(),
        })
        .collect()
}

fn degree_json(d: &ClosureDegree) -> String {
    format!(
        r#"{{"degree":{},"xl_refuted":{},"closure_refuted":{},"saturated":{},"xl_rank":{},"closure_rank":{},"columns":{},"closure_products":{},"closure_word_ops":{},"secs":{:.3}}}"#,
        d.degree,
        d.xl_refuted,
        d.closure_refuted,
        d.saturated,
        d.xl_rank,
        d.closure_rank,
        d.columns,
        d.closure_products,
        d.closure_word_ops,
        d.secs
    )
}

fn monomials_up_to(n_vars: u64, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d as u64 {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
    }
    total
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let d_min: u32 = flag("--d-min").map_or(3, |v| v.parse().expect("--d-min"));
    let d_max: u32 = flag("--d-max").map_or(4, |v| v.parse().expect("--d-max"));
    let max_cols: f64 = flag("--max-cols").map_or(600_000.0, |v| v.parse().expect("--max-cols"));
    let f4_budget = flag("--f4-budget").map(|v| Duration::from_secs(v.parse().expect("secs")));
    let names: Option<Vec<String>> =
        flag("--names").map(|v| v.split(',').map(str::to_string).collect());
    let seed: u64 = flag("--seed").map_or(0, |v| v.parse().expect("--seed"));
    let planted = has("--planted");
    let mut store = flag("--checkpoint-dir").map(|d| {
        Store::open(std::path::Path::new(&d), Duration::from_secs(900)).expect("--checkpoint-dir")
    });
    if store.is_some() {
        Store::install_signal_handlers();
    }

    let mut instances = Vec::new();
    if let Some(file) = flag("--instances") {
        instances.extend(recorded(&file, &names));
    }
    if let Some(spec) = flag("--draw") {
        for cell in spec.split(',') {
            let parts: Vec<&str> = cell.split(':').collect();
            let (n, family, count): (u32, &str, usize) = (
                parts[0].parse().expect("n"),
                parts[1],
                parts[2].parse().expect("count"),
            );
            let mut report = |inst: &Instance, sols: u64| {
                println!(
                    r#"{{"name":"{}","n":{},"family":"{}","seed":{seed},"kind":"satisfiable","solutions":{sols},"replaced":true,"modulus_hex":"{:x}","a2":{},"b_hex":"{:x}","z_hex":"{:x}","V_basis_hex":[{}]}}"#,
                    inst.name,
                    inst.n,
                    inst.family,
                    inst.modulus,
                    inst.a2,
                    inst.b,
                    inst.z,
                    inst.basis
                        .iter()
                        .map(|w| format!("\"{w:x}\""))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            };
            instances.extend(draw_cell(n, family, count, seed, planted, &mut report));
        }
    }
    assert!(
        !instances.is_empty(),
        "no instances: --instances FILE or --draw n:F:count"
    );

    for inst in &instances {
        let irr = irreducible(inst.n, inst.modulus);
        let gf = Gf2::new(&irr);
        let asr = ArtinSchreier::new(&gf, inst.n);
        let st = FieldStructure::new(inst.n, &irr);
        let to_el = |w: u64| F2mElement::from_biguint(&BigUint::from(w), inst.n);
        let basis: Vec<F2mElement> = inst.basis.iter().map(|&w| to_el(w)).collect();
        let started = Instant::now();
        let solutions = solution_count(inst, &gf, &asr);
        let count_secs = started.elapsed().as_secs_f64();
        let sys = build_decomposition_system(&basis, &to_el(inst.z), &to_el(inst.b), 2, &st)
            .expect("system");
        let mut body = format!(
            r#""n_vars":{},"n_eqs":{},"solutions":{solutions},"count_secs":{count_secs:.3}"#,
            sys.n_vars,
            sys.equations.len()
        );
        if has("--closure") {
            let mut scan = Vec::new();
            let mut unbuilt = None;
            for d in d_min..=d_max {
                if monomials_up_to(sys.n_vars as u64, d) > max_cols {
                    unbuilt = Some(d);
                    break;
                }
                let got = match store.as_mut() {
                    None => closure_refutation_scan(&sys.equations, sys.n_vars, d, d),
                    Some(s) => match closure_refutation_scan_checkpointed(
                        &sys.equations,
                        sys.n_vars,
                        d,
                        d,
                        s,
                    )
                    .expect("checkpointed scan")
                    {
                        ScanEnd::Done(v) => v,
                        ScanEnd::Interrupted => std::process::exit(143),
                    },
                };
                let refuted = got.iter().any(|x| x.closure_refuted);
                let built = !got.is_empty();
                scan.extend(got);
                if refuted || !built {
                    if !built {
                        unbuilt = Some(d);
                    }
                    break;
                }
            }
            let closure_degree = scan.iter().find(|x| x.closure_refuted).map(|x| x.degree);
            let xl_degree = scan.iter().find(|x| x.xl_refuted).map(|x| x.degree);
            let lfd = match (closure_degree, scan.last()) {
                (Some(d), _) => format!(r#"{{"exact":{d}}}"#),
                (None, Some(last)) => format!(r#"{{"at_least":{}}}"#, last.degree + 1),
                (None, None) => r#"{"unmeasured":true}"#.to_string(),
            };
            body += &format!(
                r#","closure":{{"d_min":{d_min},"d_max":{d_max},"max_cols":{max_cols},"last_fall_degree":{lfd},"xl_refutation_degree":{},"first_unbuilt_degree":{},"scan":[{}]}}"#,
                xl_degree.map_or("null".into(), |d| d.to_string()),
                unbuilt.map_or("null".into(), |d| d.to_string()),
                scan.iter().map(degree_json).collect::<Vec<_>>().join(",")
            );
        }
        if has("--f4") {
            let (gb, stats) = groebner_basis_f4(sys.equations.clone(), sys.n_vars, f4_budget);
            let refuted =
                gb.len() == 1 && gb[0].terms.len() == 1 && mask_low64(gb[0].terms[0].mask) == 0;
            body += &format!(
                r#","f4":{{"degree_reached":{},"solving_degree":{},"max_poly_degree":{},"refuted":{refuted},"timed_out":{},"oversize":{},"steps":{},"word_xors":{},"peak_matrix_bytes":{},"wall_s":{:.3}}}"#,
                stats.degree_reached,
                stats.solving_degree,
                stats.max_poly_degree,
                stats.timed_out,
                stats.oversize,
                stats.steps,
                stats.word_xors,
                stats.peak_matrix_bytes,
                stats.wall_ns as f64 / 1e9
            );
        }
        println!(
            r#"{{"name":"{}","n":{},"k":{},"family":"{}","seed":{},"planted":{planted},"modulus_hex":"{:x}","a2":{},"b_hex":"{:x}","z_hex":"{:x}","V_basis_hex":[{}],{body}}}"#,
            inst.name,
            inst.n,
            inst.basis.len(),
            inst.family,
            seed,
            inst.modulus,
            inst.a2,
            inst.b,
            inst.z,
            inst.basis
                .iter()
                .map(|w| format!("\"{w:x}\""))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
