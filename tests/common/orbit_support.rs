//! Support shared by `tests/ecbench_large_prime.rs` and the orbit examples
//! (`examples/orbit_large_prime_bench.rs`, `examples/orbit_window_structure.rs`),
//! included with `#[path = ...] mod orbit_support;`.
//!
//! The first half is an **independent oracle** for the Frobenius-orbit
//! structure of a large-prime factor base.  It deliberately does not call the
//! library's `orbit::canonicalise`, `LambdaPowers` or the table build in
//! `ecbench_large_prime`: it recomputes the factor base from its
//! specification (canonical projected points of the abscissae `x < 2^env`,
//! small iff some abscissa `x < 2^small` projects to it) and decides which
//! points are Frobenius conjugates by brute force, pair by pair, with
//! `frobenius_k` and `neg` on the curve.  A test that compares the library's
//! counts with these is therefore not tautological.
//!
//! The second half builds an instance over a chosen irreducible modulus (the
//! same abstract Koblitz curve in a different polynomial basis) and the
//! seeded random modulus search used by the realisation series.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};

use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, IrreduciblePoly};
use crypto_lib::cryptanalysis::ecbench_large_prime::binary_explicit_instance;
use crypto_lib::cryptanalysis::ic_boundary::{ArtinSchreier, BinaryInstance};
use crypto_lib::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crypto_lib::cryptanalysis::koblitz_index_calculus::is_irreducible_f2;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;

/// `SplitMix64`, the generator used by the bench for planted logarithms.
pub struct SplitMix(pub u64);

impl SplitMix {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

// ------------------------------------------------------------------ oracle

/// The member of `{P, -P}` with the smaller `(x, y)`.
pub fn canonical(fast: &FastCurve, p: FastPoint) -> FastPoint {
    let neg = fast.neg(p);
    if (p.x, p.y) <= (neg.x, neg.y) {
        p
    } else {
        neg
    }
}

/// One canonical projected point of the factor base and the raw abscissae
/// that project onto it.
#[derive(Clone, Debug)]
pub struct OraclePoint {
    pub point: FastPoint,
    pub raw_abscissae: Vec<u64>,
}

/// The factor base recomputed from its specification.
#[derive(Clone, Debug)]
pub struct OracleBase {
    pub small: Vec<OraclePoint>,
    pub large: Vec<OraclePoint>,
}

/// Canonical projected points of every raw point with abscissa `x < 2^env`;
/// a point is small iff some raw point with `x < 2^small` projects onto it.
/// Both lists are in increasing packed-key order.
pub fn oracle_base(inst: &BinaryInstance, small_dim: u32, env_dim: u32) -> OracleBase {
    let mut by_key: BTreeMap<u64, (OraclePoint, bool)> = BTreeMap::new();
    for x in 0..1u64 << env_dim {
        for raw in inst.points_with_x(x) {
            let projected = inst.fast.mul_u64(raw, inst.cofactor);
            if projected.infinity {
                continue;
            }
            let c = canonical(&inst.fast, projected);
            let entry = by_key.entry(c.pack()).or_insert_with(|| {
                (
                    OraclePoint {
                        point: c,
                        raw_abscissae: Vec::new(),
                    },
                    false,
                )
            });
            entry.1 |= x < 1u64 << small_dim;
            if !entry.0.raw_abscissae.contains(&x) {
                entry.0.raw_abscissae.push(x);
            }
        }
    }
    let mut base = OracleBase {
        small: Vec::new(),
        large: Vec::new(),
    };
    for (_, (point, small)) in by_key {
        if small {
            base.small.push(point);
        } else {
            base.large.push(point);
        }
    }
    base
}

/// The packed canonical keys of `{ +-pi^k(p) : 0 <= k < n }`.
pub fn orbit_keys(inst: &BinaryInstance, p: FastPoint) -> HashSet<u64> {
    let mut keys = HashSet::new();
    let mut q = p;
    for _ in 0..inst.n {
        keys.insert(canonical(&inst.fast, q).pack());
        q = inst.fast.frobenius_k(q, 1);
    }
    keys
}

/// Union-find over `0..n`.
struct Dsu(Vec<usize>);

impl Dsu {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }
    fn find(&mut self, mut i: usize) -> usize {
        while self.0[i] != i {
            self.0[i] = self.0[self.0[i]];
            i = self.0[i];
        }
        i
    }
    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.0[b.max(a)] = b.min(a);
        }
    }
}

/// Connected components of `points` under "is a signed Frobenius conjugate
/// of", decided by testing every pair `(i, j)`, `i < j`.  Returns each
/// point's component label (the smallest member index).
pub fn conjugacy_components(inst: &BinaryInstance, points: &[FastPoint]) -> Vec<usize> {
    let images: Vec<HashSet<u64>> = points.iter().map(|&p| orbit_keys(inst, p)).collect();
    let packs: Vec<u64> = points
        .iter()
        .map(|&p| canonical(&inst.fast, p).pack())
        .collect();
    let mut dsu = Dsu::new(points.len());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if images[i].contains(&packs[j]) {
                dsu.union(i, j);
            }
        }
    }
    (0..points.len()).map(|i| dsu.find(i)).collect()
}

/// What the library reports as `large_points`, `large_orbits`,
/// `merged_points` and `absorbed_into_small`, recomputed by brute force.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OracleCounts {
    pub large_points: usize,
    /// Orbits among all large points (the plain-orbit arms).
    pub orbits: usize,
    /// `large_points - orbits`.
    pub merged: usize,
    /// Large points whose orbit contains a small point.
    pub absorbed: usize,
    /// Orbits among the large points that are not absorbed.
    pub orbits_after_absorb: usize,
    /// `large_points - absorbed - orbits_after_absorb` (the absorb arm).
    pub merged_after_absorb: usize,
}

pub fn brute_force_counts(inst: &BinaryInstance, base: &OracleBase) -> OracleCounts {
    let large: Vec<FastPoint> = base.large.iter().map(|p| p.point).collect();
    let labels = conjugacy_components(inst, &large);
    let orbits = labels
        .iter()
        .enumerate()
        .filter(|&(i, &label)| i == label)
        .count();
    let small_packs: HashSet<u64> = base.small.iter().map(|p| p.point.pack()).collect();
    let absorbed_flags: Vec<bool> = large
        .iter()
        .map(|&p| orbit_keys(inst, p).iter().any(|k| small_packs.contains(k)))
        .collect();
    let absorbed = absorbed_flags.iter().filter(|&&a| a).count();
    let rest: Vec<FastPoint> = large
        .iter()
        .zip(&absorbed_flags)
        .filter(|(_, &a)| !a)
        .map(|(&p, _)| p)
        .collect();
    let rest_labels = conjugacy_components(inst, &rest);
    let orbits_after_absorb = rest_labels
        .iter()
        .enumerate()
        .filter(|&(i, &label)| i == label)
        .count();
    OracleCounts {
        large_points: large.len(),
        orbits,
        merged: large.len() - orbits,
        absorbed,
        orbits_after_absorb,
        merged_after_absorb: large.len() - absorbed - orbits_after_absorb,
    }
}

/// The `m` with `xa^(2^m) == xb`, `0 <= m < n`, if any.
pub fn raw_frobenius_power(inst: &BinaryInstance, xa: u64, xb: u64) -> Option<u32> {
    let mut x = xa;
    for m in 0..inst.n {
        if x == xb {
            return Some(m);
        }
        x = inst.fast.field.sqr(x);
    }
    None
}

/// The merges of the plain-orbit table split by cause.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MergeSplit {
    /// Orbits with more than one large member.
    pub multi_member_orbits: usize,
    pub merged: usize,
    /// Merges whose members are raw-abscissa Frobenius images of each other.
    pub raw_frobenius: usize,
    /// Merges that exist only after the cofactor projection.
    pub projected_only: usize,
    /// Size of the largest orbit.
    pub largest_orbit: usize,
}

/// For every orbit of the large points, group its members into classes of
/// raw-abscissa Frobenius images (`x1 = x0^(2^m)` for raw abscissae of the
/// two members).  A class of `c` members accounts for `c - 1` raw-Frobenius
/// merges; joining `k` classes into one orbit takes `k - 1` further merges,
/// which exist only after projection.
pub fn classify_merges(inst: &BinaryInstance, base: &OracleBase) -> MergeSplit {
    let large: Vec<FastPoint> = base.large.iter().map(|p| p.point).collect();
    let labels = conjugacy_components(inst, &large);
    let mut orbits: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &label) in labels.iter().enumerate() {
        orbits.entry(label).or_default().push(i);
    }
    let mut split = MergeSplit::default();
    for members in orbits.values().filter(|m| m.len() > 1) {
        split.multi_member_orbits += 1;
        split.merged += members.len() - 1;
        split.largest_orbit = split.largest_orbit.max(members.len());
        let mut dsu = Dsu::new(members.len());
        for a in 0..members.len() {
            for b in a + 1..members.len() {
                let linked = base.large[members[a]].raw_abscissae.iter().any(|&xa| {
                    base.large[members[b]]
                        .raw_abscissae
                        .iter()
                        .any(|&xb| raw_frobenius_power(inst, xa, xb).is_some())
                });
                if linked {
                    dsu.union(a, b);
                }
            }
        }
        let classes = (0..members.len()).filter(|&i| dsu.find(i) == i).count();
        split.raw_frobenius += members.len() - classes;
        split.projected_only += classes - 1;
    }
    split
}

// -------------------------------------------------- instances over a modulus

/// A seeded random irreducible modulus of degree `n` (bit `n` and bit 0 set),
/// with at least `ceil(n/3) + 2` terms.  `which = 1` is the realisation
/// series' modulus.
pub fn random_modulus(n: u32, which: u64) -> u64 {
    assert!((3..=62).contains(&n) && which >= 1);
    let mut rng = SplitMix(0x006d_6f64_756c_7573 ^ u64::from(n) ^ ((which - 1) << 48));
    let below = (1u64 << n) - 2;
    loop {
        let candidate = (1u64 << n) | 1 | (rng.next() & below);
        if candidate.count_ones() < n.div_ceil(3) + 2 {
            continue;
        }
        if is_irreducible_f2(candidate) {
            return candidate;
        }
    }
}

/// The Koblitz curve `K_a` over `F_{2^n}` represented over `modulus`, with a
/// generator of its order-`r` subgroup found by cofactor-clearing the first
/// point that gives a non-identity multiple.  The group order and `r` are
/// those of the abstract curve (independent of the representation) and are
/// re-certified by `binary_explicit_instance`.
pub fn instance_with_modulus(
    a: u64,
    n: u32,
    modulus: u64,
    group_order: u64,
    r: u64,
) -> Result<BinaryInstance, String> {
    let low_terms = (0..n).filter(|i| (modulus >> i) & 1 == 1).collect();
    let irreducible = IrreduciblePoly {
        degree: n,
        low_terms,
    };
    let gf = Gf2::new(&irreducible);
    let ash = ArtinSchreier::new(&gf);
    let h = group_order / r;
    let shell = BinaryCurve {
        m: n,
        irreducible,
        a: gf.to_element(a),
        b: gf.to_element(1),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(r),
        cofactor: BigUint::from(h),
    };
    let fast = FastCurve::new(&shell).ok_or("curve too wide for the fast arithmetic")?;
    for x in 2..1u64 << n {
        let inv = fast.field.inv(x);
        // y = x u with u^2 + u = x + a + b/x^2.
        let c = x ^ a ^ fast.field.sqr(inv);
        let Some(u) = ash.solve(c) else {
            continue;
        };
        let g = fast.mul_u64(FastPoint::affine(x, fast.field.mul(x, u)), h);
        if !g.infinity && fast.mul_u64(g, r).infinity {
            return binary_explicit_instance(n, modulus, a, 1, group_order, r, g.x, g.y);
        }
    }
    Err(format!(
        "no subgroup generator found over modulus 0x{modulus:x}"
    ))
}
