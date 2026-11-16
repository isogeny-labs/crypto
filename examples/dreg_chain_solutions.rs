//! Lists every Boolean solution of the m = 3 chained S₃ system of a committed
//! draw, and checks each one against the system's equations.
//!
//! The enumeration is the one [`chained_s3_solution_count_fast`] counts: for
//! each summand pair `(x₁, x₂)` in `span(basis)²`, every root `e₁` of the first
//! link (a quadratic, at most two roots), then every root `x₃` of the closing
//! link that lies in the span. Each solution is written as the Boolean point of
//! [`build_decomposition_system`]'s layout (summand `i` owns `[iℓ, (i+1)ℓ)`,
//! then `e₁`'s `n` coordinates), and every equation is evaluated there.
//!
//! ```sh
//! cargo run --release --example dreg_chain_solutions -- --draw 1 \
//!     research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl
//! ```
//!
//! One JSON line per solution, then a summary line. The count must equal the
//! draw's committed `solutions`.
//!
//! [`chained_s3_solution_count_fast`]: crypto_lib::cryptanalysis::koblitz_bench::chained_s3_solution_count_fast

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::ic_boundary::ArtinSchreier;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, Mask};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let draw: u64 = args
        .iter()
        .position(|a| a == "--draw")
        .and_then(|i| args.get(i + 1))
        .map(|v| v.parse().expect("--draw K"))
        .expect("--draw K");
    let file = args
        .iter()
        .enumerate()
        .find(|(i, a)| !a.starts_with("--") && !(*i > 0 && args[i - 1] == "--draw"))
        .map(|(_, a)| a.clone())
        .expect("draw file");
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{file}: {e}"));
    let v: Value = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("json"))
        .find(|v| v["draw"].as_u64() == Some(draw) && v.get("control").is_none())
        .expect("draw not in file");
    assert_eq!(
        v.get("m").and_then(Value::as_u64).unwrap_or(3),
        3,
        "m = 3 only"
    );
    let n = v["n"].as_u64().expect("n") as u32;
    let ell = v["ell"].as_u64().expect("ell") as usize;
    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let gf = Gf2::new(&irr);
    let ash = ArtinSchreier::new(&gf);
    let words: Vec<u64> = v["v_basis"]
        .as_array()
        .expect("v_basis")
        .iter()
        .map(|w| w.as_u64().unwrap())
        .collect();
    assert_eq!(words.len(), ell);
    let (xr, bb) = (v["x_r"].as_u64().unwrap(), 1u64);
    let basis: Vec<F2mElement> = words
        .iter()
        .map(|&w| F2mElement::from_biguint(&BigUint::from(w), n))
        .collect();
    let sys = build_decomposition_system(
        &basis,
        &F2mElement::from_biguint(&BigUint::from(xr), n),
        &F2mElement::one(n),
        3,
        &st,
    )
    .expect("system");
    // span[c] = Σ_t c_t · basis[t]; its coefficient vector c is the summand's
    // Boolean coordinates.
    let span: Vec<u64> = (0..1u64 << ell)
        .map(|c| {
            (0..ell)
                .filter(|t| c >> t & 1 == 1)
                .fold(0, |acc, t| acc ^ words[t])
        })
        .collect();
    let index: HashMap<u64, u64> = span
        .iter()
        .enumerate()
        .map(|(c, &x)| (x, c as u64))
        .collect();
    // Every d with S₃(a, c, d) = (a+c)²d² + (ac)d + (ac)² + b = 0, at most two
    // (the same case analysis as the fast counter).
    let roots = |a: u64, c: u64| -> Vec<u64> {
        let (lead, mid) = (gf.sqr(a ^ c), gf.mul(a, c));
        let tail = gf.sqr(mid) ^ bb;
        if lead == 0 {
            return if mid == 0 {
                vec![]
            } else {
                vec![gf.mul(tail, gf.inv(mid))]
            };
        }
        if mid == 0 {
            return vec![gf.sqr_k(gf.mul(tail, gf.inv(lead)), n - 1)];
        }
        let scale = gf.mul(mid, gf.inv(lead));
        let rhs = gf.mul(gf.mul(tail, lead), gf.inv(gf.sqr(mid)));
        ash.solve(rhs)
            .map_or(vec![], |z| vec![gf.mul(scale, z), gf.mul(scale, z ^ 1)])
    };
    let mut found = 0u64;
    let mut all_vanish = true;
    for (c1, &x1) in span.iter().enumerate() {
        for (c2, &x2) in span.iter().enumerate() {
            for e1 in roots(x1, x2) {
                for x3 in roots(e1, xr) {
                    let Some(&c3) = index.get(&x3) else { continue };
                    found += 1;
                    let mut point: Mask = 0;
                    for (block, c) in [c1 as u64, c2 as u64, c3].into_iter().enumerate() {
                        for t in 0..ell {
                            if c >> t & 1 == 1 {
                                point |= bit((block * ell + t) as u32);
                            }
                        }
                    }
                    for k in 0..n as usize {
                        if e1 >> k & 1 == 1 {
                            point |= bit((3 * ell + k) as u32);
                        }
                    }
                    let vanishes = sys.equations.iter().all(|p| p.eval(point) == 0);
                    all_vanish &= vanishes;
                    let var = |k: usize| (point >> k & 1) as u8;
                    println!(
                        r#"{{"draw":{draw},"x1":{c1},"x2":{c2},"x3":{c3},"e1":{e1},"vanishes":{vanishes},"bits":{{"7":{},"15":{},"23":{},"24":{}}}}}"#,
                        var(7),
                        var(15),
                        var(23),
                        var(24)
                    );
                }
            }
        }
    }
    let committed = v["solutions"].as_u64();
    println!(
        r#"{{"draw":{draw},"solutions":{found},"committed_solutions":{},"matches_committed":{},"all_vanish":{all_vanish}}}"#,
        committed.map_or("null".to_string(), |c| c.to_string()),
        committed == Some(found)
    );
}
