//! Original Prime31: exact support for eight unweighted signed B8 leaves.
//! This is a generic GRADING oracle for a coverage theorem, not a candidate
//! non-generic relation extractor and not an ECDLP solver.
//!
//! Prior source: research/prime31_ic_20261115/native/lib.rs at 1df71464...
//! Protocol: ../rr9/RR9_SIGN_CANCELLATION_COVERAGE_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::HashSet;
use std::time::Instant;

const EXPECTED_X: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
const MAX_LEAVES: usize = 8;
type SignedMultiples = [[Point; 17]; MAX_LEAVES];

fn choose(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    let k = k.min(n-k);
    let mut out = 1u64;
    for i in 0..k {
        out = (out * (n-i)) / (i+1);
    }
    out
}

// Number of vectors in Z^d with L1 norm exactly r.
// This is independent of any elliptic-curve group arithmetic.
fn shell(d: u64, r: u64) -> u64 {
    if r == 0 { return 1; }
    (1..=d.min(r)).map(|j|
        choose(d,j) * (1u64 << j) * choose(r-1,j-1)
    ).sum()
}

fn first_x8(ar: &mut Arithmetic) -> [(u64,u64); MAX_LEAVES] {
    let mut out = Vec::new();
    for x in 0..10_000u64 {
        let rhs = ar.rhs(PRIME31,x);
        if let Some(y) = ar.sqrt(rhs,PRIME31.p) {
            if y != 0 { out.push((x,y.min(PRIME31.p-y))); }
        }
        if out.len() == MAX_LEAVES { break; }
    }
    assert_eq!(out.len(), MAX_LEAVES);
    assert_eq!(out.iter().map(|pt| pt.0).collect::<Vec<_>>(), EXPECTED_X);
    for &pt in &out { assert!(ar.on_curve(PRIME31,Some(pt))); }
    out.try_into().expect("exactly eight points")
}

fn multiples(ar: &mut Arithmetic, base: &[(u64,u64);MAX_LEAVES]) -> SignedMultiples {
    let mut out = [[None;17];MAX_LEAVES];
    for i in 0..MAX_LEAVES {
        for k in 1..=MAX_LEAVES {
            out[i][8+k] = ar.add(PRIME31,out[i][8+k-1],Some(base[i]));
            out[i][8-k] = ar.neg(PRIME31,out[i][8+k]);
        }
        assert_eq!(out[i][8],None);
    }
    out
}

// Enumerate each coefficient vector ONCE. This is an out-of-band
// generic group grading control. Never feed its sets to F4 / candidate.
fn enumerate_vectors(
    i: usize,
    remaining: usize,
    parity: usize,
    sum: Point,
    signed: &SignedMultiples,
    ar: &mut Arithmetic,
    support: &mut HashSet<Point>,
    count: &mut u64,
) {
    if i == MAX_LEAVES {
        if parity == 0 {
            *count += 1;
            support.insert(sum);
        }
        return;
    }
    for k in -(remaining as i16)..=(remaining as i16) {
        let abs = k.unsigned_abs() as usize;
        if i == MAX_LEAVES - 1 && (parity + abs) % 2 != 0 {
            continue;
        }
        let next = if k == 0 { sum } else {
            ar.add(PRIME31,sum,signed[i][(k + 8) as usize])
        };
        enumerate_vectors(
            i+1, remaining-abs, (parity+abs)%2,
            next, signed, ar, support, count
        );
    }
}

fn squarefree(signed: &SignedMultiples, ar: &mut Arithmetic) -> HashSet<Point> {
    let mut set = HashSet::new();
    for word in 0..256u16 {
        let mut sum = None;
        for i in 0..MAX_LEAVES {
            let sign = if (word >> i) & 1 == 0 { -1 } else { 1 };
            sum = ar.add(PRIME31,sum,signed[i][(sign+8) as usize]);
        }
        set.insert(sum);
    }
    set
}

#[derive(Debug)]
struct Outcome {
    count: u64,
    unique: usize,
    squarefree: usize,
    group_additions: u64,
    elapsed_ns: u128,
    base: [(u64,u64);MAX_LEAVES],
}
fn calculate() -> Outcome {
    let mut certificate = Arithmetic::default();
    check_curve(PRIME31,&mut certificate).expect("unchanged ordinary cofactor-one curve");
    let mut ar = Arithmetic::default();
    let base = first_x8(&mut ar);
    let start = Instant::now();
    let signed = multiples(&mut ar,&base);
    let mut support = HashSet::with_capacity(190_000);
    let mut count = 0;
    enumerate_vectors(0,8,0,None,&signed,&mut ar,&mut support,&mut count);
    let squarefree = squarefree(&signed,&mut ar);
    let elapsed_ns = start.elapsed().as_nanos();
    assert_eq!(count,187_137,"analytic coefficient-vector count");
    assert_eq!(squarefree.len(),256,"all sign words give distinct group sums on original curve");
    assert!(squarefree.is_subset(&support));
    // This independently computed expected support cardinality is recorded
    // from a separate Python BigInt calculation, not from the source's DFS.
    assert_eq!(support.len(),187_137,"original-curve exact support cardinality");
    Outcome {
        count,unique:support.len(),squarefree:squarefree.len(),
        group_additions:ar.counts.group_additions,
        elapsed_ns,base
    }
}
fn main() {
    let o = calculate();
    let shells:Vec<u64>=(0..=8).step_by(2).map(|r|shell(8,r)).collect();
    println!("{}",json!({
        "schema":"prime31.rr9.unweighted-signed-cancellation-coverage.v1",
        "p":PRIME31.p, "a":PRIME31.a,"b":PRIME31.b,
        "n":PRIME31.n, "generator":PRIME31.g,
        "base":"original first-eight-rational-x", "x":o.base.map(|pt|pt.0),
        "l1_even_shells":shells,
        "signed_net_vector_count":o.count,
        "unweighted_sums_distinct_exact":o.unique,
        "fixed_squarefree_8x_distinct_exact":o.squarefree,
        "uniform_unweighted_probability_numerator":o.unique,
        "uniform_squarefree_probability_numerator":o.squarefree,
        "uniform_probability_denominator":PRIME31.n,
        "group_additions_grading":o.group_additions,
        "elapsed_grading_ns_not_solver":o.elapsed_ns.to_string(),
        "generic_enumeration_grading_only":true,
        "candidate_unknown_index_extraction":false,
        "ecdlp_logs_recovered":0,
        "complete_solver_timing_measured":false
    }));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytic_even_l1_shells_and_upper_bound() {
        let counts:Vec<_>=(0..=8).step_by(2).map(|r|shell(8,r)).collect();
        assert_eq!(counts,vec![1,128,2816,27008,157184]);
        assert_eq!(counts.into_iter().sum::<u64>(),187137);
        assert_eq!(choose(23,8),490314);
        assert_eq!(1u64<<8,256);
    }
    #[test]
    fn prime31_actual_group_support() {
        let o=calculate();
        assert_eq!(o.count,187137);
        assert_eq!(o.unique,187137);
        assert_eq!(o.squarefree,256);
        assert_eq!(o.base.map(|pt|pt.0),EXPECTED_X);
    }
}
