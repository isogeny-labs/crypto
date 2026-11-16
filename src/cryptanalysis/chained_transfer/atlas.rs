//! # The transfer atlas: which representations a prime-field ECDLP reaches.
//!
//! A *node* is a representation of the discrete-log instance (an elliptic curve
//! over `F_p`, or the genus-2 Jacobian of a bielliptic cover). An *edge* is an
//! explicit, efficiently evaluable group homomorphism that carries the
//! order-`r` logarithm from one node to another. This module records, for a
//! bielliptic curve `C/F_p` with quotients `E1, E2`, every node reachable from
//! `E1` and classifies each edge, so the question "can we transfer again, and
//! is the destination easier?" is answered by data, not assertion.
//!
//! The reachable set is small and its shape is forced by two facts this module
//! measures:
//!
//! - **Up to the cover, then back.** `φ1^* : E1 → J(C)` lands the logarithm in
//!   the sub-abelian-variety `φ1^*(E1) ⊂ J(C)`. Pushing back with `φ1_*`
//!   returns `[2]` times the point (`φ1_*φ1^* = [2]`); the index-calculus cost
//!   in `J(C)` is higher, not lower (see `measure`).
//! - **Across to the second factor is zero.** The only other elliptic image of
//!   `J(C)` is `φ2_* : J(C) → E2`, and `φ2_* φ1^* = 0`. So the "second
//!   transfer" to the complementary factor **kills the carried logarithm**: it
//!   reaches `E2` only when `E1` and `E2` are already isogenous, and then at no
//!   change in hardness. [`cross_factor_is_zero`] checks this on every sample.
//!
//! The down-edges the research goal names — to an *anomalous* curve (where the
//! log is easy) or to a *binary* curve — are governed by invariants this module
//! reports: an isogeny preserves the trace, so no isogeny edge reaches an
//! anomalous curve unless the start is anomalous; and no edge here changes the
//! characteristic, so a binary node is unreachable (the only known bridge is a
//! global lift of degree `≥ m`, `cryptanalysis:research/xedni_cross_field`).

use super::bielliptic::{Bielliptic, Quotient};
use super::ec::{Curve, Pt};

/// Arithmetic invariants of an elliptic node over `F_p` that decide whether its
/// discrete log is special (easy) and whether an edge can reach it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct NodeFacts {
    pub label: String,
    pub p: u64,
    pub order: u64,
    /// Frobenius trace `t = p + 1 − #E`.
    pub trace: i64,
    /// `#E = p`: the anomalous case, where the additive transfer solves the DLP
    /// in linear time (Smart / SASS / Semaev–Satoh–Araki).
    pub anomalous: bool,
    /// `t ≡ 0 (mod p)` (here `t = 0` for `p > 3`): supersingular, embedding
    /// degree `≤ 6`, pairing-attackable.
    pub supersingular: bool,
    /// Largest prime `r` with `r ∥ #E` (the usable cyclic subgroup).
    pub largest_prime: u64,
    /// Embedding degree of the `largest_prime` subgroup: least `k` with
    /// `r | p^k − 1`, or `None` when `r = p` (anomalous: no `k`) or `k` exceeds
    /// the search bound (cryptographically large, pairing attack infeasible).
    pub embedding_degree: Option<u64>,
}

fn largest_prime_factor_sqfree(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

/// Embedding degree: least `k ≤ bound` with `r | p^k − 1`; `None` otherwise.
pub fn embedding_degree(p: u64, r: u64, bound: u64) -> Option<u64> {
    if r <= 1 || p.is_multiple_of(r) {
        return None; // r = p (anomalous) or r | p: no pairing embedding
    }
    let mut acc = 1u128 % r as u128;
    for k in 1..=bound {
        acc = acc * (p as u128 % r as u128) % r as u128;
        if acc == 1 {
            return Some(k);
        }
    }
    None
}

/// The arithmetic facts of an elliptic node.
pub fn node_facts(e: &Curve, label: &str) -> NodeFacts {
    let p = e.k.p;
    let order = e.order();
    let trace = p as i64 + 1 - order as i64;
    let r = largest_prime_factor_sqfree(order);
    NodeFacts {
        label: label.to_string(),
        p,
        order,
        trace,
        anomalous: order == p,
        supersingular: trace.rem_euclid(p as i64) == 0,
        largest_prime: r,
        embedding_degree: embedding_degree(p, r, 400),
    }
}

/// `φ_{other}_* ∘ φ_q^* (P) = O` for every sampled point `P` on the `q`-factor:
/// the second transfer to the complementary factor kills the carried log.
pub fn cross_factor_is_zero(b: &Bielliptic, q: Quotient, samples: u64) -> bool {
    let e = *b.curve(q);
    let other = match q {
        Quotient::E1 => Quotient::E2,
        Quotient::E2 => Quotient::E1,
    };
    (0..samples).all(|s| {
        let pt: Pt = e.point_from_seed(s.wrapping_mul(0x9E37_79B9) + 1);
        if !b.fiber_generic(q, &pt) {
            return true;
        }
        let d = b.pull(q, &pt);
        b.push(other, &d).is_none()
    })
}

/// The reachable set from `E1`, with each node's facts and the forced edges.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AtlasReport {
    pub p: u64,
    pub e1: NodeFacts,
    pub e2: NodeFacts,
    pub jac_order: u64,
    /// `#E1 = #E2`: the only case in which `E1` and `E2` can be isogenous, i.e.
    /// in which a nonzero map between the two factors exists at all.
    pub factors_same_order: bool,
    /// `φ2_* φ1^* = 0` verified on samples (second transfer to `E2` is zero).
    pub cross_e1_to_e2_zero: bool,
    pub cross_e2_to_e1_zero: bool,
    /// Does any reachable node have an *easier* discrete log than `E1`?
    /// A node is easier if it is anomalous, supersingular, or has a smaller
    /// largest prime subgroup than `E1`. The reachable elliptic nodes are `E1`,
    /// its isogenous curves (same facts), and — only if `factors_same_order` —
    /// `E2`.
    pub easier_node_reachable: bool,
}

/// Build the atlas report for a bielliptic curve.
pub fn atlas_report(b: &Bielliptic, samples: u64) -> AtlasReport {
    let e1 = node_facts(b.curve(Quotient::E1), "E1");
    let e2 = node_facts(b.curve(Quotient::E2), "E2");
    let jac_order = e1.order * e2.order;
    let factors_same_order = e1.order == e2.order;
    // E2 carries the E1-log only through a nonzero cross-factor map, which
    // exists only if the factors are isogenous (⇒ same order). Otherwise the
    // only reachable elliptic nodes are E1 and its isogeny class.
    let e1_special = e1.anomalous || e1.supersingular;
    let e2_reachable_and_easier = factors_same_order
        && (e2.anomalous || e2.supersingular || e2.largest_prime < e1.largest_prime);
    AtlasReport {
        p: b.k.p,
        easier_node_reachable: e1_special || e2_reachable_and_easier,
        e1,
        e2,
        jac_order,
        factors_same_order,
        cross_e1_to_e2_zero: cross_factor_is_zero(b, Quotient::E1, samples),
        cross_e2_to_e1_zero: cross_factor_is_zero(b, Quotient::E2, samples),
    }
}

/// Sweeping the cover of a *fixed* target curve: what the complementary factor
/// becomes. For a target cubic `F` over `F_p`, each shift `c` gives the cover
/// `C_c : y² = F(x² + c)`, whose quotient is the target (same order for all `c`)
/// and whose complementary factor `E2(c)` ranges over many orders. The log
/// never rides the cover to `E2(c)` (`φ2_*φ1^* ≡ 0`); `E2(c)` is isogenous to
/// the target only when `#E2(c) = #target`, and that is the only way a separate
/// map could reach it — at equal order, so no easier.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SweepReport {
    pub p: u64,
    pub target_order: u64,
    pub target_largest_prime: u64,
    /// Valid covers found (nonsingular, with a rational Weierstrass point).
    pub covers: u64,
    /// Number of distinct complement orders seen (size of the spectrum).
    pub distinct_complement_orders: u64,
    pub min_complement_order: u64,
    pub max_complement_order: u64,
    /// Covers whose complement is isogenous to the target (`#E2 = #target`): the
    /// only ones a separate isogeny could use, and then at equal hardness.
    pub complement_isogenous_to_target: u64,
    /// Complements that are anomalous / supersingular / have a strictly smaller
    /// largest prime than the target (structurally easier — but unreachable).
    pub complement_anomalous: u64,
    pub complement_supersingular: u64,
    pub complement_smaller_prime: u64,
    /// Of the structurally-easier complements, how many are *reachable*
    /// (isogenous to the target). The claim is that this is 0: easier ⇒
    /// different structure ⇒ different order ⇒ not isogenous.
    pub easier_and_reachable: u64,
    /// `φ2_*φ1^* = 0` held on every sampled cover (the cover never carries the
    /// log to the complement).
    pub cover_never_carries_log: bool,
}

/// Expand the target cubic `F` shifted by `c`: `G(X) = F(X + c)`, as
/// `[g0, g1, g2, g3]`.
fn shift_cubic(k: &super::field::Fp, f: [u64; 4], c: u64) -> [u64; 4] {
    let [c0, c1, c2, c3] = f;
    let c2m = |a, b| k.mul(a, b);
    let g3 = c3;
    let g2 = k.add(c2, c2m(3, c2m(c3, c)));
    let g1 = k.add(k.add(c1, c2m(2, c2m(c2, c))), c2m(3, c2m(c3, c2m(c, c))));
    let g0 = k.add(
        k.add(c0, c2m(c1, c)),
        k.add(c2m(c2, c2m(c, c)), c2m(c3, c2m(c, c2m(c, c)))),
    );
    [g0, g1, g2, g3]
}

/// Sweep every shift `c ∈ [0, max_c)` of the cover of the target `F`.
pub fn sweep_target(p: u64, f: [u64; 4], max_c: u64) -> Option<SweepReport> {
    let k = super::field::Fp::new(p)?;
    // Target order read straight from the elliptic model of the cubic `F`
    // (the quotient `E1 : Y² = F(X)` in monic form), independent of whether any
    // particular cover has a rational Weierstrass point.
    let target_e1 = Curve::new(k, f[2], k.mul(f[1], f[3]), k.mul(f[0], k.mul(f[3], f[3])))?;
    let target = node_facts(&target_e1, "target");
    let mut orders = std::collections::BTreeSet::new();
    let mut rep = SweepReport {
        p,
        target_order: target.order,
        target_largest_prime: target.largest_prime,
        covers: 0,
        distinct_complement_orders: 0,
        min_complement_order: u64::MAX,
        max_complement_order: 0,
        complement_isogenous_to_target: 0,
        complement_anomalous: 0,
        complement_supersingular: 0,
        complement_smaller_prime: 0,
        easier_and_reachable: 0,
        cover_never_carries_log: true,
    };
    for c in 0..max_c {
        let g = shift_cubic(&k, f, c);
        let Ok(b) = Bielliptic::new(p, g) else {
            continue;
        };
        // The quotient order must stay equal to the target (iso across shifts).
        let e1 = b.curve(Quotient::E1).order();
        if e1 != target.order {
            // A shift can renormalise the monic model; skip the rare mismatch.
            continue;
        }
        let e2 = node_facts(b.curve(Quotient::E2), "E2");
        rep.covers += 1;
        orders.insert(e2.order);
        rep.min_complement_order = rep.min_complement_order.min(e2.order);
        rep.max_complement_order = rep.max_complement_order.max(e2.order);
        let isog = e2.order == target.order;
        rep.complement_isogenous_to_target += isog as u64;
        // "Easier" is RELATIVE to the target: a strictly smaller largest prime,
        // or a special (anomalous/supersingular) complement when the target is
        // not already special. An absolute flag would wrongly count a reachable
        // complement of an already-special target as a win.
        let target_special = target.anomalous || target.supersingular;
        let strictly_easier = e2.largest_prime < target.largest_prime
            || (!target_special && (e2.anomalous || e2.supersingular));
        rep.complement_anomalous += e2.anomalous as u64;
        rep.complement_supersingular += e2.supersingular as u64;
        rep.complement_smaller_prime += (e2.largest_prime < target.largest_prime) as u64;
        // Reachable ⇒ same order ⇒ same trace ⇒ identical hardness, so a
        // reachable complement is never strictly easier. This stays 0.
        rep.easier_and_reachable += (strictly_easier && isog) as u64;
        if rep.covers <= 40 && !cross_factor_is_zero(&b, Quotient::E1, 16) {
            rep.cover_never_carries_log = false;
        }
    }
    rep.distinct_complement_orders = orders.len() as u64;
    if rep.min_complement_order == u64::MAX {
        rep.min_complement_order = 0;
    }
    Some(rep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::chained_transfer::field::Fp;

    #[test]
    fn cover_sweep_hits_many_orders_but_traps_the_log() {
        // Fix a target cubic over F_1009 and sweep its cover.
        let rep = sweep_target(1009, [7, 3, 2, 1], 1009).expect("sweep");
        assert!(rep.covers > 100, "enough valid covers: {}", rep.covers);
        // The complement ranges over many distinct orders across the Hasse
        // interval — different-order target curves the cover reaches.
        assert!(
            rep.distinct_complement_orders > 20,
            "spectrum size {}",
            rep.distinct_complement_orders
        );
        // Many complements are structurally easier (smaller largest prime) ...
        assert!(rep.complement_smaller_prime > 0);
        // ... yet the cover never carries the log to any of them ...
        assert!(rep.cover_never_carries_log, "φ2_*φ1^* = 0 on every cover");
        // ... and no reachable (isogenous) complement is ever strictly easier
        // than the target (same order ⇒ identical hardness) — including when the
        // target is itself special, the case the adversarial sweep flagged.
        assert_eq!(rep.easier_and_reachable, 0, "reachable ⇒ not easier");
    }

    #[test]
    fn cross_factor_zero_and_node_facts() {
        let mut found = 0;
        for seed in 0..400u64 {
            let p = 1009u64;
            let c = [
                1 + (seed.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (seed.wrapping_mul(40_503) >> 5) % p,
                (seed.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (seed.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            let Ok(b) = Bielliptic::new(p, c) else {
                continue;
            };
            let rep = atlas_report(&b, 30);
            // The second-transfer-to-the-other-factor map is always zero.
            assert!(rep.cross_e1_to_e2_zero, "φ2_*φ1^* = 0");
            assert!(rep.cross_e2_to_e1_zero, "φ1_*φ2^* = 0");
            assert_eq!(rep.jac_order, rep.e1.order * rep.e2.order);
            // Trace is consistent with the Hasse bound.
            let bound = 2 * (p as f64).sqrt() as i64 + 1;
            assert!(rep.e1.trace.abs() <= bound && rep.e2.trace.abs() <= bound);
            found += 1;
            if found >= 8 {
                break;
            }
        }
        assert!(found >= 8);
    }

    #[test]
    fn embedding_degree_basics() {
        // r | p^k − 1 at the claimed k.
        let k = Fp::new(1009).unwrap();
        let _ = k;
        assert_eq!(embedding_degree(1009, 101, 400), {
            // compute reference
            let mut acc = 1u128;
            let mut kk = None;
            for i in 1..=400u64 {
                acc = acc * (1009u128 % 101) % 101;
                if acc == 1 {
                    kk = Some(i);
                    break;
                }
            }
            kk
        });
        assert_eq!(embedding_degree(101, 101, 400), None); // anomalous: r = p
    }
}
