//! # Selecting the abscissa subspace of `binary-subspace`
//!
//! The default `binary-subspace` base is the polynomial-basis prefix
//! `<1, z, ..., z^(l-1)>`.  This module adds two opt-in selection rules
//! ported from the `ml-cryptanalysis` factor-base cost-model experiments
//! (`fb-cost-model-construct`, `fb-cost-model-rank`):
//!
//! * `select=strict4e`: a sieve-constructed subspace `V` of dimension `l`
//!   in which every non-zero abscissa lies in `4E` (a *strict* subspace).
//! * `select=gated-rank`: draw `candidates` subspaces (strict sieve draws,
//!   a few unconstrained random ones and the default prefix), drop those
//!   whose exact class-compatible decomposition ceiling is zero, and keep
//!   the one with the smallest rank-aware key (see [`gated_rank_key`]).
//!
//! ## The exact `4E` test (construct report, section 1, T4)
//!
//! On `y^2 + xy = x^3 + 1` over `F_{2^n}`, `n` odd, a point with abscissa
//! `x != 0` lies in `4E` iff `Tr x = 0`, `Tr(1/x) = 0` and
//! `Tr(x*z) = Tr(1) = 1` for `z^2 + z = 1/x`.  The second root `z + 1`
//! gives the same value because `Tr x = 0`.  With cofactor `h = 4`,
//! `4E = <G>`.  The test is only valid for `a = 0, b = 1`, `n` odd and
//! `h = 4`; any other instance is refused (never silently approximated).
//!
//! ## Cost accounting
//!
//! The sieve scans every abscissa of the field once (`2^n` membership
//! tests, each an inversion, an Artin-Schreier solve and three traces) and
//! then does subset-membership look-ups.  None of that is priced by the
//! framework's group-operation counters, so it is recorded as
//! `*_uncharged` counters on the base's cost (AGENTS.md section 5a).

use crate::cryptanalysis::ic_boundary::{
    binary_subspace_factor_base, exact_decomposition_ceiling, BinaryGroup, BinaryInstance,
    FactorBase,
};
use crate::cryptanalysis::koblitz_fast::FastPoint;

/// Largest field degree for which the `2^n`-bit `4E` bitmap is built.
pub const MAX_SIEVE_DEGREE: u32 = 26;

/// Why `strict4e` cannot be offered on an instance, or `None` if it can.
pub fn strict4e_refusal(inst: &BinaryInstance) -> Option<String> {
    if inst.n.is_multiple_of(2) {
        return Some(format!(
            "n = {} is even; the 4E criterion needs odd n",
            inst.n
        ));
    }
    if inst.cofactor != 4 {
        return Some(format!(
            "cofactor h = {} is not 4; the 4E criterion equals <G> only when h = 4",
            inst.cofactor
        ));
    }
    if inst.a != 0 || inst.b != 1 {
        return Some(
            "the 4E criterion is proved for y^2 + xy = x^3 + 1 (a = 0, b = 1) only".into(),
        );
    }
    if inst.n > MAX_SIEVE_DEGREE {
        return Some(format!(
            "n = {} exceeds the bitmap limit {MAX_SIEVE_DEGREE}",
            inst.n
        ));
    }
    None
}

/// The exact `4E` membership test on an abscissa (`x != 0`).
pub fn in_4e(inst: &BinaryInstance, x: u64) -> bool {
    if x == 0 {
        return false;
    }
    let ash = &inst.artin_schreier;
    if ash.trace(x) != 0 {
        return false;
    }
    let f = &inst.fast.field;
    let inv = f.inv(x);
    if ash.trace(inv) != 0 {
        return false;
    }
    let Some(z) = ash.solve(inv) else {
        return false;
    };
    // Tr(1) = 1 for odd n.
    ash.trace(f.mul(x, z)) == 1
}

/// Bitmap of `X4 = { x != 0 : x in 4E }` over all of `F_{2^n}`.
pub struct X4Map {
    pub n: u32,
    bits: Vec<u64>,
    pub count: u64,
}

impl X4Map {
    pub fn build(inst: &BinaryInstance) -> Self {
        let size = 1u64 << inst.n;
        let mut bits = vec![0u64; (size as usize).div_ceil(64)];
        let mut count = 0;
        for x in 1..size {
            if in_4e(inst, x) {
                bits[(x >> 6) as usize] |= 1 << (x & 63);
                count += 1;
            }
        }
        Self {
            n: inst.n,
            bits,
            count,
        }
    }
    #[inline]
    pub fn contains(&self, x: u64) -> bool {
        (self.bits[(x >> 6) as usize] >> (x & 63)) & 1 == 1
    }
}

/// Deterministic splitmix64 stream.
pub struct SplitMix(pub u64);
impl SplitMix {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// A constructed subspace with its measured strictness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedBasis {
    pub basis: Vec<u64>,
    /// Non-zero elements of the span that lie in `4E`.
    pub x4_in_span: u64,
    /// `2^l - 1`.
    pub span_nonzero: u64,
    /// Every non-zero element is in `4E`.
    pub strict: bool,
    /// Number of basis vectors chosen by the fallback rule.
    pub fallback_steps: u32,
    /// Subset-membership look-ups performed (uncharged work).
    pub lookups: u64,
}

fn span(basis: &[u64]) -> Vec<u64> {
    let mut v = vec![0u64];
    for &b in basis {
        let k = v.len();
        for i in 0..k {
            v.push(v[i] ^ b);
        }
    }
    v
}

/// The sieve constructor: next vector uniform among `{v in X4 : v + w in X4
/// for all w in V}`; when that set is empty, the vector outside `V`
/// maximising the number of `X4` elements in the enlarged span (ties
/// uniform).  Seeded and deterministic.  Coordinate-descent polishing of
/// the original constructor is not ported.
pub fn strict4e_sieve(map: &X4Map, l: u32, seed: u64) -> Result<SelectedBasis, String> {
    if l == 0 || l > 30 || l >= map.n {
        return Err(format!("dimension {l} outside 1..{}", map.n));
    }
    let mut rng = SplitMix(seed ^ 0xA5A5_5A5A_1234_5678);
    let size = 1u64 << map.n;
    let mut basis: Vec<u64> = Vec::new();
    let mut vs = vec![0u64];
    let mut fallback_steps = 0;
    let mut lookups = 0u64;
    for _ in 0..l {
        // Strict extension candidates, reservoir-sampled.
        let mut pick = 0u64;
        let mut seen = 0u64;
        for v in 1..size {
            if !map.contains(v) {
                continue;
            }
            let mut ok = true;
            for &w in &vs[1..] {
                lookups += 1;
                if !map.contains(v ^ w) {
                    ok = false;
                    break;
                }
            }
            if ok {
                seen += 1;
                if rng.next_u64().is_multiple_of(seen) {
                    pick = v;
                }
            }
        }
        if seen == 0 {
            fallback_steps += 1;
            let mut best = 0u64;
            let mut best_n = 0u64;
            let mut ties = 0u64;
            for v in 1..size {
                if vs.contains(&v) {
                    continue;
                }
                let mut c = u64::from(map.contains(v));
                for &w in &vs[1..] {
                    lookups += 1;
                    c += u64::from(map.contains(v ^ w));
                }
                // Equal count counts the new coset once per element of V.
                match c.cmp(&best_n) {
                    std::cmp::Ordering::Greater => {
                        best_n = c;
                        best = v;
                        ties = 1;
                    }
                    std::cmp::Ordering::Equal => {
                        ties += 1;
                        if rng.next_u64().is_multiple_of(ties) {
                            best = v;
                        }
                    }
                    _ => {}
                }
            }
            pick = best;
        }
        basis.push(pick);
        vs = span(&basis);
    }
    let x4_in_span = vs[1..].iter().filter(|&&x| map.contains(x)).count() as u64;
    let span_nonzero = (1u64 << l) - 1;
    Ok(SelectedBasis {
        basis,
        x4_in_span,
        span_nonzero,
        strict: x4_in_span == span_nonzero,
        fallback_steps,
        lookups,
    })
}

/// A uniformly random `l`-dimensional subspace basis (unconstrained).
pub fn random_basis(n: u32, l: u32, seed: u64) -> Vec<u64> {
    let mut rng = SplitMix(seed ^ 0x0DDB_1A5E_5BAD_5EED);
    let mut basis: Vec<u64> = Vec::new();
    let mut piv = [0u64; 64];
    while basis.len() < l as usize {
        let w = rng.next_u64() & ((1u64 << n) - 1);
        let mut x = w;
        while x != 0 {
            let k = 63 - x.leading_zeros() as usize;
            if piv[k] == 0 {
                piv[k] = x;
                break;
            }
            x ^= piv[k];
        }
        if x != 0 {
            basis.push(w);
        }
    }
    basis
}

/// Rank-aware key of a candidate base (lower is better):
/// `log2(ops/P) + log2(rank3^1.7 + 1)`.
///
/// Substitutions, stated plainly: the Python ridge for `log2(ops/P)` is not
/// available.  Candidates here share dimension, oracle and instance, so
/// per-query operations are equal and `ops/P` varies with the predicted
/// yield `P` alone; `P` is proportional to the exact class-compatible
/// ceiling `N0` (`exact_decomposition_ceiling`), so `log2(ops/P)` is
/// replaced by `-log2(N0/r)` (constant dropped).  The exact achievable
/// `rank3` is replaced by its upper bound, the number of columns.
pub fn gated_rank_key(ceiling: f64, columns: usize) -> f64 {
    -ceiling.log2() + ((columns as f64).powf(1.7) + 1.0).log2()
}

/// One scored candidate of `select=gated-rank`.
#[derive(Clone, Debug)]
pub struct ScoredCandidate {
    pub basis: Vec<u64>,
    pub origin: String,
    pub strict: bool,
    pub ceiling: f64,
    pub columns: usize,
    /// `None` when gated out (zero predicted yield).
    pub key: Option<f64>,
}

/// Draw candidates, gate on zero ceiling, pick the minimal key.  Returns the
/// winner and every scored candidate.
pub fn gated_rank_select(
    inst: &BinaryInstance,
    map: &X4Map,
    l: u32,
    seed: u64,
    candidates: u32,
    m: u32,
) -> Result<(SelectedBasis, Vec<ScoredCandidate>), String> {
    let mut pool: Vec<(Vec<u64>, String, SelectedBasis)> = Vec::new();
    let mk_plain = |basis: Vec<u64>| {
        let x4 = span(&basis)[1..]
            .iter()
            .filter(|&&x| map.contains(x))
            .count() as u64;
        SelectedBasis {
            x4_in_span: x4,
            span_nonzero: (1u64 << basis.len()) - 1,
            strict: x4 == (1u64 << basis.len()) - 1,
            basis,
            fallback_steps: 0,
            lookups: 0,
        }
    };
    for i in 0..candidates {
        let s = strict4e_sieve(map, l, seed.wrapping_mul(1_000_003).wrapping_add(i as u64))?;
        pool.push((s.basis.clone(), format!("sieve#{i}"), s));
    }
    for i in 0..candidates.div_ceil(4) {
        let b = random_basis(inst.n, l, seed.wrapping_mul(7_919).wrapping_add(i as u64));
        pool.push((b.clone(), format!("random#{i}"), mk_plain(b)));
    }
    let prefix: Vec<u64> = (0..l).map(|k| 1u64 << k).collect();
    pool.push((prefix.clone(), "prefix".into(), mk_plain(prefix)));

    let g = BinaryGroup(&inst.fast);
    let mut scored = Vec::new();
    let mut best: Option<(f64, usize)> = None;
    for (idx, (basis, origin, sel)) in pool.iter().enumerate() {
        let fb: FactorBase<FastPoint> = binary_subspace_factor_base(inst, basis);
        let ceiling = exact_decomposition_ceiling(&g, &fb.points, inst.r, m)
            .map(|(_, c)| c)
            .unwrap_or(0.0);
        let key = (ceiling > 0.0 && fb.columns > 0).then(|| gated_rank_key(ceiling, fb.columns));
        if let Some(k) = key {
            if best.is_none_or(|(b, _)| k < b) {
                best = Some((k, idx));
            }
        }
        scored.push(ScoredCandidate {
            basis: basis.clone(),
            origin: origin.clone(),
            strict: sel.strict,
            ceiling,
            columns: fb.columns,
            key,
        });
    }
    let (_, idx) = best.ok_or("every candidate subspace has zero predicted yield")?;
    Ok((pool.swap_remove(idx).2, scored))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::koblitz_instance;

    fn smul(inst: &BinaryInstance, p: FastPoint, k: u64) -> FastPoint {
        use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
        BinaryGroup(&inst.fast).mul(&mut GroupOps::default(), p, k)
    }

    fn ground_truth(n: u32) {
        let inst = koblitz_instance(0, n).unwrap();
        assert_eq!(inst.cofactor, 4, "n={n}");
        assert!(strict4e_refusal(&inst).is_none());
        let mut members = 0u64;
        for x in 1..(1u64 << n) {
            let pts = inst.points_with_x(x);
            let truth = match pts.first() {
                // Membership in 4E = <G> when h = 4: [r]P = O.
                Some(&p) => smul(&inst, p, inst.r).infinity,
                None => false,
            };
            assert_eq!(in_4e(&inst, x), truth, "n={n} x={x:#x}");
            members += u64::from(truth);
        }
        // Each member abscissa carries two points of <G>: |<G>| = r + ... .
        assert_eq!(
            2 * members + 1,
            inst.r,
            "n={n}: <G> minus O has 2 points per x"
        );
    }

    #[test]
    fn criterion_matches_scalar_multiplication_n13() {
        ground_truth(13);
    }

    #[test]
    fn criterion_matches_scalar_multiplication_n19() {
        ground_truth(19);
    }

    #[test]
    fn four_e_equals_subgroup_via_e4_too() {
        // [r]P in E[4] for every point (h = 4), and P in <G> iff [r]P = O.
        let inst = koblitz_instance(0, 13).unwrap();
        for x in 1..(1u64 << 13) {
            if let Some(&p) = inst.points_with_x(x).first() {
                let c = smul(&inst, p, inst.r);
                assert!(smul(&inst, c, 4).infinity);
            }
        }
    }

    #[test]
    fn refuses_even_n_and_wrong_cofactor() {
        // n = 17, a = 0 has h = 548 (not 4); a = 1 has h = 2; even n is refused first.
        let mut refused = 0;
        for (a, n) in [(0u8, 17u32), (1, 17), (1, 13), (0, 14), (0, 16), (1, 14)] {
            if let Some(inst) = koblitz_instance(a, n) {
                assert!(
                    strict4e_refusal(&inst).is_some(),
                    "a={a} n={n} h={}",
                    inst.cofactor
                );
                refused += 1;
            }
        }
        assert!(refused >= 1, "no refusal case constructible");
        let ok = koblitz_instance(0, 13).unwrap();
        assert!(strict4e_refusal(&ok).is_none());
    }

    #[test]
    fn default_path_is_unchanged_and_select_refusal_is_clear() {
        use crate::cryptanalysis::ic_boundary::GroupOps;
        use crate::cryptanalysis::ic_framework::plugins::BinarySubspaceBase;
        use crate::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
        let inst = koblitz_instance(0, 13).unwrap();
        let g = BinaryGroup(&inst.fast);
        let ctx = InstanceCtx {
            group: &g,
            generator: inst.generator,
            target: inst.generator,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        };
        let b = BinarySubspaceBase { instance: &inst };
        let build = |spec: &str| {
            let (_, p) = Params::parse_spec(spec).unwrap();
            b.build(&ctx, &p, &mut GroupOps::default())
        };
        let plain = build("binary-subspace:dimension=3").unwrap();
        let prefix = build("binary-subspace:dimension=3,select=prefix").unwrap();
        assert_eq!(plain.points, prefix.points);
        assert_eq!(plain.subspace_basis, Some(vec![1, 2, 4]));
        let s1 = build("binary-subspace:dimension=3,select=strict4e,seed=7").unwrap();
        let s2 = build("binary-subspace:dimension=3,select=strict4e,seed=7").unwrap();
        assert_eq!(s1.subspace_basis, s2.subspace_basis);
        assert_eq!(
            s1.cost.native.get("strict4e_x4_in_span"),
            s1.cost.native.get("strict4e_span_nonzero")
        );
        assert!(build("binary-subspace:dimension=3,select=bogus").is_err());
        assert!(build("binary-subspace:dimension=3,select=strict4e,basis=1;2;4").is_err());
    }

    #[test]
    fn constructed_bases_are_strict_and_deterministic() {
        let inst = koblitz_instance(0, 13).unwrap();
        let map = X4Map::build(&inst);
        for l in [2u32, 3] {
            let mut strict = 0;
            for seed in 0..8 {
                let a = strict4e_sieve(&map, l, seed).unwrap();
                let b = strict4e_sieve(&map, l, seed).unwrap();
                assert_eq!(a, b, "deterministic per seed");
                // The recorded strictness is the truth about the span.
                let truth = span(&a.basis)[1..].iter().all(|&x| in_4e(&inst, x));
                assert_eq!(a.strict, truth);
                strict += u64::from(a.strict);
            }
            assert!(strict > 0, "n=13 l={l}: no strict base in 8 seeds");
        }
        // l = 4 has no strict subspace at n = 13 (exact count 0): fallback.
        let f = strict4e_sieve(&map, 4, 1).unwrap();
        assert!(!f.strict && f.fallback_steps >= 1);
        assert!(f.x4_in_span >= 12, "fallback keeps most of the span in 4E");
    }

    #[test]
    fn gated_rank_is_deterministic_and_never_zero_yield() {
        let inst = koblitz_instance(0, 13).unwrap();
        let map = X4Map::build(&inst);
        let (a, sa) = gated_rank_select(&inst, &map, 3, 5, 6, 3).unwrap();
        let (b, _) = gated_rank_select(&inst, &map, 3, 5, 6, 3).unwrap();
        assert_eq!(a, b);
        let win = sa.iter().find(|c| c.basis == a.basis).unwrap();
        assert!(win.ceiling > 0.0 && win.key.is_some());
    }
}
