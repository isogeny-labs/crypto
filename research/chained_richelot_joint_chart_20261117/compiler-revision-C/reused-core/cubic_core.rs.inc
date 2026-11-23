struct CubicField {
    p: u64,
    q: FpPoly,
}
impl CubicField {
    fn with_cubic(p: u64, coefficients: [u64; 3]) -> Result<Self, String> {
        if !(5..=101).contains(&p) || coefficients.iter().any(|c| *c >= p) {
            return Err("cubic prototype requires canonical coefficients,prime5..101".into());
        }
        Fp2Ctx::try_new(p).map_err(|e| e.to_string())?;
        let q = FpPoly::from_coeffs(
            coefficients
                .into_iter()
                .chain([1])
                .map(BigUint::from)
                .collect(),
            BigUint::from(p),
        );
        if (0..p).any(|x| q.eval(&BigUint::from(x)).is_zero()) {
            return Err("reducible cubic modulus".into());
        }
        Ok(Self { p, q })
    }
    fn z(&self) -> E {
        FpPoly::zero(self.q.p.clone())
    }
    fn c(&self, n: u64) -> E {
        FpPoly::constant(BigUint::from(n % self.p), self.q.p.clone())
    }
    fn element(&self, n: u64) -> E {
        FpPoly::from_coeffs(
            [
                n % self.p,
                n / self.p % self.p,
                n / (self.p * self.p) % self.p,
            ]
            .into_iter()
            .map(BigUint::from)
            .collect(),
            self.q.p.clone(),
        )
    }
    fn mul(&self, a: &E, b: &E) -> E {
        a.mul(b).rem(&self.q)
    }
    fn pow(&self, a: &E, mut n: u64) -> E {
        let mut a = a.clone();
        let mut out = self.c(1);
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(&out, &a);
            }
            a = self.mul(&a, &a);
            n >>= 1;
        }
        out
    }
    fn inv(&self, a: &E) -> E {
        let (g, s, _) = a.ext_gcd(&self.q);
        assert_eq!(g, self.c(1), "zero/nonunit in cubic residue field");
        s.rem(&self.q)
    }
    fn encoding(&self, a: &E) -> u64 {
        (0..3)
            .map(|i| a.coeff(i).to_u64().unwrap() * self.p.pow(i as u32))
            .sum()
    }
    /// Deterministic Tonelli-Shanks in the checked cubic field. The
    /// smaller coefficient encoding preserves the old F17 root convention.
    fn sqrt(&self, a: &E) -> Option<E> {
        if a.is_zero() {
            return Some(self.z());
        }
        let cardinality = self.p.pow(3);
        if self.pow(a, (cardinality - 1) / 2) != self.c(1) {
            return None;
        }
        let (mut odd, mut power) = (cardinality - 1, 0_u32);
        while odd.is_multiple_of(2) {
            odd /= 2;
            power += 1;
        }
        let nr = Fp2Ctx::try_new(self.p).unwrap().nonresidue();
        let mut c = self.pow(&self.c(nr), odd);
        let mut t = self.pow(a, odd);
        let mut x = self.pow(a, odd.div_ceil(2));
        while t != self.c(1) {
            let mut i = 1_u32;
            let mut test = self.mul(&t, &t);
            while i < power && test != self.c(1) {
                test = self.mul(&test, &test);
                i += 1;
            }
            assert!(i < power, "Tonelli orbit failed in checked field");
            let b = self.pow(&c, 1_u64 << (power - i - 1));
            x = self.mul(&x, &b);
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            power = i;
        }
        assert_eq!(self.mul(&x, &x), *a);
        let negative = x.neg();
        Some(if self.encoding(&negative) < self.encoding(&x) {
            negative
        } else {
            x
        })
    }
    fn base(&self, a: &E) -> u64 {
        assert!(
            a.degree().is_none_or(|d| d == 0),
            "coefficient did not descend"
        );
        a.coeff(0).to_u64().unwrap()
    }
    fn trim(&self, mut a: Poly) -> Poly {
        while a.last().is_some_and(FpPoly::is_zero) {
            a.pop();
        }
        a
    }
    fn add_poly(&self, a: &Poly, b: &Poly) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    a.get(i)
                        .cloned()
                        .unwrap_or_else(|| self.z())
                        .add(&b.get(i).cloned().unwrap_or_else(|| self.z()))
                })
                .collect(),
        )
    }
    fn neg_poly(&self, a: &Poly) -> Poly {
        a.iter().map(FpPoly::neg).collect()
    }
    fn mul_poly(&self, a: &Poly, b: &Poly) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![self.z(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = out[i + j].add(&self.mul(x, y));
            }
        }
        self.trim(out)
    }
    fn derivative(&self, a: &Poly) -> Poly {
        self.trim(
            a.iter()
                .enumerate()
                .skip(1)
                .map(|(i, c)| self.mul(c, &self.c(i as u64)))
                .collect(),
        )
    }
    fn product(&self, g: &[Poly; 3]) -> Poly {
        self.mul_poly(&self.mul_poly(&g[0], &g[1]), &g[2])
    }
    fn determinant(&self, g: &[Poly; 3]) -> E {
        let mut out = self.z();
        for i in 0..3 {
            out = out.add(&self.mul(&self.mul(&g[0][i], &g[1][(i + 1) % 3]), &g[2][(i + 2) % 3]));
            out = out.sub(&self.mul(&self.mul(&g[0][i], &g[1][(i + 2) % 3]), &g[2][(i + 1) % 3]));
        }
        out
    }
    fn dual_factors(&self, g: &[Poly; 3], delta: &E) -> [Poly; 3] {
        let scale = self.inv(delta);
        std::array::from_fn(|i| {
            let a = &g[(i + 1) % 3];
            let b = &g[(i + 2) % 3];
            let bracket = self.add_poly(
                &self.mul_poly(&self.derivative(a), b),
                &self.neg_poly(&self.mul_poly(a, &self.derivative(b))),
            );
            bracket.iter().map(|c| self.mul(c, &scale)).collect()
        })
    }
}
