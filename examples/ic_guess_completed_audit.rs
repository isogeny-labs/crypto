//! Completed balanced-prefix accounting and separately executable HKY replay.
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::{
    koblitz_groebner::{build_decomposition_system, FieldStructure},
    koblitz_index_calculus::find_irreducible_sparse,
    semaev_decomp::Gf2,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};
#[allow(dead_code)]
#[path = "ic_algebraic_followup/hky.rs"]
mod hky;
#[allow(dead_code)]
#[path = "ic_algebraic_followup/wide.rs"]
mod wide;
const ORDER: [usize; 4] = [7, 15, 23, 24];
fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn source(study: &Path) -> Value {
    let r = read(&study.join("pipeline-receipts/guess-completion-20261025-01/receipt.json"));
    assert_eq!(r["recorded_new_attempts"], 116);
    assert_eq!(r["runner_ended"], true);
    assert_eq!(r["attempts"].as_array().unwrap().len(), 124);
    r
}
fn summary(study: &Path) -> Value {
    let receipt = source(study);
    let groups = receipt["groups"].as_array().unwrap();
    let attempts = receipt["attempts"].as_array().unwrap();
    let mut draws = vec![];
    for draw in [0, 3, 5, 6] {
        let gs: Vec<_> = (0..=4)
            .map(|k| {
                groups
                    .iter()
                    .find(|g| g["draw"] == draw && g["guesses"] == k)
                    .unwrap()
            })
            .collect();
        let min = gs
            .iter()
            .position(|g| g["verdict"] == "all_branches_refuted")
            .unwrap();
        assert!(gs[..min]
            .iter()
            .all(|g| g["verdict"] == "has_nonrefuting_branch"));
        let search_words = gs[..=min].iter().try_fold(0u64, |s, g| {
            s.checked_add(g["all_branch_solver_word_ops"].as_u64()?)
        });
        draws.push(json!({"draw":draw,"balanced_prefix_minimum":min,"passing_variables":ORDER[..min],
            "passing_set_solver_words":gs[min]["all_branch_solver_word_ops"],"search_through_first_passing_set_solver_words":search_words,
            "groups":gs,"scope":"minimum only in the registered balanced-prefix family at degree four; not over all variable subsets"}));
    }
    let count = |kind: &str| attempts.iter().filter(|a| a["verdict"] == kind).count();
    let k4_words = groups
        .iter()
        .filter(|g| g["guesses"] == 4)
        .try_fold(0u64, |s, g| {
            s.checked_add(g["all_branch_solver_word_ops"].as_u64()?)
        });
    let k4_refuted = groups
        .iter()
        .filter(|g| g["guesses"] == 4)
        .map(|g| g["refuted"].as_u64().unwrap())
        .sum::<u64>();
    assert_eq!(k4_refuted, 64);
    json!({"schema":"ic-completed-balanced-guess-summary/v1","recorded_new_attempts":116,"total_attempts_including_old_draw0_k3":124,
        "producer_refuted":count("refuted"),"producer_saturated_nonrefuting":count("saturated_not_refuted"),"producer_unknown":count("unknown"),
        "k4_branches_refuted":k4_refuted,"k4_all_branch_solver_words":k4_words,"draws":draws,
        "status":"all_registered_attempts_terminated_native_raw_accounting_recomputed",
        "independent_hky_replay":"separate pending program; not performed by this summary",
        "complete_query_or_dlp_cost":null,"scope":"Original x-only Boolean fixtures; declared reference/twist subgroup-support gaps remain. No ordinary-query or global-minimum claim."})
}
fn specialised(eqs: &[Vec<u128>], vars: &[usize], branch: u64) -> (Vec<Vec<u128>>, usize) {
    assert!(vars.len() <= 4 && branch < (1u64 << vars.len()));
    let mut polys = vec![];
    let mut active = 0u128;
    for p in eqs {
        let mut terms = BTreeSet::new();
        for &term in p {
            let mut t = term;
            let mut zero = false;
            for (i, &v) in vars.iter().enumerate() {
                if t >> v & 1 == 1 {
                    if branch >> i & 1 == 0 {
                        zero = true;
                        break;
                    }
                    t &= !(1u128 << v);
                }
            }
            if !zero && !terms.insert(t) {
                terms.remove(&t);
            }
        }
        for &t in &terms {
            active |= t;
        }
        polys.push(terms.into_iter().collect::<Vec<_>>());
    }
    let indices: Vec<_> = (0..128).filter(|&i| active >> i & 1 == 1).collect();
    let result = polys
        .iter()
        .map(|p| {
            p.iter()
                .map(|t| {
                    indices
                        .iter()
                        .enumerate()
                        .fold(0u128, |s, (j, &i)| s | (((t >> i) & 1) << j))
                })
                .collect()
        })
        .collect();
    (result, indices.len())
}
fn columns(n: usize, d: usize) -> usize {
    let (mut total, mut term) = (1, 1);
    for k in 1..=d.min(n) {
        term = term * (n + 1 - k) / k;
        total += term;
    }
    total
}
fn replay(study: &Path, draw: u64, k: usize, branch: u64) -> Value {
    assert!(k <= 4 && [0, 3, 5, 6].contains(&draw) && branch < 1u64 << k);
    let receipt = source(study);
    let attempts = receipt["attempts"].as_array().unwrap();
    let original = attempts
        .iter()
        .find(|a| a["draw"] == draw && a["guesses"] == k && a["branch"] == branch)
        .unwrap();
    let input = std::fs::read(study.join("inputs/guess-ell8.jsonl")).unwrap();
    let hash = hex::encode(Sha256::digest(&input));
    assert_eq!(
        hash,
        "95ee147e1221ce4ef6c98bdd0b808398a01362343ffaa66debc217d6eeff5a9b"
    );
    let row = std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|r| r["draw"] == draw)
        .unwrap();
    assert_eq!(row["solutions"], 0);
    let n = row["n"].as_u64().unwrap() as u32;
    let m = row["m"].as_u64().unwrap_or(3) as usize;
    let bs: Vec<_> = row["v_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect();
    let irr = find_irreducible_sparse(n).unwrap();
    let gf = Gf2::new(&irr);
    let xr = row["x_r"].as_u64().unwrap();
    let element = |x| F2mElement::from_biguint(&BigUint::from(x), n);
    let base: Vec<_> = bs.iter().copied().map(element).collect();
    let st = FieldStructure::new(n, &irr);
    let sys = build_decomposition_system(&base, &element(xr), &element(1u64), m, &st).unwrap();
    let (eqs, nv) = wide::chain_for_b(&gf, &bs, xr, m, 1);
    assert_eq!(nv, sys.n_vars);
    let old: Vec<Vec<u128>> = sys
        .equations
        .iter()
        .map(|p| p.terms.iter().map(|t| u128::from(t.mask)).collect())
        .collect();
    let (a, active) = specialised(&old, &ORDER[..k], branch);
    let (b, other) = specialised(&eqs, &ORDER[..k], branch);
    assert_eq!(a, b);
    assert_eq!(active, other);
    let cols = columns(active, 4);
    assert!(cols <= 200000);
    let (plain, rank, rounds) = hky::closure_par(&a, active as u32, 4, cols);
    let verdict = if rank == cols {
        "refuted"
    } else {
        "saturated_not_refuted"
    };
    let old = original["verdict"].as_str().unwrap();
    if old != "unknown" {
        assert_eq!(old, verdict);
    }
    json!({"schema":"ic-balanced-guess-hky-replay/v1","draw":draw,"guesses":k,"branch":branch,"guessed_variables":ORDER[..k],
        "input_sha256":hash,"producer_row_sha256":original["rows_sha256"],"producer_verdict":old,"hky_verdict":verdict,
        "equation_builders_agree":true,"active_variables":active,"full_columns":cols,"plain_rank":plain,"closure_rank":rank,"hky_rounds":rounds,
        "producer_solver_words":original["solver_word_ops"],"independent_solver_words":null,"status":"independent_hky_verdict_agrees_or_producer_was_censored",
        "scope":"Finite original Boolean branch; subgroup/query-law qualification and complete price unchanged"})
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let v = match a.first().map(String::as_str) {
        Some("summary") if a.len() == 2 => summary(Path::new(&a[1])),
        Some("case") if a.len() == 5 => replay(
            Path::new(&a[1]),
            a[2].parse().unwrap(),
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
        ),
        _ => panic!("summary STUDY | case STUDY DRAW K BRANCH"),
    };
    println!("{v}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn substitution_xors_duplicates_and_removes_only_inactive_coordinates() {
        let p = vec![vec![0, 1, 2, 3], vec![1, 4, 5]];
        assert_eq!(specialised(&p, &[0], 1), (vec![vec![], vec![0]], 0));
        assert_eq!(specialised(&p, &[0], 0), (vec![vec![0, 1], vec![2]], 2));
        assert_eq!(columns(42, 4), 124314);
    }
}
