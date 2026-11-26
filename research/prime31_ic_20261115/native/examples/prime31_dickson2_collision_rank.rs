//! Prime31 Dickson depth-10 two-summand homogeneous-collision rank census.
//!
//! Uses the repository's original Rust Arithmetic/Base, not a replacement solver.
//! Every retained row is an exact equality of two independently enumerated EC sums.
//! This is deterministic free-relation bookkeeping, NOT blind polynomial extraction
//! and not a complete ECDLP speed benchmark.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use std::collections::{HashMap, HashSet};

const N: u64 = 2_147_446_831;
const B: usize = 509;

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % N as u128) as u64
}

fn inv(mut a: u64) -> u64 {
    assert_ne!(a, 0);
    let mut e = N - 2;
    let mut out = 1_u64;
    while e != 0 {
        if e & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    out
}

fn collision_row(first: (usize, usize), next: (usize, usize)) -> Vec<u64> {
    let mut row = vec![0_u64; B];
    for (index, direction) in [
        (first.0, 1_i64),
        (first.1, 1),
        (next.0, -1),
        (next.1, -1),
    ] {
        let signed = if index % 2 == 0 { direction } else { -direction };
        let base_index = index / 2;
        assert!(base_index < B);
        row[base_index] = (row[base_index] as i64 + signed)
            .rem_euclid(N as i64) as u64;
    }
    row
}

fn projective(row: &[u64]) -> Option<Vec<(u16, u64)>> {
    let first = row.iter().copied().find(|&x| x != 0)?;
    let factor = inv(first);
    Some(
        row.iter()
            .enumerate()
            .filter(|(_, &x)| x != 0)
            .map(|(j, &x)| (j as u16, mul(x, factor)))
            .collect(),
    )
}

struct Rank {
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
}

impl Rank {
    fn new() -> Self {
        Self { pivots: vec![None; B], rank: 0 }
    }

    fn insert(&mut self, mut row: Vec<u64>) {
        for i in 0..B {
            let factor = row[i];
            if factor == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[i] {
                for j in i..B {
                    row[j] = (row[j] + N - mul(factor, pivot[j])) % N;
                }
            } else {
                let reciprocal = inv(factor);
                for v in &mut row[i..] {
                    *v = mul(*v, reciprocal);
                }
                self.pivots[i] = Some(row);
                self.rank += 1;
                return;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let mut arithmetic = Arithmetic::default();
    check_curve(PRIME31, &mut arithmetic)?;
    let base = Base::build(PRIME31, 10, &mut arithmetic)?;
    assert_eq!(base.points.len(), B);

    let mut signed = Vec::with_capacity(2 * B);
    for &point in &base.points {
        signed.push(Some(point));
        signed.push(arithmetic.neg(PRIME31, Some(point)));
    }
    let mut first: HashMap<Option<(u64, u64)>, (usize, usize)> =
        HashMap::with_capacity(520_000);
    let mut relations = HashSet::new();
    let mut rank = Rank::new();
    let mut records = 0_u64;
    let mut infinity_records = 0_u64;
    let mut identity_collisions = 0_u64;
    let mut nonidentity_collisions = 0_u64;

    for i in 0..signed.len() {
        for j in i..signed.len() {
            records += 1;
            let sum = arithmetic.add(PRIME31, signed[i], signed[j]);
            if sum.is_none() {
                infinity_records += 1;
            } else {
                assert!(arithmetic.on_curve(PRIME31, sum));
            }
            if let Some(&previous) = first.get(&sum) {
                if sum.is_none() {
                    identity_collisions += 1;
                } else {
                    nonidentity_collisions += 1;
                    let row = collision_row(previous, (i, j));
                    let normalized =
                        projective(&row).expect("nonidentity collision has zero formal row");
                    if relations.insert(normalized) {
                        rank.insert(row);
                    }
                }
            } else {
                first.insert(sum, (i, j));
            }
        }
    }
    assert_eq!(records, 518_671);
    assert_eq!(first.len(), 518_085);
    assert_eq!(infinity_records, 509);
    assert_eq!(identity_collisions, 508);
    assert_eq!(nonidentity_collisions, 78);
    assert_eq!(relations.len(), 13);
    assert_eq!(rank.rank, 13);
    assert_eq!(B - rank.rank, 496);
    let expected_before = (B as f64 * PRIME31.n as f64) / 518_162_f64;
    let expected_after = ((B - rank.rank) as f64 * PRIME31.n as f64) / 518_162_f64;
    println!(
        "{{\"p\":{},\"n\":{},\"base_size\":{},\"pair_records\":{},\"distinct_sums\":{},\"identity_collision_records\":{},\"nonidentity_collision_records\":{},\"projective_relation_classes\":{},\"independent_homogeneous_rank\":{},\"residual_unknown_logs\":{},\"conditional_queries_before\":{:.3},\"conditional_queries_after\":{:.3},\"classification\":\"exact_rank_not_complete_ecdlp\"}}",
        PRIME31.p, PRIME31.n, B, records, first.len(),
        identity_collisions, nonidentity_collisions,
        relations.len(), rank.rank, B-rank.rank, expected_before, expected_after
    );
    Ok(())
}
