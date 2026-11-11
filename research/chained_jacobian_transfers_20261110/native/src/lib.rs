//! Fixed public F_101 fixture, not a general-purpose cryptographic library.
//!
//! C: y²=x⁶+1, W=(10,0), t=1/(x-10), Y=y*t³. The odd quintic model
//! uses W as its unique infinity. Mumford (u,v) means [D-deg(u)W].
//! Norms split degree-two closed support in F_101[s]/(s²-2), retain
//! multiplicity, and explicitly handle both original infinities (t=0).
//! Pullback takes a whole fiber, including nonsplit and ramified fibers;
//! it does not choose square roots of an elliptic abscissa.

pub const P: i64 = 101;
pub const ROOT: i64 = 10;
pub type Poly = Vec<i64>;

pub mod richelot;

pub fn fp(a: i64) -> i64 {
    a.rem_euclid(P)
}
pub fn pow(mut a: i64, mut n: u64) -> i64 {
    a = fp(a);
    let mut b = 1;
    while n != 0 {
        if n & 1 != 0 {
            b = fp(b * a);
        }
        a = fp(a * a);
        n >>= 1;
    }
    b
}
pub fn inv(a: i64) -> i64 {
    assert_ne!(fp(a), 0, "inverse of zero");
    pow(a, 99)
}

pub fn trim(mut a: Poly) -> Poly {
    for x in &mut a {
        *x = fp(*x);
    }
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
pub fn padd(a: &[i64], b: &[i64]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0))
            .collect(),
    )
}
pub fn pneg(a: &[i64]) -> Poly {
    trim(a.iter().map(|x| -x).collect())
}
pub fn psub(a: &[i64], b: &[i64]) -> Poly {
    padd(a, &pneg(b))
}
pub fn pmul(a: &[i64], b: &[i64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = fp(c[i + j] + x * y);
        }
    }
    trim(c)
}
pub fn divrem(a: &[i64], b: &[i64]) -> (Poly, Poly) {
    let mut a = trim(a.to_vec());
    let b = trim(b.to_vec());
    assert!(!b.is_empty());
    let mut q = vec![0; (a.len() + 1).saturating_sub(b.len())];
    let lead_inv = inv(*b.last().unwrap());
    while !a.is_empty() && a.len() >= b.len() {
        let k = a.len() - b.len();
        let c = fp(a[a.len() - 1] * lead_inv);
        q[k] = c;
        for (j, bj) in b.iter().enumerate() {
            a[k + j] = fp(a[k + j] - c * bj);
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn exact(a: &[i64], b: &[i64]) -> Poly {
    let (q, r) = divrem(a, b);
    assert!(r.is_empty(), "nonexact polynomial division");
    q
}
fn monic(a: &[i64]) -> Poly {
    assert!(!a.is_empty());
    pmul(a, &[inv(*a.last().unwrap())])
}
fn xgcd(mut a: Poly, mut b: Poly) -> (Poly, Poly, Poly) {
    let (mut s0, mut s1, mut t0, mut t1) = (vec![1], vec![], vec![], vec![1]);
    while !b.is_empty() {
        let (q, r) = divrem(&a, &b);
        a = b;
        b = r;
        (s0, s1) = (s1.clone(), psub(&s0, &pmul(&q, &s1)));
        (t0, t1) = (t1.clone(), psub(&t0, &pmul(&q, &t1)));
    }
    let z = [inv(*a.last().unwrap())];
    (pmul(&a, &z), pmul(&s0, &z), pmul(&t0, &z))
}

/// t^6*((10+1/t)^6+1); its degree-six coefficient vanishes.
pub fn quintic() -> Poly {
    [1, 6, 15, 20, 15, 6]
        .iter()
        .enumerate()
        .map(|(i, c)| fp(c * pow(ROOT, i as u64)))
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Divisor {
    pub u: Poly,
    pub v: Poly,
}

impl Divisor {
    pub fn zero() -> Self {
        Self {
            u: vec![1],
            v: vec![],
        }
    }
    /// Canonicalizes a semireduced pair. Rejects zero u and a failed curve congruence.
    pub fn new(u: Poly, v: Poly) -> Result<Self, &'static str> {
        let mut u = trim(u);
        if u.is_empty() {
            return Err("zero u");
        }
        u = monic(&u);
        let mut v = divrem(&v, &u).1;
        if !divrem(&psub(&pmul(&v, &v), &quintic()), &u).1.is_empty() {
            return Err("u does not divide v^2-F");
        }
        while u.len() > 3 {
            u = monic(&exact(&psub(&quintic(), &pmul(&v, &v)), &u));
            v = divrem(&pneg(&v), &u).1;
        }
        Ok(Self { u, v })
    }
    pub fn valid(&self) -> bool {
        self.u.last() == Some(&1)
            && self.u.len() <= 3
            && self.v.len() < self.u.len()
            && trim(self.u.clone()) == self.u
            && trim(self.v.clone()) == self.v
            && divrem(&psub(&pmul(&self.v, &self.v), &quintic()), &self.u)
                .1
                .is_empty()
    }
    pub fn neg(&self) -> Self {
        Self::new(self.u.clone(), pneg(&self.v)).unwrap()
    }
    pub fn add(&self, rhs: &Self) -> Self {
        assert!(self.valid() && rhs.valid());
        let (d1, a, b) = xgcd(self.u.clone(), rhs.u.clone());
        let (d, c, e) = xgcd(d1, padd(&self.v, &rhs.v));
        let u = exact(&pmul(&self.u, &rhs.u), &pmul(&d, &d));
        let numerator = padd(
            &padd(
                &pmul(&pmul(&pmul(&a, &c), &self.u), &rhs.v),
                &pmul(&pmul(&pmul(&b, &c), &rhs.u), &self.v),
            ),
            &pmul(&e, &padd(&pmul(&self.v, &rhs.v), &quintic())),
        );
        Self::new(u, exact(&numerator, &d)).unwrap()
    }
    pub fn mul(&self, mut n: u64) -> Self {
        let mut a = self.clone();
        let mut b = Self::zero();
        while n != 0 {
            if n & 1 != 0 {
                b = b.add(&a);
            }
            a = a.add(&a);
            n >>= 1;
        }
        b
    }
}

/// a+b*s, s²=2 (2 is a nonsquare modulo 101).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F2(pub i64, pub i64);
impl F2 {
    pub const ZERO: Self = Self(0, 0);
    pub const ONE: Self = Self(1, 0);
    pub fn base(a: i64) -> Self {
        Self(fp(a), 0)
    }
    pub fn plus(self, b: Self) -> Self {
        Self(fp(self.0 + b.0), fp(self.1 + b.1))
    }
    pub fn minus(self, b: Self) -> Self {
        self.plus(b.negative())
    }
    pub fn negative(self) -> Self {
        Self(fp(-self.0), fp(-self.1))
    }
    pub fn times(self, b: Self) -> Self {
        Self(
            fp(self.0 * b.0 + 2 * self.1 * b.1),
            fp(self.0 * b.1 + self.1 * b.0),
        )
    }
    pub fn inverse(self) -> Self {
        let z = inv(self.0 * self.0 - 2 * self.1 * self.1);
        Self(fp(self.0 * z), fp(-self.1 * z))
    }
    pub fn power(self, mut n: u64) -> Self {
        let mut a = self;
        let mut b = Self::ONE;
        while n != 0 {
            if n & 1 != 0 {
                b = b.times(a);
            }
            a = a.times(a);
            n >>= 1;
        }
        b
    }
    pub fn conjugate(self) -> Self {
        Self(self.0, fp(-self.1))
    }
}
pub type Point = Option<(F2, F2)>;
pub fn point(x: i64, y: i64) -> Point {
    Some((F2::base(x), F2::base(y)))
}
fn eval(a: &[i64], t: F2) -> F2 {
    a.iter()
        .rev()
        .fold(F2::ZERO, |s, c| s.times(t).plus(F2::base(*c)))
}
pub fn ec_on(q: Point, a2: i64, a4: i64, a6: i64) -> bool {
    match q {
        None => true,
        Some((x, y)) => {
            y.times(y)
                == x.power(3)
                    .plus(F2::base(a2).times(x.power(2)))
                    .plus(F2::base(a4).times(x))
                    .plus(F2::base(a6))
        }
    }
}
pub fn ec_add(q: Point, r: Point, a2: i64, a4: i64) -> Point {
    let (x, y) = match q {
        None => return r,
        Some(z) => z,
    };
    let (u, v) = match r {
        None => return q,
        Some(z) => z,
    };
    if x == u && y.plus(v) == F2::ZERO {
        return None;
    }
    let m = if x == u {
        F2::base(3)
            .times(x.power(2))
            .plus(F2::base(2 * a2).times(x))
            .plus(F2::base(a4))
            .times(F2::base(2).times(y).inverse())
    } else {
        v.minus(y).times(u.minus(x).inverse())
    };
    let z = m.power(2).minus(F2::base(a2)).minus(x).minus(u);
    Some((z, m.times(x.minus(z)).minus(y)))
}
pub fn ec_mul(mut q: Point, mut n: u64, a2: i64, a4: i64) -> Point {
    let mut r = None;
    while n != 0 {
        if n & 1 != 0 {
            r = ec_add(r, q, a2, a4);
        }
        q = ec_add(q, q, a2, a4);
        n >>= 1;
    }
    r
}
pub fn ec_points() -> Vec<Point> {
    let mut out = vec![None];
    for x in 0..P {
        for y in 0..P {
            let q = point(x, y);
            if ec_on(q, 0, 0, 1) {
                out.push(q);
            }
        }
    }
    out
}

/// Whole-fiber pullback f*: E(F101) -> J(C)(F101), with O mapped to zero.
pub fn pull(q: Point) -> Result<Divisor, &'static str> {
    if !ec_on(q, 0, 0, 1) {
        return Err("point outside E");
    }
    let (x, y) = match q {
        None => return Ok(Divisor::zero()),
        Some(z) => z,
    };
    if x.1 != 0 || y.1 != 0 {
        return Err("pullback domain is E(F101)");
    }
    // The degree drops at x=ROOT²: one fiber point is W, the chosen infinity.
    let u = trim(vec![1, 2 * ROOT, fp(ROOT * ROOT - x.0)]);
    let v = divrem(&[0, 0, 0, y.0], &u).1;
    Divisor::new(u, v)
}

#[derive(Clone, Copy, Debug)]
pub enum Quotient {
    F,
    G,
}
fn curve_map(t: F2, y_cap: F2, quotient: Quotient) -> Point {
    if t == F2::ZERO {
        // Original infinity +/-: f -> O, g -> (0,+/-1).
        return match quotient {
            Quotient::F => None,
            Quotient::G => Some((F2::ZERO, y_cap)),
        };
    }
    let x = F2::base(ROOT).plus(t.inverse());
    let y = y_cap.times(t.power(3).inverse());
    match quotient {
        Quotient::F => Some((x.power(2), y)),
        Quotient::G => {
            if x == F2::ZERO {
                None
            } else {
                Some((x.power(2).inverse(), y.times(x.power(3).inverse())))
            }
        }
    }
}
fn sqrt_base(a: i64) -> Option<i64> {
    (0..P).find(|x| fp(x * x) == fp(a))
}

/// Norm of ANY reduced F101-rational class. Infinity is Ok(None), never an error.
pub fn push(d: &Divisor, quotient: Quotient) -> Result<Point, &'static str> {
    if !d.valid() {
        return Err("invalid canonical Mumford pair");
    }
    let roots = match d.u.len() {
        1 => vec![],
        2 => vec![F2::base(-d.u[0])],
        3 => {
            let disc = fp(d.u[1] * d.u[1] - 4 * d.u[0]);
            let s = match sqrt_base(disc) {
                Some(z) => F2::base(z),
                None => F2(
                    0,
                    sqrt_base(disc * inv(2)).ok_or("quadratic splitting failed")?,
                ),
            };
            let center = F2::base(-d.u[1]);
            vec![
                center.plus(s).times(F2::base(inv(2))),
                center.minus(s).times(F2::base(inv(2))),
            ]
        }
        _ => return Err("degree above two"),
    };
    let mut out = None;
    for t in roots {
        let y = eval(&d.v, t);
        if y.power(2) != eval(&quintic(), t) {
            return Err("split support not on C");
        }
        let image = curve_map(t, y, quotient);
        if !ec_on(image, 0, 0, 1) {
            return Err("quotient point outside E");
        }
        out = ec_add(out, image, 0, 0);
    }
    // Both quotients send W to (-1,0), which has order two.
    if (d.u.len() - 1) & 1 != 0 {
        out = ec_add(out, point(-1, 0), 0, 0);
    }
    if let Some((x, y)) = out {
        if x.1 != 0 || y.1 != 0 {
            return Err("norm did not descend to F101");
        }
    }
    Ok(out)
}

pub fn phi(q: Point) -> Point {
    q.and_then(|(x, y)| {
        let big_x = x.plus(F2::ONE);
        if big_x == F2::ZERO {
            return None;
        }
        Some((
            big_x
                .minus(F2::base(3))
                .plus(F2::base(3).times(big_x.inverse())),
            y.times(F2::ONE.minus(F2::base(3).times(big_x.power(2).inverse()))),
        ))
    })
}

/// Enumerate canonical rational classes; quadratic v is solved coefficientwise.
/// This is a finite fixture for tests, not a DLP algorithm.
pub fn rational_classes() -> Vec<Divisor> {
    let mut out = vec![Divisor::zero()];
    for t in 0..P {
        for y in 0..P {
            if fp(y * y) == eval(&quintic(), F2::base(t)).0 {
                out.push(Divisor::new(vec![fp(-t), 1], trim(vec![y])).unwrap());
            }
        }
    }
    for b in 0..P {
        for c in 0..P {
            let u = vec![c, b, 1];
            let rem = divrem(&quintic(), &u).1;
            let a0 = *rem.first().unwrap_or(&0);
            let a1 = *rem.get(1).unwrap_or(&0);
            for v1 in 0..P {
                if v1 == 0 {
                    if a1 == 0 {
                        for v0 in 0..P {
                            if fp(v0 * v0) == a0 {
                                out.push(Divisor::new(u.clone(), trim(vec![v0])).unwrap());
                            }
                        }
                    }
                } else {
                    let v0 = fp((a1 + b * v1 * v1) * inv(2 * v1));
                    if fp(v0 * v0 - c * v1 * v1) == a0 {
                        out.push(Divisor::new(u.clone(), vec![v0, v1]).unwrap());
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_and_model() {
        assert_eq!(pow(2, 50), 100);
        assert_eq!(fp(pow(ROOT, 6) + 1), 0);
        for a in 0..P {
            for b in 0..P {
                let z = F2(a, b);
                assert_eq!(z.power(101), z.conjugate());
                if z != F2::ZERO {
                    assert_eq!(z.times(z.inverse()), F2::ONE);
                }
            }
        }
        let f = quintic();
        assert_eq!(
            xgcd(
                f.clone(),
                trim(
                    f.iter()
                        .enumerate()
                        .skip(1)
                        .map(|(i, c)| fp(i as i64 * c))
                        .collect()
                )
            )
            .0,
            vec![1]
        );
    }
    #[test]
    fn all_elliptic_inputs_including_exceptions() {
        let points = ec_points();
        assert_eq!(points.len(), 102);
        let mut nonsplit = 0;
        let mut ramified = 0;
        for q in &points {
            let d = pull(*q).unwrap();
            assert_eq!(
                push(&d, Quotient::F),
                Ok(ec_mul(*q, 2, 0, 0)),
                "{q:?} {d:?}"
            );
            assert_eq!(push(&d, Quotient::G), Ok(None), "{q:?} {d:?}");
            if let Some((x, _)) = q {
                if x.0 == 0 {
                    ramified += 1;
                }
                if sqrt_base(x.0).is_none() {
                    nonsplit += 1;
                }
            }
            assert!(ec_on(phi(*q), 6, -3, 0));
            for r in &points {
                assert_eq!(
                    pull(ec_add(*q, *r, 0, 0)).unwrap(),
                    d.add(&pull(*r).unwrap())
                );
                assert_eq!(phi(ec_add(*q, *r, 0, 0)), ec_add(phi(*q), phi(*r), 6, -3));
            }
        }
        assert!(nonsplit > 0 && ramified == 2);
        assert!(pull(point(-1, 0)).unwrap().u.len() == 2);
        for y in [1, 100] {
            let infinity = Divisor::new(vec![0, 1], vec![y]).unwrap();
            assert_eq!(push(&infinity, Quotient::F), Ok(point(-1, 0)));
            assert_eq!(
                push(&infinity, Quotient::G),
                Ok(ec_add(point(0, y), point(-1, 0), 0, 0))
            );
        }
    }
    #[test]
    fn arbitrary_rational_classes_and_norm_homomorphisms() {
        let classes = rational_classes();
        assert_eq!(classes.len(), 10404);
        let unique: std::collections::BTreeSet<_> = classes.iter().cloned().collect();
        assert_eq!(unique.len(), classes.len());
        let generators = [&classes[1], &classes[37], &classes[111]];
        let mut fibers = std::collections::BTreeMap::new();
        let mut nonsplit = 0;
        let mut repeated = 0;
        for d in &classes {
            assert_eq!(d.add(&d.neg()), Divisor::zero());
            assert_eq!(d.add(&Divisor::zero()), *d);
            let f = push(d, Quotient::F).unwrap();
            let g = push(d, Quotient::G).unwrap();
            let key = f.map(|(x, y)| (x.0, y.0));
            *fibers.entry(key).or_insert(0_usize) += 1;
            if d.u.len() == 3 {
                let disc = fp(d.u[1] * d.u[1] - 4 * d.u[0]);
                if sqrt_base(disc).is_none() {
                    nonsplit += 1;
                }
                if disc == 0 {
                    repeated += 1;
                }
            }
            for h in generators {
                let sum = d.add(h);
                assert_eq!(
                    push(&sum, Quotient::F).unwrap(),
                    ec_add(f, push(h, Quotient::F).unwrap(), 0, 0)
                );
                assert_eq!(
                    push(&sum, Quotient::G).unwrap(),
                    ec_add(g, push(h, Quotient::G).unwrap(), 0, 0)
                );
            }
        }
        assert_eq!(fibers.len(), 102);
        assert!(fibers.values().all(|n| *n == 102));
        assert!(nonsplit > 0 && repeated > 0);
    }
    #[test]
    fn order_17_dlp_chain_and_rejections() {
        let p = point(4, 41);
        let dp = pull(p).unwrap();
        let tp = phi(push(&dp, Quotient::F).unwrap());
        assert_eq!(ec_mul(p, 17, 0, 0), None);
        assert_ne!(tp, None);
        assert_eq!(dp.mul(17), Divisor::zero());
        for k in 0..17 {
            let q = ec_mul(p, k, 0, 0);
            assert_eq!(pull(q).unwrap(), dp.mul(k));
            assert_eq!(
                phi(push(&pull(q).unwrap(), Quotient::F).unwrap()),
                ec_mul(tp, k, 6, -3)
            );
        }
        assert!(pull(point(1, 1)).is_err());
        assert!(Divisor::new(vec![], vec![]).is_err());
        assert!(Divisor::new(vec![0, 1], vec![]).is_err());
        assert!(push(
            &Divisor {
                u: vec![1, 0],
                v: vec![]
            },
            Quotient::F
        )
        .is_err());
        assert_eq!(push(&Divisor::zero(), Quotient::F), Ok(None));
    }
}
