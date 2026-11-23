//! Checked small-prime Richelot backend, reusing the repository's unchanged
//! Fp2 context (odd primes <=4096). Exhaustive fixtures are capped at101.
//! This is development arithmetic, not cryptographic-size field support.
// The reference field source is part of the source-pin closure and CI checkout.
#[allow(dead_code, clippy::wrong_self_convention)]
#[path = "../../../../src/prime_hyperelliptic/fp2.rs"]
mod field_reference;
pub use field_reference::MAX_FP2_MODULUS;
use field_reference::{Fp2 as E, Fp2Ctx};

pub const MAX_ENUMERATION_PRIME: u64 = 101;
type Poly = Vec<E>;
type Result<T> = std::result::Result<T, &'static str>;
pub type Point = Option<(u64, u64)>;
type ExtendedPoint = Option<(E, E)>;

#[derive(Clone, Copy, Debug)]
struct Field(Fp2Ctx);
impl PartialEq for Field {
    fn eq(&self, b: &Self) -> bool {
        self.0.modulus() == b.0.modulus() && self.0.nonresidue() == b.0.nonresidue()
    }
}
impl Eq for Field {}
impl Field {
    fn new(p: u64) -> Result<Self> {
        let ctx = Fp2Ctx::try_new(p).map_err(|_| "requires an odd prime <=4096")?;
        ctx.validate().map_err(|_| "invalid field context")?;
        Ok(Self(ctx))
    }
    fn p(self) -> u64 {
        self.0.modulus()
    }
    fn e(self, a: i64) -> E {
        self.0.from_fp(a.rem_euclid(self.p() as i64) as u64)
    }
    fn z(self) -> E {
        self.0.zero()
    }
    fn o(self) -> E {
        self.0.one()
    }
    fn add(self, a: E, b: E) -> E {
        self.0.add(a, b)
    }
    fn neg(self, a: E) -> E {
        self.0.neg(a)
    }
    fn sub(self, a: E, b: E) -> E {
        self.0.sub(a, b)
    }
    fn mul(self, a: E, b: E) -> E {
        self.0.mul(a, b)
    }
    fn inv(self, a: E) -> E {
        self.0.inv(a).expect("nonzero field denominator")
    }
    fn pow(self, a: E, n: u64) -> E {
        self.0.pow(a, n)
    }
    fn base(self, a: &[i64]) -> Poly {
        self.trim(a.iter().map(|x| self.e(*x)).collect())
    }
    fn trim(self, mut a: Poly) -> Poly {
        while a.last() == Some(&self.z()) {
            a.pop();
        }
        a
    }
    fn plus(self, a: &[E], b: &[E]) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    self.add(
                        *a.get(i).unwrap_or(&self.z()),
                        *b.get(i).unwrap_or(&self.z()),
                    )
                })
                .collect(),
        )
    }
    fn negative(self, a: &[E]) -> Poly {
        self.trim(a.iter().map(|x| self.neg(*x)).collect())
    }
    fn minus(self, a: &[E], b: &[E]) -> Poly {
        self.plus(a, &self.negative(b))
    }
    fn times(self, a: &[E], b: &[E]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![self.z(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = self.add(out[i + j], self.mul(*x, *y));
            }
        }
        self.trim(out)
    }
    fn divide(self, a: &[E], b: &[E]) -> (Poly, Poly) {
        let mut a = self.trim(a.to_vec());
        let b = self.trim(b.to_vec());
        assert!(!b.is_empty());
        let mut q = vec![self.z(); (a.len() + 1).saturating_sub(b.len())];
        let il = self.inv(*b.last().unwrap());
        while !a.is_empty() && a.len() >= b.len() {
            let k = a.len() - b.len();
            let c = self.mul(*a.last().unwrap(), il);
            q[k] = c;
            for (j, bj) in b.iter().enumerate() {
                a[k + j] = self.sub(a[k + j], self.mul(c, *bj));
            }
            a = self.trim(a);
        }
        (self.trim(q), a)
    }
    fn exact(self, a: &[E], b: &[E]) -> Poly {
        let (q, r) = self.divide(a, b);
        assert!(r.is_empty());
        q
    }
    fn monic(self, a: &[E]) -> Poly {
        self.times(a, &[self.inv(*a.last().unwrap())])
    }
    fn gcd(self, mut a: Poly, mut b: Poly) -> (Poly, Poly, Poly) {
        let (mut s, mut ss, mut t, mut tt) = (self.base(&[1]), vec![], vec![], self.base(&[1]));
        while !b.is_empty() {
            let (q, r) = self.divide(&a, &b);
            a = b;
            b = r;
            (s, ss) = (ss.clone(), self.minus(&s, &self.times(&q, &ss)));
            (t, tt) = (tt.clone(), self.minus(&t, &self.times(&q, &tt)));
        }
        let z = [self.inv(*a.last().unwrap())];
        (self.times(&a, &z), self.times(&s, &z), self.times(&t, &z))
    }
    fn value(self, a: &[E], x: E) -> E {
        a.iter()
            .rev()
            .fold(self.z(), |v, c| self.add(self.mul(v, x), *c))
    }
    fn derivative(self, a: &[E]) -> Poly {
        self.trim(
            a.iter()
                .enumerate()
                .skip(1)
                .map(|(i, c)| self.mul(*c, self.e(i as i64)))
                .collect(),
        )
    }
    fn rational(self, a: &[E]) -> Result<Vec<u64>> {
        if a.iter().any(|x| x.b != 0) {
            return Err("coefficients did not descend to prime field");
        }
        Ok(a.iter().map(|x| x.a).collect())
    }
    fn sqrt_base(self, x: E) -> Option<E> {
        (0..self.p())
            .map(|i| self.e(i as i64))
            .find(|r| self.mul(*r, *r) == x)
    }
    fn roots(self, u: &[E]) -> Result<Vec<E>> {
        if u.iter().any(|x| x.b != 0) {
            return Err("rational support required");
        }
        match u.len() {
            1 => Ok(vec![]),
            2 => Ok(vec![self.neg(u[0])]),
            3 => {
                let d = self.sub(self.mul(u[1], u[1]), self.mul(self.e(4), u[0]));
                let s = match self.sqrt_base(d) {
                    Some(s) => s,
                    None => {
                        let nonresidue = self.e(self.0.nonresidue() as i64);
                        let ratio = self.mul(d, self.inv(nonresidue));
                        let t = self
                            .sqrt_base(ratio)
                            .ok_or("quadratic support did not split")?;
                        self.0.element(0, t.a)
                    }
                };
                let half = self.inv(self.e(2));
                let m = self.neg(u[1]);
                Ok(vec![
                    self.mul(self.add(m, s), half),
                    self.mul(self.sub(m, s), half),
                ])
            }
            _ => Err("support degree exceeds genus two"),
        }
    }
    fn chart(self, a: &[E], root: E) -> Poly {
        let mut out = vec![self.z(); 7];
        let mut power = self.base(&[1]);
        for (i, c) in a.iter().enumerate() {
            for (j, z) in power.iter().enumerate() {
                out[j + 6 - i] = self.add(out[j + 6 - i], self.mul(*c, *z));
            }
            power = self.times(&power, &[self.o(), root]);
        }
        self.trim(out)
    }
    fn chart_pair(self, u: &[E], v: &[E], f: &[E], root: E) -> Result<Pair> {
        if u.len() != 3 {
            return Err("quadratic fiber required");
        }
        let mut uu = vec![self.z(); 3];
        let mut power = self.base(&[1]);
        for (i, c) in u.iter().enumerate() {
            for (j, z) in power.iter().enumerate() {
                uu[j + 2 - i] = self.add(uu[j + 2 - i], self.mul(*c, *z));
            }
            power = self.times(&power, &[self.o(), root]);
        }
        let mut vv = vec![self.z(); 4];
        let mut power = self.base(&[1]);
        for (i, c) in v.iter().enumerate() {
            for (j, z) in power.iter().enumerate() {
                vv[j + 3 - i] = self.add(vv[j + 3 - i], self.mul(*c, *z));
            }
            power = self.times(&power, &[self.o(), root]);
        }
        self.pair(uu, vv, f)
    }
    fn pair(self, mut u: Poly, mut v: Poly, f: &[E]) -> Result<Pair> {
        u = self.trim(u);
        if u.is_empty() {
            return Err("zero u");
        }
        u = self.monic(&u);
        v = self.divide(&v, &u).1;
        if !self
            .divide(&self.minus(&self.times(&v, &v), f), &u)
            .1
            .is_empty()
        {
            return Err("curve congruence failed");
        }
        while u.len() > 3 {
            u = self.monic(&self.exact(&self.minus(f, &self.times(&v, &v)), &u));
            v = self.divide(&self.negative(&v), &u).1;
        }
        Ok(Pair { u, v })
    }
    fn zero_pair(self) -> Pair {
        Pair {
            u: self.base(&[1]),
            v: vec![],
        }
    }
    fn neg_pair(self, a: &Pair) -> Pair {
        Pair {
            u: a.u.clone(),
            v: self.negative(&a.v),
        }
    }
    fn add_pair(self, a: &Pair, b: &Pair, f: &[E]) -> Pair {
        let (d1, s, t) = self.gcd(a.u.clone(), b.u.clone());
        let (d, c, e) = self.gcd(d1, self.plus(&a.v, &b.v));
        let u = self.exact(&self.times(&a.u, &b.u), &self.times(&d, &d));
        let v = self.exact(
            &self.plus(
                &self.plus(
                    &self.times(&self.times(&self.times(&s, &c), &a.u), &b.v),
                    &self.times(&self.times(&self.times(&t, &c), &b.u), &a.v),
                ),
                &self.times(&e, &self.plus(&self.times(&a.v, &b.v), f)),
            ),
            &d,
        );
        self.pair(u, v, f).expect("valid Cantor composition")
    }
    fn mul_pair(self, a: &Pair, mut n: u64, f: &[E]) -> Pair {
        let mut a = a.clone();
        let mut out = self.zero_pair();
        while n != 0 {
            if n & 1 != 0 {
                out = self.add_pair(&out, &a, f);
            }
            a = self.add_pair(&a, &a, f);
            n >>= 1;
        }
        out
    }
    fn point_class(self, z: E, w: E, f: &[E], root: E) -> Result<Pair> {
        if z == root {
            return Err("target base must be removed as a divisor");
        }
        let t = self.inv(self.sub(z, root));
        self.pair(
            vec![self.neg(t), self.o()],
            vec![self.mul(w, self.pow(t, 3))],
            f,
        )
    }
    fn fiber(
        self,
        t: E,
        y_cap: E,
        f: &[E],
        g: &[Poly; 3],
        h: &[Poly; 3],
        bases: (E, E),
    ) -> Result<Pair> {
        let (sr, tr) = bases;
        if t == self.z() {
            let l1 = *g[0].last().unwrap();
            let l2 = *g[1].last().unwrap();
            let u = self.monic(&self.plus(&self.times(&h[0], &[l1]), &self.times(&h[1], &[l2])));
            let v = self
                .divide(&self.times(&h[0], &[self.mul(l1, self.inv(y_cap))]), &u)
                .1;
            return self.chart_pair(&u, &v, f, tr);
        }
        let x = self.add(sr, self.inv(t));
        let y = self.mul(y_cap, self.inv(self.pow(t, 3)));
        let g1 = self.value(&g[0], x);
        let g2 = self.value(&g[1], x);
        if y == self.z() {
            if g1 == self.z() {
                return self.chart_pair(&h[1], &[], f, tr);
            }
            if g2 == self.z() {
                return self.chart_pair(&h[0], &[], f, tr);
            }
            if self.value(&g[2], x) == self.z() {
                return Ok(self.zero_pair());
            }
            return Err("zero y outside branch locus");
        }
        let u = self.plus(&self.times(&h[0], &[g1]), &self.times(&h[1], &[g2]));
        if u.len() == 2 {
            let z = self.mul(self.neg(u[0]), self.inv(u[1]));
            let w = self.mul(
                self.mul(self.mul(g1, self.value(&h[0], z)), self.sub(x, z)),
                self.inv(y),
            );
            let finite = self.point_class(z, w, f, tr)?;
            let iy = self.neg(self.mul(self.mul(g1, *h[0].last().unwrap()), self.inv(y)));
            let infinity = self.pair(self.base(&[0, 1]), vec![iy], f)?;
            return Ok(self.add_pair(&finite, &infinity, f));
        }
        if u.len() != 3 {
            return Err("unexpected correspondence fiber dimension");
        }
        let u = self.monic(&u);
        let v = self
            .divide(
                &self.times(
                    &self.times(&h[0], &[self.mul(g1, self.inv(y))]),
                    &[x, self.e(-1)],
                ),
                &u,
            )
            .1;
        self.chart_pair(&u, &v, f, tr)
    }
    fn evaluate(
        self,
        d: &Pair,
        f: &[E],
        g: &[Poly; 3],
        h: &[Poly; 3],
        sr: E,
        tr: E,
    ) -> Result<Pair> {
        let mut result = self.zero_pair();
        for t in self.roots(&d.u)? {
            result = self.add_pair(
                &result,
                &self.fiber(t, self.value(&d.v, t), f, g, h, (sr, tr))?,
                f,
            );
        }
        let base = self.chart_pair(&h[1], &[], f, tr)?;
        result = self.add_pair(
            &result,
            &self.neg_pair(&self.mul_pair(&base, (d.u.len() - 1) as u64, f)),
            f,
        );
        self.rational(&result.u)?;
        self.rational(&result.v)?;
        Ok(result)
    }
    fn ec_on(self, q: ExtendedPoint, a: E, b: E, c: E) -> bool {
        q.is_none_or(|(x, y)| {
            self.mul(y, y)
                == self.add(
                    self.add(
                        self.add(self.pow(x, 3), self.mul(a, self.pow(x, 2))),
                        self.mul(b, x),
                    ),
                    c,
                )
        })
    }
    fn ec_add(self, p: ExtendedPoint, q: ExtendedPoint, a: E, b: E) -> ExtendedPoint {
        let ((x, y), (u, v)) = match (p, q) {
            (None, _) => return q,
            (_, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        if x == u && self.add(y, v) == self.z() {
            return None;
        }
        let slope = if x == u {
            self.mul(
                self.add(
                    self.add(
                        self.mul(self.e(3), self.pow(x, 2)),
                        self.mul(self.e(2), self.mul(a, x)),
                    ),
                    b,
                ),
                self.inv(self.mul(self.e(2), y)),
            )
        } else {
            self.mul(self.sub(v, y), self.inv(self.sub(u, x)))
        };
        let xx = self.sub(self.sub(self.sub(self.mul(slope, slope), a), x), u);
        Some((xx, self.sub(self.mul(slope, self.sub(x, xx)), y)))
    }
    fn ec_mul(self, p: ExtendedPoint, mut n: u64, a: E, b: E) -> ExtendedPoint {
        let mut p = p;
        let mut out = None;
        while n != 0 {
            if n & 1 != 0 {
                out = self.ec_add(out, p, a, b);
            }
            p = self.ec_add(p, p, a, b);
            n >>= 1;
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pair {
    u: Poly,
    v: Poly,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    First,
    Second,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    field: Field,
    sextic: Poly,
    root: E,
    f: Poly,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    model: Model,
    d: Pair,
}

impl Model {
    fn new(field: Field, sextic: Poly, root: E) -> Result<Self> {
        let s = field;
        if sextic.len() != 7 {
            return Err("degree-six model required");
        }
        if s.gcd(sextic.clone(), s.derivative(&sextic)).0 != s.base(&[1]) {
            return Err("singular sextic");
        }
        if s.value(&sextic, root) != s.z() {
            return Err("base is not Weierstrass");
        }
        let f = s.chart(&sextic, root);
        if f.len() != 6 {
            return Err("degree-five chart required");
        }
        Ok(Self {
            field,
            sextic,
            root,
            f,
        })
    }
    pub fn from_sextic(prime: u64, sextic: Vec<i64>, root: i64) -> Result<Self> {
        let f = Field::new(prime)?;
        Self::new(f, f.base(&sextic), f.e(root))
    }
    pub fn prime(&self) -> u64 {
        self.field.p()
    }
    pub fn nonresidue(&self) -> u64 {
        self.field.0.nonresidue()
    }
    pub fn root(&self) -> u64 {
        self.root.a
    }
    pub fn sextic(&self) -> Vec<u64> {
        self.field.rational(&self.sextic).unwrap()
    }
    pub fn quintic(&self) -> Vec<u64> {
        self.field.rational(&self.f).unwrap()
    }
    pub fn class(&self, u: Vec<i64>, v: Vec<i64>) -> Result<Class> {
        let s = self.field;
        Ok(Class {
            model: self.clone(),
            d: s.pair(s.base(&u), s.base(&v), &self.f)?,
        })
    }
    pub fn zero(&self) -> Class {
        Class {
            model: self.clone(),
            d: self.field.zero_pair(),
        }
    }
    pub fn classes(&self) -> Result<Vec<Class>> {
        if self.prime() > MAX_ENUMERATION_PRIME {
            return Err("exhaustive enumeration capped at101");
        }
        let s = self.field;
        let p = s.p();
        let mut out = vec![self.zero()];
        for t in 0..p {
            for y in 0..p {
                if s.pow(s.e(y as i64), 2) == s.value(&self.f, s.e(t as i64)) {
                    out.push(self.class(vec![-(t as i64), 1], vec![y as i64])?);
                }
            }
        }
        for b in 0..p {
            for c in 0..p {
                let u = s.base(&[c as i64, b as i64, 1]);
                let rem = s.divide(&self.f, &u).1;
                let f0 = *rem.first().unwrap_or(&s.z());
                let f1 = *rem.get(1).unwrap_or(&s.z());
                for v1 in 0..p {
                    if v1 == 0 {
                        if f1 == s.z() {
                            for v0 in 0..p {
                                if s.pow(s.e(v0 as i64), 2) == f0 {
                                    out.push(
                                        self.class(vec![c as i64, b as i64, 1], vec![v0 as i64])?,
                                    );
                                }
                            }
                        }
                    } else {
                        let v1 = s.e(v1 as i64);
                        let v0 = s.mul(
                            s.add(f1, s.mul(s.e(b as i64), s.pow(v1, 2))),
                            s.inv(s.mul(s.e(2), v1)),
                        );
                        if s.sub(s.pow(v0, 2), s.mul(s.e(c as i64), s.pow(v1, 2))) == f0 {
                            out.push(self.class(
                                vec![c as i64, b as i64, 1],
                                vec![v0.a as i64, v1.a as i64],
                            )?);
                        }
                    }
                }
            }
        }
        Ok(out)
    }
    fn ec_coefficients(&self, q: Projection) -> Result<(E, E, E, E)> {
        let s = self.field;
        if [1, 3, 5].iter().any(|i| self.sextic[*i] != s.z()) {
            return Err("model is not bielliptic-even");
        }
        let (l, a, b, c) = match q {
            Projection::First => (
                self.sextic[6],
                self.sextic[4],
                self.sextic[2],
                self.sextic[0],
            ),
            Projection::Second => (
                self.sextic[0],
                self.sextic[2],
                self.sextic[4],
                self.sextic[6],
            ),
        };
        if l == s.z() {
            return Err("singular quotient");
        }
        Ok((l, a, s.mul(l, b), s.mul(s.pow(l, 2), c)))
    }
    pub fn elliptic_coefficients(&self, q: Projection) -> Result<(u64, u64, u64)> {
        let (_, a, b, c) = self.ec_coefficients(q)?;
        Ok((a.a, b.a, c.a))
    }
    fn point(&self, q: Point) -> Result<ExtendedPoint> {
        if q.is_some_and(|(x, y)| x >= self.prime() || y >= self.prime()) {
            return Err("noncanonical point");
        }
        Ok(q.map(|(x, y)| (self.field.e(x as i64), self.field.e(y as i64))))
    }
    fn descend_point(&self, q: ExtendedPoint) -> Result<Point> {
        if q.is_some_and(|(x, y)| x.b != 0 || y.b != 0) {
            return Err("point did not descend");
        }
        Ok(q.map(|(x, y)| (x.a, y.a)))
    }
    pub fn elliptic_points(&self, q: Projection) -> Result<Vec<Point>> {
        if self.prime() > MAX_ENUMERATION_PRIME {
            return Err("point enumeration capped at101");
        }
        let (_, a, b, c) = self.ec_coefficients(q)?;
        let s = self.field;
        let mut out = vec![None];
        for x in 0..s.p() {
            for y in 0..s.p() {
                let q = Some((s.e(x as i64), s.e(y as i64)));
                if s.ec_on(q, a, b, c) {
                    out.push(Some((x, y)));
                }
            }
        }
        Ok(out)
    }
    pub fn elliptic_mul(&self, q: Point, n: u64, projection: Projection) -> Result<Point> {
        let (_, a, b, c) = self.ec_coefficients(projection)?;
        let q = self.point(q)?;
        if !self.field.ec_on(q, a, b, c) {
            return Err("point outside quotient");
        }
        self.descend_point(self.field.ec_mul(q, n, a, b))
    }
    pub fn pull(&self, q: Point) -> Result<Class> {
        let (l, a, b, c) = self.ec_coefficients(Projection::First)?;
        let q = self.point(q)?;
        let s = self.field;
        if !s.ec_on(q, a, b, c) {
            return Err("point outside quotient");
        }
        let (x, y) = match q {
            None => return Ok(self.zero()),
            Some(q) => q,
        };
        let u = s.trim(vec![
            s.o(),
            s.mul(s.e(2), self.root),
            s.sub(s.pow(self.root, 2), s.mul(x, s.inv(l))),
        ]);
        let v = s.divide(&[s.z(), s.z(), s.z(), s.mul(y, s.inv(l))], &u).1;
        Ok(Class {
            model: self.clone(),
            d: s.pair(u, v, &self.f)?,
        })
    }
    pub fn norm(&self, d: &Class, q: Projection) -> Result<Point> {
        if d.model != *self {
            return Err("field/curve/chart mismatch");
        }
        let (l, a, b, c) = self.ec_coefficients(q)?;
        let s = self.field;
        let mut out = None;
        for t in s.roots(&d.d.u)? {
            let yy = s.value(&d.d.v, t);
            let image = if t == s.z() {
                match q {
                    Projection::First => None,
                    Projection::Second => Some((s.z(), s.mul(l, yy))),
                }
            } else {
                let x = s.add(self.root, s.inv(t));
                let y = s.mul(yy, s.inv(s.pow(t, 3)));
                match q {
                    Projection::First => Some((s.mul(l, s.pow(x, 2)), s.mul(l, y))),
                    Projection::Second => {
                        if x == s.z() {
                            None
                        } else {
                            Some((
                                s.mul(l, s.inv(s.pow(x, 2))),
                                s.mul(s.mul(l, y), s.inv(s.pow(x, 3))),
                            ))
                        }
                    }
                }
            };
            if !s.ec_on(image, a, b, c) {
                return Err("norm support outside quotient");
            }
            out = s.ec_add(out, image, a, b);
        }
        if (d.d.u.len() - 1) & 1 != 0 {
            let x = match q {
                Projection::First => s.pow(self.root, 2),
                Projection::Second => s.inv(s.pow(self.root, 2)),
            };
            out = s.ec_add(out, Some((s.mul(l, x), s.z())), a, b);
        }
        self.descend_point(out)
    }
}
impl Class {
    pub fn model(&self) -> &Model {
        &self.model
    }
    pub fn coefficients(&self) -> (Vec<u64>, Vec<u64>) {
        (
            self.model.field.rational(&self.d.u).unwrap(),
            self.model.field.rational(&self.d.v).unwrap(),
        )
    }
    pub fn add(&self, b: &Self) -> Result<Self> {
        if self.model != b.model {
            return Err("field/curve/chart mismatch");
        }
        Ok(Self {
            model: self.model.clone(),
            d: self.model.field.add_pair(&self.d, &b.d, &self.model.f),
        })
    }
    pub fn neg(&self) -> Self {
        Self {
            model: self.model.clone(),
            d: self.model.field.neg_pair(&self.d),
        }
    }
    pub fn mul(&self, n: u64) -> Self {
        Self {
            model: self.model.clone(),
            d: self.model.field.mul_pair(&self.d, n, &self.model.f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Richelot {
    source: Model,
    target: Model,
    g: [Poly; 3],
    h: [Poly; 3],
    delta: u64,
}
impl Richelot {
    pub fn new(
        prime: u64,
        factors: [[i64; 3]; 3],
        source_root: i64,
        target_root: i64,
    ) -> Result<Self> {
        let s = Field::new(prime)?;
        let g = factors.map(|p| s.base(&p));
        if g.iter().any(|p| p.len() != 3) {
            return Err("three quadratic factors required");
        }
        let mut d = s.z();
        for i in 0..3 {
            d = s.add(
                d,
                s.mul(s.mul(g[0][i], g[1][(i + 1) % 3]), g[2][(i + 2) % 3]),
            );
            d = s.sub(
                d,
                s.mul(s.mul(g[0][i], g[1][(i + 2) % 3]), g[2][(i + 1) % 3]),
            );
        }
        if d == s.z() {
            return Err("degenerate Richelot splitting");
        }
        let h: [Poly; 3] = std::array::from_fn(|i| {
            let a = &g[(i + 1) % 3];
            let b = &g[(i + 2) % 3];
            s.times(
                &s.minus(&s.times(&s.derivative(a), b), &s.times(a, &s.derivative(b))),
                &[s.inv(d)],
            )
        });
        let source = Model::new(s, s.times(&s.times(&g[0], &g[1]), &g[2]), s.e(source_root))?;
        let target = Model::new(s, s.times(&s.times(&h[0], &h[1]), &h[2]), s.e(target_root))?;
        if s.value(&g[0], source.root) != s.z() || s.value(&h[0], target.root) != s.z() {
            return Err("bases must lie in first factors");
        }
        Ok(Self {
            source,
            target,
            g,
            h,
            delta: d.a,
        })
    }
    pub fn source(&self) -> &Model {
        &self.source
    }
    pub fn target(&self) -> &Model {
        &self.target
    }
    pub fn determinant(&self) -> u64 {
        self.delta
    }
    pub fn transfer(&self, d: &Class) -> Result<Class> {
        if d.model != self.source {
            return Err("source field/curve/chart mismatch");
        }
        let s = self.source.field;
        let image = s.evaluate(
            &d.d,
            &self.target.f,
            &self.g,
            &self.h,
            self.source.root,
            self.target.root,
        )?;
        Ok(Class {
            model: self.target.clone(),
            d: image,
        })
    }
    pub fn dual(&self, d: &Class) -> Result<Class> {
        if d.model != self.target {
            return Err("target field/curve/chart mismatch");
        }
        let s = self.source.field;
        let image = s.evaluate(
            &d.d,
            &self.source.f,
            &self.h,
            &self.g,
            self.target.root,
            self.source.root,
        )?;
        Ok(Class {
            model: self.source.clone(),
            d: s.neg_pair(&image),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_fields_complete_domains_and_source_norm() {
        for prime in [13, 29, 31, 101] {
            let r = Richelot::new(prime, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
            let source = r.source.classes().unwrap();
            let target = r.target.classes().unwrap();
            assert_eq!(source.len(), target.len());
            let mut fibers = std::collections::BTreeMap::new();
            let h = &source[11];
            let ih = r.transfer(h).unwrap();
            for d in &source {
                let image = r.transfer(d).unwrap();
                *fibers.entry(image.coefficients()).or_insert(0_usize) += 1;
                assert_eq!(r.dual(&image).unwrap(), d.mul(2));
                assert_eq!(
                    r.transfer(&d.add(h).unwrap()).unwrap(),
                    image.add(&ih).unwrap()
                );
            }
            assert_eq!(fibers.len() * 4, source.len());
            assert!(fibers.values().all(|n| *n == 4));
            for d in &target {
                assert_eq!(r.transfer(&r.dual(d).unwrap()).unwrap(), d.mul(2));
            }
            let points = r.source.elliptic_points(Projection::First).unwrap();
            for q in &points {
                let d = r.source.pull(*q).unwrap();
                assert_eq!(
                    r.source.norm(&d, Projection::First).unwrap(),
                    r.source.elliptic_mul(*q, 2, Projection::First).unwrap()
                );
                assert_eq!(r.source.norm(&d, Projection::Second).unwrap(), None);
            }
            let order = points.len() as u64;
            let prime_order = (3..=order)
                .rev()
                .find(|v| order.is_multiple_of(*v) && (2..*v).all(|d| !v.is_multiple_of(d)))
                .unwrap();
            let mut cofactor = order;
            let mut depth = 0;
            while cofactor.is_multiple_of(prime_order) {
                cofactor /= prime_order;
                depth += 1;
            }
            let p = Some(
                points
                    .iter()
                    .find_map(|q| {
                        let mut current = r
                            .source
                            .elliptic_mul(*q, cofactor, Projection::First)
                            .unwrap();
                        for _ in 0..depth {
                            current?;
                            let next = r
                                .source
                                .elliptic_mul(current, prime_order, Projection::First)
                                .unwrap();
                            if next.is_none() {
                                return current;
                            }
                            current = next;
                        }
                        None
                    })
                    .unwrap(),
            );
            let image = r.transfer(&r.source.pull(p).unwrap()).unwrap();
            assert_ne!(image, r.target.zero());
            assert_eq!(image.mul(prime_order), r.target.zero());
            for k in 0..prime_order {
                let q = r.source.elliptic_mul(p, k, Projection::First).unwrap();
                assert_eq!(
                    r.transfer(&r.source.pull(q).unwrap()).unwrap(),
                    image.mul(k)
                );
            }
            println!(
                "p={prime}, selected odd subgroup r={prime_order}, P={p:?}, target class={:?}",
                image.coefficients()
            );
            let two = |m: &Model, ds: &[Class]| ds.iter().filter(|d| d.mul(2) == m.zero()).count();
            println!(
                "p={prime}, nonresidue={}, #J={}, rational2-torsion={}/{}, #E={}",
                r.source.nonresidue(),
                source.len(),
                two(&r.source, &source),
                two(&r.target, &target),
                points.len()
            );
        }
    }
    #[test]
    fn field_bounds_domains_and_bad_models_fail_closed() {
        for prime in [0, 1, 2, 9, 15, 4099, u64::MAX] {
            assert!(Richelot::new(prime, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).is_err());
        }
        assert!(Richelot::new(5, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).is_err());
        let a = Richelot::new(13, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        let b = Richelot::new(29, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        assert!(a.source.zero().add(&b.source.zero()).is_err());
        assert!(a.transfer(&a.target.zero()).is_err());
        assert!(a.transfer(&b.source.zero()).is_err());
        assert!(a.source.class(vec![], vec![]).is_err());
        assert!(a.source.pull(Some((13, 0))).is_err());
        let large = Richelot::new(109, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
        assert!(large.source.classes().is_err());
        assert!(large.source.elliptic_points(Projection::First).is_err());
        assert!(Model::from_sextic(13, vec![0, 0, 0, 0, 0, 0, 1], 0).is_err());
    }

    #[test]
    fn successor_shortcut_on_all_curve_points_and_whole_fibers() {
        for prime in [13, 29, 31, 101] {
            let r = Richelot::new(prime, [[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2).unwrap();
            let s = r.source.field;
            let scale = s.inv(s.e(500));
            let (_, a, b, c) = r.target.ec_coefficients(Projection::First).unwrap();
            assert_eq!(
                (a, b, c),
                (
                    s.mul(s.e(-102), scale),
                    s.mul(s.e(393), s.pow(scale, 2)),
                    s.mul(s.e(-4), s.pow(scale, 3))
                )
            );
            let direct = |q: ExtendedPoint| {
                q.and_then(|(x, y)| {
                    let u = s.sub(x, s.o());
                    if u == s.z() {
                        return None;
                    }
                    let xx = s.add(s.add(u, s.e(9)), s.mul(s.e(24), s.inv(u)));
                    let yy = s.mul(y, s.sub(s.o(), s.mul(s.e(24), s.inv(s.pow(u, 2)))));
                    Some((
                        s.add(s.mul(xx, s.inv(s.e(100))), s.inv(s.e(125))),
                        s.neg(s.mul(yy, s.inv(s.e(1000)))),
                    ))
                })
            };
            // The norm of the actual base correspondence fiber is the
            // target rational2-torsion T=(1/125,0), not the identity.
            let base_image = Class {
                model: r.target.clone(),
                d: s.chart_pair(&r.h[1], &[], &r.target.f, r.target.root)
                    .unwrap(),
            };
            assert_eq!(
                r.target.norm(&base_image, Projection::First).unwrap(),
                Some((s.inv(s.e(125)).a, 0))
            );
            let mut curve_points = 0;
            for x in 0..prime {
                for y in 0..prime {
                    let (x, y) = (s.e(x as i64), s.e(y as i64));
                    if s.pow(y, 2) != s.value(&r.source.sextic, x) {
                        continue;
                    }
                    curve_points += 1;
                    let d = if x == r.source.root {
                        r.source.zero()
                    } else {
                        let t = s.inv(s.sub(x, r.source.root));
                        r.source
                            .class(
                                vec![s.neg(t).a as i64, 1],
                                vec![s.mul(y, s.pow(t, 3)).a as i64],
                            )
                            .unwrap()
                    };
                    let predicted = direct(Some((s.pow(x, 2), y)));
                    assert!(s.ec_on(predicted, a, b, c));
                    assert_eq!(
                        r.target
                            .norm(&r.transfer(&d).unwrap(), Projection::First)
                            .unwrap(),
                        r.target.descend_point(predicted).unwrap()
                    );
                }
            }
            for sign in [-1, 1] {
                let infinity = r.source.class(vec![0, 1], vec![sign]).unwrap();
                assert_eq!(
                    r.target
                        .norm(&r.transfer(&infinity).unwrap(), Projection::First)
                        .unwrap(),
                    None
                );
            }
            for q in r.source.elliptic_points(Projection::First).unwrap() {
                let image = r.transfer(&r.source.pull(q).unwrap()).unwrap();
                let doubled = r.source.elliptic_mul(q, 2, Projection::First).unwrap();
                let predicted = direct(r.source.point(doubled).unwrap());
                assert_eq!(
                    r.target.norm(&image, Projection::First).unwrap(),
                    r.target.descend_point(predicted).unwrap()
                );
                assert_eq!(r.target.norm(&image, Projection::Second).unwrap(), None);
            }
            println!("PASS p={prime} successor rational shortcut: {curve_points} affine curve points plus both infinities, actual base norm(1/125,0), all whole elliptic fibers");
        }
    }

    #[cfg(feature = "small-prime")]
    #[test]
    fn both_f101_hops_match_the_frozen_model_backend() {
        use super::super::richelot::model::Richelot as Fixed;
        for (factors, sr, tr) in [
            ([[-1, 0, 1], [4, 1, 1], [4, -1, 1]], 1, 2),
            ([[36, -20, 1], [-56, -26, 1], [10080, 920, 20]], 2, 11),
        ] {
            let small = Richelot::new(101, factors, sr, tr).unwrap();
            let fixed = Fixed::new(factors, sr, tr).unwrap();
            let coefficients = |(u, v): (Vec<i64>, Vec<i64>)| {
                (
                    u.into_iter().map(|x| x as u64).collect::<Vec<_>>(),
                    v.into_iter().map(|x| x as u64).collect::<Vec<_>>(),
                )
            };
            for d in fixed.source().classes() {
                let (u, v) = d.coefficients();
                let sd = small.source.class(u, v).unwrap();
                assert_eq!(
                    small.transfer(&sd).unwrap().coefficients(),
                    coefficients(fixed.transfer(&d).unwrap().coefficients())
                );
                assert_eq!(
                    sd.mul(2).coefficients(),
                    coefficients(d.mul(2).coefficients())
                );
            }
            for d in fixed.target().classes() {
                let (u, v) = d.coefficients();
                let sd = small.target.class(u, v).unwrap();
                assert_eq!(
                    small.dual(&sd).unwrap().coefficients(),
                    coefficients(fixed.dual(&d).unwrap().coefficients())
                );
            }
        }
    }
}
