//! F_p, F_p[t] and F_{p^n} = F_p[t]/(t^n - sum m_i t^i), n <= 4, p < 2^31.

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    pub fn below128(&mut self, n: u128) -> u128 {
        (((self.next() as u128) << 64) | self.next() as u128) % n
    }
}

/// Derived seed: every (experiment, size, index) gets its own stream.
pub fn derive(seed: u64, tag: &str, a: u64, b: u64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325 ^ seed;
    for byte in tag.bytes().chain(a.to_le_bytes()).chain(b.to_le_bytes()) {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    Rng(h).next()
}

#[inline]
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn powm(mut a: u64, mut e: u128, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero");
    powm(a, (p - 2) as u128, p)
}
pub fn mulmod128(a: u128, b: u128, m: u128) -> u128 {
    // m < 2^64 in all uses
    let (mut a, mut b, mut r) = (a % m, b, 0u128);
    while b > 0 {
        if b & 1 == 1 {
            r = (r + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    r
}
pub fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for sp in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % sp == 0 {
            return n == sp;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = 1u128;
        let (mut b, mut e) = (a % n, d);
        while e > 0 {
            if e & 1 == 1 {
                x = mulmod128(x, b, n);
            }
            b = mulmod128(b, b, n);
            e >>= 1;
        }
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod128(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
pub fn factor(mut n: u128) -> Vec<(u128, u32)> {
    let mut out = vec![];
    let mut d = 2u128;
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
        if d > 1 << 24 && is_prime(n) {
            break;
        }
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if powm(a, ((p - 1) / 2) as u128, p) != 1 {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powm(z, ((p - 1) / 2) as u128, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powm(z, q as u128, p);
    let mut t = powm(a, q as u128, p);
    let mut r = powm(a, ((q + 1) / 2) as u128, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u128 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

// ---------------- univariate polynomials over F_p (low -> high) ----------------
pub fn ptrim(mut f: Vec<u64>) -> Vec<u64> {
    while f.last() == Some(&0) {
        f.pop();
    }
    f
}
pub fn pmul(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    if f.is_empty() || g.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; f.len() + g.len() - 1];
    for (i, &a) in f.iter().enumerate() {
        if a == 0 {
            continue;
        }
        for (j, &b) in g.iter().enumerate() {
            r[i + j] = (r[i + j] + mulm(a, b, p)) % p;
        }
    }
    ptrim(r)
}
pub fn prem(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    let mut r = ptrim(f.to_vec());
    let g = ptrim(g.to_vec());
    let dg = g.len() - 1;
    let li = invm(g[dg], p);
    while r.len() > dg {
        let dr = r.len() - 1;
        let c = mulm(r[dr], li, p);
        for i in 0..=dg {
            let idx = dr - dg + i;
            r[idx] = (r[idx] + p - mulm(c, g[i], p)) % p;
        }
        r = ptrim(r);
    }
    r
}
pub fn pgcd(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (ptrim(f.to_vec()), ptrim(g.to_vec()));
    while !b.is_empty() {
        let r = prem(&a, &b, p);
        a = b;
        b = r;
    }
    if let Some(&l) = a.last() {
        let li = invm(l, p);
        a.iter_mut().for_each(|c| *c = mulm(*c, li, p));
    }
    a
}
pub fn ppowmod(base: &[u64], mut e: u128, m: &[u64], p: u64) -> Vec<u64> {
    let mut r = vec![1u64];
    let mut b = prem(base, m, p);
    while e > 0 {
        if e & 1 == 1 {
            r = prem(&pmul(&r, &b, p), m, p);
        }
        b = prem(&pmul(&b, &b, p), m, p);
        e >>= 1;
    }
    r
}
pub fn peval(f: &[u64], x: u64, p: u64) -> u64 {
    f.iter().rev().fold(0, |acc, &c| (mulm(acc, x, p) + c) % p)
}
/// Distinct roots in F_p (p odd) of f.
pub fn proots(f: &[u64], p: u64, rng: &mut Rng) -> Vec<u64> {
    let f = ptrim(f.to_vec());
    if f.len() <= 1 {
        return vec![];
    }
    let xp = ppowmod(&[0, 1], p as u128, &f, p);
    let mut xpmx = xp.clone();
    if xpmx.len() < 2 {
        xpmx.resize(2, 0);
    }
    xpmx[1] = (xpmx[1] + p - 1) % p;
    let g = pgcd(&f, &ptrim(xpmx), p);
    let mut out = vec![];
    split(&g, p, rng, &mut out);
    out.sort();
    out
}
fn split(g: &[u64], p: u64, rng: &mut Rng, out: &mut Vec<u64>) {
    let d = g.len().saturating_sub(1);
    if d == 0 {
        return;
    }
    if d == 1 {
        out.push((p - mulm(g[0], invm(g[1], p), p)) % p);
        return;
    }
    loop {
        let delta = rng.below(p);
        let h = ppowmod(&[delta, 1], ((p - 1) / 2) as u128, g, p);
        let mut h1 = h.clone();
        if h1.is_empty() {
            h1.push(0);
        }
        h1[0] = (h1[0] + p - 1) % p;
        let a = pgcd(g, &ptrim(h1), p);
        let da = a.len().saturating_sub(1);
        if da > 0 && da < d {
            let b = pdiv_exact(g, &a, p);
            split(&a, p, rng, out);
            split(&b, p, rng, out);
            return;
        }
    }
}
pub fn pdiv_exact(f: &[u64], g: &[u64], p: u64) -> Vec<u64> {
    let mut r = ptrim(f.to_vec());
    let dg = g.len() - 1;
    let li = invm(g[dg], p);
    let mut q = vec![0u64; r.len() - dg];
    while r.len() > dg {
        let dr = r.len() - 1;
        let c = mulm(r[dr], li, p);
        q[dr - dg] = c;
        for i in 0..=dg {
            let idx = dr - dg + i;
            r[idx] = (r[idx] + p - mulm(c, g[i], p)) % p;
        }
        r = ptrim(r);
    }
    ptrim(q)
}

// ---------------- extension field ----------------
pub type E = [u64; 4];

#[derive(Clone)]
pub struct Fq {
    pub p: u64,
    pub n: usize,
    /// t^n = sum_{i<n} m[i] t^i
    pub m: [u64; 4],
    pub q: u128,
    nonres: E,
}

impl Fq {
    pub fn new(p: u64, n: usize, rng: &mut Rng) -> Fq {
        assert!(p > 2 && p < (1 << 31) && (1..=4).contains(&n));
        let q = (p as u128).pow(n as u32);
        let mut m = [0u64; 4];
        if n > 1 {
            loop {
                for c in m.iter_mut().take(n) {
                    *c = rng.below(p);
                }
                // modulus f(t) = t^n - sum m_i t^i ; irreducible test (n<=3: no roots; n=4: no roots and no quadratic factor)
                let mut f = vec![0u64; n + 1];
                for i in 0..n {
                    f[i] = (p - m[i]) % p;
                }
                f[n] = 1;
                if irreducible(&f, p) {
                    break;
                }
            }
        }
        let mut fq = Fq { p, n, m, q, nonres: [0; 4] };
        loop {
            let z = fq.rand(rng);
            if !fq.is_zero(&z) && !fq.is_square(&z) {
                fq.nonres = z;
                break;
            }
        }
        fq
    }
    pub fn zero(&self) -> E {
        [0; 4]
    }
    pub fn one(&self) -> E {
        [1, 0, 0, 0]
    }
    pub fn c(&self, v: u64) -> E {
        [v % self.p, 0, 0, 0]
    }
    pub fn ci(&self, v: i64) -> E {
        let p = self.p as i64;
        [(((v % p) + p) % p) as u64, 0, 0, 0]
    }
    pub fn is_zero(&self, a: &E) -> bool {
        a.iter().all(|&x| x == 0)
    }
    pub fn add(&self, a: &E, b: &E) -> E {
        let mut r = [0; 4];
        for i in 0..self.n {
            let s = a[i] + b[i];
            r[i] = if s >= self.p { s - self.p } else { s };
        }
        r
    }
    pub fn sub(&self, a: &E, b: &E) -> E {
        let mut r = [0; 4];
        for i in 0..self.n {
            r[i] = if a[i] >= b[i] { a[i] - b[i] } else { a[i] + self.p - b[i] };
        }
        r
    }
    pub fn neg(&self, a: &E) -> E {
        self.sub(&[0; 4], a)
    }
    pub fn mul(&self, a: &E, b: &E) -> E {
        let (n, p) = (self.n, self.p);
        let mut c = [0u64; 8];
        for i in 0..n {
            if a[i] == 0 {
                continue;
            }
            for j in 0..n {
                c[i + j] = (c[i + j] + a[i] * b[j]) % p;
            }
        }
        for k in (n..2 * n - 1).rev() {
            let co = c[k];
            if co != 0 {
                c[k] = 0;
                for i in 0..n {
                    c[k - n + i] = (c[k - n + i] + co * self.m[i]) % p;
                }
            }
        }
        [c[0], c[1], c[2], c[3]]
    }
    pub fn sq(&self, a: &E) -> E {
        self.mul(a, a)
    }
    pub fn pow(&self, a: &E, mut e: u128) -> E {
        let mut r = self.one();
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
    pub fn inv(&self, a: &E) -> E {
        assert!(!self.is_zero(a), "Fq inverse of zero");
        self.pow(a, self.q - 2)
    }
    pub fn is_square(&self, a: &E) -> bool {
        self.is_zero(a) || self.pow(a, (self.q - 1) / 2) == self.one()
    }
    pub fn sqrt(&self, a: &E) -> Option<E> {
        if self.is_zero(a) {
            return Some(self.zero());
        }
        if !self.is_square(a) {
            return None;
        }
        let (mut t, mut s) = (self.q - 1, 0u32);
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut m = s;
        let mut c = self.pow(&self.nonres, t);
        let mut tt = self.pow(a, t);
        let mut r = self.pow(a, (t + 1) / 2);
        let one = self.one();
        while tt != one {
            let mut i = 0;
            let mut x = tt;
            while x != one {
                x = self.sq(&x);
                i += 1;
            }
            let b = self.pow(&c, 1u128 << (m - i - 1));
            m = i;
            c = self.sq(&b);
            tt = self.mul(&tt, &c);
            r = self.mul(&r, &b);
        }
        Some(r)
    }
    pub fn rand(&self, rng: &mut Rng) -> E {
        let mut r = [0; 4];
        for c in r.iter_mut().take(self.n) {
            *c = rng.below(self.p);
        }
        r
    }
    pub fn in_base(&self, a: &E) -> bool {
        a[1..].iter().all(|&x| x == 0)
    }
    /// canonical representative of {y, -y}: lexicographically smaller array
    pub fn canon_sign(&self, y: &E) -> (E, bool) {
        let ny = self.neg(y);
        let ky: Vec<u64> = y.iter().rev().cloned().collect();
        let kn: Vec<u64> = ny.iter().rev().cloned().collect();
        if ky <= kn {
            (*y, true)
        } else {
            (ny, false)
        }
    }
}

fn irreducible(f: &[u64], p: u64) -> bool {
    let n = f.len() - 1;
    // x^{p^k} - x has no common factor with f for k <= n/2
    let mut xpk = vec![0u64, 1];
    for _ in 0..n / 2 {
        xpk = ppowmod(&xpk, p as u128, f, p);
        let mut d = xpk.clone();
        d.resize(d.len().max(2), 0);
        d[1] = (d[1] + p - 1) % p;
        let g = pgcd(f, &ptrim(d), p);
        if g.len() > 1 {
            return false;
        }
    }
    true
}
