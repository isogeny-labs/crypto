//! E_{a,b}: y^2 + xy = x^3 + a x^2 + b over F_{2^n}. Koblitz curves have b = 1 (ECC2K-130: a = 0).
//! `Group` is the prime-order subgroup G (order r = largest prime factor of #E, r^2 not dividing #E),
//! walked once: `dlog[x]` is the class logarithm k in 1..=(r-1)/2 with x(kg) = x (k and r-k give the same x).

use crate::field::Fq;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pt {
    Inf,
    Aff(u32, u32),
}

pub const NONE: u32 = u32::MAX;

pub struct Curve<'a> {
    pub f: &'a Fq,
    pub a: u32,
    pub b: u32,
}

impl<'a> Curve<'a> {
    pub fn koblitz(f: &'a Fq, a: u32) -> Curve<'a> {
        Curve { f, a, b: 1 }
    }
    pub fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(x, x ^ y),
        }
    }
    pub fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::Aff(x, y) => {
                let f = self.f;
                let lhs = f.sq(y) ^ f.mul(x, y);
                let rhs = f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b;
                lhs == rhs
            }
        }
    }
    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::Aff(x1, y1), Pt::Aff(x2, y2)) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == 0 {
                        return Pt::Inf;
                    }
                    let l = x1 ^ f.mul(y1, f.inv(x1));
                    let x3 = f.sq(l) ^ l ^ self.a;
                    let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                    Pt::Aff(x3, y3)
                } else {
                    let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                    let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ self.a;
                    let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                    Pt::Aff(x3, y3)
                }
            }
        }
    }
    pub fn mul(&self, p: Pt, mut k: u64) -> Pt {
        let (mut r, mut b) = (Pt::Inf, p);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    /// A y with (x,y) on the curve, or None.
    pub fn lift_x(&self, x: u32) -> Option<u32> {
        let f = self.f;
        if x == 0 {
            return Some(f.sqrt(self.b));
        }
        let c = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        if f.tr(c) == 1 {
            return None;
        }
        Some(f.mul(x, f.ht(c)))
    }
    /// #E(F_q) by trace counting (O, the point with x = 0, and two points per good x != 0).
    pub fn count(&self) -> u32 {
        let f = self.f;
        let mut c = 2u32;
        for x in 1..f.q {
            if f.tr(x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)))) == 0 {
                c += 2;
            }
        }
        c
    }
    /// Frobenius endomorphism sigma(P) = (x^2, y^2) (valid for b in F_2 as a map E -> E).
    pub fn frob_pt(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(self.f.sq(x), self.f.sq(y)),
        }
    }
}

pub fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut v = vec![];
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            v.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        v.push(n);
    }
    v
}

pub fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

/// What the plan needs without building the log table: (#E, r, ok-for-census).
pub fn curve_summary(f: &Fq, a: u32) -> (u32, u32, Result<(), String>) {
    let ne = Curve::koblitz(f, a).count();
    let r = *prime_factors(ne).last().unwrap();
    let chk = if (ne as u64) % ((r as u64) * (r as u64)) == 0 {
        Err(format!("r^2 divides #E={ne}"))
    } else if (r - 1) % f.n != 0 {
        Err(format!("n={} does not divide r-1 (r={r})", f.n))
    } else if r < 900 {
        Err(format!("r={r} < 900: group too small for the census"))
    } else {
        Ok(())
    };
    (ne, r, chk)
}

pub struct Group {
    pub n: u32,
    pub a: u32,
    pub q: u32,
    pub ne: u32,
    pub cof: u32,
    pub r: u32,
    /// Frobenius eigenvalue on G: sigma(P) = lam * P, order n in Z_r^*.
    pub lam: u32,
    pub g: Pt,
    /// dlog[x] in 1..=(r-1)/2 for x a class of G, else NONE.
    pub dlog: Vec<u32>,
    /// xs[d-1] = x of the class with logarithm d.
    pub xs: Vec<u32>,
}

impl Group {
    pub fn half(&self) -> u32 {
        (self.r - 1) / 2
    }
    #[inline]
    pub fn class(&self, v: u32) -> u32 {
        v.min(self.r - v)
    }
    pub fn mulmod(&self, a: u32, b: u32) -> u32 {
        ((a as u64 * b as u64) % self.r as u64) as u32
    }
    pub fn lam_pow(&self, j: u32) -> u32 {
        pow_mod(self.lam as u64, j as u64, self.r as u64) as u32
    }
    /// The point k*g.
    pub fn pt(&self, curve: &Curve, k: u32) -> Pt {
        curve.mul(self.g, k as u64)
    }
}

pub fn build_group(f: &Fq, a: u32) -> Result<Group, String> {
    let c = Curve::koblitz(f, a);
    let (ne, r, chk) = curve_summary(f, a);
    chk?;
    let cof = ne / r;
    let mut g = Pt::Inf;
    for x in 2..f.q {
        if let Some(y) = c.lift_x(x) {
            let cand = c.mul(Pt::Aff(x, y), cof as u64);
            if cand != Pt::Inf {
                g = cand;
                break;
            }
        }
    }
    if g == Pt::Inf {
        return Err("no generator of the prime-order subgroup found".into());
    }
    if c.mul(g, r as u64) != Pt::Inf {
        return Err("generator does not have order r".into());
    }
    let half = (r - 1) / 2;
    let mut dlog = vec![NONE; f.q as usize];
    let mut xs = Vec::with_capacity(half as usize);
    let mut p = g;
    for k in 1..=half {
        match p {
            Pt::Aff(x, _) => {
                if dlog[x as usize] != NONE {
                    return Err(format!(
                        "x repeated at k={k}: not a prime-order cyclic walk"
                    ));
                }
                dlog[x as usize] = k;
                xs.push(x);
            }
            Pt::Inf => return Err("walk hit O early".into()),
        }
        p = c.add(p, g);
    }
    // p = (half+1) g = -(half) g: same x as the last class.
    match p {
        Pt::Aff(x, _) if x == *xs.last().unwrap() => {}
        _ => return Err("walk did not close at (r+1)/2".into()),
    }
    // Frobenius eigenvalue.
    let gp = match g {
        Pt::Aff(..) => c.frob_pt(g),
        Pt::Inf => unreachable!(),
    };
    let gx2 = match gp {
        Pt::Aff(x, _) => x,
        Pt::Inf => unreachable!(),
    };
    let k = dlog[gx2 as usize];
    if k == NONE {
        return Err("sigma(g) has x outside G".into());
    }
    let lam = if c.mul(g, k as u64) == gp {
        k
    } else if c.mul(g, (r - k) as u64) == gp {
        r - k
    } else {
        return Err("sigma(g) is not a multiple of g".into());
    };
    if lam == 1 || pow_mod(lam as u64, f.n as u64, r as u64) != 1 {
        return Err(format!("lambda={lam} does not have order n"));
    }
    Ok(Group {
        n: f.n,
        a,
        q: f.q,
        ne,
        cof,
        r,
        lam,
        g,
        dlog,
        xs,
    })
}
