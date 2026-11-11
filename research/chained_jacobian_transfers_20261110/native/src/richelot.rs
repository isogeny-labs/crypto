//! Explicit fixed Richelot correspondence, Kunzweiler (DCC 2024), Prop. 7–8.
//! Source factors: x²+1, x²+10x-1, x²-10x-1, in THIS order.
//! Target coefficients depend on determinant/sign convention; never retwist.
//! Generic affine support only: unsupported Weierstrass/pole cases return Err.
//! The complete order-17 elliptic pullback image is tested, including identity.

use super::*;
type EP = Vec<F2>;

fn tr(mut p: EP) -> EP {
    while p.last() == Some(&F2::ZERO) {
        p.pop();
    }
    p
}
fn base(p: &[i64]) -> EP {
    p.iter().map(|x| F2::base(*x)).collect()
}
fn plus(a: &[F2], b: &[F2]) -> EP {
    tr((0..a.len().max(b.len()))
        .map(|i| {
            a.get(i)
                .copied()
                .unwrap_or(F2::ZERO)
                .plus(b.get(i).copied().unwrap_or(F2::ZERO))
        })
        .collect())
}
fn negative(a: &[F2]) -> EP {
    tr(a.iter().map(|x| x.negative()).collect())
}
fn minus(a: &[F2], b: &[F2]) -> EP {
    plus(a, &negative(b))
}
fn times(a: &[F2], b: &[F2]) -> EP {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![F2::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = c[i + j].plus(x.times(*y));
        }
    }
    tr(c)
}
fn quotient(a: &[F2], b: &[F2]) -> (EP, EP) {
    let mut a = tr(a.to_vec());
    let b = tr(b.to_vec());
    assert!(!b.is_empty());
    let mut q = vec![F2::ZERO; (a.len() + 1).saturating_sub(b.len())];
    let z = b.last().unwrap().inverse();
    while !a.is_empty() && a.len() >= b.len() {
        let k = a.len() - b.len();
        let c = a.last().unwrap().times(z);
        q[k] = c;
        for (j, bj) in b.iter().enumerate() {
            a[k + j] = a[k + j].minus(c.times(*bj));
        }
        a = tr(a);
    }
    (tr(q), a)
}
fn exact_div(a: &[F2], b: &[F2]) -> EP {
    let (q, r) = quotient(a, b);
    assert!(r.is_empty());
    q
}
fn mono(a: &[F2]) -> EP {
    times(a, &[a.last().unwrap().inverse()])
}
fn egcd(mut a: EP, mut b: EP) -> (EP, EP, EP) {
    let (mut s, mut ss, mut t, mut tt) = (base(&[1]), vec![], vec![], base(&[1]));
    while !b.is_empty() {
        let (q, r) = quotient(&a, &b);
        a = b;
        b = r;
        (s, ss) = (ss.clone(), minus(&s, &times(&q, &ss)));
        (t, tt) = (tt.clone(), minus(&t, &times(&q, &tt)));
    }
    let z = [a.last().unwrap().inverse()];
    (times(&a, &z), times(&s, &z), times(&t, &z))
}
fn value(a: &[F2], x: F2) -> F2 {
    a.iter().rev().fold(F2::ZERO, |v, c| v.times(x).plus(*c))
}
fn derivative(a: &[F2]) -> EP {
    tr(a.iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| c.times(F2::base(i as i64)))
        .collect())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ED {
    u: EP,
    v: EP,
}
impl ED {
    fn zero() -> Self {
        Self {
            u: base(&[1]),
            v: vec![],
        }
    }
    fn new(mut u: EP, mut v: EP, f: &[F2]) -> Result<Self, &'static str> {
        u = tr(u);
        if u.is_empty() {
            return Err("zero target u");
        }
        u = mono(&u);
        v = quotient(&v, &u).1;
        if !quotient(&minus(&times(&v, &v), f), &u).1.is_empty() {
            return Err("target curve congruence failed");
        }
        while u.len() > 3 {
            u = mono(&exact_div(&minus(f, &times(&v, &v)), &u));
            v = quotient(&negative(&v), &u).1;
        }
        Ok(Self { u, v })
    }
    fn add(&self, b: &Self, f: &[F2]) -> Self {
        let (d1, s, t) = egcd(self.u.clone(), b.u.clone());
        let (d, c, e) = egcd(d1, plus(&self.v, &b.v));
        let u = exact_div(&times(&self.u, &b.u), &times(&d, &d));
        let v = exact_div(
            &plus(
                &plus(
                    &times(&times(&times(&s, &c), &self.u), &b.v),
                    &times(&times(&times(&t, &c), &b.u), &self.v),
                ),
                &times(&e, &plus(&times(&self.v, &b.v), f)),
            ),
            &d,
        );
        Self::new(u, v, f).unwrap()
    }
    fn neg(&self) -> Self {
        Self {
            u: self.u.clone(),
            v: negative(&self.v),
        }
    }
    fn mul(&self, mut n: u64, f: &[F2]) -> Self {
        let mut a = self.clone();
        let mut b = Self::zero();
        while n != 0 {
            if n & 1 != 0 {
                b = b.add(&a, f);
            }
            a = a.add(&a, f);
            n >>= 1;
        }
        b
    }
}

fn factors() -> [EP; 3] {
    [base(&[1, 0, 1]), base(&[-1, 10, 1]), base(&[-1, -10, 1])]
}
pub fn determinant() -> i64 {
    let g = factors();
    let mut d = F2::ZERO;
    for i in 0..3 {
        d = d.plus(g[0][i].times(g[1][(i + 1) % 3]).times(g[2][(i + 2) % 3]));
        d = d.minus(g[0][i].times(g[1][(i + 2) % 3]).times(g[2][(i + 1) % 3]));
    }
    assert_eq!(d.1, 0);
    d.0
}
fn dual_factors() -> [EP; 3] {
    let g = factors();
    let z = F2::base(inv(determinant()));
    std::array::from_fn(|i| {
        let a = &g[(i + 1) % 3];
        let b = &g[(i + 2) % 3];
        times(
            &minus(&times(&derivative(a), b), &times(a, &derivative(b))),
            &[z],
        )
    })
}
pub fn target_sextic() -> Poly {
    let h = dual_factors();
    times(&times(&h[0], &h[1]), &h[2])
        .iter()
        .map(|x| {
            assert_eq!(x.1, 0);
            x.0
        })
        .collect()
}
// F_new(T)=T^6*F_old(ROOT+1/T), valid because ROOT is a Weierstrass point.
fn target_f() -> EP {
    let f = base(&target_sextic());
    let mut out = vec![F2::ZERO; 7];
    let mut power = base(&[1]);
    for (i, c) in f.iter().enumerate() {
        for (j, z) in power.iter().enumerate() {
            out[j + 6 - i] = out[j + 6 - i].plus(c.times(*z));
        }
        power = times(&power, &base(&[1, ROOT]));
    }
    tr(out)
}
pub fn target_quintic() -> Poly {
    target_f()
        .iter()
        .map(|z| {
            assert_eq!(z.1, 0);
            z.0
        })
        .collect()
}
// u has degree exactly two and v degree below two, on the old sextic chart.
fn to_chart(u: &[F2], v: &[F2], f: &[F2]) -> Result<ED, &'static str> {
    if u.len() != 3 {
        return Err("Richelot fiber degree drop unsupported");
    }
    let mut uu = vec![F2::ZERO; 3];
    let mut power = base(&[1]);
    for (i, c) in u.iter().enumerate() {
        for (j, z) in power.iter().enumerate() {
            uu[j + 2 - i] = uu[j + 2 - i].plus(c.times(*z));
        }
        power = times(&power, &base(&[1, ROOT]));
    }
    if uu[2] == F2::ZERO {
        return Err("Richelot fiber meets target base point");
    }
    let mut vv = vec![F2::ZERO; 4];
    let mut power = base(&[1]);
    for (i, c) in v.iter().enumerate() {
        for (j, z) in power.iter().enumerate() {
            vv[j + 3 - i] = vv[j + 3 - i].plus(c.times(*z));
        }
        power = times(&power, &base(&[1, ROOT]));
    }
    ED::new(uu, vv, f)
}
fn roots(u: &[i64]) -> Result<Vec<F2>, &'static str> {
    match u.len() {
        1 => Ok(vec![]),
        2 => Ok(vec![F2::base(-u[0])]),
        3 => {
            let disc = fp(u[1] * u[1] - 4 * u[0]);
            let s = match sqrt_base(disc) {
                Some(x) => F2::base(x),
                None => F2(0, sqrt_base(disc * inv(2)).ok_or("splitting failed")?),
            };
            let mid = F2::base(-u[1]);
            let half = F2::base(inv(2));
            Ok(vec![mid.plus(s).times(half), mid.minus(s).times(half)])
        }
        _ => Err("unsupported support degree"),
    }
}

/// Canonical class on the FIXED Richelot target. Construction is private.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image(ED);
impl Image {
    pub fn zero() -> Self {
        Self(ED::zero())
    }
    pub fn add(&self, b: &Self) -> Self {
        Self(self.0.add(&b.0, &target_f()))
    }
    pub fn mul(&self, n: u64) -> Self {
        Self(self.0.mul(n, &target_f()))
    }
    pub fn coefficients(&self) -> (Poly, Poly) {
        (
            self.0.u.iter().map(|z| z.0).collect(),
            self.0.v.iter().map(|z| z.0).collect(),
        )
    }
}

/// Partial evaluator of a genuine degree-four (2,2)-isogeny.
/// No source/target DLP solver or scalar is called. Unsupported support is Err.
pub fn transfer(d: &Divisor) -> Result<Image, &'static str> {
    if !d.valid() {
        return Err("invalid source divisor");
    }
    let f = target_f();
    let g = factors();
    let h = dual_factors();
    let mut result = ED::zero();
    for t in roots(&d.u)? {
        if t == F2::ZERO {
            return Err("source infinity chart not supported by this evaluator");
        }
        let x = F2::base(ROOT).plus(t.inverse());
        let y = eval(&d.v, t).times(t.power(3).inverse());
        if y == F2::ZERO {
            return Err("Weierstrass support not supported by this evaluator");
        }
        let g1 = value(&g[0], x);
        let g2 = value(&g[1], x);
        let u = tr(plus(&times(&h[0], &[g1]), &times(&h[1], &[g2])));
        if u.len() != 3 {
            return Err("Richelot affine fiber degree drop");
        }
        let u = mono(&u);
        let v = quotient(
            &times(&times(&h[0], &[g1.times(y.inverse())]), &[x, F2::base(-1)]),
            &u,
        )
        .1;
        result = result.add(&to_chart(&u, &v, &f)?, &f);
    }
    // ψ(W)=(H2,0). Subtract deg(D)*ψ(W), not an assumed zero base image.
    let base_image = to_chart(&h[1], &[], &f)?;
    result = result.add(&base_image.mul((d.u.len() - 1) as u64, &f).neg(), &f);
    if result.u.iter().chain(&result.v).any(|z| z.1 != 0) {
        return Err("image did not descend to F101");
    }
    Ok(Image(result))
}

/// Coefficients a2,a4,a6 after X=L*z², V=L*y, L=leading(sextic).
pub fn endpoint() -> (i64, i64, i64) {
    let f = target_sextic();
    (f[4], fp(f[6] * f[2]), fp(f[6] * f[6] * f[0]))
}
pub fn norm(image: &Image, q: Quotient) -> Result<Point, &'static str> {
    let (u, v) = image.coefficients();
    let l = F2::base(target_sextic()[6]);
    let (a2, a4, a6) = endpoint();
    let mut out = None;
    for t in roots(&u)? {
        let y_cap = eval(&v, t);
        if y_cap.power(2) != value(&target_f(), t) {
            return Err("invalid target support");
        }
        let point = if t == F2::ZERO {
            match q {
                Quotient::F => None,
                Quotient::G => Some((F2::ZERO, l.times(y_cap))),
            }
        } else {
            let mut x = F2::base(ROOT).plus(t.inverse());
            let mut y = y_cap.times(t.power(3).inverse());
            match q {
                Quotient::F => Some((l.times(x.power(2)), l.times(y))),
                Quotient::G => {
                    if x == F2::ZERO {
                        None
                    } else {
                        y = y.times(x.power(3).inverse());
                        x = x.inverse();
                        Some((l.times(x.power(2)), l.times(y)))
                    }
                }
            }
        };
        if !ec_on(point, a2, a4, a6) {
            return Err("endpoint outside curve");
        }
        out = ec_add(out, point, a2, a4);
    }
    if (u.len() - 1) & 1 != 0 {
        out = ec_add(out, Some((l.times(F2::base(-1)), F2::ZERO)), a2, a4);
    }
    if let Some((x, y)) = out {
        if x.1 != 0 || y.1 != 0 {
            return Err("endpoint did not descend");
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_models_and_sign() {
        let g = factors();
        assert_eq!(
            times(&times(&g[0], &g[1]), &g[2]),
            base(&[1, 0, 0, 0, 0, 0, 1])
        );
        assert_eq!(determinant(), 40);
        assert_eq!(target_sextic(), vec![41, 0, 9, 0, 9, 0, 41]);
        assert_eq!(endpoint(), (9, 66, 39));
        let f = target_f();
        assert_eq!(f.len(), 6);
        assert_eq!(egcd(f.clone(), derivative(&f)).0, base(&[1]));
    }
    #[test]
    fn entire_order_17_image_and_both_endpoint_projections() {
        let p = point(4, 41);
        let d = pull(p).unwrap();
        let image = transfer(&d).unwrap();
        assert_ne!(image, Image::zero());
        assert_eq!(image.mul(17), Image::zero());
        let a = norm(&image, Quotient::F).unwrap();
        let b = norm(&image, Quotient::G).unwrap();
        println!("Richelot endpoint F={a:?}, G={b:?}");
        assert!(a.is_some() || b.is_some());
        let (a2, a4, _) = endpoint();
        for k in 0..17 {
            let dk = pull(ec_mul(p, k, 0, 0)).unwrap();
            let ik = transfer(&dk).unwrap();
            assert_eq!(ik, image.mul(k));
            assert_eq!(norm(&ik, Quotient::F).unwrap(), ec_mul(a, k, a2, a4));
            assert_eq!(norm(&ik, Quotient::G).unwrap(), ec_mul(b, k, a2, a4));
            for j in 0..17 {
                let dj = pull(ec_mul(p, j, 0, 0)).unwrap();
                assert_eq!(
                    transfer(&dk.add(&dj)).unwrap(),
                    ik.add(&transfer(&dj).unwrap())
                );
            }
        }
        assert!(transfer(&pull(point(-1, 0)).unwrap()).is_err());
    }
}
