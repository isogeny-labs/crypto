//! Koblitz-type curve E_a: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n} (b = 1, as ECC2K-130
//! has a = 0, b = 1), a cyclic-group logarithm table, and the 2-torsion point T = (0,1).
//! Translation by T acts on x as iota: x -> 1/x (checked exactly in the tests).

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
}

impl<'a> Curve<'a> {
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
                let rhs = f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ 1;
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
    /// A y with (x,y) on the curve, or None (x = 0 gives y = 1 = sqrt(b)).
    pub fn lift_x(&self, x: u32) -> Option<u32> {
        let f = self.f;
        if x == 0 {
            return Some(1);
        }
        let c = x ^ self.a ^ f.inv(f.sq(x));
        if f.tr(c) == 1 {
            return None;
        }
        Some(f.mul(x, f.ht(c)))
    }
    /// #E(F_q) by trace counting.
    pub fn count(&self) -> u32 {
        let f = self.f;
        let mut c = 2u32; // O and T
        for x in 1..f.q {
            if f.tr(x ^ self.a ^ f.inv(f.sq(x))) == 0 {
                c += 2;
            }
        }
        c
    }
}

fn prime_factors(mut n: u32) -> Vec<u32> {
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

pub fn largest_prime_factor(n: u32) -> u32 {
    *prime_factors(n).last().unwrap()
}

/// Cyclic group E(F_q) = <g>; `logx[x]` is the smaller of {k, ne-k} with x(k g) = x, or NONE.
pub struct Group {
    pub ne: u32,
    /// ne / 2: the log of the 2-torsion point T = (0,1).
    pub m: u32,
    /// 2-part of ne (the cofactor). G = {l : h | l} is the odd-order subgroup.
    pub h: u32,
    pub q: u32,
    pub a: u32,
    pub logx: Vec<u32>,
    /// the generator g used for the logarithm table (log 1)
    pub gen: Pt,
}

pub fn build_group(f: &Fq, a: u32) -> Result<Group, String> {
    let c = Curve { f, a };
    let ne = c.count();
    if ne % 2 != 0 {
        return Err(format!("#E={ne} odd"));
    }
    let primes = prime_factors(ne);
    let mut gen = None;
    for x in 2..f.q.min(5000) {
        if let Some(y) = c.lift_x(x) {
            let g = Pt::Aff(x, y);
            if primes.iter().all(|&p| c.mul(g, (ne / p) as u64) != Pt::Inf) {
                gen = Some(g);
                break;
            }
        }
    }
    let g = gen.ok_or_else(|| format!("E(F_q) (#E={ne}) not cyclic or no generator found"))?;
    let mut logx = vec![NONE; f.q as usize];
    let mut p = Pt::Inf;
    for k in 1..ne {
        p = c.add(p, g);
        match p {
            Pt::Aff(x, _) => {
                if logx[x as usize] == NONE {
                    logx[x as usize] = k;
                }
            }
            Pt::Inf => return Err("walk hit O early".into()),
        }
    }
    if c.add(p, g) != Pt::Inf {
        return Err("walk did not close".into());
    }
    Ok(Group {
        ne,
        m: ne / 2,
        h: 1 << ne.trailing_zeros(),
        q: f.q,
        a,
        logx,
        gen: g,
    })
}
