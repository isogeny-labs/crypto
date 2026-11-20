//! **End-to-end binary index calculus: `S₄` pairs-and-solve against the
//! factored `S₃·S₃` oracle, through to the discrete logarithm.**
//!
//! For each odd `n` this runs the boundary ledger's char-2 instance
//! (`ic_boundary::run_char2_instance`): a random curve `y² + xy = x³ + ax² + b`
//! over `F_{2ⁿ}` with cofactor ≤ 8, the low-order subspace base of
//! dimension `⌈n/3⌉`, and every variant through the same relation loop,
//! linear algebra and verification, `repeats` times on shared targets,
//! with matched rho walks as the reference.  The two arms compared here are
//! `semaev_s4_pairs_and_solve_m3` (`SubspaceOracle`) and
//! `semaev_s4_factored_m3` (`FactoredSubspaceOracle`); the meet-in-the-middle
//! rows are printed for context.
//!
//! Before each instance runs, the two oracles are checked for identical
//! decisions on random and planted targets, with every witness verified.
//!
//! ```bash
//! cargo run --release --example binary_factored_ic_e2e -- [out.jsonl] [repeats] [n ...]
//! ```
use crypto_lib::cryptanalysis::binary_radical_root::FactoredSubspaceOracle;
use crypto_lib::cryptanalysis::ic_boundary::{
    random_binary_instance, run_char2_instance, BoundaryConfig,
};
use crypto_lib::cryptanalysis::semaev_decomp::{
    eval_s4_general, quartic_in_x3_general, SubspaceOracle,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::BTreeMap;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "binary_factored_ic_e2e.jsonl".into());
    let repeats: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);
    let ns: Vec<u32> = if args.len() > 3 {
        args[3..]
            .iter()
            .map(|s| s.parse().expect("degree"))
            .collect()
    } else {
        vec![15, 17, 19, 21, 23, 25, 27]
    };
    let cfg = BoundaryConfig {
        repeats,
        s4_max_degree: 31,
        ..BoundaryConfig::default()
    };
    let mut out = std::fs::File::create(&out_path).expect("output file");
    println!(
        "config: repeats={repeats} rho_repeats={} seed={:#x}",
        cfg.rho_repeats, cfg.seed
    );

    for &n in &ns {
        let Some(inst) = random_binary_instance(n, cfg.seed, 8) else {
            println!("n={n}: no instance");
            continue;
        };
        // ── Pre-flight: identical decisions on this instance's base. ──
        let l = n.div_ceil(3);
        let basis: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
        let base = SubspaceOracle::new(&basis, inst.b, &inst.gf);
        let Some(fac) = FactoredSubspaceOracle::new(&basis, inst.b, &inst.gf) else {
            println!("n={n}: factored oracle needs odd n");
            continue;
        };
        let mut rng = StdRng::seed_from_u64(0xe2e ^ n as u64);
        let (mut agree, mut yes, mut checked) = (true, 0, 0);
        for i in 0..400 {
            let xr = if i % 2 == 0 {
                let pick = |rng: &mut StdRng| base.span[rng.gen_range(0..base.span.len())];
                let (a, b, c) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
                let q = quartic_in_x3_general(a, b, c, inst.b, &inst.gf);
                let ev = |t: u64| {
                    (0..q.c.len())
                        .rev()
                        .fold(0u64, |v, j| inst.gf.mul(v, t) ^ q.c[j])
                };
                match (1..1u64 << n.min(16)).find(|&t| ev(t) == 0) {
                    Some(t) => t,
                    None => continue,
                }
            } else {
                rng.gen::<u64>() & ((1u64 << n) - 1)
            };
            let (w, _) = base.decompose(xr, &inst.gf);
            let (g, _) = fac.decompose(xr, &inst.gf);
            checked += 1;
            let ok_witness = match g {
                None => true,
                Some([a, b, c]) => {
                    fac.contains(a)
                        && fac.contains(b)
                        && fac.contains(c)
                        && eval_s4_general(a, b, c, xr, inst.b, &inst.gf) == 0
                }
            };
            if w.is_some() != g.is_some() || !ok_witness {
                agree = false;
                eprintln!("n={n} xr={xr:#x}: base={w:?} factored={g:?}");
            }
            yes += g.is_some() as u32;
        }
        println!(
            "n={n} {}: preflight {checked} targets ({yes} decompose) — {}",
            inst.name,
            if agree {
                "identical decisions"
            } else {
                "MISMATCH"
            }
        );
        assert!(agree, "the oracles disagree; not running the ledger");

        // ── The ledger: every variant to the logarithm, rho as reference. ──
        let t0 = std::time::Instant::now();
        let Some(res) = run_char2_instance(&inst, &cfg) else {
            println!("n={n}: no admissible summand count");
            continue;
        };
        let wall = t0.elapsed().as_secs_f64();
        writeln!(out, "{}", serde_json::to_string(&res).expect("serialise")).unwrap();
        out.flush().unwrap();

        let rho_wall: f64 =
            res.rho.iter().map(|r| r.wall_ns as f64).sum::<f64>() / res.rho.len().max(1) as f64;
        println!(
            "  r=2^{:.1}  rho: mean S={:.2}  mean wall {:.2} ms ({} walks)  | ns/pair: s4 {:.1}  s4f {:.1}  | ledger {:.1} s",
            res.log2_r,
            res.rho_s_mean,
            rho_wall / 1e6,
            res.rho.len(),
            res.calibration_measured.ns_per_s4_pair.unwrap_or(f64::NAN),
            res.calibration_measured.ns_per_s4f_pair.unwrap_or(f64::NAN),
            wall
        );
        let mut by: BTreeMap<&str, Vec<_>> = BTreeMap::new();
        for v in &res.variants {
            by.entry(v.name.as_str()).or_default().push(v);
        }
        println!(
            "  {:<34} {:>3} {:>4} {:>9} {:>9} {:>10} {:>10} {:>11} {:>9} {:>8} {:>8}",
            "variant",
            "run",
            "ok",
            "trials",
            "rels",
            "rel ms",
            "LA ms",
            "total ms",
            "S",
            "S_wall",
            "×rho"
        );
        let mean = |xs: &[f64]| xs.iter().sum::<f64>() / xs.len().max(1) as f64;
        for (name, vs) in &by {
            let ok = vs.iter().filter(|v| v.verified).count();
            let f = |g: &dyn Fn(&&crypto_lib::cryptanalysis::ic_boundary::VariantResult) -> f64| {
                mean(&vs.iter().map(g).collect::<Vec<f64>>())
            };
            println!(
                "  {:<34} {:>3} {:>4} {:>9.0} {:>9.0} {:>10.1} {:>10.1} {:>11.1} {:>9.2} {:>8.2} {:>8.2}",
                name,
                vs.len(),
                ok,
                f(&|v| v.trials as f64),
                f(&|v| v.relations_found as f64),
                f(&|v| v.relations.wall_ns as f64 / 1e6),
                f(&|v| v.linear_algebra.wall_ns as f64 / 1e6),
                f(&|v| v.total_wall_ns as f64 / 1e6),
                f(&|v| v.s),
                f(&|v| v.s_wall),
                f(&|v| v.ratio_to_rho),
            );
        }
    }
    println!("records: {out_path}");
}
