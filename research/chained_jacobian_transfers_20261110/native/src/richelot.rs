//! Explicit fixed Richelot correspondence, Kunzweiler (DCC 2024), Prop. 7–8.
//! Source factors: x²+1, x²+10x-1, x²-10x-1, in THIS order.
//! Target coefficients depend on determinant/sign convention; never retwist.
//! Exceptional fibers are extended on the smooth projective source: original
//! infinities, Weierstrass support, and a target infinity in degree-drop fibers.
//! Invalid inputs are errors; the fixed-field rational class domain is tested.

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

fn finite_target_point(z: F2, w: F2, f: &[F2]) -> Result<ED, &'static str> {
    if z == F2::base(ROOT) {
        return Err("target Weierstrass base point must be removed as a divisor");
    }
    let t = z.minus(F2::base(ROOT)).inverse();
    ED::new(vec![t.negative(), F2::ONE], vec![w.times(t.power(3))], f)
}
fn fiber_image(t: F2, y_cap: F2, f: &[F2], g: &[EP; 3], h: &[EP; 3]) -> Result<ED, &'static str> {
    if t == F2::ZERO {
        // x=∞: G1/G2 -> 1, and y/x³ -> y_cap=+/-1.
        let g1lead = *g[0].last().unwrap();
        let g2lead = *g[1].last().unwrap();
        let u = mono(&plus(&times(&h[0], &[g1lead]), &times(&h[1], &[g2lead])));
        let v = quotient(&times(&h[0], &[g1lead.times(y_cap.inverse())]), &u).1;
        return to_chart(&u, &v, f);
    }
    let x = F2::base(ROOT).plus(t.inverse());
    let y = y_cap.times(t.power(3).inverse());
    let g1 = value(&g[0], x);
    let g2 = value(&g[1], x);
    if y == F2::ZERO {
        if g1 == F2::ZERO {
            return to_chart(&h[1], &[], f);
        }
        if g2 == F2::ZERO {
            return to_chart(&h[0], &[], f);
        }
        if value(&g[2], x) == F2::ZERO {
            // The completed fiber is a hyperelliptic conjugate pair over z=x.
            // Its class relative to 2W' is zero, not a repeated signed point.
            return Ok(ED::zero());
        }
        return Err("zero-y support outside the source branch locus");
    }
    let u = tr(plus(&times(&h[0], &[g1]), &times(&h[1], &[g2])));
    if u.len() == 2 {
        // One finite point and one original target infinity. Reducing v mod
        // the linear finite polynomial alone would silently lose that point.
        let z = u[0].negative().times(u[1].inverse());
        let w = g1
            .times(value(&h[0], z))
            .times(x.minus(z))
            .times(y.inverse());
        let finite = finite_target_point(z, w, f)?;
        let infinity_y = g1
            .times(*h[0].last().unwrap())
            .negative()
            .times(y.inverse());
        let infinity = ED::new(base(&[0, 1]), vec![infinity_y], f)?;
        return Ok(finite.add(&infinity, f));
    }
    if u.len() != 3 {
        return Err("unexpected correspondence fiber dimension");
    }
    let u = mono(&u);
    let v = quotient(
        &times(&times(&h[0], &[g1.times(y.inverse())]), &[x, F2::base(-1)]),
        &u,
    )
    .1;
    to_chart(&u, &v, f)
}

/// Fixed-field evaluator of a genuine degree-four (2,2)-isogeny.
/// No source/target DLP solver or scalar is called. Invalid inputs are Err.
pub fn transfer(d: &Divisor) -> Result<Image, &'static str> {
    if !d.valid() {
        return Err("invalid source divisor");
    }
    let f = target_f();
    let g = factors();
    let h = dual_factors();
    Ok(Image(evaluate(&d.u, &d.v, &f, &g, &h)?))
}

fn evaluate(u: &[i64], v: &[i64], f: &[F2], g: &[EP; 3], h: &[EP; 3]) -> Result<ED, &'static str> {
    let mut result = ED::zero();
    for t in roots(u)? {
        result = result.add(&fiber_image(t, eval(v, t), f, g, h)?, f);
    }
    // ψ(W)=(H2,0). Subtract deg(D)*ψ(W), not an assumed zero base image.
    let base_image = to_chart(&h[1], &[], f)?;
    result = result.add(&base_image.mul((u.len() - 1) as u64, f).neg(), f);
    if result.u.iter().chain(&result.v).any(|z| z.1 != 0) {
        return Err("image did not descend to F101");
    }
    Ok(result)
}

/// Sign-corrected reversed correspondence on the represented target image.
/// Normalizing the brackets twice recovers G, but swaps (x-z) to (z-x).
/// Negation restores the dual's positive norm convention.
pub fn dual(image: &Image) -> Result<Divisor, &'static str> {
    let (u, v) = image.coefficients();
    let d = evaluate(&u, &v, &base(&quintic()), &dual_factors(), &factors())?.neg();
    Divisor::new(
        d.u.iter().map(|z| z.0).collect(),
        d.v.iter().map(|z| z.0).collect(),
    )
}

/// Coefficients a2,a4,a6 after X=L*z², V=L*y, L=leading(sextic).
pub fn endpoint() -> (i64, i64, i64) {
    let f = target_sextic();
    (f[4], fp(f[6] * f[2]), fp(f[6] * f[6] * f[0]))
}
/// Deduplication control, not used to evaluate the Jacobian correspondence.
/// Historical degree-two phi after doubling, then the fixed codomain isomorphism.
pub fn collapsed_endpoint(q: Point) -> Point {
    phi(ec_mul(q, 2, 0, 0)).map(|(u, v)| {
        (
            F2::base(82).times(u).plus(F2::base(60)),
            F2::base(98).times(v),
        )
    })
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
        let (a, b, c) = endpoint();
        let disc =
            fp(a * a * b * b - 4 * b * b * b - 4 * a * a * a * c - 27 * c * c + 18 * a * b * c);
        assert_ne!(disc, 0, "singular elliptic endpoint");
        assert_eq!(fp(16 * a * a - 48 * b), 47);
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
        assert_eq!(
            transfer(&pull(point(-1, 0)).unwrap()).unwrap(),
            Image::zero()
        );
    }

    #[test]
    fn every_rational_class_kernel_fibers_and_additivity() {
        let classes = rational_classes();
        let mut fibers = std::collections::BTreeMap::new();
        let translations = [&classes[1], &classes[37], &classes[111]];
        let mut kernel = 0;
        for d in &classes {
            let image = transfer(d).unwrap_or_else(|e| panic!("{e}: {d:?}"));
            if image == Image::zero() {
                kernel += 1;
                assert_eq!(d.mul(2), Divisor::zero());
            }
            *fibers.entry(image.coefficients()).or_insert(0_usize) += 1;
            for h in translations {
                assert_eq!(
                    transfer(&d.add(h)).unwrap(),
                    image.add(&transfer(h).unwrap()),
                    "{d:?}, {h:?}"
                );
            }
        }
        assert_eq!(kernel, 4);
        assert_eq!(fibers.len(), 2601);
        assert!(fibers.values().all(|n| *n == 4));
    }

    #[test]
    fn complete_elliptic_domain_route_deduplication_control() {
        for q in ec_points() {
            let via_jacobians = norm(&transfer(&pull(q).unwrap()).unwrap(), Quotient::F).unwrap();
            assert_eq!(via_jacobians, collapsed_endpoint(q), "source point {q:?}");
        }
    }

    #[test]
    fn dual_norm_and_order_17_point_recovery() {
        for d in rational_classes() {
            assert_eq!(dual(&transfer(&d).unwrap()).unwrap(), d.mul(2), "{d:?}");
        }
        for k in 0..17 {
            let q = ec_mul(point(4, 41), k, 0, 0);
            let image = transfer(&pull(q).unwrap()).unwrap();
            let back = push(&dual(&image).unwrap(), Quotient::F).unwrap();
            // norm_f * dual_R * R * pull_f = [4], inverse(4) mod 17=13.
            assert_eq!(ec_mul(back, 13, 0, 0), q);
        }
    }
}
