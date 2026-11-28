use crate::fp::Fp;

pub const MAXN: usize = 24;

#[derive(Clone, Copy, Debug)]
pub struct Poly {
    pub c: [u32; MAXN],
    pub n: usize,
}
impl PartialEq for Poly {
    fn eq(&self, o: &Self) -> bool {
        self.n == o.n && self.c[..self.n] == o.c[..o.n]
    }
}
impl Eq for Poly {}

impl Poly {
    pub fn zero() -> Poly {
        Poly { c: [0; MAXN], n: 0 }
    }
    pub fn one() -> Poly {
        let mut p = Poly::zero();
        p.c[0] = 1;
        p.n = 1;
        p
    }
    pub fn x() -> Poly {
        let mut p = Poly::zero();
        p.c[1] = 1;
        p.n = 2;
        p
    }
    pub fn from(v: &[u64], m: u64) -> Poly {
        assert!(v.len() <= MAXN);
        let mut p = Poly::zero();
        for (i, &a) in v.iter().enumerate() {
            p.c[i] = (a % m) as u32;
        }
        p.n = v.len();
        p.trim();
        p
    }
    pub fn trim(&mut self) {
        while self.n > 0 && self.c[self.n - 1] == 0 {
            self.n -= 1;
        }
    }
    pub fn deg(&self) -> isize {
        self.n as isize - 1
    }
    pub fn is_zero(&self) -> bool {
        self.n == 0
    }
    pub fn lc(&self) -> u64 {
        self.c[self.n - 1] as u64
    }
    pub fn get(&self, i: usize) -> u64 {
        if i < self.n {
            self.c[i] as u64
        } else {
            0
        }
    }
    pub fn to_vec(&self) -> Vec<u64> {
        self.c[..self.n].iter().map(|&x| x as u64).collect()
    }
}

impl Fp {
    pub fn padd(&self, a: &Poly, b: &Poly) -> Poly {
        let mut r = Poly::zero();
        let n = a.n.max(b.n);
        for i in 0..n {
            r.c[i] = ((a.c[i] as u64 + b.c[i] as u64) % self.p) as u32;
        }
        r.n = n;
        r.trim();
        r
    }
    pub fn psub(&self, a: &Poly, b: &Poly) -> Poly {
        let mut r = Poly::zero();
        let n = a.n.max(b.n);
        for i in 0..n {
            r.c[i] = ((a.c[i] as u64 + self.p - b.c[i] as u64) % self.p) as u32;
        }
        r.n = n;
        r.trim();
        r
    }
    pub fn pneg(&self, a: &Poly) -> Poly {
        let mut r = *a;
        for i in 0..a.n {
            r.c[i] = ((self.p - a.c[i] as u64) % self.p) as u32;
        }
        r.trim();
        r
    }
    pub fn pscale(&self, a: &Poly, s: u64) -> Poly {
        let mut r = *a;
        for i in 0..a.n {
            r.c[i] = (a.c[i] as u64 * s % self.p) as u32;
        }
        r.trim();
        r
    }
    pub fn pmul(&self, a: &Poly, b: &Poly) -> Poly {
        if a.n == 0 || b.n == 0 {
            return Poly::zero();
        }
        let nn = a.n + b.n - 1;
        assert!(nn <= MAXN, "poly overflow");
        let mut acc = [0u64; MAXN];
        for i in 0..a.n {
            let ai = a.c[i] as u64;
            if ai == 0 {
                continue;
            }
            for j in 0..b.n {
                acc[i + j] += ai * b.c[j] as u64;
            }
        }
        let mut r = Poly::zero();
        for k in 0..nn {
            r.c[k] = (acc[k] % self.p) as u32;
        }
        r.n = nn;
        r.trim();
        r
    }
    pub fn pdivrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
        assert!(b.n > 0, "division by zero poly");
        let mut q = Poly::zero();
        let mut r = *a;
        if a.n < b.n {
            return (q, r);
        }
        let p = self.p;
        let il = self.inv(b.lc());
        let bn = b.n;
        for i in (0..=(a.n - bn)).rev() {
            let top = r.c[i + bn - 1] as u64;
            if top == 0 {
                continue;
            }
            let coef = top * il % p;
            q.c[i] = coef as u32;
            for j in 0..bn {
                let sub = coef * b.c[j] as u64 % p;
                r.c[i + j] = ((r.c[i + j] as u64 + p - sub) % p) as u32;
            }
        }
        q.n = a.n - bn + 1;
        q.trim();
        r.n = r.n.min(bn - 1);
        r.trim();
        (q, r)
    }
    pub fn pmod(&self, a: &Poly, b: &Poly) -> Poly {
        self.pdivrem(a, b).1
    }
    pub fn pmonic(&self, a: &Poly) -> Poly {
        if a.n == 0 {
            return *a;
        }
        self.pscale(a, self.inv(a.lc()))
    }
    pub fn pgcd(&self, a: &Poly, b: &Poly) -> Poly {
        let (mut r0, mut r1) = (*a, *b);
        while !r1.is_zero() {
            let r = self.pmod(&r0, &r1);
            r0 = r1;
            r1 = r;
        }
        self.pmonic(&r0)
    }
    /// d = s a + t b, d monic (zero if both zero).
    pub fn pxgcd(&self, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (*a, *b);
        let (mut s0, mut s1) = (Poly::one(), Poly::zero());
        let (mut t0, mut t1) = (Poly::zero(), Poly::one());
        while !r1.is_zero() {
            let (q, r) = self.pdivrem(&r0, &r1);
            let s2 = self.psub(&s0, &self.pmul(&q, &s1));
            let t2 = self.psub(&t0, &self.pmul(&q, &t1));
            r0 = r1;
            r1 = r;
            s0 = s1;
            s1 = s2;
            t0 = t1;
            t1 = t2;
        }
        if r0.is_zero() {
            return (Poly::zero(), Poly::zero(), Poly::zero());
        }
        let il = self.inv(r0.lc());
        (self.pscale(&r0, il), self.pscale(&s0, il), self.pscale(&t0, il))
    }
    pub fn pderiv(&self, a: &Poly) -> Poly {
        let mut r = Poly::zero();
        if a.n <= 1 {
            return r;
        }
        for i in 1..a.n {
            r.c[i - 1] = (a.c[i] as u64 * (i as u64 % self.p) % self.p) as u32;
        }
        r.n = a.n - 1;
        r.trim();
        r
    }
    pub fn ppowmod(&self, base: &Poly, mut e: u64, m: &Poly) -> Poly {
        let mut r = self.pmod(&Poly::one(), m);
        let mut b = self.pmod(base, m);
        while e > 0 {
            if e & 1 == 1 {
                r = self.pmod(&self.pmul(&r, &b), m);
            }
            b = self.pmod(&self.pmul(&b, &b), m);
            e >>= 1;
        }
        r
    }
    pub fn peval(&self, a: &Poly, x: u64) -> u64 {
        let mut acc = 0u64;
        for i in (0..a.n).rev() {
            acc = (acc * x + a.c[i] as u64) % self.p;
        }
        acc
    }
    pub fn is_squarefree(&self, f: &Poly) -> bool {
        self.pgcd(f, &self.pderiv(f)).n == 1
    }
    /// Distinct-degree factorisation degrees of a squarefree degree-7 poly (ascending by d).
    pub fn ddf_degrees(&self, f: &Poly) -> Vec<usize> {
        let mut rem = *f;
        let mut h = Poly::x();
        let mut out = vec![];
        for d in 1..=7usize {
            h = self.ppowmod(&h, self.p, f);
            let g = self.pgcd(&rem, &self.psub(&h, &Poly::x()));
            if g.n > 1 {
                let dg = g.n - 1;
                assert!(dg % d == 0);
                for _ in 0..(dg / d) {
                    out.push(d);
                }
                rem = self.pdivrem(&rem, &g).0;
            }
            if rem.n <= 1 {
                break;
            }
        }
        out
    }
}
