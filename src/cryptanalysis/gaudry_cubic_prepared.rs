//! The same symmetric S3 equations with cached curve constants and a
//! stack-sized polynomial workspace. Generic/exceptional root recovery is
//! shared with the reference oracle; most rejected residuals allocate nothing.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Poly5 {
    c: [u64; 5],
    len: usize,
}

impl Poly5 {
    fn new(values: &[u64]) -> Self {
        assert!(values.len() <= 5);
        let mut out = Self {
            c: [0; 5],
            len: values.len(),
        };
        out.c[..values.len()].copy_from_slice(values);
        out.trim();
        out
    }

    fn trim(&mut self) {
        while self.len > 0 && self.c[self.len - 1] == 0 {
            self.len -= 1;
        }
    }

    fn add(self, rhs: Self, ring: &PolyRing) -> Self {
        let mut out = Self::new(&[]);
        out.len = self.len.max(rhs.len);
        for i in 0..out.len {
            out.c[i] = am(self.c[i], rhs.c[i], ring.p);
        }
        out.trim();
        out
    }

    fn sub(self, rhs: Self, ring: &PolyRing) -> Self {
        let mut out = Self::new(&[]);
        out.len = self.len.max(rhs.len);
        for i in 0..out.len {
            out.c[i] = sm(self.c[i], rhs.c[i], ring.p);
        }
        out.trim();
        out
    }

    fn mul(self, rhs: Self, ring: &PolyRing) -> Self {
        if self.len == 0 || rhs.len == 0 {
            return Self::new(&[]);
        }
        let len = self.len + rhs.len - 1;
        assert!(len <= 5, "S3 polynomial workspace degree exceeded");
        let mut out = Self { c: [0; 5], len };
        ring.count((self.len * rhs.len) as u64);
        for i in 0..self.len {
            for j in 0..rhs.len {
                out.c[i + j] = am(out.c[i + j], mm(self.c[i], rhs.c[j], ring.p), ring.p);
            }
        }
        out.trim();
        out
    }

    fn rem(mut self, rhs: Self, ring: &PolyRing) -> Self {
        assert!(rhs.len > 0);
        let inv = inv_mod(rhs.c[rhs.len - 1], ring.p);
        while self.len >= rhs.len {
            let coef = mm(self.c[self.len - 1], inv, ring.p);
            let shift = self.len - rhs.len;
            ring.count(rhs.len as u64 + 1);
            for i in 0..rhs.len {
                self.c[shift + i] = sm(self.c[shift + i], mm(coef, rhs.c[i], ring.p), ring.p);
            }
            self.trim();
        }
        self
    }

    fn mul_delayed(self, rhs: Self, ring: &PolyRing) -> Self {
        if self.len == 0 || rhs.len == 0 {
            return Self::new(&[]);
        }
        let len = self.len + rhs.len - 1;
        assert!(len <= 5, "S3 polynomial workspace degree exceeded");
        if !(2..1 << 20).contains(&ring.p)
            || self.c[..self.len].iter().any(|&c| c >= ring.p)
            || rhs.c[..rhs.len].iter().any(|&c| c >= ring.p)
        {
            return self.mul(rhs, ring);
        }
        // At most five normalized products: sum < 5 * 2^40 < 2^43.
        let mut out = Self { c: [0; 5], len };
        ring.count((self.len * rhs.len) as u64);
        for i in 0..self.len {
            for j in 0..rhs.len {
                super::super::jv_bitops::hook_mul(ring.p);
                super::super::jv_bitops::hook_add(ring.p);
                out.c[i + j] += self.c[i] * rhs.c[j];
            }
        }
        for c in &mut out.c[..len] {
            *c %= ring.p;
        }
        out.trim();
        out
    }

    fn gcd(self, rhs: Self, ring: &PolyRing) -> Self {
        self.gcd_with_remainders(rhs, ring, false)
    }

    // lc(rhs)^d * self = quotient * rhs + remainder. Every scale is a
    // nonzero field unit, so Euclidean GCDs are preserved up to a scalar.
    fn pseudo_rem(mut self, rhs: Self, ring: &PolyRing) -> Self {
        assert!(rhs.len > 0);
        let leading = rhs.c[rhs.len - 1];
        while self.len >= rhs.len {
            let top = self.len - 1;
            let coef = self.c[top];
            let shift = self.len - rhs.len;
            ring.count((top + rhs.len - 1) as u64);
            for c in &mut self.c[..top] {
                *c = mm(*c, leading, ring.p);
            }
            for i in 0..rhs.len - 1 {
                self.c[shift + i] = sm(self.c[shift + i], mm(coef, rhs.c[i], ring.p), ring.p);
            }
            // The two leading products cancel symbolically.
            self.c[top] = 0;
            self.trim();
        }
        self
    }

    fn gcd_with_remainders(mut self, mut rhs: Self, ring: &PolyRing, pseudo: bool) -> Self {
        while rhs.len > 0 {
            // A nonzero constant proves coprimality; no remaining polynomial
            // division or finite-field root extraction is needed.
            if rhs.len == 1 {
                return Self::new(&[1]);
            }
            let rem = if pseudo {
                self.pseudo_rem(rhs, ring)
            } else {
                self.rem(rhs, ring)
            };
            self = rhs;
            rhs = rem;
        }
        if self.len > 0 {
            let inv = inv_mod(self.c[self.len - 1], ring.p);
            ring.count(self.len as u64);
            for c in &mut self.c[..self.len] {
                *c = mm(*c, inv, ring.p);
            }
        }
        self
    }

    fn upoly(self) -> UPoly {
        UPoly(self.c[..self.len].to_vec())
    }
}

pub(super) struct PreparedSymmetricPair {
    aa: E3,
    a2: E3,
    b4: E3,
    pseudo_remainders: bool,
    delayed_products: bool,
}

impl PreparedSymmetricPair {
    pub fn new(inst: &Instance3) -> Self {
        let f = &inst.curve.field;
        Self {
            aa: f.sq(&inst.curve.a),
            a2: f.scale(&inst.curve.a, 2),
            b4: f.scale(&inst.curve.b, 4),
            pseudo_remainders: false,
            delayed_products: false,
        }
    }

    pub fn new_pseudo(inst: &Instance3) -> Self {
        Self {
            pseudo_remainders: true,
            ..Self::new(inst)
        }
    }

    pub fn new_delayed_pseudo(inst: &Instance3) -> Self {
        Self {
            delayed_products: true,
            ..Self::new_pseudo(inst)
        }
    }

    pub fn pair_test(
        &self,
        inst: &Instance3,
        base: &SubspaceBase,
        y: &Pt3,
        rng: &mut StdRng,
        stats: &mut SymmetricPairStats,
    ) -> (Vec<Vec<(usize, i64)>>, u64) {
        stats.calls += 1;
        if y.inf {
            return (vec![], 0);
        }
        let f = &inst.curve.field;
        let before = f.muls();
        let x2 = f.sq(&y.x);
        let uc = f.neg(&f.add(&f.scale(&x2, 4), &self.a2));
        let us = f.neg(&f.scale(&y.x, 2));
        let vc = f.sub(&self.aa, &f.mul(&self.b4, &y.x));
        let vs = f.neg(&f.add(&f.mul(&self.a2, &y.x), &self.b4));
        let component_muls = f.muls() - before;
        let u: [Poly5; 3] = std::array::from_fn(|k| Poly5::new(&[uc.0[k], us.0[k]]));
        let v: [Poly5; 3] = std::array::from_fn(|k| Poly5::new(&[vc.0[k], vs.0[k], x2.0[k]]));
        let ring = PolyRing::new(f.p);
        let mul = |a: Poly5, b: Poly5| {
            if self.delayed_products {
                a.mul_delayed(b, &ring)
            } else {
                a.mul(b, &ring)
            }
        };
        let eliminant = mul(u[1], v[2]).sub(mul(u[2], v[1]), &ring);
        if eliminant.len == 0 {
            stats.degenerate_scans += 1;
            return (
                symmetric::scan_pairs(inst, base, y),
                component_muls + ring.muls.get(),
            );
        }
        stats.cubic_eliminations += 1;
        let pivot = if u[1].len > 0 { 1 } else { 2 };
        let compatibility = mul(v[pivot], v[pivot])
            .sub(mul(mul(u[pivot], u[0]), v[pivot]), &ring)
            .add(mul(mul(u[pivot], u[pivot]), v[0]), &ring);
        let common = if self.pseudo_remainders {
            eliminant.gcd_with_remainders(compatibility, &ring, true)
        } else {
            eliminant.gcd(compatibility, &ring)
        };
        if common.len == 1 {
            stats.constant_gcd_rejections += 1;
            return (vec![], component_muls + ring.muls.get());
        }
        let u = u.map(Poly5::upoly);
        let v = v.map(Poly5::upoly);
        let found =
            symmetric::finish_candidates(inst, base, y, rng, stats, &u, &v, &common.upoly(), &ring);
        (found, component_muls + ring.muls.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_polynomials_match_reference_including_zero_and_degree_drops() {
        let ring = PolyRing::new(7);
        let mut rng = StdRng::seed_from_u64(29);
        for _ in 0..300 {
            let a = Poly5::new(
                &(0..rng.gen_range(0..=5))
                    .map(|_| rng.gen_range(0..7))
                    .collect::<Vec<_>>(),
            );
            let b = Poly5::new(
                &(0..rng.gen_range(0..=5))
                    .map(|_| rng.gen_range(0..7))
                    .collect::<Vec<_>>(),
            );
            assert_eq!(a.add(b, &ring).upoly(), ring.add(&a.upoly(), &b.upoly()));
            assert_eq!(a.sub(b, &ring).upoly(), ring.sub(&a.upoly(), &b.upoly()));
            assert_eq!(a.gcd(b, &ring).upoly(), ring.gcd(&a.upoly(), &b.upoly()));
            if b.len > 0 {
                assert_eq!(a.rem(b, &ring).upoly(), ring.rem(&a.upoly(), &b.upoly()));
            }
        }
    }

    #[test]
    fn prepared_matches_all_tiny_targets_and_general_cubic() {
        let mut instances = vec![generate_instance3(7, 3), generate_instance3(7, 7)];
        let field = Fp3::with_irreducible_cubic(11, [1, 0, 4]).unwrap();
        let curve = Curve3::new(field, E3([1, 1, 0]), E3([2, 0, 1]), 5, Pt3::INFINITY);
        instances.push(Instance3 {
            curve,
            q: Pt3::INFINITY,
            d: 0,
        });
        for inst in instances {
            let mut rng = StdRng::seed_from_u64(31);
            let base = SubspaceBase::build(&inst, &mut rng);
            let prepared = PreparedSymmetricPair::new(&inst);
            let pseudo = PreparedSymmetricPair::new_pseudo(&inst);
            let delayed = PreparedSymmetricPair::new_delayed_pseudo(&inst);
            let mut points = Vec::new();
            if !inst.curve.g.inf {
                for d in 1..inst.curve.n {
                    points.push(inst.curve.mul(&inst.curve.g, d));
                }
            }
            for a in &base.points {
                for b in &base.points {
                    points.push(inst.curve.add(a, b));
                    points.push(inst.curve.sub(a, b));
                }
            }
            for point in points {
                let mut a = StdRng::seed_from_u64(37);
                let mut b = a.clone();
                let mut c = a.clone();
                let mut d = a.clone();
                let got = prepared
                    .pair_test(
                        &inst,
                        &base,
                        &point,
                        &mut a,
                        &mut SymmetricPairStats::default(),
                    )
                    .0;
                let expected = symmetric::pair_test(
                    &inst,
                    &base,
                    &point,
                    &mut b,
                    &mut SymmetricPairStats::default(),
                )
                .0;
                assert_eq!(got, expected, "point={point:?}");
                let (fraction_free, pseudo_cost) = pseudo.pair_test(
                    &inst,
                    &base,
                    &point,
                    &mut c,
                    &mut SymmetricPairStats::default(),
                );
                let (delayed_found, delayed_cost) = delayed.pair_test(
                    &inst,
                    &base,
                    &point,
                    &mut d,
                    &mut SymmetricPairStats::default(),
                );
                assert_eq!(delayed_found, fraction_free, "delayed point={point:?}");
                assert_eq!(
                    delayed_cost, pseudo_cost,
                    "semantic products point={point:?}"
                );
                assert_eq!(
                    c.clone().gen::<u64>(),
                    d.gen::<u64>(),
                    "delayed root RNG stream changed"
                );
                assert_eq!(fraction_free, expected, "pseudo point={point:?}");
                assert_eq!(
                    a.clone().gen::<u64>(),
                    c.gen::<u64>(),
                    "pseudo root RNG stream changed"
                );
                assert_eq!(a.gen::<u64>(), b.gen::<u64>(), "root RNG stream changed");
            }
        }
    }

    #[test]
    fn pseudo_gcd_matches_reference_with_planted_factors_and_degree_drops() {
        let mut rng = StdRng::seed_from_u64(20261116);
        for p in [5, 7, 127, 1039, 1_048_573] {
            let ring = PolyRing::new(p);
            for _ in 0..600 {
                let a = Poly5::new(
                    &(0..rng.gen_range(0..=5))
                        .map(|_| rng.gen_range(0..p))
                        .collect::<Vec<_>>(),
                );
                let b = Poly5::new(
                    &(0..rng.gen_range(0..=5))
                        .map(|_| rng.gen_range(0..p))
                        .collect::<Vec<_>>(),
                );
                assert_eq!(
                    a.gcd_with_remainders(b, &ring, true).upoly(),
                    ring.gcd(&a.upoly(), &b.upoly()),
                    "p={p}, a={a:?}, b={b:?}"
                );
                let factor = Poly5::new(&[rng.gen_range(0..p), 1]);
                let left = Poly5::new(&[rng.gen_range(0..p), rng.gen_range(0..p), 1]);
                let right = Poly5::new(&[rng.gen_range(0..p), 1]);
                let a = factor.mul(left, &ring);
                let b = factor.mul(right, &ring);
                assert_eq!(
                    a.gcd_with_remainders(b, &ring, true).upoly(),
                    ring.gcd(&a.upoly(), &b.upoly()),
                    "planted p={p}"
                );
            }
        }
    }

    #[test]
    fn pseudo_pipeline_preserves_relation_stream_for_both_backends_and_samplers() {
        let inst = generate_instance3(67, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        for sampler in [ResidualSampler3::FreshScalar, ResidualSampler3::RAddingLong] {
            for backend in [LargePrimeBackend::Pivot, LargePrimeBackend::SymbolicGraph] {
                let mut opts = GaudryOptions {
                    max_residuals: 100000,
                    solver: Solver::PreparedSymmetricPairOnly,
                    small_base: (base.len() as f64).powf(2.0 / 3.0).ceil() as usize,
                    max_large_primes: 2,
                    ..GaudryOptions::default()
                };
                let baseline = run_gaudry_with_sampler(&inst, &base, 3, &opts, backend, sampler);
                opts.solver = Solver::PseudoRemainderPairOnly;
                let candidate = run_gaudry_with_sampler(&inst, &base, 3, &opts, backend, sampler);
                opts.solver = Solver::DelayedPseudoRemainderPairOnly;
                let delayed = run_gaudry_with_sampler(&inst, &base, 3, &opts, backend, sampler);
                assert_eq!(baseline.correct, Some(true));
                assert_eq!(candidate.correct, Some(true));
                assert_eq!(baseline.recovered_scalar, candidate.recovered_scalar);
                assert_eq!(baseline.lp_input_digest, candidate.lp_input_digest);
                assert_eq!(baseline.residuals, candidate.residuals);
                assert_eq!(baseline.group_ops, candidate.group_ops);
                assert_eq!(baseline.decompositions, candidate.decompositions);
                assert_eq!(baseline.lp_histogram, candidate.lp_histogram);
                assert_eq!(delayed.correct, Some(true));
                assert_eq!(delayed.lp_input_digest, candidate.lp_input_digest);
                assert_eq!(delayed.residuals, candidate.residuals);
                assert_eq!(delayed.group_ops, candidate.group_ops);
                assert_eq!(delayed.recovered_scalar, candidate.recovered_scalar);
                assert_eq!(delayed.oracle_fp_muls, candidate.oracle_fp_muls);
            }
        }
    }

    #[test]
    fn delayed_products_match_scalar_at_bounds_and_outside_the_guard() {
        let mut rng = StdRng::seed_from_u64(20261117);
        for p in [2, 7, 127, 1039, 1_048_573, 1_048_583] {
            let ring = PolyRing::new(p);
            let mut cases = vec![
                (Poly5::new(&[p - 1; 3]), Poly5::new(&[p - 1; 3])),
                (Poly5::new(&[p + 1, p + 2]), Poly5::new(&[1, p - 1])),
            ];
            for a_len in 0..=5 {
                for b_len in 0..=5 {
                    if a_len > 0 && b_len > 0 && a_len + b_len > 6 {
                        continue;
                    }
                    for _ in 0..12 {
                        let a = Poly5::new(
                            &(0..a_len).map(|_| rng.gen_range(0..p)).collect::<Vec<_>>(),
                        );
                        let b = Poly5::new(
                            &(0..b_len).map(|_| rng.gen_range(0..p)).collect::<Vec<_>>(),
                        );
                        cases.push((a, b));
                    }
                }
            }
            for (a, b) in cases {
                let before = ring.muls.get();
                let scalar = a.mul(b, &ring);
                let scalar_work = ring.muls.get() - before;
                let before = ring.muls.get();
                let delayed = a.mul_delayed(b, &ring);
                assert_eq!(scalar, delayed, "p={p}, a={a:?}, b={b:?}");
                assert_eq!(scalar_work, ring.muls.get() - before);
            }
        }
    }
}
