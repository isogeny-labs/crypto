//! rho-fold: which Pollard-rho folds and per-step savings legitimately apply
//! to the graph-IC lane's public targets, E: y^2 = x(x - a)(x - sigma a)
//! over F_{p^6} = F_{q^3}, q = p^2 (jv_cover tower, same w and s choice).
//!
//! Standalone (rustc only, no crates).  Reads one target per stdin line:
//!   label p l alpha[6] gx[6] gy[6] qx[6] qy[6]
//! (coefficient order x6 = [a0.re, a0.im, a1.re, a1.im, a2.re, a2.im] for
//! a0 + a1*theta + a2*theta^2, theta^3 = s, and Fq = Fp[t]/(t^2 - w)).
//! Optional "RUN label steps bitops" lines carry recorded rho arms for the
//! per-step comparison.  Every result is [pcc:unverified].

use std::io::Read;

type E2 = [u64; 2];
type E6 = [E2; 3];

fn pw(mut b: u64, mut e: u128, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * b as u128) % p as u128) as u64;
        }
        b = ((b as u128 * b as u128) % p as u128) as u64;
        e >>= 1;
    }
    r
}

#[derive(Clone, Copy, Default, Debug)]
struct Cnt {
    m: u64,
    a: u64,
    i: u64,
}

struct F {
    p: u64,
    w: u64,
    s: E2,
    zeta: E2,
    /// counters for the counted (Karatsuba) arithmetic only
    c: std::cell::Cell<Cnt>,
}

impl F {
    fn new(p: u64) -> F {
        assert!(p % 2 == 1 && p > 3 && p % 3 != 0);
        let w = (2..p).find(|&w| pw(w, ((p - 1) / 2) as u128, p) != 1).unwrap();
        let mut f = F {
            p,
            w,
            s: [0, 0],
            zeta: [1, 0],
            c: Default::default(),
        };
        let q1 = (p as u128) * (p as u128) - 1;
        // jv_cover::Fq3::new candidate order
        let mut cands: Vec<E2> = Vec::new();
        if p % 3 == 1 {
            for x in 1..p {
                cands.push([x, 0]);
            }
        }
        'o: for y in 1..p {
            for x in 0..p {
                cands.push([x, y]);
                if cands.len() > 4 * p as usize {
                    break 'o;
                }
            }
        }
        let s = *cands
            .iter()
            .find(|s| f.e2pow(s, q1 / 3) != [1, 0])
            .unwrap();
        f.s = s;
        f.zeta = f.e2pow(&s, q1 / 3);
        f
    }
    // ---------- uncounted reference arithmetic ----------
    fn ad(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }
    fn sb(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    fn ml(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn e2mul(&self, a: &E2, b: &E2) -> E2 {
        [
            self.ad(self.ml(a[0], b[0]), self.ml(self.w, self.ml(a[1], b[1]))),
            self.ad(self.ml(a[0], b[1]), self.ml(a[1], b[0])),
        ]
    }
    fn e2add(&self, a: &E2, b: &E2) -> E2 {
        [self.ad(a[0], b[0]), self.ad(a[1], b[1])]
    }
    fn e2sub(&self, a: &E2, b: &E2) -> E2 {
        [self.sb(a[0], b[0]), self.sb(a[1], b[1])]
    }
    fn e2pow(&self, a: &E2, mut e: u128) -> E2 {
        let mut r = [1, 0];
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.e2mul(&r, &b);
            }
            b = self.e2mul(&b, &b);
            e >>= 1;
        }
        r
    }
    fn mul(&self, a: &E6, b: &E6) -> E6 {
        // schoolbook, theta^3 = s
        let m = |i: usize, j: usize| self.e2mul(&a[i], &b[j]);
        let c0 = m(0, 0);
        let c1 = self.e2add(&m(0, 1), &m(1, 0));
        let c2 = self.e2add(&self.e2add(&m(0, 2), &m(1, 1)), &m(2, 0));
        let c3 = self.e2add(&m(1, 2), &m(2, 1));
        let c4 = m(2, 2);
        [
            self.e2add(&c0, &self.e2mul(&self.s, &c3)),
            self.e2add(&c1, &self.e2mul(&self.s, &c4)),
            c2,
        ]
    }
    fn add(&self, a: &E6, b: &E6) -> E6 {
        [self.e2add(&a[0], &b[0]), self.e2add(&a[1], &b[1]), self.e2add(&a[2], &b[2])]
    }
    fn sub(&self, a: &E6, b: &E6) -> E6 {
        [self.e2sub(&a[0], &b[0]), self.e2sub(&a[1], &b[1]), self.e2sub(&a[2], &b[2])]
    }
    fn one(&self) -> E6 {
        [[1, 0], [0, 0], [0, 0]]
    }
    fn cst(&self, k: u64) -> E6 {
        [[k % self.p, 0], [0, 0], [0, 0]]
    }
    fn pow(&self, a: &E6, mut e: u128) -> E6 {
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
    fn pk(&self, k: u32) -> u128 {
        (self.p as u128).pow(k)
    }
    fn inv(&self, a: &E6) -> E6 {
        self.pow(a, self.pk(6) - 2)
    }
    fn frob(&self, a: &E6, k: u32) -> E6 {
        self.pow(a, self.pk(k))
    }
    fn is_zero(a: &E6) -> bool {
        *a == [[0, 0]; 3]
    }
    fn is_square(&self, a: &E6) -> bool {
        self.pow(a, (self.pk(6) - 1) / 2) == self.one()
    }
    /// smallest k | 6 with a in F_{p^k}
    fn deg(&self, a: &E6) -> u32 {
        for k in [1, 2, 3, 6] {
            if self.frob(a, k) == *a {
                return k;
            }
        }
        unreachable!()
    }
    fn sigma_repo(&self, a: &E6) -> E6 {
        let z2 = self.e2mul(&self.zeta, &self.zeta);
        [a[0], self.e2mul(&a[1], &self.zeta), self.e2mul(&a[2], &z2)]
    }

    // ---------- counted Karatsuba arithmetic (bitop-style counts) ----------
    fn bump(&self, m: u64, a: u64, i: u64) {
        let mut c = self.c.get();
        c.m += m;
        c.a += a;
        c.i += i;
        self.c.set(c);
    }
    fn reset(&self) -> Cnt {
        let c = self.c.get();
        self.c.set(Cnt::default());
        c
    }
    fn k2add(&self, a: &E2, b: &E2) -> E2 {
        self.bump(0, 2, 0);
        self.e2add(a, b)
    }
    fn k2sub(&self, a: &E2, b: &E2) -> E2 {
        self.bump(0, 2, 0);
        self.e2sub(a, b)
    }
    /// Karatsuba over Fq: 3 products, 1 product by w (charged as a product)
    fn k2mul(&self, a: &E2, b: &E2) -> E2 {
        let v0 = self.ml(a[0], b[0]);
        let v1 = self.ml(a[1], b[1]);
        let m = self.ml(self.ad(a[0], a[1]), self.ad(b[0], b[1]));
        let c1 = self.sb(self.sb(m, v0), v1);
        let c0 = self.ad(v0, self.ml(self.w, v1));
        self.bump(4, 5, 0);
        [c0, c1]
    }
    /// (a0^2 + w a1^2, 2 a0 a1)
    fn k2sq(&self, a: &E2) -> E2 {
        let s0 = self.ml(a[0], a[0]);
        let s1 = self.ml(a[1], a[1]);
        let c0 = self.ad(s0, self.ml(self.w, s1));
        let v = self.ml(a[0], a[1]);
        self.bump(4, 2, 0);
        [c0, self.ad(v, v)]
    }
    /// product by s (s in Fp when p = 1 mod 3: two products; else a k2mul)
    fn k2s(&self, a: &E2) -> E2 {
        if self.s[1] == 0 {
            self.bump(2, 0, 0);
            [self.ml(a[0], self.s[0]), self.ml(a[1], self.s[0])]
        } else {
            self.k2mul(a, &self.s)
        }
    }
    fn k2inv(&self, a: &E2) -> E2 {
        let n = self.sb(self.ml(a[0], a[0]), self.ml(self.w, self.ml(a[1], a[1])));
        let ni = pw(n, (self.p - 2) as u128, self.p);
        self.bump(5, 2, 1);
        [self.ml(a[0], ni), self.sb(0, self.ml(a[1], ni))]
    }
    /// Karatsuba cubic over Fq (6 Fq products, 2 by s)
    fn kmul(&self, a: &E6, b: &E6) -> E6 {
        let v0 = self.k2mul(&a[0], &b[0]);
        let v1 = self.k2mul(&a[1], &b[1]);
        let v2 = self.k2mul(&a[2], &b[2]);
        let m12 = self.k2mul(&self.k2add(&a[1], &a[2]), &self.k2add(&b[1], &b[2]));
        let m01 = self.k2mul(&self.k2add(&a[0], &a[1]), &self.k2add(&b[0], &b[1]));
        let m02 = self.k2mul(&self.k2add(&a[0], &a[2]), &self.k2add(&b[0], &b[2]));
        let t12 = self.k2sub(&self.k2sub(&m12, &v1), &v2);
        let c0 = self.k2add(&v0, &self.k2s(&t12));
        let c1 = self.k2add(&self.k2sub(&self.k2sub(&m01, &v0), &v1), &self.k2s(&v2));
        let c2 = self.k2add(&self.k2sub(&self.k2sub(&m02, &v0), &v2), &v1);
        [c0, c1, c2]
    }
    /// Chung-Hasan SQR2
    fn ksq(&self, a: &E6) -> E6 {
        let s0 = self.k2sq(&a[0]);
        let p01 = self.k2mul(&a[0], &a[1]);
        let s1 = self.k2add(&p01, &p01);
        let s2 = self.k2sq(&self.k2add(&self.k2sub(&a[0], &a[1]), &a[2]));
        let p12 = self.k2mul(&a[1], &a[2]);
        let s3 = self.k2add(&p12, &p12);
        let s4 = self.k2sq(&a[2]);
        let c0 = self.k2add(&s0, &self.k2s(&s3));
        let c1 = self.k2add(&s1, &self.k2s(&s4));
        let c2 = self.k2sub(
            &self.k2sub(&self.k2add(&self.k2add(&s1, &s2), &s3), &s0),
            &s4,
        );
        [c0, c1, c2]
    }
    /// adjugate inversion through the Fq norm
    fn kinv(&self, a: &E6) -> E6 {
        let a00 = self.k2sq(&a[0]);
        let a11 = self.k2sq(&a[1]);
        let a22 = self.k2sq(&a[2]);
        let a12 = self.k2mul(&a[1], &a[2]);
        let a01 = self.k2mul(&a[0], &a[1]);
        let a02 = self.k2mul(&a[0], &a[2]);
        let c0 = self.k2sub(&a00, &self.k2s(&a12));
        let c1 = self.k2sub(&self.k2s(&a22), &a01);
        let c2 = self.k2sub(&a11, &a02);
        let t = self.k2add(&self.k2mul(&a[2], &c1), &self.k2mul(&a[1], &c2));
        let n = self.k2add(&self.k2mul(&a[0], &c0), &self.k2s(&t));
        let ni = self.k2inv(&n);
        [self.k2mul(&c0, &ni), self.k2mul(&c1, &ni), self.k2mul(&c2, &ni)]
    }
}

// ---------- elliptic curve y^2 = x^3 + a2 x^2 + a4 x ----------
#[derive(Clone, Copy, PartialEq, Debug)]
enum P {
    O,
    A(E6, E6),
}

struct Ec<'a> {
    f: &'a F,
    a2: E6,
    a4: E6,
}
impl<'a> Ec<'a> {
    fn on(&self, pt: &P) -> bool {
        match pt {
            P::O => true,
            P::A(x, y) => {
                let f = self.f;
                let x2 = f.mul(x, x);
                let rhs = f.add(&f.add(&f.mul(&x2, x), &f.mul(&self.a2, &x2)), &f.mul(&self.a4, x));
                f.mul(y, y) == rhs
            }
        }
    }
    fn add(&self, p1: &P, p2: &P) -> P {
        let f = self.f;
        match (p1, p2) {
            (P::O, q) | (q, P::O) => *q,
            (P::A(x1, y1), P::A(x2, y2)) => {
                let lam = if x1 == x2 {
                    if F::is_zero(&f.add(y1, y2)) {
                        return P::O;
                    }
                    let x1s = f.mul(x1, x1);
                    let num = f.add(
                        &f.add(&f.mul(&f.cst(3), &x1s), &f.mul(&f.mul(&f.cst(2), &self.a2), x1)),
                        &self.a4,
                    );
                    f.mul(&num, &f.inv(&f.add(y1, y1)))
                } else {
                    f.mul(&f.sub(y2, y1), &f.inv(&f.sub(x2, x1)))
                };
                let x3 = f.sub(&f.sub(&f.sub(&f.mul(&lam, &lam), &self.a2), x1), x2);
                let y3 = f.sub(&f.mul(&lam, &f.sub(x1, &x3)), y1);
                P::A(x3, y3)
            }
        }
    }
    fn mul(&self, pt: &P, mut k: u128) -> P {
        let mut r = P::O;
        let mut b = *pt;
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


/// For k in {1,2,3}: (k, j in F_{p^k}, E isomorphic over F_{p^6} to its p^k-conjugate),
/// the condition for a degree-p^k (twisted) Frobenius endomorphism of E over F_{p^6}.
fn endo_test(f: &F, a2: &E6, a4: &E6) -> Vec<(u32, bool, bool)> {
    let a2s = f.mul(a2, a2);
    let c = f.sub(&a2s, &f.mul(&f.cst(3), a4));
    let disc = f.mul(&f.mul(a4, a4), &f.sub(&a2s, &f.mul(&f.cst(4), a4)));
    let j = f.mul(&f.mul(&f.cst(256), &f.mul(&f.mul(&c, &c), &c)), &f.inv(&disc));
    let special = F::is_zero(&j) || j == f.cst(1728);
    let three_inv = f.inv(&f.cst(3));
    let a_s = f.sub(a4, &f.mul(&a2s, &three_inv));
    let b_s = f.sub(
        &f.mul(&f.mul(&f.cst(2), &f.mul(&a2s, a2)), &f.inv(&f.cst(27))),
        &f.mul(&f.mul(a2, a4), &three_inv),
    );
    [1u32, 2, 3]
        .iter()
        .map(|&k| {
            let jk = f.frob(&j, k) == j;
            let mut iso = false;
            if jk && !special {
                let ap = f.frob(&a_s, k);
                let bp = f.frob(&b_s, k);
                let z = f.mul(&f.mul(&bp, &a_s), &f.inv(&f.mul(&b_s, &ap)));
                // consistency: z^2 must equal A'/A when j agrees
                assert!(f.mul(&z, &z) == f.mul(&ap, &f.inv(&a_s)));
                iso = f.is_square(&z);
            }
            (k, jk, iso)
        })
        .collect()
}

fn mulmod(a: u128, b: u128, n: u128) -> u128 {
    // n < 2^64 here
    (a % n) * (b % n) % n
}
fn powmod(mut b: u128, mut e: u128, n: u128) -> u128 {
    let mut r = 1u128;
    b %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, n);
        }
        b = mulmod(b, b, n);
        e >>= 1;
    }
    r
}
fn is_prime(n: u128) -> bool {
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
    'w: for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}
fn isqrt(n: u128) -> u128 {
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn e6_from(v: &[u64]) -> E6 {
    [[v[0], v[1]], [v[2], v[3]], [v[4], v[5]]]
}
fn fmt(a: &E6) -> String {
    format!("{:?}", a)
}

// bitop/v1 prime-field prices at b bits (docs/units/BITOPS.md)
fn prices(b: u64) -> (u64, u64, u64) {
    let mulp = 2 * (b * b + 5 * b * (b - 1)) + 13 * b;
    let addp = 13 * b;
    let invp = 2 * b * 13 * b;
    (mulp, addp, invp)
}
fn price(c: &Cnt, b: u64) -> f64 {
    let (m, a, i) = prices(b);
    (c.m * m + c.a * a + c.i * i) as f64
}

struct Out {
    lines: Vec<String>,
}
impl Out {
    fn p(&mut self, s: String) {
        println!("{s}");
        self.lines.push(s);
    }
}

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_to_string(&mut inp).unwrap();
    let mut o = Out { lines: vec![] };
    let mut runs: Vec<(String, f64, f64)> = vec![];
    let mut targets: Vec<(String, Vec<u128>)> = vec![];
    for line in inp.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.is_empty() {
            continue;
        }
        if t[0] == "RUN" {
            runs.push((t[1].to_string(), t[2].parse().unwrap(), t[3].parse().unwrap()));
        } else {
            targets.push((t[0].to_string(), t[1..].iter().map(|x| x.parse().unwrap()).collect()));
        }
    }
    let mut all_ok = true;
    let mut primary_l = 0u128;
    let mut primary_p = 0u64;
    for (label, v) in &targets {
        let p = v[0] as u64;
        let l = v[1];
        let w64: Vec<u64> = v[2..].iter().map(|&x| x as u64).collect();
        let alpha = e6_from(&w64[0..6]);
        let g = P::A(e6_from(&w64[6..12]), e6_from(&w64[12..18]));
        let q = P::A(e6_from(&w64[18..24]), e6_from(&w64[24..30]));
        let f = F::new(p);
        let sa = f.frob(&alpha, 2);
        // repo sigma (zeta form) must equal the q-power Frobenius
        let sigma_ok = f.sigma_repo(&alpha) == sa;
        let a2 = f.sub(&[[0, 0]; 3], &f.add(&alpha, &sa));
        let a4 = f.mul(&alpha, &sa);
        let ec = Ec { f: &f, a2, a4 };
        let on_g = ec.on(&g);
        let on_q = ec.on(&q);
        let lg = ec.mul(&g, l) == P::O;
        let lq = ec.mul(&q, l) == P::O;
        let lprime = is_prime(l);
        let p6 = f.pk(6);
        let n = 4 * l;
        let t = p6 as i128 + 1 - n as i128;
        let hasse = (t * t) as u128 <= 4 * p6;
        o.p(format!("## target {label}: p={p} w={} s={:?} zeta={:?} l={l}", f.w, f.s, f.zeta));
        o.p(format!(
            "  checks: sigma(zeta form)==x^q {sigma_ok}; G,Q on curve {on_g},{on_q}; [l]G=O {lg}; [l]Q=O {lq}; l prime {lprime}; #E=4l trace t={t} within Hasse {hasse}"
        ));
        all_ok &= sigma_ok && on_g && on_q && lg && lq && lprime && hasse;
        // subfields
        let da = f.deg(&alpha);
        let d2 = f.deg(&a2);
        let d4 = f.deg(&a4);
        // j = 256 (a2^2 - 3 a4)^3 / (a4^2 (a2^2 - 4 a4))
        let a2s = f.mul(&a2, &a2);
        let c = f.sub(&a2s, &f.mul(&f.cst(3), &a4));
        let disc = f.mul(&f.mul(&a4, &a4), &f.sub(&a2s, &f.mul(&f.cst(4), &a4)));
        let j = f.mul(&f.mul(&f.cst(256), &f.mul(&f.mul(&c, &c), &c)), &f.inv(&disc));
        let dj = f.deg(&j);
        let j0 = F::is_zero(&j);
        let j1728 = j == f.cst(1728);
        o.p(format!(
            "  field of definition: deg(alpha)={da} deg(a2)={d2} deg(a4)={d4}; j={} deg(j)={dj}; j=0 {j0}; j=1728 {j1728}",
            fmt(&j)
        ));
        // twisted-Frobenius endomorphism test: E ~= E^(p^k) over F_{p^6}
        let endo: Vec<String> = endo_test(&f, &a2, &a4)
            .iter()
            .map(|(k, jk, iso)| format!("k={k}: j^(p^k)=j {jk}, E~=E^(p^k) over F_p6 {iso}"))
            .collect();
        o.p(format!("  subfield-Frobenius endomorphism: {}", endo.join("; ")));
        // isogeny-class base-change test: V_{6/k}(tau) = +-t, |tau| <= 2 sqrt(p^k)
        let mut bc = Vec::new();
        for k in [1u32, 2, 3] {
            let m = 6 / k;
            let pk = f.pk(k) as i128;
            let bound = isqrt(4 * pk as u128) as i128;
            let mut hits = vec![];
            for tau in -bound..=bound {
                let (mut v0, mut v1) = (2i128, tau);
                for _ in 1..m {
                    let v2 = tau * v1 - pk * v0;
                    v0 = v1;
                    v1 = v2;
                }
                if v1 == t || v1 == -t {
                    hits.push((tau, if v1 == t { "+" } else { "-" }));
                }
            }
            bc.push(format!("k={k}: matches {:?}", hits));
        }
        o.p(format!("  isogeny class from F_(p^k) (V_(6/k)(tau)=+-t): {}", bc.join("; ")));
        // higher twists (u of order 3,4,6) need Z[zeta3] or Z[i] in End: 4p^6 - t^2 = 3 f^2 or f^2
        let dfr = (4 * p6 as i128 - t * t) as u128;
        let sq = |x: u128| isqrt(x) * isqrt(x) == x;
        let cm3 = dfr % 3 == 0 && sq(dfr / 3);
        let cm4 = sq(dfr);
        o.p(format!("  Frobenius discriminant 4p^6-t^2={dfr}: Q(sqrt-3) {cm3}, Q(i) {cm4} (either needed for order-3/4/6 automorphisms anywhere in the isogeny class)"));
        // embedding degree of l in F_{p^6}
        let qm = p6 % l;
        let mut emb = 0;
        let mut acc = 1u128;
        for e in 1..=200 {
            acc = mulmod(acc, qm, l);
            if acc == 1 {
                emb = e;
                break;
            }
        }
        o.p(format!(
            "  embedding degree of l over F_p6: {} ; l==p {}",
            if emb == 0 { ">200".to_string() } else { emb.to_string() },
            l == p as u128
        ));
        // automorphism group order relevant to rho on <G>
        let aut = if j0 { 6 } else if j1728 { 4 } else { 2 };
        let frob_fold = if endo.iter().any(|s| s.ends_with("true")) { "available (see above)" } else { "none" };
        o.p(format!("  legitimate fold: |Aut|={aut} (negation only unless j in {{0,1728}}); Frobenius fold: {frob_fold}"));
        if primary_l == 0 || label.contains("run41-51") {
            primary_l = l;
            primary_p = p;
        }
    }


    // ---------- detector controls (positive and negative) ----------
    {
        let f = F::new(1009);
        let e = |v: [u64; 6]| e6_from(&v);
        let fmt_c = |r: Vec<(u32, bool, bool)>| {
            r.iter().map(|(k, a, b)| format!("k{k}:{a}/{b}")).collect::<Vec<_>>().join(" ")
        };
        // A: curve over F_p (a2=1, a4=3): expect true/true for k=1,2,3
        let ca = endo_test(&f, &e([1, 0, 0, 0, 0, 0]), &e([3, 0, 0, 0, 0, 0]));
        // B: its quadratic twist by a nonsquare d of F_p6 (GLS-type): expect iso true
        let mut d = e([5, 7, 3, 1, 0, 2]);
        while f.is_square(&d) {
            d = f.add(&d, &f.one());
        }
        let cb = endo_test(&f, &f.mul(&d, &e([1, 0, 0, 0, 0, 0])), &f.mul(&f.mul(&d, &d), &e([3, 0, 0, 0, 0, 0])));
        // C: curve over F_q = F_p2 only: expect k=2 true, k=1,3 false
        let cc = endo_test(&f, &e([5, 7, 0, 0, 0, 0]), &e([3, 0, 0, 0, 0, 0]));
        // D: curve over F_p3 only (coefficients in F_p(theta)): expect k=3 true, k=1,2 false
        let cd = endo_test(&f, &e([1, 0, 4, 0, 0, 0]), &e([3, 0, 0, 0, 2, 0]));
        let pass = ca.iter().all(|x| x.1 && x.2)
            && cb.iter().all(|x| x.1 && x.2)
            && cc.iter().map(|x| x.2).collect::<Vec<_>>() == vec![false, true, false]
            && cd.iter().map(|x| x.2).collect::<Vec<_>>() == vec![false, false, true];
        o.p(format!(
            "\n## detector controls (k: j-in-subfield/isomorphic-to-conjugate): A over F_p [{}]; B quadratic twist of A [{}]; C over F_p2 [{}]; D over F_p3 [{}]; expected pattern {}",
            fmt_c(ca), fmt_c(cb), fmt_c(cc), fmt_c(cd), pass
        ));
        all_ok &= pass;
    }
    // ---------- counted arithmetic check and per-step costs ----------
    let p = primary_p;
    let f = F::new(p);
    let b = 64 - (p as u64).leading_zeros() as u64;
    let mut seed = 0x1234_5678_9abc_def0u64;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed % p
    };
    let mut ok = true;
    let (mut cm, mut cs, mut ci) = (Cnt::default(), Cnt::default(), Cnt::default());
    for it in 0..2000 {
        let a: E6 = [[rnd(), rnd()], [rnd(), rnd()], [rnd(), rnd()]];
        let bb: E6 = [[rnd(), rnd()], [rnd(), rnd()], [rnd(), rnd()]];
        f.reset();
        ok &= f.kmul(&a, &bb) == f.mul(&a, &bb);
        let c1 = f.reset();
        ok &= f.ksq(&a) == f.mul(&a, &a);
        let c2 = f.reset();
        if !F::is_zero(&a) {
            ok &= f.mul(&f.kinv(&a), &a) == f.one();
        }
        let c3 = f.reset();
        if it == 0 {
            cm = c1;
            cs = c2;
            ci = c3;
        }
    }
    all_ok &= ok;
    o.p(format!("\n## counted Karatsuba F_p6 arithmetic at p={p} (b={b} bits), 2000 random operand checks vs schoolbook/x*x^-1=1: {ok}"));
    o.p(format!("  Karatsuba mul {:?}; Chung-Hasan sq {:?}; adjugate inv {:?}", cm, cs, ci));
    let native_mul = Cnt { m: 55, a: 44, i: 0 };
    let native_inv = Cnt { m: 80, a: 42, i: 1 };
    let (pm, pa, pi) = prices(b);
    o.p(format!("  bitop/v1 prices at b={b}: field_mul {pm}, field_add {pa}, field_inv {pi}"));
    o.p(format!(
        "  per F_p6 op bitops: native(registered) mul/sq {:.0} inv {:.0}; Karatsuba mul {:.0} sq {:.0} inv {:.0}",
        price(&native_mul, b),
        price(&native_inv, b),
        price(&cm, b),
        price(&cs, b),
        price(&ci, b)
    ));
    // per-step model: affine addition given 1/dx = 2 mul + 1 sq + 42 adds (incl. dx),
    // Montgomery trick 3(k-1)/k mul + inv/k.
    let step = |mul: f64, sq: f64, inv: f64, k: f64| -> f64 {
        2.0 * mul + sq + 3.0 * (k - 1.0) / k * mul + inv / k + 42.0 * pa as f64
    };
    let nm = price(&native_mul, b);
    let ni = price(&native_inv, b);
    let km = price(&cm, b);
    let ks = price(&cs, b);
    let ki = price(&ci, b);
    o.p("\n## per-step bitop model (arithmetic only; bookkeeping overhead from recorded arms added below)".into());
    let mut best_k = (0.0, f64::MAX);
    for k in [1.0, 2.0, 4.0, 8.0, 16.0] {
        let sn = step(nm, nm, ni, k);
        let sk = step(km, ks, ki, k);
        if sk < best_k.1 {
            best_k = (k, sk);
        }
        o.p(format!("  batch {k:>2}: registered-native {sn:>9.0}  Karatsuba {sk:>9.0}  ratio {:.3}", sn / sk));
    }
    let sn1 = step(nm, nm, ni, 1.0);
    let sn16 = step(nm, nm, ni, 16.0);
    // overhead calibration against recorded arms
    o.p("\n## recorded run50 rho arms (one cold run each, same target)".into());
    let l = primary_l as f64;
    let e_plain = (std::f64::consts::PI * l / 2.0).sqrt();
    let e_neg = (std::f64::consts::PI * l / 4.0).sqrt();
    let r_adj = (1.0f64 / (1.0 - 1.0 / 256.0)).sqrt();
    let cyc = 1.0 + 2.0 * 1.0 / (2.0 * 256.0); // ~2 wasted steps per 2-cycle, rate ~1/(2r)
    let mut ovh = vec![];
    for (lab, steps, bits) in &runs {
        let per = bits / steps;
        let model = if lab.contains("b16") { sn16 } else { sn1 };
        let neg = lab.contains("neg");
        let e = if neg { e_neg } else { e_plain };
        ovh.push((lab.clone(), per - model));
        o.p(format!(
            "  {lab}: steps {steps:.0} = {:.3} x E[steps] ({:.4e}); bitops/step {per:.0} (model arithmetic {model:.0}, overhead {:.0}); bitops {bits:.4e} vs expected {:.4e}",
            steps / e,
            e,
            per - model,
            e * r_adj * (if neg { cyc } else { 1.0 }) * per
        ));
    }
    let ov_plain = ovh.iter().find(|x| x.0.contains("plain-b1")).map(|x| x.1).unwrap_or(0.0);
    let ov_neg = ovh.iter().find(|x| x.0.contains("neg-b1")).map(|x| x.1).unwrap_or(0.0);
    o.p("\n## expected cost of each rho variant on l (r=256 adding walk factor sqrt(1/(1-1/r)) included)".into());
    o.p(format!("  l={primary_l} sqrt(pi l/2)={e_plain:.4e} sqrt(pi l/4)={e_neg:.4e}; r-adding factor {r_adj:.5}"));
    let rows = [
        ("plain b1 registered-native", e_plain * r_adj, sn1 + ov_plain),
        ("plain b16 registered-native", e_plain * r_adj, sn16 + ov_plain),
        ("negation b1 registered-native", e_neg * r_adj * cyc, sn1 + ov_neg),
        ("negation b16 registered-native", e_neg * r_adj * cyc, sn16 + ov_neg),
        ("negation b1 Karatsuba", e_neg * r_adj * cyc, step(km, ks, ki, 1.0) + ov_neg),
        ("negation best-k Karatsuba", e_neg * r_adj * cyc, best_k.1 + ov_neg),
    ];
    let base = rows[2].1 * rows[2].2;
    for (lab, it, per) in rows {
        o.p(format!(
            "  {lab:<32} E[iter] {it:.4e}  bitops/iter {per:>9.0}  E[bitops] {:.4e}  (x{:.3} vs negation b1 registered)",
            it * per,
            it * per / base
        ));
    }
    o.p(format!("  best Karatsuba batch k={}", best_k.0));
    o.p(format!("\nALL CHECKS PASS: {all_ok}"));
    if !all_ok {
        std::process::exit(1);
    }
}
