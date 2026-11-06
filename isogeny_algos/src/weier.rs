//! General Weierstrass curves y^2 + a1 x y + a3 y = x^3 + a2 x^2 + a4 x + a6 over any field, and
//! Vélu's formulas in the characteristic-independent form (Vélu 1971, as in Silverman III.4 and
//! Kohel's thesis), so that isogenies can be computed in characteristic 2 and 3 where the
//! short-form coefficients (which divide by 2, 3, 6, 24, 27) are unavailable.
use crate::curve::Pt;
use crate::field::{Field, Rng};
use crate::poly::{self, Poly};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GWCurve<E> {
    pub a1: E,
    pub a2: E,
    pub a3: E,
    pub a4: E,
    pub a6: E,
}

impl<E: Copy + PartialEq> GWCurve<E> {
    pub fn new(a: [E; 5]) -> Self {
        GWCurve {
            a1: a[0],
            a2: a[1],
            a3: a[2],
            a4: a[3],
            a6: a[4],
        }
    }
    pub fn on_curve<F: Field<E = E>>(&self, f: &F, p: &Pt<E>) -> bool {
        match *p {
            Pt::Inf => true,
            Pt::Aff(x, y) => {
                let l = f.add(
                    f.add(f.sq(y), f.mul(self.a1, f.mul(x, y))),
                    f.mul(self.a3, y),
                );
                let r = f.add(
                    f.add(
                        f.add(f.mul(f.sq(x), x), f.mul(self.a2, f.sq(x))),
                        f.mul(self.a4, x),
                    ),
                    self.a6,
                );
                l == r
            }
        }
    }
    pub fn neg<F: Field<E = E>>(&self, f: &F, p: &Pt<E>) -> Pt<E> {
        match *p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(x, f.sub(f.sub(f.neg(y), f.mul(self.a1, x)), self.a3)),
        }
    }
    pub fn add<F: Field<E = E>>(&self, f: &F, p: &Pt<E>, q: &Pt<E>) -> Pt<E> {
        let (x1, y1) = match *p {
            Pt::Inf => return *q,
            Pt::Aff(x, y) => (x, y),
        };
        let (x2, y2) = match *q {
            Pt::Inf => return *p,
            Pt::Aff(x, y) => (x, y),
        };
        if x1 == x2 && y2 == self.neg_y(f, x1, y1) {
            return Pt::Inf;
        }
        let lam = if x1 == x2 {
            // doubling: (3x^2 + 2 a2 x + a4 - a1 y) / (2y + a1 x + a3)
            let num = f.sub(
                f.add(
                    f.add(
                        f.mul(f.from_u64(3), f.sq(x1)),
                        f.mul(f.mul(f.from_u64(2), self.a2), x1),
                    ),
                    self.a4,
                ),
                f.mul(self.a1, y1),
            );
            let den = f.add(f.add(f.mul(f.from_u64(2), y1), f.mul(self.a1, x1)), self.a3);
            f.div(num, den)
        } else {
            f.div(f.sub(y2, y1), f.sub(x2, x1))
        };
        // x3 = lam^2 + a1 lam - a2 - x1 - x2;  y3 = -(lam (x3 - x1) + y1) - a1 x3 - a3
        let x3 = f.sub(
            f.sub(f.sub(f.add(f.sq(lam), f.mul(self.a1, lam)), self.a2), x1),
            x2,
        );
        let y3 = f.sub(
            f.sub(
                f.neg(f.add(f.mul(lam, f.sub(x3, x1)), y1)),
                f.mul(self.a1, x3),
            ),
            self.a3,
        );
        Pt::Aff(x3, y3)
    }
    fn neg_y<F: Field<E = E>>(&self, f: &F, x: E, y: E) -> E {
        f.sub(f.sub(f.neg(y), f.mul(self.a1, x)), self.a3)
    }
    pub fn mul<F: Field<E = E>>(&self, f: &F, p: &Pt<E>, mut k: u64) -> Pt<E> {
        let mut r = Pt::Inf;
        let mut b = *p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(f, &r, &b);
            }
            b = self.add(f, &b, &b);
            k >>= 1;
        }
        r
    }
    /// b2, b4, b6, b8, c4, discriminant, j (standard formulas; integer constants reduce in F).
    pub fn disc<F: Field<E = E>>(&self, f: &F) -> E {
        let (b2, b4, b6, b8) = self.b_invariants(f);
        let c = |v: u64| f.from_u64(v);
        // Delta = -b2^2 b8 - 8 b4^3 - 27 b6^2 + 9 b2 b4 b6
        let t1 = f.neg(f.mul(f.sq(b2), b8));
        let t2 = f.mul(c(8), f.mul(b4, f.sq(b4)));
        let t3 = f.mul(c(27), f.sq(b6));
        let t4 = f.mul(c(9), f.mul(b2, f.mul(b4, b6)));
        f.add(f.sub(f.sub(t1, t2), t3), t4)
    }
    pub fn b_invariants<F: Field<E = E>>(&self, f: &F) -> (E, E, E, E) {
        let c = |v: u64| f.from_u64(v);
        let b2 = f.add(f.sq(self.a1), f.mul(c(4), self.a2));
        let b4 = f.add(f.mul(c(2), self.a4), f.mul(self.a1, self.a3));
        let b6 = f.add(f.sq(self.a3), f.mul(c(4), self.a6));
        let b8 = f.add(
            f.sub(
                f.add(
                    f.mul(f.sq(self.a1), self.a6),
                    f.mul(c(4), f.mul(self.a2, self.a6)),
                ),
                f.mul(self.a1, f.mul(self.a3, self.a4)),
            ),
            f.sub(f.mul(self.a2, f.sq(self.a3)), f.sq(self.a4)),
        );
        (b2, b4, b6, b8)
    }
    pub fn j<F: Field<E = E>>(&self, f: &F) -> E {
        let (b2, b4, _, _) = self.b_invariants(f);
        let c4 = f.sub(f.sq(b2), f.mul(f.from_u64(24), b4));
        f.div(f.mul(f.sq(c4), c4), self.disc(f))
    }
}

/// An odd-degree Vélu isogeny with cyclic kernel, in the general Weierstrass form.
pub struct GWVelu<E> {
    pub dom: GWCurve<E>,
    pub cod: GWCurve<E>,
    pub deg: u64,
    /// per kernel pair {Q, -Q}: (xQ, yQ, tQ, uQ)
    pub data: Vec<(E, E, E, E)>,
}

/// Vélu from a generator P of a cyclic subgroup of odd order ell (coprime to char), using the
/// representatives P, 2P, ..., ((ell-1)/2) P (no 2-torsion in an odd-order group).
pub fn gw_velu_cyclic<F: Field>(f: &F, e: &GWCurve<F::E>, p: &Pt<F::E>, ell: u64) -> GWVelu<F::E> {
    let mut q = *p;
    let mut data = Vec::with_capacity((ell / 2) as usize);
    let (mut t, mut w) = (f.zero(), f.zero());
    for i in 0..ell / 2 {
        if i > 0 {
            q = e.add(f, &q, p);
        }
        let (xq, yq) = match q {
            Pt::Aff(x, y) => (x, y),
            Pt::Inf => panic!("kernel generator order mismatch"),
        };
        // gxQ = 3 xQ^2 + 2 a2 xQ + a4 - a1 yQ;  gyQ = -2 yQ - a1 xQ - a3
        let gx = f.sub(
            f.add(
                f.add(
                    f.mul(f.from_u64(3), f.sq(xq)),
                    f.mul(f.mul(f.from_u64(2), e.a2), xq),
                ),
                e.a4,
            ),
            f.mul(e.a1, yq),
        );
        let gy = f.sub(
            f.sub(f.neg(f.mul(f.from_u64(2), yq)), f.mul(e.a1, xq)),
            e.a3,
        );
        // Q not 2-torsion: tQ = 2 gxQ - a1 gyQ, uQ = gyQ^2
        let tq = f.sub(f.mul(f.from_u64(2), gx), f.mul(e.a1, gy));
        let uq = f.sq(gy);
        t = f.add(t, tq);
        w = f.add(w, f.add(uq, f.mul(xq, tq)));
        let _ = gx;
        data.push((xq, yq, tq, uq));
    }
    let c = |v: u64| f.from_u64(v);
    let b2 = f.add(f.sq(e.a1), f.mul(c(4), e.a2));
    let cod = GWCurve {
        a1: e.a1,
        a2: e.a2,
        a3: e.a3,
        a4: f.sub(e.a4, f.mul(c(5), t)),
        a6: f.sub(f.sub(e.a6, f.mul(b2, t)), f.mul(c(7), w)),
    };
    GWVelu {
        dom: *e,
        cod,
        deg: ell,
        data,
    }
}

impl<E: Copy + PartialEq> GWVelu<E> {
    pub fn eval<F: Field<E = E>>(&self, f: &F, p: &Pt<E>) -> Pt<E> {
        let (x, y) = match *p {
            Pt::Inf => return Pt::Inf,
            Pt::Aff(x, y) => (x, y),
        };
        let mut xx = x;
        // X'(x) = 1 - sum tQ/(x-xQ)^2 + 2 uQ/(x-xQ)^3
        let mut dxdx = f.one();
        for &(xq, _yq, tq, uq) in &self.data {
            let dx = f.sub(x, xq);
            if f.is_zero(dx) {
                return Pt::Inf; // in the kernel
            }
            let di = f.inv(dx);
            let di2 = f.sq(di);
            let di3 = f.mul(di2, di);
            xx = f.add(xx, f.add(f.mul(tq, di), f.mul(uq, di2)));
            dxdx = f.sub(
                dxdx,
                f.add(f.mul(tq, di2), f.mul(f.mul(f.from_u64(2), uq), di3)),
            );
        }
        // the Vélu isogeny is normalised: dX/(2Y + A1 X + A3) = dx/(2y + a1 x + a3), so
        // 2Y + A1 X + A3 = (2y + a1 x + a3) X'(x), hence Y = ((2y+a1x+a3) X'(x) - A1 X - A3)/2
        let two_y = f.add(
            f.add(f.mul(f.from_u64(2), y), f.mul(self.dom.a1, x)),
            self.dom.a3,
        );
        let yy = f.div(
            f.sub(
                f.sub(f.mul(two_y, dxdx), f.mul(self.cod.a1, xx)),
                self.cod.a3,
            ),
            f.from_u64(2),
        );
        Pt::Aff(xx, yy)
    }
}

/// x-only division polynomials of a general Weierstrass curve: f_n = psi_n for odd n and
/// psi_n / psi_2 for even n, with psi_2^2 = F = 4x^3 + b2 x^2 + 2 b4 x + b6 (Silverman, Ex. 3.7).
/// Integer formulas in the b-invariants, valid in every characteristic.
pub fn division_poly<F: Field>(f: &F, e: &GWCurve<F::E>, n: usize) -> Poly<F> {
    let c = |v: u64| f.from_u64(v);
    let (b2, b4, b6, b8) = e.b_invariants(f);
    let big_f: Poly<F> = vec![b6, f.mul(c(2), b4), b2, c(4)];
    let f2 = poly::mul(f, &big_f, &big_f);
    let mut memo: Vec<Option<Poly<F>>> = vec![None; n.max(4) + 3];
    memo[0] = Some(vec![]);
    memo[1] = Some(vec![f.one()]);
    memo[2] = Some(vec![f.one()]);
    memo[3] = Some(vec![b8, f.mul(c(3), b6), f.mul(c(3), b4), b2, c(3)]);
    memo[4] = Some(vec![
        f.sub(f.mul(b4, b8), f.sq(b6)),
        f.sub(f.mul(b2, b8), f.mul(b4, b6)),
        f.mul(c(10), b8),
        f.mul(c(10), b6),
        f.mul(c(5), b4),
        b2,
        c(2),
    ]);
    fn get<F: Field>(f: &F, k: usize, memo: &mut Vec<Option<Poly<F>>>, f2: &Poly<F>) -> Poly<F> {
        if let Some(v) = &memo[k] {
            return v.clone();
        }
        let cube = |p: &Poly<F>| poly::mul(f, p, &poly::mul(f, p, p));
        let r = if k % 2 == 1 {
            let m = (k - 1) / 2;
            let (a, b) = (get(f, m + 2, memo, f2), get(f, m, memo, f2));
            let (c1, d1) = (get(f, m - 1, memo, f2), get(f, m + 1, memo, f2));
            let t1 = poly::mul(f, &a, &cube(&b));
            let t2 = poly::mul(f, &c1, &cube(&d1));
            if m.is_multiple_of(2) {
                poly::sub(f, &poly::mul(f, f2, &t1), &t2)
            } else {
                poly::sub(f, &t1, &poly::mul(f, f2, &t2))
            }
        } else {
            let m = k / 2;
            let gm = get(f, m, memo, f2);
            let (gp2, gm1) = (get(f, m + 2, memo, f2), get(f, m - 1, memo, f2));
            let (gm2, gp1) = (get(f, m - 2, memo, f2), get(f, m + 1, memo, f2));
            let l = poly::mul(f, &gp2, &poly::mul(f, &gm1, &gm1));
            let r = poly::mul(f, &gm2, &poly::mul(f, &gp1, &gp1));
            poly::mul(f, &gm, &poly::sub(f, &l, &r))
        };
        memo[k] = Some(r.clone());
        r
    }
    let mut r = get(f, n, &mut memo, &f2);
    poly::trim(f, &mut r);
    r
}

/// Power sums p1, p2, p3 of the roots of a monic h of degree >= 3 (padded with zero roots
/// otherwise), by Newton's identities (no division).
fn power_sums<F: Field>(f: &F, h: &Poly<F>) -> (F::E, F::E, F::E) {
    let d = h.len() - 1;
    let coef = |k: usize| if k <= d { h[d - k] } else { f.zero() };
    let e1 = f.neg(coef(1));
    let e2 = coef(2);
    let e3 = f.neg(coef(3));
    let c = |v: u64| f.from_u64(v);
    let p1 = e1;
    let p2 = f.sub(f.sq(e1), f.mul(c(2), e2));
    let p3 = f.add(
        f.sub(f.mul(e1, f.sq(e1)), f.mul(c(3), f.mul(e1, e2))),
        f.mul(c(3), e3),
    );
    (p1, p2, p3)
}

/// Codomain of the separable isogeny of odd degree 2 deg(h) + 1 whose kernel has abscissas the
/// roots of the monic h (Kohel's thesis, section 2.4, the odd case): with t = 6 p2 + b2 p1 + d b4
/// and w = 10 p3 + 2 b2 p2 + 3 b4 p1 + d b6, the codomain is
/// [a1, a2, a3, a4 - 5t, a6 - b2 t - 7w].
pub fn kohel_codomain<F: Field>(f: &F, e: &GWCurve<F::E>, h: &Poly<F>) -> GWCurve<F::E> {
    let c = |v: u64| f.from_u64(v);
    let d = c((h.len() - 1) as u64);
    let (b2, b4, b6, _) = e.b_invariants(f);
    let (p1, p2, p3) = power_sums(f, h);
    let t = f.add(f.add(f.mul(c(6), p2), f.mul(b2, p1)), f.mul(d, b4));
    let w = f.add(
        f.add(f.mul(c(10), p3), f.mul(c(2), f.mul(b2, p2))),
        f.add(f.mul(c(3), f.mul(b4, p1)), f.mul(d, b6)),
    );
    GWCurve::new([
        e.a1,
        e.a2,
        e.a3,
        f.sub(e.a4, f.mul(c(5), t)),
        f.sub(f.sub(e.a6, f.mul(b2, t)), f.mul(c(7), w)),
    ])
}

/// x-map (num, den) of that isogeny: x + sum_Q t_Q/(x - x_Q) + u_Q/(x - x_Q)^2 with
/// t_Q = 6 x_Q^2 + b2 x_Q + b4, u_Q = 4 x_Q^3 + b2 x_Q^2 + 2 b4 x_Q + b6. Using
/// sum g(x_Q)/(x - x_Q) = R_g/h with R_g = g h' mod h (h squarefree), it is
/// (x h^2 + R_t h - (R_u' h - R_u h')) / h^2.
pub fn x_map<F: Field>(f: &F, e: &GWCurve<F::E>, h: &Poly<F>) -> (Poly<F>, Poly<F>) {
    let c = |v: u64| f.from_u64(v);
    let (b2, b4, b6, _) = e.b_invariants(f);
    let hp = poly::derivative(f, h);
    let tq: Poly<F> = vec![b4, b2, c(6)];
    let uq: Poly<F> = vec![b6, f.mul(c(2), b4), b2, c(4)];
    let rt = poly::rem(f, &poly::mul(f, &tq, &hp), h);
    let ru = poly::rem(f, &poly::mul(f, &uq, &hp), h);
    let h2 = poly::mul(f, h, h);
    let mut num = poly::mul(f, &poly::x_poly(f), &h2);
    num = poly::add(f, &num, &poly::mul(f, &rt, h));
    let dru = poly::sub(
        f,
        &poly::mul(f, &poly::derivative(f, &ru), h),
        &poly::mul(f, &ru, &hp),
    );
    num = poly::sub(f, &num, &dru);
    (num, h2)
}

/// x(2P) as a function of x(P): (x^4 - b4 x^2 - 2 b6 x - b8) / (4x^3 + b2 x^2 + 2 b4 x + b6);
/// None at 2-torsion.
pub fn x_double<F: Field>(f: &F, e: &GWCurve<F::E>, x: F::E) -> Option<F::E> {
    let c = |v: u64| f.from_u64(v);
    let (b2, b4, b6, b8) = e.b_invariants(f);
    let x2 = f.sq(x);
    let num = f.sub(
        f.sub(f.sq(x2), f.mul(b4, x2)),
        f.add(f.mul(c(2), f.mul(b6, x)), b8),
    );
    let den = f.add(
        f.add(f.mul(c(4), f.mul(x2, x)), f.mul(b2, x2)),
        f.add(f.mul(c(2), f.mul(b4, x)), b6),
    );
    if f.is_zero(den) {
        None
    } else {
        Some(f.div(num, den))
    }
}

/// Is the monic h (degree (l-1)/2) the kernel polynomial of an l-isogeny from e (l odd)?
/// h must divide the l-division polynomial, and its x-map must commute with doubling,
/// phi(x(2P)) = x(2 phi(P)) on the Kohel codomain, for the abscissas `xs` supplied (rational
/// points, say; abscissas of kernel points are skipped).
pub fn is_kernel<F: Field>(f: &F, e: &GWCurve<F::E>, h: &Poly<F>, xs: &[F::E]) -> bool {
    let ell = 2 * (h.len() - 1) + 1;
    let psi = division_poly(f, e, ell);
    if !poly::rem(f, &psi, h).iter().all(|&c| f.is_zero(c)) {
        return false;
    }
    let cod = kohel_codomain(f, e, h);
    if f.is_zero(cod.disc(f)) {
        return false;
    }
    let (num, den) = x_map(f, e, h);
    let phi = |x: F::E| -> Option<F::E> {
        let dv = poly::eval(f, &den, x);
        if f.is_zero(dv) {
            None
        } else {
            Some(f.div(poly::eval(f, &num, x), dv))
        }
    };
    for &x in xs {
        let (Some(x2), Some(px)) = (x_double(f, e, x), phi(x)) else {
            continue;
        };
        let (Some(lhs), Some(rhs)) = (phi(x2), x_double(f, &cod, px)) else {
            continue;
        };
        if lhs != rhs {
            return false;
        }
    }
    true
}

/// Abscissas of up to `k` random F_q-points of e (odd characteristic: y from a square root).
pub fn random_abscissas<F: Field>(f: &F, e: &GWCurve<F::E>, k: usize, rng: &mut Rng) -> Vec<F::E> {
    let c = |v: u64| f.from_u64(v);
    let mut out = vec![];
    for _ in 0..40 * k {
        if out.len() == k {
            break;
        }
        let x = f.random(rng);
        // (2y + a1 x + a3)^2 = 4x^3 + b2 x^2 + 2 b4 x + b6
        let (b2, b4, b6, _) = e.b_invariants(f);
        let x2 = f.sq(x);
        let r = f.add(
            f.add(f.mul(c(4), f.mul(x2, x)), f.mul(b2, x2)),
            f.add(f.mul(c(2), f.mul(b4, x)), b6),
        );
        if f.sqrt(r).is_some() {
            out.push(x);
        }
    }
    out
}

/// All F_q-rational kernel polynomials of l-isogenies from e (l odd prime, char != l): products
/// of irreducible factors of the l-division polynomial of total degree (l - 1)/2 that pass
/// [`is_kernel`] (independent reference for the kernel-finding algorithms).
pub fn kernel_polys<F: Field>(f: &F, e: &GWCurve<F::E>, ell: u64, rng: &mut Rng) -> Vec<Poly<F>> {
    let psi = poly::monic(f, &division_poly(f, e, ell as usize));
    let facs = poly::factor_squarefree(f, &psi, rng);
    let d = ((ell - 1) / 2) as usize;
    let xs = random_abscissas(f, e, 8, rng);
    let mut out = vec![];
    let mut stack = vec![(0usize, d, vec![f.one()])];
    while let Some((i, left, h)) = stack.pop() {
        if left == 0 {
            if is_kernel(f, e, &h, &xs) {
                out.push(h);
            }
            continue;
        }
        if i == facs.len() {
            continue;
        }
        let k = facs[i].len() - 1;
        if k <= left {
            stack.push((i + 1, left - k, poly::mul(f, &h, &facs[i])));
        }
        stack.push((i + 1, left, h));
    }
    out
}
