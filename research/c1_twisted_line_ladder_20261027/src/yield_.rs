//! Exact decomposition counts N(R) = #{(P1,P2,P3,P4) in FB^4 : P1+P2+P3+P4 = R}
//! (ordered, FB closed under negation) by a meet-in-the-middle pair table.
//!
//! Let c(A) = #{(P1,P2) in FB^2 : P1+P2 = A} (ordered). FB = -FB gives
//! c(A) = c(-A), so c depends on x(A) alone: S(x) = #ordered pairs whose sum
//! has abscissa x equals 2 c(A) for x != 0 and c(T) for x = 0, T = (0, sqrt b).
//! Then N(R) = sum_A c(A) c(R - A), summed over the classes {A, -A}.
use crate::curve::*;
use crate::gf::*;

pub struct PairTable {
    pub m: u64, // |FB| = c(O)
    keys: Vec<u64>,
    cnt: Vec<u32>,
    bits: u32,
    // dense list of classes with x != 0
    pub cx: Vec<E4>,
    pub cy: Vec<E4>,
    pub cs: Vec<u32>,
    pub s0: u64, // c(T)
}

const EMPTY: u64 = u64::MAX;

#[inline(always)]
fn hash(key: u64, bits: u32) -> usize {
    (key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - bits)) as usize
}

impl PairTable {
    fn find(&self, key: u64) -> Option<usize> {
        let mask = (1usize << self.bits) - 1;
        let mut h = hash(key, self.bits);
        loop {
            let k = self.keys[h];
            if k == key {
                return Some(h);
            }
            if k == EMPTY {
                return None;
            }
            h = (h + 1) & mask;
        }
    }
    #[inline]
    pub fn s_of(&self, f: &Fq4, x: &E4) -> u32 {
        match self.find(f.pack(x)) {
            Some(h) => self.cnt[h],
            None => 0,
        }
    }
    /// c(P) for any point P (ordered pair count)
    #[inline]
    pub fn c_of(&self, f: &Fq4, p: &Pt) -> u64 {
        if p.inf {
            self.m
        } else if is_zero(&p.x) {
            self.s0
        } else {
            (self.s_of(f, &p.x) / 2) as u64
        }
    }

    pub fn build(cv: &Curve, fb: &[Pt]) -> PairTable {
        let f = cv.f;
        let m = fb.len();
        let npairs = m * (m - 1) / 2 + m;
        let mut bits = 10;
        while (1usize << bits) < npairs.saturating_mul(2) {
            bits += 1;
        }
        let mut keys = vec![EMPTY; 1 << bits];
        let mut cnt = vec![0u32; 1 << bits];
        let mut ys: Vec<E4> = vec![ZERO; 0];
        let _ = &mut ys;
        let mut yv = vec![ZERO; 1 << bits];
        let mask = (1usize << bits) - 1;
        let mut insert = |x: &E4, y: &E4, add_n: u32| {
            let key = f.pack(x);
            let mut h = hash(key, bits);
            loop {
                if keys[h] == key {
                    cnt[h] += add_n;
                    return;
                }
                if keys[h] == EMPTY {
                    keys[h] = key;
                    cnt[h] = add_n;
                    yv[h] = *y;
                    return;
                }
                h = (h + 1) & mask;
            }
        };
        let mut dens: Vec<E4> = Vec::new();
        let mut scratch: Vec<E4> = Vec::new();
        for i in 0..m {
            // doubling
            let d = cv.add(&fb[i], &fb[i]);
            assert!(!d.inf);
            insert(&d.x, &d.y, 1);
            let js: Vec<usize> = (i + 1..m).filter(|&j| fb[j].x != fb[i].x).collect();
            dens.clear();
            for &j in &js {
                dens.push(add(&fb[i].x, &fb[j].x));
            }
            f.batch_inv(&mut dens, &mut scratch);
            for (t, &j) in js.iter().enumerate() {
                let lam = f.mul(&add(&fb[i].y, &fb[j].y), &dens[t]);
                let xs = add(&fb[i].x, &fb[j].x);
                let x3 = add(&add(&add(&f.sq(&lam), &lam), &xs), &cv.a);
                let y3 = add(&add(&f.mul(&lam, &add(&fb[i].x, &x3)), &x3), &fb[i].y);
                insert(&x3, &y3, 2);
            }
        }
        // classes
        let mut cx = vec![];
        let mut cy = vec![];
        let mut cs = vec![];
        let mut s0 = 0u64;
        for h in 0..(1usize << bits) {
            if keys[h] != EMPTY {
                let x = f.unpack(keys[h]);
                if is_zero(&x) {
                    s0 = cnt[h] as u64;
                } else {
                    assert!(cnt[h] % 2 == 0);
                    cx.push(x);
                    cy.push(yv[h]);
                    cs.push(cnt[h]);
                }
            }
        }
        PairTable { m: m as u64, keys, cnt, bits, cx, cy, cs, s0 }
    }

    /// N(R) for R != O.
    pub fn count(&self, cv: &Curve, r: &Pt) -> u64 {
        let f = cv.f;
        assert!(!r.inf);
        let mut total: u64 = self.m * self.c_of(f, r); // A = O
        // class x = 0: A = T (single point)
        if self.s0 > 0 {
            let t = Pt { x: ZERO, y: f.sqrt(&cv.b), inf: false };
            let b = cv.add(r, &t);
            total += self.s0 * self.c_of(f, &b);
        }
        const CH: usize = 2048;
        let n = self.cx.len();
        let mut dens: Vec<E4> = Vec::with_capacity(CH);
        let mut scratch: Vec<E4> = Vec::with_capacity(CH);
        let mut idx: Vec<usize> = Vec::with_capacity(CH);
        let mut start = 0;
        while start < n {
            let end = (start + CH).min(n);
            dens.clear();
            idx.clear();
            for i in start..end {
                if self.cx[i] == r.x {
                    // A = +-R: slow path
                    let a = Pt { x: self.cx[i], y: self.cy[i], inf: false };
                    let na = cv.neg(&a);
                    let c = (self.cs[i] / 2) as u64;
                    total += c * self.c_of(f, &cv.add(r, &na));
                    total += c * self.c_of(f, &cv.add(r, &a));
                } else {
                    idx.push(i);
                    dens.push(add(&r.x, &self.cx[i]));
                }
            }
            f.batch_inv(&mut dens, &mut scratch);
            for (t, &i) in idx.iter().enumerate() {
                let xs = add(&r.x, &self.cx[i]);
                let base = add(&xs, &cv.a);
                let ysum = add(&r.y, &self.cy[i]);
                let lp = f.mul(&ysum, &dens[t]);
                let lm = f.mul(&add(&ysum, &self.cx[i]), &dens[t]);
                let x3p = add(&add(&f.sq(&lp), &lp), &base);
                let x3m = add(&add(&f.sq(&lm), &lm), &base);
                let cp = self.c_pt_x(f, &x3p);
                let cm = self.c_pt_x(f, &x3m);
                total += (self.cs[i] / 2) as u64 * (cp + cm);
            }
            start = end;
        }
        total
    }
    #[inline]
    fn c_pt_x(&self, f: &Fq4, x: &E4) -> u64 {
        if is_zero(x) {
            self.s0
        } else {
            (self.s_of(f, x) / 2) as u64
        }
    }
    /// N(O) = sum_A c(A) c(-A) = sum c(A)^2
    pub fn count_o(&self) -> u64 {
        let mut t = self.m * self.m + self.s0 * self.s0;
        for &s in &self.cs {
            let c = (s / 2) as u64;
            t += 2 * c * c;
        }
        t
    }
}

/// Independent check: direct triple enumeration for one target.
pub fn count_brute(cv: &Curve, fb: &[Pt], r: &Pt) -> u64 {
    let f = cv.f;
    let mut inb = std::collections::HashSet::new();
    for p in fb {
        inb.insert(f.pack(&p.x));
    }
    let mut n = 0u64;
    for p1 in fb {
        let r1 = cv.add(r, &cv.neg(p1));
        for p2 in fb {
            let r2 = cv.add(&r1, &cv.neg(p2));
            for p3 in fb {
                let r3 = cv.add(&r2, &cv.neg(p3));
                if !r3.inf && inb.contains(&f.pack(&r3.x)) {
                    n += 1;
                }
            }
        }
    }
    n
}
