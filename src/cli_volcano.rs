//! `crypto volcano …` — the command-line surface for
//! [`crypto_lib::cryptanalysis::volcano`].
//!
//! ```text
//! crypto volcano profile --koblitz-n 83                 # conductor, depth, level tables
//! crypto volcano profile --ecc2k130 --ell 263
//! crypto volcano profile --q 1000003 --trace 1234       # prime-field curve by (p, trace)
//! crypto volcano level1 --d -7 --ell 263                # Φ_ℓ(X, j0) over Z by CRT
//! crypto volcano koblitz-step --n 83 --ell 6473         # the level-1 curves over F_(2^n)
//! ```
//!
//! Every subcommand prints the model and the checks it ran; nothing is
//! reported that was not computed.

use clap::Subcommand;
use crypto_lib::binary_ecc::f2m::F2mElement;
use crypto_lib::cryptanalysis::volcano::{arith::Rng, binary, f2poly, fp, level1, profile};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use rayon::prelude::*;
use std::time::Instant;

#[derive(Subcommand)]
pub enum VolcanoOp {
    /// Frobenius discriminant, conductor, volcano depth per prime, class numbers and the
    /// group structure E(F_q) at every level.
    ///
    /// Give the curve as `--koblitz-n N [--a2 0|1]` (y²+xy=x³+a₂x²+1 over F_(2^N)),
    /// `--ecc2k130`, or `--q <prime> --trace <t>` for a prime-field curve.  `--ell L` prints the
    /// level table of the L-volcano.
    Profile {
        #[arg(long)]
        koblitz_n: Option<u32>,
        /// Koblitz curve with a₂ = 1 instead of 0 (trace +1 over F_2).
        #[arg(long, default_value_t = false)]
        a2: bool,
        #[arg(long, default_value_t = false)]
        ecc2k130: bool,
        /// Field size (prime) for a prime-field curve.
        #[arg(long)]
        q: Option<String>,
        /// Frobenius trace for a prime-field curve (may be negative).
        #[arg(long, allow_hyphen_values = true)]
        trace: Option<String>,
        /// Print the level table for this prime.
        #[arg(long)]
        ell: Option<String>,
    },
    /// Φ_ℓ(X, j₀) over ℤ for the class-number-one crater of discriminant D, by CRT over
    /// word-size primes with Vélu on curves whose whole ℓ-torsion is rational.
    Level1 {
        /// Fundamental discriminant of class number one: -3 -4 -7 -8 -11 -19 -43 -67 -163.
        #[arg(long, allow_hyphen_values = true, default_value_t = -7)]
        d: i64,
        #[arg(long)]
        ell: u64,
        /// Worker threads (default: all cores).
        #[arg(long)]
        threads: Option<usize>,
        /// Write the integer coefficients (X^0 first, one per line) to this file.
        #[arg(long)]
        out: Option<String>,
        /// Also reduce mod 2 and print the factor structure over F_2.
        #[arg(long, default_value_t = false)]
        mod2: bool,
        /// Primes per parallel round (>= 8).
        #[arg(long, default_value_t = 16)]
        batch: usize,
    },
    /// The first descending step of the Koblitz curve's ℓ-volcano over F_(2^n): all
    /// neighbours as j-invariants in F_(2^n), grouped into Frobenius orbits, with a sample
    /// of them verified by group order.
    KoblitzStep {
        #[arg(long)]
        n: u32,
        #[arg(long)]
        ell: u64,
        #[arg(long, default_value_t = false)]
        a2: bool,
        /// Number of neighbours to verify by building the curve and testing [N]P = O.
        #[arg(long, default_value_t = 24)]
        verify: usize,
        #[arg(long)]
        threads: Option<usize>,
        /// Write the neighbour j-invariants (hex, one per line) to this file.
        #[arg(long)]
        out: Option<String>,
        #[arg(long, default_value_t = 16)]
        batch: usize,
    },
    /// Write ecbench specs for the Koblitz crater and a sample of its verified level-1
    /// neighbours over F_(2^n) (n <= 62), so the layers can be measured in ecbench's one unit.
    ///
    /// Produces `crater.json`, `neighbors.json` and `curves.json` (the identity of every
    /// neighbour: j-invariant, twist, modulus, order, generator) in `--out`.
    EcbenchSpec {
        #[arg(long)]
        n: u32,
        #[arg(long)]
        ell: u64,
        #[arg(long, default_value_t = false)]
        a2: bool,
        /// How many neighbours to include (spread over the Frobenius orbits).
        #[arg(long, default_value_t = 12)]
        neighbors: usize,
        #[arg(long, default_value_t = 8)]
        targets: usize,
        #[arg(long, default_value_t = 3)]
        rounds: usize,
        #[arg(long, default_value_t = 20261018)]
        seed: u64,
        #[arg(long)]
        out: String,
        #[arg(long)]
        threads: Option<usize>,
        #[arg(long, default_value_t = 16)]
        batch: usize,
    },
    /// Prime fields: build a tall volcano by CM, explore it exactly from the crater over the
    /// roots of Φ_ℓ mod p, and verify it (level sizes = class numbers, degrees, group structure,
    /// blind descent).  Optionally write ecbench specs for sampled vertices of every level.
    Explore {
        /// Class-number-one fundamental discriminant of the crater.
        #[arg(long, allow_hyphen_values = true, default_value_t = -7)]
        d: i64,
        #[arg(long)]
        ell: u64,
        /// Volcano depth k = v_ℓ(f); the floor holds about ℓ^k curves.
        #[arg(long)]
        depth: u32,
        /// Requested lower bound on v_ℓ(A): E[ℓ^m] is rational at the crater and the group
        /// structure varies with the level.
        #[arg(long, default_value_t = 0)]
        m: u32,
        #[arg(long, default_value_t = 40)]
        bits: u32,
        #[arg(long, default_value_t = 20261018)]
        seed: u64,
        /// Require #E = (smooth part below 2^S) × (one prime), so rho cost is set by a large prime.
        #[arg(long)]
        slack: Option<u32>,
        /// Vertices (spread over the levels) whose group structure is measured and checked.
        #[arg(long, default_value_t = 24)]
        verify: usize,
        /// Write `level,j,total,up,down,horizontal` for every vertex.
        #[arg(long)]
        vertices_out: Option<String>,
        /// Write ecbench specs (layers.json, vertices.json) for sampled vertices here.
        #[arg(long)]
        spec_dir: Option<String>,
        #[arg(long, default_value_t = 2)]
        per_level: usize,
        #[arg(long, default_value_t = 8)]
        targets: usize,
        #[arg(long, default_value_t = 3)]
        rounds: usize,
    },
    /// Prime fields, tall volcanoes: build a volcano by CM, verify its top levels by
    /// breadth-first search and its lower levels by random descents from the crater (degree rule
    /// at every step, blind descent from every sampled vertex, group structure of every sampled
    /// curve), and write ecbench specs with a set number of solves per level.
    Sample {
        /// Class-number-one fundamental discriminant of the crater.
        #[arg(long, allow_hyphen_values = true, default_value_t = -7)]
        d: i64,
        #[arg(long)]
        ell: u64,
        /// Volcano depth k = v_ℓ(f).
        #[arg(long)]
        depth: u32,
        /// Lower bound on v_ℓ(A) (group structure varies with the level when m > 0).
        #[arg(long, default_value_t = 0)]
        m: u32,
        #[arg(long, default_value_t = 46)]
        bits: u32,
        #[arg(long, default_value_t = 20261018)]
        seed: u64,
        /// Require #E = (smooth part below 2^S) × (one prime).
        #[arg(long)]
        slack: Option<u32>,
        /// Levels 0..=B are found completely by breadth-first search and their sizes checked
        /// against the class numbers.
        #[arg(long, default_value_t = 10)]
        bfs_levels: u32,
        /// Curves sampled per level (all of them on levels that have fewer).
        #[arg(long, default_value_t = 4)]
        per_level: usize,
        /// Solves (targets × rounds, summed over the level's curves) at the crater and the floor.
        #[arg(long, default_value_t = 300)]
        end_solves: usize,
        /// Solves at every other level.
        #[arg(long, default_value_t = 80)]
        level_solves: usize,
        /// Solves of each arm in the paired rho.negation / rho.plain control (floor curves).
        #[arg(long, default_value_t = 0)]
        control_solves: usize,
        /// Measured rounds per workload (each round is an independent walk).
        #[arg(long, default_value_t = 2)]
        rounds: usize,
        #[arg(long, default_value_t = 1)]
        warmup: usize,
        /// Directory for the specs (`spec_t<T>.json`, `control.json`) and `vertices.json`.
        #[arg(long)]
        spec_dir: Option<String>,
    },
    /// Blind descent to the floor of the ℓ-volcano: needs only p, the trace, ℓ and a start j.
    Descend {
        #[arg(long)]
        p: u64,
        #[arg(long, allow_hyphen_values = true)]
        trace: i64,
        #[arg(long)]
        ell: u64,
        #[arg(long)]
        j: u64,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
}

fn set_threads(t: Option<usize>) {
    if let Some(t) = t {
        let _ = rayon::ThreadPoolBuilder::new().num_threads(t).build_global();
    }
}

pub fn run(op: VolcanoOp) {
    match op {
        VolcanoOp::Profile { koblitz_n, a2, ecc2k130, q, trace, ell } => cmd_profile(koblitz_n, a2, ecc2k130, q, trace, ell),
        VolcanoOp::Level1 { d, ell, threads, out, mod2, batch } => cmd_level1(d, ell, threads, out, mod2, batch),
        VolcanoOp::KoblitzStep { n, ell, a2, verify, threads, out, batch } => cmd_koblitz_step(n, ell, a2, verify, threads, out, batch),
        VolcanoOp::Explore { d, ell, depth, m, bits, seed, slack, verify, vertices_out, spec_dir, per_level, targets, rounds } => {
            cmd_explore(d, ell, depth, m, bits, seed, slack, verify, vertices_out, spec_dir, per_level, targets, rounds)
        }
        VolcanoOp::Descend { p, trace, ell, j, seed } => cmd_descend(p, trace, ell, j, seed),
        VolcanoOp::Sample { d, ell, depth, m, bits, seed, slack, bfs_levels, per_level, end_solves, level_solves, control_solves, rounds, warmup, spec_dir } => {
            cmd_sample(d, ell, depth, m, bits, seed, slack, bfs_levels, per_level, end_solves, level_solves, control_solves, rounds, warmup, spec_dir)
        }
        VolcanoOp::EcbenchSpec { n, ell, a2, neighbors, targets, rounds, seed, out, threads, batch } => {
            cmd_ecbench_spec(n, ell, a2, neighbors, targets, rounds, seed, out, threads, batch)
        }
    }
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(2);
}

fn cmd_profile(koblitz_n: Option<u32>, a2: bool, ecc2k130: bool, q: Option<String>, trace: Option<String>, ell: Option<String>) {
    let (label, prof) = if ecc2k130 || koblitz_n.is_some() {
        let n = if ecc2k130 { 131 } else { koblitz_n.unwrap() };
        let (t, u) = binary::koblitz_trace(n, a2);
        let qq = BigUint::one() << n;
        let known = Some((-7i64, u.magnitude().clone()));
        (format!("Koblitz y² + xy = x³ + {}x² + 1 over F_(2^{n})", if a2 { "1·" } else { "0·" }), profile::analyze(qq, t, known))
    } else if let (Some(q), Some(t)) = (q, trace) {
        let qq: BigUint = q.parse().unwrap_or_else(|_| die("--q must be a decimal integer"));
        let tt: BigInt = t.parse().unwrap_or_else(|_| die("--trace must be a decimal integer"));
        (format!("prime field F_{qq}, trace {tt}"), profile::analyze(qq, tt, None))
    } else {
        die("give --koblitz-n N, --ecc2k130, or --q P --trace T");
    };
    let p = prof.unwrap_or_else(|e| die(&e));
    println!("# Volcano profile: {label}");
    println!();
    println!("field size q        {}", p.q);
    println!("trace t             {}", p.t);
    {
        let (nf, ncof) = profile::factor(&p.n_order);
        let s: Vec<String> = nf.iter().map(|(l, e)| if *e > 1 { format!("{l}^{e}") } else { l.to_string() }).collect();
        println!("#E(F_q) = q+1-t     {} = {}{}", p.n_order, s.join(" · "), ncof.map(|c| format!(" · [unfactored {c}]")).unwrap_or_default());
    }
    println!("Δ = t² − 4q         {}", p.delta);
    println!("D_K                 {}   (h(O_K) = {})", p.d_k, p.h_k.map(|h| h.to_string()).unwrap_or_else(|| "not computed".into()));
    let fac: Vec<String> = p.f_factors.iter().map(|(l, e)| if *e > 1 { format!("{l}^{e}") } else { l.to_string() }).collect();
    println!("conductor f         {} = {}", p.f, if fac.is_empty() { "1".into() } else { fac.join(" · ") });
    if let Some(c) = &p.f_unfactored {
        println!("  (unfactored composite part: {c}; f is a lower bound, level tables are suppressed)");
    }
    println!();
    println!("Per prime dividing f (volcano depth = v_ℓ(f); all ℓ+1 kernels are rational when depth ≥ 1):");
    println!("{:>22} {:>6} {:>6} {:>12} {:>26} {:>28}", "ℓ", "depth", "(D/ℓ)", "horizontal", "descending", "h(O_ℓ) = level-1 size");
    for r in profile::prime_rows(&p) {
        println!(
            "{:>22} {:>6} {:>6} {:>12} {:>26} {:>28}",
            r.ell.to_string(),
            r.depth,
            r.kron,
            r.crater_horizontal,
            r.crater_descending.to_string(),
            r.h_floor_over_crater.map(|h| h.to_string()).unwrap_or_else(|| "-".into())
        );
    }
    if let Some(e) = ell {
        let l: BigUint = e.parse().unwrap_or_else(|_| die("--ell must be an integer"));
        for cc in profile::crater_conductors(&p, &l) {
            println!();
            match profile::level_table(&p, &l, &cc) {
                Err(e) => println!("level table: {e}"),
                Ok(rows) => {
                    let note = if cc.is_one() { "  [contains the Koblitz curve at level 0]" } else { "" };
                    println!("{l}-volcano with crater conductor {cc}{note}   (E(F_q) ≅ Z/n1 × Z/n2, n1 | n2):");
                    println!("{:>5} {:>30} {:>30} {:>16} {:>40}", "level", "conductor", "curves at level", "n1", "n2");
                    for r in rows {
                        println!(
                            "{:>5} {:>30} {:>30} {:>16} {:>40}",
                            r.level,
                            r.conductor.to_string(),
                            r.n_curves.map(|h| h.to_string()).unwrap_or_else(|| "-".into()),
                            r.n1.to_string(),
                            r.n2.to_string()
                        );
                    }
                }
            }
        }
    }
}

fn cmd_level1(d: i64, ell: u64, threads: Option<usize>, out: Option<String>, mod2: bool, batch: usize) {
    set_threads(threads);
    let t0 = Instant::now();
    let (coeffs, rep) = level1::phi_at_crater(d, ell, batch, true).unwrap_or_else(|e| die(&e));
    println!("# Φ_{ell}(X, j₀) over Z, j₀ = j(O_K), D = {d} (j₀ = {})", level1::class_one_j(d).unwrap());
    println!("degree              {}", coeffs.len() - 1);
    println!("primes used         {}  (+{} confirming)", rep.primes_used, rep.primes_confirming);
    println!("max coefficient     {} bits", rep.max_coeff_bits);
    println!("time                {:.1} s", t0.elapsed().as_secs_f64());
    if let Some(path) = out {
        let body: Vec<String> = coeffs.iter().map(|c| c.to_string()).collect();
        std::fs::write(&path, body.join("\n") + "\n").unwrap_or_else(|e| die(&format!("{path}: {e}")));
        println!("wrote               {path}");
    }
    if mod2 {
        let bits = level1::mod2(&coeffs);
        let f = f2poly::F2Poly::from_bits(&bits);
        let t1 = Instant::now();
        // report the factor structure over F_2 (all degrees up to ell + 1)
        let rf = f2poly::rational_factors(&f, (ell + 1) as usize);
        let mut by_deg: std::collections::BTreeMap<(usize, u64), usize> = Default::default();
        for (_, d, m) in &rf.factors {
            *by_deg.entry((*d, *m)).or_default() += 1;
        }
        println!("Φ mod 2 factors over F_2 ({:.2} s):", t1.elapsed().as_secs_f64());
        for ((deg, mult), cnt) in by_deg {
            println!("  {cnt} irreducible factor(s) of degree {deg}, multiplicity {mult}");
        }
        if rf.leftover_deg > 0 {
            println!("  unresolved degree {}", rf.leftover_deg);
        }
    }
}

fn cmd_koblitz_step(n: u32, ell: u64, a2: bool, verify: usize, threads: Option<usize>, out: Option<String>, batch: usize) {
    set_threads(threads);
    if n % 2 == 0 {
        die("odd n only (half-trace is used to build points)");
    }
    let t_all = Instant::now();
    let (t, u) = binary::koblitz_trace(n, a2);
    let f = u.magnitude().clone();
    let big_ell = BigUint::from(ell);
    if !(&f % &big_ell).is_zero() {
        die(&format!("ℓ = {ell} does not divide the conductor f = {f}: no vertical isogenies at this ℓ"));
    }
    let kron = profile::kronecker(&BigInt::from(-7), &big_ell);
    let h = profile::class_number_of_order(-7, 1, &big_ell);
    let horiz = 1 + kron;
    println!("# Koblitz step: y² + xy = x³ + {}x² + 1 over F_(2^{n}), ℓ = {ell}", if a2 { 1 } else { 0 });
    println!("t_n = {t},  conductor f = {f},  (−7/ℓ) = {kron}");
    println!("expected: {horiz} horizontal (back to j = 1) + {} descending = {} kernels; level 1 holds h(O_ℓ) = {h} curves", ell as i64 + 1 - horiz as i64, ell + 1);
    println!();
    println!("[1/4] Φ_ℓ(X, j₀) over Z by CRT (D = −7, j₀ = −3375)");
    let (coeffs, rep) = level1::phi_at_crater(-7, ell, batch, true).unwrap_or_else(|e| die(&e));
    println!("      {} primes, coefficients up to {} bits, {:.1} s", rep.primes_used, rep.max_coeff_bits, rep.seconds);
    println!("[2/4] reduce mod 2 and factor over F_2");
    let bits = level1::mod2(&coeffs);
    let phi2 = f2poly::F2Poly::from_bits(&bits);
    let t1 = Instant::now();
    let rf = f2poly::rational_factors(&phi2, n as usize);
    println!("      {:.2} s; leftover (non-rational) degree {}", t1.elapsed().as_secs_f64(), rf.leftover_deg);
    let mut one_mult = 0u64;
    let mut orbit_facs: Vec<&f2poly::F2Poly> = Vec::new();
    let mut orbit_deg_hist: std::collections::BTreeMap<usize, usize> = Default::default();
    for (g, d, m) in &rf.factors {
        if *d == 1 && g.coeff(0) && *m >= 1 && g.deg() == Some(1) {
            one_mult += *m; // X + 1  ->  j = 1
            continue;
        }
        if *m != 1 {
            println!("      warning: factor of degree {d} with multiplicity {m}");
        }
        orbit_facs.push(g);
        *orbit_deg_hist.entry(*d).or_default() += 1;
    }
    println!("      root j = 1 (the crater) with multiplicity {one_mult}  (expected {horiz})");
    for (d, c) in &orbit_deg_hist {
        println!("      {c} Frobenius orbit(s) of size {d}");
    }
    let total_roots: usize = orbit_facs.iter().map(|g| g.deg().unwrap()).sum();
    println!("      distinct level-1 roots in F_(2^{n}): {total_roots}  (expected {h})");
    println!("[3/4] roots in F_(2^{n})");
    let irr = binary::find_irreducible(n);
    let t2 = Instant::now();
    let orbits: Vec<Vec<F2mElement>> = orbit_facs
        .par_iter()
        .enumerate()
        .map(|(i, g)| {
            let mut rng = 0xC0FFEEu64 ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15);
            binary::orbit_of(g, n, &irr, &mut rng)
        })
        .collect();
    let js: Vec<F2mElement> = orbits.into_iter().flatten().collect();
    let mut seen = std::collections::HashSet::new();
    let distinct = js.iter().filter(|j| seen.insert(j.to_biguint())).count();
    println!("      {} roots, {} distinct, {:.1} s", js.len(), distinct, t2.elapsed().as_secs_f64());
    println!("[4/4] verify {} neighbours: build the curve over F_(2^{n}) and test [N]P = O, N = 2^n + 1 − t_n", verify.min(js.len()));
    let big_n = binary::koblitz_order(n, a2);
    let step = (js.len() / verify.max(1)).max(1);
    let sample: Vec<&F2mElement> = js.iter().step_by(step).take(verify).collect();
    let results: Vec<(bool, bool)> = sample
        .par_iter()
        .enumerate()
        .map(|(i, j)| {
            let mut rng = 0xBADC0DEu64 ^ (i as u64).wrapping_mul(0x2545F4914F6CDD1D);
            let c0 = binary::curve_for_j(j, false, n, &irr);
            let c1 = binary::curve_for_j(j, true, n, &irr);
            (binary::order_test(&c0, &big_n, 5, &mut rng), binary::order_test(&c1, &big_n, 5, &mut rng))
        })
        .collect();
    let exactly_one = results.iter().filter(|(a, b)| a ^ b).count();
    println!("      {exactly_one} of {} neighbours have exactly one twist of order N (all must)", results.len());
    // control: random j in the field should essentially never pass
    let mut rng = 0xFEEDu64;
    let mut false_pos = 0;
    for _ in 0..8 {
        let mut w = vec![0u64; (n as usize).div_ceil(64)];
        for x in w.iter_mut() {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *x = rng;
        }
        let jr = F2mElement::from_words(&w, n);
        if jr.is_zero() {
            continue;
        }
        let c0 = binary::curve_for_j(&jr, false, n, &irr);
        let c1 = binary::curve_for_j(&jr, true, n, &irr);
        if binary::order_test(&c0, &big_n, 3, &mut rng) || binary::order_test(&c1, &big_n, 3, &mut rng) {
            false_pos += 1;
        }
    }
    println!("      control: {false_pos} of 8 random j-invariants pass the same test (expect 0)");
    if let Some(path) = out {
        let body: Vec<String> = js.iter().map(|j| j.to_biguint().to_str_radix(16)).collect();
        std::fs::write(&path, body.join("\n") + "\n").unwrap_or_else(|e| die(&format!("{path}: {e}")));
        println!("wrote {} j-invariants to {path}", body.len());
    }
    let ok = one_mult == horiz as u64 && distinct == js.len() && BigUint::from(distinct) == h && exactly_one == results.len() && false_pos == 0 && rf.leftover_deg == 0;
    println!();
    println!("total {:.1} s — {}", t_all.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASS" } else { "A CHECK FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

fn cmd_ecbench_spec(n: u32, ell: u64, a2: bool, neighbors: usize, targets: usize, rounds: usize, seed: u64, out: String, threads: Option<usize>, batch: usize) {
    use crypto_lib::binary_ecc::curve::{scalar_mul, BinaryPoint};
    set_threads(threads);
    if !(3..=62).contains(&n) || n % 2 == 0 {
        die("ecbench binary_explicit takes odd 3 <= n <= 62");
    }
    let (t, u) = binary::koblitz_trace(n, a2);
    let big_ell = BigUint::from(ell);
    if !(u.magnitude() % &big_ell).is_zero() {
        die(&format!("ℓ = {ell} does not divide the conductor"));
    }
    let big_n = binary::koblitz_order(n, a2);
    let (nf, ncof) = profile::factor(&big_n);
    if ncof.is_some() {
        die("group order not fully factored");
    }
    let r = nf.iter().map(|(p, _)| p.clone()).max().unwrap();
    let cof = &big_n / &r;
    if cof > BigUint::from(1u64 << 24) {
        die("no usable prime-order subgroup: cofactor above 2^24");
    }
    let (coeffs, rep) = level1::phi_at_crater(-7, ell, batch, false).unwrap_or_else(|e| die(&e));
    let bits = level1::mod2(&coeffs);
    let rf = f2poly::rational_factors(&f2poly::F2Poly::from_bits(&bits), n as usize);
    let irr = binary::find_irreducible(n);
    let facs: Vec<&f2poly::F2Poly> = rf.factors.iter().filter(|(g, _, _)| !(g.deg() == Some(1))).map(|(g, _, _)| g).collect();
    let js: Vec<F2mElement> = facs
        .par_iter()
        .enumerate()
        .map(|(i, g)| {
            let mut rng = 0xC0FFEEu64 ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15);
            binary::orbit_of(g, n, &irr, &mut rng)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();
    eprintln!("{} level-1 neighbours found ({} primes, {:.1} s)", js.len(), rep.primes_used, rep.seconds);
    let modulus: u64 = irr.low_terms.iter().fold(1u64 << n, |v, &term| v | (1u64 << term));
    let step = (js.len() / neighbors.max(1)).max(1);
    let mut rng = seed ^ 0x5EED;
    let mut curves = Vec::new();
    let mut specs = Vec::new();
    for j in js.iter().step_by(step).take(neighbors) {
        // the twist whose order is N
        let mut chosen = None;
        for twist in [false, true] {
            let c = binary::curve_for_j(j, twist, n, &irr);
            if binary::order_test(&c, &big_n, 6, &mut rng) {
                chosen = Some((twist, c));
                break;
            }
        }
        let (twist, curve) = chosen.unwrap_or_else(|| die("a neighbour has no twist of order N"));
        // generator of the prime-order subgroup
        let g = loop {
            let p = binary::random_point(&curve, &mut rng);
            let q = scalar_mul(&curve, &p, &cof);
            if q != BinaryPoint::Infinity {
                break q;
            }
        };
        if scalar_mul(&curve, &g, &r) != BinaryPoint::Infinity {
            die("generator does not have order r");
        }
        let (gx, gy) = match &g {
            BinaryPoint::Affine { x, y } => (x.to_biguint(), y.to_biguint()),
            BinaryPoint::Infinity => unreachable!(),
        };
        let to_u64 = |v: &BigUint| -> u64 { v.to_u64_digits().first().copied().unwrap_or(0) };
        let a_u64 = if twist { 1u64 } else { 0u64 };
        let b_u64 = to_u64(&curve.b.to_biguint());
        specs.push(serde_json::json!({
            "kind": "binary_explicit", "n": n, "modulus": modulus, "a": a_u64, "b": b_u64,
            "group_order": to_u64(&big_n), "r": to_u64(&r), "gx": to_u64(&gx), "gy": to_u64(&gy)
        }));
        curves.push(serde_json::json!({
            "j_hex": j.to_biguint().to_str_radix(16), "a2": a_u64, "a6": b_u64, "gx": to_u64(&gx), "gy": to_u64(&gy)
        }));
    }
    std::fs::create_dir_all(&out).unwrap_or_else(|e| die(&format!("{out}: {e}")));
    let measurement = serde_json::json!({"rounds": rounds, "warmup": 1, "order": "alternate", "seed": seed % 1000, "isolation_required": "L0", "timeout_seconds": 600});
    let label = format!("Koblitz K_{} over F_2^{n} and its {} level-1 neighbours (ℓ = {ell})", if a2 { 1 } else { 0 }, specs.len());
    let crater = serde_json::json!({
        "schema": "ecbench.spec/v1",
        "label": format!("crater: {label}"),
        "description": "The Koblitz curve (End = O_K, conductor 1). Generated by `crypto volcano ecbench-spec`; counted operations only.",
        "workloads": {"curves": [{"kind": "koblitz", "a": if a2 {1} else {0}, "n": n}], "targets_per_curve": targets * 4, "target_seed": seed},
        "arms": [
            {"name": "rho-frob-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
            {"name": "rho-neg", "role": "baseline", "method": {"id": "rho.negation"}},
            {"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}},
            {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
        ],
        "measurement": measurement
    });
    let nb = serde_json::json!({
        "schema": "ecbench.spec/v1",
        "label": format!("level-1 neighbours: {label}"),
        "description": "Curves with End of conductor ℓ in the isogeny class of the Koblitz curve, from Φ_ℓ(X, j₀) by CRT, each checked by [N]P = O. Generated by `crypto volcano ecbench-spec`; counted operations only.",
        "workloads": {"curves": specs, "targets_per_curve": targets, "target_seed": seed},
        "arms": [
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}},
            {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
        ],
        "measurement": measurement
    });
    let ident = serde_json::json!({
        "n": n, "ell": ell, "a2_of_crater": a2, "trace": t.to_string(), "group_order": big_n.to_string(),
        "r": r.to_string(), "cofactor": cof.to_string(), "modulus": modulus, "neighbours": curves
    });
    for (name, v) in [("crater.json", &crater), ("neighbors.json", &nb), ("curves.json", &ident)] {
        let path = format!("{out}/{name}");
        std::fs::write(&path, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap_or_else(|e| die(&format!("{path}: {e}")));
        println!("wrote {path}");
    }
    println!("run:  ecbench run --wait --cpus inherit --spec {out}/crater.json --out <session-dir-1>");
    println!("      ecbench run --wait --cpus inherit --spec {out}/neighbors.json --out <session-dir-2>");
}

#[allow(clippy::too_many_arguments)]
fn cmd_explore(d: i64, ell: u64, depth: u32, m: u32, bits: u32, seed: u64, slack: Option<u32>, verify: usize, vertices_out: Option<String>, spec_dir: Option<String>, per_level: usize, targets: usize, rounds: usize) {
    use std::collections::BTreeMap;
    let t0 = Instant::now();
    let inst = fp::find_instance(ell, d, depth, m, bits, seed, slack).unwrap_or_else(|e| die(&e));
    let n = inst.order();
    let (nf, _) = profile::factor(&BigUint::from(n));
    let nfs: Vec<String> = nf.iter().map(|(l, e)| if *e > 1 { format!("{l}^{e}") } else { l.to_string() }).collect();
    println!("# Prime-field volcano: D = {d}, ℓ = {ell}, depth {depth}, m = {m}");
    println!("p = {} ({} bits), trace t = {}, #E = {} = {}", inst.p, 64 - inst.p.leading_zeros(), inst.t, n, nfs.join(" · "));
    println!("t² − 4p = {} · ({})²,  f = ℓ^k = {}", d, inst.conductor(), inst.conductor());
    let ctx = fp::Ctx::new(inst.p, ell).unwrap_or_else(|e| die(&e));
    let vol = fp::explore(&inst, &ctx, 20_000_000).unwrap_or_else(|e| die(&e));
    let got = fp::level_sizes(&vol, depth);
    let want = fp::expected_level_sizes(&inst);
    println!("\nlevel sizes (BFS distance from the crater) against the class numbers h(O_(ℓ^i)):");
    let mut ok_levels = true;
    for (i, (g, w)) in got.iter().zip(&want).enumerate() {
        let eq = BigUint::from(*g) == *w;
        ok_levels &= eq;
        println!("  level {i:>2}: {g:>10}   expected {w:>10}  {}", if eq { "ok" } else { "MISMATCH" });
    }
    let viol = fp::degree_violations(&inst, &vol);
    println!("vertices: {}; rational-kernel rule (ℓ+1 off the floor, 1 on it): {} violations", vol.level.len(), viol);
    // group structure at a spread of vertices
    let mut rng = Rng(seed ^ 0xD1CE);
    let mut by_level: BTreeMap<u32, Vec<u64>> = BTreeMap::new();
    for &j in &vol.order {
        by_level.entry(vol.level[&j]).or_default().push(j);
    }
    let mut checked = 0;
    let mut bad_group = 0;
    let fac = fp::group_order_factors(n);
    'outer: loop {
        for (&l, js) in &by_level {
            if checked >= verify {
                break 'outer;
            }
            let j = js[(rng.next() as usize) % js.len()];
            let ec = fp::curve_of(&inst, j, &mut rng).unwrap_or_else(|e| die(&e));
            let got = fp::group_structure(&ec, n, &fac, &mut rng).unwrap_or_else(|e| die(&e));
            let want = fp::predicted_structure(&inst, l).unwrap_or_else(|e| die(&e));
            if got != want {
                bad_group += 1;
                println!("  group mismatch at level {l}: measured {got:?}, predicted {want:?}");
            }
            checked += 1;
        }
        if by_level.is_empty() {
            break;
        }
    }
    println!("group structure Z/n1 × Z/n2 measured from point orders at {checked} vertices: {bad_group} differ from O_d/(π−1)");
    let mut bad_desc = 0;
    let trials = verify.min(vol.order.len());
    for i in 0..trials {
        let j = vol.order[(i * 7919) % vol.order.len()];
        match fp::blind_descent(&ctx, depth, j, &mut rng) {
            Ok(ds) => {
                let floor = *ds.path.last().unwrap();
                if vol.level[&floor] != depth || ds.start_level != vol.level[&j] || ds.path.len() as u32 - 1 != depth - vol.level[&j] {
                    bad_desc += 1;
                }
            }
            Err(_) => bad_desc += 1,
        }
    }
    println!("blind descent from {trials} vertices (p, t, ℓ and j only): {bad_desc} disagree with the BFS truth");
    if let Some(path) = vertices_out {
        let mut body = String::from("level,j,rational_kernels,up,down,horizontal\n");
        for &j in &vol.order {
            let l = vol.level[&j];
            let (mut up, mut down, mut hor) = (0, 0, 0);
            for &(s, dst, e) in vol.edges.iter().filter(|x| x.0 == j) {
                let _ = s;
                let ld = vol.level[&dst];
                if ld == l {
                    hor += e;
                } else if ld > l {
                    down += e;
                } else {
                    up += e;
                }
            }
            body.push_str(&format!("{l},{j},{},{up},{down},{hor}\n", vol.totals[&j]));
        }
        std::fs::write(&path, body).unwrap_or_else(|e| die(&format!("{path}: {e}")));
        println!("wrote {path}");
    }
    if let Some(dir) = spec_dir {
        std::fs::create_dir_all(&dir).unwrap_or_else(|e| die(&format!("{dir}: {e}")));
        let mut curves = Vec::new();
        let mut meta = Vec::new();
        for (&l, js) in &by_level {
            let step = (js.len() / per_level.max(1)).max(1);
            for &j in js.iter().step_by(step).take(per_level) {
                let vc = fp::vertex_curve(&inst, l, j, &mut rng).unwrap_or_else(|e| die(&e));
                curves.push(serde_json::json!({"kind": "prime_explicit", "p": inst.p, "a": vc.a, "b": vc.b, "group_order": vc.group_order, "r": vc.r, "gx": vc.gx, "gy": vc.gy}));
                meta.push(serde_json::json!({"level": vc.level, "j": vc.j, "a": vc.a, "b": vc.b, "n1": vc.n1, "n2": vc.n2, "r": vc.r, "cofactor": vc.group_order / vc.r}));
            }
        }
        let spec = serde_json::json!({
            "schema": "ecbench.spec/v1",
            "label": format!("layers of the {ell}-volcano, D = {d}, depth {depth}, p = {}", inst.p),
            "description": "Sampled vertices of every level, built by `crypto volcano explore`; counted operations only.",
            "workloads": {"curves": curves, "targets_per_curve": targets, "target_seed": seed},
            "arms": [
                {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
                {"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}},
                {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
            ],
            "measurement": {"rounds": rounds, "warmup": 1, "order": "alternate", "seed": seed % 1000, "isolation_required": "L0", "timeout_seconds": 600}
        });
        for (name, v) in [("layers.json", &spec), ("vertices.json", &serde_json::json!({"p": inst.p, "t": inst.t, "ell": ell, "d": d, "depth": depth, "m": m, "vertices": meta}))] {
            let path = format!("{dir}/{name}");
            std::fs::write(&path, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap_or_else(|e| die(&format!("{path}: {e}")));
            println!("wrote {path}");
        }
    }
    let ok = ok_levels && viol == 0 && bad_group == 0 && bad_desc == 0;
    println!("\ntotal {:.1} s — {}", t0.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASS" } else { "A CHECK FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_arguments)]
fn cmd_sample(
    d: i64,
    ell: u64,
    depth: u32,
    m: u32,
    bits: u32,
    seed: u64,
    slack: Option<u32>,
    bfs_levels: u32,
    per_level: usize,
    end_solves: usize,
    level_solves: usize,
    control_solves: usize,
    rounds: usize,
    warmup: usize,
    spec_dir: Option<String>,
) {
    use std::collections::{BTreeMap, BTreeSet};
    let t0 = Instant::now();
    if rounds == 0 || per_level == 0 {
        die("--rounds and --per-level must be positive");
    }
    let inst = fp::find_instance(ell, d, depth, m, bits, seed, slack).unwrap_or_else(|e| die(&e));
    let n = inst.order();
    let (nf, _) = profile::factor(&BigUint::from(n));
    let nfs: Vec<String> = nf.iter().map(|(l, e)| if *e > 1 { format!("{l}^{e}") } else { l.to_string() }).collect();
    println!("# Prime-field volcano (sampled): D = {d}, ℓ = {ell}, depth {depth}, m = {m}");
    println!("p = {} ({} bits), trace t = {}, #E = {} = {}", inst.p, 64 - inst.p.leading_zeros(), inst.t, n, nfs.join(" · "));
    println!("t² − 4p = {} · ({})²,  f = ℓ^k", d, inst.conductor());
    let ctx = fp::Ctx::new(inst.p, ell).unwrap_or_else(|e| die(&e));
    let want = fp::expected_level_sizes(&inst);
    let top = bfs_levels.min(depth);
    let vol = fp::explore_to(&inst, &ctx, top, 20_000_000).unwrap_or_else(|e| die(&e));
    let got = fp::level_sizes(&vol, top);
    let mut ok = true;
    println!("\nlevels 0..={top} by breadth-first search, against the class numbers h(O_(ℓ^i)):");
    for i in 0..=top as usize {
        let eq = BigUint::from(got[i]) == want[i];
        ok &= eq;
        println!("  level {i:>2}: {:>10}   expected {:>10}  {}", got[i], want[i], if eq { "ok" } else { "MISMATCH" });
    }
    let viol = vol.totals.iter().filter(|(j, &t)| t != if vol.level[*j] < depth { ell as u32 + 1 } else { 1 }).count();
    ok &= viol == 0;
    println!("degree rule at the {} expanded vertices: {viol} violations", vol.totals.len());
    for i in top as usize + 1..=depth as usize {
        println!("  level {i:>2}: (sampled)  expected {:>10}", want[i]);
    }
    // sample vertices: from the BFS sets above `top`, by random descents below it
    let mut rng = Rng(seed ^ 0x5A3B1E);
    let mut levels: BTreeMap<u32, Vec<u64>> = BTreeMap::new();
    for &j in &vol.order {
        levels.entry(vol.level[&j]).or_default().push(j);
    }
    let mut chosen: BTreeMap<u32, Vec<u64>> = BTreeMap::new();
    for (&l, js) in &levels {
        let mut pool = js.clone();
        let take = per_level.min(pool.len());
        let mut pick = Vec::new();
        for _ in 0..take {
            let i = (rng.next() as usize) % pool.len();
            pick.push(pool.swap_remove(i));
        }
        chosen.insert(l, pick);
    }
    let j0 = fp::crater_j(&inst);
    let mut descents = 0usize;
    let mut bad_descent = 0usize;
    if depth > top {
        let mut seen: BTreeMap<u32, BTreeSet<u64>> = BTreeMap::new();
        while (top + 1..=depth).any(|l| seen.get(&l).map_or(0, |s| s.len()) < per_level) {
            descents += 1;
            if descents > 50 * per_level + 50 {
                die("random descents keep revisiting the same vertices");
            }
            match fp::random_descent(&ctx, j0, depth, &mut rng) {
                Ok(path) => {
                    for (l, &(j, _)) in path.iter().enumerate() {
                        let l = l as u32;
                        if l <= top {
                            if vol.level.get(&j) != Some(&l) {
                                bad_descent += 1;
                            }
                            continue;
                        }
                        let s = seen.entry(l).or_default();
                        if s.len() < per_level && s.insert(j) {
                            chosen.entry(l).or_default().push(j);
                        }
                    }
                }
                Err(e) => {
                    println!("  descent failed: {e}");
                    bad_descent += 1;
                }
            }
        }
    }
    ok &= bad_descent == 0;
    println!("random descents from the crater: {descents} (degree rule at every step; levels inside the BFS range agree): {bad_descent} failures");
    // independent level check: blind descent from every sampled vertex below the crater
    let mut bad_blind = 0usize;
    let mut blind = 0usize;
    for (&l, js) in &chosen {
        for &j in js {
            if l == 0 {
                continue;
            }
            blind += 1;
            match fp::blind_descent(&ctx, depth, j, &mut rng) {
                Ok(ds) if ds.start_level == l && ds.path.len() as u32 - 1 == depth - l => {}
                _ => bad_blind += 1,
            }
        }
    }
    ok &= bad_blind == 0;
    println!("blind descent (p, t, ℓ and j only) from {blind} sampled vertices: {bad_blind} give a level other than the sampled one");
    // curves, group structures, specs
    let mut metas = Vec::new();
    let mut by_targets: BTreeMap<usize, Vec<(serde_json::Value, usize)>> = BTreeMap::new();
    let mut bad_group = 0usize;
    let mut floor_curves = Vec::new();
    let mut r_all = BTreeSet::new();
    for (&l, js) in &chosen {
        let solves = if l == 0 || l == depth { end_solves } else { level_solves };
        let targets = solves.div_ceil(js.len() * rounds).max(1);
        for &j in js {
            let vc = fp::vertex_curve(&inst, l, j, &mut rng).unwrap_or_else(|e| die(&e));
            let pred = fp::predicted_structure(&inst, l).unwrap_or_else(|e| die(&e));
            if (vc.n1, vc.n2) != pred {
                bad_group += 1;
                println!("  group mismatch at level {l}: measured ({}, {}), predicted {pred:?}", vc.n1, vc.n2);
            }
            r_all.insert(vc.r);
            let cs = serde_json::json!({"kind": "prime_explicit", "p": inst.p, "a": vc.a, "b": vc.b, "group_order": vc.group_order, "r": vc.r, "gx": vc.gx, "gy": vc.gy});
            if l == depth {
                floor_curves.push(cs.clone());
            }
            metas.push(serde_json::json!({"level": l, "j": j, "a": vc.a, "b": vc.b, "n1": vc.n1, "n2": vc.n2, "r": vc.r, "cofactor": vc.group_order / vc.r, "spec": format!("spec_t{targets}.json")}));
            by_targets.entry(targets).or_default().push((cs, l as usize));
        }
    }
    ok &= bad_group == 0 && r_all.len() == 1;
    let r = *r_all.iter().next().unwrap();
    println!("group structure of {} sampled curves (from point orders) against O_d/(π−1): {bad_group} differ; prime subgroup r = {r} ({:.1} bits), common to all: {}", metas.len(), (r as f64).log2(), r_all.len() == 1);
    if let Some(dir) = spec_dir {
        std::fs::create_dir_all(&dir).unwrap_or_else(|e| die(&format!("{dir}: {e}")));
        let label = format!("{ell}-volcano, D = {d}, depth {depth}, m = {m}, p = {}", inst.p);
        let measurement = serde_json::json!({"rounds": rounds, "warmup": warmup, "order": "alternate", "seed": seed % 1000, "isolation_required": "L0", "timeout_seconds": 3600});
        let mut write = |name: &str, v: &serde_json::Value| {
            let path = format!("{dir}/{name}");
            std::fs::write(&path, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap_or_else(|e| die(&format!("{path}: {e}")));
            println!("wrote {path}");
        };
        for (t, cs) in &by_targets {
            let levels: BTreeSet<usize> = cs.iter().map(|x| x.1).collect();
            let spec = serde_json::json!({
                "schema": "ecbench.spec/v1",
                "label": format!("{label}: levels {levels:?}, {t} targets per curve"),
                "description": "Sampled vertices of the levels named in the label, built and verified by `crypto volcano sample`; counted operations only.",
                "workloads": {"curves": cs.iter().map(|x| x.0.clone()).collect::<Vec<_>>(), "targets_per_curve": t, "target_seed": seed},
                "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}],
                "measurement": measurement
            });
            write(&format!("spec_t{t}.json"), &spec);
        }
        if control_solves > 0 && !floor_curves.is_empty() {
            let t = control_solves.div_ceil(floor_curves.len() * rounds).max(1);
            let spec = serde_json::json!({
                "schema": "ecbench.spec/v1",
                "label": format!("{label}: positive control on the floor curves, {t} targets per curve"),
                "description": "rho.negation against rho.plain on the same workloads and seeds: the difference the level study must be able to see.",
                "workloads": {"curves": floor_curves, "targets_per_curve": t, "target_seed": seed ^ 0xC0},
                "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}, {"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}}],
                "measurement": measurement
            });
            write("control.json", &spec);
        }
        write("vertices.json", &serde_json::json!({"p": inst.p, "t": inst.t, "ell": ell, "d": d, "depth": depth, "m": m, "r": r, "bfs_levels": top, "level_sizes": want.iter().map(|w| w.to_string()).collect::<Vec<_>>(), "vertices": metas}));
    }
    println!("\ntotal {:.1} s — {}", t0.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASS" } else { "A CHECK FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

fn cmd_descend(p: u64, trace: i64, ell: u64, j: u64, seed: u64) {
    let prof = profile::analyze(BigUint::from(p), BigInt::from(trace), None).unwrap_or_else(|e| die(&e));
    let k = prof.f_factors.iter().find(|(l, _)| *l == BigUint::from(ell)).map(|(_, v)| *v).unwrap_or(0);
    println!("p = {p}, t = {trace}: D_K = {}, f = {}, depth of the {ell}-volcano k = {k}", prof.d_k, prof.f);
    let ctx = fp::Ctx::new(p, ell).unwrap_or_else(|e| die(&e));
    let mut rng = Rng(seed | 1);
    let t0 = Instant::now();
    let ds = fp::blind_descent(&ctx, k, j, &mut rng).unwrap_or_else(|e| die(&e));
    println!("start j = {j} is at level {} of {k}  (End conductor {} · {ell}^{})", ds.start_level, prof.f.clone() / BigUint::from(ell).pow(k), ds.start_level);
    println!("path to the floor ({} steps, {:.3} s):", ds.path.len() - 1, t0.elapsed().as_secs_f64());
    for (i, v) in ds.path.iter().enumerate() {
        println!("  {i:>3}  j = {v}");
    }
    println!("floor j = {}", ds.path.last().unwrap());
}
