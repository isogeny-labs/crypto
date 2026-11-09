//! cmsem: coefficient-size / curve-model study of prime-field Semaev decomposition.
//!
//! Subcommands
//!   find <mode> <p> <a0> <b0> <n> <K> [arg]   class-member search (n*P = O filter)
//!        mode = small (arg = max |coef|), a3 (arg = max |b|), rand (arg = seed)
//!   run <cell.json-like args via stdin lines>   decomposition runs, JSONL to stdout
//!
//! The run input is a plain line format (written by setup.py) so the tool has
//! no dependencies:
//!   cell <name> <p> <n_group> <B> <T> <mode s3|s4> <threads> <order_seed> <xcheck>
//!   arm <name> <a> <b> <coord X|MONT|EDY> <alpha> <s> <lo> <rootfinder WS3|MS3|NONE> <planted 0|1> <seed>
//! All field elements are given as canonical decimal integers mod p.

use std::cell::Cell;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod analyze;
mod nt;
mod setup;
mod verify;

thread_local! {
    static MULS: Cell<u64> = Cell::new(0);
    static JACS: Cell<u64> = Cell::new(0);
    static SQRTS: Cell<u64> = Cell::new(0);
    static INVS: Cell<u64> = Cell::new(0);
}
fn cnt(c: &'static std::thread::LocalKey<Cell<u64>>) {
    c.with(|v| v.set(v.get() + 1));
}
fn take(c: &'static std::thread::LocalKey<Cell<u64>>) -> u64 {
    c.with(|v| v.replace(0))
}

// ---------------------------------------------------------------- RNG
#[derive(Clone)]
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        // rejection sampling, unbiased
        let zone = u64::MAX - (u64::MAX % m);
        loop {
            let v = self.next();
            if v < zone {
                return v % m;
            }
        }
    }
}
fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// ---------------------------------------------------------------- field (Montgomery, p < 2^62)
#[derive(Clone, Copy)]
struct Fp {
    p: u64,
    pinv: u64, // -p^{-1} mod 2^64
    r2: u64,
    one: u64,
}
impl Fp {
    fn new(p: u64) -> Fp {
        assert!(p < (1u64 << 62) && p % 2 == 1);
        let mut inv: u64 = 1;
        for _ in 0..7 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        let pinv = inv.wrapping_neg();
        let r = ((1u128 << 64) % p as u128) as u64;
        let r2 = ((r as u128 * r as u128) % p as u128) as u64;
        Fp { p, pinv, r2, one: r }
    }
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.pinv);
        let u = ((t + (m as u128) * (self.p as u128)) >> 64) as u64;
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        cnt(&MULS);
        self.redc(a as u128 * b as u128)
    }
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline(always)]
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    fn to_m(&self, x: u64) -> u64 {
        self.mul(x % self.p, self.r2)
    }
    fn from_m(&self, x: u64) -> u64 {
        cnt(&MULS);
        self.redc(x as u128)
    }
    fn pow(&self, a: u64, mut e: u64) -> u64 {
        let mut r = self.one;
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        cnt(&INVS);
        self.pow(a, self.p - 2)
    }
    /// square root for p = 3 mod 4; None if non-residue
    fn sqrt(&self, a: u64) -> Option<u64> {
        cnt(&SQRTS);
        let s = self.pow(a, (self.p + 1) / 4);
        if self.mul(s, s) == a {
            Some(s)
        } else {
            None
        }
    }
    /// Jacobi symbol of a Montgomery-form element (converts, 1 mult)
    fn jacobi_m(&self, am: u64) -> i32 {
        let a = self.from_m(am);
        cnt(&JACS);
        jacobi(a, self.p)
    }
    fn small(&self, v: i64) -> u64 {
        let c = if v >= 0 {
            (v as u64) % self.p
        } else {
            self.p - ((-v) as u64 % self.p)
        };
        self.to_m(c)
    }
}
fn jacobi(mut a: u64, mut n: u64) -> i32 {
    if a == 0 {
        return 0;
    }
    let mut t = 1i32;
    while a != 0 {
        let z = a.trailing_zeros();
        a >>= z;
        if z & 1 == 1 && (n % 8 == 3 || n % 8 == 5) {
            t = -t;
        }
        if a % 4 == 3 && n % 4 == 3 {
            t = -t;
        }
        let r = n % a;
        n = a;
        a = r;
    }
    if n == 1 {
        t
    } else {
        0
    }
}
fn batch_inv(f: &Fp, v: &mut [u64], scratch: &mut Vec<u64>) {
    // in-place inverses; all entries nonzero
    let n = v.len();
    if n == 0 {
        return;
    }
    scratch.clear();
    let mut acc = f.one;
    for &x in v.iter() {
        scratch.push(acc);
        acc = f.mul(acc, x);
    }
    let mut inv = f.inv(acc);
    for i in (0..n).rev() {
        let t = f.mul(inv, scratch[i]);
        inv = f.mul(inv, v[i]);
        v[i] = t;
    }
}

// ---------------------------------------------------------------- curve (short Weierstrass)
#[derive(Clone, Copy)]
struct Curve {
    f: Fp,
    a: u64,
    b: u64,
}
impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let f = &self.f;
        let x2 = f.mul(x, x);
        f.add(f.mul(f.add(x2, self.a), x), self.b)
    }
    fn random_point(&self, rng: &mut Rng) -> (u64, u64) {
        let f = &self.f;
        loop {
            let x = f.to_m(rng.below(f.p));
            let r = self.rhs(x);
            if r == 0 {
                continue;
            }
            if let Some(y) = f.sqrt(r) {
                let y = if rng.next() & 1 == 1 { f.neg(y) } else { y };
                return (x, y);
            }
        }
    }
    // Jacobian
    fn jdbl(&self, p: (u64, u64, u64)) -> (u64, u64, u64) {
        let f = &self.f;
        let (x, y, z) = p;
        if z == 0 || y == 0 {
            return (f.one, f.one, 0);
        }
        let xx = f.mul(x, x);
        let yy = f.mul(y, y);
        let yyyy = f.mul(yy, yy);
        let zz = f.mul(z, z);
        let s = f.mul(f.add(f.add(x, x), f.add(x, x)), yy);
        let m = f.add(f.add(f.add(xx, xx), xx), f.mul(self.a, f.mul(zz, zz)));
        let x3 = f.sub(f.mul(m, m), f.add(s, s));
        let y8 = f.add(f.add(f.add(yyyy, yyyy), f.add(yyyy, yyyy)), f.add(f.add(yyyy, yyyy), f.add(yyyy, yyyy)));
        let y3 = f.sub(f.mul(m, f.sub(s, x3)), y8);
        let z3 = f.mul(f.add(y, y), z);
        (x3, y3, z3)
    }
    fn jmadd(&self, p: (u64, u64, u64), q: (u64, u64)) -> (u64, u64, u64) {
        let f = &self.f;
        let (x1, y1, z1) = p;
        if z1 == 0 {
            return (q.0, q.1, f.one);
        }
        let z1z1 = f.mul(z1, z1);
        let u2 = f.mul(q.0, z1z1);
        let s2 = f.mul(q.1, f.mul(z1, z1z1));
        let h = f.sub(u2, x1);
        let r = f.sub(s2, y1);
        if h == 0 {
            if r == 0 {
                return self.jdbl(p);
            }
            return (f.one, f.one, 0);
        }
        let hh = f.mul(h, h);
        let hhh = f.mul(h, hh);
        let v = f.mul(x1, hh);
        let x3 = f.sub(f.sub(f.mul(r, r), hhh), f.add(v, v));
        let y3 = f.sub(f.mul(r, f.sub(v, x3)), f.mul(y1, hhh));
        let z3 = f.mul(z1, h);
        (x3, y3, z3)
    }
    fn mul_is_inf(&self, k: u64, p: (u64, u64)) -> bool {
        let mut acc = (self.f.one, self.f.one, 0u64);
        for i in (0..64).rev() {
            acc = self.jdbl(acc);
            if (k >> i) & 1 == 1 {
                acc = self.jmadd(acc, p);
            }
        }
        acc.2 == 0
    }
    fn to_affine(&self, j: (u64, u64, u64)) -> Option<(u64, u64)> {
        if j.2 == 0 {
            return None;
        }
        let f = &self.f;
        let zi = f.inv(j.2);
        let zi2 = f.mul(zi, zi);
        Some((f.mul(j.0, zi2), f.mul(j.1, f.mul(zi2, zi))))
    }
    fn add_aff(&self, p: (u64, u64), q: (u64, u64)) -> Option<(u64, u64)> {
        let j = self.jmadd((p.0, p.1, self.f.one), q);
        self.to_affine(j)
    }
}

// ---------------------------------------------------------------- find
fn cmd_find(args: &[String]) {
    let mode = args[0].as_str();
    let p: u64 = args[1].parse().unwrap();
    let n: u64 = args[4].parse().unwrap();
    let k: usize = args[5].parse().unwrap();
    let arg: u64 = args[6].parse().unwrap();
    let (tst, el, fl) = find_members(mode, p, n, k, arg);
    println!(
        "{{\"mode\":\"{}\",\"tested\":{},\"found\":{},\"secs\":{:.3},\"members\":[{}]}}",
        mode,
        tst,
        fl.len(),
        el,
        fl.iter()
            .map(|(a, b, ac, bc)| format!("{{\"a_small\":{},\"b_small\":{},\"a\":{},\"b\":{}}}", a, b, ac, bc))
            .collect::<Vec<_>>()
            .join(",")
    );
}

pub(crate) fn find_members(mode: &str, p: u64, n: u64, k: usize, arg: u64) -> (usize, f64, Vec<(i64, i64, u64, u64)>) {
    let f = Fp::new(p);
    let nthreads = 4usize;
    let found = Arc::new(Mutex::new(Vec::<(i64, i64, u64, u64)>::new()));
    let tested = Arc::new(AtomicUsize::new(0));
    let t0 = Instant::now();
    let test = move |a: u64, b: u64, rng: &mut Rng| -> bool {
        // a, b Montgomery form
        let c = Curve { f, a, b };
        // discriminant
        let a3 = f.mul(f.mul(a, a), a);
        let d = f.add(f.mul(f.small(4), a3), f.mul(f.small(27), f.mul(b, b)));
        if d == 0 {
            return false;
        }
        let pt = c.random_point(rng);
        if !c.mul_is_inf(n, pt) {
            return false;
        }
        let pt2 = c.random_point(rng);
        c.mul_is_inf(n, pt2)
    };
    match mode {
        "small" | "a3" => {
            // enumerate shells s = 1..=arg of max(|a|,|b|) (small) or |b| (a3)
            let maxs = arg as i64;
            let next_shell = Arc::new(AtomicUsize::new(1));
            let stop_at = Arc::new(Mutex::new(i64::MAX));
            let mut hs = vec![];
            for tid in 0..nthreads {
                let found = found.clone();
                let tested = tested.clone();
                let next_shell = next_shell.clone();
                let stop_at = stop_at.clone();
                let mode = mode.to_string();
                let test = test;
                hs.push(std::thread::spawn(move || {
                    let mut rng = Rng(fnv(&format!("find/{}/{}", mode, tid)));
                    loop {
                        let s = next_shell.fetch_add(1, Ordering::SeqCst) as i64;
                        if s > maxs || s > *stop_at.lock().unwrap() {
                            break;
                        }
                        let mut pairs: Vec<(i64, i64)> = vec![];
                        if mode == "small" {
                            for a in -s..=s {
                                for b in [-s, s] {
                                    pairs.push((a, b));
                                }
                            }
                            for b in (-s + 1)..s {
                                for a in [-s, s] {
                                    pairs.push((a, b));
                                }
                            }
                        } else {
                            pairs.push((-3, s));
                            pairs.push((-3, -s));
                        }
                        for (a, b) in pairs {
                            if b == 0 || a == 0 {
                                continue; // j = 1728 / 0 excluded (E2)
                            }
                            tested.fetch_add(1, Ordering::Relaxed);
                            if test(f.small(a), f.small(b), &mut rng) {
                                let ac = if a >= 0 { a as u64 } else { p - (-a) as u64 };
                                let bc = if b >= 0 { b as u64 } else { p - (-b) as u64 };
                                let mut fl = found.lock().unwrap();
                                fl.push((a, b, ac, bc));
                                if fl.len() >= k {
                                    let mut st = stop_at.lock().unwrap();
                                    if s < *st {
                                        *st = s; // finish this shell, then stop
                                    }
                                }
                            }
                        }
                    }
                }));
            }
            for h in hs {
                h.join().unwrap();
            }
        }
        "rand" => {
            let mut hs = vec![];
            for tid in 0..nthreads {
                let found = found.clone();
                let tested = tested.clone();
                hs.push(std::thread::spawn(move || {
                    let mut rng = Rng(fnv(&format!("find/rand/{}/{}", arg, tid)));
                    loop {
                        if found.lock().unwrap().len() >= k {
                            break;
                        }
                        for _ in 0..256 {
                            let a = rng.below(p);
                            let b = rng.below(p);
                            tested.fetch_add(1, Ordering::Relaxed);
                            if test(f.to_m(a), f.to_m(b), &mut rng) {
                                found.lock().unwrap().push((0, 0, a, b));
                                break;
                            }
                        }
                    }
                }));
            }
            for h in hs {
                h.join().unwrap();
            }
        }
        _ => panic!("mode"),
    }
    let mut fl = found.lock().unwrap().clone();
    fl.sort_by_key(|&(a, b, _, _)| (a.abs().max(b.abs()), a.abs() + b.abs(), a, b));
    let el = t0.elapsed().as_secs_f64();
    let tst = tested.load(Ordering::SeqCst);
    (tst, el, fl)
}

// ---------------------------------------------------------------- arms / bases
#[derive(Clone, Copy, PartialEq)]
enum Coord {
    X,
    Mont,
    EdY,
}
#[derive(Clone, Copy, PartialEq)]
enum Root {
    WS3,
    MS3,
    None,
}
struct Arm {
    name: String,
    curve: Curve,
    coord: Coord,
    alpha: u64, // mont
    sinv: u64,  // mont, 1/s
    amont: u64, // Montgomery A = 3 alpha / s (mont)
    lo: u64,    // canonical
    root: Root,
    planted: bool,
    seed: u64,
    // base: Weierstrass x (mont), model coordinate c (mont)
    bx: Vec<u64>,
    bc: Vec<u64>,
    fpts: u64,
}
struct CellCfg {
    name: String,
    p: u64,
    ngroup: u64,
    bw: u64,
    t: usize,
    mode: String,
    threads: usize,
    order_seed: u64,
    xcheck: usize,
}

fn in_interval(f: &Fp, cm: u64, lo: u64, bw: u64) -> bool {
    let c = f.from_m(cm);
    let d = if c >= lo { c - lo } else { c + f.p - lo };
    d < bw
}
/// model coordinate of a Weierstrass x (mont); None if undefined (EdY at u = -1)
fn coord_of(arm: &Arm, xw: u64) -> Option<u64> {
    let f = &arm.curve.f;
    match arm.coord {
        Coord::X => Some(xw),
        Coord::Mont => Some(f.mul(f.sub(xw, arm.alpha), arm.sinv)),
        Coord::EdY => {
            let u = f.mul(f.sub(xw, arm.alpha), arm.sinv);
            let den = f.add(u, f.one);
            if den == 0 {
                return None;
            }
            Some(f.mul(f.sub(u, f.one), f.inv(den)))
        }
    }
}
fn build_base(arm: &mut Arm, cfg: &CellCfg) {
    let f = arm.curve.f;
    let mut bx = vec![];
    let mut bc = vec![];
    let mut fpts = 0u64;
    for k in 0..cfg.bw {
        let c = (arm.lo + k) % f.p;
        let cm = f.to_m(c);
        let xw = match arm.coord {
            Coord::X => cm,
            Coord::Mont => f.add(f.mul(cm, f.inv(arm.sinv)), arm.alpha),
            Coord::EdY => {
                // u = (1 + y) / (1 - y)
                let den = f.sub(f.one, cm);
                if den == 0 {
                    continue;
                }
                let u = f.mul(f.add(f.one, cm), f.inv(den));
                f.add(f.mul(u, f.inv(arm.sinv)), arm.alpha)
            }
        };
        let r = arm.curve.rhs(xw);
        let j = f.jacobi_m(r);
        if j >= 0 {
            bx.push(xw);
            bc.push(cm);
            fpts += if j == 0 { 1 } else { 2 };
        }
    }
    arm.bx = bx;
    arm.bc = bc;
    arm.fpts = fpts;
}

/// S3 root-finder count for target abscissa (model coordinate) cr over the base.
/// Weierstrass: S3(x1, X, xr) ; Montgomery: same with Montgomery coefficients.
fn s3_count(arm: &Arm, cr: u64, bw: u64, buf: &mut S3Buf) -> u64 {
    let f = &arm.curve.f;
    let (a, b) = (arm.curve.a, arm.curve.b);
    let two = f.add(f.one, f.one);
    let four = f.add(two, two);
    let mut count = 0u64;
    const CH: usize = 1024;
    let n = arm.bc.len();
    let mut i0 = 0;
    while i0 < n {
        let i1 = (i0 + CH).min(n);
        buf.num1.clear();
        buf.num2.clear();
        buf.den.clear();
        for i in i0..i1 {
            let x1 = arm.bc[i];
            let s = f.add(x1, cr);
            let pr = f.mul(x1, cr);
            let d = f.sub(x1, cr);
            let c2 = f.mul(d, d);
            let (c1, c0) = match arm.root {
                Root::WS3 => {
                    let inner = f.add(f.mul(s, f.add(pr, a)), f.add(b, b));
                    let c1 = f.neg(f.add(inner, inner));
                    let t = f.sub(pr, a);
                    let c0 = f.sub(f.mul(t, t), f.mul(four, f.mul(b, s)));
                    (c1, c0)
                }
                Root::MS3 => {
                    let am = arm.amont;
                    let inner = f.add(f.mul(f.add(pr, f.one), s), f.mul(f.add(am, am), pr));
                    let c1 = f.neg(f.add(inner, inner));
                    let t = f.sub(pr, f.one);
                    (c1, f.mul(t, t))
                }
                Root::None => unreachable!(),
            };
            if c2 == 0 {
                // linear: c1 X + c0 = 0 (the other root is at infinity)
                if c1 != 0 {
                    let x = f.mul(f.neg(c0), f.inv(c1));
                    if in_interval(f, x, arm.lo, bw) {
                        count += 1;
                    }
                }
                continue;
            }
            let disc = f.sub(f.mul(c1, c1), f.mul(four, f.mul(c2, c0)));
            let j = f.jacobi_m(disc);
            if j < 0 {
                continue;
            }
            let sq = if j == 0 { 0 } else { f.sqrt(disc).unwrap() };
            let mc1 = f.neg(c1);
            buf.num1.push(f.add(mc1, sq));
            buf.num2.push(f.sub(mc1, sq));
            buf.den.push(f.add(c2, c2));
        }
        batch_inv(f, &mut buf.den, &mut buf.scratch);
        for k in 0..buf.den.len() {
            let r1 = f.mul(buf.num1[k], buf.den[k]);
            let r2 = f.mul(buf.num2[k], buf.den[k]);
            if in_interval(f, r1, arm.lo, bw) {
                count += 1;
            }
            if in_interval(f, r2, arm.lo, bw) {
                count += 1;
            }
        }
        i0 = i1;
    }
    count
}
struct S3Buf {
    num1: Vec<u64>,
    num2: Vec<u64>,
    den: Vec<u64>,
    scratch: Vec<u64>,
}

/// group-law count Y(R) = #{P in F : R - P in F}, Weierstrass arithmetic,
/// model coordinate tested for interval membership.
fn gl_count(arm: &Arm, r: (u64, u64), bw: u64) -> u64 {
    let f = &arm.curve.f;
    let c = &arm.curve;
    let mut count = 0u64;
    for &x1 in arm.bx.iter() {
        let rr = c.rhs(x1);
        let y1 = if rr == 0 { 0 } else { f.sqrt(rr).unwrap() };
        let pts: Vec<(u64, u64)> = if y1 == 0 { vec![(x1, 0)] } else { vec![(x1, y1), (x1, f.neg(y1))] };
        for p1 in pts {
            // R - P1
            let q = c.add_aff(r, (p1.0, f.neg(p1.1)));
            if let Some(q) = q {
                if let Some(cq) = coord_of(arm, q.0) {
                    if in_interval(f, cq, arm.lo, bw) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn target(arm: &Arm, idx: usize) -> ((u64, u64), bool) {
    let mut rng = Rng(arm.seed ^ fnv(&format!("target/{}", idx)));
    let c = &arm.curve;
    if arm.planted && rng.below(10) == 0 {
        let pick = |rng: &mut Rng| -> (u64, u64) {
            let x = arm.bx[rng.below(arm.bx.len() as u64) as usize];
            let rr = c.rhs(x);
            let y = if rr == 0 { 0 } else { c.f.sqrt(rr).unwrap() };
            if rng.next() & 1 == 1 {
                (x, c.f.neg(y))
            } else {
                (x, y)
            }
        };
        loop {
            let p1 = pick(&mut rng);
            let p2 = pick(&mut rng);
            if let Some(r) = c.add_aff(p1, p2) {
                return (r, true);
            }
        }
    }
    (c.random_point(&mut rng), false)
}

fn parse_cell(lines: &[String]) -> (CellCfg, Vec<Arm>) {
    let mut cfg: Option<CellCfg> = None;
    let mut arms = vec![];
    for l in lines {
        let w: Vec<&str> = l.split_whitespace().collect();
        if w.is_empty() {
            continue;
        }
        match w[0] {
            "cell" => {
                cfg = Some(CellCfg {
                    name: w[1].into(),
                    p: w[2].parse().unwrap(),
                    ngroup: w[3].parse().unwrap(),
                    bw: w[4].parse().unwrap(),
                    t: w[5].parse().unwrap(),
                    mode: w[6].into(),
                    threads: w[7].parse().unwrap(),
                    order_seed: w[8].parse().unwrap(),
                    xcheck: w[9].parse().unwrap(),
                })
            }
            "arm" => {
                let c = cfg.as_ref().unwrap();
                let f = Fp::new(c.p);
                let a = f.to_m(w[2].parse().unwrap());
                let b = f.to_m(w[3].parse().unwrap());
                let coord = match w[4] {
                    "X" => Coord::X,
                    "MONT" => Coord::Mont,
                    "EDY" => Coord::EdY,
                    _ => panic!(),
                };
                let alpha = f.to_m(w[5].parse().unwrap());
                let s = f.to_m(w[6].parse().unwrap());
                let sinv = if s == 0 { 0 } else { f.inv(s) };
                // A = 3 alpha / s
                let amont = if s == 0 { 0 } else { f.mul(f.small(3), f.mul(alpha, sinv)) };
                arms.push(Arm {
                    name: w[1].into(),
                    curve: Curve { f, a, b },
                    coord,
                    alpha,
                    sinv,
                    amont,
                    lo: w[7].parse().unwrap(),
                    root: match w[8] {
                        "WS3" => Root::WS3,
                        "MS3" => Root::MS3,
                        _ => Root::None,
                    },
                    planted: w[9] == "1",
                    seed: fnv(w[10]),
                    bx: vec![],
                    bc: vec![],
                    fpts: 0,
                });
            }
            _ => panic!("bad line {}", l),
        }
    }
    (cfg.unwrap(), arms)
}

fn cmd_run() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let (cfg, mut arms) = parse_cell(&lines);
    let t0 = Instant::now();
    // build bases (parallel over arms)
    {
        let cfg_ref = &cfg;
        std::thread::scope(|sc| {
            let chunks: Vec<&mut Arm> = arms.iter_mut().collect();
            let per = (chunks.len() + cfg_ref.threads - 1) / cfg_ref.threads;
            let mut it = chunks.into_iter();
            let mut groups: Vec<Vec<&mut Arm>> = vec![];
            loop {
                let g: Vec<&mut Arm> = it.by_ref().take(per.max(1)).collect();
                if g.is_empty() {
                    break;
                }
                groups.push(g);
            }
            for g in groups {
                sc.spawn(move || {
                    for a in g {
                        build_base(a, cfg_ref);
                    }
                });
            }
        });
    }
    for a in arms.iter() {
        eprintln!("base {} |F_x|={} |F|={} ({:.1}s)", a.name, a.bc.len(), a.fpts, t0.elapsed().as_secs_f64());
        println!(
            "{{\"kind\":\"base\",\"cell\":\"{}\",\"arm\":\"{}\",\"fx\":{},\"fpts\":{},\"ngroup\":{},\"B\":{}}}",
            cfg.name, a.name, a.bc.len(), a.fpts, cfg.ngroup, cfg.bw
        );
    }
    // jobs, shuffled
    let mut jobs: Vec<(usize, usize)> = vec![];
    for ai in 0..arms.len() {
        for t in 0..cfg.t {
            jobs.push((ai, t));
        }
    }
    let mut rng = Rng(cfg.order_seed);
    for i in (1..jobs.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        jobs.swap(i, j);
    }
    let next = AtomicUsize::new(0);
    let out = Mutex::new(io::stdout());
    let done = AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..cfg.threads {
            sc.spawn(|| {
                let mut buf = S3Buf { num1: vec![], num2: vec![], den: vec![], scratch: vec![] };
                loop {
                    let k = next.fetch_add(1, Ordering::SeqCst);
                    if k >= jobs.len() {
                        break;
                    }
                    let (ai, ti) = jobs[k];
                    let arm = &arms[ai];
                    let f = &arm.curve.f;
                    let (r, planted) = target(arm, ti);
                    let cr = coord_of(arm, r.0);
                    take(&MULS);
                    take(&JACS);
                    take(&SQRTS);
                    take(&INVS);
                    let st = Instant::now();
                    let mut y_rf: Option<u64> = None;
                    let mut y4: Option<u64> = None;
                    if arm.root != Root::None {
                        let cr = cr.unwrap();
                        if cfg.mode == "s3" {
                            y_rf = Some(s3_count(arm, cr, cfg.bw, &mut buf));
                        } else {
                            // S4 by chaining: roots of S3(x1, cr, X) are x(R -+ P1)
                            let mut tot = 0u64;
                            let mut inner: Vec<u64> = vec![];
                            for i in 0..arm.bc.len() {
                                let x1 = arm.bc[i];
                                let (a, b) = (arm.curve.a, arm.curve.b);
                                let s = f.add(x1, cr);
                                let pr = f.mul(x1, cr);
                                let d = f.sub(x1, cr);
                                let c2 = f.mul(d, d);
                                let two = f.add(f.one, f.one);
                                let four = f.add(two, two);
                                let inner_t = f.add(f.mul(s, f.add(pr, a)), f.add(b, b));
                                let c1 = f.neg(f.add(inner_t, inner_t));
                                let t = f.sub(pr, a);
                                let c0 = f.sub(f.mul(t, t), f.mul(four, f.mul(b, s)));
                                inner.clear();
                                if c2 == 0 {
                                    if c1 != 0 {
                                        inner.push(f.mul(f.neg(c0), f.inv(c1)));
                                    }
                                } else {
                                    let disc = f.sub(f.mul(c1, c1), f.mul(four, f.mul(c2, c0)));
                                    let sq = f.sqrt(disc).expect("R+-P1 rational");
                                    let di = f.inv(f.add(c2, c2));
                                    inner.push(f.mul(f.add(f.neg(c1), sq), di));
                                    inner.push(f.mul(f.sub(f.neg(c1), sq), di));
                                }
                                for &xp in inner.iter() {
                                    tot += s3_count(arm, xp, cfg.bw, &mut buf);
                                }
                            }
                            y4 = Some(tot);
                        }
                    }
                    let ns = st.elapsed().as_nanos();
                    let (m, jc, sq, iv) = (take(&MULS), take(&JACS), take(&SQRTS), take(&INVS));
                    let mut y_gl: Option<u64> = None;
                    if cfg.mode == "s3" && (ti < cfg.xcheck || arm.root == Root::None) {
                        y_gl = Some(gl_count(arm, r, cfg.bw));
                        take(&MULS);
                        take(&JACS);
                        take(&SQRTS);
                        take(&INVS);
                    }
                    let o = |v: Option<u64>| v.map(|x| x.to_string()).unwrap_or("null".into());
                    let line = format!(
                        "{{\"kind\":\"target\",\"cell\":\"{}\",\"arm\":\"{}\",\"t\":{},\"planted\":{},\"y_rf\":{},\"y_gl\":{},\"y4\":{},\"fx\":{},\"fpts\":{},\"muls\":{},\"jacobi\":{},\"sqrt\":{},\"inv\":{},\"ns\":{},\"job\":{}}}",
                        cfg.name, arm.name, ti, planted, o(y_rf), o(y_gl), o(y4), arm.bc.len(), arm.fpts, m, jc, sq, iv, ns, k
                    );
                    let mut w = out.lock().unwrap();
                    writeln!(w, "{}", line).unwrap();
                    let d = done.fetch_add(1, Ordering::SeqCst) + 1;
                    if d % 200 == 0 {
                        eprintln!("{} / {} jobs ({:.0}s)", d, jobs.len(), t0.elapsed().as_secs_f64());
                    }
                }
            });
        }
    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("find") => cmd_find(&args[2..]),
        Some("run") => cmd_run(),
        Some("setup") => setup::cmd_setup(&args[2..]),
        Some("verify") => verify::cmd_verify(&args[2..]),
        Some("analyze") => analyze::cmd_analyze(&args[2..]),
        Some("card") => {
            let p: u64 = args[2].parse().unwrap();
            let mut rng = Rng(7);
            println!("{:?}", nt::card(p, args[3].parse().unwrap(), args[4].parse().unwrap(), &mut rng));
        }
        _ => {
            eprintln!("usage: cmsem find <mode> <p> <a0> <b0> <n> <K> <arg> | cmsem run < cell.txt");
            std::process::exit(2);
        }
    }
}
