//! Orbit-parity criterion and descent case for lines x0 + u F_q on
//! y^2 = f(x) over K = F_{q^n}, q = p odd (PROTOCOL.md, frozen at 89611afe58).
//!
//!   orbit run <out_dir>
//!   orbit summarize <out_dir>
//!   orbit validate <out_dir> <claimed.json>

mod sha;

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

fn run_curve(b: &B, p: u64, n: usize, arm: &str, ci: usize) -> Vec<String> {
    let cell = format!("\"p\":{p},\"n\":{n}");
    let mut rng = Rng::new(&format!("orbit-20261106/{arm}/{p}/{n}/{ci}"));
    let mut out = vec![];
    let nonres_k = loop {
        let z = b.rand_sub(n, &mut rng);
        if b.chi(&z, n) == -1 {
            break z;
        }
    };
    if arm == "generic" {
        // a2, a4, a6 uniform in K, three distinct roots
        let (c, es) = loop {
            let c = Curve {
                a2: b.rand_sub(n, &mut rng),
                a4: b.rand_sub(n, &mut rng),
                a6: b.rand_sub(n, &mut rng),
            };
            let f: BP = vec![c.a6.clone(), c.a4.clone(), c.a2.clone(), b.one()];
            let df: BP = vec![c.a4.clone(), b.mul(&b.c(2), &c.a2), b.c(3)];
            let g = bp_gcd(b, &f, &df);
            if g.len() == 1 {
                let es = roots(b, &f, &mut rng);
                break (c, es);
            }
        };
        assert_eq!(es.len(), 3);
        for e in &es {
            assert!(b.is_zero(&fval(b, &c, e)));
        }
        let jk0 = n % 2 == 0 && j_in_sub(b, &c, n / 2);
        out.push(format!(
            "{{\"kind\":\"curve\",{cell},\"arm\":\"generic\",\"curve\":{ci},\"j_in_K0\":{jk0}}}"
        ));
        for li in 0..200 {
            let x0 = b.rand_sub(n, &mut rng);
            let u = loop {
                let u = b.rand_sub(n, &mut rng);
                if !b.is_zero(&u) {
                    break u;
                }
            };
            let r = line_eval(b, n, &c, &es, &x0, &u);
            out.push(line_row(&cell, "generic-random", ci, li, &r, ""));
        }
        return out;
    }
    // descent curve: f(x) = alpha^3 f0((x - beta)/alpha), f0 over K0
    let h = n / 2;
    let nonres_k0 = loop {
        let z = b.rand_sub(h, &mut rng);
        if b.chi(&z, h) == -1 {
            break z;
        }
    };
    let (f0, e0) = loop {
        let f0: BP = vec![
            b.rand_sub(h, &mut rng),
            b.rand_sub(h, &mut rng),
            b.rand_sub(h, &mut rng),
            b.one(),
        ];
        let df: BP = vec![f0[1].clone(), b.mul(&b.c(2), &f0[2]), b.c(3)];
        if bp_gcd(b, &f0, &df).len() == 1 {
            let r = roots(b, &f0, &mut rng);
            break (f0, r);
        }
    };
    assert_eq!(e0.len(), 3);
    let alpha = loop {
        let a = b.rand_sub(n, &mut rng);
        if !b.is_zero(&a) {
            break a;
        }
    };
    let beta = b.rand_sub(n, &mut rng);
    let es: Vec<El> = e0.iter().map(|e| b.add(&b.mul(&alpha, e), &beta)).collect();
    let c = cubic_from_roots(b, &es);
    for v in [&c.a2, &c.a4, &c.a6] {
        assert!(b.in_sub(v, n), "descent curve not over K");
    }
    let jk0 = j_in_sub(b, &c, h);
    // point counts of C: Y^2 = F(t) over K0 for the built lines. F(t) =
    // f0(gamma + nu t)/nu^3 is K0-isomorphic to f0 (t -> gamma + nu t, Y ->
    // Y nu^{-3/2} up to a K0-twist), so count C0: Y^2 = f0 and its twist,
    // and track each line's twist class by chi_K0(nu).
    let k0_elems = enumerate_sub(b, h, &mut rng);
    let f0v = |x: &El| -> El {
        let t = b.add(x, &f0[2]);
        let t = b.add(&b.mul(&t, x), &f0[1]);
        b.add(&b.mul(&t, x), &f0[0])
    };
    let (mut n_c0, mut n_c0t) = (1u64, 1u64);
    for x in &k0_elems {
        let ch = b.chi(&f0v(x), h);
        n_c0 += (1 + ch) as u64;
        n_c0t += (1 - ch) as u64;
    }
    let order = n_c0 * n_c0t; // #E(K) when E ~ C0 over K
    let fac = factor(order);
    let (ell, v) = *fac.last().unwrap();
    let ellv = ell.pow(v);
    let ell_c0 = n_c0 % ell == 0;
    let ell_c0t = n_c0t % ell == 0;
    // order check on E(K): E ~ C0 over K iff alpha is a square in K; else
    // E is the K-twist and #E = 2(q^n + 1) - #C0(K).
    let qn = p.pow(n as u32);
    let alpha_sq = b.chi(&alpha, n) == 1;
    let order_e = if alpha_sq {
        order
    } else {
        2 * (qn + 1) - order
    };
    let mut order_ok = true;
    for _ in 0..8 {
        let pt = loop {
            let x = b.rand_sub(n, &mut rng);
            let fx = fval(b, &c, &x);
            if b.chi(&fx, n) == 1 {
                break Some((x.clone(), b.sqrt_sub(&fx, n, &nonres_k)));
            }
        };
        if pmul(b, &c, &pt, order_e).is_some() {
            order_ok = false;
        }
    }
    out.push(format!(
        "{{\"kind\":\"curve\",{cell},\"arm\":\"descent\",\"curve\":{ci},\"j_in_K0\":{jk0},\"alpha_square\":{alpha_sq},\"nC0\":{n_c0},\"nC0twist\":{n_c0t},\"orderE\":{order_e},\"order_ok\":{order_ok},\"ell\":{ell},\"ell_in_C0\":{ell_c0},\"ell_in_C0twist\":{ell_c0t}}}"
    ));
    let h_cof = order_e / ellv;
    for li in 0..40 {
        let gamma = b.rand_sub(h, &mut rng);
        let nu = loop {
            let z = b.rand_sub(h, &mut rng);
            if !b.is_zero(&z) {
                break z;
            }
        };
        let x0 = b.add(&beta, &b.mul(&alpha, &gamma));
        let u = b.mul(&alpha, &nu);
        let r = line_eval(b, n, &c, &es, &x0, &u);
        // F(t) = f0(gamma + nu t) / nu^3 in K0[t]
        let ui = b.inv(&u);
        let u3i = b.mul(&b.sq(&ui), &ui);
        let mut extra = String::new();
        let all_lift = r.nsq == 0;
        let pred_all_lift = b.chi(&b.mul(&b.sq(&u), &u), n) == 1;
        let mut coeff_k0 = true;
        // dead points (only on all-lift lines, only if ell on exactly one side)
        let mut lifted = 0u32;
        let (mut dead_obs, mut dead_pred, mut mism) = (0u32, 0u32, 0u32);
        // the F's own twist class relative to f0: F(t) = f0(gamma+nu t)/nu^3,
        // so C_F ~ C0 over K0 iff chi_K0(nu) = 1.
        let nu_sq = b.chi(&nu, h) == 1;
        let (ell_in_cf, ell_in_cft) = if nu_sq {
            (ell_c0, ell_c0t)
        } else {
            (ell_c0t, ell_c0)
        };
        let decidable = all_lift && (ell_in_cf != ell_in_cft) && alpha_sq;
        for t in 0..p {
            let x = b.add(&x0, &b.mul(&u, &b.c(t)));
            let fx = fval(b, &c, &x);
            let ft = b.mul(&fx, &u3i);
            if !b.in_sub(&ft, h) {
                coeff_k0 = false;
            }
            if !decidable {
                continue;
            }
            let chf = b.chi(&ft, h);
            if b.chi(&fx, n) == -1 {
                continue;
            }
            lifted += 1;
            let pt: Pt = if b.is_zero(&fx) {
                Some((x.clone(), b.zero()))
            } else {
                Some((x.clone(), b.sqrt_sub(&fx, n, &nonres_k)))
            };
            let dead = pmul(b, &c, &pt, h_cof).is_none();
            // C_F side iff chi_K0(F(t)) in {1, 0}; twist side iff in {-1, 0}
            let pdead = if ell_in_cf { chf != 1 } else { chf != -1 };
            dead_obs += dead as u32;
            dead_pred += pdead as u32;
            mism += (dead != pdead) as u32;
        }
        let _ = &nonres_k0;
        write!(
            extra,
            ",\"all_lift_pred\":{pred_all_lift},\"coeff_in_K0\":{coeff_k0},\"decidable\":{decidable},\"lifted\":{lifted},\"dead_obs\":{dead_obs},\"dead_pred\":{dead_pred},\"dead_mismatch\":{mism}"
        )
        .unwrap();
        out.push(line_row(&cell, "descent-built", ci, li, &r, &extra));
    }
    for li in 0..40 {
        let x0 = b.rand_sub(n, &mut rng);
        let u = loop {
            let u = b.rand_sub(n, &mut rng);
            if !b.is_zero(&u) {
                break u;
            }
        };
        let r = line_eval(b, n, &c, &es, &x0, &u);
        out.push(line_row(&cell, "descent-random", ci, li, &r, ""));
    }
    out
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

const CELLS: &[(u64, usize)] = &[(13, 2), (13, 3), (13, 4), (101, 2), (101, 3), (101, 4)];

fn run(out_dir: &str) {
    std::fs::create_dir_all(out_dir).unwrap();
    let mut fh = std::fs::File::create(format!("{out_dir}/rows.jsonl")).unwrap();
    for &(p, n) in CELLS {
        let t0 = std::time::Instant::now();
        let mut rng = Rng::new(&format!("orbit-20261106/field/{p}/{n}"));
        let b = B::new(p, 6 * n, &mut rng);
        writeln!(
            fh,
            "{{\"kind\":\"field\",\"p\":{p},\"n\":{n},\"modulus\":{:?}}}",
            b.m
        )
        .unwrap();
        let mut jobs: Vec<(&str, usize)> = (0..8).map(|i| ("generic", i)).collect();
        if n % 2 == 0 {
            jobs.extend((0..8).map(|i| ("descent", i)));
        }
        let mut results: Vec<Vec<String>> = vec![vec![]; jobs.len()];
        std::thread::scope(|s| {
            let mut hs = vec![];
            for (slot, (arm, ci)) in results.iter_mut().zip(jobs.iter()) {
                let b = &b;
                hs.push(s.spawn(move || {
                    *slot = run_curve(b, p, n, arm, *ci);
                }));
            }
            for h in hs {
                h.join().unwrap();
            }
        });
        for r in results.into_iter().flatten() {
            writeln!(fh, "{r}").unwrap();
        }
        fh.flush().unwrap();
        eprintln!("cell ({p},{n}) {:.1} s", t0.elapsed().as_secs_f64());
    }
}

// --------------------------------------------------------- summarize ----

pub fn jget<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    if let Some(s) = rest.strip_prefix('"') {
        return s.find('"').map(|j| &s[..j]);
    }
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim())
}
pub fn jnum(l: &str, k: &str) -> u64 {
    jget(l, k)
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("{k} in {l}"))
}
pub fn jbool(l: &str, k: &str) -> bool {
    jget(l, k) == Some("true")
}

/// Weil: chance constancy excluded iff q - 3n > (3n - 1) sqrt(q).
fn weil_decides(p: u64, n: u64) -> bool {
    (p as f64 - 3.0 * n as f64) > (3.0 * n as f64 - 1.0) * (p as f64).sqrt()
}

pub struct Summary {
    pub vals: Vec<(String, f64)>,
}

pub fn compute(dir: &str) -> (String, String, Summary) {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let rows: Vec<&str> = text.lines().collect();
    let lines: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("line"))
        .collect();
    let curves: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("curve"))
        .collect();
    let mut md = String::new();
    let mut vals: Vec<(String, f64)> = vec![];
    let put = |k: &str, v: f64, vals: &mut Vec<(String, f64)>| vals.push((k.to_string(), v));

    // O1
    let crit: Vec<&&str> = lines.iter().filter(|l| jbool(l, "crit")).collect();
    let o1_bad = crit.iter().filter(|l| !jbool(l, "constant")).count();
    let o1 = o1_bad == 0 && !crit.is_empty();
    put("crit_lines", crit.len() as f64, &mut vals);
    put("crit_nonconstant", o1_bad as f64, &mut vals);
    // O2 / O2'
    let mut o2_bad = 0;
    let mut o2_lines = 0;
    let mut tab = String::new();
    writeln!(tab, "| p | n | Weil decides | lines | criterion true | constant | criterion false but constant | criterion true, not constant |\n|--:|--:|:--|--:|--:|--:|--:|--:|").unwrap();
    for &(p, n) in CELLS {
        let g: Vec<&&str> = lines
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
            .collect();
        let ct = g.iter().filter(|l| jbool(l, "crit")).count();
        let cons = g.iter().filter(|l| jbool(l, "constant")).count();
        let fc = g
            .iter()
            .filter(|l| !jbool(l, "crit") && jbool(l, "constant"))
            .count();
        let tn = g
            .iter()
            .filter(|l| jbool(l, "crit") && !jbool(l, "constant"))
            .count();
        let wd = weil_decides(p, n as u64);
        if wd {
            o2_bad += fc;
            o2_lines += g.iter().filter(|l| !jbool(l, "crit")).count();
        }
        put(&format!("false_constant_{p}_{n}"), fc as f64, &mut vals);
        writeln!(
            tab,
            "| {p} | {n} | {wd} | {} | {ct} | {cons} | {fc} | {tn} |",
            g.len()
        )
        .unwrap();
    }
    let o2 = o2_bad == 0 && o2_lines > 0;
    // O3
    let built: Vec<&&str> = lines
        .iter()
        .filter(|l| jget(l, "arm") == Some("descent-built"))
        .collect();
    let o3_bad = built
        .iter()
        .filter(|l| {
            !(jbool(l, "crit")
                && jbool(l, "half_stable")
                && jbool(l, "coeff_in_K0")
                && jbool(l, "constant")
                && (jbool(l, "all_lift_pred") == (jnum(l, "nsq") == 0)))
        })
        .count();
    let o3 = o3_bad == 0 && !built.is_empty();
    put("built_lines", built.len() as f64, &mut vals);
    put("built_bad", o3_bad as f64, &mut vals);
    // O4
    let dec: Vec<&&str> = built
        .iter()
        .copied()
        .filter(|l| jbool(l, "decidable"))
        .collect();
    let mism: u64 = dec.iter().map(|l| jnum(l, "dead_mismatch")).sum();
    let mut o4_frac_ok = true;
    let mut dtab = String::new();
    writeln!(dtab, "| p | n | decidable lines | lifted x | dead observed | dead predicted | mismatches | dead fraction |\n|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for &(p, n) in CELLS {
        if n % 2 == 1 {
            continue;
        }
        let g: Vec<&&&str> = dec
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
            .collect();
        let lifted: u64 = g.iter().map(|l| jnum(l, "lifted")).sum();
        let dobs: u64 = g.iter().map(|l| jnum(l, "dead_obs")).sum();
        let dpred: u64 = g.iter().map(|l| jnum(l, "dead_pred")).sum();
        let mm: u64 = g.iter().map(|l| jnum(l, "dead_mismatch")).sum();
        let frac = if lifted > 0 {
            dobs as f64 / lifted as f64
        } else {
            f64::NAN
        };
        if lifted > 0 && !(0.35..=0.65).contains(&frac) {
            o4_frac_ok = false;
        }
        put(&format!("dead_mismatch_{p}_{n}"), mm as f64, &mut vals);
        put(&format!("lifted_{p}_{n}"), lifted as f64, &mut vals);
        put(&format!("dead_obs_{p}_{n}"), dobs as f64, &mut vals);
        writeln!(
            dtab,
            "| {p} | {n} | {} | {lifted} | {dobs} | {dpred} | {mm} | {frac:.3} |",
            g.len()
        )
        .unwrap();
    }
    let o4 = mism == 0 && !dec.is_empty() && o4_frac_ok;
    // post hoc (not a decision rule): are the mismatches the l-side points
    // whose order is prime to l? Expected count = sum over lines of
    // (l-side lifted x) / l^v, l^v the l-part of #E(K).
    let mut ph = String::new();
    writeln!(ph, "| p | n | mismatches with predicted dead, observed alive | observed dead, predicted alive | expected (l-side x)/l^v |\n|--:|--:|--:|--:|--:|").unwrap();
    for &(p, n) in CELLS {
        if n % 2 == 1 {
            continue;
        }
        let (mut fa, mut fd, mut exp) = (0u64, 0u64, 0f64);
        for l in dec
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
        {
            let cv = curves
                .iter()
                .find(|c| {
                    jget(c, "arm") == Some("descent")
                        && jnum(c, "p") == p
                        && jnum(c, "n") == n as u64
                        && jnum(c, "curve") == jnum(l, "curve")
                })
                .unwrap();
            let (ell, ord) = (jnum(cv, "ell"), jnum(cv, "orderE"));
            let mut lv = 1u64;
            while ord % (lv * ell) == 0 {
                lv *= ell;
            }
            let (lifted, dp, dobs, mm) = (
                jnum(l, "lifted"),
                jnum(l, "dead_pred"),
                jnum(l, "dead_obs"),
                jnum(l, "dead_mismatch"),
            );
            // mm = fd + fa, dobs - dp = fd - fa
            let fd_l = (mm + dobs - dp) / 2;
            fd += fd_l;
            fa += mm - fd_l;
            exp += (lifted - dp) as f64 / lv as f64;
        }
        put(
            &format!("posthoc_pred_dead_obs_alive_{p}_{n}"),
            fa as f64,
            &mut vals,
        );
        writeln!(ph, "| {p} | {n} | {fa} | {fd} | {exp:.1} |").unwrap();
    }
    // O5
    let o5_bad = lines.iter().filter(|l| !jbool(l, "int_ok")).count();
    let o5 = o5_bad == 0;
    put("int_bad", o5_bad as f64, &mut vals);
    // curves
    let dcur: Vec<&&str> = curves
        .iter()
        .filter(|l| jget(l, "arm") == Some("descent"))
        .collect();
    let ord_bad = dcur.iter().filter(|l| !jbool(l, "order_ok")).count();
    let jk0_desc = dcur.iter().filter(|l| jbool(l, "j_in_K0")).count();
    let gcur: Vec<&&str> = curves
        .iter()
        .filter(|l| jget(l, "arm") == Some("generic"))
        .collect();
    let jk0_gen = gcur.iter().filter(|l| jbool(l, "j_in_K0")).count();
    let both = dcur
        .iter()
        .filter(|l| jbool(l, "ell_in_C0") && jbool(l, "ell_in_C0twist"))
        .count();
    put("descent_curves", dcur.len() as f64, &mut vals);
    put("descent_order_bad", ord_bad as f64, &mut vals);
    put("descent_j_in_K0", jk0_desc as f64, &mut vals);
    // stable_half among random lines
    let rnd_half = lines
        .iter()
        .filter(|l| jget(l, "arm") != Some("descent-built") && jbool(l, "half_stable"))
        .count();
    put("random_half_stable", rnd_half as f64, &mut vals);

    let v = |b: bool| if b { "PASS" } else { "FAIL" };
    writeln!(md, "# Summary (coded rules, PROTOCOL.md)\n").unwrap();
    writeln!(
        md,
        "- **O1** (criterion ⇒ constant): {} criterion lines, {} not constant: **{}**",
        crit.len(),
        o1_bad,
        v(o1)
    )
    .unwrap();
    writeln!(md, "- **O2** (⇐ where Weil decides: (101,2), (101,3)): {} lines failing the criterion, {} constant: **{}**", o2_lines, o2_bad, v(o2)).unwrap();
    writeln!(md, "- **O3** (descent lines: criterion, σ^(n/2)-stable, F ∈ K₀[t], all-lift iff χ_K(u³) = 1): {} lines, {} exceptions: **{}**", built.len(), o3_bad, v(o3)).unwrap();
    writeln!(md, "- **O4** (dead set predicted exactly; pooled fraction in [0.35, 0.65]): {} decidable lines, {} mismatches, fraction rule ok = {}: **{}**", dec.len(), mism, o4_frac_ok, v(o4)).unwrap();
    writeln!(
        md,
        "- **O5** (n·r/s integer, Σ = 3n): {} exceptions: **{}**",
        o5_bad,
        v(o5)
    )
    .unwrap();
    writeln!(md, "- Descent curves: {} (j ∈ K₀ on {}; order check failed on {}; ℓ on both sides on {}). Generic curves with j ∈ K₀ (n even): {}. σ^(n/2)-stable root sets among random lines: {}.\n", dcur.len(), jk0_desc, ord_bad, both, jk0_gen, rnd_half).unwrap();
    writeln!(md, "## Post hoc: direction of the O4 mismatches\n\n{ph}").unwrap();
    writeln!(md, "## Criterion vs constancy per cell\n\n{tab}\n## Dead points on all-lift descent lines\n\n{dtab}").unwrap();
    let js = format!(
        "{{\"O1\":{o1},\"O2\":{o2},\"O3\":{o3},\"O4\":{o4},\"O5\":{o5},\"values\":{{{}}}}}\n",
        vals.iter()
            .map(|(k, v)| format!("\"{k}\":{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    (md, js, Summary { vals })
}

fn summarize(dir: &str) {
    let (md, js, s) = compute(dir);
    std::fs::write(format!("{dir}/SUMMARY.md"), &md).unwrap();
    std::fs::write(format!("{dir}/summary.json"), js).unwrap();
    std::fs::write(format!("{dir}/fig1_orbit.svg"), svg(dir, &s)).unwrap();
    print!("{md}");
}

fn svg(dir: &str, _s: &Summary) -> String {
    // panel per cell: stacked bars of line outcomes (criterion x constancy)
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let lines: Vec<&str> = text
        .lines()
        .filter(|l| jget(l, "kind") == Some("line"))
        .collect();
    let (w, h) = (820.0, 330.0);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"20\" y=\"24\" font-size=\"13\" font-weight=\"bold\">Lines by criterion and observed constancy (log scale count)</text>\n");
    let cats: [(&str, &str, Box<dyn Fn(&str) -> bool>); 4] = [
        (
            "criterion ∧ constant",
            "#2e86c1",
            Box::new(|l: &str| jbool(l, "crit") && jbool(l, "constant")),
        ),
        (
            "criterion ∧ not constant",
            "#c0392b",
            Box::new(|l: &str| jbool(l, "crit") && !jbool(l, "constant")),
        ),
        (
            "¬criterion ∧ constant",
            "#e67e22",
            Box::new(|l: &str| !jbool(l, "crit") && jbool(l, "constant")),
        ),
        (
            "¬criterion ∧ not constant",
            "#95a5a6",
            Box::new(|l: &str| !jbool(l, "crit") && !jbool(l, "constant")),
        ),
    ];
    let (x0, top, ph) = (60.0, 50.0, 220.0);
    let ymax = 4.0f64; // log10
    let y = |c: f64| {
        top + ph
            - if c > 0.0 {
                ((c.log10() + 0.3) / (ymax + 0.3)).min(1.0) * ph
            } else {
                0.0
            }
    };
    for t in 0..=4 {
        let v = 10f64.powi(t);
        writeln!(s, "<line x1=\"{x0}\" y1=\"{}\" x2=\"800\" y2=\"{}\" stroke=\"#ddd\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\">{v}</text>", y(v), y(v), x0 - 4.0, y(v) + 4.0).unwrap();
    }
    let bw = 740.0 / CELLS.len() as f64;
    for (i, &(p, n)) in CELLS.iter().enumerate() {
        let g: Vec<&&str> = lines
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
            .collect();
        let cx = x0 + bw * i as f64;
        for (j, (_, col, f)) in cats.iter().enumerate() {
            let c = g.iter().filter(|l| f(l)).count() as f64;
            let sub = (bw - 12.0) / 4.0;
            let bx = cx + 6.0 + sub * j as f64;
            writeln!(s, "<rect x=\"{bx}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{col}\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"9\">{c}</text>", y(c), sub - 2.0, top + ph - y(c), bx + sub / 2.0, y(c) - 3.0).unwrap();
        }
        writeln!(
            s,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">p={p}, n={n}{}</text>",
            cx + bw / 2.0,
            top + ph + 16.0,
            if weil_decides(p, n as u64) {
                " (Weil)"
            } else {
                ""
            }
        )
        .unwrap();
    }
    for (j, (lab, col, _)) in cats.iter().enumerate() {
        let lx = 60.0 + 190.0 * j as f64;
        writeln!(s, "<rect x=\"{lx}\" y=\"305\" width=\"10\" height=\"10\" fill=\"{col}\"/><text x=\"{}\" y=\"314\">{lab}</text>", lx + 14.0).unwrap();
    }
    s.push_str("</svg>\n");
    s
}

fn validate(dir: &str, claimed: &str) {
    let (_, _, s) = compute(dir);
    let text = std::fs::read_to_string(claimed).unwrap();
    let mut heads = vec![];
    let mut ok = true;
    for line in text.lines() {
        let t = line.trim().trim_end_matches(',');
        let Some((k, v)) = t.split_once(':') else {
            continue;
        };
        let k = k.trim().trim_matches('"');
        let Ok(v) = v.trim().parse::<f64>() else {
            continue;
        };
        let got = s
            .vals
            .iter()
            .find(|x| x.0 == k)
            .unwrap_or_else(|| panic!("no value {k}"))
            .1;
        if got != v {
            ok = false;
        }
        heads.push(format!(
            "    {{\"name\": \"{k}\", \"claimed\": {v}, \"recomputed\": {got}, \"rel_tol\": 0}}"
        ));
    }
    // rows.jsonl lives in the artifact store (AGENTS.md section 13); the
    // committed manifest pins it. Check the local copy against that pin and
    // pin the manifest itself as the raw data.
    let data = std::fs::read(format!("{dir}/rows.jsonl")).unwrap();
    let rows_sha = sha::sha256_hex(&data);
    let manifest = std::fs::read(format!("{dir}/.artifacts.json")).expect(".artifacts.json");
    let mtext = String::from_utf8_lossy(&manifest).to_string();
    if !mtext.contains(&rows_sha) {
        eprintln!("rows.jsonl sha256 {rows_sha} is not the manifest's pin");
        ok = false;
    }
    let rep = format!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/lift_dense_orbit_parity_20261106\",\n  \"recomputed_by\": {{\"tool\": \"research/lift_dense_orbit_parity_20261106/orbit (orbit validate)\", \"command\": \"orbit validate <run> claimed.json\"}},\n  \"note\": \"Producer-run recompute of README headline numbers from the pinned raw rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n    {{\"path\": \".artifacts.json\", \"sha256\": \"{}\", \"pins\": {{\"rows.jsonl\": \"{}\"}}}}\n  ],\n  \"headline\": [\n{}\n  ]\n}}\n",
        if ok { "pass" } else { "fail" },
        sha::sha256_hex(&manifest),
        rows_sha,
        heads.join(",\n")
    );
    std::fs::write(format!("{dir}/validator-report.json"), rep).unwrap();
    eprintln!(
        "validator-report.json: {}",
        if ok { "pass" } else { "FAIL" }
    );
    if !ok {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => run(&args[2]),
        Some("summarize") => summarize(&args[2]),
        Some("validate") => validate(&args[2], &args[3]),
        _ => eprintln!("usage: orbit run|summarize|validate <dir> [claimed.json]"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_and_roots() {
        let mut rng = Rng::new("t");
        let b = B::new(13, 12, &mut rng);
        // x^p^d = x for a random element
        let a = b.rand(&mut rng);
        assert_eq!(b.frob_k(&a, 12), a);
        // inverse
        assert_eq!(b.mul(&a, &b.inv(&a)), b.one());
        // roots of a product of three known linear factors
        let es: Vec<El> = (0..3).map(|_| b.rand(&mut rng)).collect();
        let c = cubic_from_roots(&b, &es);
        let f: BP = vec![c.a6.clone(), c.a4.clone(), c.a2.clone(), b.one()];
        let mut r = roots(&b, &f, &mut rng);
        let mut e2 = es.clone();
        r.sort();
        e2.sort();
        assert_eq!(r, e2);
    }
    #[test]
    fn sha() {
        assert_eq!(
            sha::sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
