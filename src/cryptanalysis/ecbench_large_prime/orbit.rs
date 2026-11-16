//! Signed Frobenius-orbit quotient of large-prime columns, and the
//! curve-level replay that checks it.
//!
//! On a Koblitz curve the 2-power Frobenius `pi(x, y) = (x^2, y^2)` is an
//! endomorphism of `E(F_{2^n})` that acts on the prime-order subgroup
//! `<G>` as multiplication by an eigenvalue `lambda` with
//! `lambda^2 - t*lambda + 2 = 0 (mod r)` and `lambda^n = 1`.  Hence, for a
//! subgroup element `Q`,
//!
//! ```text
//!     log pi^k(Q) = lambda^k * log Q,        log(-Q) = -log Q.
//! ```
//!
//! A large prime `Q` therefore belongs to a *signed Frobenius orbit*
//! `{ +-pi^k(Q) : 0 <= k < n }`, and two partial relations whose large primes
//! are different members of one orbit can cancel *provided the coefficient
//! carries the phase*: if `Q = eps * pi^{-k}(Q0)` for the orbit
//! representative `Q0`, the column of `Q0` receives `eps * lambda^{-k}` times
//! the coefficient of `Q`.  Merging orbit members by identifier alone, without
//! the factor, would cancel unequal group elements; [`RelationFrame`] is the
//! independent check that catches exactly that.

use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::cryptanalysis::ic_boundary::BinaryInstance;
use crate::cryptanalysis::koblitz_fast::FastPoint;

use super::canonical;

#[inline]
pub(super) fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// `a^{-1} mod m` for prime `m`, or `None` when `a == 0 (mod m)`.
pub(super) fn inv_mod(a: u64, m: u64) -> Option<u64> {
    let a = a % m;
    if a == 0 {
        return None;
    }
    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    (old_r == 1).then(|| old_s.rem_euclid(m as i128) as u64)
}

/// The powers `lambda^j (mod r)`, `0 <= j < n`, of the Frobenius eigenvalue
/// on the order-`r` subgroup.
#[derive(Clone, Debug)]
pub struct LambdaPowers {
    pub r: u64,
    pub n: u32,
    pub lambda: u64,
    pow: Vec<u64>,
}

impl LambdaPowers {
    /// Certify the eigenvalue of `instance` and tabulate its powers.
    ///
    /// Refuses unless the instance is a Koblitz curve, `lambda` satisfies the
    /// Frobenius characteristic equation modulo `r`, `lambda^n = 1`, and
    /// `pi(G) = [lambda]G` on the actual generator.
    pub fn new(instance: &BinaryInstance) -> Result<Self, String> {
        let koblitz = instance
            .koblitz
            .as_ref()
            .ok_or("orbit quotient needs a Koblitz instance (a in {0,1}, b = 1)")?;
        if !koblitz.frobenius_is_endomorphism || koblitz.q != 2 || koblitz.k != 1 {
            return Err("orbit quotient needs the 2-power Frobenius to be an endomorphism".into());
        }
        let r = instance.r;
        let n = instance.n;
        // `r as i64` below and `... + 2 + r` overflow for r >= 2^63, and the
        // batch filter refuses such a modulus anyway; say so here, clearly,
        // instead of failing later or wrapping.
        if r >= 1u64 << 63 {
            return Err(format!(
                "orbit quotient needs a subgroup order r < 2^63 (the filter's modulus bound), got r = {r}"
            ));
        }
        let lambda = (&koblitz.lambda % BigUint::from(r))
            .to_u64()
            .ok_or("lambda does not fit a word")?;
        let t = koblitz.trace.rem_euclid(r as i64) as u64;
        let characteristic = (mul_mod(lambda, lambda, r) + 2 + r - mul_mod(t, lambda, r)) % r;
        if characteristic != 0 {
            return Err(format!(
                "lambda={lambda} does not satisfy lambda^2 - t*lambda + 2 = 0 (mod {r})"
            ));
        }
        let mut pow = Vec::with_capacity(n as usize);
        let mut acc = 1u64;
        for _ in 0..n {
            pow.push(acc);
            acc = mul_mod(acc, lambda, r);
        }
        if acc != 1 {
            return Err(format!("lambda^{n} != 1 (mod {r}); Frobenius has order n"));
        }
        let g = instance.generator;
        if instance.fast.frobenius_k(g, 1) != instance.fast.mul_u64(g, lambda) {
            return Err("pi(G) != [lambda]G on the generator".into());
        }
        Ok(Self { r, n, lambda, pow })
    }

    /// `lambda^j (mod r)`, with the exponent taken modulo `n`.
    pub fn pow(&self, j: u32) -> u64 {
        self.pow[(j % self.n) as usize]
    }
}

/// The signed Frobenius-orbit representative of a subgroup point and the
/// factor that expresses the point's logarithm through it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrbitCanon {
    /// Orbit representative: the member with least abscissa, sign-normalised.
    pub rep: FastPoint,
    /// `k` with `pi^k(Q) = +-rep`: the Frobenius phase.
    pub phase: u32,
    /// Whether `pi^k(Q) = -rep` rather than `+rep`.
    pub negated: bool,
    /// `log Q = factor * log rep (mod r)`, i.e. `eps * lambda^{-k}`.
    pub factor: u64,
}

/// Canonicalise a non-identity subgroup point to its signed Frobenius orbit.
///
/// The representative minimises the abscissa `x^{2^k}` over `0 <= k < n`
/// (first `k` on a tie) and then normalises the sign exactly as the
/// collector's own `canonical` does.
pub fn canonicalise(instance: &BinaryInstance, powers: &LambdaPowers, q: FastPoint) -> OrbitCanon {
    debug_assert!(!q.infinity);
    let field = &instance.fast.field;
    let (mut x, mut best_x, mut best_k) = (q.x, q.x, 0u32);
    for k in 1..instance.n {
        x = field.sqr(x);
        if x < best_x {
            best_x = x;
            best_k = k;
        }
    }
    let moved = instance.fast.frobenius_k(q, best_k);
    let (rep, negated) = canonical(&instance.fast, moved);
    // pi^k(Q) = eps * rep, so log Q = eps * lambda^{-k} * log rep.
    let mut factor = powers.pow(instance.n - best_k);
    if negated {
        factor = (powers.r - factor) % powers.r;
    }
    OrbitCanon {
        rep,
        phase: best_k,
        negated,
        factor,
    }
}

/// The curve-level meaning of a relation row.
///
/// A row over the collector's columns, `sum_c v_c * x_c - B*d = rhs` with
/// `x_c = log_G(points[c])` and `d = log_G(target)`, is the group identity
/// `sum_c [v_c] points[c] = [rhs]G + [B]T`.  This checks that identity with
/// scalar multiplications only, so it does not trust the filter, the
/// replay certificate or the orbit table: a wrong phase coefficient leaves a
/// nonzero multiple of a nonidentity point on one side and the check fails.
pub struct RelationFrame<'a> {
    pub instance: &'a BinaryInstance,
    pub target: FastPoint,
    /// Group element of each column id; the `d_col` entry is unused.
    pub points: &'a [FastPoint],
    pub d_col: usize,
}

impl RelationFrame<'_> {
    /// Whether the row `terms = [(column id, coefficient)]`, `rhs` holds in
    /// the group.  The `d_col` coefficient is `-B`.
    pub fn row_holds<I: IntoIterator<Item = (u64, u64)>>(&self, terms: I, rhs: u64) -> bool {
        let fast = &self.instance.fast;
        let r = self.instance.r;
        let mut lhs = FastPoint::INFINITY;
        let mut d_coefficient = 0u64;
        for (column, coefficient) in terms {
            let coefficient = coefficient % r;
            if column as usize == self.d_col {
                d_coefficient = coefficient;
                continue;
            }
            let Some(&point) = self.points.get(column as usize) else {
                return false;
            };
            lhs = fast.add(lhs, fast.mul_u64(point, coefficient));
        }
        let b = (r - d_coefficient) % r;
        let rhs_point = fast.add(
            fast.mul_u64(self.instance.generator, rhs % r),
            fast.mul_u64(self.target, b),
        );
        (lhs.infinity && rhs_point.infinity) || lhs == rhs_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench_large_prime::{
        import_family, parse_manifest, ImportedFamily,
    };
    use crate::cryptanalysis::large_prime_filter::{
        filter_large_primes, replay_complete_relation, LargePrimeFilterOptions, PartialRelation,
    };

    const MANIFEST: &[u8] = include_bytes!("../../../tests/fixtures/ecbench_manifest_small.json");

    fn fixture() -> ImportedFamily {
        let manifest = parse_manifest(MANIFEST).unwrap();
        import_family(&manifest.families[0]).unwrap().0
    }

    #[test]
    fn lambda_is_certified_and_has_order_dividing_n() {
        let family = fixture();
        let powers = LambdaPowers::new(&family.instance).unwrap();
        assert_eq!(powers.r, 991);
        assert_eq!(powers.pow(0), 1);
        assert_eq!(mul_mod(powers.pow(7), powers.pow(4), powers.r), 1);
        assert_ne!(powers.lambda, 1);
    }

    #[test]
    fn canonical_orbit_is_constant_and_its_factor_inverts() {
        let family = fixture();
        let inst = &family.instance;
        let powers = LambdaPowers::new(inst).unwrap();
        let fast = &inst.fast;
        for q in 1..200u64 {
            let point = fast.mul_u64(inst.generator, q);
            let base = canonicalise(inst, &powers, point);
            assert_eq!(fast.mul_u64(base.rep, base.factor), point);
            for k in 0..inst.n {
                for negate in [false, true] {
                    let mut member = fast.frobenius_k(point, k);
                    if negate {
                        member = fast.neg(member);
                    }
                    let c = canonicalise(inst, &powers, member);
                    assert_eq!(c.rep, base.rep, "q={q} k={k} negate={negate}");
                    assert_eq!(fast.mul_u64(c.rep, c.factor), member);
                }
            }
        }
    }

    /// Two relations, `S1 + Q` and `S2 + pi^3(Q)`, whose large primes are
    /// Frobenius conjugates, built from known logarithms so every column
    /// point and every right-hand side is exact.
    struct Pair {
        family: ImportedFamily,
        powers: LambdaPowers,
        points: Vec<FastPoint>,
        c1: OrbitCanon,
        c2: OrbitCanon,
        s: [u64; 2],
        rhs: [u64; 2],
    }

    const D_COL: u32 = 2;
    const ORBIT_COL: u64 = 3;

    fn pair() -> Pair {
        let family = fixture();
        let inst = &family.instance;
        let powers = LambdaPowers::new(inst).unwrap();
        let fast = &inst.fast;
        let r = inst.r;
        let d = family.expected;
        let mul = |k: u64| fast.mul_u64(inst.generator, k % r);
        let (s1, s2, q, phase) = (17u64, 29u64, 123u64, 3u32);
        let big_q = mul(q);
        let conjugate = fast.frobenius_k(big_q, phase);
        assert_eq!(conjugate, fast.mul_u64(big_q, powers.pow(phase)));
        assert_ne!(conjugate, big_q);
        assert_ne!(conjugate, fast.neg(big_q));
        let c1 = canonicalise(inst, &powers, big_q);
        let c2 = canonicalise(inst, &powers, conjugate);
        assert_eq!(c1.rep, c2.rep, "the two large primes share one orbit");
        let log_conjugate = mul_mod(powers.pow(phase), q, r);
        // A + B*d = (small + large) with B = 5 and 7: rhs = A.
        let (b1, b2) = (5u64, 7u64);
        let a1 = (s1 + q + r * 8 - b1 * d % r) % r;
        let a2 = (s2 + log_conjugate + r * 8 - b2 * d % r) % r;
        assert_eq!((a1 + b1 * d) % r, (s1 + q) % r, "row 1 is a true identity");
        let points = vec![mul(s1), mul(s2), FastPoint::INFINITY, c1.rep];
        Pair {
            family,
            powers,
            points,
            c1,
            c2,
            s: [b1, b2],
            rhs: [a1, a2],
        }
    }

    fn rows(p: &Pair, second_factor: u64) -> Vec<PartialRelation> {
        let r = p.powers.r;
        vec![
            PartialRelation {
                id: 1,
                small: vec![(0, 1), (D_COL, (r - p.s[0]) % r)],
                large: vec![(ORBIT_COL, p.c1.factor)],
                rhs: p.rhs[0],
            },
            PartialRelation {
                id: 2,
                small: vec![(1, 1), (D_COL, (r - p.s[1]) % r)],
                large: vec![(ORBIT_COL, second_factor)],
                rhs: p.rhs[1],
            },
        ]
    }

    fn frame(p: &Pair) -> RelationFrame<'_> {
        RelationFrame {
            instance: &p.family.instance,
            target: p.family.target,
            points: &p.points,
            d_col: D_COL as usize,
        }
    }

    fn options(p: &Pair) -> LargePrimeFilterOptions {
        LargePrimeFilterOptions {
            modulus: p.powers.r,
            ..Default::default()
        }
    }

    #[test]
    fn both_input_rows_hold_as_group_identities() {
        let p = pair();
        for row in rows(&p, p.c2.factor) {
            let terms = row
                .small
                .iter()
                .map(|&(c, v)| (c as u64, v))
                .chain(row.large.iter().copied());
            assert!(frame(&p).row_holds(terms, row.rhs), "row {}", row.id);
        }
    }

    #[test]
    fn frobenius_conjugate_large_primes_cancel_and_replay_on_the_curve() {
        let p = pair();
        let inputs = rows(&p, p.c2.factor);
        let (complete, report) = filter_large_primes(inputs.clone(), &options(&p)).unwrap();
        assert_eq!(complete.len(), 1, "{report:?}");
        assert_eq!(report.cycles_closed, 1);
        let replay = replay_complete_relation(&complete[0], &inputs, p.powers.r).unwrap();
        assert_eq!(replay, complete[0]);
        let terms = complete[0].small.iter().map(|&(c, v)| (c as u64, v));
        assert!(frame(&p).row_holds(terms, complete[0].rhs));
    }

    #[test]
    fn without_the_orbit_quotient_the_same_two_rows_do_not_close() {
        let p = pair();
        // Plain identifiers: the two large primes are distinct vertices of
        // degree one, so both rows peel and nothing is emitted.
        let mut inputs = rows(&p, 1);
        inputs[0].large = vec![(3, 1)];
        inputs[1].large = vec![(4, 1)];
        let (complete, report) = filter_large_primes(inputs, &options(&p)).unwrap();
        assert!(complete.is_empty(), "{report:?}");
        assert_eq!(report.peeled_relations, 2);
    }

    #[test]
    fn a_wrong_phase_or_sign_cancels_in_the_filter_but_fails_the_curve_replay() {
        let p = pair();
        let r = p.powers.r;
        let wrong = [
            ("phase off by one", mul_mod(p.c2.factor, p.powers.lambda, r)),
            ("sign flipped", (r - p.c2.factor) % r),
            ("identifier only", 1),
        ];
        for (label, factor) in wrong {
            assert_ne!(factor, p.c2.factor, "{label}");
            let inputs = rows(&p, factor);
            let (complete, _) = filter_large_primes(inputs.clone(), &options(&p)).unwrap();
            assert_eq!(
                complete.len(),
                1,
                "{label}: the filter cancels by arithmetic"
            );
            // The linear replay agrees with the filter: it cannot see the error.
            assert!(
                replay_complete_relation(&complete[0], &inputs, r).is_some(),
                "{label}"
            );
            let terms = complete[0].small.iter().map(|&(c, v)| (c as u64, v));
            assert!(
                !frame(&p).row_holds(terms, complete[0].rhs),
                "{label}: the curve replay must reject a wrong phase coefficient"
            );
        }
    }
}
