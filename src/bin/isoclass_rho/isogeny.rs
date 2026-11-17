//! Rational `ℓ`-isogenies of a curve over `F_q`, found two ways, with the
//! explicit map on points from the kernel polynomial.
//!
//! * **Division route** (any characteristic, any `j`): factor the
//!   `ℓ`-division polynomial over `F_q` into the irreducible factors whose
//!   degree `d` divides `(ℓ − 1)/2`; a root `x(T)` of such a factor lives
//!   in `F_{q^d}`, and the x-only ladder gives `x(iT)`, `i ≤ (ℓ − 1)/2`,
//!   in that field.  `⟨T⟩` is rational exactly when the kernel polynomial
//!   `h = ∏ (x − x(iT))` has its coefficients in `F_q`.
//! * **Elkies route** (odd `q`, `j ∉ {0, 1728}`): the roots of
//!   `Φ_ℓ(j, Y)` are the neighbours' `j`-invariants, and the library's
//!   Elkies construction (`crypto_lib::cryptanalysis::isogeny_walk::kernel`)
//!   gives each kernel polynomial.
//!
//! Both routes end in the same [`Kernel`]: the monic kernel polynomial `h`
//! of degree `(ℓ − 1)/2`, Vélu's codomain, and the map on points.  Over
//! `F_p` the normalised map is `(X(x), y·X'(x))` with Kohel's
//! `X = ℓx − 2σ − 2(3x² + a)·h'/h − 4(x³ + ax + b)·(h'/h)'`.  Over
//! `F_{2^m}` (`y² + xy = x³ + ax² + b`), with `L = h'/h` and the Hasse
//! derivatives `D₂h`, `D₃h`,
//! `X = x + xL + x²L²`, `Y = y + x³L₃ + xL + (x + y + x²)(xL² + L)`,
//! `L₃ = L³ + L·D₂h/h + D₃h/h`, and the codomain is `(a, b + σ + σ²)`,
//! `σ = Σ_{Q∈S} x_Q`.  Every edge the walk uses is verified on points
//! afterwards (image on the codomain, homomorphism, order preserved).

use std::time::Instant;

use crypto_lib::cryptanalysis::isogeny_walk::curve::Model;
use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LibField;
use crypto_lib::cryptanalysis::isogeny_walk::modpoly::ModPoly;
use crypto_lib::cryptanalysis::isogeny_walk::{kernel as lkernel, poly as lpoly};
use num_bigint::BigUint;
use rand::rngs::SmallRng;

use super::field::{Curve, Fp, Fq, Pt, Ring};
use super::poly::{self, Ext, Poly};

/// One rational `ℓ`-isogeny from a curve: kernel polynomial, codomain, the
/// precomputed derivative polynomials, and how it was found.
#[derive(Clone, Debug)]
pub struct Kernel {
    pub ell: u64,
    /// Monic, degree `(ℓ − 1)/2` (`x − x₀` for `ℓ = 2` over `F_p`).
    pub h: Poly,
    pub codomain: Curve,
    /// `h'`, `h''`, `h'''` over `F_p`; `D₁h`, `D₂h`, `D₃h` over `F_{2^m}`.
    pub dh: [Poly; 3],
    /// `Σ_{Q ∈ S} x_Q` (half the kernel, one of each `±` pair).
    pub sigma: u64,
    /// Degree of the field the kernel's `x`-coordinates generate: the
    /// smallest Frobenius orbit among the roots of `h`.
    pub field_degree: usize,
    pub route: &'static str,
    /// Milliseconds spent finding this kernel (shared by the kernels found
    /// together).
    pub find_ms: f64,
}

/// `ℓ`-division polynomial `f_ℓ` (odd `ℓ`), a polynomial in `x` whose roots
/// are the `x`-coordinates of the non-zero `ℓ`-torsion.
pub fn division_poly<F: Fq>(f: &F, c: &Curve, ell: u64) -> Poly {
    assert!(ell >= 3 && ell % 2 == 1);
    let top = ell as usize + 2;
    let mut d: Vec<Poly> = Vec::with_capacity(top + 1);
    if f.char2() {
        // f₀ = 0, f₁ = 1, f₂ = x, f₃ = x⁴ + x³ + b, f₄ = x⁶ + bx²
        d.push(vec![]);
        d.push(vec![1]);
        d.push(vec![0, 1]);
        d.push(poly::trim(vec![c.b, 0, 0, 1, 1]));
        d.push(poly::trim(vec![0, 0, c.b, 0, 0, 0, 1]));
        for n in 5..=top {
            let k = n / 2;
            let v = if n % 2 == 1 {
                // f_{2k+1} = f_{k+2} f_k³ + f_{k−1} f_{k+1}³
                let t1 = poly::mul(f, &d[k + 2], &cube(f, &d[k]));
                let t2 = poly::mul(f, &d[k - 1], &cube(f, &d[k + 1]));
                poly::add(f, &t1, &t2)
            } else {
                // f_{2k} = f_k (f_{k+2} f_{k−1}² + f_{k−2} f_{k+1}²) / x
                let t1 = poly::mul(f, &d[k + 2], &poly::mul(f, &d[k - 1], &d[k - 1]));
                let t2 = poly::mul(f, &d[k - 2], &poly::mul(f, &d[k + 1], &d[k + 1]));
                let s = poly::mul(f, &d[k], &poly::add(f, &t1, &t2));
                let (q, r) = poly::divrem(f, &s, &vec![0, 1]);
                assert!(
                    r.is_empty(),
                    "char-2 division polynomial recursion: f_{{2k}} not divisible by x"
                );
                q
            };
            d.push(v);
        }
    } else {
        let (a, b) = (c.a, c.b);
        let a2 = f.fmul(a, a);
        let a3 = f.fmul(a2, a);
        let b2 = f.fmul(b, b);
        let ab = f.fmul(a, b);
        let s = |k: u64, v: u64| f.fmul(f.fsmall(k), v);
        // F = 4(x³ + ax + b)
        let big_f = poly::trim(vec![s(4, b), s(4, a), 0, f.fsmall(4)]);
        let f2sq = poly::mul(f, &big_f, &big_f);
        d.push(vec![]);
        d.push(vec![1]);
        d.push(vec![1]);
        // f₃ = 3x⁴ + 6ax² + 12bx − a²
        d.push(poly::trim(vec![
            f.fneg(a2),
            s(12, b),
            s(6, a),
            0,
            f.fsmall(3),
        ]));
        // f₄ = 2(x⁶ + 5ax⁴ + 20bx³ − 5a²x² − 4abx − 8b² − a³)
        d.push(poly::trim(vec![
            s(2, f.fneg(f.fadd(s(8, b2), a3))),
            s(2, f.fneg(s(4, ab))),
            s(2, f.fneg(s(5, a2))),
            s(2, s(20, b)),
            s(2, s(5, a)),
            0,
            f.fsmall(2),
        ]));
        for n in 5..=top {
            let k = n / 2;
            let v = if n % 2 == 1 {
                let mut t1 = poly::mul(f, &d[k + 2], &cube(f, &d[k]));
                let mut t2 = poly::mul(f, &d[k - 1], &cube(f, &d[k + 1]));
                if k % 2 == 0 {
                    t1 = poly::mul(f, &t1, &f2sq);
                } else {
                    t2 = poly::mul(f, &t2, &f2sq);
                }
                poly::sub(f, &t1, &t2)
            } else {
                let t1 = poly::mul(f, &d[k + 2], &poly::mul(f, &d[k - 1], &d[k - 1]));
                let t2 = poly::mul(f, &d[k - 2], &poly::mul(f, &d[k + 1], &d[k + 1]));
                poly::mul(f, &d[k], &poly::sub(f, &t1, &t2))
            };
            d.push(v);
        }
    }
    poly::monic(f, &d[ell as usize])
}

fn cube<F: Fq>(f: &F, a: &Poly) -> Poly {
    poly::mul(f, a, &poly::mul(f, a, a))
}

/// `x(kP)` from `x(P)` by the x-only ladder in any ring; `None` if some
/// multiple hit `O` or a coincidence stopped the ladder.
pub fn x_ladder<F: Fq, R: Ring>(
    f: &F,
    r: &R,
    c: &Curve,
    x1: &R::El,
    k: usize,
) -> Option<Vec<R::El>> {
    // xs[i] = x((i+1)P)
    let mut xs = vec![x1.clone()];
    if k >= 2 {
        xs.push(f.x_dbl(r, c, x1)?);
    }
    for i in 2..k {
        let next = f.x_sum(r, c, &xs[i - 1], x1, &xs[i - 2])?;
        xs.push(next);
    }
    Some(xs)
}

/// Vélu's codomain and the derivative data from a kernel polynomial.
pub fn finish_kernel<F: Fq>(
    f: &F,
    c: &Curve,
    ell: u64,
    h: &Poly,
    route: &'static str,
    find_ms: f64,
) -> Kernel {
    let h = poly::monic(f, h);
    let d = poly::degree(&h).expect("kernel polynomial");
    let sigma = if d == 0 { 0 } else { f.fneg(h[d - 1]) }; // sum of roots
    let codomain = if f.char2() {
        Curve {
            a: c.a,
            b: f.fadd(f.fadd(c.b, sigma), f.fmul(sigma, sigma)),
        }
    } else if ell == 2 {
        let x0 = f.fneg(h[0]);
        let t = f.fadd(f.fmul(f.fsmall(3), f.fmul(x0, x0)), c.a);
        Curve {
            a: f.fsub(c.a, f.fmul(f.fsmall(5), t)),
            b: f.fsub(c.b, f.fmul(f.fsmall(7), f.fmul(x0, t))),
        }
    } else {
        velu_codomain_odd(f, c, &h)
    };
    let dh = if f.char2() {
        [
            poly::hasse(f, &h, 1),
            poly::hasse(f, &h, 2),
            poly::hasse(f, &h, 3),
        ]
    } else {
        let h1 = poly::derivative(f, &h);
        let h2 = poly::derivative(f, &h1);
        let h3 = poly::derivative(f, &h2);
        [h1, h2, h3]
    };
    let field_degree = if d == 0 {
        1
    } else {
        poly::min_factor_degree(f, &h)
    };
    Kernel {
        ell,
        h,
        codomain,
        dh,
        sigma,
        field_degree,
        route,
        find_ms,
    }
}

/// Vélu's codomain over `F_p` for odd `ℓ` from the power sums of the kernel
/// polynomial's roots: `a' = a − 5t`, `b' = b − 7w` with `t = Σ t_Q`,
/// `w = Σ (u_Q + x_Q t_Q)` over `Q ∈ S`, `t_Q = 2(3x_Q² + a)`,
/// `u_Q = 4(x_Q³ + ax_Q + b)`.
fn velu_codomain_odd<F: Fq>(f: &F, c: &Curve, h: &Poly) -> Curve {
    let d = poly::degree(h).unwrap();
    // power sums s1 = Σx, s2 = Σx², s3 = Σx³ by Newton's identities on the monic h
    let e = |i: usize| -> u64 {
        // elementary symmetric e_i = (−1)^i h_{d−i}
        if i > d {
            return 0;
        }
        let v = h[d - i];
        if i % 2 == 1 {
            f.fneg(v)
        } else {
            v
        }
    };
    let s1 = e(1);
    let s2 = f.fsub(f.fmul(e(1), s1), f.fmul(f.fsmall(2), e(2)));
    let s3 = f.fadd(
        f.fsub(f.fmul(e(1), s2), f.fmul(e(2), s1)),
        f.fmul(f.fsmall(3), e(3)),
    );
    let dd = f.fsmall(d as u64);
    let (a, b) = (c.a, c.b);
    // t = Σ 2(3x² + a) = 6 s2 + 2 a d
    let t = f.fadd(f.fmul(f.fsmall(6), s2), f.fmul(f.fsmall(2), f.fmul(a, dd)));
    // u = Σ 4(x³ + a x + b) = 4 s3 + 4 a s1 + 4 b d
    let u = f.fmul(
        f.fsmall(4),
        f.fadd(f.fadd(s3, f.fmul(a, s1)), f.fmul(b, dd)),
    );
    // Σ x t_Q = Σ 2x(3x² + a) = 6 s3 + 2 a s1
    let xt = f.fadd(f.fmul(f.fsmall(6), s3), f.fmul(f.fsmall(2), f.fmul(a, s1)));
    let w = f.fadd(u, xt);
    Curve {
        a: f.fsub(a, f.fmul(f.fsmall(5), t)),
        b: f.fsub(b, f.fmul(f.fsmall(7), w)),
    }
}

impl Kernel {
    /// The isogeny applied to a point of the source curve.
    pub fn map<F: Fq>(&self, f: &F, c: &Curve, p: Pt) -> Pt {
        let (x, y) = p?;
        let hx = poly::eval(f, &self.h, x);
        if hx == 0 {
            return None; // in the kernel
        }
        let hi = f.finv(hx);
        let d1 = f.fmul(poly::eval(f, &self.dh[0], x), hi);
        let d2 = f.fmul(poly::eval(f, &self.dh[1], x), hi);
        let d3 = f.fmul(poly::eval(f, &self.dh[2], x), hi);
        if f.char2() {
            let l = d1;
            let l2 = f.fsqr(l);
            let l3 = f.fadd(f.fadd(f.fmul(l2, l), f.fmul(l, d2)), d3);
            let x2 = f.fsqr(x);
            let xx = f.fadd(f.fadd(x, f.fmul(x, l)), f.fmul(x2, l2));
            let x3 = f.fmul(x2, x);
            let t = f.fadd(f.fmul(x, l2), l);
            let yy = f.fadd(
                f.fadd(f.fadd(y, f.fmul(x3, l3)), f.fmul(x, l)),
                f.fmul(f.fadd(f.fadd(x, y), x2), t),
            );
            return Some((xx, yy));
        }
        if self.ell == 2 {
            let x0 = f.fneg(self.h[0]);
            let t = f.fadd(f.fmul(f.fsmall(3), f.fmul(x0, x0)), c.a);
            let i = f.finv(f.fsub(x, x0));
            let xx = f.fadd(x, f.fmul(t, i));
            let yy = f.fmul(y, f.fsub(1, f.fmul(t, f.fmul(i, i))));
            return Some((xx, yy));
        }
        let l = d1;
        let lp = f.fsub(d2, f.fmul(l, l));
        let lpp = f.fsub(
            f.fsub(d3, f.fmul(l, d2)),
            f.fmul(f.fsmall(2), f.fmul(l, lp)),
        );
        let x2 = f.fmul(x, x);
        let big_f = f.fadd(f.fmul(f.fadd(x2, c.a), x), c.b);
        let fp = f.fadd(f.fmul(f.fsmall(3), x2), c.a);
        let fpp = f.fmul(f.fsmall(6), x);
        let ell = f.fsmall(self.ell);
        let xx = f.fsub(
            f.fsub(
                f.fsub(f.fmul(ell, x), f.fmul(f.fsmall(2), self.sigma)),
                f.fmul(f.fsmall(2), f.fmul(fp, l)),
            ),
            f.fmul(f.fsmall(4), f.fmul(big_f, lp)),
        );
        let xp = f.fsub(
            f.fsub(
                f.fsub(ell, f.fmul(f.fsmall(2), f.fmul(fpp, l))),
                f.fmul(f.fsmall(6), f.fmul(fp, lp)),
            ),
            f.fmul(f.fsmall(4), f.fmul(big_f, lpp)),
        );
        Some((xx, f.fmul(y, xp)))
    }
}

/// Rational `ℓ`-kernels by the division route (odd `ℓ`, any `j`, any
/// characteristic).
pub fn kernels_division<F: Fq>(f: &F, c: &Curve, ell: u64, rng: &mut SmallRng) -> Vec<Kernel> {
    let t0 = Instant::now();
    let psi = division_poly(f, c, ell);
    assert!(
        poly::is_squarefree(f, &psi),
        "division polynomial not squarefree"
    );
    let half = (ell as usize - 1) / 2;
    let (parts, _) = poly::ddf(f, &psi, half);
    let mut found: Vec<Poly> = Vec::new();
    for (d, g) in parts {
        if !half.is_multiple_of(d) {
            continue;
        }
        let mut irr = Vec::new();
        poly::edf(f, &g, d, rng, &mut irr);
        for gi in irr {
            let ext = Ext::new(f, &gi);
            let xt = ext.gen();
            let Some(xs) = x_ladder(f, &ext, c, &xt, half + 1) else {
                continue;
            };
            // order ℓ: x(((ℓ+1)/2) T) = x(((ℓ−1)/2) T)
            if xs[half] != xs[half - 1] {
                eprintln!("isoclass-rho: warning: a root of f_{ell} is not ℓ-torsion (ladder check failed)");
                continue;
            }
            let hp = poly::product_of_linear(&ext, &xs[..half]);
            let mut h: Poly = Vec::with_capacity(hp.len());
            let mut rational = true;
            for coeff in &hp {
                match ext.to_base(coeff) {
                    Some(v) => h.push(v),
                    None => {
                        rational = false;
                        break;
                    }
                }
            }
            if !rational {
                continue;
            }
            let h = poly::trim(h);
            if !found.contains(&h) {
                found.push(h);
            }
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    found
        .into_iter()
        .map(|h| finish_kernel(f, c, ell, &h, "division", ms))
        .collect()
}

/// Rational 2-kernels over `F_p`: the roots of `x³ + ax + b`.
pub fn kernels_two(f: &Fp, c: &Curve, rng: &mut SmallRng) -> Vec<Kernel> {
    let t0 = Instant::now();
    let cubic = poly::trim(vec![c.b, c.a, 0, 1]);
    let rs = poly::roots(f, &cubic, rng);
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    rs.into_iter()
        .map(|x0| finish_kernel(f, c, 2, &vec![f.fneg(x0), 1], "cubic", ms))
        .collect()
}

/// The library's big-integer field and model for `(p, a, b)`.
pub fn lib_field(p: u64) -> LibField {
    LibField::new(&BigUint::from(p)).expect("p prime")
}

fn fe_to_u64(lf: &LibField, v: &crypto_lib::cryptanalysis::isogeny_walk::field::Fe) -> u64 {
    lf.to_big(v).iter_u64_digits().next().unwrap_or(0)
}

/// Neighbour `j`-invariants with multiplicity from `Φ_ℓ(j, Y)`.
pub fn phi_neighbours(f: &Fp, lf: &LibField, phi: &ModPoly, j: u64, seed: u64) -> Vec<u64> {
    let jfe = lf.from_u64(j);
    let py = phi.at_x(lf, &jfe);
    let roots = lpoly::roots(lf, &py, seed);
    // multiplicity: divide out each root while it divides
    let mut out = Vec::new();
    let mut cur = py.clone();
    for r in roots {
        let lin = vec![lf.neg(&r), lf.one()];
        loop {
            let (q, rem) = lpoly::divrem(lf, &cur, &lin);
            if lpoly::degree(&rem).is_some() {
                break;
            }
            out.push(fe_to_u64(lf, &r));
            cur = q;
        }
    }
    let _ = f;
    out
}

/// Rational `ℓ`-kernels over `F_p` by the Elkies route.  `Err` when the
/// construction declines (special `j`, a multiple root): the caller falls
/// back to the division route.
pub fn kernels_elkies(
    f: &Fp,
    lf: &LibField,
    phi: &ModPoly,
    c: &Curve,
    seed: u64,
) -> Result<Vec<Kernel>, String> {
    let t0 = Instant::now();
    let j = f.ec_j(c).ok_or("singular")?;
    let model = Model {
        a: lf.from_u64(c.a),
        b: lf.from_u64(c.b),
    };
    let jfe = lf.from_u64(j);
    let py = phi.at_x(lf, &jfe);
    let roots = lpoly::roots(lf, &py, seed);
    let mut out = Vec::new();
    for r in roots {
        let iso = lkernel::elkies_isogeny(lf, &model, phi, &r).map_err(|e| format!("{e:?}"))?;
        let h: Poly = iso.kernel.iter().map(|v| fe_to_u64(lf, v)).collect();
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        let k = finish_kernel(f, c, phi.ell, &h, "elkies", ms);
        let cod = Curve {
            a: fe_to_u64(lf, &iso.codomain.a),
            b: fe_to_u64(lf, &iso.codomain.b),
        };
        if cod != k.codomain {
            return Err("Vélu codomain from h differs from the Elkies codomain".into());
        }
        out.push(k);
    }
    // a multiple root means two kernels with one codomain j: Elkies cannot
    // separate them, so the division route must
    let with_mult = phi_neighbours(f, lf, phi, j, seed).len();
    if with_mult != out.len() {
        return Err(format!(
            "{with_mult} neighbours with multiplicity, {} simple roots",
            out.len()
        ));
    }
    Ok(out)
}

/// Check an isogeny on points: images on the codomain, additive, and
/// `n·φ(P) = O ≠ φ(P)`.  Returns an error string on the first failure.
pub fn verify_map<F: Fq>(
    f: &F,
    c: &Curve,
    k: &Kernel,
    pts: &[(u64, u64)],
    n: u64,
) -> Result<(), String> {
    let cod = k.codomain;
    if f.ec_j(&cod).is_none() {
        return Err("codomain is singular".into());
    }
    for (i, &p) in pts.iter().enumerate() {
        let ip = k.map(f, c, Some(p));
        if !f.ec_on(&cod, ip) {
            return Err(format!("image of point {i} is off the codomain"));
        }
        if ip.is_none() {
            return Err(format!("point {i} maps to O"));
        }
        if f.ec_mul(&cod, ip, n).is_some() {
            return Err(format!("n·φ(P{i}) ≠ O"));
        }
        for &q in &pts[i + 1..] {
            let iq = k.map(f, c, Some(q));
            let sum = k.map(f, c, f.ec_add(c, Some(p), Some(q)));
            if f.ec_add(&cod, ip, iq) != sum {
                return Err("φ(P + Q) ≠ φ(P) + φ(Q)".into());
            }
        }
    }
    Ok(())
}

// ================================================================== torsion route

use num_bigint::BigInt;
use num_traits::{One, Zero};

/// `#E(F_{q^d})` and `#E'(F_{q^d})` (the quadratic twist) from the trace
/// `t` over `F_q`: `s_i = α^i + β^i` with `s_0 = 2`, `s_1 = t`,
/// `s_{i+1} = t·s_i − q·s_{i−1}`.
pub fn extension_orders(q: u64, t: i64, d: usize) -> (BigInt, BigInt) {
    let qb = BigInt::from(q);
    let tb = BigInt::from(t);
    let (mut s0, mut s1) = (BigInt::from(2), tb.clone());
    for _ in 1..d {
        let s2 = &tb * &s1 - &qb * &s0;
        s0 = s1;
        s1 = s2;
    }
    if d == 0 {
        s1 = BigInt::from(2);
    }
    let qd = num_traits::pow::pow(qb, d);
    let one = BigInt::one();
    (&qd + &one - &s1, &qd + &one + &s1)
}

fn big_bits(n: &BigInt) -> Vec<bool> {
    let (_, digits) = n.to_u64_digits();
    let mut bits: Vec<bool> = Vec::new();
    for w in digits.iter().rev() {
        for i in (0..64).rev() {
            bits.push((w >> i) & 1 == 1);
        }
    }
    let first = bits.iter().position(|&b| b).unwrap_or(bits.len());
    bits[first..].to_vec()
}

/// Rational `ℓ`-kernels by the torsion route: the `x`-coordinates of
/// `E[ℓ]` lie in `F_{q^d}`, `d = ord(±λ)` for the Frobenius eigenvalue `λ`
/// (every `ℓ`-subgroup is rational when `π ≡ λ` is a scalar mod `ℓ`, as it
/// is off the floor of the `ℓ`-volcano for `ℓ | f`).  A random `x ∈
/// F_{q^d}` is the abscissa of a point of `E` or of its twist over
/// `F_{q^d}`; the x-only ladder by the matching cofactor gives an
/// `ℓ`-torsion abscissa, its multiples give the kernel polynomial, whose
/// coefficients must land in `F_q`.  Finds up to `count` distinct kernels
/// (a sample of the `ℓ + 1`), each verified on points by the caller.
/// What the torsion route keeps for the dual edges: the extension modulus
/// and one ℓ-torsion abscissa (in `F_q[y]/(g)`) per kernel found.
#[derive(Clone, Debug)]
pub struct TorsionCtx {
    pub g: Poly,
    pub xs: Vec<Poly>,
}

/// The kernel polynomial of `⟨T⟩` from the abscissae `xs[i] = x((i+1)T)`,
/// `i < (ℓ−1)/2`, as the product over Frobenius orbits `{x(±iλ^j T)}`:
/// each orbit's polynomial has `F_q` coefficients (`None` if one does not,
/// i.e. `⟨T⟩` is not rational or `λ` is not its eigenvalue).
fn kernel_from_orbits<F: Fq>(f: &F, ext: &Ext<F>, xs: &[Poly], ell: u64, lam: u64) -> Option<Poly> {
    let half = (ell as usize - 1) / 2;
    let idx = |k: u64| -> usize {
        let k = k % ell;
        let k = if k > ell / 2 { ell - k } else { k };
        k as usize - 1
    };
    let mut seen = vec![false; half];
    let mut h: Poly = vec![1];
    for i in 1..=half as u64 {
        if seen[i as usize - 1] {
            continue;
        }
        let mut orbit: Vec<Poly> = Vec::new();
        let mut k = i;
        loop {
            let ix = idx(k);
            if seen[ix] {
                break;
            }
            seen[ix] = true;
            orbit.push(xs[ix].clone());
            k = k * lam % ell;
        }
        let op = poly::product_of_linear(ext, &orbit);
        let mut ob: Poly = Vec::with_capacity(op.len());
        for coeff in &op {
            ob.push(ext.to_base(coeff)?);
        }
        h = poly::mul(f, &h, &poly::trim(ob));
    }
    Some(poly::trim(h))
}

pub fn kernels_torsion<F: Fq>(
    f: &F,
    c: &Curve,
    ell: u64,
    t: i64,
    q: u64,
    d: usize,
    count: usize,
    rng: &mut SmallRng,
) -> (Vec<Kernel>, TorsionCtx) {
    let t0 = Instant::now();
    let gcoef = poly::irreducible_f2(d);
    let g: Poly = poly::trim(gcoef.iter().map(|&b| f.fsmall(b)).collect());
    let ext = Ext::new(f, &g);
    let (m_e, m_tw) = extension_orders(q, t, d);
    let lam = frobenius_scalar(t, q, ell);
    eprintln!(
        "    torsion ℓ = {ell}: F_q^{d} built in {:.2}s (λ = {lam})",
        t0.elapsed().as_secs_f64()
    );
    let ellb = BigInt::from(ell);
    let half = (ell as usize - 1) / 2;
    let mut found: Vec<Poly> = Vec::new();
    let mut found_x: Vec<Poly> = Vec::new();
    let mut attempts = 0;
    while found.len() < count && attempts < 40 * count {
        attempts += 1;
        let x: Poly = poly::trim((0..d).map(|_| f.frandom(rng)).collect());
        if x.is_empty() {
            continue;
        }
        'orders: for m in [&m_e, &m_tw] {
            let mut v = 0u32;
            let mut cof = m.clone();
            while (&cof % &ellb).is_zero() {
                cof /= &ellb;
                v += 1;
            }
            if v == 0 {
                continue;
            }
            let t1 = Instant::now();
            let Some(mut y) = poly::x_mul_bits(f, &ext, c, &x, &big_bits(&cof)) else {
                continue;
            };
            eprintln!(
                "    torsion ℓ = {ell}: ladder of {} bits in {:.2}s (attempt {attempts}, v = {v})",
                big_bits(&cof).len(),
                t1.elapsed().as_secs_f64()
            );
            for _ in 0..v {
                // ℓ-torsion test: x(((ℓ+1)/2)T) = x(((ℓ−1)/2)T)
                if let Some(xs) = x_ladder(f, &ext, c, &y, half + 1) {
                    if xs[half] == xs[half - 1] {
                        match kernel_from_orbits(f, &ext, &xs[..half], ell, lam) {
                            Some(h) => {
                                if !found.contains(&h) {
                                    found.push(h);
                                    found_x.push(y.clone());
                                }
                            }
                            None => eprintln!("isoclass-rho: warning: torsion route found an ℓ-subgroup whose kernel polynomial is not over F_q (λ = {lam})"),
                        }
                        break 'orders;
                    }
                }
                match poly::x_mul_bits(f, &ext, c, &y, &big_bits(&ellb)) {
                    Some(z) => y = z,
                    None => break,
                }
            }
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    let ks = found
        .into_iter()
        .map(|h| finish_kernel(f, c, ell, &h, "torsion", ms))
        .collect();
    (ks, TorsionCtx { g, xs: found_x })
}

/// Evaluate an `F_q[x]` polynomial at a ring element.
fn eval_in<R: Ring>(r: &R, a: &Poly, x: &R::El) -> R::El {
    let mut acc = r.zero();
    for &c in a.iter().rev() {
        acc = r.add(&r.mul(&acc, x), &r.lift(c));
    }
    acc
}

/// The `x`-map of a kernel applied to an abscissa in an extension ring.
fn x_map_in<F: Fq, R: Ring>(f: &F, r: &R, c: &Curve, k: &Kernel, x: &R::El) -> Option<R::El> {
    let hx = eval_in(r, &k.h, x);
    let hi = r.inv(&hx)?;
    let d1 = r.mul(&eval_in(r, &k.dh[0], x), &hi);
    if f.char2() {
        let l = d1;
        let x2 = r.sqr(x);
        Some(r.add(&r.add(x, &r.mul(x, &l)), &r.mul(&x2, &r.sqr(&l))))
    } else {
        let d2 = r.mul(&eval_in(r, &k.dh[1], x), &hi);
        let l = d1;
        let lp = r.sub(&d2, &r.sqr(&l));
        let x2 = r.sqr(x);
        let big_f = r.add(&r.mul(&r.add(&x2, &r.lift(c.a)), x), &r.lift(c.b));
        let fp = r.add(&r.mul(&r.small(3), &x2), &r.lift(c.a));
        let ell = r.small(k.ell);
        Some(r.sub(
            &r.sub(
                &r.sub(&r.mul(&ell, x), &r.mul(&r.small(2), &r.lift(k.sigma))),
                &r.mul(&r.small(2), &r.mul(&fp, &l)),
            ),
            &r.mul(&r.small(4), &r.mul(&big_f, &lp)),
        ))
    }
}

/// The dual direction of a sampled edge `φ: E → E'` (kernel `k`): push an
/// ℓ-torsion abscissa of `E` outside `ker φ` through `φ`; its image
/// generates `ker φ̂`, and the Frobenius-orbit product gives the kernel
/// polynomial on `E'`.  Needs a second sampled kernel in `ctx`.
pub fn dual_torsion<F: Fq>(
    f: &F,
    c: &Curve,
    k: &Kernel,
    ctx: &TorsionCtx,
    t: i64,
    q: u64,
) -> Result<Kernel, String> {
    let t0 = Instant::now();
    let ell = k.ell;
    let ext = Ext::new(f, &ctx.g);
    let half = (ell as usize - 1) / 2;
    let lam = frobenius_scalar(t, q, ell);
    let other = ctx
        .xs
        .iter()
        .find(|x| !ext.is_zero(&eval_in(&ext, &k.h, x)))
        .ok_or("no ℓ-torsion abscissa outside the kernel in the torsion context")?;
    let y = x_map_in(f, &ext, c, k, other).ok_or("image abscissa undefined")?;
    let cod = k.codomain;
    let xs = x_ladder(f, &ext, &cod, &y, half + 1).ok_or("ladder on the codomain failed")?;
    if xs[half] != xs[half - 1] {
        return Err("image of the torsion abscissa is not ℓ-torsion".into());
    }
    let h = kernel_from_orbits(f, &ext, &xs[..half], ell, lam)
        .ok_or("dual kernel polynomial is not over F_q")?;
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    Ok(finish_kernel(f, &cod, ell, &h, "torsion-dual", ms))
}

/// The Frobenius scalar `λ ≡ t/2 (mod ℓ)` on `E[ℓ]` when `ℓ | f` off the
/// floor (`π ≡ λ`), else a root of `X² − tX + q`.
pub fn frobenius_scalar(t: i64, q: u64, ell: u64) -> u64 {
    let tm = t.rem_euclid(ell as i64) as u64;
    let qm = q % ell;
    let disc = (tm * tm % ell + 4 * (ell - qm) % ell) % ell;
    if disc == 0 {
        return tm * super::field::inv_mod(2, ell) % ell;
    }
    for x in 0..ell {
        if (x * x % ell + ell - tm * x % ell) % ell == (ell - qm) % ell {
            return x;
        }
    }
    1
}

/// `ord(±λ)` in `(Z/ℓ)^*/±1` for the Frobenius eigenvalue `λ = t/2 mod ℓ`
/// (the scalar case, `ℓ | f` off the floor), else the orders of the two
/// eigenvalues' classes.  Returns the degree the kernel abscissae need.
pub fn eigen_orbit(t: i64, q: u64, ell: u64) -> usize {
    let tm = t.rem_euclid(ell as i64) as u64;
    let qm = q % ell;
    // roots of X² − tX + q mod ℓ
    let mut roots = Vec::new();
    for x in 0..ell {
        if (x * x % ell + ell - tm * x % ell) % ell == (ell - qm) % ell {
            roots.push(x);
        }
    }
    let ord_pm = |lam: u64| -> usize {
        let mut x = 1u64;
        for k in 1..ell as usize {
            x = x * lam % ell;
            if x == 1 || x == ell - 1 {
                return k;
            }
        }
        ell as usize - 1
    };
    roots
        .iter()
        .map(|&r| ord_pm(r))
        .max()
        .unwrap_or(ell as usize - 1)
}
