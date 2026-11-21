// jcount: point counter for smooth plane quartics over F_q (q odd prime, q < 2^31, q > 3).
// Build: rustc --edition 2021 -O jcount.rs     (no crates, std only)
//
// For a quartic F(x,y,z) = sum f[i] * mon_i with the monomial order of
// pic::mons_of_degree(4) (f[0] = x^4, f[1] = x^3 y, f[2] = x^3 z, ...), it counts
// N_k = #C(F_{q^k}) for k = 1,2,3 (projective points), forms the L-polynomial of the
// Frobenius characteristic polynomial P(T) (same Newton recursion as
// cryptanalysis curve_construction/lpoly.rs char_poly_from_counts) and prints #J = P(1)
// and its factorisation.
//
// Method: for every x in F_{Q}, Q = q^k, the affine points over x are the distinct roots of
// g(y) = F(x,y,1) in F_Q, counted as deg gcd(g, y^Q - y). y^Q mod g is obtained with one
// square-and-multiply for y^q and (k-1) q-Frobenius applications, each a linear map in the
// powers (y^q)^i mod g (so log2(Q) = k log2(q) squarings become log2(q) squarings plus
// (k-1) d x d matrix steps). x is only enumerated up to Frobenius orbits. gcd degrees use
// pseudo-remainder sequences (no inversions). Points at z = 0 are counted from the binary
// quartic F(X,Y,0). No square root is needed (so q = 1 mod 4 and q = 3 mod 4 are alike);
// `tonelli` is only used by the optional `points` subcommand.
#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

// ---------------------------------------------------------------- F_q
#[derive(Clone, Copy)]
struct Fq {
    q: u64,
    mu: u64,
}
impl Fq {
    fn new(q: u64) -> Fq {
        assert!(q > 3 && q < (1 << 31));
        Fq { q, mu: u64::MAX / q }
    }
    #[inline(always)]
    fn red(&self, n: u64) -> u64 {
        let h = ((n as u128 * self.mu as u128) >> 64) as u64;
        let r = n - h * self.q;
        if r >= self.q {
            r - self.q
        } else {
            r
        }
    }
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.q {
            s - self.q
        } else {
            s
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.q - b
        }
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.red(a * b)
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a % self.q != 0);
        let (mut r0, mut r1) = (self.q as i64, a as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let qq = r0 / r1;
            let r2 = r0 - qq * r1;
            r0 = r1;
            r1 = r2;
            let t2 = t0 - qq * t1;
            t0 = t1;
            t1 = t2;
        }
        assert_eq!(r0, 1);
        (((t0 % self.q as i64) + self.q as i64) % self.q as i64) as u64
    }
    fn is_square(&self, a: u64) -> bool {
        a == 0 || self.pow(a, (self.q - 1) / 2) == 1
    }
    /// Tonelli-Shanks square root (any odd prime q).
    fn tonelli(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if !self.is_square(a) {
            return None;
        }
        let q = self.q;
        let mut s = 0;
        let mut t = q - 1;
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        if s == 1 {
            return Some(self.pow(a, (q + 1) / 4));
        }
        let mut z = 2;
        while self.is_square(z) {
            z += 1;
        }
        let mut c = self.pow(z, t);
        let mut r = self.pow(a, (t + 1) / 2);
        let mut tt = self.pow(a, t);
        let mut m = s;
        while tt != 1 {
            let mut i = 0;
            let mut x = tt;
            while x != 1 {
                x = self.mul(x, x);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            r = self.mul(r, b);
            c = self.mul(b, b);
            tt = self.mul(tt, c);
            m = i;
        }
        Some(r)
    }
}

// ---------------------------------------------------------------- F_{q^k}, k <= 3
type E = [u64; 3];
const EZ: E = [0, 0, 0];
const E1: E = [1, 0, 0];

struct Ext {
    f: Fq,
    k: usize,
    m0: u64, // k=2: w^2 = m0 ; k=3: w^3 = m1 w + m0
    m1: u64,
    fr: [E; 3], // fr[i] = (w^i)^q
}
impl Ext {
    fn new(q: u64, k: usize) -> Ext {
        let f = Fq::new(q);
        let (mut m0, mut m1) = (0, 0);
        if k == 2 {
            m0 = 2;
            while f.is_square(m0) {
                m0 += 1;
            }
        } else if k == 3 {
            'outer: for a in 0..q {
                for b in 1..q {
                    // x^3 - a x - b irreducible iff no root
                    let mut ok = true;
                    for x in 0..q {
                        let v = f.sub(f.sub(f.mul(f.mul(x, x), x), f.mul(a, x)), b);
                        if v == 0 {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        m1 = a;
                        m0 = b;
                        break 'outer;
                    }
                }
            }
            assert!(m0 != 0, "no irreducible cubic");
        }
        let mut e = Ext { f, k, m0, m1, fr: [E1, EZ, EZ] };
        if k >= 2 {
            let w: E = [0, 1, 0];
            let wq = e.pow(&w, q);
            e.fr[1] = wq;
            e.fr[2] = e.mul(&wq, &wq);
        }
        e
    }
    #[inline(always)]
    fn add(&self, a: &E, b: &E) -> E {
        [self.f.add(a[0], b[0]), self.f.add(a[1], b[1]), self.f.add(a[2], b[2])]
    }
    #[inline(always)]
    fn sub(&self, a: &E, b: &E) -> E {
        [self.f.sub(a[0], b[0]), self.f.sub(a[1], b[1]), self.f.sub(a[2], b[2])]
    }
    #[inline(always)]
    fn is_zero(a: &E) -> bool {
        a[0] == 0 && a[1] == 0 && a[2] == 0
    }
    #[inline(always)]
    fn scal(&self, a: &E, s: u64) -> E {
        [self.f.mul(a[0], s), self.f.mul(a[1], s), self.f.mul(a[2], s)]
    }
    #[inline(always)]
    fn mul(&self, a: &E, b: &E) -> E {
        let f = &self.f;
        match self.k {
            1 => [f.mul(a[0], b[0]), 0, 0],
            2 => {
                let t = f.red(a[1] * b[1]);
                [f.red(a[0] * b[0] + t * self.m0), f.red(a[0] * b[1] + a[1] * b[0]), 0]
            }
            _ => {
                let p0 = f.red(a[0] * b[0]);
                let p1 = f.red(a[0] * b[1] + a[1] * b[0]);
                let p2 = f.red(a[0] * b[2] + a[1] * b[1] + a[2] * b[0]);
                let p3 = f.red(a[1] * b[2] + a[2] * b[1]);
                let p4 = f.red(a[2] * b[2]);
                [
                    f.red(p0 + p3 * self.m0),
                    f.red(p1 + p3 * self.m1 + p4 * self.m0),
                    f.red(p2 + p4 * self.m1),
                ]
            }
        }
    }
    fn pow(&self, a: &E, mut e: u64) -> E {
        let mut r = E1;
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    /// q-Frobenius
    #[inline(always)]
    fn sigma(&self, a: &E) -> E {
        let f = &self.f;
        match self.k {
            1 => *a,
            2 => [
                f.red(a[0] + a[1] * self.fr[1][0]),
                f.red(a[1] * self.fr[1][1]),
                0,
            ],
            _ => {
                let mut c = [0u64; 3];
                for j in 0..3 {
                    c[j] = f.red(a[0] * self.fr[0][j] + a[1] * self.fr[1][j] + a[2] * self.fr[2][j]);
                }
                c
            }
        }
    }
    fn inv(&self, a: &E) -> E {
        match self.k {
            1 => [self.f.inv(a[0]), 0, 0],
            2 => {
                let s = self.sigma(a);
                let n = self.mul(a, &s)[0];
                self.scal(&s, self.f.inv(n))
            }
            _ => {
                let s1 = self.sigma(a);
                let s2 = self.sigma(&s1);
                let t = self.mul(&s1, &s2);
                let n = self.mul(a, &t)[0];
                self.scal(&t, self.f.inv(n))
            }
        }
    }
    fn code(&self, a: &E) -> u64 {
        let q = self.f.q;
        a[0] + q * a[1] + q * q * a[2]
    }
    fn decode(&self, mut c: u64) -> E {
        let q = self.f.q;
        let a0 = c % q;
        c /= q;
        let a1 = c % q;
        c /= q;
        [a0, a1, c]
    }
}

// ---------------------------------------------------------------- polynomials over F_Q
fn deg(p: &[E; 5]) -> i32 {
    for i in (0..5).rev() {
        if !Ext::is_zero(&p[i]) {
            return i as i32;
        }
    }
    -1
}

fn prem(ex: &Ext, a: &[E; 5], b: &[E; 5]) -> [E; 5] {
    let db = deg(b);
    let lb = b[db as usize];
    let mut r = *a;
    loop {
        let dr = deg(&r);
        if dr < db {
            return r;
        }
        let lr = r[dr as usize];
        for i in 0..5 {
            r[i] = ex.mul(&lb, &r[i]);
        }
        let sh = (dr - db) as usize;
        for i in 0..=(db as usize) {
            let t = ex.mul(&lr, &b[i]);
            r[i + sh] = ex.sub(&r[i + sh], &t);
        }
    }
}

fn gcd_deg(ex: &Ext, a0: &[E; 5], b0: &[E; 5]) -> i32 {
    let (mut a, mut b) = (*a0, *b0);
    while deg(&b) >= 0 {
        let r = prem(ex, &a, &b);
        a = b;
        b = r;
    }
    deg(&a)
}

type P4 = [E; 4];

/// a*b mod g (g monic of degree d, 2 <= d <= 4; a,b of degree < d)
fn mulmod(ex: &Ext, a: &P4, b: &P4, g: &[E; 5], d: usize) -> P4 {
    let mut prod = [EZ; 7];
    for i in 0..d {
        if Ext::is_zero(&a[i]) {
            continue;
        }
        for j in 0..d {
            if Ext::is_zero(&b[j]) {
                continue;
            }
            let t = ex.mul(&a[i], &b[j]);
            prod[i + j] = ex.add(&prod[i + j], &t);
        }
    }
    for t in (d..=(2 * d - 2)).rev() {
        let c = prod[t];
        if Ext::is_zero(&c) {
            continue;
        }
        for j in 0..d {
            let u = ex.mul(&c, &g[j]);
            prod[t - d + j] = ex.sub(&prod[t - d + j], &u);
        }
    }
    [prod[0], prod[1], prod[2], prod[3]]
}

fn muly(ex: &Ext, a: &P4, g: &[E; 5], d: usize) -> P4 {
    let top = a[d - 1];
    let mut r = [EZ; 4];
    for i in 1..d {
        r[i] = a[i - 1];
    }
    if !Ext::is_zero(&top) {
        for j in 0..d {
            let u = ex.mul(&top, &g[j]);
            r[j] = ex.sub(&r[j], &u);
        }
    }
    r
}

/// (sum h_i y^i)^q = sum sigma(h_i) S_i
fn frobpoly(ex: &Ext, h: &P4, s: &[P4; 4], d: usize) -> P4 {
    let mut r = [EZ; 4];
    for i in 0..d {
        if Ext::is_zero(&h[i]) {
            continue;
        }
        let c = ex.sigma(&h[i]);
        for j in 0..d {
            let t = ex.mul(&c, &s[i][j]);
            r[j] = ex.add(&r[j], &t);
        }
    }
    r
}

/// number of distinct roots in F_Q of g (coefficients g[0..5]); -1 if g == 0.
fn roots_count(ex: &Ext, g: &[E; 5]) -> i64 {
    let d = deg(g);
    if d < 0 {
        return -1;
    }
    if d == 0 {
        return 0;
    }
    if d == 1 {
        return 1;
    }
    let d = d as usize;
    // monic
    let li = ex.inv(&g[d]);
    let mut gm = [EZ; 5];
    for i in 0..=d {
        gm[i] = ex.mul(&g[i], &li);
    }
    // s = y^q mod gm
    let q = ex.f.q;
    let nb = 64 - q.leading_zeros() as usize;
    let mut s: P4 = [EZ; 4];
    s[1] = E1;
    for bit in (0..nb - 1).rev() {
        s = mulmod(ex, &s, &s, &gm, d);
        if (q >> bit) & 1 == 1 {
            s = muly(ex, &s, &gm, d);
        }
    }
    let mut h = s;
    if ex.k >= 2 {
        let mut sp = [[EZ; 4]; 4];
        sp[0][0] = E1;
        sp[1] = s;
        for i in 2..d {
            sp[i] = mulmod(ex, &sp[i - 1], &s, &gm, d);
        }
        for _ in 1..ex.k {
            h = frobpoly(ex, &h, &sp, d);
        }
    }
    // u = h - y
    let mut u5 = [EZ; 5];
    for i in 0..d {
        u5[i] = h[i];
    }
    u5[1] = ex.sub(&u5[1], &E1);
    if deg(&u5) < 0 {
        return d as i64;
    }
    gcd_deg(ex, &gm, &u5) as i64
}

// ---------------------------------------------------------------- curve
fn mons4() -> Vec<(usize, usize, usize)> {
    let mut v = Vec::new();
    for a in (0..=4).rev() {
        for b in (0..=(4 - a)).rev() {
            v.push((a, b, 4 - a - b));
        }
    }
    v
}

struct Curve {
    q: u64,
    f: [u64; 15],
    idx: [[usize; 5]; 5], // idx[a][b] = index of monomial x^a y^b z^(4-a-b)
}
impl Curve {
    fn new(q: u64, f: &[u64]) -> Curve {
        assert_eq!(f.len(), 15);
        let mut c = Curve { q, f: [0; 15], idx: [[99; 5]; 5] };
        for (i, &(a, b, _)) in mons4().iter().enumerate() {
            c.idx[a][b] = i;
            c.f[i] = f[i] % q;
        }
        c
    }
    fn co(&self, a: usize, b: usize) -> u64 {
        self.f[self.idx[a][b]]
    }
}

fn smooth(c: &Curve) -> bool {
    // (F_X,F_Y,F_Z)_7 = S_7  <=> smooth (see pic::smoothness_certificate)
    let fq = Fq::new(c.q);
    let m4 = mons4();
    // partial derivative coefficient on degree-3 monomial (a,b,c)
    let mut parts = vec![[[[0u64; 4]; 4]; 4]; 3];
    for (i, &(a, b, cc)) in m4.iter().enumerate() {
        let co = c.f[i];
        if co == 0 {
            continue;
        }
        if a > 0 {
            parts[0][a - 1][b][cc] = fq.mul(co, a as u64);
        }
        if b > 0 {
            parts[1][a][b - 1][cc] = fq.mul(co, b as u64);
        }
        if cc > 0 {
            parts[2][a][b][cc - 1] = fq.mul(co, cc as u64);
        }
    }
    let mut col = [[0usize; 8]; 8];
    let mut nc = 0;
    for a in 0..8 {
        for b in 0..(8 - a) {
            col[a][b] = nc;
            nc += 1;
        }
    }
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for p in 0..3 {
        for &(a2, b2, _) in &m4 {
            let mut r = vec![0u64; nc];
            for a in 0..4 {
                for b in 0..(4 - a) {
                    for cc in 0..(4 - a - b) {
                        let v = parts[p][a][b][cc];
                        if a + b + cc == 3 && v != 0 {
                            r[col[a + a2][b + b2]] = v;
                        }
                    }
                }
            }
            rows.push(r);
        }
    }
    let mut rank = 0;
    for cidx in 0..nc {
        let Some(pv) = (rank..rows.len()).find(|&i| rows[i][cidx] != 0) else { continue };
        rows.swap(rank, pv);
        let inv = fq.inv(rows[rank][cidx]);
        for j in cidx..nc {
            rows[rank][j] = fq.mul(rows[rank][j], inv);
        }
        for i in 0..rows.len() {
            if i != rank && rows[i][cidx] != 0 {
                let m = rows[i][cidx];
                for j in cidx..nc {
                    let t = fq.mul(m, rows[rank][j]);
                    rows[i][j] = fq.sub(rows[i][j], t);
                }
            }
        }
        rank += 1;
    }
    rank == nc
}

// ---------------------------------------------------------------- counting
fn count_infinity(c: &Curve, ex: &Ext) -> i64 {
    let mut g = [EZ; 5];
    for b in 0..=4 {
        g[b] = [c.co(4 - b, b), 0, 0];
    }
    let r = roots_count(ex, &g);
    assert!(r >= 0, "z=0 is a component: singular");
    r + if c.co(0, 4) == 0 { 1 } else { 0 }
}

fn worker(c: &Curve, ex: &Ext, next: &AtomicU64, total: u64) -> i64 {
    const CH: u64 = 1 << 12;
    let k = ex.k;
    let mut sum = 0i64;
    loop {
        let s = next.fetch_add(CH, Ordering::Relaxed);
        if s >= total {
            return sum;
        }
        let e = (s + CH).min(total);
        for code in s..e {
            let x = ex.decode(code);
            let mut wgt = 1i64;
            if k > 1 && (x[1] != 0 || x[2] != 0) {
                let s1 = ex.sigma(&x);
                if ex.code(&s1) < code {
                    continue;
                }
                if k == 3 {
                    let s2 = ex.sigma(&s1);
                    if ex.code(&s2) < code {
                        continue;
                    }
                }
                wgt = k as i64;
            }
            let x2 = ex.mul(&x, &x);
            let x3 = ex.mul(&x2, &x);
            let x4 = ex.mul(&x3, &x);
            let xp = [E1, x, x2, x3, x4];
            let mut g = [EZ; 5];
            for b in 0..=4 {
                let mut acc = EZ;
                for a in 0..=(4 - b) {
                    let co = c.co(a, b);
                    if co != 0 {
                        let t = ex.scal(&xp[a], co);
                        acc = ex.add(&acc, &t);
                    }
                }
                g[b] = acc;
            }
            let r = roots_count(ex, &g);
            assert!(r >= 0, "vertical line component: singular");
            sum += wgt * r;
        }
    }
}

fn count_k(c: &Curve, k: usize, threads: usize) -> (i64, f64) {
    let t0 = Instant::now();
    let ex = Ext::new(c.q, k);
    let total = c.q.pow(k as u32);
    let next = AtomicU64::new(0);
    let aff: i64 = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..threads).map(|_| sc.spawn(|| worker(c, &ex, &next, total))).collect();
        hs.into_iter().map(|h| h.join().unwrap()).sum()
    });
    let n = aff + count_infinity(c, &ex);
    (n, t0.elapsed().as_secs_f64())
}

/// independent brute force: evaluate F at every point of P^2(F_{q^k})
fn brute_k(c: &Curve, k: usize) -> i64 {
    let ex = Ext::new(c.q, k);
    let total = c.q.pow(k as u32);
    let m4 = mons4();
    let eval = |x: &E, y: &E, z: &E| -> bool {
        let mut acc = EZ;
        for (i, &(a, b, cc)) in m4.iter().enumerate() {
            if c.f[i] == 0 {
                continue;
            }
            let mut t = E1;
            for _ in 0..a {
                t = ex.mul(&t, x);
            }
            for _ in 0..b {
                t = ex.mul(&t, y);
            }
            for _ in 0..cc {
                t = ex.mul(&t, z);
            }
            let t = ex.scal(&t, c.f[i]);
            acc = ex.add(&acc, &t);
        }
        Ext::is_zero(&acc)
    };
    let one = E1;
    let zero = EZ;
    let mut n = 0i64;
    for xc in 0..total {
        let x = ex.decode(xc);
        for yc in 0..total {
            let y = ex.decode(yc);
            if eval(&x, &y, &one) {
                n += 1;
            }
        }
        if eval(&x, &one, &zero) {
            n += 1;
        }
    }
    if eval(&one, &zero, &zero) {
        n += 1;
    }
    n
}

// ---------------------------------------------------------------- L-polynomial
/// c[0..=6]: P(T) = sum c_i T^(6-i); None if Newton integrality fails.
fn lpoly(n: &[i64; 3], q: i64) -> Option<[i128; 7]> {
    let q = q as i128;
    let s: Vec<i128> = (0..3).map(|i| n[i] as i128 - q.pow(i as u32 + 1) - 1).collect();
    let mut c = [0i128; 7];
    c[0] = 1;
    for k in 1..=3usize {
        let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
        if num % k as i128 != 0 {
            return None;
        }
        c[k] = num / k as i128;
    }
    for j in 1..=3usize {
        c[3 + j] = q.pow(j as u32) * c[3 - j];
    }
    Some(c)
}
fn jorder(c: &[i128; 7]) -> i128 {
    c.iter().sum()
}

// ---------------------------------------------------------------- factoring (u64)
fn mulmod64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn powmod64(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod64(r, a, m);
        }
        a = mulmod64(a, a, m);
        e >>= 1;
    }
    r
}
fn is_prime64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod64(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod64(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}
fn gcd64(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd64(b, a % b)
    }
}
fn rho64(n: u64) -> u64 {
    let mut c = 1u64;
    loop {
        let f = |x: u64| (mulmod64(x, x, n) + c) % n;
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd64(if x > y { x - y } else { y - x }, n);
        }
        if d != n {
            return d;
        }
        c += 1;
    }
}
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out: Vec<(u64, u32)> = Vec::new();
    let mut p = 2u64;
    while p < 100000 && p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    let mut stack = vec![n];
    let mut big: Vec<u64> = Vec::new();
    while let Some(m) = stack.pop() {
        if m == 1 {
            continue;
        }
        if is_prime64(m) {
            big.push(m);
        } else {
            let d = rho64(m);
            stack.push(d);
            stack.push(m / d);
        }
    }
    big.sort();
    for b in big {
        if let Some(l) = out.last_mut() {
            if l.0 == b {
                l.1 += 1;
                continue;
            }
        }
        out.push((b, 1));
    }
    out
}
fn fac_str(f: &[(u64, u32)]) -> String {
    f.iter().map(|(p, e)| if *e == 1 { format!("{}", p) } else { format!("{}^{}", p, e) }).collect::<Vec<_>>().join("*")
}

// ---------------------------------------------------------------- misc
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn rand_smooth(q: u64, rng: &mut Rng) -> Curve {
    loop {
        let f: Vec<u64> = (0..15).map(|_| rng.below(q)).collect();
        let c = Curve::new(q, &f);
        if smooth(&c) {
            return c;
        }
    }
}

fn parse_f(s: &str) -> Vec<u64> {
    s.split(',').map(|x| x.trim().parse::<u64>().unwrap()).collect()
}

struct Res {
    n: [i64; 3],
    t: [f64; 3],
    c: Option<[i128; 7]>,
}
fn full_count(c: &Curve, kmax: usize, threads: usize) -> Res {
    let mut n = [0i64; 3];
    let mut t = [0f64; 3];
    for k in 1..=kmax {
        let (nk, tk) = count_k(c, k, threads);
        n[k - 1] = nk;
        t[k - 1] = tk;
    }
    let cc = if kmax == 3 { lpoly(&n, c.q as i64) } else { None };
    Res { n, t, c: cc }
}

fn fmt_f(f: &[u64; 15]) -> String {
    format!("[{}]", f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

fn jline(c: &Curve, r: &Res, kmax: usize) -> String {
    let mut s = format!("{{\"rec\":\"jcount\",\"q\":{},\"f\":{},\"N\":[{}]", c.q, fmt_f(&c.f),
        r.n[..kmax].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    s += &format!(",\"t_s\":[{}]", r.t[..kmax].iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>().join(","));
    if kmax == 3 {
        match &r.c {
            None => s += ",\"lpoly\":null,\"err\":\"Newton integrality failed\"",
            Some(cc) => {
                let j = jorder(cc);
                s += &format!(",\"P_coeffs_T6_down\":[{}]", cc.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
                s += &format!(",\"J\":{}", j);
                if j > 0 && j < (1i128 << 64) {
                    s += &format!(",\"J_factors\":\"{}\"", fac_str(&factor(j as u64)));
                }
                let qf = c.q as f64;
                let lo = (qf.sqrt() - 1.0).powi(6);
                let hi = (qf.sqrt() + 1.0).powi(6);
                s += &format!(",\"weil_ok\":{}", (j as f64) >= lo && (j as f64) <= hi);
            }
        }
    }
    s + "}"
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: jcount count|brute|rand|jsonl|time|search|points ...");
        std::process::exit(2);
    }
    let threads: usize = arg(&args, "--threads").map(|s| s.parse().unwrap()).unwrap_or(1);
    let kmax: usize = arg(&args, "--kmax").map(|s| s.parse().unwrap()).unwrap_or(3);
    match args[1].as_str() {
        "count" => {
            let q: u64 = arg(&args, "--q").unwrap().parse().unwrap();
            let c = Curve::new(q, &parse_f(&arg(&args, "--f").unwrap()));
            if !smooth(&c) {
                println!("{{\"rec\":\"jcount\",\"q\":{},\"err\":\"singular\"}}", q);
                return;
            }
            let r = full_count(&c, kmax, threads);
            println!("{}", jline(&c, &r, kmax));
        }
        "brute" => {
            let q: u64 = arg(&args, "--q").unwrap().parse().unwrap();
            let c = Curve::new(q, &parse_f(&arg(&args, "--f").unwrap()));
            let n: Vec<i64> = (1..=kmax).map(|k| brute_k(&c, k)).collect();
            println!("{{\"rec\":\"brute\",\"q\":{},\"N\":{:?}}}", q, n);
        }
        // random smooth quartics: fast counter vs brute force (q <= 19)
        "rand" => {
            let q: u64 = arg(&args, "--q").unwrap().parse().unwrap();
            let seed: u64 = arg(&args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(1);
            let n: usize = arg(&args, "--n").map(|s| s.parse().unwrap()).unwrap_or(10);
            let mut rng = Rng(seed * 1000003 + q);
            let (mut ok, mut bad) = (0, 0);
            for _ in 0..n {
                let c = rand_smooth(q, &mut rng);
                let r = full_count(&c, kmax, 1);
                let b: Vec<i64> = (1..=kmax).map(|k| brute_k(&c, k)).collect();
                let same = (0..kmax).all(|i| r.n[i] == b[i]);
                let integ = kmax < 3 || r.c.is_some();
                if same && integ {
                    ok += 1;
                } else {
                    bad += 1;
                }
                println!("{{\"rec\":\"valid\",\"q\":{},\"f\":{},\"fast\":{:?},\"brute\":{:?},\"match\":{},\"newton_ok\":{}}}", q, fmt_f(&c.f), &r.n[..kmax], b, same, integ);
            }
            println!("{{\"rec\":\"valid_summary\",\"q\":{},\"curves\":{},\"match\":{},\"mismatch\":{}}}", q, n, ok, bad);
        }
        // instances.jsonl-style file: count each and compare to "order"
        "jsonl" => {
            let text = std::fs::read_to_string(&args[2]).unwrap();
            let qsel: Option<u64> = arg(&args, "--q").map(|s| s.parse().unwrap());
            let seedsel: Option<u64> = arg(&args, "--seed").map(|s| s.parse().unwrap());
            for line in text.lines() {
                let get = |key: &str| -> Option<String> {
                    let p = line.find(&format!("\"{}\":", key))?;
                    let rest = &line[p + key.len() + 3..];
                    if rest.starts_with('[') {
                        let e = rest.find(']')?;
                        Some(rest[1..e].to_string())
                    } else {
                        let e = rest.find(|ch| ch == ',' || ch == '}')?;
                        Some(rest[..e].to_string())
                    }
                };
                let q: u64 = get("q").unwrap().parse().unwrap();
                let seed: u64 = get("seed").unwrap().parse().unwrap();
                if qsel.map_or(false, |s| s != q) || seedsel.map_or(false, |s| s != seed) {
                    continue;
                }
                let order: u64 = get("order").unwrap().parse().unwrap();
                let c = Curve::new(q, &parse_f(&get("f").unwrap()));
                let sm = smooth(&c);
                let r = full_count(&c, 3, threads);
                let j = r.c.map(|cc| jorder(&cc));
                println!("{{\"rec\":\"ghs_check\",\"q\":{},\"seed\":{},\"smooth\":{},\"N\":{:?},\"J\":{},\"order\":{},\"equal\":{},\"t_s\":{:.2}}}", q, seed, sm, r.n, j.map_or("null".into(), |x| x.to_string()), order, j == Some(order as i128), r.t.iter().sum::<f64>());
            }
        }
        // timing vs q on a random smooth quartic
        "time" => {
            let qs: Vec<u64> = arg(&args, "--qs").unwrap().split(',').map(|x| x.parse().unwrap()).collect();
            for q in qs {
                let mut rng = Rng(7 * 1000003 + q);
                let c = rand_smooth(q, &mut rng);
                let r = full_count(&c, kmax, threads);
                let reps3 = (q as f64).powi(3) / 3.0;
                println!("{{\"rec\":\"time\",\"q\":{},\"threads\":{},\"N\":{:?},\"t_s\":{:?},\"ns_per_rep_k3\":{:.0}}}", q, threads, &r.n[..kmax], &r.t[..kmax], if kmax == 3 { r.t[2] * 1e9 * threads as f64 / reps3 } else { 0.0 });
            }
        }
        // search for cofactor <= maxcof Jacobians: ell prime, #J = cof * ell
        "search" => {
            let q: u64 = arg(&args, "--q").unwrap().parse().unwrap();
            let seed: u64 = arg(&args, "--seed").unwrap().parse().unwrap();
            let want: usize = arg(&args, "--want").map(|s| s.parse().unwrap()).unwrap_or(3);
            let maxcof: u64 = arg(&args, "--maxcof").map(|s| s.parse().unwrap()).unwrap_or(8);
            let mut rng = Rng(seed * 7919 + q * 31);
            let (mut found, mut tried) = (0, 0);
            let t0 = Instant::now();
            while found < want {
                let c = rand_smooth(q, &mut rng);
                let (n1, _) = count_k(&c, 1, 1);
                if n1 < 4 {
                    continue;
                }
                tried += 1;
                let r = full_count(&c, 3, threads);
                let Some(cc) = r.c else { continue };
                let j = jorder(&cc);
                let ju = j as u64;
                let mut hit = None;
                for cof in 1..=maxcof {
                    if ju % cof == 0 && ju / cof > maxcof && is_prime64(ju / cof) {
                        hit = Some(cof);
                        break;
                    }
                }
                println!("{{\"rec\":\"search_try\",\"q\":{},\"try\":{},\"J\":{},\"fac\":\"{}\",\"cof\":{},\"elapsed_s\":{:.1}}}", q, tried, j, fac_str(&factor(ju)), hit.map_or("null".into(), |x| x.to_string()), t0.elapsed().as_secs_f64());
                if let Some(cof) = hit {
                    found += 1;
                    println!("{{\"rec\":\"instance\",\"q\":{},\"seed\":{},\"idx\":{},\"f\":{},\"N\":{:?},\"order\":{},\"cof\":{},\"ell\":{}}}", q, seed, found, fmt_f(&c.f), r.n, j, cof, ju / cof);
                }
            }
        }
        "points" => {
            // list rational points via Tonelli-Shanks on the discriminant of the y-quadratic
            // is not implemented for a quartic; this subcommand only self-tests the sqrt.
            let q: u64 = arg(&args, "--q").unwrap().parse().unwrap();
            let f = Fq::new(q);
            let mut bad = 0;
            for a in 0..q.min(200000) {
                match f.tonelli(a) {
                    Some(r) => {
                        if f.mul(r, r) != a {
                            bad += 1
                        }
                    }
                    None => {
                        if f.is_square(a) {
                            bad += 1
                        }
                    }
                }
            }
            println!("{{\"rec\":\"tonelli_selftest\",\"q\":{},\"q_mod4\":{},\"bad\":{}}}", q, q % 4, bad);
        }
        _ => {
            eprintln!("unknown subcommand");
            std::process::exit(2);
        }
    }
}
