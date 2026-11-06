//! Long-Weierstrass curve Y^2 = X^3 + a2 X^2 + a4 X + a6 over any FieldOps,
//! group order by baby-step giant-step, and a matched Pollard-rho baseline.
use crate::field::{FieldOps, K4, E4};
use std::collections::HashMap;

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

pub type Pt<El> = Option<(El, El)>;

pub struct Curve<'a, F: FieldOps> {
    pub f: &'a F,
    pub a2: F::El,
    pub a4: F::El,
    pub a6: F::El,
}

impl<'a, F: FieldOps> Curve<'a, F> {
    pub fn rhs(&self, x: &F::El) -> F::El {
        let f = self.f;
        let x2 = f.mul(x, x);
        let x3 = f.mul(&x2, x);
        f.add(&f.add(&x3, &f.mul(&self.a2, &x2)), &f.add(&f.mul(&self.a4, x), &self.a6))
    }
    pub fn on_curve(&self, p: &Pt<F::El>) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.f.mul(y, y) == self.rhs(x),
        }
    }
    pub fn neg(&self, p: &Pt<F::El>) -> Pt<F::El> {
        p.map(|(x, y)| (x, self.f.neg(&y)))
    }
    pub fn add(&self, p: &Pt<F::El>, q: &Pt<F::El>) -> Pt<F::El> {
        let f = self.f;
        let (x1, y1) = match p {
            None => return *q,
            Some(v) => *v,
        };
        let (x2, y2) = match q {
            None => return *p,
            Some(v) => *v,
        };
        let lam = if x1 == x2 {
            if f.is_zero(&f.add(&y1, &y2)) {
                return None;
            }
            // tangent: (3x^2 + 2 a2 x + a4) / (2y)
            let three = f.from_u64(3);
            let two = f.from_u64(2);
            let num = f.add(
                &f.add(&f.mul(&three, &f.mul(&x1, &x1)), &f.mul(&two, &f.mul(&self.a2, &x1))),
                &self.a4,
            );
            f.mul(&num, &f.inv(&f.mul(&two, &y1)))
        } else {
            f.mul(&f.sub(&y2, &y1), &f.inv(&f.sub(&x2, &x1)))
        };
        let x3 = f.sub(&f.sub(&f.sub(&f.mul(&lam, &lam), &self.a2), &x1), &x2);
        let y3 = f.sub(&f.mul(&lam, &f.sub(&x1, &x3)), &y1);
        Some((x3, y3))
    }
    pub fn mul(&self, p: &Pt<F::El>, k: u128) -> Pt<F::El> {
        let mut r: Pt<F::El> = None;
        let mut b = *p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            b = self.add(&b, &b);
            k >>= 1;
        }
        r
    }
}

/// Random point of E(K4) (square roots taken through L by the caller's closure).
pub fn random_point(
    c: &Curve<K4>,
    sqrt_k4: &dyn Fn(&E4) -> Option<E4>,
    rng: &mut Rng,
) -> Pt<E4> {
    let k = c.f;
    loop {
        let x = [rng.below(k.p), rng.below(k.p), rng.below(k.p), rng.below(k.p)];
        let r = c.rhs(&x);
        if let Some(y) = sqrt_k4(&r) {
            let y = if rng.next() & 1 == 1 { k.neg(&y) } else { y };
            return Some((x, y));
        }
    }
}

/// #E(K4) by BSGS over the Hasse interval, intersecting candidate sets of
/// several random points until one candidate remains.
pub fn group_order(c: &Curve<K4>, sqrt_k4: &dyn Fn(&E4) -> Option<E4>, rng: &mut Rng) -> u128 {
    let p = c.f.p as u128;
    let q = p.pow(4);
    let lo = q + 1 - 2 * p * p;
    let width = 4 * p * p;
    let mut cands: Option<Vec<u128>> = None;
    for _round in 0..40 {
        let pt = random_point(c, sqrt_k4, rng);
        let m = ((width as f64).sqrt() as u128) + 1;
        let mut baby: HashMap<E4, Vec<(u128, E4)>> = HashMap::new();
        let mut acc: Pt<E4> = None;
        for j in 0..=m {
            if let Some((x, y)) = acc {
                baby.entry(x).or_default().push((j, y));
            }
            acc = c.add(&acc, &pt);
        }
        let mp = c.mul(&pt, m);
        let mut g = c.mul(&pt, lo);
        let mut found = Vec::new();
        let mut i: u128 = 0;
        while i * m <= width + m {
            // want lo + i*m + j with (lo + i m) P + j P = O  <=> jP = -(G_i)
            match g {
                None => found.push(lo + i * m),
                Some((gx, gy)) => {
                    if let Some(v) = baby.get(&gx) {
                        for &(j, y) in v {
                            if c.f.add(&y, &gy) == [0; 4] {
                                found.push(lo + i * m + j);
                            }
                        }
                    }
                }
            }
            g = c.add(&g, &mp);
            i += 1;
        }
        found.retain(|&n| n >= lo && n <= lo + width);
        found.sort();
        found.dedup();
        let next: Vec<u128> = match &cands {
            None => found,
            Some(prev) => prev.iter().cloned().filter(|n| found.contains(n)).collect(),
        };
        cands = Some(next);
        if cands.as_ref().unwrap().len() == 1 {
            return cands.unwrap()[0];
        }
    }
    panic!("group order not determined: {:?}", cands.map(|v| v.len()));
}

pub fn factor(mut n: u128) -> Vec<(u128, u32)> {
    let mut out = Vec::new();
    let mut d: u128 = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            out.push((d, e));
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

pub fn mulr(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
pub fn powr(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut x = 1u64 % r;
    a %= r;
    while e > 0 {
        if e & 1 == 1 {
            x = mulr(x, a, r);
        }
        a = mulr(a, a, r);
        e >>= 1;
    }
    x
}
pub fn invr(a: u64, r: u64) -> u64 {
    assert!(a % r != 0);
    powr(a, r - 2, r)
}

pub struct RhoResult {
    pub log: Option<u64>,
    pub steps: u64,
    pub seconds: f64,
    pub k4_muls: u64,
}

/// Pollard rho with a 32-adding walk and Brent cycle detection on <P> of prime order r.
pub fn pollard_rho(c: &Curve<K4>, p: &Pt<E4>, q: &Pt<E4>, r: u64, rng: &mut Rng) -> RhoResult {
    let t0 = std::time::Instant::now();
    let m0 = c.f.muls.get();
    const NB: usize = 32;
    let mut total_steps = 0u64;
    loop {
        let mut ma = [0u64; NB];
        let mut mb = [0u64; NB];
        let mut mp: Vec<Pt<E4>> = Vec::with_capacity(NB);
        for j in 0..NB {
            ma[j] = rng.below(r);
            mb[j] = rng.below(r);
            mp.push(c.add(&c.mul(p, ma[j] as u128), &c.mul(q, mb[j] as u128)));
        }
        let idx = |pt: &Pt<E4>| -> usize {
            match pt {
                None => 0,
                Some((x, _)) => ((x[0].wrapping_mul(0x9E37) ^ x[1].wrapping_mul(31) ^ x[2] ^ x[3].wrapping_mul(7)) % NB as u64) as usize,
            }
        };
        let (mut a, mut b) = (rng.below(r), rng.below(r));
        let mut w = c.add(&c.mul(p, a as u128), &c.mul(q, b as u128));
        let (mut sa, mut sb, mut sw) = (a, b, w);
        let mut power = 1u64;
        let mut lam = 0u64;
        loop {
            let j = idx(&w);
            w = c.add(&w, &mp[j]);
            a = (a + ma[j]) % r;
            b = (b + mb[j]) % r;
            total_steps += 1;
            lam += 1;
            if w == sw {
                if b != sb {
                    let num = (sa + r - a) % r;
                    let den = (b + r - sb) % r;
                    let x = mulr(num, invr(den, r), r);
                    return RhoResult {
                        log: Some(x),
                        steps: total_steps,
                        seconds: t0.elapsed().as_secs_f64(),
                        k4_muls: c.f.muls.get() - m0,
                    };
                }
                break; // useless collision; restart with a new walk
            }
            if lam == power {
                sa = a;
                sb = b;
                sw = w;
                power *= 2;
                lam = 0;
            }
            if total_steps > 200_000_000 {
                return RhoResult { log: None, steps: total_steps, seconds: t0.elapsed().as_secs_f64(), k4_muls: c.f.muls.get() - m0 };
            }
        }
    }
}
