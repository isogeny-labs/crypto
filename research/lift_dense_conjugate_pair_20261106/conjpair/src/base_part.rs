


use std::fmt::Write as _;
use std::io::Write as _;

// ------------------------------------------------------------ bignum ----

type Big = Vec<u64>; // little-endian limbs

fn big_from(x: u64) -> Big {
    vec![x]
}
fn big_mul_small(a: &Big, m: u64) -> Big {
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry: u128 = 0;
    for &w in a {
        let v = w as u128 * m as u128 + carry;
        out.push(v as u64);
        carry = v >> 64;
    }
    if carry > 0 {
        out.push(carry as u64);
    }
    out
}
fn big_sub_small(a: &Big, m: u64) -> Big {
    let mut out = a.clone();
    let mut borrow = m;
    for w in out.iter_mut() {
        let (v, b) = w.overflowing_sub(borrow);
        *w = v;
        borrow = b as u64;
        if borrow == 0 {
            break;
        }
    }
    out
}
fn big_shr1(a: &Big) -> Big {
    let mut out = a.clone();
    for i in 0..out.len() {
        out[i] >>= 1;
        if i + 1 < a.len() {
            out[i] |= a[i + 1] << 63;
        }
    }
    out
}
fn big_bits(a: &Big) -> usize {
    for i in (0..a.len()).rev() {
        if a[i] != 0 {
            return i * 64 + 64 - a[i].leading_zeros() as usize;
        }
    }
    0
}
fn big_bit(a: &Big, i: usize) -> bool {
    i / 64 < a.len() && (a[i / 64] >> (i % 64)) & 1 == 1
}
fn big_pow_small(p: u64, e: u32) -> Big {
    let mut r = big_from(1);
    for _ in 0..e {
        r = big_mul_small(&r, p);
    }
    r
}

// ----------------------------------------------------------- F_p[z] ----

fn inv_p(a: u64, p: u64) -> u64 {
    // Fermat
    let (mut r, mut b, mut e) = (1u64, a % p, p - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

fn ptrim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

/// a mod m over F_p (m monic).
fn pmod(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = a.to_vec();
    ptrim(&mut r);
    let dm = m.len() - 1;
    while r.len() > dm {
        let c = *r.last().unwrap();
        let sh = r.len() - 1 - dm;
        for i in 0..=dm {
            r[sh + i] = (r[sh + i] + p - c * m[i] % p) % p;
        }
        ptrim(&mut r);
    }
    r
}
fn pmulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % p;
        }
    }
    pmod(&r, m, p)
}
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let lc = inv_p(*b.last().unwrap(), p);
        let bm: Vec<u64> = b.iter().map(|&c| c * lc % p).collect();
        let r = pmod(&a, &bm, p);
        a = b;
        b = r;
    }
    a
}

// --------------------------------------------------------- big field ----

#[derive(Clone)]
struct B {
    p: u64,
    d: usize,
    m: Vec<u64>, // monic, degree d
    q_all: Big,  // p^d
}

type El = Vec<u64>; // length d

impl B {
    /// Seeded search for an irreducible monic m of degree d; Rabin test.
    fn new(p: u64, d: usize, rng: &mut Rng) -> B {
        loop {
            let mut m: Vec<u64> = (0..d).map(|_| rng.below(p)).collect();
            m.push(1);
            if Self::rabin(&m, p, d) {
                return B {
                    p,
                    d,
                    m,
                    q_all: big_pow_small(p, d as u32),
                };
            }
        }
    }
    fn xpow_pk(m: &[u64], p: u64, k: usize) -> Vec<u64> {
        // x^(p^k) mod m by k p-th powerings
        let mut r = vec![0, 1];
        r = pmod(&r, m, p);
        for _ in 0..k {
            let base = r.clone();
            let mut acc = vec![1u64];
            let (mut b, mut e) = (base, p);
            while e > 0 {
                if e & 1 == 1 {
                    acc = pmulmod(&acc, &b, m, p);
                }
                b = pmulmod(&b, &b, m, p);
                e >>= 1;
            }
            r = acc;
        }
        r
    }
    fn rabin(m: &[u64], p: u64, d: usize) -> bool {
        let x = pmod(&[0, 1], m, p);
        if Self::xpow_pk(m, p, d) != x {
            return false;
        }
        let mut dd = d;
        let mut r = 2;
        let mut primes = vec![];
        while r * r <= dd {
            if dd % r == 0 {
                primes.push(r);
                while dd % r == 0 {
                    dd /= r;
                }
            }
            r += 1;
        }
        if dd > 1 {
            primes.push(dd);
        }
        for r in primes {
            let mut h = Self::xpow_pk(m, p, d / r);
            while h.len() < 2 {
                h.push(0);
            }
            h[1] = (h[1] + p - 1) % p;
            let g = pgcd(&h, m, p);
            if g.len() != 1 {
                return false;
            }
        }
        true
    }
    fn zero(&self) -> El {
        vec![0; self.d]
    }
    fn one(&self) -> El {
        let mut o = self.zero();
        o[0] = 1;
        o
    }
    fn c(&self, v: u64) -> El {
        let mut o = self.zero();
        o[0] = v % self.p;
        o
    }
    fn is_zero(&self, a: &El) -> bool {
        a.iter().all(|&x| x == 0)
    }
    fn add(&self, a: &El, b: &El) -> El {
        a.iter().zip(b).map(|(x, y)| (x + y) % self.p).collect()
    }
    fn sub(&self, a: &El, b: &El) -> El {
        a.iter()
            .zip(b)
            .map(|(x, y)| (x + self.p - y) % self.p)
            .collect()
    }
    fn neg(&self, a: &El) -> El {
        a.iter().map(|x| (self.p - x) % self.p).collect()
    }
    fn mul(&self, a: &El, b: &El) -> El {
        let p = self.p;
        let d = self.d;
        let mut r = vec![0u64; 2 * d - 1];
        for i in 0..d {
            if a[i] == 0 {
                continue;
            }
            for j in 0..d {
                r[i + j] += a[i] * b[j];
            }
            if i % 8 == 7 {
                for v in r.iter_mut() {
                    *v %= p;
                }
            }
        }
        for v in r.iter_mut() {
            *v %= p;
        }
        for k in (d..2 * d - 1).rev() {
            let c = r[k];
            if c == 0 {
                continue;
            }
            r[k] = 0;
            for i in 0..d {
                r[k - d + i] = (r[k - d + i] + p * p - c * self.m[i]) % p;
            }
        }
        r.truncate(d);
        r
    }
    fn sq(&self, a: &El) -> El {
        self.mul(a, a)
    }
    fn pow_u(&self, a: &El, mut e: u64) -> El {
        let (mut r, mut b) = (self.one(), a.clone());
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.sq(&b);
            e >>= 1;
        }
        r
    }
    fn pow_big(&self, a: &El, e: &Big) -> El {
        let mut r = self.one();
        for i in (0..big_bits(e)).rev() {
            r = self.sq(&r);
            if big_bit(e, i) {
                r = self.mul(&r, a);
            }
        }
        r
    }
    fn inv(&self, a: &El) -> El {
        assert!(!self.is_zero(a));
        let r = self.pow_big(a, &big_sub_small(&self.q_all, 2));
        debug_assert!(self.mul(a, &r) == self.one());
        r
    }
    fn frob(&self, a: &El) -> El {
        self.pow_u(a, self.p)
    }
    /// a^(p^k)
    fn frob_k(&self, a: &El, k: usize) -> El {
        let mut z = a.clone();
        for _ in 0..k {
            z = self.frob(&z);
        }
        z
    }
    fn in_sub(&self, a: &El, k: usize) -> bool {
        self.frob_k(a, k) == *a
    }
    fn rand(&self, rng: &mut Rng) -> El {
        (0..self.d).map(|_| rng.below(self.p)).collect()
    }
    /// uniform element of the subfield F_{p^k} via Tr_{B/F_{p^k}}
    fn rand_sub(&self, k: usize, rng: &mut Rng) -> El {
        let z = self.rand(rng);
        let mut t = self.zero();
        let mut y = z;
        for _ in 0..self.d / k {
            t = self.add(&t, &y);
            y = self.frob_k(&y, k);
        }
        debug_assert!(self.in_sub(&t, k));
        t
    }
    /// quadratic character of the subfield F_{p^k} on a (a in it): 1, -1, 0
    fn chi(&self, a: &El, k: usize) -> i32 {
        if self.is_zero(a) {
            return 0;
        }
        let qk = self.p.pow(k as u32);
        let v = self.pow_u(a, (qk - 1) / 2);
        if v == self.one() {
            1
        } else {
            assert!(v == self.c(self.p - 1), "chi not +-1");
            -1
        }
    }
    /// square root in F_{p^k} (Tonelli-Shanks), a a nonzero square there
    fn sqrt_sub(&self, a: &El, k: usize, nonres: &El) -> El {
        let qk = self.p.pow(k as u32);
        let mut s = 0;
        let mut qo = qk - 1;
        while qo % 2 == 0 {
            qo /= 2;
            s += 1;
        }
        let mut mm = s;
        let mut c = self.pow_u(nonres, qo);
        let mut t = self.pow_u(a, qo);
        let mut r = self.pow_u(a, (qo + 1) / 2);
        let one = self.one();
        while t != one {
            let mut i = 0;
            let mut tt = t.clone();
            while tt != one {
                tt = self.sq(&tt);
                i += 1;
            }
            let mut b = c.clone();
            for _ in 0..(mm - i - 1) {
                b = self.sq(&b);
            }
            mm = i;
            c = self.sq(&b);
            t = self.mul(&t, &c);
            r = self.mul(&r, &b);
        }
        debug_assert!(self.sq(&r) == *a);
        r
    }
}

// ------------------------------------------------ polys over B (cubic) ----

type BP = Vec<El>; // coefficients low to high

fn bp_trim(b: &B, a: &mut BP) {
    while a.last().is_some_and(|c| b.is_zero(c)) {
        a.pop();
    }
}
fn bp_mod(b: &B, a: &BP, m: &BP) -> BP {
    let mut r = a.clone();
    bp_trim(b, &mut r);
    let dm = m.len() - 1;
    let lci = b.inv(m.last().unwrap());
    while r.len() > dm {
        let c = b.mul(r.last().unwrap(), &lci);
        let sh = r.len() - 1 - dm;
        for i in 0..=dm {
            r[sh + i] = b.sub(&r[sh + i], &b.mul(&c, &m[i]));
        }
        bp_trim(b, &mut r);
    }
    r
}
fn bp_mulmod(b: &B, x: &BP, y: &BP, m: &BP) -> BP {
    if x.is_empty() || y.is_empty() {
        return vec![];
    }
    let mut r = vec![b.zero(); x.len() + y.len() - 1];
    for (i, u) in x.iter().enumerate() {
        for (j, v) in y.iter().enumerate() {
            r[i + j] = b.add(&r[i + j], &b.mul(u, v));
        }
    }
    bp_mod(b, &r, m)
}
fn bp_powmod(b: &B, x: &BP, e: &Big, m: &BP) -> BP {
    let mut r: BP = vec![b.one()];
    for i in (0..big_bits(e)).rev() {
        r = bp_mulmod(b, &r, &r, m);
        if big_bit(e, i) {
            r = bp_mulmod(b, &r, x, m);
        }
    }
    r
}
fn bp_gcd(b: &B, x: &BP, y: &BP) -> BP {
    let (mut a, mut c) = (x.clone(), y.clone());
    bp_trim(b, &mut a);
    bp_trim(b, &mut c);
    while !c.is_empty() {
        let r = bp_mod(b, &a, &c);
        a = c;
        c = r;
    }
    let li = b.inv(a.last().unwrap());
    a.iter().map(|v| b.mul(v, &li)).collect()
}
fn bp_divexact(b: &B, a: &BP, m: &BP) -> BP {
    // a / m, m monic
    let mut r = a.clone();
    bp_trim(b, &mut r);
    let dm = m.len() - 1;
    let mut qt = vec![b.zero(); r.len() - dm];
    while r.len() > dm {
        let c = r.last().unwrap().clone();
        let sh = r.len() - 1 - dm;
        qt[sh] = c.clone();
        for i in 0..=dm {
            r[sh + i] = b.sub(&r[sh + i], &b.mul(&c, &m[i]));
        }
        bp_trim(b, &mut r);
    }
    assert!(r.is_empty(), "not exact");
    qt
}

/// All roots in B of a monic squarefree polynomial that splits in B.
fn roots(b: &B, f: &BP, rng: &mut Rng) -> Vec<El> {
    let deg = f.len() - 1;
    if deg == 0 {
        return vec![];
    }
    if deg == 1 {
        return vec![b.neg(&f[0])];
    }
    let e = big_shr1(&big_sub_small(&b.q_all, 1));
    loop {
        let delta = b.rand(rng);
        let x: BP = vec![delta, b.one()];
        let mut h = bp_powmod(b, &x, &e, f);
        if h.is_empty() {
            h.push(b.zero());
        }
        h[0] = b.sub(&h[0], &b.one());
        let g = bp_gcd(b, f, &h);
        let dg = g.len() - 1;
        if dg > 0 && dg < deg {
            let other = bp_divexact(b, f, &g);
            let mut r = roots(b, &g, rng);
            r.extend(roots(b, &other, rng));
            return r;
        }
    }
}

// --------------------------------------------------------------- rng ----

struct Rng(u64);
impl Rng {
    fn new(tag: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for c in tag.bytes() {
            h ^= c as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        Rng(h)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        let zone = u64::MAX - (u64::MAX % m);
        loop {
            let v = self.next();
            if v < zone {
                return v % m;
            }
        }
    }
}

// ------------------------------------------------------------- curve ----

#[derive(Clone)]
struct Curve {
    a2: El,
    a4: El,
    a6: El,
}

type Pt = Option<(El, El)>;

fn fval(b: &B, c: &Curve, x: &El) -> El {
    // ((x + a2) x + a4) x + a6
    let t = b.add(x, &c.a2);
    let t = b.add(&b.mul(&t, x), &c.a4);
    b.add(&b.mul(&t, x), &c.a6)
}

fn padd(b: &B, c: &Curve, p: &Pt, q: &Pt) -> Pt {
    let (x1, y1) = match p {
        None => return q.clone(),
        Some(v) => v,
    };
    let (x2, y2) = match q {
        None => return p.clone(),
        Some(v) => v,
    };
    let lam = if x1 == x2 {
        if b.is_zero(&b.add(y1, y2)) {
            return None;
        }
        // (3x^2 + 2 a2 x + a4) / 2y
        let num = b.add(
            &b.add(
                &b.mul(&b.c(3), &b.sq(x1)),
                &b.mul(&b.mul(&b.c(2), &c.a2), x1),
            ),
            &c.a4,
        );
        b.mul(&num, &b.inv(&b.mul(&b.c(2), y1)))
    } else {
        b.mul(&b.sub(y2, y1), &b.inv(&b.sub(x2, x1)))
    };
    let x3 = b.sub(&b.sub(&b.sub(&b.sq(&lam), &c.a2), x1), x2);
    let y3 = b.sub(&b.mul(&lam, &b.sub(x1, &x3)), y1);
    Some((x3, y3))
}

fn pmul(b: &B, c: &Curve, p: &Pt, mut k: u64) -> Pt {
    let (mut r, mut s) = (None, p.clone());
    while k > 0 {
        if k & 1 == 1 {
            r = padd(b, c, &r, &s);
        }
        s = padd(b, c, &s, &s);
        k >>= 1;
    }
    r
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut v = vec![];
    let mut d = 2;
    while d * d <= n {
        let mut e = 0;
        while n % d == 0 {
            n /= d;
            e += 1;
        }
        if e > 0 {
            v.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        v.push((n, 1));
    }
    v
}

fn j_in_sub(b: &B, c: &Curve, k: usize) -> bool {
    let b2 = b.mul(&b.c(4), &c.a2);
    let b4 = b.mul(&b.c(2), &c.a4);
    let b6 = b.mul(&b.c(4), &c.a6);
    let b8 = b.sub(&b.mul(&b.c(4), &b.mul(&c.a2, &c.a6)), &b.sq(&c.a4));
    let c4 = b.sub(&b.sq(&b2), &b.mul(&b.c(24), &b4));
    let disc = {
        let t1 = b.neg(&b.mul(&b.sq(&b2), &b8));
        let t2 = b.mul(&b.c(8), &b.mul(&b.sq(&b4), &b4));
        let t3 = b.mul(&b.c(27), &b.sq(&b6));
        let t4 = b.mul(&b.c(9), &b.mul(&b2, &b.mul(&b4, &b6)));
        b.add(&b.sub(&b.sub(&t1, &t2), &t3), &t4)
    };
    assert!(!b.is_zero(&disc));
    let j = b.mul(&b.mul(&b.sq(&c4), &c4), &b.inv(&disc));
    b.in_sub(&j, k)
}

// -------------------------------------------------------------- lines ----

struct LineRes {
    crit: bool,
    int_ok: bool,
    orbits: String,
    half_stable: bool,
    sq: u32,
    nsq: u32,
    zero: u32,
    chi_u: i32,
}

fn line_eval(b: &B, n: usize, c: &Curve, es: &[El], x0: &El, u: &El) -> LineRes {
    let ui = b.inv(u);
    let ts: Vec<El> = es.iter().map(|e| b.mul(&b.sub(e, x0), &ui)).collect();
    // orbits under t -> t^p
    let mut seen = vec![false; 3];
    let (mut crit, mut int_ok) = (true, true);
    let mut orbs = vec![];
    let mut total = 0usize;
    for i in 0..3 {
        if seen[i] {
            continue;
        }
        let mut orbit = vec![ts[i].clone()];
        loop {
            let nx = b.frob(orbit.last().unwrap());
            if nx == orbit[0] {
                break;
            }
            orbit.push(nx);
        }
        let s = orbit.len();
        let mut r = 0;
        for j in 0..3 {
            if orbit.contains(&ts[j]) {
                seen[j] = true;
                r += 1;
            }
        }
        if (n * r) % s != 0 {
            int_ok = false;
        }
        if ((n * r) / s) % 2 != 0 || (n * r) % s != 0 {
            crit = false;
        }
        total += (n * r / s.max(1)) * s;
        orbs.push(format!("{s}:{r}"));
    }
    if total != 3 * n {
        int_ok = false;
    }
    let half_stable = n % 2 == 0 && {
        let h: Vec<El> = ts.iter().map(|t| b.frob_k(t, n / 2)).collect();
        h.iter().all(|x| ts.contains(x))
    };
    let (mut sq, mut nsq, mut zero) = (0, 0, 0);
    for t in 0..b.p {
        let x = b.add(x0, &b.mul(u, &b.c(t)));
        match b.chi(&fval(b, c, &x), n) {
            1 => sq += 1,
            -1 => nsq += 1,
            _ => zero += 1,
        }
    }
    LineRes {
        crit,
        int_ok,
        orbits: orbs.join(";"),
        half_stable,
        sq,
        nsq,
        zero,
        chi_u: b.chi(u, n),
    }
}

fn cubic_from_roots(b: &B, es: &[El]) -> Curve {
    let s1 = b.add(&b.add(&es[0], &es[1]), &es[2]);
    let s2 = b.add(
        &b.add(&b.mul(&es[0], &es[1]), &b.mul(&es[0], &es[2])),
        &b.mul(&es[1], &es[2]),
    );
    let s3 = b.mul(&b.mul(&es[0], &es[1]), &es[2]);
    Curve {
        a2: b.neg(&s1),
        a4: s2,
        a6: b.neg(&s3),
    }
}

fn line_row(cell: &str, arm: &str, ci: usize, li: usize, r: &LineRes, extra: &str) -> String {
    format!(
        "{{\"kind\":\"line\",{cell},\"arm\":\"{arm}\",\"curve\":{ci},\"line\":{li},\"crit\":{},\"int_ok\":{},\"orbits\":\"{}\",\"half_stable\":{},\"sq\":{},\"nsq\":{},\"zero\":{},\"constant\":{},\"chi_u\":{}{extra}}}",
        r.crit,
        r.int_ok,
        r.orbits,
        r.half_stable,
        r.sq,
        r.nsq,
        r.zero,
        r.nsq == 0 || r.sq == 0,
        r.chi_u
    )
}

/// All elements of F_{p^k} inside B (basis from traces, then enumerate).
fn enumerate_sub(b: &B, k: usize, rng: &mut Rng) -> Vec<El> {
    let mut basis: Vec<El> = vec![];
    // Gaussian elimination over F_p on coordinate vectors
    let mut ech: Vec<(usize, El)> = vec![];
    while basis.len() < k {
        let v = b.rand_sub(k, rng);
        let mut w = v.clone();
        for (piv, row) in &ech {
            if w[*piv] != 0 {
                let c = w[*piv];
                w = b.sub(&w, &row.iter().map(|x| x * c % b.p).collect::<El>());
            }
        }
        if let Some(piv) = w.iter().position(|&x| x != 0) {
            let inv = inv_p(w[piv], b.p);
            let row: El = w.iter().map(|x| x * inv % b.p).collect();
            ech.push((piv, row));
            basis.push(v);
        }
    }
    let total = b.p.pow(k as u32);
    let mut out = Vec::with_capacity(total as usize);
    for idx in 0..total {
        let mut e = b.zero();
        let mut r = idx;
        for bv in &basis {
            let c = r % b.p;
            r /= b.p;
            e = b.add(&e, &bv.iter().map(|x| x * c % b.p).collect::<El>());
        }
        out.push(e);
    }
    out
}

