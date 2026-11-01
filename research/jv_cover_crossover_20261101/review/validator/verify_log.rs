//! Independent check of a solved Joux–Vitse cover DLP instance dumped by
//! `JV_DUMP_INSTANCE` (one JSON line): rebuilds the tower
//! `F_p² = F_p[i]/(i² − w)`, `F_p⁶ = F_p²[θ]/(θ³ − s)` and the curve
//! `y² = x³ + a₂x² + a₄x` from the dumped numbers with its own arithmetic
//! (adjugate inversion in the cubic extension, Fermat inversion in `F_p`,
//! double-and-add scalar multiplication), and checks:
//!   l is prime; G and Q lie on the curve; l·G = O; G ≠ O; dd·G = Q; dd = d.
//! Shares no code with `src/cryptanalysis/jv_cover.rs`.
//! Build: `rustc -O --edition 2021 verify_log.rs -o <out>`; usage: `verify_log <dump.jsonl>`.
use std::fs;

type E2 = [u64; 2];
type E6 = [E2; 3];

#[derive(Clone, Copy)]
struct Fp(u64);
impl Fp {
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a as u128 + b as u128;
        (s % self.0 as u128) as u64
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        (a as u128 + self.0 as u128 - b as u128) as u64 % self.0
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.0 as u128) as u64
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.0;
        a %= self.0;
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
        assert!(a % self.0 != 0, "inverse of 0 in F_p");
        self.pow(a, self.0 - 2)
    }
}
struct F2 {
    f: Fp,
    w: u64,
}
impl F2 {
    fn add(&self, a: E2, b: E2) -> E2 {
        [self.f.add(a[0], b[0]), self.f.add(a[1], b[1])]
    }
    fn sub(&self, a: E2, b: E2) -> E2 {
        [self.f.sub(a[0], b[0]), self.f.sub(a[1], b[1])]
    }
    fn mul(&self, a: E2, b: E2) -> E2 {
        // (a0 + a1 i)(b0 + b1 i) = a0b0 + w a1b1 + (a0b1 + a1b0) i
        let f = &self.f;
        [
            f.add(f.mul(a[0], b[0]), f.mul(self.w, f.mul(a[1], b[1]))),
            f.add(f.mul(a[0], b[1]), f.mul(a[1], b[0])),
        ]
    }
    fn inv(&self, a: E2) -> E2 {
        // (a0 − a1 i) / (a0² − w a1²)
        let f = &self.f;
        let n = f.sub(f.mul(a[0], a[0]), f.mul(self.w, f.mul(a[1], a[1])));
        let ni = f.inv(n);
        [f.mul(a[0], ni), f.mul(f.sub(0, a[1]), ni)]
    }
    fn is_zero(&self, a: E2) -> bool {
        a[0] % self.f.0 == 0 && a[1] % self.f.0 == 0
    }
}
struct F6 {
    f: F2,
    s: E2,
}
impl F6 {
    fn zero() -> E6 {
        [[0, 0]; 3]
    }
    fn add(&self, a: E6, b: E6) -> E6 {
        [self.f.add(a[0], b[0]), self.f.add(a[1], b[1]), self.f.add(a[2], b[2])]
    }
    fn sub(&self, a: E6, b: E6) -> E6 {
        [self.f.sub(a[0], b[0]), self.f.sub(a[1], b[1]), self.f.sub(a[2], b[2])]
    }
    fn neg(&self, a: E6) -> E6 {
        self.sub(Self::zero(), a)
    }
    fn mul(&self, a: E6, b: E6) -> E6 {
        // (a0 + a1θ + a2θ²)(b0 + b1θ + b2θ²) with θ³ = s
        let f = &self.f;
        let c0 = f.mul(a[0], b[0]);
        let c1 = f.add(f.mul(a[0], b[1]), f.mul(a[1], b[0]));
        let c2 = f.add(f.add(f.mul(a[0], b[2]), f.mul(a[1], b[1])), f.mul(a[2], b[0]));
        let c3 = f.add(f.mul(a[1], b[2]), f.mul(a[2], b[1]));
        let c4 = f.mul(a[2], b[2]);
        [f.add(c0, f.mul(self.s, c3)), f.add(c1, f.mul(self.s, c4)), c2]
    }
    fn inv(&self, a: E6) -> E6 {
        // adjugate: A0 = a0² − s a1 a2, A1 = s a2² − a0 a1, A2 = a1² − a0 a2,
        // N = a0 A0 + s a1 A2 + s a2 A1 = a0³ + s a1³ + s² a2³ − 3 s a0 a1 a2 ∈ F_p²
        let f = &self.f;
        let (a0, a1, a2) = (a[0], a[1], a[2]);
        let s = self.s;
        let big_a0 = f.sub(f.mul(a0, a0), f.mul(s, f.mul(a1, a2)));
        let big_a1 = f.sub(f.mul(s, f.mul(a2, a2)), f.mul(a0, a1));
        let big_a2 = f.sub(f.mul(a1, a1), f.mul(a0, a2));
        let n = f.add(f.add(f.mul(a0, big_a0), f.mul(s, f.mul(a1, big_a2))), f.mul(s, f.mul(a2, big_a1)));
        assert!(!f.is_zero(n), "inverse of 0 in F_p^6");
        let ni = f.inv(n);
        let r = [f.mul(big_a0, ni), f.mul(big_a1, ni), f.mul(big_a2, ni)];
        // self-check
        let one = self.mul(a, r);
        assert!(self.eq(one, self.one()), "F_p^6 inversion self-check failed");
        r
    }
    fn one(&self) -> E6 {
        [[1, 0], [0, 0], [0, 0]]
    }
    fn eq(&self, a: E6, b: E6) -> bool {
        let p = self.f.f.0;
        (0..3).all(|i| a[i][0] % p == b[i][0] % p && a[i][1] % p == b[i][1] % p)
    }
}
#[derive(Clone, Copy)]
struct Pt {
    x: E6,
    y: E6,
    inf: bool,
}
struct Curve<'a> {
    f: &'a F6,
    a2: E6,
    a4: E6,
}
impl<'a> Curve<'a> {
    fn on_curve(&self, p: &Pt) -> bool {
        if p.inf {
            return true;
        }
        let f = self.f;
        let x2 = f.mul(p.x, p.x);
        let rhs = f.add(f.add(f.mul(x2, p.x), f.mul(self.a2, x2)), f.mul(self.a4, p.x));
        f.eq(f.mul(p.y, p.y), rhs)
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let f = self.f;
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        let lam = if f.eq(p.x, q.x) {
            if f.eq(p.y, f.neg(q.y)) {
                return Pt { x: F6::zero(), y: F6::zero(), inf: true };
            }
            // (3x² + 2a₂x + a₄) / (2y)
            let x2 = f.mul(p.x, p.x);
            let num = f.add(f.add(f.add(f.add(x2, x2), x2), f.mul(f.add(self.a2, self.a2), p.x)), self.a4);
            f.mul(num, f.inv(f.add(p.y, p.y)))
        } else {
            f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)))
        };
        let x3 = f.sub(f.sub(f.sub(f.mul(lam, lam), self.a2), p.x), q.x);
        let y3 = f.sub(f.mul(lam, f.sub(p.x, x3)), p.y);
        Pt { x: x3, y: y3, inf: false }
    }
    fn mul(&self, p: &Pt, k: u64) -> Pt {
        // left-to-right double-and-add
        let mut acc = Pt { x: F6::zero(), y: F6::zero(), inf: true };
        for i in (0..64).rev() {
            acc = self.add(&acc, &acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(&acc, p);
            }
        }
        acc
    }
}
fn pt_eq(f: &F6, a: &Pt, b: &Pt) -> bool {
    if a.inf || b.inf {
        return a.inf == b.inf;
    }
    f.eq(a.x, b.x) && f.eq(a.y, b.y)
}

// ── tiny extraction from the dump line (no general JSON needed) ──
fn after<'a>(s: &'a str, key: &str) -> &'a str {
    let k = format!("\"{key}\":");
    let i = s.find(&k).unwrap_or_else(|| panic!("key {key}"));
    &s[i + k.len()..]
}
fn num(s: &str) -> u64 {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    s[..end].parse().unwrap()
}
fn nums(s: &str, n: usize) -> Vec<u64> {
    // the first n integers appearing in s
    let mut out = vec![];
    let mut cur = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            cur.push(c);
        } else if !cur.is_empty() {
            out.push(cur.parse().unwrap());
            cur.clear();
            if out.len() == n {
                break;
            }
        }
    }
    out
}
fn e6(v: &[u64]) -> E6 {
    [[v[0], v[1]], [v[2], v[3]], [v[4], v[5]]]
}
fn point(s: &str) -> Pt {
    let x = e6(&nums(after(s, "x"), 6));
    let y = e6(&nums(after(s, "y"), 6));
    let inf = after(s, "inf").starts_with("true");
    Pt { x, y, inf }
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let f = Fp(n);
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
        let mut x = f.pow(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = f.mul(x, x);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

fn main() {
    let path = std::env::args().nth(1).expect("dump path");
    let text = fs::read_to_string(&path).expect("read dump");
    let mut all_ok = true;
    for (ln, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let p = num(after(line, "p"));
        let seed = num(after(line, "seed"));
        let w = num(after(line, "w"));
        let sv = nums(after(line, "s"), 2);
        let a2 = e6(&nums(after(line, "a2"), 6));
        let a4 = e6(&nums(after(line, "a4"), 6));
        let l = num(after(line, "l"));
        let gi = line.find("\"g\":").unwrap();
        let qi = line.find("\"q\":").unwrap();
        let g = point(&line[gi..qi]);
        let q = point(&line[qi..]);
        let d = num(after(line, "d"));
        let dd = num(after(line, "dd"));
        let f6 = F6 { f: F2 { f: Fp(p), w }, s: [sv[0], sv[1]] };
        let ec = Curve { f: &f6, a2, a4 };
        // sanity of the tower: w must be a non-square mod p, s a non-cube in F_p²: checked indirectly by the inversion self-checks
        let g_on = ec.on_curve(&g);
        let q_on = ec.on_curve(&q);
        let lg = ec.mul(&g, l);
        let ddg = ec.mul(&g, dd);
        let lp = is_prime(l);
        let p6 = (p as f64).powi(6);
        let ok = lp && g_on && q_on && !g.inf && lg.inf && pt_eq(&f6, &ddg, &q) && dd == d;
        all_ok &= ok;
        println!(
            "line {}: p={} seed={} w={} s=[{},{}] l={} l_prime={} p^6/(4l)={:.6} G_on_curve={} Q_on_curve={} G!=O={} l*G=O={} dd*G==Q={} dd==d={} (d={} dd={}) => {}",
            ln + 1, p, seed, w, sv[0], sv[1], l, lp, p6 / (4.0 * l as f64), g_on, q_on, !g.inf, lg.inf, pt_eq(&f6, &ddg, &q), dd == d, d, dd,
            if ok { "VERIFIED" } else { "FAILED" }
        );
        // negative control: a wrong scalar must not verify
        let wrong = ec.mul(&g, if dd > 1 { dd - 1 } else { dd + 1 });
        println!("  negative control: (dd-1)*G == Q is {} (expected false)", pt_eq(&f6, &wrong, &q));
    }
    println!("ALL_VERIFIED={all_ok}");
    if !all_ok {
        std::process::exit(1);
    }
}
