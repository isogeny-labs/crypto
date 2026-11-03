//! Falsifier A of `docs/ic/CERTICOM131_IC_PROGRAM.md`: the resolving degree
//! of FPPR decomposition systems on **random (non-Koblitz) binary curves**,
//! at `m = 3` and `ℓ ≈ n/3`, paired with the Koblitz `b = 1` system on the
//! same draw.
//!
//! Over `F_{2^n}` the summation polynomials of `y² + xy = x³ + ax² + b`
//! depend on `b` only, so the Weil-restricted `m = 3` system of a random
//! curve differs from the Koblitz system that `dreg_ladder` measures in
//! exactly one place: the constant `b`.  Each draw takes `b` from a real
//! random curve (`ic_boundary::random_binary_instance`, a large prime-order
//! subgroup, named by its ICV1 slug), samples a random `ℓ`-dimensional
//! subspace `V` and target abscissa `x_R` exactly as `dreg_ladder` does,
//! counts the system's Boolean solutions exactly, and — when there are none
//! — measures the first fall degree and the resolving degree (the Macaulay
//! degree at which the constant 1 enters the row space, or every variable
//! is pinned).  The same `(V, x_R)` is then measured with `b = 1`.
//!
//! ```sh
//! F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 \
//!   cargo run --release --example certicom131_fppr_dreg -- \
//!     --cells 7:2:7,11:4:7,13:4:7 --unsat 6 --seed 20261020
//! ```
//!
//! A cell is `n:ℓ:d_max`.  `--unsat-index K` measures only the cell's
//! `K`-th unsatisfiable draw (one cell per process), replaying the earlier
//! draws through the exact count alone, so a large cell runs one process
//! per draw.  `--no-pair` skips the `b = 1` measurement.  One JSON line per
//! measured draw on stdout; the summary table on stderr.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::ic_boundary::random_binary_instance;
use crypto_lib::cryptanalysis::koblitz_bench::{
    chained_s3_solution_count_fast, ladder_measure_mb, ladder_sample, semi_regular_dreg,
    LadderDraw, LadderOutcome,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn outcome_json(o: &LadderOutcome) -> String {
    match o {
        LadderOutcome::Satisfiable => r#"{"kind":"satisfiable"}"#.into(),
        LadderOutcome::Resolved { degree, refuted } => {
            format!(r#"{{"kind":"resolved","degree":{degree},"refuted":{refuted}}}"#)
        }
        LadderOutcome::AtLeast(d) => format!(r#"{{"kind":"at_least","degree":{d}}}"#),
        LadderOutcome::CapsHit { built } => format!(
            r#"{{"kind":"caps_hit","built":{}}}"#,
            built.map_or("null".to_string(), |b| b.to_string())
        ),
    }
}

fn outcome_short(o: &LadderOutcome) -> String {
    match o {
        LadderOutcome::Satisfiable => "sat".into(),
        LadderOutcome::Resolved {
            degree,
            refuted: true,
        } => format!("{degree}"),
        LadderOutcome::Resolved {
            degree,
            refuted: false,
        } => format!("{degree}p"),
        LadderOutcome::AtLeast(d) => format!("≥{d}"),
        LadderOutcome::CapsHit { built } => {
            format!("caps@{}", built.map_or("-".into(), |b| b.to_string()))
        }
    }
}

/// A random curve for draw `i` of a cell: its slug, `a` and `b`.
fn curve_for(n: u32, cell_seed: u64, i: usize) -> (String, u64, u64) {
    for j in 0..64u64 {
        let s = cell_seed ^ (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (j << 56);
        if let Some(inst) = random_binary_instance(n, s, 8) {
            return (inst.name.clone(), inst.a, inst.b);
        }
    }
    panic!("no random binary curve over F_2^{n}");
}

fn elem(w: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(w), n)
}

struct Measured {
    draw: usize,
    slug: String,
    a: u64,
    b: u64,
    rand: LadderDraw,
    koblitz: Option<LadderDraw>,
}

#[allow(clippy::too_many_arguments)]
fn measure(
    n: u32,
    _ell: usize,
    d_max: u32,
    ffd_max: u32,
    pair: bool,
    draw: usize,
    slug: String,
    a: u64,
    b: u64,
    basis: &[F2mElement],
    x_r: &F2mElement,
) -> Measured {
    let rand = ladder_measure_mb(3, n, basis, x_r, &elem(b, n), 1, d_max, ffd_max).expect("cell");
    let koblitz = pair.then(|| {
        ladder_measure_mb(3, n, basis, x_r, &F2mElement::one(n), 1, d_max, ffd_max).expect("cell")
    });
    Measured {
        draw,
        slug,
        a,
        b,
        rand,
        koblitz,
    }
}

fn print_json(n: u32, ell: usize, d_max: u32, m: &Measured, semireg: Option<u32>) {
    let d = &m.rand;
    let pair = m.koblitz.as_ref().map_or("null".to_string(), |k| {
        format!(
            r#"{{"solutions":{},"outcome":{},"ffd":{},"secs":{:.3}}}"#,
            k.solutions,
            outcome_json(&k.outcome),
            k.ffd.map_or("null".to_string(), |f| f.to_string()),
            k.secs
        )
    });
    println!(
        r#"{{"cell":"n{n}l{ell}","n":{n},"ell":{ell},"m":3,"surplus":{},"d_max":{d_max},"draw":{},"curve":"{}","a":{},"b":{},"n_vars":{},"n_eqs":{},"v_basis":{:?},"x_r":{},"solutions":{},"outcome":{},"ffd":{},"semireg":{},"secs":{:.3},"koblitz_b1":{pair}}}"#,
        n as i64 - 3 * ell as i64,
        m.draw,
        m.slug,
        m.a,
        m.b,
        d.n_vars,
        d.n_eqs,
        d.v_basis,
        d.x_r,
        d.solutions,
        outcome_json(&d.outcome),
        d.ffd.map_or("null".to_string(), |f| f.to_string()),
        semireg.map_or("null".to_string(), |s| s.to_string()),
        d.secs,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let num = |name: &str, default: u64| flag(name).and_then(|v| v.parse().ok()).unwrap_or(default);
    let cells: Vec<(u32, usize, u32)> = flag("--cells")
        .unwrap_or_else(|| "7:2:7".into())
        .split(',')
        .map(|c| {
            let p: Vec<u64> = c
                .split(':')
                .map(|x| x.parse().expect("cell is n:ell:d_max"))
                .collect();
            assert_eq!(p.len(), 3, "cell is n:ell:d_max");
            (p[0] as u32, p[1] as usize, p[2] as u32)
        })
        .collect();
    let want_unsat = num("--unsat", 6) as usize;
    let max_draws = num("--max-draws", 4096) as usize;
    let ffd_max = num("--ffd-max", 5) as u32;
    let seed = num("--seed", 20261020);
    let pair = !args.iter().any(|a| a == "--no-pair");
    let only = flag("--unsat-index").map(|v| v.parse::<usize>().expect("--unsat-index K"));
    if only.is_some() {
        assert_eq!(cells.len(), 1, "one cell per process with --unsat-index");
    }

    let mut table = Vec::new();
    for &(n, ell, d_max) in &cells {
        let cell_seed = seed ^ (u64::from(n) << 40) ^ ((ell as u64) << 32) ^ 0xB1AB_0000;
        let mut rng = StdRng::seed_from_u64(cell_seed);
        let irr = find_irreducible_sparse(n).expect("field");
        let n_vars = n as usize + 3 * ell;
        let degrees: Vec<u32> = std::iter::repeat_n(3, n as usize)
            .chain(std::iter::repeat_n(2, n as usize))
            .collect();
        let semireg = semi_regular_dreg(n_vars, &degrees);
        let (mut drawn, mut sat) = (0usize, 0usize);
        let mut measured: Vec<Measured> = Vec::new();
        let mut unsat_seen = 0usize;
        while drawn < max_draws {
            let (basis, x_r) = ladder_sample(n, ell, &mut rng);
            let (slug, a, b) = curve_for(n, cell_seed, drawn);
            drawn += 1;
            let solutions = chained_s3_solution_count_fast(&basis, &x_r, &elem(b, n), &irr);
            if solutions > 0 {
                sat += 1;
                continue;
            }
            let k = unsat_seen;
            unsat_seen += 1;
            if let Some(want) = only {
                if k < want {
                    continue;
                }
            }
            let m = measure(
                n,
                ell,
                d_max,
                ffd_max,
                pair,
                drawn - 1,
                slug,
                a,
                b,
                &basis,
                &x_r,
            );
            print_json(n, ell, d_max, &m, semireg);
            eprintln!(
                "n={n} ℓ={ell}: unsat draw {k} ({}) -> random-b {} ffd {} | b=1 {} ({:.1}s)",
                m.slug,
                outcome_short(&m.rand.outcome),
                m.rand.ffd.map_or(">".into(), |f| f.to_string()),
                m.koblitz
                    .as_ref()
                    .map_or("-".into(), |k| outcome_short(&k.outcome)),
                m.rand.secs + m.koblitz.as_ref().map_or(0.0, |k| k.secs)
            );
            measured.push(m);
            if only.is_some() || measured.len() >= want_unsat {
                break;
            }
        }
        table.push((n, ell, d_max, n_vars, drawn, sat, semireg, measured));
    }

    eprintln!();
    eprintln!("| n | ℓ | S | vars | d_max | draws | sat | random-b D | FFD | b=1 D (same V, x_R) | semi-regular |");
    eprintln!("|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|--:|");
    for (n, ell, d_max, n_vars, drawn, sat, semireg, ms) in &table {
        let r: Vec<String> = ms.iter().map(|m| outcome_short(&m.rand.outcome)).collect();
        let f: Vec<String> = ms
            .iter()
            .map(|m| m.rand.ffd.map_or(">".into(), |x| x.to_string()))
            .collect();
        let k: Vec<String> = ms
            .iter()
            .map(|m| {
                m.koblitz
                    .as_ref()
                    .map_or("-".into(), |k| outcome_short(&k.outcome))
            })
            .collect();
        eprintln!(
            "| {n} | {ell} | {} | {n_vars} | {d_max} | {drawn} | {sat} | {} | {} | {} | {} |",
            *n as i64 - 3 * *ell as i64,
            r.join(" "),
            f.join(" "),
            k.join(" "),
            semireg.map_or("-".into(), |s| s.to_string())
        );
    }
}
