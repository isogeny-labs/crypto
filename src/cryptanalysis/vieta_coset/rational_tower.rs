//! Certified finite fibers and division-free homogeneous membership in the
//! quadratic Vieta algebra. Construction is charged, never hidden in a query.
use super::*;

#[derive(Clone, Debug)]
pub struct RationalMap<E> {
    /// Coefficients in ascending order.
    pub numerator: Vec<E>,
    pub denominator: Vec<E>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn context(p: u64) -> Arithmetic<WordField> {
        Arithmetic::new(WordField(p), Limits::default()).unwrap()
    }
    fn quadratic() -> RationalMap<u64> {
        RationalMap {
            numerator: vec![1, 0, 1],
            denominator: vec![0, 2],
        }
    }
    #[test]
    fn rational_tower_complete_relations_match_scalar_enumeration() {
        let mut diagonal_relations = 0;
        let mut torsion_targets = 0;
        for (p, curve, maps, c) in [
            (17, Curve { a: 2, b: 2 }, vec![quadratic(), quadratic()], 4),
            (17, Curve { a: 1, b: 0 }, vec![quadratic(), quadratic()], 4),
            (17, Curve { a: 2, b: 2 }, vec![], 9),
            (
                97,
                Curve { a: 2, b: 3 },
                vec![RationalMap {
                    numerator: vec![0, 0, 0, 1],
                    denominator: vec![1],
                }],
                8,
            ),
            (
                97,
                Curve { a: 0, b: 7 },
                vec![
                    quadratic(),
                    RationalMap {
                        numerator: vec![0, 6, 0, 2],
                        denominator: vec![2, 0, 6],
                    },
                ],
                58,
            ),
        ] {
            let setup = context(p);
            let tower = setup.certify_tower(&maps, c, 42).unwrap();
            for x in 0..p {
                assert_eq!(
                    setup.contains_tower(&tower, &x).unwrap(),
                    tower.leaves.contains(&x)
                );
            }
            let base = setup.tower_factor_base(&curve, &tower).unwrap();
            let mut targets = vec![None];
            for x in 0..p {
                for y in 0..p {
                    if setup.on_curve(&curve, &Some((x, y))) {
                        targets.push(Some((x, y)));
                    }
                }
            }
            for target in targets {
                let expected = context(p).lookup_scalar(&curve, &base, &target).unwrap();
                diagonal_relations += expected.iter().filter(|(a, b)| a == b).count();
                if target.is_some_and(|(_, y)| y == 0) {
                    torsion_targets += 1;
                }
                for mode in [PolynomialBackend::Schoolbook, PolynomialBackend::HalfGcd] {
                    assert_eq!(
                        context(p)
                            .with_polynomials(mode)
                            .solve_tower(&curve, &tower, &target, 19)
                            .unwrap(),
                        expected
                    );
                }
            }
        }
        assert!(diagonal_relations > 0 && torsion_targets > 0);
    }
    #[test]
    fn rational_tower_rejects_partial_repeated_pole_and_invalid_descriptors() {
        let a = context(17);
        let full = a.certify_tower(&[quadratic(), quadratic()], 4, 11).unwrap();
        assert_eq!(full.abscissae(), [2, 9, 10, 12]);
        assert!(!a.contains_tower(&full, &0).unwrap());
        for maps in [vec![quadratic()], vec![quadratic(), quadratic()]] {
            for c in [1, 2] {
                assert!(a.certify_tower(&maps, c, 42).is_err());
            }
        }
        assert!(a
            .certify_tower(&[quadratic(), quadratic()], 14, 42)
            .is_err());
        for m in [
            RationalMap {
                numerator: vec![16, 0, 1],
                denominator: vec![16, 1],
            },
            RationalMap {
                numerator: vec![1, 0, 1],
                denominator: vec![1, 0, 1],
            },
            RationalMap {
                numerator: vec![1, 0, 1],
                denominator: vec![0],
            },
            RationalMap {
                numerator: vec![17, 0, 1],
                denominator: vec![1],
            },
        ] {
            assert!(a.certify_tower(&[m], 4, 42).is_err());
        }
        assert!(a.certify_tower(&vec![quadratic(); 15], 4, 42).is_err());
        assert!(a.certify_tower(&[], 17, 42).is_err());
        assert!(context(97).contains_tower(&full, &2).is_err());
        assert!(context(97)
            .tower_factor_base(&Curve { a: 2, b: 3 }, &full)
            .is_err());
        assert!(context(97)
            .solve_tower(&Curve { a: 2, b: 3 }, &full, &None, 42)
            .is_err());
        let limited = Arithmetic::new(
            WordField(17),
            Limits {
                max_muls: 1,
                ..Limits::default()
            },
        )
        .unwrap();
        assert!(matches!(
            limited.certify_tower(&[quadratic()], 4, 42),
            Err(Error::Budget(_))
        ));
    }
    #[test]
    fn rational_tower_uses_the_same_wide_field_implementation() {
        let field = WideField(BigUint::from(17u64));
        let a = Arithmetic::new(field.clone(), Limits::default())
            .unwrap()
            .with_polynomials(PolynomialBackend::HalfGcd);
        let m = RationalMap {
            numerator: vec![1u8.into(), 0u8.into(), 1u8.into()],
            denominator: vec![0u8.into(), 2u8.into()],
        };
        let tower = a.certify_tower(&[m.clone(), m], 4u8.into(), 42).unwrap();
        let e = Curve {
            a: 2u8.into(),
            b: 2u8.into(),
        };
        let r = Some((6u8.into(), 14u8.into()));
        let base = a.tower_factor_base(&e, &tower).unwrap();
        assert_eq!(
            a.solve_tower(&e, &tower, &r, 42).unwrap(),
            a.lookup_scalar(&e, &base, &r).unwrap()
        );
    }
    #[test]
    fn normalized_velu_map_supplies_a_certified_prime_curve_factor_base() {
        let a = context(17);
        let map = a.velu_two_map(&Curve { a: 1, b: 0 }, 0).unwrap();
        assert_eq!(map.numerator, vec![1, 0, 1]);
        assert_eq!(map.denominator, vec![0, 1]);
        let tower = a.certify_tower(&[map], 6, 42).unwrap();
        assert_eq!(tower.abscissae(), [8, 15]);
        let target_curve = Curve { a: 2, b: 2 }; // order19; the auxiliary curve is separate.
        let base = a.tower_factor_base(&target_curve, &tower).unwrap();
        for x in 0..17 {
            for y in 0..17 {
                let r = Some((x, y));
                if a.on_curve(&target_curve, &r) {
                    assert_eq!(
                        a.solve_tower(&target_curve, &tower, &r, 42).unwrap(),
                        a.lookup_scalar(&target_curve, &base, &r).unwrap()
                    );
                }
            }
        }
        assert!(a.velu_two_map(&Curve { a: 1, b: 0 }, 1).is_err());
    }
}

/// The private fields can only be built after every backward fiber is checked.
/// The prime-modulus precondition of Arithmetic still applies.
#[derive(Clone, Debug)]
pub struct SplitTower<E> {
    maps: Vec<RationalMap<E>>,
    terminal: E,
    leaves: Vec<E>,
    modulus: BigUint,
}
impl<E> SplitTower<E> {
    pub fn abscissae(&self) -> &[E] {
        &self.leaves
    }
    pub fn maps(&self) -> &[RationalMap<E>] {
        &self.maps
    }
    pub fn terminal(&self) -> &E {
        &self.terminal
    }
}
type Pair<E> = (Poly<E>, Poly<E>);

impl<F: PrimeField> Arithmetic<F> {
    /// A fully checked Mobius conjugate of repeated squaring. This is a
    /// rational-map fixture, not an auxiliary-isogeny search or DLP transfer.
    pub fn conjugate_square_tower(
        &self,
        depth: u32,
        g: F::E,
        seed: u64,
    ) -> Result<SplitTower<F::E>> {
        if depth > 14 || !self.field.canonical(&g) || g == self.zero() {
            return Err(Error::Invalid(
                "invalid conjugate-square depth or representative",
            ));
        }
        let m = 1usize << depth;
        if (self.field.modulus() - 1u8) % m != BigUint::zero() {
            return Err(Error::Invalid("tower degree must divide p-1"));
        }
        let terminal_power = self.pow(&g, &BigUint::from(m))?;
        if terminal_power == self.one() {
            return Err(Error::Invalid("Mobius fiber contains infinity"));
        }
        let terminal = self.mul(
            &self.add(&terminal_power, &self.one()),
            &self.inv(&self.sub(&terminal_power, &self.one()))?,
        );
        let map = RationalMap {
            numerator: vec![self.one(), self.zero(), self.one()],
            denominator: vec![self.zero(), self.n(2)],
        };
        self.certify_tower(&vec![map; depth as usize], terminal, seed)
    }
    /// Normalized Vélu x-map for a rational nonidentity 2-torsion kernel:
    /// x -> x + tau/(x-xi), tau=3*xi^2+A. The auxiliary curve supplies a
    /// field subset; no transfer of a discrete logarithm is performed.
    pub fn velu_two_map(&self, auxiliary: &Curve<F::E>, xi: F::E) -> Result<RationalMap<F::E>> {
        self.validate_curve(auxiliary)?;
        if !self.on_curve(auxiliary, &Some((xi.clone(), self.zero()))) {
            return Err(Error::Invalid(
                "kernel is not rational nonidentity two-torsion",
            ));
        }
        let tau = self.add(&self.mul(&self.n(3), &self.mul(&xi, &xi)), &auxiliary.a);
        if tau == self.zero() {
            return Err(Error::Invalid("degenerate Velu map"));
        }
        self.check()?;
        Ok(RationalMap {
            numerator: vec![tau, self.neg(&xi), self.one()],
            denominator: vec![self.neg(&xi), self.one()],
        })
    }
    /// Validate coprimality/degree/poles and every finite fiber, including all
    /// sibling fibers. An empty tower is the singleton terminal set.
    pub fn certify_tower(
        &self,
        maps: &[RationalMap<F::E>],
        terminal: F::E,
        mut seed: u64,
    ) -> Result<SplitTower<F::E>> {
        if !self.field.canonical(&terminal) {
            return Err(Error::Invalid("noncanonical terminal"));
        }
        let mut normalized = Vec::new();
        let mut size = 1usize;
        for map in maps {
            if map
                .numerator
                .iter()
                .chain(&map.denominator)
                .any(|v| !self.field.canonical(v))
            {
                return Err(Error::Invalid("noncanonical rational map"));
            }
            let n = self.trim(map.numerator.clone());
            let d = self.trim(map.denominator.clone());
            if n.len() < 3 || d.len() >= n.len() || self.is_zero(&d) {
                return Err(Error::Invalid("need deg N=q>=2 and nonzero deg D<q"));
            }
            size = size
                .checked_mul(n.len() - 1)
                .filter(|n| *n <= 16384)
                .ok_or(Error::Invalid("tower degree exceeds16384"))?;
            if self.gcd(&n, &d)?.len() != 1 {
                return Err(Error::Invalid("noncoprime rational map"));
            }
            normalized.push(RationalMap {
                numerator: n,
                denominator: d,
            });
        }
        let mut leaves = vec![terminal.clone()];
        for map in normalized.iter().rev() {
            let mut previous = BTreeSet::new();
            for child in &leaves {
                self.check()?;
                self.count(|w| w.certified_fibers += 1);
                let fiber = self.psub_direct(&map.numerator, &self.pscale(&map.denominator, child));
                let roots = self.roots(&fiber, &mut seed)?;
                if roots.len() != map.numerator.len() - 1 {
                    return Err(Error::Invalid("fiber is not fully split and separable"));
                }
                for root in roots {
                    if self.eval(&map.denominator, &root) == self.zero() || !previous.insert(root) {
                        return Err(Error::Inconsistent("pole or duplicated fiber leaf"));
                    }
                }
            }
            leaves = previous.into_iter().collect();
        }
        if leaves.len() != size {
            return Err(Error::Inconsistent("wrong split fiber size"));
        }
        self.check()?;
        Ok(SplitTower {
            maps: normalized,
            terminal,
            leaves,
            modulus: self.field.modulus(),
        })
    }

    /// Homogeneous scalar evaluation, without dividing by a denominator.
    pub fn contains_tower(&self, tower: &SplitTower<F::E>, x: &F::E) -> Result<bool> {
        if tower.modulus != self.field.modulus() {
            return Err(Error::Invalid("tower field mismatch"));
        }
        if !self.field.canonical(x) {
            return Err(Error::Invalid("noncanonical abscissa"));
        }
        let (mut u, mut w) = (x.clone(), self.one());
        for map in &tower.maps {
            self.check()?;
            self.count(|v| v.rational_map_steps += 1);
            let q = map.numerator.len() - 1;
            let mut powers = vec![self.one()];
            for j in 1..=q {
                powers.push(self.mul(&powers[j - 1], &w));
            }
            let hom = |coeff: &[F::E]| {
                let mut acc = coeff.get(q).cloned().unwrap_or_else(|| self.zero());
                for k in (0..q).rev() {
                    acc = self.mul(&acc, &u);
                    if let Some(c) = coeff.get(k) {
                        acc = self.add(&acc, &self.mul(c, &powers[q - k]));
                    }
                }
                acc
            };
            (u, w) = (hom(&map.numerator), hom(&map.denominator));
        }
        self.check()?;
        Ok(w != self.zero() && u == self.mul(&tower.terminal, &w))
    }

    pub fn tower_factor_base(
        &self,
        curve: &Curve<F::E>,
        tower: &SplitTower<F::E>,
    ) -> Result<Vec<(F::E, F::E)>> {
        self.validate_curve(curve)?;
        if tower.modulus != self.field.modulus() {
            return Err(Error::Invalid("tower field mismatch"));
        }
        let mut base = vec![];
        for x in &tower.leaves {
            self.check()?;
            let rhs = self.add(
                &self.add(&self.mul(&self.mul(x, x), x), &self.mul(&curve.a, x)),
                &curve.b,
            );
            if let Some(y) = self.sqrt(&rhs)? {
                base.push((x.clone(), y.clone()));
                if y != self.zero() {
                    base.push((x.clone(), self.neg(&y)));
                }
            }
        }
        base.sort();
        Ok(base)
    }

    fn pair_add(&self, a: &Pair<F::E>, b: &Pair<F::E>) -> Pair<F::E> {
        (self.padd(&a.0, &b.0), self.padd(&a.1, &b.1))
    }
    fn pair_scale(&self, a: &Pair<F::E>, c: &F::E) -> Pair<F::E> {
        (self.pscale(&a.0, c), self.pscale(&a.1, c))
    }
    fn pair_mul(
        &self,
        a: &Pair<F::E>,
        b: &Pair<F::E>,
        s: &[F::E],
        v: &[F::E],
    ) -> Result<Pair<F::E>> {
        self.qmul((&a.0, &a.1), (&b.0, &b.1), s, v)
    }
    fn tower_remainder(
        &self,
        tower: &SplitTower<F::E>,
        s: &[F::E],
        v: &[F::E],
    ) -> Result<Pair<F::E>> {
        let (mut u, mut w) = (
            (vec![self.one()], vec![self.zero()]),
            (vec![self.zero()], vec![self.one()]),
        );
        for map in &tower.maps {
            self.count(|v| v.rational_map_steps += 1);
            let q = map.numerator.len() - 1;
            let mut powers = vec![(vec![self.zero()], vec![self.one()])];
            for j in 1..=q {
                powers.push(self.pair_mul(&powers[j - 1], &w, s, v)?);
            }
            let hom = |coeff: &[F::E]| -> Result<Pair<F::E>> {
                let mut acc = (
                    vec![self.zero()],
                    vec![coeff.get(q).cloned().unwrap_or_else(|| self.zero())],
                );
                for k in (0..q).rev() {
                    acc = self.pair_mul(&acc, &u, s, v)?;
                    if let Some(c) = coeff.get(k) {
                        acc = self.pair_add(&acc, &self.pair_scale(&powers[q - k], c));
                    }
                }
                Ok(acc)
            };
            (u, w) = (hom(&map.numerator)?, hom(&map.denominator)?);
        }
        let w = self.pair_scale(&w, &tower.terminal);
        Ok((self.psub_direct(&u.0, &w.0), self.psub_direct(&u.1, &w.1)))
    }

    /// Complete ordered finite-point decompositions. The finite-target path
    /// composes homogeneous pairs and never enumerates the certified leaves.
    pub fn solve_tower(
        &self,
        e: &Curve<F::E>,
        tower: &SplitTower<F::E>,
        target: &Point<F::E>,
        seed: u64,
    ) -> Result<Vec<Relation<F::E>>> {
        self.validate_curve(e)?;
        if tower.modulus != self.field.modulus() {
            return Err(Error::Invalid("tower field mismatch"));
        }
        if !self.on_curve(e, target) {
            return Err(Error::Invalid("target off curve"));
        }
        let Some((t, r)) = target else {
            return Ok(self
                .tower_factor_base(e, tower)?
                .into_iter()
                .map(|p| {
                    let q = (p.0.clone(), self.neg(&p.1));
                    (p, q)
                })
                .collect());
        };
        let s = vec![self.neg(t), self.zero(), self.one()];
        let v = vec![
            self.add(&e.a, &self.mul(t, t)),
            self.mul(&self.n(2), r),
            t.clone(),
        ];
        let (a, b) = self.tower_remainder(tower, &s, &v)?;
        let g = self.gcd(&a, &b)?;
        let delta = self.psub_direct(&self.pmul(&s, &s)?, &self.pscale(&v, &self.n(4)));
        let inv2 = self.inv(&self.n(2))?;
        let half_s = self.pscale(&s, &inv2);
        // H(s/2) modulo Delta, not divisibility by the repeated quadratic.
        let ra = self.divrem(&a, &delta)?.1;
        let rb = self.divrem(&b, &delta)?.1;
        let diagonal = self.gcd(&delta, &self.padd(&self.pmul(&ra, &half_s)?, &rb))?;
        let mut rng = seed;
        let mut out = BTreeSet::new();
        for (f, repeated) in [(&g, false), (&diagonal, true)] {
            for l in self.roots(f, &mut rng)? {
                let sum = self.eval(&s, &l);
                let disc = self.eval(&delta, &l);
                if repeated != (disc == self.zero()) {
                    continue;
                }
                let Some(root) = self.sqrt(&disc)? else {
                    return Err(Error::Inconsistent(
                        "split tower gave nonsquare discriminant",
                    ));
                };
                let x = self.mul(&self.add(&sum, &root), &inv2);
                let y = self.sub(&sum, &x);
                let p = (x.clone(), self.sub(&self.mul(&l, &self.sub(&x, t)), r));
                let q = (y.clone(), self.sub(&self.mul(&l, &self.sub(&y, t)), r));
                if !self.contains_tower(tower, &x)?
                    || !self.contains_tower(tower, &y)?
                    || !self.on_curve(e, &Some(p.clone()))
                    || !self.on_curve(e, &Some(q.clone()))
                    || self.group_add(e, &Some(p.clone()), &Some(q.clone()))? != *target
                {
                    return Err(Error::Inconsistent("invalid tower witness"));
                }
                out.insert((p.clone(), q.clone()));
                out.insert((q, p));
            }
        }
        self.check()?;
        Ok(out.into_iter().collect())
    }
}
