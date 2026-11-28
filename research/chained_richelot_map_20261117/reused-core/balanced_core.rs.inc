#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    f: FpPoly,
    u: FpPoly,
    v: FpPoly,
    /// Effective degree2 divisor: D_fin + plus*inf+ + (2-deg(u)-plus)*inf-.
    /// The class subtracts K=inf+ + inf-. Identity is(u=1,v=0,plus=1).
    plus: u8,
}
impl Class {
    pub(super) fn components(&self) -> (&FpPoly, &FpPoly, u8) {
        (&self.u, &self.v, self.plus)
    }
    fn degree(&self) -> usize {
        self.u.degree().unwrap()
    }
    fn key(&self) -> Key {
        let c = |f: &FpPoly| f.coeffs.iter().map(|c| c.to_u64().unwrap()).collect();
        (c(&self.u), c(&self.v), self.plus)
    }
}

#[derive(Clone)]
pub struct Balanced {
    p: u64,
    f: FpPoly,
    polynomial_part: FpPoly,
}
fn canonical(f: &FpPoly, p: &BigUint) -> bool {
    f.p == *p && f.coeffs.iter().all(|c| c < p) && f.coeffs.last().is_none_or(|c| !c.is_zero())
}
impl Balanced {
    pub(super) fn equation_and_infinity(&self) -> (&FpPoly, &FpPoly) {
        (&self.f, &self.polynomial_part)
    }
    pub fn new(p: u64, coefficients: &[u64]) -> Result<Self> {
        if !(5..=101).contains(&p)
            || coefficients.len() != 7
            || coefficients.iter().any(|c| *c >= p)
        {
            return Err("balanced prototype requires canonical sextic over prime5..101".into());
        }
        Fp2Ctx::try_new(p).map_err(|e| e.to_string())?;
        let modulus = BigUint::from(p);
        let f = FpPoly::from_coeffs(
            coefficients.iter().copied().map(BigUint::from).collect(),
            modulus.clone(),
        );
        if f.lead() != BigUint::one() {
            return Err("only monic split-infinity sextics supported".into());
        }
        let derivative = FpPoly::from_coeffs(
            (1..7)
                .map(|i| BigUint::from(i as u64) * f.coeff(i) % &modulus)
                .collect(),
            modulus.clone(),
        );
        if f.gcd(&derivative) != FpPoly::one(modulus.clone()) {
            return Err("singular sextic".into());
        }
        let inv2 = p.div_ceil(2);
        let c2 = coefficients[5] * inv2 % p;
        let c1 = (coefficients[4] + p - c2 * c2 % p) * inv2 % p;
        let c0 = (coefficients[3] + p - 2 * c2 * c1 % p) * inv2 % p;
        let polynomial_part = FpPoly::from_coeffs(
            [c0, c1, c2, 1].into_iter().map(BigUint::from).collect(),
            modulus,
        );
        if f.sub(&polynomial_part.mul(&polynomial_part))
            .degree()
            .is_some_and(|d| d > 2)
        {
            return Err("invalid infinity expansion".into());
        }
        Ok(Self {
            p,
            f,
            polynomial_part,
        })
    }
    fn poly(&self, c: &[u64]) -> FpPoly {
        FpPoly::from_coeffs(
            c.iter().copied().map(BigUint::from).collect(),
            BigUint::from(self.p),
        )
    }
    pub(super) fn wrap(&self, u: FpPoly, v: FpPoly, plus: u8) -> Result<Class> {
        let d = Class {
            f: self.f.clone(),
            u,
            v,
            plus,
        };
        self.check(&d)?;
        Ok(d)
    }
    pub(super) fn check(&self, d: &Class) -> Result<()> {
        if d.f != self.f || !canonical(&d.u, &self.f.p) || !canonical(&d.v, &self.f.p) {
            return Err("balanced curve/field/canonical tag mismatch".into());
        }
        let m = d.u.degree().ok_or("zero support")?;
        if m > 2
            || usize::from(d.plus) + m > 2
            || d.u.lead() != BigUint::one()
            || d.v.degree().is_some_and(|n| n >= m)
        {
            return Err("noncanonical balanced degree or infinity counts".into());
        }
        if !self.f.sub(&d.v.mul(&d.v)).rem(&d.u).is_zero() {
            return Err("support does not lie on sextic".into());
        }
        Ok(())
    }
    pub fn zero(&self) -> Class {
        self.wrap(self.poly(&[1]), self.poly(&[]), 1).unwrap()
    }
    pub fn delta(&self) -> Class {
        self.wrap(self.poly(&[1]), self.poly(&[]), 2).unwrap()
    }
    pub fn point(&self, x: u64, y: u64, plus: u8) -> Result<Class> {
        if x >= self.p || y >= self.p {
            return Err("noncanonical point".into());
        }
        self.wrap(
            self.poly(&[(self.p - x) % self.p, 1]),
            self.poly(&[y]),
            plus,
        )
    }
    pub fn neg(&self, d: &Class) -> Result<Class> {
        self.check(d)?;
        self.wrap(
            d.u.clone(),
            d.v.neg().rem(&d.u),
            2 - d.degree() as u8 - d.plus,
        )
    }
    /// Semireduced affine Cantor composition; no odd-degree reduction.
    fn compose(&self, a: &Class, b: &Class) -> Result<(FpPoly, FpPoly, usize)> {
        let (d1, e1, e2) = a.u.ext_gcd(&b.u);
        let (d, c1, c2) = d1.ext_gcd(&a.v.add(&b.v));
        let exact = |a: &FpPoly, b: &FpPoly| -> Result<FpPoly> {
            let (q, r) = a.divrem(b);
            if !r.is_zero() {
                return Err("nonexact balanced composition".into());
            }
            Ok(q)
        };
        let u = exact(&a.u.mul(&b.u), &d.mul(&d))?;
        let numerator = c1
            .mul(&e1)
            .mul(&a.u)
            .mul(&b.v)
            .add(&c1.mul(&e2).mul(&b.u).mul(&a.v))
            .add(&c2.mul(&a.v.mul(&b.v).add(&self.f)));
        let v = exact(&numerator, &d)?.rem(&u);
        Ok((u, v, d.degree().unwrap()))
    }
    pub fn add(&self, a: &Class, b: &Class) -> Result<Class> {
        self.check(a)?;
        self.check(b)?;
        let (u, v, removed) = self.compose(a, b)?;
        let plus = i64::from(a.plus) + i64::from(b.plus) + removed as i64 - 1;
        self.reduce_composed(u, v, plus)
    }

    /// Checked entry for an existing semireduced fibre of finite degree<=4.
    /// The reduction body below is extracted unchanged from the old add.
    pub(super) fn reduce_support(&self, u: FpPoly, v: FpPoly, plus: i64) -> Result<Class> {
        let m = u.degree().ok_or("zero semireduced support")?;
        if m > 4
            || !(-1..=3).contains(&plus)
            || u.lead() != BigUint::one()
            || !canonical(&u, &self.f.p)
            || !canonical(&v, &self.f.p)
            || v.degree().is_some_and(|r| r >= m)
        {
            return Err("unsupported semireduced fibre shape/field".into());
        }
        if !self.f.sub(&v.mul(&v)).rem(&u).is_zero() {
            return Err("semireduced fibre is off curve".into());
        }
        self.reduce_composed(u, v, plus)
    }

    fn reduce_composed(&self, u: FpPoly, v: FpPoly, plus: i64) -> Result<Class> {
        let m = u.degree().ok_or("zero composed support")?;
        if m <= 2 && plus >= 0 && plus <= 2 - m as i64 {
            return self.wrap(u, v, plus as u8);
        }
        // The degree4 effective sum has only one infinity sign if it is
        // outside the reduced range. Match that branch at infinity.
        let line = if m == 4 {
            v
        } else {
            let g = if plus >= 0 {
                self.polynomial_part.clone()
            } else {
                self.polynomial_part.neg()
            };
            g.add(&v.sub(&g).rem(&u))
        };
        let numerator = self.f.sub(&line.mul(&line));
        let total = numerator.degree().ok_or("sextic became a square")?;
        let (raw, remainder) = numerator.divrem(&u);
        if !remainder.is_zero() || raw.is_zero() {
            return Err("nonexact infinity reduction".into());
        }
        let next = raw.monic();
        let r = next.degree().unwrap();
        // For line leading +1, v_inf+(y-line)=3-deg(f-line²).
        // Otherwise the positive branch has pole3. This also handles the
        // exceptional degree4 interpolation with leading coefficient +/-1.
        let valuation = if line.coeff(3) == BigUint::one() {
            3 - total as i64
        } else {
            -3
        };
        let next_plus = plus - valuation - r as i64;
        if r > 2 || next_plus < 0 || next_plus > 2 - r as i64 {
            return Err(format!(
                "unbalanced reduction m={m},n={plus},r={r},n'={next_plus}"
            ));
        }
        let next_v = line.neg().rem(&next);
        self.wrap(next, next_v, next_plus as u8)
    }
    pub fn mul(&self, d: &Class, mut n: u64) -> Result<Class> {
        self.check(d)?;
        let mut a = d.clone();
        let mut out = self.zero();
        while n != 0 {
            if n & 1 != 0 {
                out = self.add(&out, &a)?;
            }
            a = self.add(&a, &a)?;
            n >>= 1;
        }
        Ok(out)
    }
}
