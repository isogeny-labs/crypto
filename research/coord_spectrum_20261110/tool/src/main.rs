//! F3: Fourier spectrum and additive energy of coordinate functions along <G>.
//! Standalone, std-only tool implementing PROTOCOL.md (frozen 2026-11-10).
//! Arithmetic ported from research/p2_lift_linear_complexity_20261110/tool (SplitMix64/FNV-1a
//! seeding, curve search, Hensel lift and formal-group machinery, JSON writer, SHA-256).

use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// ------------------------------------------------------------------ utils

fn modinv(a: u128, m: u128) -> Option<u128> {
    // extended Euclid in i64 (m < 2^62)
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (m as i64, (a % m) as i64);
    while nr != 0 {
        let q = r / nr;
        let x = t - q * nt;
        t = nt;
        nt = x;
        let y = r - q * nr;
        r = nr;
        nr = y;
    }
    if r != 1 {
        return None;
    }
    if t < 0 {
        t += m as i64;
    }
    Some(t as u128)
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * b as u128) % m as u128) as u64;
        }
        b = ((b as u128 * b as u128) % m as u128) as u64;
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}


// FNV-1a seeded SplitMix64
struct Rng(u64);
impl Rng {
    fn from_seed_str(s: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in s.bytes() {
            h ^= b as u64;
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
}

// SHA-256
fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(t1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = t1.wrapping_add(t2);
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut s = String::new();
    for x in h {
        write!(s, "{:08x}", x).unwrap();
    }
    s
}



// ------------------------------------------------------------------ JSON

#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Int(i128),
    F(f64),
    S(String),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

fn jo(items: Vec<(&str, J)>) -> J {
    J::O(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn ji<T: Into<i128>>(x: T) -> J {
    J::Int(x.into())
}
fn js(s: &str) -> J {
    J::S(s.to_string())
}

impl J {
    fn scalar(&self) -> bool {
        !matches!(self, J::A(_) | J::O(_))
    }
    fn render(&self, out: &mut String, ind: usize) {
        match self {
            J::Null => out.push_str("null"),
            J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            J::Int(i) => write!(out, "{}", i).unwrap(),
            J::F(f) => {
                if f.is_finite() {
                    write!(out, "{:e}", f).unwrap()
                } else {
                    out.push_str("null")
                }
            }
            J::S(s) => {
                out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            J::A(v) => {
                if v.iter().all(|x| x.scalar()) {
                    out.push('[');
                    for (i, x) in v.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        x.render(out, ind);
                    }
                    out.push(']');
                } else {
                    out.push_str("[\n");
                    for (i, x) in v.iter().enumerate() {
                        out.push_str(&" ".repeat(ind + 2));
                        x.render(out, ind + 2);
                        if i + 1 < v.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    out.push_str(&" ".repeat(ind));
                    out.push(']');
                }
            }
            J::O(v) => {
                out.push_str("{\n");
                for (i, (k, x)) in v.iter().enumerate() {
                    out.push_str(&" ".repeat(ind + 2));
                    J::S(k.clone()).render(out, ind);
                    out.push_str(": ");
                    x.render(out, ind + 2);
                    if i + 1 < v.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                out.push_str(&" ".repeat(ind));
                out.push('}');
            }
        }
    }
}

// ------------------------------------------------------------------ curve

type Pt = Option<(u64, u64)>;
type LPt = Option<(u128, u128)>;

#[derive(Clone, Debug)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    r: u64, // order of the generator (prime, != p)
    name: String,
}
#[derive(Default, Clone, Debug)]
struct Counts {
    lifts: u64,
    inv_mod_p2: u64,
    inv_mod_p: u64,
    affine_p_adds: u64,
    lifted_adds: u64,
    lifted_doublings: u64,
    kernel_mixed_adds: u64,
    formal_extractions: u64,
}

impl Counts {
    fn merge(&mut self, o: &Counts) {
        self.lifts += o.lifts;
        self.inv_mod_p2 += o.inv_mod_p2;
        self.inv_mod_p += o.inv_mod_p;
        self.affine_p_adds += o.affine_p_adds;
        self.lifted_adds += o.lifted_adds;
        self.lifted_doublings += o.lifted_doublings;
        self.kernel_mixed_adds += o.kernel_mixed_adds;
        self.formal_extractions += o.formal_extractions;
    }
    fn json(&self) -> J {
        jo(vec![
            ("hensel_lifts", ji(self.lifts)),
            ("inversions_mod_p2", ji(self.inv_mod_p2)),
            ("inversions_mod_p", ji(self.inv_mod_p)),
            ("affine_mod_p_adds", ji(self.affine_p_adds)),
            ("lifted_adds", ji(self.lifted_adds)),
            ("lifted_doublings", ji(self.lifted_doublings)),
            ("kernel_mixed_adds", ji(self.kernel_mixed_adds)),
            ("formal_extractions", ji(self.formal_extractions)),
        ])
    }
}


impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (x * x % p * x + self.a * x + self.b) % p
    }
    fn on_curve(&self, pt: Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => (y * y) % self.p == self.rhs(x),
        }
    }
    fn add(&self, l: Pt, r: Pt, cnt: &mut Counts) -> Result<Pt, String> {
        let p = self.p;
        let (a, b) = match (l, r) {
            (None, r) => return Ok(r),
            (l, None) => return Ok(l),
            (Some(a), Some(b)) => (a, b),
        };
        let (x1, y1) = a;
        let (x2, y2) = b;
        if x1 == x2 && (y1 + y2) % p == 0 {
            return Ok(None);
        }
        cnt.affine_p_adds += 1;
        cnt.inv_mod_p += 1;
        let slope = if a == b {
            (3 * x1 * x1 + self.a) % p * modinv((2 * y1) as u128, p as u128).ok_or("inv2y")? as u64 % p
        } else {
            (y2 + p - y1) % p * modinv(((x2 + p - x1) % p) as u128, p as u128).ok_or("invdx")? as u64 % p
        };
        let x3 = (slope * slope % p + 2 * p - x1 - x2) % p;
        let y3 = (slope * ((x1 + p - x3) % p) % p + p - y1) % p;
        let out = Some((x3, y3));
        if !self.on_curve(out) {
            return Err("base-field group addition left the curve".into());
        }
        Ok(out)
    }
    fn mul(&self, mut s: u64, pt: Pt) -> Result<Pt, String> {
        let mut c = Counts::default();
        let mut out = None;
        let mut addend = pt;
        while s > 0 {
            if s & 1 == 1 {
                out = self.add(out, addend, &mut c)?;
            }
            addend = self.add(addend, addend, &mut c)?;
            s >>= 1;
        }
        Ok(out)
    }
    /// sqrt table: smallest y in 1..p with y^2 = v (0 = none)
    fn sqrt_table(&self) -> Vec<u32> {
        let p = self.p;
        let mut t = vec![0u32; p as usize];
        for y in 1..p {
            let v = (y * y % p) as usize;
            if t[v] == 0 {
                t[v] = y as u32;
            }
        }
        t
    }
    /// H110 first_point: smallest x, smallest y>=1, with [r]P = O.
    fn first_point(&self) -> Result<(u64, u64), String> {
        let sq = self.sqrt_table();
        for x in 0..self.p {
            let y = sq[self.rhs(x) as usize] as u64;
            if y != 0 {
                let pt = Some((x, y));
                if self.mul(self.r, pt)?.is_none() {
                    return Ok((x, y));
                }
            }
        }
        Err(format!("no order-{} point on {}", self.r, self.name))
    }
    fn point_count(&self) -> u64 {
        let sq = self.sqrt_table();
        let mut n = 1u64;
        for x in 0..self.p {
            let v = self.rhs(x);
            if v == 0 {
                n += 1;
            } else if sq[v as usize] != 0 {
                n += 2;
            }
        }
        n
    }
}

// ------------------------------------------------------- lifted arithmetic

fn on_lifted(c: &Curve, pt: (u128, u128)) -> bool {
    let n = (c.p as u128) * (c.p as u128);
    let (x, y) = pt;
    let lhs = y * y % n;
    let x3 = x * x % n * x % n;
    let rhs = (x3 + (c.a as u128) * x % n + c.b as u128) % n;
    lhs == rhs
}

fn hensel_lift(c: &Curve, pt: (u64, u64), u: u64, cnt: &mut Counts) -> Result<(u128, u128), String> {
    let p = c.p as u128;
    let n = p * p;
    let (x, y) = (pt.0 as i128, pt.1 as i128);
    if pt.1 % c.p == 0 {
        return Err("nonidentity point has y=0".into());
    }
    let disc = x * x * x + (c.a as i128) * x + c.b as i128 - y * y;
    if disc.rem_euclid(p as i128) != 0 {
        return Err("base point is not on the reduced curve".into());
    }
    let cq = (disc.div_euclid(p as i128)).rem_euclid(p as i128) as u128;
    let xu = pt.0 as u128;
    let yu = pt.1 as u128;
    let tangent = (3 * xu * xu + c.a as u128) % p;
    let num = (cq + tangent * (u as u128 % p)) % p;
    let inv = modinv(2 * yu % p, p).ok_or("inv 2y mod p")?;
    cnt.inv_mod_p += 1;
    let v = num * inv % p;
    cnt.lifts += 1;
    let lifted = ((xu + p * (u as u128 % p)) % n, (yu + p * v) % n);
    if !on_lifted(c, lifted) {
        return Err("Hensel lift missed the lifted curve".into());
    }
    Ok(lifted)
}

fn sub_n(a: u128, b: u128, n: u128) -> u128 {
    (a + n - b) % n
}

fn affine_add_lifted(c: &Curve, l: LPt, r: LPt, cnt: &mut Counts) -> Result<LPt, String> {
    let (a, b) = match (l, r) {
        (None, r) => return Ok(r),
        (l, None) => return Ok(l),
        (Some(a), Some(b)) => (a, b),
    };
    let p = c.p as u128;
    let n = p * p;
    let (x1, y1) = a;
    let (x2, y2) = b;
    let (num, den);
    if a == b {
        den = 2 * y1 % n;
        if den % p == 0 {
            return Err("nonunit doubling denominator".into());
        }
        num = (3 * (x1 * x1 % n) + c.a as u128) % n;
        cnt.lifted_doublings += 1;
    } else {
        den = sub_n(x2, x1, n);
        if den % p == 0 {
            return Err("nonunit addition denominator".into());
        }
        num = sub_n(y2, y1, n);
        cnt.lifted_adds += 1;
    }
    cnt.inv_mod_p2 += 1;
    let slope = num * modinv(den, n).ok_or("inv mod p^2")? % n;
    let x3 = sub_n(sub_n(slope * slope % n, x1, n), x2, n);
    let y3 = sub_n(slope * sub_n(x1, x3, n) % n, y1, n);
    if !on_lifted(c, (x3, y3)) {
        return Err("affine lifted addition left the curve".into());
    }
    Ok(Some((x3, y3)))
}

fn affine_mul_lifted(c: &Curve, mut s: u64, pt: (u128, u128), cnt: &mut Counts) -> Result<LPt, String> {
    let mut out: LPt = None;
    let mut addend: LPt = Some(pt);
    while s > 0 {
        if s & 1 == 1 {
            out = affine_add_lifted(c, out, addend, cnt)?;
        }
        s >>= 1;
        if s > 0 {
            addend = affine_add_lifted(c, addend, addend, cnt)?;
        }
    }
    Ok(out)
}

fn mixed_kernel_sum(c: &Curve, l: (u128, u128), r: (u128, u128), cnt: &mut Counts) -> Result<(u128, u128, u128), String> {
    let n = (c.p as u128) * (c.p as u128);
    let (x1, y1) = l;
    let (x2, y2) = r;
    let h = sub_n(x2, x1, n);
    let two_h = 2 * h % n;
    let i = two_h * two_h % n;
    let j = h * i % n;
    let rr = 2 * sub_n(y2, y1, n) % n;
    let v = x1 * i % n;
    let x3 = sub_n(sub_n(rr * rr % n, j, n), 2 * v % n, n);
    let y3 = sub_n(rr * sub_n(v, x3, n) % n, 2 * (y1 * j % n) % n, n);
    let z3 = two_h;
    cnt.kernel_mixed_adds += 1;
    // projective check y^2 = x^3 + a x z^4 + b z^6
    let z2 = z3 * z3 % n;
    let z4 = z2 * z2 % n;
    let z6 = z4 * z2 % n;
    let rhs = (x3 * x3 % n * x3 % n + (c.a as u128) * (x3 * z4 % n) % n + (c.b as u128) * z6 % n) % n;
    if y3 * y3 % n != rhs {
        return Err("mixed kernel sum left the projective curve".into());
    }
    Ok((x3, y3, z3))
}

fn r_multiple_kernel(c: &Curve, lifted: (u128, u128), split: u64, cnt: &mut Counts) -> Result<(u128, u128, u128), String> {
    if !(1 <= split && split < c.r) {
        return Err("invalid split".into());
    }
    let left = affine_mul_lifted(c, split, lifted, cnt)?;
    let right = affine_mul_lifted(c, c.r - split, lifted, cnt)?;
    let (left, right) = match (left, right) {
        (Some(l), Some(r)) => (l, r),
        _ => return Err("proper split unexpectedly reached identity".into()),
    };
    let p = c.p as u128;
    let lr = Some(((left.0 % p) as u64, (left.1 % p) as u64));
    let rr = Some(((right.0 % p) as u64, (right.1 % p) as u64));
    if c.add(lr, rr, cnt)?.is_some() {
        return Err("split reductions do not sum to identity".into());
    }
    mixed_kernel_sum(c, left, right, cnt)
}

fn formal_tau(c: &Curve, k: (u128, u128, u128), cnt: &mut Counts) -> Result<u64, String> {
    let p = c.p as u128;
    let n = p * p;
    let (x, y, z) = k;
    if z % p != 0 {
        return Err("[r] lift is not in the reduction kernel".into());
    }
    if y % p == 0 {
        return Err("projective Y is not a unit".into());
    }
    cnt.inv_mod_p2 += 1;
    let t = ((n - x * z % n) % n) * modinv(y, n).ok_or("inv Y")? % n;
    if t % p != 0 {
        return Err("formal parameter is not divisible by p".into());
    }
    cnt.formal_extractions += 1;
    Ok((t / p % p) as u64)
}

fn tau_for_section(c: &Curve, pt: (u64, u64), u: u64, split: u64, cnt: &mut Counts) -> Result<u64, String> {
    let lifted = hensel_lift(c, pt, u, cnt)?;
    let k = r_multiple_kernel(c, lifted, split, cnt)?;
    formal_tau(c, k, cnt)
}

// -------------------------------------------------------------- parallel

fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= items.len() {
                    break;
                }
                let r = f(&items[i]);
                out.lock().unwrap()[i] = Some(r);
            });
        }
    });
    out.into_inner().unwrap().into_iter().map(|x| x.unwrap()).collect()
}

// ------------------------------------------------------------ complex FFT

#[derive(Clone, Copy, Debug, Default)]
struct C {
    re: f64,
    im: f64,
}

impl C {
    fn mul(self, o: C) -> C {
        C { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
    fn add(self, o: C) -> C {
        C { re: self.re + o.re, im: self.im + o.im }
    }
    fn sub(self, o: C) -> C {
        C { re: self.re - o.re, im: self.im - o.im }
    }
    fn conj(self) -> C {
        C { re: self.re, im: -self.im }
    }
    fn scale(self, s: f64) -> C {
        C { re: self.re * s, im: self.im * s }
    }
    fn abs2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

fn cis(theta: f64) -> C {
    let (s, c) = theta.sin_cos();
    C { re: c, im: s }
}

/// tw[k] = exp(-2 pi i k / n), k < n/2.
fn twiddles(n: usize) -> Vec<C> {
    (0..n / 2).map(|k| cis(-2.0 * std::f64::consts::PI * (k as f64) / (n as f64))).collect()
}

/// In-place radix-2 FFT, forward kernel exp(-2 pi i jk/n), or its conjugate when `inverse`
/// (unscaled).
fn fft(a: &mut [C], tw: &[C], inverse: bool) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let step = n / len;
        for start in (0..n).step_by(len) {
            for k in 0..half {
                let mut w = tw[k * step];
                if inverse {
                    w = w.conj();
                }
                let u = a[start + k];
                let v = a[start + k + half].mul(w);
                a[start + k] = u.add(v);
                a[start + k + half] = u.sub(v);
            }
        }
        len <<= 1;
    }
}

/// Bluestein chirp-z transform of prime (any) length r over a power-of-two FFT.
struct Bluestein {
    r: usize,
    n: usize,
    tw: Vec<C>,
    chirp: Vec<C>,
    hf: Vec<C>,
}

impl Bluestein {
    fn new(r: usize) -> Bluestein {
        let n = (2 * r - 1).next_power_of_two().max(2);
        let tw = twiddles(n);
        let chirp: Vec<C> = (0..r)
            .map(|k| {
                let m = ((k as u64 * k as u64) % (2 * r as u64)) as f64;
                cis(-std::f64::consts::PI * m / (r as f64))
            })
            .collect();
        let mut hf = vec![C::default(); n];
        for m in 0..r {
            let c = chirp[m].conj();
            hf[m] = c;
            if m > 0 {
                hf[n - m] = c;
            }
        }
        fft(&mut hf, &tw, false);
        Bluestein { r, n, tw, chirp, hf }
    }
    /// butterflies of one transform (two length-n FFTs)
    fn butterflies(&self) -> u64 {
        2 * (self.n as u64 / 2) * (self.n.trailing_zeros() as u64)
    }
    /// `buf` has length n; buf[..r] holds z. Afterwards buf[..r][b] = sum_k z_k exp(-2 pi i b k / r).
    fn transform(&self, buf: &mut [C]) {
        let (r, n) = (self.r, self.n);
        assert_eq!(buf.len(), n);
        for k in 0..r {
            buf[k] = buf[k].mul(self.chirp[k]);
        }
        for x in buf[r..].iter_mut() {
            *x = C::default();
        }
        fft(buf, &self.tw, false);
        for i in 0..n {
            buf[i] = buf[i].mul(self.hf[i]);
        }
        fft(buf, &self.tw, true);
        let s = 1.0 / n as f64;
        for b in 0..r {
            buf[b] = buf[b].mul(self.chirp[b]).scale(s);
        }
    }
}

/// Neumaier compensated accumulator
#[derive(Default, Clone, Copy)]
struct Acc {
    s: f64,
    c: f64,
}
impl Acc {
    fn add(&mut self, x: f64) {
        let t = self.s + x;
        if self.s.abs() >= x.abs() {
            self.c += (self.s - t) + x;
        } else {
            self.c += (x - t) + self.s;
        }
        self.s = t;
    }
    fn val(&self) -> f64 {
        self.s + self.c
    }
}

/// exp(2 pi i m / p) for m < p
fn phase_table(p: u64) -> Vec<C> {
    (0..p).map(|m| cis(2.0 * std::f64::consts::PI * (m as f64) / (p as f64))).collect()
}

struct SpecResult {
    eta: f64, // gated statistic: max over a and b != 0
    a: u64,
    b: u64,
    eta_b0: f64, // secondary: max over a of |S(a,0)|/r
    parseval_dev: f64,
    naive_dev: f64,
    transforms: u64,
}

/// Core: for each a in alist, z = fill(a) (length r), S(a,b) = sum_k z_k e(b k / r).
/// eta = max over a, b != 0 of |S|/r; eta_b0 = max over a of |S(a,0)|/r.
fn spectrum_gen(bl: &Bluestein, alist: &[u64], fill: &dyn Fn(u64, &mut [C])) -> SpecResult {
    let r = bl.r;
    let mut buf = vec![C::default(); bl.n];
    let mut best_sq = -1.0f64;
    let mut best_a = 0u64;
    let mut best_bp = 1usize;
    let mut best_val = C::default();
    let mut b0_sq = 0.0f64;
    let mut pdev = 0f64;
    for &a in alist {
        fill(a, &mut buf[..r]);
        let mut zs = Acc::default();
        for k in 0..r {
            zs.add(buf[k].abs2());
        }
        let r2 = (r as f64) * zs.val(); // Parseval: sum |S|^2 = r sum |z|^2 (= r^2 when |z| = 1)
        bl.transform(&mut buf);
        let mut acc = Acc::default();
        for b in 0..r {
            let s = buf[b].abs2();
            acc.add(s);
            if b == 0 {
                b0_sq = b0_sq.max(s);
            } else if s > best_sq {
                best_sq = s;
                best_a = a;
                best_bp = b;
                best_val = buf[b];
            }
        }
        pdev = pdev.max((acc.val() - r2).abs() / r2);
    }
    // S(a,b) = X_{-b}: our transform used exp(-2 pi i ...), so b = -b' mod r.
    let b = ((r - best_bp) % r) as u64;
    let mut z = vec![C::default(); r];
    fill(best_a, &mut z);
    let (mut sre, mut sim) = (Acc::default(), Acc::default());
    for k in 0..r {
        let m = ((b * k as u64) % r as u64) as f64;
        let t = z[k].mul(cis(2.0 * std::f64::consts::PI * m / (r as f64)));
        sre.add(t.re);
        sim.add(t.im);
    }
    let d = C { re: sre.val() - best_val.re, im: sim.val() - best_val.im };
    SpecResult {
        eta: best_sq.max(0.0).sqrt() / r as f64,
        a: best_a,
        b,
        eta_b0: b0_sq.sqrt() / r as f64,
        parseval_dev: pdev,
        naive_dev: d.abs2().sqrt() / r as f64,
        transforms: alist.len() as u64,
    }
}

/// F_p-valued g: z_k = e(a g(k)/p).
fn spectrum_eta(g: &[u32], p: u64, bl: &Bluestein, phase: &[C], alist: &[u64]) -> SpecResult {
    assert_eq!(g.len(), bl.r);
    spectrum_gen(bl, alist, &|a, out| {
        for k in 0..out.len() {
            out[k] = phase[((a * g[k] as u64) % p) as usize];
        }
    })
}

/// +-1/0-valued sequence used directly as complex values (no a loop; a reported as 0).
fn spectrum_pm(v: &[i8], bl: &Bluestein) -> SpecResult {
    assert_eq!(v.len(), bl.r);
    spectrum_gen(bl, &[0], &|_, out| {
        for k in 0..out.len() {
            out[k] = C { re: v[k] as f64, im: 0.0 };
        }
    })
}

// ----------------------------------------------------------------- energy

/// E(A) = #{(a,b,c,d) in A^4 : a+b = c+d mod p} = sum_s N(s)^2, N(s) = #ordered pairs summing to s.
/// Elements must lie in [0,p). `cnt` is scratch.
fn additive_energy(a: &[u32], p: usize, cnt: &mut Vec<u32>) -> u64 {
    cnt.clear();
    cnt.resize(p, 0);
    let n = a.len();
    for i in 0..n {
        let ai = a[i] as usize;
        let mut s = 2 * ai;
        if s >= p {
            s -= p;
        }
        cnt[s] += 1;
        for j in i + 1..n {
            let mut s = ai + a[j] as usize;
            if s >= p {
                s -= p;
            }
            cnt[s] += 2;
        }
    }
    cnt.iter().map(|&c| (c as u64) * (c as u64)).sum()
}

fn random_subset(rng: &mut Rng, p: u64, k: usize) -> Vec<u32> {
    let mut seen = vec![false; p as usize];
    let mut v = Vec::with_capacity(k);
    while v.len() < k {
        let x = (rng.next() % p) as usize;
        if !seen[x] {
            seen[x] = true;
            v.push(x as u32);
        }
    }
    v
}

// ------------------------------------------------------------ curve sets

struct Spec {
    name: String,
    set: &'static str,
    curve: Curve,
    gen: (u64, u64),
    extra: Vec<(String, J)>,
}

fn prime_below(limit: u64) -> u64 {
    let mut q = limit - 1;
    while !is_prime(q) {
        q -= 1;
    }
    q
}

fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn r_specs(bits: u32, count: usize) -> Result<Vec<Spec>, String> {
    let p = prime_below(1u64 << bits);
    let mut v = vec![];
    for i in 0..count {
        let mut rng = Rng::from_seed_str(&format!("F3/curve/{}/{}", bits, i));
        let mut tries = 0u64;
        let (a, b, n) = loop {
            tries += 1;
            if tries > 100000 {
                return Err("curve search exhausted".into());
            }
            let a = rng.next() % p;
            let b = rng.next() % p;
            let disc = (4 * (a as u128).pow(3) + 27 * (b as u128).pow(2)) % p as u128;
            if disc == 0 || a == 0 || b == 0 {
                continue; // singular, or j in {0, 1728}
            }
            let c = Curve { p, a, b, r: 0, name: String::new() };
            let n = c.point_count();
            if n != p && is_prime(n) {
                break (a, b, n);
            }
        };
        let name = format!("R{}_{}", bits, i);
        let curve = Curve { p, a, b, r: n, name: name.clone() };
        let gen = curve.first_point()?;
        v.push(Spec {
            name,
            set: if bits == 16 { "R16" } else { "R20" },
            curve,
            gen,
            extra: vec![
                ("candidates_tried".into(), ji(tries as i64)),
                ("order_method".into(), js("exhaustive Legendre count")),
            ],
        });
    }
    Ok(v)
}

/// Anomalous curve by CM with D = -11: 4p - 1 = 11 v^2, j = -32768, the twist with p points.
fn anom_spec(i: usize, bits: u32) -> Result<Spec, String> {
    let t = 1u64 << bits;
    let mut rng = Rng::from_seed_str(&format!("F3/anom/{}", i));
    let v_hi = isqrt((4 * t - 1) / 11);
    let v_lo = isqrt((4 * (t * 9 / 10) - 1) / 11);
    let mut v = v_lo + rng.next() % (v_hi - v_lo + 1);
    if v % 2 == 0 {
        v += 1;
    }
    let p = loop {
        let p = (11 * v * v + 1) / 4;
        if p > 40000 && is_prime(p) {
            break p;
        }
        v += 2;
    };
    assert_eq!(4 * p - 1, 11 * v * v);
    let jm = (p - 32768 % p) % p;
    let j1728 = (jm + p - 1728) % p;
    let tt = jm * j1728 % p;
    let a0 = (p - 3 * tt % p) % p;
    let b0 = 2 * jm % p * (j1728 * j1728 % p) % p;
    let mut d = 2u64;
    while powmod(d, (p - 1) / 2, p) != p - 1 {
        d += 1;
    }
    let a1 = a0 * (d * d % p) % p;
    let b1 = b0 * (d * d % p * d % p) % p;
    let mut chosen = None;
    let mut counts = vec![];
    for (twisted, (a, b)) in [(false, (a0, b0)), (true, (a1, b1))] {
        let c = Curve { p, a, b, r: p, name: String::new() };
        let n = c.point_count();
        counts.push(ji(n as i64));
        if n == p && chosen.is_none() {
            chosen = Some((twisted, a, b));
        }
    }
    let (twisted, a, b) = chosen.ok_or("neither the curve nor its twist has p points")?;
    let name = format!("ANOM_{}", bits);
    let curve = Curve { p, a, b, r: p, name: name.clone() };
    // extra verification: [p]P = O for seeded random points
    let sq = curve.sqrt_table();
    let mut checked = 0;
    while checked < 5 {
        let x = rng.next() % p;
        let y = sq[curve.rhs(x) as usize] as u64;
        if y == 0 {
            continue;
        }
        let y = if rng.next() & 1 == 1 { p - y } else { y };
        if curve.mul(p, Some((x, y)))?.is_some() {
            return Err("[p]P != O on anomalous candidate".into());
        }
        checked += 1;
    }
    let gen = curve.first_point()?;
    Ok(Spec {
        name,
        set: "ANOM",
        curve,
        gen,
        extra: vec![
            ("cm_v".into(), ji(v as i64)),
            ("cm_discriminant".into(), ji(-11)),
            ("j_invariant".into(), ji(-32768)),
            ("twisted".into(), J::Bool(twisted)),
            ("twist_nonresidue_d".into(), ji(d as i64)),
            ("exhaustive_counts_curve_then_twist".into(), J::A(counts)),
            ("random_points_with_pP_eq_O".into(), ji(checked)),
            ("order_method".into(), js("exhaustive Legendre count + [p]P=O on 5 seeded points")),
        ],
    })
}

// ------------------------------------------------------------- analysis

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    X,
    Y,
    Chi(u64),
    Dickson,
    Lambda,
    Plant,
    Null(usize),
    NullPm(usize),
    NullEven(usize),
    NullOdd(usize),
    NullEvenPm(usize),
}

/// Null classes: 0 uniform F_p, 1 uniform +-1, 2 even F_p, 3 odd F_p, 4 even +-1.
fn null_class(k: &Kind) -> Option<usize> {
    match k {
        Kind::Null(_) => Some(0),
        Kind::NullPm(_) => Some(1),
        Kind::NullEven(_) => Some(2),
        Kind::NullOdd(_) => Some(3),
        Kind::NullEvenPm(_) => Some(4),
        _ => None,
    }
}

const NULL_CLASS_NAMES: [&str; 5] = ["uniform_fp", "uniform_pm1", "even_fp", "odd_fp", "even_pm1"];

/// symmetry-class null that a function's ratio is taken against
fn ratio_class(k: &Kind) -> usize {
    match k {
        Kind::X | Kind::Dickson => 2,
        Kind::Y | Kind::Lambda => 3,
        Kind::Chi(_) => 4,
        _ => 0,
    }
}

/// Null sequences of Z/r with the negation symmetry of the coordinate map; g(0) = 0.
/// class 2: g_k = g_{r-k} (random F_p per class); 3: g_{r-k} = -g_k mod p; 4: even +-1.
fn symmetric_null(class: usize, p: u64, r: usize, rng: &mut Rng) -> Vec<i64> {
    let mut g = vec![0i64; r];
    for k in 1..=(r - 1) / 2 {
        let (a, b) = match class {
            2 => {
                let v = (rng.next() % p) as i64;
                (v, v)
            }
            3 => {
                let v = (rng.next() % p) as i64;
                (v, (p as i64 - v) % p as i64)
            }
            _ => {
                let v = if rng.next() & 1 == 1 { 1 } else { -1 };
                (v, v)
            }
        };
        g[k] = a;
        g[r - k] = b;
    }
    g
}

impl Kind {
    fn name(&self) -> String {
        match self {
            Kind::X => "x".into(),
            Kind::Y => "y".into(),
            Kind::Chi(c) => format!("chi(x+{})", c),
            Kind::Dickson => "dickson".into(),
            Kind::Lambda => "lambda".into(),
            Kind::Plant => "PLANT".into(),
            Kind::Null(j) => format!("NULL_{}", j),
            Kind::NullPm(j) => format!("NULLPM_{}", j),
            Kind::NullEven(j) => format!("NULLEVEN_{}", j),
            Kind::NullOdd(j) => format!("NULLODD_{}", j),
            Kind::NullEvenPm(j) => format!("NULLEVENPM_{}", j),
        }
    }
}

struct FnRow {
    kind: Kind,
    res: SpecResult,
}

struct EnergyRow {
    k: usize,
    distinct: usize,
    pair_sums: u64,
    e_a: u64,
    null_mean: f64,
    null_sd: f64,
    ratio: f64,
    z: Option<f64>,
    band: f64, // 3 sd / mean
    in_band: bool,
}

struct CurveRes {
    name: String,
    set: &'static str,
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    gen: (u64, u64),
    extra: Vec<(String, J)>,
    dickson_deg: usize,
    rows: Vec<FnRow>,
    null_medians: [f64; 5],
    energy: Vec<EnergyRow>,
    alist: Vec<u64>,
    group_counts: Counts,
    lambda_counts: Option<Counts>,
    lambda_linear_violations: Option<u64>,
    fft_n: usize,
    fft_butterflies_per_transform: u64,
}

impl CurveRes {
    /// eta over the matching null: +-1 sequences for chi rows, F_p-valued functions otherwise
    fn ratio(&self, row: &FnRow) -> f64 {
        row.res.eta / self.null_medians[ratio_class(&row.kind)]
    }
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        0.5 * (s[n / 2 - 1] + s[n / 2])
    }
}

fn dickson_degree(r: u64) -> usize {
    (r as f64).powf(0.25).ceil() as usize
}

fn lambda_orbit(c: &Curve, xs: &[u32], ys: &[u32], threads: usize) -> Result<(Vec<u32>, Counts), String> {
    if c.r != c.p {
        return Err("lambda needs an anomalous curve".into());
    }
    let r = xs.len(); // == c.r in a run; tests pass a prefix
    let half = (c.r - 1) / 2;
    let chunk = 4096usize;
    let starts: Vec<usize> = (1..r).step_by(chunk).collect();
    let res = par_map(&starts, threads, |&s| {
        let e = (s + chunk).min(r);
        let mut cnt = Counts::default();
        let mut out = Vec::with_capacity(e - s);
        for k in s..e {
            match tau_for_section(c, (xs[k] as u64, ys[k] as u64), 0, half, &mut cnt) {
                Ok(t) => out.push(t as u32),
                Err(m) => return Err(format!("k={}: {}", k, m)),
            }
        }
        Ok((s, out, cnt))
    });
    let mut lam = vec![0u32; r];
    let mut cnt = Counts::default();
    for x in res {
        let (s, out, cc) = x?;
        lam[s..s + out.len()].copy_from_slice(&out);
        cnt.merge(&cc);
    }
    Ok((lam, cnt))
}

fn orbit(c: &Curve, g: (u64, u64)) -> Result<(Vec<u32>, Vec<u32>, Counts), String> {
    let r = c.r as usize;
    let mut cnt = Counts::default();
    let mut xs = vec![0u32; r];
    let mut ys = vec![0u32; r];
    let gp = Some(g);
    let mut cur: Pt = None;
    for k in 1..r {
        cur = c.add(cur, gp, &mut cnt)?;
        let (x, y) = cur.ok_or("orbit reached identity early")?;
        xs[k] = x as u32;
        ys[k] = y as u32;
    }
    if c.add(cur, gp, &mut cnt)?.is_some() {
        return Err("generator orbit failed (order != r)".into());
    }
    Ok((xs, ys, cnt))
}

fn analyze_curve(spec: &Spec, nulls: usize, threads: usize) -> Result<CurveRes, String> {
    let c = &spec.curve;
    let (p, r) = (c.p, c.r as usize);
    let (xs, ys, group_counts) = orbit(c, spec.gen)?;
    let sq = c.sqrt_table();
    let legendre = |v: u64| -> u32 {
        if v == 0 {
            0
        } else if sq[v as usize] != 0 {
            1
        } else {
            (p - 1) as u32
        }
    };
    let dn = dickson_degree(c.r);
    let bl = Bluestein::new(r);
    let phase = phase_table(p);
    // 64 values of a: a = 1 plus 63 seeded distinct nonzero
    let alist = {
        let mut rng = Rng::from_seed_str(&format!("F3/a/{}", c.name));
        let mut v = vec![1u64];
        while v.len() < 64 {
            let a = 1 + rng.next() % (p - 1);
            if !v.contains(&a) {
                v.push(a);
            }
        }
        v
    };
    let anom = spec.set == "ANOM";
    let mut lambda_vals: Option<Vec<u32>> = None;
    let mut lambda_counts = None;
    let mut lambda_viol = None;
    if anom {
        let (lam, cnt) = lambda_orbit(c, &xs, &ys, threads)?;
        let l1 = lam[1] as u64;
        if l1 == 0 {
            return Err("lambda(G) = 0: Smart lift degenerate".into());
        }
        let viol = (1..r).filter(|&k| lam[k] as u64 != (k as u64 * l1) % p).count() as u64;
        lambda_vals = Some(lam);
        lambda_counts = Some(cnt);
        lambda_viol = Some(viol);
    }
    let mut kinds = vec![Kind::X, Kind::Y, Kind::Chi(0), Kind::Chi(1), Kind::Chi(2), Kind::Dickson];
    if anom {
        kinds.push(Kind::Lambda);
    }
    kinds.push(Kind::Plant);
    for j in 0..nulls {
        kinds.push(Kind::Null(j));
    }
    for j in 0..nulls {
        kinds.push(Kind::NullPm(j));
    }
    for j in 0..nulls {
        kinds.push(Kind::NullEven(j));
    }
    for j in 0..nulls {
        kinds.push(Kind::NullOdd(j));
    }
    for j in 0..nulls {
        kinds.push(Kind::NullEvenPm(j));
    }
    let rows: Vec<FnRow> = par_map(&kinds, threads, |&kind| {
        let pm: Option<Vec<i8>> = match kind {
            Kind::Chi(cc) => Some(
                (0..r)
                    .map(|k| match (k == 0, legendre((xs[k] as u64 + cc) % p)) {
                        (true, _) | (_, 0) => 0i8,
                        (_, 1) => 1,
                        _ => -1,
                    })
                    .collect(),
            ),
            Kind::NullPm(j) => {
                let mut rng = Rng::from_seed_str(&format!("F3/nullpm/{}/{}", c.name, j));
                Some((0..r).map(|_| if rng.next() & 1 == 1 { 1i8 } else { -1 }).collect())
            }
            Kind::NullEvenPm(j) => {
                let mut rng = Rng::from_seed_str(&format!("F3/null/evenpm/{}/{}", c.name, j));
                Some(symmetric_null(4, p, r, &mut rng).into_iter().map(|v| v as i8).collect())
            }
            _ => None,
        };
        if let Some(v) = pm {
            return FnRow { kind, res: spectrum_pm(&v, &bl) };
        }
        let g: Vec<u32> = match kind {
            Kind::X => xs.clone(),
            Kind::Y => ys.clone(),
            Kind::Dickson => (0..r)
                .map(|k| {
                    if k == 0 {
                        return 0;
                    }
                    let x = xs[k] as u64;
                    let (mut d0, mut d1) = (2 % p, x);
                    for _ in 1..dn {
                        let d2 = (x * d1 % p + p - d0) % p;
                        d0 = d1;
                        d1 = d2;
                    }
                    d1 as u32
                })
                .collect(),
            Kind::Lambda => lambda_vals.clone().unwrap(),
            Kind::Plant => (0..r).map(|k| ((k as u128 * p as u128) / r as u128) as u32).collect(),
            Kind::Null(j) => {
                let mut rng = Rng::from_seed_str(&format!("F3/null/{}/{}", c.name, j));
                (0..r).map(|_| (rng.next() % p) as u32).collect()
            }
            Kind::NullEven(j) => {
                let mut rng = Rng::from_seed_str(&format!("F3/null/even/{}/{}", c.name, j));
                symmetric_null(2, p, r, &mut rng).into_iter().map(|v| v as u32).collect()
            }
            Kind::NullOdd(j) => {
                let mut rng = Rng::from_seed_str(&format!("F3/null/odd/{}/{}", c.name, j));
                symmetric_null(3, p, r, &mut rng).into_iter().map(|v| v as u32).collect()
            }
            Kind::Chi(_) | Kind::NullPm(_) | Kind::NullEvenPm(_) => unreachable!(),
        };
        FnRow { kind, res: spectrum_eta(&g, p, &bl, &phase, &alist) }
    });
    let mut null_medians = [f64::NAN; 5];
    for cl in 0..5 {
        let v: Vec<f64> = rows.iter().filter(|x| null_class(&x.kind) == Some(cl)).map(|x| x.res.eta).collect();
        if !v.is_empty() {
            null_medians[cl] = median(&v);
        }
    }
    // energy
    let mut energy = vec![];
    for kf in [(r as f64).sqrt().ceil(), (r as f64).powf(2.0 / 3.0).ceil()] {
        let k = kf as usize;
        let a: Vec<u32> = xs[1..=k].to_vec();
        let mut sorted = a.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let mut scratch = Vec::new();
        let e_a = additive_energy(&a, p as usize, &mut scratch);
        let js_: Vec<usize> = (0..50).collect();
        let es: Vec<f64> = par_map(&js_, threads, |&j| {
            let mut rng = Rng::from_seed_str(&format!("F3/energy/{}/{}", c.name, j));
            let s = random_subset(&mut rng, p, k);
            let mut scratch = Vec::new();
            additive_energy(&s, p as usize, &mut scratch) as f64
        });
        let n = es.len() as f64;
        let mean = es.iter().sum::<f64>() / n;
        let sd = (es.iter().map(|e| (e - mean) * (e - mean)).sum::<f64>() / (n - 1.0)).sqrt();
        let ratio = e_a as f64 / mean;
        let band = 3.0 * sd / mean;
        energy.push(EnergyRow {
            k,
            distinct: sorted.len(),
            pair_sums: (k as u64 * (k as u64 + 1) / 2) * 51,
            e_a,
            null_mean: mean,
            null_sd: sd,
            ratio,
            z: if sd > 0.0 { Some((e_a as f64 - mean) / sd) } else { None },
            band,
            in_band: (ratio - 1.0).abs() <= band,
        });
    }
    Ok(CurveRes {
        name: spec.name.clone(),
        set: spec.set,
        p,
        a: c.a,
        b: c.b,
        r: c.r,
        gen: spec.gen,
        extra: spec.extra.clone(),
        dickson_deg: dn,
        rows,
        null_medians,
        energy,
        alist,
        group_counts,
        lambda_counts,
        lambda_linear_violations: lambda_viol,
        fft_n: bl.n,
        fft_butterflies_per_transform: bl.butterflies(),
    })
}

// ------------------------------------------------------------------ CLI

struct Args {
    out: String,
    sets: Vec<String>,
    quick: bool,
    protocol: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        out: "results/run.json".into(),
        sets: ["R16", "R20", "ANOM"].iter().map(|s| s.to_string()).collect(),
        quick: false,
        protocol: None,
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out" => {
                i += 1;
                a.out = argv.get(i).ok_or("--out needs a value")?.clone();
            }
            "--sets" => {
                i += 1;
                a.sets = argv.get(i).ok_or("--sets needs a value")?.split(',').map(|s| s.to_string()).collect();
            }
            "--protocol" => {
                i += 1;
                a.protocol = Some(argv.get(i).ok_or("--protocol needs a value")?.clone());
            }
            "--quick" => a.quick = true,
            x => return Err(format!("unknown argument {}", x)),
        }
        i += 1;
    }
    if a.quick {
        a.sets = vec!["R16".into()];
    }
    for s in &a.sets {
        if !["R16", "R20", "ANOM", "ANOM16"].contains(&s.as_str()) {
            return Err(format!("unknown set {}", s));
        }
    }
    Ok(a)
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(64);
        }
    };
    std::process::exit(run(&args));
}

fn jf(x: f64) -> J {
    J::F(x)
}

fn run(args: &Args) -> i32 {
    let t0 = std::time::Instant::now();
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let (ncurves, nulls) = if args.quick { (2usize, 5usize) } else { (8, 20) };
    let proto_path = {
        let mut cands: Vec<String> = vec![];
        if let Some(p) = &args.protocol {
            cands.push(p.clone());
        }
        cands.push(format!("{}/../PROTOCOL.md", env!("CARGO_MANIFEST_DIR")));
        cands.push("../PROTOCOL.md".into());
        cands.push("PROTOCOL.md".into());
        cands.into_iter().find(|p| std::path::Path::new(p).exists())
    };
    let protocol_sha = match &proto_path {
        Some(p) => sha256(&std::fs::read(p).unwrap()),
        None => "unavailable".to_string(),
    };
    let source_sha = sha256(include_bytes!("main.rs"));

    let mut specs: Vec<Spec> = vec![];
    for set in &args.sets {
        let r = match set.as_str() {
            "R16" => r_specs(16, ncurves),
            "R20" => r_specs(20, ncurves),
            "ANOM16" => anom_spec(0, 16).map(|a| vec![a]), // extra: 2^16 positive control only
            _ => anom_spec(0, 16).and_then(|a| anom_spec(1, 20).map(|b| vec![a, b])),
        };
        match r {
            Ok(mut v) => specs.append(&mut v),
            Err(e) => {
                eprintln!("fatal: curve set {}: {}", set, e);
                return 2;
            }
        }
    }
    eprintln!("[{:.1}s] {} curves built", t0.elapsed().as_secs_f64(), specs.len());

    let mut results: Vec<CurveRes> = vec![];
    for sp in &specs {
        match analyze_curve(sp, nulls, threads) {
            Ok(r) => {
                eprintln!("[{:.1}s] {} done", t0.elapsed().as_secs_f64(), sp.name);
                print_curve(&r);
                results.push(r);
            }
            Err(e) => {
                eprintln!("fatal: curve {}: {}", sp.name, e);
                return 2;
            }
        }
    }

    // ---- summary
    let mut sum = vec![];
    // P1 and P4
    let p1_kinds = |k: &Kind| matches!(k, Kind::X | Kind::Y | Kind::Chi(_) | Kind::Dickson);
    for set in ["R16", "R20", "ANOM"] {
        let cs: Vec<&CurveRes> = results.iter().filter(|c| c.set == set).collect();
        if cs.is_empty() {
            continue;
        }
        let key = if set == "ANOM" { "P4_anom_coordinates" } else { "P1_coordinate_functions" };
        let key = key.to_string();
        let (mut mn, mut mx) = (f64::INFINITY, 0f64);
        let (mut n, mut out_band, mut above15) = (0i64, 0i64, 0i64);
        let mut curves_with_above15 = 0i64;
        for cr in &cs {
            let mut any = false;
            for row in cr.rows.iter().filter(|x| {
                if set == "ANOM" {
                    matches!(x.kind, Kind::X | Kind::Y | Kind::Chi(_))
                } else {
                    p1_kinds(&x.kind)
                }
            }) {
                let q = cr.ratio(row);
                mn = mn.min(q);
                mx = mx.max(q);
                n += 1;
                if !(0.8..=1.25).contains(&q) {
                    out_band += 1;
                }
                if q > 1.5 {
                    above15 += 1;
                    any = true;
                }
            }
            if any {
                curves_with_above15 += 1;
            }
        }
        sum.push((
            format!("{}_{}", key, set),
            jo(vec![
                ("curves", ji(cs.len() as i64)),
                ("pairs", ji(n)),
                ("eta_over_null_median_min", jf(mn)),
                ("eta_over_null_median_max", jf(mx)),
                ("pairs_outside_0.8_1.25", ji(out_band)),
                ("pairs_above_1.5", ji(above15)),
                ("curves_with_a_pair_above_1.5", ji(curves_with_above15)),
            ]),
        ));
        // P3
        let (mut n3, mut out3, mut maxabsz) = (0i64, 0i64, 0f64);
        let mut ratios = (f64::INFINITY, 0f64);
        for cr in &cs {
            for e in &cr.energy {
                n3 += 1;
                if !e.in_band {
                    out3 += 1;
                }
                if let Some(z) = e.z {
                    maxabsz = maxabsz.max(z.abs());
                }
                ratios = (ratios.0.min(e.ratio), ratios.1.max(e.ratio));
            }
        }
        sum.push((
            format!("P3_energy_{}", set),
            jo(vec![
                ("curve_K_pairs", ji(n3)),
                ("outside_1_pm_3sd_over_mean", ji(out3)),
                ("energy_ratio_min", jf(ratios.0)),
                ("energy_ratio_max", jf(ratios.1)),
                ("max_abs_z", jf(maxabsz)),
            ]),
        ));
    }
    // P2
    let lam_etas: Vec<f64> = results
        .iter()
        .flat_map(|c| c.rows.iter().filter(|x| x.kind == Kind::Lambda).map(|x| x.res.eta))
        .collect();
    let plant_etas: Vec<f64> = results
        .iter()
        .flat_map(|c| c.rows.iter().filter(|x| x.kind == Kind::Plant).map(|x| x.res.eta))
        .collect();
    let minf = |v: &Vec<f64>| v.iter().cloned().fold(f64::INFINITY, f64::min);
    let lam_min = minf(&lam_etas);
    let plant_min = minf(&plant_etas);
    sum.push((
        "P2_controls".into(),
        jo(vec![
            ("lambda_eta_min", if lam_etas.is_empty() { J::Null } else { jf(lam_min) }),
            ("lambda_curves", ji(lam_etas.len() as i64)),
            ("plant_eta_min", if plant_etas.is_empty() { J::Null } else { jf(plant_min) }),
            ("plant_curves", ji(plant_etas.len() as i64)),
            ("lambda_linear_violations_total", ji(results.iter().filter_map(|c| c.lambda_linear_violations).sum::<u64>())),
        ]),
    ));
    // instrument
    let max_pdev = results.iter().flat_map(|c| c.rows.iter().map(|x| x.res.parseval_dev)).fold(0f64, f64::max);
    let max_ndev = results.iter().flat_map(|c| c.rows.iter().map(|x| x.res.naive_dev)).fold(0f64, f64::max);
    let fft_ok = max_ndev <= 1e-6 && max_pdev <= 1e-6;
    let instr_fail = !fft_ok || (!plant_etas.is_empty() && plant_min < 0.5) || (!lam_etas.is_empty() && lam_min < 0.5);
    sum.push((
        "instrument".into(),
        jo(vec![
            ("parseval_max_relative_deviation_from_r2", jf(max_pdev)),
            ("fft_vs_naive_max_abs_error_over_r", jf(max_ndev)),
            ("fft_error_bound_1e-6_r_holds", J::Bool(fft_ok)),
            ("rule1_instrument_failure_flag", J::Bool(instr_fail)),
        ]),
    ));

    // ---- JSON
    let curves_json: Vec<J> = results.iter().map(curve_json).collect();
    let doc_summary_clone = sum.clone();
    let doc = jo(vec![
        (
            "header",
            jo(vec![
                ("study", js("F3 coord_spectrum_20261110")),
                ("protocol_sha256", J::S(protocol_sha.clone())),
                ("tool_source_sha256", J::S(source_sha.clone())),
                ("quick", J::Bool(args.quick)),
                ("sets", J::A(args.sets.iter().map(|s| js(s)).collect())),
                ("curves_per_random_set", ji(ncurves as i64)),
                ("null_functions_per_curve", ji(nulls as i64)),
                ("a_values_per_function", ji(64)),
                ("energy_null_subsets", ji(50)),
                ("convention", js("g(O)=0 at k=0 for x,y,chi,dickson,lambda; chi rows use +-1 values directly (no a); eta gated over b != 0 (PROTOCOL section 6); PLANT g(k)=floor(k p/r), k in 0..r; S(a,b)=sum_k exp(2 pi i (a g(k)/p + b k/r))")),
            ]),
        ),
        ("curves", J::A(curves_json)),
        ("summary", J::O(sum)),
    ]);
    let mut text = String::new();
    doc.render(&mut text, 0);
    text.push('\n');
    if let Some(parent) = std::path::Path::new(&args.out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(&args.out, &text) {
        eprintln!("cannot write {}: {}", args.out, e);
        return 2;
    }
    println!("\nsummary (numbers, not verdicts):");
    for (k, v) in &doc_summary_clone {
        let mut t = String::new();
        v.render(&mut t, 2);
        println!("  {}: {}", k, t.replace('\n', " ").replace("  ", " "));
    }
    println!("\nprotocol_sha256   {}", protocol_sha);
    println!("tool_source_sha256 {}", source_sha);
    println!("fft: parseval max rel dev {:.3e}, fft-vs-naive max |dS|/r {:.3e}, bound 1e-6 holds: {}", max_pdev, max_ndev, fft_ok);
    println!("wrote {} ({} bytes), {:.1}s on {} threads", args.out, text.len(), t0.elapsed().as_secs_f64(), threads);
    0
}

fn print_curve(cr: &CurveRes) {
    println!(
        "\n== {} p={} a={} b={} r={} G=({},{}) dickson_n={}",
        cr.name, cr.p, cr.a, cr.b, cr.r, cr.gen.0, cr.gen.1, cr.dickson_deg
    );
    println!("  {:<10} {:>11} {:>9}  {:<16} {:>11}", "g", "eta(b!=0)", "eta/med", "argmax(a,b)", "eta_b0");
    for row in cr.rows.iter().filter(|x| null_class(&x.kind).is_none()) {
        println!(
            "  {:<10} {:>11.5e} {:>9.4}  {:<16} {:>11.5e}",
            row.kind.name(),
            row.res.eta,
            cr.ratio(row),
            format!("({},{})", row.res.a, row.res.b),
            row.res.eta_b0
        );
    }
    for cl in 0..5 {
        let v: Vec<f64> = cr.rows.iter().filter(|x| null_class(&x.kind) == Some(cl)).map(|x| x.res.eta).collect();
        if !v.is_empty() {
            let mn = v.iter().cloned().fold(f64::INFINITY, f64::min);
            let mx = v.iter().cloned().fold(0f64, f64::max);
            println!("  NULL {:<12} n={} median {:.5e} min {:.5e} max {:.5e}", NULL_CLASS_NAMES[cl], v.len(), cr.null_medians[cl], mn, mx);
        }
    }
    for e in &cr.energy {
        println!(
            "  energy K={:<6} E={:<14} null mean {:.5e} sd {:.3e}  ratio {:.5} z {} band(3sd/mean) {:.2e} in_band {}",
            e.k,
            e.e_a,
            e.null_mean,
            e.null_sd,
            e.ratio,
            e.z.map(|z| format!("{:+.2}", z)).unwrap_or("na".into()),
            e.band,
            e.in_band
        );
    }
    if let Some(v) = cr.lambda_linear_violations {
        println!("  lambda linearity violations: {}", v);
    }
}

fn curve_json(cr: &CurveRes) -> J {
    let mut rows = vec![];
    let mut null_vals: Vec<Vec<f64>> = vec![vec![]; 5];
    let mut transforms = 0u64;
    for row in &cr.rows {
        transforms += row.res.transforms;
        if let Some(cl) = null_class(&row.kind) {
            null_vals[cl].push(row.res.eta);
            continue;
        }
        rows.push(jo(vec![
            ("g", J::S(row.kind.name())),
            ("eta", jf(row.res.eta)),
            ("argmax_a", ji(row.res.a as i64)),
            ("argmax_b", ji(row.res.b as i64)),
            ("eta_over_null_median", jf(cr.ratio(row))),
            ("null_used", js(NULL_CLASS_NAMES[ratio_class(&row.kind)])),
            ("eta_b0", jf(row.res.eta_b0)),
            ("parseval_max_rel_dev", jf(row.res.parseval_dev)),
            ("fft_vs_naive_abs_error_over_r", jf(row.res.naive_dev)),
            ("fft_transforms", ji(row.res.transforms as i64)),
        ]));
    }
    let en: Vec<J> = cr
        .energy
        .iter()
        .map(|e| {
            jo(vec![
                ("K", ji(e.k as i64)),
                ("distinct_elements", ji(e.distinct as i64)),
                ("E", ji(e.e_a as i64)),
                ("null_mean", jf(e.null_mean)),
                ("null_sd", jf(e.null_sd)),
                ("ratio_to_null_mean", jf(e.ratio)),
                ("z", e.z.map(jf).unwrap_or(J::Null)),
                ("band_3sd_over_mean", jf(e.band)),
                ("within_1_pm_band", J::Bool(e.in_band)),
                ("pair_sums_counted", ji(e.pair_sums as i64)),
            ])
        })
        .collect();
    let pair_sums: u64 = cr.energy.iter().map(|e| e.pair_sums).sum();
    let mut fields: Vec<(String, J)> = vec![
        ("name".into(), J::S(cr.name.clone())),
        ("set".into(), js(cr.set)),
        ("p".into(), ji(cr.p as i64)),
        ("a".into(), ji(cr.a as i64)),
        ("b".into(), ji(cr.b as i64)),
        ("r".into(), ji(cr.r as i64)),
        ("G".into(), J::A(vec![ji(cr.gen.0 as i64), ji(cr.gen.1 as i64)])),
    ];
    fields.extend(cr.extra.iter().cloned());
    fields.push(("dickson_degree".into(), ji(cr.dickson_deg as i64)));
    fields.push(("a_values".into(), J::A(cr.alist.iter().map(|&a| ji(a as i64)).collect())));
    fields.push(("rows".into(), J::A(rows)));
    let mut nl = vec![];
    for cl in 0..5 {
        let v = &null_vals[cl];
        nl.push((
            NULL_CLASS_NAMES[cl].to_string(),
            jo(vec![
                ("n", ji(v.len() as i64)),
                ("eta_values", J::A(v.iter().map(|&x| jf(x)).collect())),
                ("eta_median", jf(cr.null_medians[cl])),
                ("eta_min", jf(v.iter().cloned().fold(f64::INFINITY, f64::min))),
                ("eta_max", jf(v.iter().cloned().fold(0f64, f64::max))),
            ]),
        ));
    }
    fields.push(("nulls".into(), J::O(nl)));
    fields.push((
        "eta_reference_sqrt_ln(64r)/r".into(),
        jf(((64.0 * cr.r as f64).ln() / cr.r as f64).sqrt()),
    ));
    fields.push(("energy".into(), J::A(en)));
    let mut counts = vec![
        ("fft_transforms", ji(transforms as i64)),
        ("fft_length", ji(cr.fft_n as i64)),
        ("fft_butterflies", ji((transforms * cr.fft_butterflies_per_transform) as i64)),
        ("pair_sums", ji(pair_sums as i64)),
        ("group_ops_orbit_affine_adds", ji(cr.group_counts.affine_p_adds as i64)),
        ("group_ops_orbit_inversions_mod_p", ji(cr.group_counts.inv_mod_p as i64)),
    ];
    if let Some(lc) = &cr.lambda_counts {
        counts.push(("lambda", lc.json()));
        counts.push(("lambda_linear_violations", ji(cr.lambda_linear_violations.unwrap_or(0) as i64)));
    }
    fields.push(("counts".into(), jo(counts)));
    J::O(fields)
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_dft(z: &[C]) -> Vec<C> {
        let r = z.len();
        (0..r)
            .map(|b| {
                let mut s = C::default();
                for k in 0..r {
                    let m = ((b * k) % r) as f64;
                    s = s.add(z[k].mul(cis(-2.0 * std::f64::consts::PI * m / r as f64)));
                }
                s
            })
            .collect()
    }

    #[test]
    fn bluestein_matches_naive() {
        for &r in &[7usize, 83, 1021] {
            let mut rng = Rng::from_seed_str(&format!("test/fft/{}", r));
            let z: Vec<C> = (0..r)
                .map(|_| C { re: (rng.next() % 2001) as f64 / 1000.0 - 1.0, im: (rng.next() % 2001) as f64 / 1000.0 - 1.0 })
                .collect();
            let want = naive_dft(&z);
            let bl = Bluestein::new(r);
            let mut buf = vec![C::default(); bl.n];
            buf[..r].copy_from_slice(&z);
            bl.transform(&mut buf);
            let err = (0..r).map(|b| buf[b].sub(want[b]).abs2().sqrt()).fold(0f64, f64::max);
            assert!(err < 1e-9 * r as f64, "r={} err={}", r, err);
        }
    }

    #[test]
    fn energy_matches_bruteforce() {
        let p = 101usize;
        let mut rng = Rng::from_seed_str("test/energy");
        let a: Vec<u32> = (0..20).map(|_| (rng.next() % p as u64) as u32).collect();
        let mut brute = 0u64;
        for &w in &a {
            for &x in &a {
                for &y in &a {
                    for &z in &a {
                        if (w + x) as usize % p == (y + z) as usize % p {
                            brute += 1;
                        }
                    }
                }
            }
        }
        let mut sc = Vec::new();
        assert_eq!(additive_energy(&a, p, &mut sc), brute);
    }

    #[test]
    fn anomalous_curve_has_p_points() {
        let sp = anom_spec(0, 16).unwrap();
        let c = &sp.curve;
        assert!(is_prime(c.p));
        assert_eq!(c.point_count(), c.p);
        assert_eq!(c.r, c.p);
        assert!(c.mul(c.p, Some(sp.gen)).unwrap().is_none());
        assert!(c.mul(1, Some(sp.gen)).unwrap().is_some());
    }

    #[test]
    fn lambda_is_linear_on_anomalous_2_16() {
        let sp = anom_spec(0, 16).unwrap();
        let c = &sp.curve;
        let p = c.p;
        let (xs, ys, _) = orbit(c, sp.gen).unwrap();
        let n = 3000usize;
        let (lam, _) = lambda_orbit(c, &xs[..n], &ys[..n], 2).unwrap();
        let l1 = lam[1] as u64;
        assert_ne!(l1, 0);
        assert_eq!(lam[2] as u64, 2 * l1 % p);
        for k in 1..n {
            assert_eq!(lam[k] as u64, (k as u64 * l1) % p, "k={}", k);
        }
    }

    #[test]
    fn pm_statistic_excludes_b0_and_sees_planted_frequency() {
        let r = 1021usize;
        let bl = Bluestein::new(r);
        let mut rng = Rng::from_seed_str("test/pm");
        let v: Vec<i8> = (0..r).map(|_| if rng.next() & 1 == 1 { 1 } else { -1 }).collect();
        let s = spectrum_pm(&v, &bl);
        assert!(s.eta < 0.15, "random pm eta={}", s.eta);
        // all-ones: S(0) = r is excluded, so eta over b != 0 is ~0 while eta_b0 = 1
        let ones = vec![1i8; r];
        let so = spectrum_pm(&ones, &bl);
        assert!(so.eta < 1e-9 && (so.eta_b0 - 1.0).abs() < 1e-12);
        // sign of cos(2 pi b0 k / r) is not exactly a pure tone; use the real part of a planted tone
        let b0 = 100u64;
        let fill = |_a: u64, out: &mut [C]| {
            for k in 0..out.len() {
                let m = ((b0 * k as u64) % r as u64) as f64;
                out[k] = C { re: (2.0 * std::f64::consts::PI * m / r as f64).cos(), im: 0.0 };
            }
        };
        let st = spectrum_gen(&bl, &[0], &fill);
        assert!((st.eta - 0.5).abs() < 1e-9, "eta={}", st.eta);
        assert!(st.b == b0 || st.b == r as u64 - b0);
    }

    #[test]
    fn symmetric_nulls_have_their_symmetry() {
        let (p, r) = (2039u64, 1021usize);
        for j in 0..5 {
            let mut rng = Rng::from_seed_str(&format!("test/sym/{}", j));
            let e = symmetric_null(2, p, r, &mut rng);
            let o = symmetric_null(3, p, r, &mut rng);
            let q = symmetric_null(4, p, r, &mut rng);
            assert_eq!((e[0], o[0], q[0]), (0, 0, 0));
            for k in 1..r {
                assert_eq!(e[k], e[r - k]);
                assert_eq!((o[k] + o[r - k]).rem_euclid(p as i64), 0);
                assert_eq!(q[k], q[r - k]);
                assert!(q[k] == 1 || q[k] == -1);
                assert!(e[k] >= 0 && (e[k] as u64) < p && o[k] >= 0 && (o[k] as u64) < p);
            }
        }
    }

    #[test]
    fn plant_is_seen_and_null_is_not() {
        let (p, r) = (2039u64, 1021usize);
        let bl = Bluestein::new(r);
        let phase = phase_table(p);
        let alist = [1u64, 5, 17];
        let g: Vec<u32> = (0..r).map(|k| ((k as u64 * p) / r as u64) as u32).collect();
        let s = spectrum_eta(&g, p, &bl, &phase, &alist);
        assert!(s.eta >= 0.5, "eta={}", s.eta);
        assert_eq!(s.b, (r as u64 - s.a) % r as u64);
        assert!(s.naive_dev < 1e-9 && s.parseval_dev < 1e-9);
        let mut rng = Rng::from_seed_str("test/null");
        let gn: Vec<u32> = (0..r).map(|_| (rng.next() % p) as u32).collect();
        let sn = spectrum_eta(&gn, p, &bl, &phase, &alist);
        assert!(sn.eta < 0.3, "null eta={}", sn.eta);
    }
}
