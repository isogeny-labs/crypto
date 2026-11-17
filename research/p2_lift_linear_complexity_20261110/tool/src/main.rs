//! F2: linear complexity of the p^2-lift defect along the group.
//! Standalone, std-only tool implementing PROTOCOL.md (frozen 2026-11-10).
//! Arithmetic port of research/p2_lift_defect_20261101/run.py (H110).

use std::collections::BTreeMap;
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

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            best = d;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

fn mult_order(p: u64, r: u64) -> u64 {
    let mut x = p % r;
    let mut e = 1;
    while x != 1 {
        x = x * p % r;
        e += 1;
    }
    e
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

fn seq_sha256(seq: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(seq.len() * 4);
    for x in seq {
        bytes.extend_from_slice(&x.to_le_bytes());
    }
    sha256(&bytes)
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
    bm_mults: u64,
    bm_inversions: u64,
}

#[allow(dead_code)]
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
        self.bm_mults += o.bm_mults;
        self.bm_inversions += o.bm_inversions;
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
            ("bm_multiplications", ji(self.bm_mults)),
            ("bm_inversions", ji(self.bm_inversions)),
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
    /// first x scan, G = [cofactor] P, accepted when nonzero (r prime)
    fn cofactor_point(&self, cofactor: u64) -> Result<(u64, u64), String> {
        let sq = self.sqrt_table();
        for x in 0..self.p {
            let y = sq[self.rhs(x) as usize] as u64;
            if y != 0 {
                if let Some(q) = self.mul(cofactor, Some((x, y)))? {
                    if self.mul(self.r, Some(q))?.is_some() {
                        return Err("cofactor point not of order r".into());
                    }
                    return Ok(q);
                }
            }
        }
        Err("no point of order r".into())
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

#[derive(Clone, Copy, Debug)]
struct Section {
    tau0: u64,
    tau1: u64,
    #[allow(dead_code)]
    slope: u64,
    torsion_u: u64,
}

fn section_record(c: &Curve, pt: (u64, u64), cnt: &mut Counts) -> Result<Section, String> {
    let p = c.p;
    let half = (c.r - 1) / 2;
    let third = std::cmp::max(1, c.r / 3);
    let tau0 = tau_for_section(c, pt, 0, half, cnt)?;
    let tau0b = tau_for_section(c, pt, 0, third, cnt)?;
    if tau0 != tau0b {
        return Err("independent split formal coordinates disagree".into());
    }
    let tau1 = tau_for_section(c, pt, 1, half, cnt)?;
    let slope = (tau1 + p - tau0) % p;
    if slope == 0 {
        return Err("section defect has zero first-layer slope".into());
    }
    cnt.inv_mod_p += 1;
    let tu = ((p - tau0) % p) as u128 * modinv(slope as u128, p as u128).ok_or("inv slope")? % p as u128;
    let tu = tu as u64;
    let ttau = tau_for_section(c, pt, tu, half, cnt)?;
    if ttau != 0 {
        return Err("solved torsion lift did not kill tau".into());
    }
    Ok(Section { tau0, tau1, slope, torsion_u: tu })
}

struct Seqs {
    o1: Vec<u32>,
    o6: Vec<u32>,
    o2: Vec<u32>,
    o3: Vec<u32>,
}

/// Sequences over k = 0..r-1 for generator g (a_0 = 0 for all).
fn gen_sequences(c: &Curve, g: (u64, u64)) -> Result<(Seqs, Counts), String> {
    let p = c.p;
    let r = c.r as usize;
    let mut cnt = Counts::default();
    let rinv = modinv(c.r as u128, p as u128).ok_or("r not invertible mod p")? as u64;
    let mut s = Seqs { o1: vec![0; r], o6: vec![0; r], o2: vec![0; r], o3: vec![0; r] };
    let mut cur: Pt = None;
    let gp = Some(g);
    for k in 1..r {
        cur = c.add(cur, gp, &mut cnt)?;
        let q = cur.ok_or("orbit reached identity early")?;
        let rec = section_record(c, q, &mut cnt).map_err(|e| format!("k={}: {}", k, e))?;
        let f0 = rec.tau0 * rinv % p;
        let f1 = rec.tau1 * rinv % p;
        s.o1[k] = f0 as u32;
        s.o6[k] = rec.torsion_u as u32;
        s.o2[k] = f1 as u32;
        s.o3[k] = ((f1 + p - f0) % p) as u32;
    }
    if c.add(cur, gp, &mut cnt)?.is_some() {
        return Err("generator orbit failed (order != r)".into());
    }
    Ok((s, cnt))
}

// ------------------------------------------------------------------ BM

/// Berlekamp-Massey over F_p on the doubled sequence (period r). Returns L.
fn bm_period(seq: &[u32], p: u64, mults: &mut u64, invs: &mut u64) -> usize {
    let r = seq.len();
    let n = 2 * r;
    let mut sr = vec![0u32; n]; // sr[k] = s[n-1-k]
    let mut s = vec![0u32; n];
    for i in 0..n {
        s[i] = seq[i % r];
        sr[n - 1 - i] = s[i];
    }
    let mut c = vec![0u32; n + 2];
    let mut b = vec![0u32; n + 2];
    let mut tmp = vec![0u32; n + 2];
    c[0] = 1;
    b[0] = 1;
    let (mut l, mut lb, mut m) = (0usize, 0usize, 1usize);
    let mut bd: u64 = 1;
    for i in 0..n {
        let off = n - 1 - i;
        let mut acc: u64 = 0;
        for (x, y) in c[..=l].iter().zip(&sr[off..off + l + 1]) {
            acc += (*x as u64) * (*y as u64);
        }
        *mults += l as u64;
        let d = acc % p;
        if d == 0 {
            m += 1;
            continue;
        }
        *invs += 1;
        let binv = modinv(bd as u128, p as u128).unwrap() as u64;
        let coef = d * binv % p;
        let w = (p - coef) % p;
        let wp = (w << 32) / p;
        let change = 2 * l <= i;
        if change {
            tmp[..=l].copy_from_slice(&c[..=l]);
        }
        {
            let dst = &mut c[m..m + lb + 1];
            for (cv, bv) in dst.iter_mut().zip(&b[..=lb]) {
                let bj = *bv as u64;
                let q = (bj * wp) >> 32;
                let mut t = bj * w - q * p;
                if t >= p {
                    t -= p;
                }
                let mut v = *cv as u64 + t;
                if v >= p {
                    v -= p;
                }
                *cv = v as u32;
            }
        }
        *mults += (lb + 1) as u64;
        if change {
            let newl = i + 1 - l;
            std::mem::swap(&mut b, &mut tmp);
            lb = l;
            l = newl;
            bd = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    l
}

/// Plain reference BM (for tests).
#[cfg(test)]
fn bm_naive(s: &[u64], p: u64) -> usize {
    let n = s.len();
    let mut c = vec![0u64; n + 1];
    let mut b = vec![0u64; n + 1];
    c[0] = 1;
    b[0] = 1;
    let (mut l, mut m, mut bd) = (0usize, 1usize, 1u64);
    for i in 0..n {
        let mut d = 0u64;
        for j in 0..=l {
            d = (d + c[j] * s[i - j]) % p;
        }
        if d == 0 {
            m += 1;
        } else {
            let coef = d * powmod(bd, p - 2, p) % p;
            let t = c.clone();
            for j in 0..n + 1 - m {
                c[j + m] = (c[j + m] + p - coef * b[j] % p) % p;
            }
            if 2 * l <= i {
                l = i + 1 - l;
                b = t;
                bd = d;
                m = 1;
            } else {
                m += 1;
            }
        }
    }
    l
}

// ------------------------------------------------------------- F_{p^2}

#[derive(Clone, Copy, PartialEq, Debug)]
struct F2(u64, u64); // a + b i, i^2 = -n

struct Fp2 {
    p: u64,
    n: u64,
}
impl Fp2 {
    fn mul(&self, x: F2, y: F2) -> F2 {
        let p = self.p as u128;
        let re = (x.0 as u128 * y.0 as u128 + p * p - (self.n as u128 * (x.1 as u128 * y.1 as u128 % p)) % p) % p;
        let im = (x.0 as u128 * y.1 as u128 + x.1 as u128 * y.0 as u128) % p;
        F2(re as u64, im as u64)
    }
    fn pow(&self, mut x: F2, mut e: u64) -> F2 {
        let mut r = F2(1, 0);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, x);
            }
            x = self.mul(x, x);
            e >>= 1;
        }
        r
    }
}

struct Plant {
    n: u64,
    alpha: F2,
    zeta: F2,
    seq: Vec<u32>,
}

/// a_k = Tr(alpha zeta^k) over F_p[i]/(i^2+n), zeta of order r | p+1.
fn plant_sequence(p: u64, r: u64, alpha: F2) -> Result<Plant, String> {
    if (p + 1) % r != 0 {
        return Err("r does not divide p+1".into());
    }
    // smallest n with -n a non-residue (so x^2 + n is irreducible)
    let mut n = 1;
    while powmod((p - n % p) % p, (p - 1) / 2, p) != p - 1 {
        n += 1;
    }
    let f = Fp2 { p, n };
    let e = (p * p - 1) / r;
    let mut zeta = None;
    for k in 0..p {
        let z = f.pow(F2(k, 1), e);
        if z != F2(1, 0) {
            zeta = Some(z);
            break;
        }
    }
    let zeta = zeta.ok_or("no zeta found")?;
    if f.pow(zeta, r) != F2(1, 0) {
        return Err("zeta^r != 1".into());
    }
    // Frobenius check: x^p == conjugate
    let probe = f.mul(alpha, zeta);
    if f.pow(probe, p) != F2(probe.0, (p - probe.1) % p) {
        return Err("Frobenius is not conjugation".into());
    }
    let mut seq = Vec::with_capacity(r as usize);
    let mut z = alpha;
    for _ in 0..r {
        seq.push((2 * z.0 % p) as u32);
        z = f.mul(z, zeta);
    }
    Ok(Plant { n, alpha, zeta, seq })
}

// ------------------------------------------------------------ curve sets

struct Spec {
    name: String,
    set: &'static str,
    curve: Curve,
    gen: (u64, u64),
    extra: Vec<(String, J)>,
}

const H_CURVES: [(u64, u64, u64, u64, &str); 7] = [
    (101, 6, 9, 83, "E101"),
    (127, 1, 7, 109, "E127"),
    (149, 2, 8, 139, "E149"),
    (211, 1, 1, 223, "E211"),
    (283, 2, 7, 281, "E283"),
    (503, 3, 8, 499, "E503"),
    (907, 1, 1, 947, "E907"),
];

fn h_specs() -> Result<Vec<Spec>, String> {
    let mut v = vec![];
    for (p, a, b, r, name) in H_CURVES {
        let curve = Curve { p, a, b, r, name: name.to_string() };
        if r == p {
            return Err("H110 excludes anomalous curves".into());
        }
        let gen = curve.first_point()?;
        v.push(Spec { name: name.to_string(), set: "H", curve, gen, extra: vec![] });
    }
    Ok(v)
}

fn prime_below(limit: u64) -> u64 {
    let mut q = limit - 1;
    while !is_prime(q) {
        q -= 1;
    }
    q
}

fn r_specs(bits: u32) -> Result<Vec<Spec>, String> {
    let p = prime_below(1u64 << bits);
    let mut v = vec![];
    for i in 0..4 {
        let mut rng = Rng::from_seed_str(&format!("F2/curve/{}/{}", bits, i));
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
            set: match bits {
                12 => "R12",
                14 => "R14",
                _ => "R16",
            },
            curve,
            gen,
            extra: vec![("candidates_tried".into(), ji(tries as i64))],
        });
    }
    Ok(v)
}

fn s_specs() -> Result<Vec<Spec>, String> {
    let mut v = vec![];
    // NB: protocol text says bits 14/15 but r >= 2^12 with 3 | p+1 and 2 | p+1 forces
    // p >= 6*4099-1 > 2^14, so bits 14 is infeasible; use 15 and 16 (deviation, reported).
    for (i, bits) in [15u32, 16].iter().enumerate() {
        let mut p = prime_below(1u64 << bits);
        let mut skipped = vec![];
        loop {
            if p % 3 == 2 && is_prime(p) {
                let r = largest_prime_factor(p + 1);
                if r >= 4096 {
                    break;
                }
                skipped.push(J::Int(p as i128));
            }
            p -= 1;
            if p < 3 {
                return Err("no supersingular prime with r >= 2^12".into());
            }
        }
        let r = largest_prime_factor(p + 1);
        let cof = (p + 1) / r;
        let mut rng = Rng::from_seed_str(&format!("F2/ss/{}", i));
        let b = loop {
            let b = rng.next() % p;
            if b != 0 {
                break b;
            }
        };
        let name = format!("S_{}", i);
        let curve = Curve { p, a: 0, b, r, name: name.clone() };
        if curve.point_count() != p + 1 {
            return Err("supersingular count != p+1".into());
        }
        let gen = curve.cofactor_point(cof)?;
        v.push(Spec {
            name,
            set: "S",
            curve,
            gen,
            extra: vec![
                ("order_total".into(), ji((p + 1) as i64)),
                ("cofactor".into(), ji(cof as i64)),
                ("p_stepped_over".into(), J::A(skipped)),
                ("bits_target".into(), ji(*bits as i64)),
            ],
        });
    }
    Ok(v)
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

// ------------------------------------------------------------- analysis

struct BmRes {
    l: usize,
    mults: u64,
    invs: u64,
    mean_nonzero: bool,
    plant: Option<Plant>,
    err: Option<String>,
}

fn run_bm(seq: &[u32], p: u64) -> BmRes {
    let sum: u64 = seq.iter().map(|&x| x as u64).sum();
    let (mut mu, mut iv) = (0, 0);
    let l = bm_period(seq, p, &mut mu, &mut iv);
    BmRes { l, mults: mu, invs: iv, mean_nonzero: sum % p != 0, plant: None, err: None }
}

fn derive_c(l: usize, mean_nonzero: bool, m: u64) -> Result<u64, String> {
    let x = l as u64 - mean_nonzero as u64;
    if x % m != 0 {
        return Err(format!("L - [mean!=0] = {} not divisible by m = {}", x, m));
    }
    Ok(x / m)
}

fn null_params(r: u64, p: u64, m: u64) -> (u64, f64, f64) {
    let n = (r - 1) / m;
    let lg = (n as f64).log10() - (m as f64) * (p as f64).log10();
    (n, 10f64.powf(lg), lg)
}

#[derive(Clone, Copy)]
enum Job {
    Row(usize, usize, usize), // curve, mult idx, observable
    Row7(usize, usize),
    Null(usize, usize),
    Plant(usize),
}

const OBS: [&str; 4] = ["O1", "O6", "O2", "O3"];

fn pick<'a>(s: &'a Seqs, o: usize) -> &'a Vec<u32> {
    match o {
        0 => &s.o1,
        1 => &s.o6,
        2 => &s.o2,
        _ => &s.o3,
    }
}

struct Args {
    out: String,
    sets: Vec<String>,
    null: usize,
    quick: bool,
    protocol: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        out: "results/run.json".into(),
        sets: ["H", "R12", "R14", "R16", "S"].iter().map(|s| s.to_string()).collect(),
        null: 200,
        quick: false,
        protocol: None,
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let mut null_given = false;
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
            "--null" => {
                i += 1;
                a.null = argv.get(i).ok_or("--null needs a value")?.parse().map_err(|_| "bad --null")?;
                null_given = true;
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
        a.sets = vec!["H".into()];
        if !null_given {
            a.null = 10;
        }
    }
    for s in &a.sets {
        if !["H", "R12", "R14", "R16", "S"].contains(&s.as_str()) {
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

fn run(args: &Args) -> i32 {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mults: Vec<u64> = if args.quick { vec![1] } else { vec![1, 2, 3, 5] };
    let mut fatal: Vec<String> = vec![];

    // protocol hash
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

    // curve specs
    let mut specs: Vec<Spec> = vec![];
    for set in &args.sets {
        let r = match set.as_str() {
            "H" => h_specs(),
            "R12" => r_specs(12),
            "R14" => r_specs(14),
            "R16" => r_specs(16),
            _ => s_specs(),
        };
        match r {
            Ok(mut v) => specs.append(&mut v),
            Err(e) => {
                eprintln!("fatal: curve set {}: {}", set, e);
                return 2;
            }
        }
    }

    // phase A: sequences
    let gen_jobs: Vec<(usize, usize)> =
        (0..specs.len()).flat_map(|c| (0..mults.len()).map(move |m| (c, m))).collect();
    let gen_res = par_map(&gen_jobs, threads, |&(ci, mi)| -> Result<(Seqs, Counts), String> {
        let sp = &specs[ci];
        let g = sp.curve.mul(mults[mi], Some(sp.gen))?.ok_or("generator relabeling reached identity")?;
        gen_sequences(&sp.curve, g)
    });
    let mut seqs: Vec<Vec<Option<(Seqs, Counts)>>> = (0..specs.len()).map(|_| (0..mults.len()).map(|_| None).collect()).collect();
    for (&(ci, mi), r) in gen_jobs.iter().zip(gen_res) {
        match r {
            Ok(x) => seqs[ci][mi] = Some(x),
            Err(e) => fatal.push(format!("instrument error, row {} x{}: {}", specs[ci].name, mults[mi], e)),
        }
    }

    // moduli m per curve
    let ms: Vec<u64> = specs.iter().map(|s| mult_order(s.curve.p, s.curve.r)).collect();

    // phase B: BM jobs
    let mut jobs: Vec<Job> = vec![];
    for ci in 0..specs.len() {
        for mi in 0..mults.len() {
            if seqs[ci][mi].is_none() {
                continue;
            }
            for o in 0..4 {
                jobs.push(Job::Row(ci, mi, o));
            }
            jobs.push(Job::Row7(ci, mi));
        }
        for j in 0..args.null {
            jobs.push(Job::Null(ci, j));
        }
        if specs[ci].set == "S" {
            jobs.push(Job::Plant(ci));
        }
    }
    let bm_res = par_map(&jobs, threads, |job| -> BmRes {
        match *job {
            Job::Row(ci, mi, o) => {
                let s = &seqs[ci][mi].as_ref().unwrap().0;
                run_bm(pick(s, o), specs[ci].curve.p)
            }
            Job::Row7(ci, mi) => {
                let c = &specs[ci].curve;
                let s = &seqs[ci][mi].as_ref().unwrap().0.o1;
                let sum: u64 = s.iter().map(|&x| x as u64).sum::<u64>() % c.p;
                let rinv = modinv(c.r as u128, c.p as u128).unwrap() as u64;
                let mean = sum * rinv % c.p;
                let shifted: Vec<u32> = s.iter().map(|&x| ((x as u64 + c.p - mean) % c.p) as u32).collect();
                run_bm(&shifted, c.p)
            }
            Job::Null(ci, j) => {
                let c = &specs[ci].curve;
                let mut rng = Rng::from_seed_str(&format!("F2/null/{}/{}", c.name, j));
                let s: Vec<u32> = (0..c.r).map(|_| (rng.next() % c.p) as u32).collect();
                run_bm(&s, c.p)
            }
            Job::Plant(ci) => {
                let c = &specs[ci].curve;
                let mut rng = Rng::from_seed_str(&format!("F2/plant/{}", c.name));
                let alpha = loop {
                    let a = F2(rng.next() % c.p, rng.next() % c.p);
                    if a != F2(0, 0) {
                        break a;
                    }
                };
                match plant_sequence(c.p, c.r, alpha) {
                    Ok(pl) => {
                        let mut r = run_bm(&pl.seq, c.p);
                        r.plant = Some(pl);
                        r
                    }
                    Err(e) => BmRes { l: 0, mults: 0, invs: 0, mean_nonzero: false, plant: None, err: Some(e) },
                }
            }
        }
    });

    // assemble
    let mut rows: Vec<J> = vec![];
    let mut null_out: Vec<J> = vec![];
    let mut plant_out: Vec<J> = vec![];
    let mut aa_out: Vec<J> = vec![];
    let mut summary_out: Vec<J> = vec![];
    let mut curves_out: Vec<J> = vec![];
    let mut table = String::new();
    // lookup helpers
    let find = |job: Job| -> &BmRes {
        let idx = jobs
            .iter()
            .position(|j| match (j, &job) {
                (Job::Row(a, b, c), Job::Row(x, y, z)) => a == x && b == y && c == z,
                (Job::Row7(a, b), Job::Row7(x, y)) => a == x && b == y,
                (Job::Null(a, b), Job::Null(x, y)) => a == x && b == y,
                (Job::Plant(a), Job::Plant(x)) => a == x,
                _ => false,
            })
            .unwrap();
        &bm_res[idx]
    };

    writeln!(
        table,
        "{:<7} {:>5} {:>6} {:>5} {:>6} | {:>13} {:>13} {:>13} {:>13} | {:<4} | {:<11} {:>10}",
        "curve", "p", "r", "m", "N", "O1 L/c", "O6 L/c", "O2 L/c", "O3 L/c", "A/A", "null c rng", "Pvanish"
    )
    .unwrap();

    for (ci, sp) in specs.iter().enumerate() {
        let c = &sp.curve;
        let m = ms[ci];
        let (nn, pv, lpv) = null_params(c.r, c.p, m);
        let mut cj = vec![
            ("name", js(&sp.name)),
            ("set", js(sp.set)),
            ("p", ji(c.p)),
            ("a", ji(c.a)),
            ("b", ji(c.b)),
            ("r", ji(c.r)),
            ("m", ji(m)),
            ("N", ji(nn)),
            ("generator", J::A(vec![ji(sp.gen.0), ji(sp.gen.1)])),
        ];
        let mut cjo = match jo(std::mem::take(&mut cj)) {
            J::O(v) => v,
            _ => unreachable!(),
        };
        for (k, v) in &sp.extra {
            cjo.push((k.clone(), v.clone()));
        }
        curves_out.push(J::O(cjo));

        // per observable per multiplier
        let mut aa_detail: Vec<J> = vec![];
        let mut aa_pass = true;
        let mut first_lc: [Option<(usize, u64, bool)>; 4] = [None; 4];
        let mut have_rows = false;
        for mi in 0..mults.len() {
            let Some((sq, gc)) = seqs[ci][mi].as_ref() else { continue };
            have_rows = true;
            for o in 0..4 {
                let bm = find(Job::Row(ci, mi, o));
                let row_ctx = format!("{} x{} {}", sp.name, mults[mi], OBS[o]);
                let cval = match derive_c(bm.l, bm.mean_nonzero, m) {
                    Ok(v) => v,
                    Err(e) => {
                        fatal.push(format!("instrument error, row {}: {}", row_ctx, e));
                        0
                    }
                };
                let mut checks = vec![
                    ("c_integer", J::Bool(true)),
                    ("tau0_two_splits_agree", J::Bool(true)),
                    ("tau_at_torsion_u_is_zero", J::Bool(true)),
                ];
                let mut l7 = J::Null;
                if o == 0 {
                    let b7 = find(Job::Row7(ci, mi));
                    let expect = bm.l - bm.mean_nonzero as usize;
                    let ok = b7.l == expect;
                    if !ok {
                        fatal.push(format!("instrument error, row {}: L7 = {} != L - [mean!=0] = {}", row_ctx, b7.l, expect));
                    }
                    l7 = ji(b7.l as i64);
                    checks.push(("L7_equals_L_minus_mean_nonzero", J::Bool(ok)));
                }
                let mut counts = gc.clone();
                counts.bm_mults = bm.mults;
                counts.bm_inversions = bm.invs;
                if o == 0 {
                    let b7 = find(Job::Row7(ci, mi));
                    counts.bm_mults += b7.mults;
                    counts.bm_inversions += b7.invs;
                }
                rows.push(jo(vec![
                    ("curve", js(&sp.name)),
                    ("set", js(sp.set)),
                    ("multiplier", ji(mults[mi])),
                    ("observable", js(OBS[o])),
                    ("p", ji(c.p)),
                    ("r", ji(c.r)),
                    ("m", ji(m)),
                    ("L", ji(bm.l as i64)),
                    ("c", ji(cval)),
                    ("N", ji(nn)),
                    ("mean_nonzero", J::Bool(bm.mean_nonzero)),
                    ("L7", l7),
                    ("P_vanish", J::F(pv)),
                    ("log10_P_vanish", J::F(lpv)),
                    ("sequence_sha256", J::S(seq_sha256(pick(sq, o)))),
                    ("checks", jo(checks)),
                    ("counts_generation_shared_by_curve_multiplier", gc.json()),
                    ("counts", counts.json()),
                ]));
                let key = (bm.l, cval, bm.mean_nonzero);
                match first_lc[o] {
                    None => first_lc[o] = Some(key),
                    Some(k0) => {
                        if k0 != key {
                            aa_pass = false;
                        }
                    }
                }
                aa_detail.push(jo(vec![
                    ("multiplier", ji(mults[mi])),
                    ("observable", js(OBS[o])),
                    ("L", ji(bm.l as i64)),
                    ("c", ji(cval)),
                    ("mean_nonzero", J::Bool(bm.mean_nonzero)),
                ]));
            }
        }
        if have_rows && !aa_pass {
            fatal.push(format!("A/A failure on curve {}", sp.name));
        }
        aa_out.push(jo(vec![
            ("curve", js(&sp.name)),
            ("multipliers", J::A(mults.iter().map(|&x| ji(x)).collect())),
            ("pass", J::Bool(have_rows && aa_pass)),
            ("detail", J::A(aa_detail)),
        ]));

        // null
        let mut hist: BTreeMap<(usize, u64), u64> = BTreeMap::new();
        let (mut cmin, mut cmax) = (u64::MAX, 0u64);
        let mut below = 0u64;
        for j in 0..args.null {
            let bm = find(Job::Null(ci, j));
            match derive_c(bm.l, bm.mean_nonzero, m) {
                Ok(cv) => {
                    *hist.entry((bm.l, cv)).or_insert(0) += 1;
                    cmin = cmin.min(cv);
                    cmax = cmax.max(cv);
                    if cv < nn {
                        below += 1;
                    }
                }
                Err(e) => fatal.push(format!("instrument error, null {} #{}: {}", sp.name, j, e)),
            }
        }
        null_out.push(jo(vec![
            ("curve", js(&sp.name)),
            ("n_sequences", ji(args.null as i64)),
            ("seed_pattern", js("F2/null/<curve>/<j>")),
            ("N", ji(nn)),
            ("P_vanish_union_bound", J::F(pv)),
            ("log10_P_vanish_union_bound", J::F(lpv)),
            ("c_min", if args.null > 0 { ji(cmin) } else { J::Null }),
            ("c_max", ji(cmax)),
            ("n_with_c_below_N", ji(below)),
            (
                "histogram",
                J::A(
                    hist.iter()
                        .map(|(&(l, cv), &n)| jo(vec![("L", ji(l as i64)), ("c", ji(cv)), ("count", ji(n))]))
                        .collect(),
                ),
            ),
        ]));

        // plant
        let mut plant_ok = true;
        if sp.set == "S" {
            let bm = find(Job::Plant(ci));
            if let Some(e) = &bm.err {
                fatal.push(format!("instrument error, plant {}: {}", sp.name, e));
                plant_ok = false;
            } else {
                let pl = bm.plant.as_ref().unwrap();
                let cv = derive_c(bm.l, bm.mean_nonzero, m).unwrap_or(0);
                let pass = bm.l == 2 && cv == 1 && m == 2;
                if !pass {
                    fatal.push(format!("positive control O5 failed on {}: L = {}, c = {}, m = {}", sp.name, bm.l, cv, m));
                }
                plant_ok = pass;
                plant_out.push(jo(vec![
                    ("curve", js(&sp.name)),
                    ("nonresidue_n", ji(pl.n)),
                    ("alpha", J::A(vec![ji(pl.alpha.0), ji(pl.alpha.1)])),
                    ("zeta", J::A(vec![ji(pl.zeta.0), ji(pl.zeta.1)])),
                    ("L", ji(bm.l as i64)),
                    ("c", ji(cv)),
                    ("m", ji(m)),
                    ("mean_nonzero", J::Bool(bm.mean_nonzero)),
                    ("pass", J::Bool(pass)),
                    ("bm_multiplications", ji(bm.mults)),
                ]));
            }
        }
        let _ = plant_ok;

        // summary + table (multiplier index 0)
        if let Some((_, _)) = seqs[ci][0].as_ref() {
            let mut cells = vec![];
            let mut srow = vec![("curve", js(&sp.name)), ("N", ji(nn)), ("m", ji(m)), ("P_vanish", J::F(pv)), ("log10_P_vanish", J::F(lpv))];
            for o in 0..4 {
                let bm = find(Job::Row(ci, 0, o));
                let cv = derive_c(bm.l, bm.mean_nonzero, m).unwrap_or(0);
                cells.push(format!("{}/{}", bm.l, cv));
                if o < 2 {
                    srow.push((
                        if o == 0 { "O1" } else { "O6" },
                        jo(vec![("L", ji(bm.l as i64)), ("c", ji(cv)), ("N", ji(nn)), ("c_minus_N", ji(cv as i64 - nn as i64))]),
                    ));
                }
            }
            summary_out.push(jo(srow));
            writeln!(
                table,
                "{:<7} {:>5} {:>6} {:>5} {:>6} | {:>13} {:>13} {:>13} {:>13} | {:<4} | {:<11} {:>10.2e}",
                sp.name,
                c.p,
                c.r,
                m,
                nn,
                cells[0],
                cells[1],
                cells[2],
                cells[3],
                if aa_pass { "ok" } else { "FAIL" },
                if args.null > 0 { format!("{}..{}", cmin, cmax) } else { "-".into() },
                pv
            )
            .unwrap();
        }
    }

    let mut hdr = vec![
        ("tool", js("p2_lift_linear_complexity 0.1.0")),
        ("protocol_sha256", J::S(protocol_sha.clone())),
        ("tool_source_sha256", J::S(source_sha.clone())),
        ("sets", J::A(args.sets.iter().map(|s| js(s)).collect())),
        ("multipliers", J::A(mults.iter().map(|&x| ji(x)).collect())),
        ("null_sequences_per_curve", ji(args.null as i64)),
        ("quick", J::Bool(args.quick)),
    ];
    hdr.push(("curves", J::A(curves_out)));
    let mut top = vec![
        ("header", jo(hdr)),
        ("rows", J::A(rows)),
        ("null", J::A(null_out)),
        ("plant", J::A(plant_out)),
        ("aa_checks", J::A(aa_out)),
        ("summary", J::A(summary_out)),
    ];
    top.push(("fatal", J::A(fatal.iter().map(|s| J::S(s.clone())).collect())));
    let mut text = String::new();
    jo(top).render(&mut text, 0);
    text.push('\n');
    if let Some(parent) = std::path::Path::new(&args.out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(&args.out, text) {
        eprintln!("cannot write {}: {}", args.out, e);
        return 3;
    }
    print!("{}", table);
    println!("protocol_sha256 {}", protocol_sha);
    println!("tool_source_sha256 {}", source_sha);
    println!("output {}", args.out);
    if !fatal.is_empty() {
        for f in &fatal {
            eprintln!("FATAL: {}", f);
        }
        return 2;
    }
    0
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
mod tests {
    use super::*;

    fn e101() -> (Curve, (u64, u64)) {
        let c = Curve { p: 101, a: 6, b: 9, r: 83, name: "E101".into() };
        let g = c.first_point().unwrap();
        (c, g)
    }

    #[test]
    fn sha256_known() {
        assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(sha256(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn bm_known_lfsr() {
        // s_n = 3 s_{n-1} + 5 s_{n-2} + 2 s_{n-3} mod 101, period irrelevant: use long period via direct BM on 2r terms
        let p = 101u64;
        let mut s = vec![1u64, 7, 4];
        for n in 3..60 {
            let v = (3 * s[n - 1] + 5 * s[n - 2] + 2 * s[n - 3]) % p;
            s.push(v);
        }
        assert_eq!(bm_naive(&s, p), 3);
        // fast BM works on doubled period sequences: build a genuinely periodic one
        // Fibonacci mod 7 has period 16 and linear complexity 2.
        let mut f = vec![0u32, 1];
        for n in 2..16 {
            let v = (f[n - 1] + f[n - 2]) % 7;
            f.push(v);
        }
        assert_eq!((f[14] + f[15]) % 7, f[0] % 7);
        let (mut a, mut b) = (0, 0);
        assert_eq!(bm_period(&f, 7, &mut a, &mut b), 2);
    }

    #[test]
    fn bm_fast_matches_naive() {
        let mut rng = Rng::from_seed_str("test/bm");
        for &p in &[7u64, 101, 65521] {
            for &r in &[5usize, 17, 83, 200] {
                let s: Vec<u32> = (0..r).map(|_| (rng.next() % p) as u32).collect();
                let d: Vec<u64> = (0..2 * r).map(|i| s[i % r] as u64).collect();
                let (mut a, mut b) = (0, 0);
                assert_eq!(bm_period(&s, p, &mut a, &mut b), bm_naive(&d, p), "p={} r={}", p, r);
            }
        }
    }

    #[test]
    fn trace_sequence_has_linear_complexity_two() {
        for &(p, r) in &[(29u64, 5u64), (23, 3), (101u64, 17u64)] {
            let pl = plant_sequence(p, r, F2(3, 5)).unwrap();
            let (mut a, mut b) = (0, 0);
            assert_eq!(bm_period(&pl.seq, p, &mut a, &mut b), 2, "p={} r={}", p, r);
        }
        // a larger one: p = 16381-ish supersingular style
        let p = 1019u64; // 1019 % 3 == 2, p+1 = 1020 = 2^2*3*5*17
        assert_eq!(p % 3, 2);
        for r in [17u64, 5] {
            let pl = plant_sequence(p, r, F2(11, 2)).unwrap();
            let (mut a, mut b) = (0, 0);
            let l = bm_period(&pl.seq, p, &mut a, &mut b);
            assert_eq!(l, 2);
            assert_eq!(mult_order(p, r), 2);
        }
    }

    #[test]
    fn hensel_lift_on_lifted_curve() {
        let (c, g) = e101();
        let mut cnt = Counts::default();
        let mut cur: Pt = None;
        for _ in 1..40 {
            cur = c.add(cur, Some(g), &mut cnt).unwrap();
            let q = cur.unwrap();
            for u in [0u64, 1, 57] {
                let l = hensel_lift(&c, q, u, &mut cnt).unwrap();
                assert!(on_lifted(&c, l));
                assert_eq!((l.0 % 101) as u64, q.0);
                assert_eq!((l.1 % 101) as u64, q.1);
            }
        }
    }

    #[test]
    fn tau0_matches_h110_reference() {
        // Reference values from research/p2_lift_defect_20261101/run.py --quick (E101, G = first_point).
        let (c, g) = e101();
        assert_eq!(g, (0, 3));
        let tau0 = [23u64, 46, 69, 1, 32, 55, 67, 94, 77, 46];
        let tau1 = [20u64, 23, 99];
        let tu = [75u64, 2, 28];
        let mut cnt = Counts::default();
        let mut cur: Pt = None;
        for k in 0..10 {
            cur = c.add(cur, Some(g), &mut cnt).unwrap();
            let rec = section_record(&c, cur.unwrap(), &mut cnt).unwrap();
            assert_eq!(rec.tau0, tau0[k], "tau0 k={}", k + 1);
            if k < 3 {
                assert_eq!(rec.tau1, tau1[k]);
                assert_eq!(rec.torsion_u, tu[k]);
            }
        }
        assert_eq!(cnt.lifts, 40);
    }

    #[test]
    fn aa_invariance_e101() {
        let (c, g) = e101();
        let mut ref_lc: Vec<(usize, u64)> = vec![];
        let m = mult_order(c.p, c.r);
        for (i, mult) in [1u64, 2, 3, 5].iter().enumerate() {
            let gm = c.mul(*mult, Some(g)).unwrap().unwrap();
            let (s, _) = gen_sequences(&c, gm).unwrap();
            let mut lc = vec![];
            for o in 0..4 {
                let r = run_bm(pick(&s, o), c.p);
                lc.push((r.l, derive_c(r.l, r.mean_nonzero, m).unwrap()));
            }
            if i == 0 {
                ref_lc = lc;
            } else {
                assert_eq!(lc, ref_lc, "multiplier {}", mult);
            }
        }
    }
}
