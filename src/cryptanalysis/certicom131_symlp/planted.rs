//! Full-size planted run of the double-large-prime pipeline on the real
//! 131-bit groups: relation points built by group operations on ECC2-131
//! or ECCp-131, the [`LpGraph`](super::relgraph::LpGraph) eliminating the
//! large primes, every full relation checked again in the group, linear
//! algebra modulo the 130-bit `r`, and the planted logarithm recovered.
//!
//! **Planted, not a decomposition.**  The factor-base and large-prime
//! points are `[k]G` for known random `k`, and the target is a synthetic
//! `Q' = [d]G`, never the published challenge point.  Each relation's
//! `(a, b)` is computed from the known logarithms, so this exercises the
//! collector, the graph, the scalar ring and the solver at the real width
//! and in the real group, while relation *finding* is left to the
//! oracles and their measured yield (`sym2`, `census`).  Nothing here
//! bears on the published targets.

use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::Rng;
use serde::Serialize;

use super::curves::{Pt, SymCurve};
use super::relgraph::{FullRelation, LpGraph, LpId, Relation};

/// Shape mix of the planted relations (fractions; the rest are full).
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Mix {
    pub one_lp: f64,
    pub two_lp: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlantedReport {
    pub curve: String,
    pub r_bits: u64,
    pub factor_base: usize,
    pub large_primes: usize,
    pub relations_generated: u64,
    pub graph: super::relgraph::GraphStats,
    pub full_relations: usize,
    pub group_checks_passed: usize,
    pub rank: usize,
    pub recovered: bool,
    pub seconds: f64,
}

/// `Σ c·P` by repeated scalar multiplication (checking only).
fn combo<C: SymCurve>(c: &C, terms: &[(&Pt<C::E>, &BigUint)]) -> Option<Pt<C::E>> {
    let mut acc: Option<Pt<C::E>> = None;
    for (p, k) in terms {
        let Some(t) = c.mul(p, k) else { continue };
        acc = match acc {
            None => Some(t),
            Some(a) => c.add(&a, &t),
        };
    }
    acc
}

fn signed(s: i8, r: &BigUint) -> BigUint {
    if s >= 0 {
        BigUint::one()
    } else {
        r - BigUint::one()
    }
}

/// Gaussian elimination modulo prime `r` on rows `[coefs | rhs]`;
/// returns the rank and, if the last unknown is determined, its value.
fn solve_last(mut rows: Vec<Vec<BigUint>>, cols: usize, r: &BigUint) -> (usize, Option<BigUint>) {
    let inv = |x: &BigUint| x.modpow(&(r - BigUint::from(2u8)), r);
    let mut rank = 0;
    let mut pivot_col = vec![usize::MAX; rows.len()];
    for col in 0..cols {
        let Some(p) = (rank..rows.len()).find(|&i| !rows[i][col].is_zero()) else {
            continue;
        };
        rows.swap(rank, p);
        let iv = inv(&rows[rank][col]);
        for v in rows[rank].iter_mut() {
            *v = (&*v * &iv) % r;
        }
        let pivot = rows[rank].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i == rank || row[col].is_zero() {
                continue;
            }
            let f = row[col].clone();
            for (v, pv) in row.iter_mut().zip(pivot.iter()) {
                *v = (&*v + r - (&f * pv) % r) % r;
            }
        }
        pivot_col[rank] = col;
        rank += 1;
    }
    let last = cols - 1;
    let value = (0..rank).find(|&i| pivot_col[i] == last).and_then(|i| {
        // Determined only if no free column feeds the pivot row.
        let free = (0..last).any(|c| !pivot_col[..rank].contains(&c) && !rows[i][c].is_zero());
        (!free).then(|| rows[i][cols].clone())
    });
    (rank, value)
}

/// Run the planted pipeline.  `g` must have prime order `r = c.order()`.
pub fn run<C: SymCurve, R: Rng>(
    c: &C,
    g: &Pt<C::E>,
    nf: usize,
    nl: usize,
    mix: Mix,
    max_relations: u64,
    rng: &mut R,
) -> PlantedReport {
    let t0 = std::time::Instant::now();
    let r = c
        .order()
        .expect("planted run needs the subgroup order")
        .clone();
    // Known logs and their points.
    let fk: Vec<BigUint> = (0..nf)
        .map(|_| rng.gen_biguint_range(&BigUint::one(), &r))
        .collect();
    let lk: Vec<BigUint> = (0..nl)
        .map(|_| rng.gen_biguint_range(&BigUint::one(), &r))
        .collect();
    let fpts: Vec<Pt<C::E>> = fk.iter().map(|k| c.mul(g, k).unwrap()).collect();
    let lpts: Vec<Pt<C::E>> = lk.iter().map(|k| c.mul(g, k).unwrap()).collect();
    let d = rng.gen_biguint_range(&BigUint::one(), &r);
    let q = c.mul(g, &d).unwrap();

    let mut graph = LpGraph::new(r.clone());
    let mut fulls: Vec<FullRelation> = Vec::new();
    let mut generated = 0u64;
    let sgn = |rng: &mut R| -> i8 {
        if rng.gen() {
            1
        } else {
            -1
        }
    };
    // Collect until the system can determine d (nf + 1 unknowns), with a
    // margin, or the budget runs out.
    while generated < max_relations && fulls.len() < nf + 8 {
        generated += 1;
        let u: f64 = rng.gen();
        let (fb, lp): (Vec<(u32, i8)>, Vec<(LpId, i8)>) = if u < mix.two_lp {
            (
                vec![],
                vec![
                    (rng.gen_range(0..nl as u64), sgn(rng)),
                    (rng.gen_range(0..nl as u64), sgn(rng)),
                ],
            )
        } else if u < mix.two_lp + mix.one_lp {
            (
                vec![(rng.gen_range(0..nf as u32), sgn(rng))],
                vec![(rng.gen_range(0..nl as u64), sgn(rng))],
            )
        } else {
            (
                vec![
                    (rng.gen_range(0..nf as u32), sgn(rng)),
                    (rng.gen_range(0..nf as u32), sgn(rng)),
                ],
                vec![],
            )
        };
        // The relation point in the group, and its (a, b) from the logs.
        let mut log = BigUint::zero();
        let mut terms: Vec<(Pt<C::E>, BigUint)> = Vec::new();
        for &(i, s) in &fb {
            log = (log + signed(s, &r) * &fk[i as usize]) % &r;
            terms.push((fpts[i as usize], signed(s, &r)));
        }
        for &(k, t) in &lp {
            log = (log + signed(t, &r) * &lk[k as usize]) % &r;
            terms.push((lpts[k as usize], signed(t, &r)));
        }
        let b = rng.gen_biguint_below(&r);
        let a = (&log + &r - (&b * &d) % &r) % &r;
        // Group check of the planted relation itself: [a]G + [b]Q = Σ ±X.
        let lhs = combo(c, &[(g, &a), (&q, &b)]);
        let refs: Vec<(&Pt<C::E>, &BigUint)> = terms.iter().map(|(p, k)| (p, k)).collect();
        assert_eq!(
            lhs,
            combo(c, &refs),
            "planted relation is false in the group"
        );
        if let Some(full) = graph.add(Relation { a, b, fb, lp }) {
            fulls.push(full);
        }
    }

    // Every full relation again in the group, from its own coefficients.
    let mut checked = 0;
    for full in &fulls {
        assert!(graph.replay(full));
        let lhs = combo(c, &[(g, &full.a), (&q, &full.b)]);
        let refs: Vec<(&Pt<C::E>, &BigUint)> = full
            .fb
            .iter()
            .map(|(i, k)| (&fpts[*i as usize], k))
            .collect();
        if lhs == combo(c, &refs) {
            checked += 1;
        }
    }

    // a + b·d = Σ c_i f_i  ⇒  rows [c_0 … c_{nf−1}, −b | a] in unknowns (f, d).
    let cols = nf + 1;
    let rows: Vec<Vec<BigUint>> = fulls
        .iter()
        .map(|full| {
            let mut row = vec![BigUint::zero(); cols + 1];
            for (i, k) in &full.fb {
                row[*i as usize] = k.clone();
            }
            row[nf] = (&r - &full.b % &r) % &r;
            row[cols] = full.a.clone();
            row
        })
        .collect();
    let (rank, value) = solve_last(rows, cols, &r);
    let recovered = value.as_ref() == Some(&d) && c.mul(g, &d) == Some(q);

    PlantedReport {
        curve: c.name().to_string(),
        r_bits: r.bits(),
        factor_base: nf,
        large_primes: nl,
        relations_generated: generated,
        graph: graph.stats.clone(),
        full_relations: fulls.len(),
        group_checks_passed: checked,
        rank,
        recovered,
        seconds: t0.elapsed().as_secs_f64(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::curves::{BinaryW, PrimeW};
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    fn check(rep: &PlantedReport) {
        assert!(rep.recovered, "{rep:?}");
        assert_eq!(rep.group_checks_passed, rep.full_relations, "{rep:?}");
        assert!(
            rep.graph.full_from_cycles > 0,
            "no relation came through a cycle: {rep:?}"
        );
        assert!(rep.r_bits >= 130);
    }

    #[test]
    fn planted_double_large_prime_recovers_d_on_ecc2_131() {
        let (c, g, _) = BinaryW::ecc2_131();
        let mut rng = StdRng::seed_from_u64(21);
        let mix = Mix {
            one_lp: 0.4,
            two_lp: 0.4,
        };
        check(&run(&c, &g, 12, 24, mix, 4000, &mut rng));
    }

    #[test]
    fn planted_double_large_prime_recovers_d_on_eccp_131() {
        let (c, g, _) = PrimeW::eccp_131();
        let mut rng = StdRng::seed_from_u64(22);
        let mix = Mix {
            one_lp: 0.4,
            two_lp: 0.4,
        };
        check(&run(&c, &g, 12, 24, mix, 4000, &mut rng));
    }
}
