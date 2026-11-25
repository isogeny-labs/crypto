//! Certified canonical-height enclosures on y^2 = x^3 + A x + B (A, B in Z).
//!
//! Normalisation: hhat(P) = lim h(x(2^n P)) / 4^n with h(m/d) = log max(|m|,|d|)
//! (the LMFDB/Sage normalisation, hhat ~ h(x)).
//!
//! For coprime (X, Z), the duplication x(2P) = F/G with
//!   F = X^4 - 2A X^2 Z^2 - 8B X Z^3 + A^2 Z^4,  G = 4(X^3 Z + A X Z^3 + B Z^4).
//! Upper:  max(|F|,|G|) <= c_up H^4, c_up = max(||F||_1, ||G||_1), H = max(|X|,|Z|).
//! Lower:  f1 F + g1 G = 4D Z^7, f2 F + g2 G = 4D X^7 (D = 4A^3 + 27B^2; the identities
//!   are checked exactly for every curve), so max(|F|,|G|) >= 4|D| H^4 / C with
//!   C = max(||f1||_1+||g1||_1, ||f2||_1+||g2||_1), and gcd(F,G) | 4D. Hence
//!   h(2P) - 4h(P) in [-log C, log c_up] and, telescoping,
//!   hhat(P) - h(P) in [-log C / 3, log c_up / 3].
//! The gcd is computed as gcd(4D, F mod 4D, G mod 4D), which is exact because
//! gcd(F, G) divides 4D.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

use crate::ec::{Pt, Short};

/// Rigorous (lower, upper) bounds on ln(n) for n >= 1.
pub fn ln_bounds(n: &BigUint) -> (f64, f64) {
    assert!(!n.is_zero());
    let bits = n.bits();
    let ln2 = std::f64::consts::LN_2;
    let (lo, hi) = if bits <= 53 {
        let v = n.iter_u64_digits().next().unwrap_or(0) as f64;
        let l = v.ln();
        (l, l)
    } else {
        let k = bits - 53;
        let m: BigUint = n >> k;
        let mf = m.iter_u64_digits().next().unwrap() as f64; // exact, < 2^53
        ((mf).ln() + (k as f64) * ln2, (mf + 1.0).ln() + (k as f64) * ln2)
    };
    // outward slack covering libm and f64 rounding (relative 1e-13 plus absolute 1e-13)
    (lo - lo.abs() * 1e-13 - 1e-13, hi + hi.abs() * 1e-13 + 1e-13)
}

type Poly = Vec<BigInt>; // homogeneous form: coefficient i multiplies X^(deg-i) Z^i

fn pmul(a: &Poly, b: &Poly) -> Poly {
    let mut r = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    r
}
fn padd(a: &Poly, b: &Poly) -> Poly {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}
fn l1(a: &Poly) -> BigInt {
    a.iter().map(|x| x.abs()).sum()
}

pub struct HeightCtx {
    pub a: BigInt,
    pub b: BigInt,
    pub four_d: BigInt,
    pub eps_plus: f64,  // upper bound of log c_up
    pub eps_minus: f64, // upper bound of log C
}

impl HeightCtx {
    pub fn new(e: &Short) -> HeightCtx {
        let a = e.a.clone();
        let b = e.b.clone();
        let d = e.disc4();
        assert!(!d.is_zero(), "singular curve");
        let i = |v: i64| BigInt::from(v);
        let a2 = &a * &a;
        let a3 = &a2 * &a;
        let b2 = &b * &b;
        let f: Poly = vec![i(1), i(0), i(-2) * &a, i(-8) * &b, a2.clone()];
        let g: Poly = vec![i(0), i(4), i(0), i(4) * &a, i(4) * &b];
        let f1: Poly = vec![i(0), i(12), i(0), i(16) * &a];
        let g1: Poly = vec![i(-3), i(0), i(5) * &a, i(27) * &b];
        let f2: Poly = vec![
            i(4) * &d,
            i(-4) * &a2 * &b,
            i(4) * &a * (i(3) * &a3 + i(22) * &b2),
            i(12) * &b * (&a3 + i(8) * &b2),
        ];
        let g2: Poly = vec![
            &a2 * &b,
            &a * (i(5) * &a3 + i(32) * &b2),
            i(2) * &b * (i(13) * &a3 + i(96) * &b2),
            i(-3) * &a2 * (&a3 + i(8) * &b2),
        ];
        // exact identity checks (degree 7 forms)
        let lhs1 = padd(&pmul(&f1, &f), &pmul(&g1, &g));
        let lhs2 = padd(&pmul(&f2, &f), &pmul(&g2, &g));
        let four_d = i(4) * &d;
        for k in 0..8 {
            let want1 = if k == 7 { four_d.clone() } else { BigInt::zero() };
            let want2 = if k == 0 { four_d.clone() } else { BigInt::zero() };
            assert_eq!(lhs1[k], want1, "identity f1F+g1G failed");
            assert_eq!(lhs2[k], want2, "identity f2F+g2G failed");
        }
        let c_up = std::cmp::max(l1(&f), l1(&g));
        let c_lo = std::cmp::max(&l1(&f1) + &l1(&g1), &l1(&f2) + &l1(&g2));
        let eps_plus = ln_bounds(c_up.magnitude()).1;
        let eps_minus = ln_bounds(c_lo.magnitude()).1;
        HeightCtx { a, b, four_d, eps_plus, eps_minus }
    }
}

/// Doubling-sequence state for one point.
pub struct Seq {
    pub x: BigInt,
    pub z: BigInt,
    pub level: u32,
    pub torsion: bool, // 2^k P = O for some k <= level
    pub start_height_bits: u64,
}

impl Seq {
    pub fn new(p: &Pt) -> Seq {
        match p {
            None => Seq { x: BigInt::one(), z: BigInt::zero(), level: 0, torsion: true, start_height_bits: 0 },
            Some((x, _)) => Seq {
                x: x.n.clone(),
                z: x.d.clone(),
                level: 0,
                torsion: false,
                start_height_bits: std::cmp::max(x.n.bits(), x.d.bits()),
            },
        }
    }
    pub fn bits(&self) -> u64 {
        std::cmp::max(self.x.bits(), self.z.bits())
    }
    pub fn double(&mut self, ctx: &HeightCtx) {
        if self.torsion {
            self.level += 1;
            return;
        }
        let (x, z) = (&self.x, &self.z);
        let x2 = x * x;
        let z2 = z * z;
        let xz = x * z;
        let x4 = &x2 * &x2;
        let x2z2 = &xz * &xz;
        let xz3 = &xz * &z2;
        let z4 = &z2 * &z2;
        let x3z = &xz * &x2;
        let f = &x4 - BigInt::from(2) * &ctx.a * &x2z2 - BigInt::from(8) * &ctx.b * &xz3 + &ctx.a * &ctx.a * &z4;
        let g = BigInt::from(4) * (&x3z + &ctx.a * &xz3 + &ctx.b * &z4);
        self.level += 1;
        if g.is_zero() {
            self.torsion = true;
            return;
        }
        let m = ctx.four_d.abs();
        let gg = m.gcd(&f.mod_floor(&m)).gcd(&g.mod_floor(&m));
        let (mut nf, mut ng) = if gg.is_one() { (f, g) } else { (f / &gg, g / &gg) };
        if ng.sign() == Sign::Minus {
            nf = -nf;
            ng = -ng;
        }
        debug_assert!(nf.gcd(&ng).is_one());
        self.x = nf;
        self.z = ng;
    }
    /// Certified enclosure of hhat at the current level.
    pub fn enclosure(&self, ctx: &HeightCtx) -> (f64, f64) {
        if self.torsion {
            return (0.0, 0.0);
        }
        let h = std::cmp::max(self.x.magnitude(), self.z.magnitude());
        let (lo, hi) = ln_bounds(h);
        let scale = 4f64.powi(self.level as i32);
        let l = ((lo - ctx.eps_minus / 3.0) / scale) * (1.0 - 1e-15);
        let u = ((hi + ctx.eps_plus / 3.0) / scale) * (1.0 + 1e-15) + 1e-300;
        (l.max(0.0), u)
    }
}

#[derive(Clone, Debug)]
pub struct RegResult {
    pub level: u32,
    pub h1: (f64, f64),
    pub h2: (f64, f64),
    pub reg_lo: f64,
    pub reg_hi: f64,
    pub certified_positive: bool,
    pub max_bits: u64,
}

/// Interval for the 2x2 regulator det [[h(P1), <P1,P2>],[<P1,P2>, h(P2)]],
/// <P,Q> = (h(P+Q) - h(P-Q)) / 4. Doubles adaptively through `levels`; stops
/// as soon as the lower end is > 0 (or after the last level, or if `stop_early`
/// is false, always runs to `levels.last()`).
pub fn regulator(
    e: &Short,
    ctx: &HeightCtx,
    p1: &Pt,
    p2: &Pt,
    levels: &[u32],
    bit_cap: u64,
    stop_early: bool,
) -> RegResult {
    let sum = e.add(p1, p2);
    let dif = e.add(p1, &e.neg(p2));
    let mut seqs = [Seq::new(p1), Seq::new(p2), Seq::new(&sum), Seq::new(&dif)];
    let mut last: Option<RegResult> = None;
    for &lv in levels {
        let mut capped = false;
        for s in seqs.iter_mut() {
            while s.level < lv {
                if s.bits() > bit_cap {
                    capped = true;
                    break;
                }
                s.double(ctx);
            }
        }
        if capped {
            break;
        }
        let enc: Vec<(f64, f64)> = seqs.iter().map(|s| s.enclosure(ctx)).collect();
        let (h1, h2, hp, hm) = (enc[0], enc[1], enc[2], enc[3]);
        let pair_lo = (hp.0 - hm.1) / 4.0;
        let pair_hi = (hp.1 - hm.0) / 4.0;
        let abs_max = pair_lo.abs().max(pair_hi.abs());
        let abs_min = if pair_lo <= 0.0 && pair_hi >= 0.0 { 0.0 } else { pair_lo.abs().min(pair_hi.abs()) };
        let t_lo = h1.0 * h2.0;
        let t_hi = h1.1 * h2.1;
        let slack = 1e-12 * (t_hi.abs() + abs_max * abs_max) + 1e-300;
        let reg_lo = t_lo - abs_max * abs_max - slack;
        let reg_hi = t_hi - abs_min * abs_min + slack;
        let max_bits = seqs.iter().map(|s| s.bits()).max().unwrap();
        let r = RegResult { level: lv, h1, h2, reg_lo, reg_hi, certified_positive: reg_lo > 0.0, max_bits };
        let done = r.certified_positive && stop_early;
        last = Some(r);
        if done {
            break;
        }
    }
    last.unwrap_or(RegResult {
        level: 0,
        h1: (0.0, f64::INFINITY),
        h2: (0.0, f64::INFINITY),
        reg_lo: f64::NEG_INFINITY,
        reg_hi: f64::INFINITY,
        certified_positive: false,
        max_bits: 0,
    })
}

/// Single-point enclosure at a fixed level (controls).
pub fn height_enclosure(ctx: &HeightCtx, p: &Pt, level: u32) -> (f64, f64) {
    let mut s = Seq::new(p);
    while s.level < level {
        s.double(ctx);
    }
    s.enclosure(ctx)
}
