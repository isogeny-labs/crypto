//! Opt-in degree-nine principal relations with an Fp-irreducible quadratic
//! remainder. The remainder is a reduced divisor class over Fp², not an
//! asserted prime divisor. Original Jacobian identities remain mandatory.
use super::{filter_and_replay, FilteredTwoLargePrimeRelations, TwoLargePrimeSieve};
use crate::cryptanalysis::gaudry_cubic::{mm, sm};
use crate::cryptanalysis::jv_cover::{pmod, pmul, pscale, psub, ptrim, pxgcd, Ctx, Div, Fld, E2};
use crate::cryptanalysis::jv_sieve::{fdeg, line_poly, FpRing, Line};
use crate::cryptanalysis::large_prime_filter::{LargePrimeFilterOptions, PartialRelation};
use std::collections::HashMap;

/// A checked original equation sum(small_i) + remainder = 0 in Jac(H).
/// Private fields prevent arbitrary graph rows from acquiring this standing.
#[derive(Clone, Debug)]
pub struct QuadraticRemainderRelation {
    small: Vec<(usize, i8)>,
    remainder: Div<E2>,
    p: u64,
    w: u64,
    h: Vec<E2>,
}

impl QuadraticRemainderRelation {
    pub fn small_terms(&self) -> &[(usize, i8)] {
        &self.small
    }

    pub fn remainder(&self) -> &Div<E2> {
        &self.remainder
    }
}

pub struct FilteredQuadraticRemainders {
    pub projection: FilteredTwoLargePrimeRelations,
    /// Stable first-occurrence labels for canonical D/-D representatives.
    pub classes: Vec<[u64; 8]>,
}

/// Key a reduced degree-two divisor modulo sign, retaining the sign itself.
/// Padding makes the descriptor independent of trailing zero coefficients.
fn canonical_class(ctx: &Ctx, d: &Div<E2>) -> ([u64; 8], i8) {
    let v: [E2; 2] = std::array::from_fn(|i| d.v.get(i).copied().unwrap_or(E2::ZERO));
    let nv = v.map(|x| ctx.f.f.neg(&x));
    let (v, sign) = if v <= nv { (v, 1) } else { (nv, -1) };
    (
        [
            d.u[0].0[0],
            d.u[0].0[1],
            d.u[1].0[0],
            d.u[1].0[1],
            v[0].0[0],
            v[0].0[1],
            v[1].0[0],
            v[1].0[1],
        ],
        sign,
    )
}

impl TwoLargePrimeSieve {
    fn remainder_source_matches(&self, ctx: &Ctx) -> bool {
        let shape_matches = self.setup.m == 9
            && ctx.f.f.p == self.setup.p
            && ctx.f.f.w == self.setup.w
            && ctx.cov.hx == self.setup.h
            && ctx.cov.hx.len() == 8
            && ctx.cov.hx.last() == Some(&E2::ONE)
            && ctx
                .cov
                .hx
                .iter()
                .all(|c| c.0.iter().all(|&v| v < self.setup.p));
        if !shape_matches {
            return false;
        }
        let fq = &ctx.f.f;
        let derivative: Vec<_> = ctx
            .cov
            .hx
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| fq.scale(c, i as u64))
            .collect();
        pxgcd(fq, &ctx.cov.hx, &derivative).0 == vec![E2::ONE]
    }

    fn irreducible_quadratic(&self, ring: &FpRing, u: &[u64]) -> bool {
        if u.len() != 3 || u[2] != 1 {
            return false;
        }
        ring.count(3);
        let disc = sm(mm(u[1], u[1], ring.p), mm(4, u[0], ring.p), ring.p);
        disc != 0 && self.setup.sqrt[disc as usize] == u64::MAX
    }

    fn check_remainder_relation(&self, ctx: &Ctx, r: &QuadraticRemainderRelation) -> bool {
        if !self.remainder_source_matches(ctx)
            || r.p != self.setup.p
            || r.w != self.setup.w
            || r.h != self.setup.h
            || r.small.len() != 7
            || r.small.windows(2).any(|a| a[0].0 >= a[1].0)
            || r.small
                .iter()
                .any(|&(i, sign)| i >= self.small || ![-1, 1].contains(&sign))
            || r.remainder.u.len() != 3
            || r.remainder
                .u
                .iter()
                .any(|c| !c.in_fp() || c.0[0] >= self.setup.p)
            || r.remainder
                .v
                .iter()
                .any(|c| c.0.iter().any(|&v| v >= self.setup.p))
        {
            return false;
        }
        let ring = FpRing::new(self.setup.p);
        let u: Vec<_> = r.remainder.u.iter().map(|c| c.0[0]).collect();
        if !self.irreducible_quadratic(&ring, &u) {
            return false;
        }
        let jac = ctx.jac();
        if !jac.is_valid(&r.remainder) {
            return false;
        }
        let mut sum = r.remainder.clone();
        for &(i, sign) in &r.small {
            let point = &self.base[i].d;
            sum = if sign == 1 {
                jac.add(&sum, point)
            } else {
                jac.sub(&sum, point)
            };
        }
        jac.is_identity(&sum)
    }

    /// Keep exactly seven distinct small roots and an irreducible quadratic
    /// over Fp. Require the actual norm identity and replay in the Jacobian.
    /// This extractor does not change the existing split-support collector.
    pub fn verified_quadratic_remainder(
        &self,
        ctx: &Ctx,
        ring: &FpRing,
        line: &Line,
        s: u64,
    ) -> Option<QuadraticRemainderRelation> {
        if !self.remainder_source_matches(ctx)
            || ring.p != self.setup.p
            || s == 0
            || s >= ring.p
            || line.a0.len() > 5
            || line.a1.len() > 5
            || line.b.len() != 2
            || line.b[1] == E2::ZERO
            || [&line.a0, &line.a1, &line.p1, &line.p2, &line.p3]
                .iter()
                .any(|v| v.iter().any(|&c| c >= ring.p))
            || line.b.iter().any(|c| c.0.iter().any(|&v| v >= ring.p))
        {
            return None;
        }
        let f = line_poly(ring, line, s);
        if fdeg(&f) != 9 {
            return None;
        }
        let fq = &ctx.f.f;
        let sq = if self.setup.sqrt[s as usize] != u64::MAX {
            E2([self.setup.sqrt[s as usize], 0])
        } else {
            ring.count(1);
            E2([
                0,
                *self
                    .setup
                    .sqrt
                    .get(mm(s, self.setup.winv, ring.p) as usize)?,
            ])
        };
        if sq.0.contains(&u64::MAX) || fq.mul(&sq, &sq) != fq.from_fp(s) {
            return None;
        }
        let mut a = Vec::with_capacity(line.a0.len().max(line.a1.len()));
        for i in 0..line.a0.len().max(line.a1.len()) {
            ring.count(1);
            a.push(E2([
                line.a0.get(i).copied().unwrap_or(0),
                mm(s, line.a1.get(i).copied().unwrap_or(0), ring.p),
            ]));
        }
        ptrim(fq, &mut a);
        let c = pscale(fq, &line.b, &sq);
        let norm = psub(
            fq,
            &pmul(fq, &a, &a),
            &pmul(fq, &pmul(fq, &c, &c), &ctx.cov.hx),
        );
        if norm != f.iter().map(|&x| fq.from_fp(x)).collect::<Vec<_>>() {
            return None;
        }
        // A+C*y has pole order nine: C has degree one, H monic degree
        // seven, and deg A<=4. Squarefree norm ensures simple support.
        ring.count(9);
        let derivative: Vec<_> = f
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| mm(i as u64 % ring.p, x, ring.p))
            .collect();
        let mut derivative = derivative;
        while derivative.last() == Some(&0) {
            derivative.pop();
        }
        if ring.gcd(&f, &derivative).len() != 1 {
            return None;
        }
        let mut u = f.clone();
        let mut small = Vec::new();
        for (i, b) in self.base[..self.small].iter().enumerate() {
            if ring.eval(&f, b.x) != 0 {
                continue;
            }
            let (q, r) = ring.divrem(&u, &[sm(0, b.x, ring.p), 1]);
            if !r.is_empty() {
                return None;
            }
            u = q;
            let x = fq.from_fp(b.x);
            let den = crate::cryptanalysis::jv_cover::peval(fq, &c, &x);
            if den == E2::ZERO {
                return None;
            }
            let ordinate = fq.neg(&fq.mul(
                &crate::cryptanalysis::jv_cover::peval(fq, &a, &x),
                &fq.inv(&den),
            ));
            let sign = if ordinate == b.y {
                1
            } else if ordinate == fq.neg(&b.y) {
                -1
            } else {
                return None;
            };
            small.push((i, sign));
        }
        if small.len() != 7 {
            return None;
        }
        let u = ring.monic(&u);
        if !self.irreducible_quadratic(ring, &u) {
            return None;
        }
        let u: Vec<_> = u.iter().map(|&x| fq.from_fp(x)).collect();
        let (gcd, inverse, _) = pxgcd(fq, &c, &u);
        if gcd != vec![E2::ONE] {
            return None;
        }
        let v = pmod(
            fq,
            &pscale(fq, &pmul(fq, &a, &inverse), &fq.neg(&E2::ONE)),
            &u,
        );
        let relation = QuadraticRemainderRelation {
            small,
            remainder: Div { u, v },
            p: ring.p,
            w: fq.w,
            h: ctx.cov.hx.clone(),
        };
        self.check_remainder_relation(ctx, &relation)
            .then_some(relation)
    }

    /// Eliminate repeated canonical remainder classes, with bounded vertex
    /// storage, original group checks and input-combination certificate replay.
    /// Rows describe cofactor-projected small logs modulo options.modulus,
    /// as in filter_checked. This method supplies no target descent.
    pub fn filter_quadratic_remainders_checked(
        &self,
        ctx: &Ctx,
        relations: &[QuadraticRemainderRelation],
        options: &LargePrimeFilterOptions,
        max_vertices: usize,
    ) -> Result<FilteredQuadraticRemainders, &'static str> {
        if !self.remainder_source_matches(ctx)
            || relations.len() > 4096
            || !(1..=4096).contains(&max_vertices)
            || !(1..=4096).contains(&options.max_sources)
            || !(1..=4096).contains(&options.max_small_weight)
            || !(3..1u64 << 63).contains(&options.modulus)
            || options.modulus.is_multiple_of(2)
        {
            return Err("invalid remainder graph domain or resource bounds");
        }
        let mut vertices = HashMap::new();
        let mut classes = Vec::new();
        let mut inputs = Vec::with_capacity(relations.len());
        let n = options.modulus;
        for (id, r) in relations.iter().enumerate() {
            if !self.check_remainder_relation(ctx, r) {
                return Err("remainder source or Jacobian mismatch");
            }
            let (key, sign) = canonical_class(ctx, &r.remainder);
            let vertex = if let Some(&v) = vertices.get(&key) {
                v
            } else {
                if classes.len() >= max_vertices {
                    return Err("remainder vertex cap");
                }
                let v = classes.len() as u64;
                vertices.insert(key, v);
                classes.push(key);
                v
            };
            inputs.push(PartialRelation {
                id: id as u64,
                small: r
                    .small
                    .iter()
                    .map(|&(i, s)| (i as u32, if s == 1 { 1 } else { n - 1 }))
                    .collect(),
                large: vec![(vertex, if sign == 1 { 1 } else { n - 1 })],
                rhs: 0,
            });
        }
        Ok(FilteredQuadraticRemainders {
            projection: filter_and_replay(&inputs, options)?,
            classes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::jv_cover::{factor_base, generate_spec, pdivrem};
    use rand::{rngs::StdRng, SeedableRng};

    // Construct F=-prod_(r=1..7)(x-r)*U, C=x and A of degree four.
    // Choose A's first two coefficients so A²-F is divisible by x²;
    // H=(A²-F)/x² is then monic of degree seven. This supplies a known
    // principal divisor independently of the extractor and no planted log.
    fn fixture(quadratic: [u64; 3]) -> (Ctx, TwoLargePrimeSieve, Line) {
        let p = 19;
        let ring = FpRing::new(p);
        let mut ctx = Ctx::new(&generate_spec(p, 2));
        let fq = &ctx.f.f;
        let mut f = quadratic.to_vec();
        for root in 1..=7 {
            f = ring.mul(&f, &[p - root, 1]);
        }
        let f = ring.scale(&f, p - 1);
        let mut rng = StdRng::seed_from_u64(101);
        let a0 = fq.sqrt(&fq.from_fp(f[0]), &mut rng).unwrap();
        let a1 = fq.mul(&fq.from_fp(f[1]), &fq.inv(&fq.scale(&a0, 2)));
        let fe: Vec<_> = f.iter().map(|&x| fq.from_fp(x)).collect();
        let (a, h) = (0..p)
            .find_map(|middle| {
                let a = vec![a0, a1, fq.from_fp(middle), E2::ZERO, E2([0, 1])];
                let (h, r) = pdivrem(
                    fq,
                    &psub(fq, &pmul(fq, &a, &a), &fe),
                    &[E2::ZERO, E2::ZERO, E2::ONE],
                );
                assert!(r.is_empty());
                let dh: Vec<_> = h
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(i, c)| fq.scale(c, i as u64))
                    .collect();
                (pxgcd(fq, &h, &dh).0 == vec![E2::ONE]).then_some((a, h))
            })
            .expect("smooth constructed genus-three model");
        assert_eq!(h.len(), 8);
        assert_eq!(h.last(), Some(&E2::ONE));
        let a0: Vec<_> = a.iter().map(|c| c.0[0]).collect();
        let a1: Vec<_> = a.iter().map(|c| c.0[1]).collect();
        let p3 = pmul(fq, &[E2::ZERO, E2::ZERO, E2::ONE], &h);
        let line = Line {
            p1: ring.sqr(&a0),
            p2: ring.scale(&ring.sqr(&a1), fq.w),
            p3: p3.iter().map(|c| c.0[0]).collect(),
            p4: vec![0, 0, 1],
            a0,
            a1,
            b: vec![E2::ZERO, E2::ONE],
        };
        ctx.cov.hx = h;
        let base = factor_base(&ctx, &mut rng);
        assert!((1..=7).all(|x| base.iter().any(|b| b.x == x)));
        let sieve = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9).unwrap();
        assert_eq!(line_poly(&ring, &line, 1), f);
        (ctx, sieve, line)
    }

    fn nonsplit_fixture() -> (Ctx, TwoLargePrimeSieve, Line) {
        // 2 is the first nonsquare modulo19; x²-2 is irreducible in F19.
        fixture([17, 0, 1])
    }

    #[test]
    fn actual_cover_supplies_a_checked_nonsplit_remainder() {
        use crate::cryptanalysis::jv_sieve::{
            b_from_index, b_space, coprime_scramble, lines_for_b, Counters,
        };
        let spec = generate_spec(251, 2);
        let ctx = Ctx::new(&spec);
        let mut rng = StdRng::seed_from_u64(109);
        let base = factor_base(&ctx, &mut rng);
        let sieve = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9).unwrap();
        let ring = FpRing::new(251);
        let scramble = coprime_scramble(b_space(sieve.setup()), &mut rng);
        let mut counters = Counters::new(251);
        for k in 0..b_space(sieve.setup()) {
            let b = b_from_index(sieve.setup(), k, scramble);
            for line in lines_for_b(&ring, &ctx.f.f, sieve.setup(), &b, &mut rng) {
                for s in sieve.sieve_line(&ring, &line, &mut counters, None).0 {
                    if let Some(r) = sieve.verified_quadratic_remainder(&ctx, &ring, &line, s) {
                        assert!(sieve
                            .verified_relation(&ctx, &ring, &line, s, 0, spec.l)
                            .is_none());
                        assert!(sieve.check_remainder_relation(&ctx, &r));
                        println!(
                            "actual-cover control: p251 seed2 B-index={k} s={s} U={:?}",
                            r.remainder.u
                        );
                        return;
                    }
                }
            }
        }
        panic!("no actual-cover remainder found in the frozen full B-space control");
    }

    #[test]
    fn retains_seven_points_and_nonsplit_quadratic_with_jacobian_replay() {
        let (ctx, sieve, line) = nonsplit_fixture();
        let r = sieve
            .verified_quadratic_remainder(&ctx, &FpRing::new(19), &line, 1)
            .unwrap();
        assert_eq!(r.small.len(), 7);
        assert_eq!(r.remainder.u, vec![E2([17, 0]), E2::ZERO, E2::ONE]);
        assert!(ctx.jac().is_valid(&r.remainder));
        // Independent direction: add the seven point classes from zero,
        // then compare directly with the negative of the returned divisor.
        let jac = ctx.jac();
        let mut small = jac.identity();
        for &(i, sign) in r.small_terms() {
            let point = if sign == 1 {
                sieve.base[i].d.clone()
            } else {
                jac.neg(&sieve.base[i].d)
            };
            small = jac.add(&small, &point);
        }
        assert_eq!(small, jac.neg(r.remainder()));
        // Split U over Fp² and reconstruct both non-Fp points separately.
        let fq = &ctx.f.f;
        let mut rng = StdRng::seed_from_u64(107);
        let x = fq.sqrt(&fq.from_fp(2), &mut rng).unwrap();
        assert!(!x.in_fp());
        let points: Vec<_> = [x, fq.neg(&x)]
            .iter()
            .map(|x| {
                let y = crate::cryptanalysis::jv_cover::peval(fq, &r.remainder.v, x);
                assert_eq!(
                    fq.mul(&y, &y),
                    crate::cryptanalysis::jv_cover::peval(fq, &ctx.cov.hx, x)
                );
                Div {
                    u: vec![fq.neg(x), E2::ONE],
                    v: vec![y],
                }
            })
            .collect();
        assert_eq!(jac.add(&points[0], &points[1]), r.remainder);
        let (gcd, _, _) = pxgcd(&ctx.f.f, &r.remainder.u, &ctx.cov.hx);
        assert_eq!(gcd, vec![E2::ONE]);
    }

    #[test]
    fn opposite_functions_keep_one_class_and_reverse_every_sign() {
        let (ctx, sieve, line) = nonsplit_fixture();
        let ring = FpRing::new(19);
        let r = sieve
            .verified_quadratic_remainder(&ctx, &ring, &line, 1)
            .unwrap();
        let mut negative = line.clone();
        negative.a0 = ring.scale(&line.a0, 18);
        negative.a1 = ring.scale(&line.a1, 18);
        let nr = sieve
            .verified_quadratic_remainder(&ctx, &ring, &negative, 1)
            .unwrap();
        assert_eq!(nr.remainder, ctx.jac().neg(&r.remainder));
        assert_eq!(
            nr.small,
            r.small.iter().map(|&(i, s)| (i, -s)).collect::<Vec<_>>()
        );
        let (key, sign) = canonical_class(&ctx, &r.remainder);
        let (nkey, nsign) = canonical_class(&ctx, &nr.remainder);
        assert_eq!(key, nkey);
        assert_eq!(sign, -nsign);
        let options = LargePrimeFilterOptions {
            modulus: 101,
            ..Default::default()
        };
        let projected = sieve
            .filter_quadratic_remainders_checked(&ctx, &[r, nr], &options, 1)
            .unwrap();
        assert_eq!(projected.classes.len(), 1);
        // These are opposite copies of one equation: they supply no new rank.
        assert!(!projected.projection.rows.is_empty());
        assert!(projected
            .projection
            .rows
            .iter()
            .all(|row| row.cols.is_empty() && row.rhs == 0));
        assert_eq!(
            projected.projection.replayed_rows,
            projected.projection.certificates.len()
        );
    }

    #[test]
    fn refuses_split_repeated_forged_wrong_domain_and_excess_resources() {
        let (mut ctx, sieve, line) = nonsplit_fixture();
        let ring = FpRing::new(19);
        let r = sieve
            .verified_quadratic_remainder(&ctx, &ring, &line, 1)
            .unwrap();
        for s in [0, 19] {
            assert!(sieve
                .verified_quadratic_remainder(&ctx, &ring, &line, s)
                .is_none());
        }
        assert!(sieve
            .verified_quadratic_remainder(&ctx, &FpRing::new(23), &line, 1)
            .is_none());
        let degree_eight = TwoLargePrimeSieve::new(&ctx, &sieve.base, sieve.small, 8).unwrap();
        assert!(degree_eight
            .verified_quadratic_remainder(&ctx, &ring, &line, 1)
            .is_none());
        let mut damaged = line.clone();
        damaged.a1[0] = (damaged.a1[0] + 1) % 19;
        assert!(sieve
            .verified_quadratic_remainder(&ctx, &ring, &damaged, 1)
            .is_none());
        damaged = line.clone();
        damaged.b[0] = E2::ONE;
        assert!(sieve
            .verified_quadratic_remainder(&ctx, &ring, &damaged, 1)
            .is_none());
        let options = LargePrimeFilterOptions {
            modulus: 101,
            ..Default::default()
        };
        let mut forged = r.clone();
        forged.small[0].1 *= -1;
        assert!(sieve
            .filter_quadratic_remainders_checked(&ctx, &[forged], &options, 1)
            .is_err());
        let mut forged = r.clone();
        forged.remainder.v[0] = E2::ZERO;
        assert!(sieve
            .filter_quadratic_remainders_checked(&ctx, &[forged], &options, 1)
            .is_err());
        assert!(sieve
            .filter_quadratic_remainders_checked(&ctx, std::slice::from_ref(&r), &options, 0)
            .is_err());
        assert!(sieve
            .filter_quadratic_remainders_checked(&ctx, &vec![r.clone(); 4097], &options, 1)
            .is_err());
        ctx.cov.hx[0] = ctx.f.f.add(&ctx.cov.hx[0], &E2::ONE);
        assert!(sieve
            .filter_quadratic_remainders_checked(&ctx, &[r], &options, 1)
            .is_err());
        let (ctx, sieve, line) = fixture([15, 2, 1]); // (x-8)(x-9): nine distinct Fp roots.
        assert!(sieve
            .verified_quadratic_remainder(&ctx, &ring, &line, 1)
            .is_none());
        let (ctx, sieve, line) = fixture([2, 6, 1]); // (x-8)(x-5): repeated small support.
        assert!(sieve
            .verified_quadratic_remainder(&ctx, &ring, &line, 1)
            .is_none());
    }
}
