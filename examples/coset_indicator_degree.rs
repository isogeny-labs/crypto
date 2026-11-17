//! Boolean (ANF) degree of the 2-torsion coset indicators restricted to a
//! factor-base subspace `V ⊂ F_{2^n}`: `x ↦ Tr(x)`, `x ↦ Tr(√b/x)` and their
//! sum, which is the solvability parity of one chain link
//! `S₃(x₁, x₂, u) = 0` (`Tr(S₃/u²) = Tr(x₁) + Tr(x₂) + Tr(√b/x₁) + Tr(√b/x₂)`
//! for `b = 1`; the repository's "trace class", `RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` §2.2).
//!
//! A pilot for the lead "the coset-indicator degree bounds the closure degree
//! from below": a refutation that goes through the parity must derive a
//! polynomial of this degree. Exact, by a Möbius transform over the `2^ℓ`
//! points of `V` (`x = 0` is assigned indicator 0).
//!
//! ```sh
//! cargo run --release --example coset_indicator_degree -- FILE.jsonl …   # committed draws' bases
//! cargo run --release --example coset_indicator_degree -- --gp 43:14 --random 43:14 --seed 7
//! ```
//!
//! One line per basis: n, ℓ, source, deg Tr(x)|V, deg Tr(1/x)|V, deg of the sum,
//! and the number of ANF terms of Tr(1/x)|V.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::Value;

fn trace(gf: &Gf2, n: u32, x: u64) -> u64 {
    let (mut acc, mut p) = (0u64, x);
    for _ in 0..n {
        acc ^= p;
        p = gf.sqr(p);
    }
    acc & 1
}

/// ANF degree and term count of a Boolean function given by its truth table
/// over `ℓ` variables (index = coordinate vector).
fn anf(table: &mut [u8], ell: usize) -> (u32, usize) {
    for i in 0..ell {
        let bit = 1usize << i;
        for c in 0..table.len() {
            if c & bit != 0 {
                table[c] ^= table[c ^ bit];
            }
        }
    }
    let mut deg = 0u32;
    let mut terms = 0usize;
    for (c, &v) in table.iter().enumerate() {
        if v != 0 {
            terms += 1;
            deg = deg.max(c.count_ones());
        }
    }
    (deg, terms)
}

fn report(n: u32, words: &[u64], source: &str) {
    let irr = find_irreducible_sparse(n).expect("field");
    let gf = Gf2::new(&irr);
    let ell = words.len();
    let span = |c: usize| -> u64 {
        (0..ell)
            .filter(|t| (c >> t) & 1 == 1)
            .fold(0u64, |acc, t| acc ^ words[t])
    };
    let size = 1usize << ell;
    let mut t_tr = vec![0u8; size];
    let mut t_inv = vec![0u8; size];
    let mut t_sum = vec![0u8; size];
    for c in 0..size {
        let x = span(c);
        let a = trace(&gf, n, x) as u8;
        let b = if x == 0 {
            0
        } else {
            trace(&gf, n, gf.inv(x)) as u8
        };
        t_tr[c] = a;
        t_inv[c] = b;
        t_sum[c] = a ^ b;
    }
    let (d_tr, _) = anf(&mut t_tr, ell);
    let (d_inv, terms_inv) = anf(&mut t_inv, ell);
    let (d_sum, _) = anf(&mut t_sum, ell);
    println!(
        r#"{{"n":{n},"ell":{ell},"source":"{source}","deg_tr":{d_tr},"deg_tr_inv":{d_inv},"deg_parity":{d_sum},"terms_tr_inv":{terms_inv}}}"#
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let mut seed = 0x5EEDu64;
    let mut jobs: Vec<(String, String)> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                seed = args[i + 1].parse().expect("--seed");
                i += 2;
            }
            "--gp" | "--random" | "--coord" => {
                jobs.push((args[i].clone(), args[i + 1].clone()));
                i += 2;
            }
            f => {
                files.push(f.to_string());
                i += 1;
            }
        }
    }
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            if v.get("control").is_some() {
                continue;
            }
            let n = v["n"].as_u64().expect("n") as u32;
            let words: Vec<u64> = v["v_basis"]
                .as_array()
                .expect("v_basis")
                .iter()
                .map(|w| w.as_u64().unwrap())
                .collect();
            let src = format!(
                "{}:draw{}",
                std::path::Path::new(f)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(""),
                v["draw"]
            );
            report(n, &words, &src);
        }
    }
    let mut rng = StdRng::seed_from_u64(seed);
    for (kind, spec) in jobs {
        let (n, ell): (u32, usize) = {
            let (a, b) = spec.split_once(':').expect("n:ell");
            (a.parse().unwrap(), b.parse().unwrap())
        };
        let irr = find_irreducible_sparse(n).expect("field");
        let gf = Gf2::new(&irr);
        let words: Vec<u64> = match kind.as_str() {
            // Geometric progression 1, θ, θ², … in the polynomial basis (θ = the field generator).
            "--gp" => {
                let mut w = Vec::with_capacity(ell);
                let mut p = 1u64;
                for _ in 0..ell {
                    w.push(p);
                    p = gf.mul(p, 2);
                }
                w
            }
            // Coordinate window: the low ℓ polynomial-basis coordinates (same as --gp when θ = the generator).
            "--coord" => (0..ell).map(|i| 1u64 << i).collect(),
            _ => random_subspace_basis(n, ell, &mut rng)
                .iter()
                .map(|e: &F2mElement| e.raw_bits().first().copied().unwrap_or(0))
                .collect(),
        };
        report(n, &words, &format!("{}:{spec}", &kind[2..]));
    }
}
