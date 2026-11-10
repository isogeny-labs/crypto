//! Short Weierstrass curve y^2 = x^3 + a x + b over F_p with affine arithmetic
//! charged through the counted field, plus planted-instance generation.
use crate::field::*;
use crate::rng::*;
use std::collections::HashMap;

pub type Pt = Option<(u64, u64)>;

pub struct Curve { pub f: Fp, pub a: u64, pub b: u64 }

impl Curve {
    pub fn new(p: u64, a: u64, b: u64) -> Curve { Curve { f: Fp::new(p), a, b } }
    pub fn on_curve(&self, p: Pt) -> bool {
        match p { None => true, Some((x, y)) => {
            let f = &self.f;
            let l = f.mul_nc(y, y);
            let r = f.add(f.mul_nc(f.mul_nc(x, x), x), f.add(f.mul_nc(self.a, x), self.b));
            l == r } }
    }
    pub fn neg(&self, p: Pt) -> Pt { p.map(|(x, y)| (x, self.f.neg(y))) }
    pub fn dbl(&self, p: Pt) -> Pt {
        let (x, y) = p?;
        if y == 0 { return None; }
        let f = &self.f;
        let x2 = f.mul(x, x);
        let num = f.add(f.add(f.add(x2, x2), x2), self.a);
        let lam = f.mul(num, f.inv(f.add(y, y)));
        let x3 = f.sub(f.sub(f.mul(lam, lam), x), x);
        let y3 = f.sub(f.mul(lam, f.sub(x, x3)), y);
        Some((x3, y3))
    }
    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let (x1, y1) = match p { None => return q, Some(v) => v };
        let (x2, y2) = match q { None => return p, Some(v) => v };
        let f = &self.f;
        if x1 == x2 {
            if f.add(y1, y2) == 0 { return None; }
            return self.dbl(p);
        }
        let lam = f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)));
        let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    pub fn mul_scalar(&self, k: u64, p: Pt) -> Pt {
        let mut r: Pt = None;
        if k == 0 { return None; }
        let top = 63 - k.leading_zeros();
        for i in (0..=top).rev() {
            r = self.dbl(r);
            if (k >> i) & 1 == 1 { r = self.add(r, p); }
        }
        r
    }
    pub fn sqrt_nc(&self, a: u64) -> Option<u64> {
        // p = 3 mod 4 only
        let f = &self.f;
        let r = f.pow_nc(a, (f.p + 1) / 4);
        if f.mul_nc(r, r) == a { Some(r) } else { None }
    }
    pub fn random_point(&self, rng: &mut Rng) -> (u64, u64) {
        let f = &self.f;
        loop {
            let x = rng.below(f.p);
            let rhs = f.add(f.add(f.mul_nc(f.mul_nc(x, x), x), f.mul_nc(self.a, x)), self.b);
            if let Some(y) = self.sqrt_nc(rhs) { if y != 0 { return (x, y); } }
        }
    }
}

fn isqrt(n: u64) -> u64 { let mut r = (n as f64).sqrt() as u64; while r * r > n { r -= 1; } while (r + 1) * (r + 1) <= n { r += 1; } r }

/// Group order by baby-step giant-step in the Hasse interval (uncounted; generation only).
pub fn count_points(c: &Curve, rng: &mut Rng) -> Option<u64> {
    let p = c.f.p;
    let s = isqrt(p);
    let lo = p + 1 - 2 * s - 2;
    let hi = p + 1 + 2 * s + 2;
    let width = hi - lo + 1;
    let m = isqrt(width) + 1;
    let mut cands: Option<Vec<u64>> = None;
    for _ in 0..10 {
        let r = Some(c.random_point(rng));
        match &cands {
            Some(cs) => {
                let keep: Vec<u64> = cs.iter().cloned().filter(|&t| c.mul_scalar(t, r).is_none()).collect();
                cands = Some(keep);
            }
            None => {
                let mut tab: HashMap<(u64, u64), u64> = HashMap::new();
                let mut jr: Pt = None;
                let mut collide = false;
                for j in 0..m {
                    if let Some(pt) = jr { if tab.insert(pt, j).is_some() { collide = true; break; } }
                    else if j != 0 { collide = true; break; }
                    jr = c.add(jr, r);
                }
                if collide { continue; }
                let t0 = c.neg(c.mul_scalar(lo, r));
                let mr = c.mul_scalar(m, r);
                let mut z = t0;
                let mut out = vec![];
                for i in 0..=m {
                    match z { None => { if i * m < width + m { out.push(lo + i * m) } }
                        Some(pt) => { if let Some(&j) = tab.get(&pt) { let u = i * m + j; if u < width { out.push(lo + u) } } } }
                    z = c.add(z, c.neg(mr));
                }
                cands = Some(out);
            }
        }
        if let Some(cs) = &cands { if cs.len() == 1 { return Some(cs[0]); } if cs.is_empty() { return None; } }
    }
    None
}

#[derive(Clone, Debug)]
pub struct Instance { pub bits: u32, pub idx: u64, pub p: u64, pub a: u64, pub b: u64, pub n: u64, pub g: (u64, u64), pub q: (u64, u64), pub k: u64, pub seed: u64 }

pub const MASTER_SEED: u64 = 0x4D494E4F52535F47; // "MINORS_G"

pub fn instance_seed(bits: u32, idx: u64) -> u64 { mix(mix(MASTER_SEED, bits as u64), idx) }

/// Planted instance with prime group order of exactly `bits` bits, p = 3 mod 4.
pub fn make_instance(bits: u32, idx: u64) -> Instance {
    assert!(bits >= 8 && bits <= 42);
    let seed = instance_seed(bits, idx);
    let mut rng = Rng::new(seed);
    let margin = (1u64 << (bits / 2 + 1)) + 6;
    let (plo, phi) = ((1u64 << (bits - 1)) + margin, (1u64 << bits) - margin - 2);
    loop {
        let p = (plo + rng.below(phi - plo)) | 3; // p = 3 mod 4
        if p > phi || !is_prime(p) { continue; }
        let a = 1 + rng.below(p - 1);
        let b = 1 + rng.below(p - 1);
        let c = Curve::new(p, a, b);
        let f = &c.f;
        let disc = f.add(f.mul_nc(4, f.mul_nc(a, f.mul_nc(a, a))), f.mul_nc(27, f.mul_nc(b, b)));
        if disc == 0 { continue; }
        let n = match count_points(&c, &mut rng) { Some(n) => n, None => continue };
        if !is_prime(n) || 64 - n.leading_zeros() != bits { continue; }
        let g = c.random_point(&mut rng);
        let k = 1 + rng.below(n - 1);
        let q = c.mul_scalar(k, Some(g)).expect("Q != O");
        return Instance { bits, idx, p, a, b, n, g, q, k, seed };
    }
}
