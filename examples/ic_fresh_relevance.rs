//! Finite curve-membership relevance of every frozen fresh target.
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

#[cfg(test)]
fn element(x: u64, n: u32) -> crypto_lib::binary_ecc::F2mElement {
    crypto_lib::binary_ecc::F2mElement::from_biguint(&num_bigint::BigUint::from(x), n)
}
fn span(bs: &[u64]) -> Vec<u64> {
    let mut out = vec![0];
    for &b in bs {
        let len = out.len();
        for i in 0..len {
            out.push(out[i] ^ b);
        }
    }
    out
}
#[allow(dead_code)]
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
fn trace(gf: &Gf2, mut x: u64) -> u64 {
    let mut t = 0;
    for _ in 0..gf.n {
        t ^= x;
        x = gf.sqr(x);
    }
    assert!(t <= 1);
    t
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "FRESH_INPUT_DIR");
    let mut paths: Vec<_> = std::fs::read_dir(Path::new(&args[0]))
        .unwrap()
        .map(Result::unwrap)
        .filter(|e| e.path().extension().is_some_and(|x| x == "jsonl"))
        .collect();
    paths.sort_by_key(|e| e.file_name());
    assert_eq!(paths.len(), 36);
    let mut files = vec![];
    let mut targets = vec![];
    let mut totals = [0u64; 4];
    for path in paths {
        let bytes = std::fs::read(path.path()).unwrap();
        let digest = format!("{:x}", Sha256::digest(&bytes));
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(rows.len(), 32);
        let n = rows[0]["n"].as_u64().unwrap() as u32;
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let quad = exact::Quadratic::new(&gf);
        let mut counts = [0u64; 4];
        for row in &rows {
            assert_eq!(row["n"], n);
            assert_eq!(row["field_modulus"], gf.irr);
            let x = row["x_r"].as_u64().unwrap();
            let rhs = gf.mul(gf.sqr(x), x) ^ 1;
            let ys = quad.roots(1, x, rhs).unwrap();
            assert!(ys.iter().all(|&y| gf.sqr(y) ^ gf.mul(x, y) == rhs));
            let trace_lifts = if x == 0 {
                true
            } else {
                trace(&gf, x ^ gf.sqr(gf.inv(x))) == 0
            };
            assert_eq!(!ys.is_empty(), trace_lifts);
            let unsat = row["solutions"].as_u64().unwrap() == 0;
            let index = 2 * usize::from(unsat) + usize::from(ys.is_empty());
            counts[index] += 1;
            totals[index] += 1;
            targets.push(
                json!({"file":path.file_name().to_str().unwrap(),"draw":row["draw"],
                "n":n,"x_r":x,"boolean_solutions":row["solutions"],"curve_liftable":!ys.is_empty(),
                "y_roots":ys,"subgroup_membership":null}),
            );
        }
        files.push(
            json!({"file":path.file_name().to_str().unwrap(),"sha256":digest,
            "m":rows[0]["m"],"n":n,"ell":rows[0]["ell"],"family":rows[0]["family"],
            "draws":32,"sat_liftable":counts[0],"sat_nonliftable":counts[1],
            "unsat_liftable":counts[2],"unsat_nonliftable":counts[3]}),
        );
    }
    assert_eq!(targets.len(), 1152);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-fresh-target-relevance/v1",
        "status":"all_target_roots_and_trace_criteria_agree","curve":"y^2+x*y=x^3+1",
        "files":files,"targets":targets,"draws":1152,
        "totals":{"sat_liftable":totals[0],"sat_nonliftable":totals[1],"unsat_liftable":totals[2],"unsat_nonliftable":totals[3]},
        "scope":"Finite target curve liftability only; subgroup membership, actual point decompositions and complete costs remain unknown."})).unwrap());
}
