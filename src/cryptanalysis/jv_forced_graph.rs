//! Checked principal witnesses and anchored composite incidence for the
//! existing signed graph. Composite rows retain both original functions.
//! Field/map/hash/container work is not a complete native-cost model.
use super::jv_forced_divisor::{CandidateFunction, Family};
use crate::cryptanalysis::jv_sieve::two_large_prime::{
    deferred_pair_graph, principal_relation, quadratic_pair_graph, quadratic_pair_relation,
};
use crate::cryptanalysis::{
    jv_cover::{BaseEl, Div, Fld, Fq, Hyp, E2},
    residual_walk::{is_prime_u64, pow_mod},
};
pub use deferred_pair_graph::{Event, Graph, Options};
pub use quadratic_pair_graph::{Class, LargeTerm, Witness};
pub use quadratic_pair_relation::{split_quartic, SplitOptions, SplitReport};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn smooth(f: &Fq, h: &[E2]) -> Result<bool, &'static str> {
    let mut a = h.to_vec();
    let mut b = principal_relation::trim(
        h.iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| f.scale(x, i as u64))
            .collect(),
    );
    while !b.is_empty() {
        if b.len() == 1 {
            return Ok(true);
        }
        let inverse = f.inv(b.last().unwrap());
        let monic: Vec<_> = b.iter().map(|x| f.mul(x, &inverse)).collect();
        let remainder = principal_relation::modulo(f, &a, &monic)?;
        a = b;
        b = remainder;
    }
    Ok(a.len() == 1)
}
fn point(f: &Fq, x: u64, y: E2) -> Div<E2> {
    Div {
        u: vec![f.neg(&f.from_fp(x)), E2::ONE],
        v: vec![y],
    }
}
fn large_term(f: &Fq, d: &Div<E2>) -> LargeTerm {
    let a = d.v.first().copied().unwrap_or(E2::ZERO);
    let b = d.v.get(1).copied().unwrap_or(E2::ZERO);
    let na = f.neg(&a);
    let nb = f.neg(&b);
    let key = [d.u[0].0[0], d.u[1].0[0], a.0[0], a.0[1], b.0[0], b.0[1]];
    let neg = [d.u[0].0[0], d.u[1].0[0], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    if neg < key {
        LargeTerm {
            class: neg,
            coefficient: -1,
        }
    } else {
        LargeTerm {
            class: key,
            coefficient: 1,
        }
    }
}
fn divisor(f: &Fq, t: &LargeTerm) -> Result<Div<E2>, &'static str> {
    if ![-1, 1].contains(&t.coefficient) || t.class.iter().any(|&x| x >= f.p) {
        return Err("canonical signed class required");
    }
    let k = t.class;
    let mut v = vec![E2([k[2], k[3]]), E2([k[4], k[5]])];
    if t.coefficient == -1 {
        for x in &mut v {
            *x = f.neg(x);
        }
    }
    Ok(Div {
        u: vec![f.from_fp(k[0]), f.from_fp(k[1]), E2::ONE],
        v: principal_relation::trim(v),
    })
}
/// Admission of the field, smooth curve and actual signed factor-base points.
/// The immutable binding includes their exact order and orientation.
pub struct Domain<'a> {
    f: &'a Fq,
    h: &'a [E2],
    base: &'a [BaseEl],
    binding: [u8; 32],
}
impl<'a> Domain<'a> {
    pub fn new(f: &'a Fq, h: &'a [E2], base: &'a [BaseEl]) -> Result<Self, &'static str> {
        if !(5..=1511).contains(&f.p)
            || !is_prime_u64(f.p)
            || f.w >= f.p
            || pow_mod(f.w, (f.p - 1) / 2, f.p) != f.p - 1
            || h.len() != 8
            || h[7] != E2::ONE
            || h.iter().any(|x| x.0.iter().any(|&x| x >= f.p))
            || base.is_empty()
            || base.len() > f.p as usize
            || base.windows(2).any(|w| w[0].x >= w[1].x)
        {
            return Err("canonical field/curve/ordered base required");
        }
        if !smooth(f, h)? {
            return Err("smooth degree7 curve required");
        }
        let mut hash = Sha256::new();
        hash.update(f.p.to_le_bytes());
        hash.update(f.w.to_le_bytes());
        for e in h {
            for x in e.0 {
                hash.update(x.to_le_bytes());
            }
        }
        hash.update((base.len() as u64).to_le_bytes());
        for b in base {
            if b.x >= f.p
                || b.y.0.iter().any(|&x| x >= f.p)
                || b.d != point(f, b.x, b.y)
                || f.sq(&b.y) != principal_relation::eval(f, h, b.x)
            {
                return Err("original factor-base point required");
            }
            hash.update(b.x.to_le_bytes());
            for x in b.y.0 {
                hash.update(x.to_le_bytes());
            }
        }
        Ok(Self {
            f,
            h,
            base,
            binding: hash.finalize().into(),
        })
    }
    fn jac(&self) -> Hyp<'_, Fq> {
        Hyp {
            f: self.f,
            h: self.h.to_vec(),
            g: 3,
        }
    }
    fn replay(&self, small: &[(usize, i64)], large: &[LargeTerm]) -> Result<(), &'static str> {
        let jac = self.jac();
        let mut sum = jac.identity();
        for &(i, c) in small {
            let b = self.base.get(i).ok_or("small index")?;
            if c.unsigned_abs() > 4096 {
                return Err("bounded integer coefficient required");
            }
            let d = if c < 0 { jac.neg(&b.d) } else { b.d.clone() };
            for _ in 0..c.unsigned_abs() {
                sum = jac.add(&sum, &d);
            }
        }
        for t in large {
            sum = jac.add(&sum, &divisor(self.f, t)?);
        }
        if !jac.is_identity(&sum) {
            return Err("original composite Jacobian replay");
        }
        Ok(())
    }
}
/// Only exact original principal admission constructs this type. It has no
/// unchecked deserializer. Serialization preserves a complete replay witness.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PrincipalProof {
    binding: [u8; 32],
    a: Vec<[u64; 2]>,
    c: Vec<[u64; 2]>,
    norm: Vec<[u64; 2]>,
    forced: LargeTerm,
    roots: Vec<u64>,
    factors: Vec<Vec<u64>>,
    small: Vec<(usize, i8)>,
    large: Vec<LargeTerm>,
}
impl PrincipalProof {
    pub fn check(
        domain: &Domain<'_>,
        family: &Family,
        function: &CandidateFunction,
        roots: &[u64],
        factors: &[Vec<u64>],
    ) -> Result<Self, &'static str> {
        if family.curve_polynomial() != domain.h || family.modulus() != domain.f.p {
            return Err("principal curve binding");
        }
        let support = if roots.len() == 3 {
            family.materialize_three(domain.f, function, roots, factors)?
        } else {
            if !factors.is_empty() {
                return Err("unexpected explicit tail factors");
            }
            family.materialize(domain.f, function, roots)?
        };
        let mut small = vec![];
        for &(x, y) in &support.small {
            let i = domain
                .base
                .binary_search_by_key(&x, |b| b.x)
                .map_err(|_| "small point outside admitted base")?;
            let b = &domain.base[i];
            let sign = if y == b.y {
                1
            } else if y == domain.f.neg(&b.y) {
                -1
            } else {
                return Err("small ordinate/base orientation");
            };
            small.push((i, sign));
        }
        let mut large: Vec<_> = support
            .quadratic
            .iter()
            .map(|d| large_term(domain.f, d))
            .collect();
        let forced = large[0];
        large.sort_by_key(|t| t.class);
        if large.windows(2).any(|w| w[0].class == w[1].class) {
            return Err("distinct large classes required");
        }
        domain.replay(
            &small
                .iter()
                .map(|&(i, c)| (i, c as i64))
                .collect::<Vec<_>>(),
            &large,
        )?;
        let encode = |v: &[E2]| v.iter().map(|e| e.0).collect();
        Ok(Self {
            binding: domain.binding,
            a: encode(&function.a),
            c: encode(&function.c),
            norm: encode(&function.norm),
            forced,
            roots: roots.to_vec(),
            factors: factors.to_vec(),
            small,
            large,
        })
    }
    pub fn recheck(&self, domain: &Domain<'_>) -> Result<(), &'static str> {
        if self.binding != domain.binding {
            return Err("principal domain/base binding");
        }
        let q = divisor(domain.f, &self.forced)?;
        let family = Family::new(domain.f, domain.h, &q.u, &q.v)?;
        let a: Vec<_> = self.a.iter().copied().map(E2).collect();
        let c: Vec<_> = self.c.iter().copied().map(E2).collect();
        let z = family.recover_parameters(domain.f, &a, &c)?;
        let function = family.instantiate(domain.f, &z)?;
        let checked = Self::check(domain, &family, &function, &self.roots, &self.factors)?;
        if checked != *self {
            return Err("original principal proof mismatch");
        }
        Ok(())
    }
    pub fn large_terms(&self) -> &[LargeTerm] {
        &self.large
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct CompositeEdge {
    binding: [u8; 32],
    small: Vec<(usize, i64)>,
    large: Vec<LargeTerm>,
    candidate: PrincipalProof,
    anchor: PrincipalProof,
    anchor_coefficient: i8,
}
fn add_small(
    out: &mut BTreeMap<usize, i64>,
    terms: &[(usize, i8)],
    scale: i64,
) -> Result<(), &'static str> {
    for &(i, c) in terms {
        let entry = out.entry(i).or_default();
        *entry = entry
            .checked_add(scale * c as i64)
            .ok_or("integer coefficient overflow")?;
    }
    Ok(())
}
impl CompositeEdge {
    pub fn cancel(
        domain: &Domain<'_>,
        candidate: PrincipalProof,
        anchor: PrincipalProof,
    ) -> Result<Self, &'static str> {
        candidate.recheck(domain)?;
        anchor.recheck(domain)?;
        if anchor.large.len() != 1 {
            return Err("one-LP anchor required");
        }
        let q = anchor.large[0];
        let coefficient = candidate
            .large
            .iter()
            .find(|t| t.class == q.class)
            .ok_or("shared anchor class absent")?
            .coefficient;
        let anchor_coefficient = -coefficient * q.coefficient;
        let mut small = BTreeMap::new();
        add_small(&mut small, &candidate.small, 1)?;
        add_small(&mut small, &anchor.small, anchor_coefficient as i64)?;
        let small: Vec<_> = small.into_iter().filter(|(_, c)| *c != 0).collect();
        let large: Vec<_> = candidate
            .large
            .iter()
            .filter(|t| t.class != q.class)
            .copied()
            .collect();
        if large.len() > 2 {
            return Err("at most two remaining graph endpoints required");
        }
        domain.replay(&small, &large)?;
        Ok(Self {
            binding: domain.binding,
            small,
            large,
            candidate,
            anchor,
            anchor_coefficient,
        })
    }
    pub fn is_trivial(&self) -> bool {
        self.small.is_empty() && self.large.is_empty()
    }
    pub fn large_terms(&self) -> &[LargeTerm] {
        &self.large
    }
    pub fn replay(&self, domain: &Domain<'_>) -> Result<(), &'static str> {
        if self.binding != domain.binding {
            return Err("composite domain/base binding");
        }
        let expected = Self::cancel(domain, self.candidate.clone(), self.anchor.clone())?;
        if expected.small != self.small
            || expected.large != self.large
            || expected.anchor_coefficient != self.anchor_coefficient
        {
            return Err("composite integer incidence mismatch");
        }
        Ok(())
    }
    /// Source index belongs to the composite-edge stream, not either original
    /// principal-function stream. Trivial cancellation does not enter graph.
    pub fn insert(
        &self,
        domain: &Domain<'_>,
        graph: &mut Graph,
        source: usize,
    ) -> Result<Vec<Event>, &'static str> {
        self.replay(domain)?;
        if self.is_trivial() {
            return Ok(vec![]);
        }
        graph.accept(source, &self.large)
    }
}
#[derive(Debug, Serialize)]
pub struct Projection {
    pub integer_small: Vec<(usize, i64)>,
    pub modulo_small: Vec<(usize, u64)>,
    pub witness: Witness,
}
/// Expand graph coefficients over integers and retain source IDs into the
/// complete composite proofs. Never treat cancellation modulo2 or L as zero.
pub fn project_integer(
    domain: &Domain<'_>,
    edges: &[CompositeEdge],
    witness: &Witness,
) -> Result<Vec<(usize, i64)>, &'static str> {
    if witness.len() > 4096 {
        return Err("bounded projection witness required");
    }
    let mut small = BTreeMap::<usize, i64>::new();
    let mut large = BTreeMap::<Class, i64>::new();
    for &(id, c) in witness {
        if ![-2, -1, 1, 2].contains(&c) {
            return Err("signed graph coefficient required");
        }
        let edge = edges.get(id).ok_or("composite source index")?;
        edge.replay(domain)?;
        for &(i, n) in &edge.small {
            let entry = small.entry(i).or_default();
            *entry = entry
                .checked_add(n * c as i64)
                .ok_or("small projection overflow")?;
        }
        for t in &edge.large {
            let entry = large.entry(t.class).or_default();
            *entry = entry
                .checked_add(c as i64 * t.coefficient as i64)
                .ok_or("large projection overflow")?;
        }
    }
    if large.values().any(|&x| x != 0) {
        return Err("nonzero integer large incidence");
    }
    let integer_small: Vec<_> = small.into_iter().filter(|(_, c)| *c != 0).collect();
    domain.replay(&integer_small, &[])?;
    Ok(integer_small)
}
pub fn project(
    domain: &Domain<'_>,
    edges: &[CompositeEdge],
    witness: &Witness,
    l: u64,
) -> Result<Projection, &'static str> {
    if !(3..1u64 << 63).contains(&l) {
        return Err("bounded projection modulus required");
    }
    let integer_small = project_integer(domain, edges, witness)?;
    let modulo_small = integer_small
        .iter()
        .filter_map(|&(i, c)| {
            let n = (c as i128).rem_euclid(l as i128) as u64;
            (n != 0).then_some((i, n))
        })
        .collect();
    Ok(Projection {
        integer_small,
        modulo_small,
        witness: witness.clone(),
    })
}

#[cfg(test)]
mod controls {
    use super::*;
    use crate::cryptanalysis::jv_sieve::FpRing;
    use rand::{rngs::StdRng, SeedableRng};
    fn fixture() -> (Fq, Vec<E2>, Vec<BaseEl>, Family, CandidateFunction) {
        let f = Fq::new(11);
        let u = vec![E2::ONE, E2::ZERO, E2::ONE];
        let mut s = vec![f.neg(&E2::ONE)];
        for x in 1..8 {
            s = principal_relation::mul(&f, &s, &[f.neg(&f.from_fp(x)), E2::ONE]);
        }
        let norm = principal_relation::mul(&f, &u, &s);
        let a0 = (0..11)
            .flat_map(|a| (0..11).map(move |b| E2([a, b])))
            .find(|a| f.sq(a) == norm[0])
            .unwrap();
        assert_eq!(a0.0[0], 0);
        let a1 = f.mul(&norm[1], &f.inv(&f.scale(&a0, 2)));
        let (a, h) = (1..11)
            .find_map(|c| {
                let a = vec![a0, a1, E2([0, c])];
                if (1..8).any(|x| principal_relation::eval(&f, &a, x) == E2::ZERO) {
                    return None;
                }
                let difference =
                    principal_relation::sub(&f, &principal_relation::mul(&f, &a, &a), &norm);
                assert_eq!(&difference[..2], &[E2::ZERO, E2::ZERO]);
                let h = difference[2..].to_vec();
                smooth(&f, &h).unwrap().then_some((a, h))
            })
            .unwrap();
        assert!(h.iter().all(|e| e.in_fp()));
        let v = principal_relation::modulo(
            &f,
            &principal_relation::mul(&f, &a, &[E2::ZERO, E2::ONE]),
            &u,
        )
        .unwrap();
        let family = Family::new(&f, &h, &u, &v).unwrap();
        let z = family
            .recover_parameters(&f, &a, &[E2::ZERO, E2::ONE])
            .unwrap();
        let value = family.instantiate(&f, &z).unwrap();
        let base = (0..11)
            .filter_map(|x| {
                let hx = principal_relation::eval(&f, &h, x);
                let y = (0..11)
                    .flat_map(|a| (0..11).map(move |b| E2([a, b])))
                    .find(|y| *y != E2::ZERO && f.sq(y) == hx)?;
                Some(BaseEl {
                    x,
                    y,
                    d: point(&f, x, y),
                })
            })
            .collect();
        (f, h, base, family, value)
    }
    fn remove_root(poly: &[u64], r: u64, p: u64) -> Vec<u64> {
        let n = poly.len() - 1;
        let mut q = vec![0; n];
        q[n - 1] = poly[n];
        for i in (1..n).rev() {
            q[i - 1] = (poly[i] + r * q[i]) % p;
        }
        assert_eq!((poly[0] + r * q[0]) % p, 0);
        q
    }
    fn candidate(domain: &Domain<'_>, family: &Family) -> PrincipalProof {
        let f = domain.f;
        let ring = FpRing::new(11);
        let mut rng = StdRng::seed_from_u64(947);
        let (mut attempts, mut quartics, mut split_refusals, mut support_refusals) = (0, 0, 0, 0);
        for c0 in 0..11 {
            for t0 in 0..11 {
                for t1 in 0..11 {
                    for t2 in 0..11 {
                        attempts += 1;
                        let value = family
                            .instantiate(f, &[c0, 0, 0, t0, 0, t1, 0, t2])
                            .unwrap();
                        let polynomial: Vec<_> = value.quotient.iter().map(|e| e.0[0]).collect();
                        let roots = ring.roots_by_table(&polynomial);
                        if roots.len() != 3 {
                            continue;
                        }
                        let mut tail = polynomial;
                        for &r in &roots {
                            tail = remove_root(&tail, r, 11);
                        }
                        quartics += 1;
                        let report =
                            split_quartic(&ring, &tail, SplitOptions { max_attempts: 8 }, &mut rng)
                                .unwrap();
                        let Some(factors) = report.factors else {
                            split_refusals += 1;
                            continue;
                        };
                        if let Ok(proof) =
                            PrincipalProof::check(domain, family, &value, &roots, &factors)
                        {
                            println!("synthetic lex candidate: points={attempts}, quartics={quartics}, split_refusals={split_refusals}, support_refusals={support_refusals}");
                            return proof;
                        }
                        support_refusals += 1;
                    }
                }
            }
        }
        panic!("no synthetic three-LP candidate within fixed 11^4 cap");
    }
    #[test]
    fn exact_composite_admits_two_endpoints_but_no_ungrounded_full_row() {
        let (f, h, base, family, value) = fixture();
        let domain = Domain::new(&f, &h, &base).unwrap();
        let anchor =
            PrincipalProof::check(&domain, &family, &value, &(1..8).collect::<Vec<_>>(), &[])
                .unwrap();
        let candidate = candidate(&domain, &family);
        assert_eq!(candidate.large.len(), 3);
        let edge = CompositeEdge::cancel(&domain, candidate, anchor.clone()).unwrap();
        assert_eq!(edge.large.len(), 2);
        assert!(!edge.is_trivial());
        edge.replay(&domain).unwrap();
        let mut graph = Graph::new(Options::default()).unwrap();
        let events = edge.insert(&domain, &mut graph, 0).unwrap();
        assert!(matches!(events.as_slice(), [Event::Deferred { source: 0 }]));
        assert_eq!(graph.stats().live_deferred, 1);
        assert!(project(&domain, std::slice::from_ref(&edge), &vec![(0, 1)], 3).is_err());
        let projection = project(&domain, &[edge], &vec![(0, 1), (0, -1)], 3).unwrap();
        assert!(projection.integer_small.is_empty());
        let trivial = CompositeEdge::cancel(&domain, anchor.clone(), anchor).unwrap();
        assert!(trivial.is_trivial());
        assert!(trivial.insert(&domain, &mut graph, 1).unwrap().is_empty());
        assert_eq!(graph.stats().sources, 1);
    }
    #[test]
    fn original_norm_incidence_sign_and_domain_tampering_are_refused() {
        let (f, h, base, family, value) = fixture();
        let domain = Domain::new(&f, &h, &base).unwrap();
        let anchor =
            PrincipalProof::check(&domain, &family, &value, &(1..8).collect::<Vec<_>>(), &[])
                .unwrap();
        let candidate = candidate(&domain, &family);
        let mut bad = candidate.clone();
        bad.norm[0][0] = (bad.norm[0][0] + 1) % 11;
        assert!(bad.recheck(&domain).is_err());
        let mut bad = candidate.clone();
        bad.large[0].coefficient *= -1;
        assert!(bad.recheck(&domain).is_err());
        let mut edge = CompositeEdge::cancel(&domain, candidate, anchor.clone()).unwrap();
        edge.anchor_coefficient *= -1;
        assert!(edge.replay(&domain).is_err());
        let mut edge = CompositeEdge::cancel(&domain, anchor.clone(), anchor.clone()).unwrap();
        edge.small.push((0, 1));
        assert!(edge.replay(&domain).is_err());
        let mut other = base.clone();
        other[0].y = f.neg(&other[0].y);
        other[0].d = point(&f, other[0].x, other[0].y);
        let changed = Domain::new(&f, &h, &other).unwrap();
        assert!(anchor.recheck(&changed).is_err());
        let singular = vec![
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2::ONE,
        ];
        assert!(Domain::new(&f, &singular, &base).is_err());
    }
}
