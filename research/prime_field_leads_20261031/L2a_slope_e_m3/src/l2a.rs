//! L2a harness: counted work of interval-constrained 3-point (S4) decomposition
//! over a prime field, p ~ 2^(3s), window [0, 2^s). Standalone (rustc -O, no deps).
//! Counted metric: Montgomery field multiplications (every one, including those inside
//! inversions, root finding and window tests) + decisions. No wall time is recorded
//! (a wall clock is read only for the 20-minute cell stop guard).
//!
//! Arms (all answer: exist x1<=x2, x3 in window, each liftable, with R = +-P1 +-P2 +-P3 ?)
//!  G  group MITM: enumerate pairs (x1<=x2), four sums R -+ P1 -+ P2 by x-only addition with
//!     chunked Montgomery batch inversion; test x(sum) < B.
//!  S  algebraic S4: per pair build the quartic S4(x1,x2,X,xR) in X, find its F_p roots
//!     (gcd with X^p-X, Cantor-Zassenhaus), test root < B and liftable.
//!  T  triple scan of S4 over all x1<=x2<=x3 (s <= 8 only; independent check).
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

// ---------------- plain helpers (uncounted precompute) ----------------
fn ad(p: u64, a: u64, b: u64) -> u64 { let s = a + b; if s >= p { s - p } else { s } }
fn sb(p: u64, a: u64, b: u64) -> u64 { if a >= b { a - b } else { a + p - b } }
fn ng(p: u64, a: u64) -> u64 { if a == 0 { 0 } else { p - a } }
fn db(p: u64, a: u64) -> u64 { ad(p, a, a) }
fn mulmod(a: u64, b: u64, m: u64) -> u64 { ((a as u128 * b as u128) % m as u128) as u64 }
fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m; a %= m;
    while e > 0 { if e & 1 == 1 { r = mulmod(r, a, m); } a = mulmod(a, a, m); e >>= 1; }
    r
}
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] { if n % q == 0 { return n == q; } }
    let mut d = n - 1; let mut r = 0; while d % 2 == 0 { d /= 2; r += 1; }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..r - 1 { x = mulmod(x, x, n); if x == n - 1 { continue 'w; } }
        return false;
    }
    true
}
fn sqrt_mod(a: u64, p: u64) -> Option<u64> { // Tonelli-Shanks
    let a = a % p; if a == 0 { return Some(0); }
    if powmod(a, (p - 1) / 2, p) != 1 { return None; }
    let mut q = p - 1; let mut s = 0; while q % 2 == 0 { q /= 2; s += 1; }
    let mut z = 2; while powmod(z, (p - 1) / 2, p) != p - 1 { z += 1; }
    let (mut m, mut c, mut t, mut r) = (s, powmod(z, q, p), powmod(a, q, p), powmod(a, (q + 1) / 2, p));
    while t != 1 {
        let mut i = 0; let mut tt = t; while tt != 1 { tt = mulmod(tt, tt, p); i += 1; }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i; c = mulmod(b, b, p); t = mulmod(t, c, p); r = mulmod(r, b, p);
    }
    Some(r)
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 { // splitmix64
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

// ---------------- counted Montgomery field ----------------
struct F { p: u64, ninv: u64, r2: u64, one: u64, m: u64, dec: u64, rng: u64 }
impl F {
    fn new(p: u64) -> F {
        let mut inv = p; for _ in 0..7 { inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv))); }
        let r = ((1u128 << 64) % p as u128) as u64;
        let r2 = mulmod(r, r, p);
        F { p, ninv: inv.wrapping_neg(), r2, one: r, m: 0, dec: 0, rng: 0x1234_5678_9abc_def1 }
    }
    #[inline] fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.ninv);
        let u = ((t + (m as u128) * (self.p as u128)) >> 64) as u64;
        if u >= self.p { u - self.p } else { u }
    }
    #[inline] fn mul(&mut self, a: u64, b: u64) -> u64 { self.m += 1; self.redc(a as u128 * b as u128) }
    #[inline] fn add(&self, a: u64, b: u64) -> u64 { let s = a + b; if s >= self.p { s - self.p } else { s } }
    #[inline] fn sub(&self, a: u64, b: u64) -> u64 { if a >= b { a - b } else { a + self.p - b } }
    #[inline] fn neg(&self, a: u64) -> u64 { if a == 0 { 0 } else { self.p - a } }
    fn tom(&self, x: u64) -> u64 { self.redc((x % self.p) as u128 * self.r2 as u128) }
    fn fromm(&self, x: u64) -> u64 { self.redc(x as u128) }
    fn pow(&mut self, a: u64, mut e: u64) -> u64 {
        let mut r = self.one; let mut b = a;
        while e > 0 { if e & 1 == 1 { r = self.mul(r, b); } e >>= 1; if e > 0 { b = self.mul(b, b); } }
        r
    }
    fn inv(&mut self, a: u64) -> u64 { let e = self.p - 2; self.pow(a, e) }
    fn rnd(&mut self) -> u64 { self.rng = self.rng.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.rng; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); self.tom((z ^ (z >> 31)) % self.p) }
    fn dbl(&self, a: u64) -> u64 { self.add(a, a) }
}

// ---------------- polynomials over F (Vec low->high, Montgomery coefficients) ----------------
fn trim(a: &mut Vec<u64>) { while a.last() == Some(&0) { a.pop(); } }
/// remainder of a by b (b nonzero, trimmed); counted
fn prem(f: &mut F, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut r = a.to_vec(); trim(&mut r);
    let db = b.len() - 1;
    if r.len() <= db { return r; }
    let li = f.inv(*b.last().unwrap());
    while r.len() > db && !r.is_empty() {
        let k = r.len() - 1 - db;
        let c = f.mul(*r.last().unwrap(), li);
        for j in 0..=db { let t = f.mul(c, b[j]); r[k + j] = sb(f.p, r[k + j], t); }
        trim(&mut r);
        if r.len() <= db { break; }
    }
    r
}
fn pgcd(f: &mut F, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut x = a.to_vec(); trim(&mut x); let mut y = b.to_vec(); trim(&mut y);
    while !y.is_empty() { let r = prem(f, &x, &y); x = y; y = r; }
    x
}
fn monic(f: &mut F, a: &mut Vec<u64>) { let li = f.inv(*a.last().unwrap()); for c in a.iter_mut() { *c = f.mul(*c, li); } }
fn pdiv(f: &mut F, a: &[u64], b: &[u64]) -> Vec<u64> { // exact quotient, b monic
    let mut r = a.to_vec(); let db = b.len() - 1; let mut q = vec![0u64; a.len() - db];
    for k in (0..q.len()).rev() {
        let c = r[k + db]; q[k] = c;
        if c != 0 { for j in 0..=db { let t = f.mul(c, b[j]); r[k + j] = sb(f.p, r[k + j], t); } }
    }
    q
}
/// a*b mod h, h monic of degree d<=4; a,b length-4 arrays (deg<d)
fn mulmod_poly(f: &mut F, a: &[u64; 4], b: &[u64; 4], h: &[u64], d: usize) -> [u64; 4] {
    let mut pr = [0u64; 8];
    for i in 0..d { if a[i] == 0 { continue; } for j in 0..d { if b[j] == 0 { continue; } let t = f.mul(a[i], b[j]); pr[i + j] = ad(f.p, pr[i + j], t); } }
    for i in (d..=2 * d.max(1) - 2).rev() {
        let c = pr[i]; if c == 0 { continue; }
        for j in 0..d { let t = f.mul(c, h[j]); pr[i - d + j] = sb(f.p, pr[i - d + j], t); }
    }
    let mut o = [0u64; 4]; o[..d].copy_from_slice(&pr[..d]); o
}
/// (X + a)^e mod h (h monic degree d>=2)
fn powmod_lin(f: &mut F, a: u64, e: u64, h: &[u64], d: usize) -> [u64; 4] {
    let mut base = [0u64; 4]; base[0] = a; base[1] = f.one;
    let mut r = [0u64; 4]; r[0] = f.one;
    let mut e = e;
    while e > 0 { if e & 1 == 1 { r = mulmod_poly(f, &r, &base, h, d); } e >>= 1; if e > 0 { base = mulmod_poly(f, &base, &base, h, d); } }
    r
}
/// X^p mod h, h monic degree d>=1 ; left-to-right binary with shift-by-X steps
fn x_pow_p(f: &mut F, h: &[u64], d: usize) -> [u64; 4] {
    let p = f.p;
    let mut r = [0u64; 4];
    if d == 1 { // X = -h0
        r[0] = ng(f.p, h[0]); // X mod h is the constant -h0 ; X^p = (-h0)^p = -h0 in F_p
        return r;
    }
    r[1] = f.one;
    let nb = 64 - p.leading_zeros();
    for bit in (0..nb - 1).rev() {
        r = mulmod_poly(f, &r.clone(), &r, h, d);
        if (p >> bit) & 1 == 1 { // multiply by X
            let c = r[d - 1];
            let mut n = [0u64; 4];
            for j in (1..d).rev() { n[j] = r[j - 1]; }
            if c != 0 { for j in 0..d { let t = f.mul(c, h[j]); n[j] = sb(f.p, n[j], t); } }
            r = n;
        }
    }
    r
}
/// all roots in F_p of monic squarefree-product-of-linears g (deg>=1)
fn split_roots(f: &mut F, g: Vec<u64>, out: &mut Vec<u64>) {
    let d = g.len() - 1;
    if d == 0 { return; }
    if d == 1 { out.push(ng(f.p, g[0])); return; }
    loop {
        let a = f.rnd();
        let w = powmod_lin(f, a, (f.p - 1) / 2, &g, d);
        let mut wv: Vec<u64> = w[..d].to_vec(); wv[0] = sb(f.p, wv[0], f.one); trim(&mut wv);
        if wv.is_empty() { continue; }
        let mut g1 = pgcd(f, &g, &wv);
        let d1 = g1.len() - 1;
        if d1 == 0 || d1 == d { continue; }
        monic(f, &mut g1);
        let g2 = pdiv(f, &g, &g1);
        split_roots(f, g1, out); split_roots(f, g2, out);
        return;
    }
}
/// roots in F_p of polynomial h (not necessarily monic, trimmed, deg>=1)
fn fp_roots(f: &mut F, h: &mut Vec<u64>) -> Vec<u64> {
    monic(f, h);
    let d = h.len() - 1;
    let r = x_pow_p(f, h, d);
    let mut rv: Vec<u64> = r[..d].to_vec();
    if d >= 2 { rv[1] = sb(f.p, rv[1], f.one); } else { rv[0] = sb(f.p, rv[0], 0); }
    // for d==1 X^p - X mod h = (-h0) - (-h0) = 0 -> g = h
    if d == 1 { rv = vec![]; }
    trim(&mut rv);
    let mut g = if rv.is_empty() { h.clone() } else { pgcd(f, h, &rv) };
    if g.len() <= 1 { return vec![]; }
    monic(f, &mut g);
    let mut out = vec![];
    split_roots(f, g, &mut out);
    out
}

// ---------------- curve / instances ----------------
struct Curve { a: u64, b: u64, am: u64, bm: u64, b2: u64, b4: u64, two: u64, three: u64,
               lift: Vec<bool>, pts: Vec<(u64, u64, u64)> /* (x plain, xm, ym) */ }
struct Target { xm: u64, ym: u64, g2: [u64; 3], g1: [u64; 3], g0: [u64; 3] }

fn build_curve(f: &F, b_win: u64, rng: &mut Rng) -> Curve {
    let p = f.p;
    loop {
        let a = rng.below(p); let b = rng.below(p);
        let disc = (4 * mulmod(a, mulmod(a, a, p), p) + 27 * mulmod(b, b, p) % p) % p;
        if disc == 0 { continue; }
        let mut lift = vec![false; b_win as usize]; let mut pts = vec![];
        for x in 0..b_win {
            let rhs = (mulmod(mulmod(x, x, p), x, p) + mulmod(a, x, p) + b) % p;
            if let Some(y) = sqrt_mod(rhs, p) { lift[x as usize] = true; pts.push((x, f.tom(x), f.tom(y))); }
        }
        let am = f.tom(a); let bm = f.tom(b);
        let b2 = db(f.p, bm); let b4 = db(f.p, b2);
        let two = f.tom(2); let three = f.tom(3);
        return Curve { a, b, am, bm, b2, b4, two, three, lift, pts };
    }
}
fn build_target(f: &mut F, c: &Curve, b_win: u64, rng: &mut Rng) -> Target {
    let p = f.p;
    loop {
        let x = b_win + rng.below(p - b_win);
        let rhs = (mulmod(mulmod(x, x, p), x, p) + mulmod(c.a, x, p) + c.b) % p;
        if let Some(y) = sqrt_mod(rhs, p) {
            let xm = f.tom(x); let ym = f.tom(y);
            let m0 = f.m;
            // g polys in x3 (z = xR): g2=[z^2,-2z,1], g1=[-2za-4b, -2z^2-2a, -2z], g0=[a^2-4zb, -2za-4b, z^2]
            let z = xm; let z2 = f.mul(z, z); let tz = db(f.p, z);
            let za = f.mul(z, c.am); let zb4 = f.mul(z, c.b4); let a2 = f.mul(c.am, c.am);
            let tza = db(f.p, za);
            let g2 = [z2, ng(f.p, tz), f.one];
            let g1c0 = ng(f.p, ad(f.p, tza, c.b4));
            let g1 = [g1c0, ng(f.p, ad(f.p, db(f.p, z2), db(f.p, c.am))), ng(f.p, tz)];
            let g0 = [sb(f.p, a2, zb4), g1c0, z2];
            let _ = m0;
            return Target { xm, ym, g2, g1, g0 };
        }
    }
}

// ---------------- Arm G ----------------
/// returns (sat, pairs)
fn arm_g(f: &mut F, c: &Curve, t: &Target, b_win: u64) -> (bool, u64) {
    let l = c.pts.len(); let mut pairs = 0u64;
    const CH: usize = 64;
    for i in 0..l {
        let (_, x1, y1) = c.pts[i];
        // A1 = R - P1, A2 = R + P1 ; one inversion for xR - x1
        let dx = sb(f.p, t.xm, x1); let id = f.inv(dx);
        let mut aa = [(0u64, 0u64); 2];
        for (k, y1s) in [ng(f.p, y1), y1].iter().enumerate() {
            let lam = { let n = sb(f.p, t.ym, *y1s); f.mul(n, id) };
            let l2 = f.mul(lam, lam);
            let xa = sb(f.p, sb(f.p, l2, t.xm), x1);
            let ya = { let d = sb(f.p, t.xm, xa); let m = f.mul(lam, d); sb(f.p, m, t.ym) };
            aa[k] = (xa, ya);
        }
        let mut j0 = i;
        while j0 < l {
            let j1 = (j0 + CH).min(l);
            let n = j1 - j0;
            // batch inverse of dx_k = xA - x2 for both A
            let mut invs = [[0u64; CH]; 2]; let mut zero = [[false; CH]; 2];
            for k in 0..2 {
                let (xa, _) = aa[k];
                let mut pre = [0u64; CH]; let mut acc = f.one;
                for q in 0..n {
                    let d = sb(f.p, xa, c.pts[j0 + q].1);
                    if d == 0 { zero[k][q] = true; pre[q] = acc; } else { acc = f.mul(acc, d); pre[q] = acc; }
                }
                let mut ia = f.inv(acc);
                for q in (0..n).rev() {
                    if zero[k][q] { continue; }
                    let before = if q == 0 { f.one } else { pre[q - 1] };
                    invs[k][q] = f.mul(ia, before);
                    let d = sb(f.p, xa, c.pts[j0 + q].1);
                    ia = f.mul(ia, d);
                }
            }
            for q in 0..n {
                let (_, x2, y2) = c.pts[j0 + q];
                f.dec += 1; pairs += 1;
                for k in 0..2 {
                    let (xa, ya) = aa[k];
                    let mut xs: [Option<u64>; 2] = [None, None];
                    if !zero[k][q] {
                        let iv = invs[k][q];
                        let lp = { let n = sb(f.p, ya, y2); f.mul(n, iv) };
                        let lm = { let n = ad(f.p, ya, y2); f.mul(n, iv) };
                        let xp = { let s = f.mul(lp, lp); sb(f.p, sb(f.p, s, xa), x2) };
                        let xm_ = { let s = f.mul(lm, lm); sb(f.p, sb(f.p, s, xa), x2) };
                        xs = [Some(xp), Some(xm_)];
                    } else {
                        // xa == x2 : A = +-P2
                        let same = ya == y2; // A = P2 -> A+P2 = 2P2 ; A-P2 = O
                        let opp = ya == ng(f.p, y2);
                        let dbl_pt = if same { Some((x2, y2)) } else if opp { Some((xa, ya)) } else { None };
                        if let Some((xd, yd)) = dbl_pt {
                            if yd != 0 {
                                let num = { let s = f.mul(xd, xd); let s3 = f.mul(s, c.three); ad(f.p, s3, c.am) };
                                let den = db(f.p, yd); let iv = f.inv(den);
                                let lam = f.mul(num, iv); let l2 = f.mul(lam, lam);
                                let xr = sb(f.p, sb(f.p, l2, xd), xd);
                                if same { xs[0] = Some(xr); } else { xs[1] = Some(xr); }
                            }
                        }
                    }
                    for v in xs.iter().flatten() {
                        f.m += 1; // window test needs one Montgomery reduction (from-Montgomery)
                        if f.fromm(*v) < b_win { return (true, pairs); }
                    }
                }
            }
            j0 = j1;
        }
    }
    (false, pairs)
}

// ---------------- Arm S ----------------
fn arm_s(f: &mut F, c: &Curve, t: &Target, b_win: u64) -> (bool, u64) {
    let l = c.pts.len(); let mut pairs = 0u64;
    for i in 0..l {
        let x1 = c.pts[i].1;
        for j in i..l {
            let x2 = c.pts[j].1;
            pairs += 1; f.dec += 1;
            // f coefficients: S3(x1,x2,X) = f2 X^2 + f1 X + f0
            let dd = sb(f.p, x1, x2); let f2 = f.mul(dd, dd);
            let pr = f.mul(x1, x2); let sm = ad(f.p, x1, x2);
            let t1 = f.mul(sm, ad(f.p, pr, c.am));
            let f1 = ng(f.p, db(f.p, ad(f.p, t1, c.b2)));
            let q = sb(f.p, pr, c.am); let q2 = f.mul(q, q);
            let f0 = sb(f.p, q2, f.mul(c.b4, sm));
            let mut u = [0u64; 3]; let mut v = [0u64; 3]; let mut w = [0u64; 3];
            for k in 0..3 {
                u[k] = sb(f.p, f.mul(f2, t.g0[k]), f.mul(f0, t.g2[k]));
                v[k] = sb(f.p, f.mul(f2, t.g1[k]), f.mul(f1, t.g2[k]));
                w[k] = sb(f.p, f.mul(f1, t.g0[k]), f.mul(f0, t.g1[k]));
            }
            let mut h = vec![0u64; 5];
            for a in 0..3 { for b in 0..3 {
                let uu = f.mul(u[a], u[b]); h[a + b] = ad(f.p, h[a + b], uu);
                let vw = f.mul(v[a], w[b]); h[a + b] = sb(f.p, h[a + b], vw);
            } }
            trim(&mut h);
            if h.is_empty() { // identically zero: every x3 works; scan window for a liftable x3
                f.dec += 1;
                if c.lift.iter().any(|&b| b) { return (true, pairs); }
                continue;
            }
            if h.len() == 1 { continue; }
            let roots = fp_roots(f, &mut h);
            for r in roots {
                f.dec += 1; f.m += 1;
                let xp = f.fromm(r);
                if xp < b_win && c.lift[xp as usize] { return (true, pairs); }
            }
        }
    }
    (false, pairs)
}

// ---------------- Arm T ----------------
fn arm_t(f: &mut F, c: &Curve, t: &Target) -> (bool, u64) {
    let l = c.pts.len(); let mut trip = 0u64;
    for i in 0..l {
        let x1 = c.pts[i].1;
        for j in i..l {
            let x2 = c.pts[j].1;
            let dd = sb(f.p, x1, x2); let f2 = f.mul(dd, dd);
            let pr = f.mul(x1, x2); let sm = ad(f.p, x1, x2);
            let t1 = f.mul(sm, ad(f.p, pr, c.am));
            let f1 = ng(f.p, db(f.p, ad(f.p, t1, c.b2)));
            let q = sb(f.p, pr, c.am); let q2 = f.mul(q, q);
            let f0 = sb(f.p, q2, f.mul(c.b4, sm));
            for k in j..l {
                let x3 = c.pts[k].1; trip += 1; f.dec += 1;
                // evaluate g_i(x3)
                let x32 = f.mul(x3, x3);
                let g2 = { let d = sb(f.p, t.xm, x3); f.mul(d, d) };
                let zx = f.mul(t.xm, x3);
                let z2 = f.mul(t.xm, t.xm);
                let g1 = { // -2z^2 x3 - 2z a - 2 z x3^2 - 2 a x3 - 4b
                    let a1 = f.mul(z2, x3); let a2 = f.mul(zx, x3); let a3 = f.mul(t.xm, c.am); let a4 = f.mul(c.am, x3);
                    let s = ad(f.p, ad(f.p, a1, a3), ad(f.p, a2, a4)); ng(f.p, ad(f.p, db(f.p, s), c.b4)) };
                let g0 = { // x3^2 z^2 - 2z(a x3 + 2b) + a^2 - 4b x3
                    let a1 = f.mul(x32, z2); let ax = f.mul(c.am, x3); let a2 = f.mul(ad(f.p, ax, c.b2), t.xm);
                    let a3 = f.mul(c.am, c.am); let a4 = f.mul(c.b4, x3);
                    sb(f.p, ad(f.p, sb(f.p, a1, db(f.p, a2)), a3), a4) };
                let u = sb(f.p, f.mul(f2, g0), f.mul(f0, g2));
                let v = sb(f.p, f.mul(f2, g1), f.mul(f1, g2));
                let w = sb(f.p, f.mul(f1, g0), f.mul(f0, g1));
                let uu = f.mul(u, u); let vw = f.mul(v, w);
                if uu == vw { return (true, trip); }
            }
        }
    }
    (false, trip)
}

// ---------------- driver ----------------
struct Row { s: u32, p: u64, curve: usize, target: usize, arm: char, sat: i32, pairs: u64, mults: u64, dec: u64, status: &'static str }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).expect("outdir").clone();
    let s_lo: u32 = args[2].parse().unwrap(); let s_hi: u32 = args[3].parse().unwrap();
    let nthreads: usize = args.get(4).map(|x| x.parse().unwrap()).unwrap_or(4);
    let ncurves = 3usize; let ntargets = 32usize; let t_max = 8u32;
    let cell_limit = 20.0 * 60.0;
    let csv_path = format!("{}/attempts.csv", out);
    let exists = std::path::Path::new(&csv_path).exists();
    let mut csv = OpenOptions::new().create(true).append(true).open(&csv_path).unwrap();
    if !exists { writeln!(csv, "s,p,curve,target,arm,sat,pairs_or_triples,mults,decisions,work,status").unwrap(); }
    for s in s_lo..=s_hi {
        let b_win = 1u64 << s;
        let mut prng = Rng(0xC0FFEE ^ ((s as u64) << 40));
        let p = loop { let c = (1u64 << (3 * s)) + prng.below(1u64 << (3 * s)) | 1; if is_prime(c) && c > 5 { break c; } };
        eprintln!("s={} p={} (2^{:.2}) B={}", s, p, (p as f64).log2(), b_win);
        for ci in 0..ncurves {
            let mut crng = Rng(0xBEEF ^ ((s as u64) << 32) ^ ((ci as u64) << 8));
            let f0 = F::new(p);
            let curve = build_curve(&f0, b_win, &mut crng);
            let mut trng = Rng(0xFACE ^ ((s as u64) << 32) ^ ((ci as u64) << 8));
            let tseeds: Vec<u64> = (0..ntargets).map(|_| trng.next()).collect();
            eprintln!("  curve {} a={} b={} liftable x in window: {}", ci, curve.a, curve.b, curve.pts.len());
            for arm in ['G', 'S', 'T'] {
                if arm == 'T' && s > t_max { continue; }
                let start = Instant::now();
                let next = AtomicUsize::new(0);
                let rows: Mutex<Vec<Row>> = Mutex::new(vec![]);
                std::thread::scope(|sc| {
                    for _ in 0..nthreads {
                        sc.spawn(|| {
                            loop {
                                let ti = next.fetch_add(1, Ordering::SeqCst);
                                if ti >= ntargets { break; }
                                if start.elapsed().as_secs_f64() > cell_limit {
                                    rows.lock().unwrap().push(Row { s, p, curve: ci, target: ti, arm, sat: -1, pairs: 0, mults: 0, dec: 0, status: "TIMEOUT_20MIN_NOT_RUN" });
                                    continue;
                                }
                                let mut f = F::new(p);
                                f.rng ^= tseeds[ti];
                                let mut rr = Rng(tseeds[ti]);
                                let t = build_target(&mut f, &curve, b_win, &mut rr);
                                f.m = 0; f.dec = 0;
                                // per-target precompute of g polys is counted: redo it counted
                                let (sat, pairs) = {
                                    // count target precompute (about 8 mults) inside attempt
                                    f.m += 8;
                                    match arm { 'G' => arm_g(&mut f, &curve, &t, b_win), 'S' => arm_s(&mut f, &curve, &t, b_win), _ => arm_t(&mut f, &curve, &t) }
                                };
                                rows.lock().unwrap().push(Row { s, p, curve: ci, target: ti, arm, sat: sat as i32, pairs, mults: f.m, dec: f.dec, status: "OK" });
                            }
                        });
                    }
                });
                let mut rows = rows.into_inner().unwrap();
                rows.sort_by_key(|r| r.target);
                let (mut ok, mut sat) = (0, 0);
                for r in &rows {
                    if r.status == "OK" { ok += 1; if r.sat == 1 { sat += 1; } }
                    writeln!(csv, "{},{},{},{},{},{},{},{},{},{},{}", r.s, r.p, r.curve, r.target, r.arm, r.sat, r.pairs, r.mults, r.dec, r.mults + r.dec, r.status).unwrap();
                }
                csv.flush().unwrap();
                eprintln!("    arm {} done: {} ok, {} sat, {:.1}s wall (stop guard only)", arm, ok, sat, start.elapsed().as_secs_f64());
            }
        }
    }
}
