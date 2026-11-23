//! Toy prime-field and binary-field curves, exact point counting and <G> enumeration.
use crate::util::*;

pub const INF: u32 = u32::MAX;

// ======================= prime field =======================
#[derive(Clone)]
pub struct PCurve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64, // #E(F_p)
    pub n: u64,     // prime order of <G>
    pub h: u64,
    pub g: (u64, u64),
}

pub fn qr_table(p: u64) -> Vec<i8> {
    let mut t = vec![-1i8; p as usize];
    t[0] = 0;
    for x in 1..=(p / 2) {
        t[(x * x % p) as usize] = 1;
    }
    t
}
pub fn count_prime(p: u64, a: u64, b: u64, qr: &[i8]) -> u64 {
    let mut s: i64 = 0;
    for x in 0..p {
        let r = (x * x % p * x + a * x + b) % p;
        s += qr[r as usize] as i64;
    }
    (p as i64 + 1 + s) as u64
}
pub fn inv_p(x: u64, p: u64) -> u64 {
    powmod(x, p - 2, p)
}
pub type PPt = Option<(u64, u64)>;
pub fn padd(c: &PCurve, p1: PPt, p2: PPt) -> PPt {
    let p = c.p;
    match (p1, p2) {
        (None, q) | (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let l = if x1 == x2 {
                if (y1 + y2) % p == 0 {
                    return None;
                }
                (3 * x1 % p * x1 + c.a) % p * inv_p(2 * y1 % p, p) % p
            } else {
                (y2 + p - y1) % p * inv_p((x2 + p - x1) % p, p) % p
            };
            let x3 = (l * l % p + 2 * p - x1 - x2) % p;
            let y3 = (l * ((x1 + p - x3) % p) % p + p - y1) % p;
            Some((x3, y3))
        }
    }
}
pub fn pmul(c: &PCurve, mut k: u64, mut q: PPt) -> PPt {
    let mut r = None;
    while k > 0 {
        if k & 1 == 1 {
            r = padd(c, r, q);
        }
        q = padd(c, q, q);
        k >>= 1;
    }
    r
}
/// first point by x with y <= (p-1)/2
pub fn first_point(p: u64, a: u64, b: u64, start_x: u64) -> (u64, u64) {
    let mut x = start_x;
    loop {
        let r = (x * x % p * x + a * x + b) % p;
        if let Some(y) = sqrt_mod(r, p) {
            if y != 0 {
                let y = if y > (p - 1) / 2 { p - y } else { y };
                return (x, y);
            }
        }
        x += 1;
    }
}
/// Enumerate kG for k = 0..n-1 (index 0 = infinity).
pub fn enum_prime(c: &PCurve) -> (Vec<u32>, Vec<u32>) {
    let n = c.n as usize;
    let (mut xs, mut ys) = (vec![INF; n], vec![INF; n]);
    let mut cur: PPt = Some(c.g);
    for k in 1..n {
        let (x, y) = cur.expect("order too small");
        xs[k] = x as u32;
        ys[k] = y as u32;
        cur = padd(c, cur, Some(c.g));
    }
    assert!(cur.is_none(), "nG != O");
    (xs, ys)
}
pub fn smallest_prime_3mod4(lo: u64) -> u64 {
    let mut p = lo;
    while !(p % 4 == 3 && is_prime(p)) {
        p += 1;
    }
    p
}
/// Family P: first `count` prime-order curves y^2 = x^3 + Ax + B, (A,B) lexicographic, B fastest.
pub fn family_p(bits: u32, count: usize) -> Vec<PCurve> {
    let p = smallest_prime_3mod4((1u64 << bits) + 1);
    let qr = qr_table(p);
    let mut out = vec![];
    'outer: for a in 1..p {
        for b in 1..p {
            if (4 * a % p * a % p * a + 27 * b % p * b) % p == 0 {
                continue;
            }
            let ord = count_prime(p, a, b, &qr);
            if is_prime(ord) && ord != p {
                let g = first_point(p, a, b, 0);
                out.push(PCurve { p, a, b, order: ord, n: ord, h: 1, g });
                if out.len() == count {
                    break 'outer;
                }
            }
        }
    }
    out
}
/// Family P4: y^2 = x(x-1)(x-e3), #E = 4n, n odd prime > 4.
/// Short Weierstrass form via x -> X - s with s = (1+e3)/3.
pub struct P4Curve {
    pub c: PCurve,
    pub e: [u64; 3], // roots in the short-Weierstrass x coordinate
}
pub fn family_p4(bits: u32) -> P4Curve {
    let p = smallest_prime_3mod4((1u64 << bits) + 1);
    let qr = qr_table(p);
    let inv3 = inv_p(3, p);
    for e3 in 2..p {
        // roots 0,1,e3 ; shift s = (0+1+e3)/3 so that roots become -s, 1-s, e3-s (sum 0)
        let s = (1 + e3) % p * inv3 % p;
        let r = [(p - s) % p, (1 + p - s) % p, (e3 + p - s) % p];
        if r[0] == r[1] || r[0] == r[2] || r[1] == r[2] {
            continue;
        }
        // A = r0 r1 + r0 r2 + r1 r2 ; B = - r0 r1 r2
        let a = (r[0] * r[1] % p + r[0] * r[2] % p + r[1] * r[2] % p) % p;
        let b = (p - r[0] * r[1] % p * r[2] % p) % p;
        let ord = count_prime(p, a, b, &qr);
        if ord % 4 == 0 && ord % 8 != 0 && is_prime(ord / 4) && ord / 4 > 4 {
            let n = ord / 4;
            let mut x0 = 0;
            loop {
                let p0 = first_point(p, a, b, x0);
                let mut c = PCurve { p, a, b, order: ord, n, h: 4, g: p0 };
                if let Some(g) = pmul(&c, 4, Some(p0)) {
                    if pmul(&c, n, Some(g)).is_none() {
                        c.g = g;
                        return P4Curve { c, e: r };
                    }
                }
                x0 = p0.0 + 1;
            }
        }
    }
    unreachable!()
}

// ======================= binary field =======================
#[derive(Clone)]
pub struct F2m {
    pub m: u32,
    pub f: u64, // modulus including x^m
    pub tr_mask: u32,
}
impl F2m {
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        let mut r: u64 = 0;
        let (a, mut b) = (a as u64, b);
        let mut i = 0;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a << i;
            }
            b >>= 1;
            i += 1;
        }
        for d in (self.m as usize..64).rev() {
            if r >> d & 1 == 1 {
                r ^= self.f << (d - self.m as usize);
            }
        }
        r as u32
    }
    pub fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    pub fn pow(&self, mut a: u32, mut e: u64) -> u32 {
        let mut r = 1u32;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            e >>= 1;
        }
        r
    }
    pub fn inv(&self, a: u32) -> u32 {
        self.pow(a, (1u64 << self.m) - 2)
    }
    pub fn tr(&self, a: u32) -> u32 {
        (a & self.tr_mask).count_ones() & 1
    }
    fn tr_slow(&self, a: u32) -> u32 {
        let mut s = a;
        let mut t = a;
        for _ in 1..self.m {
            t = self.sq(t);
            s ^= t;
        }
        assert!(s <= 1);
        s
    }
    /// solve z^2 + z = c (m odd), requires Tr(c)=0
    pub fn half_trace(&self, c: u32) -> u32 {
        let mut z = 0;
        let mut t = c;
        for _ in 0..=(self.m - 1) / 2 {
            z ^= t;
            t = self.sq(self.sq(t));
        }
        z
    }
    pub fn new(m: u32) -> F2m {
        // smallest irreducible trinomial, else pentanomial
        let mut cands: Vec<u64> = vec![];
        for k in 1..m {
            cands.push((1u64 << m) | (1 << k) | 1);
        }
        let mut pent: Vec<u64> = vec![];
        for k3 in 3..m {
            for k2 in 2..k3 {
                for k1 in 1..k2 {
                    pent.push((1u64 << m) | (1 << k3) | (1 << k2) | (1 << k1) | 1);
                }
            }
        }
        pent.sort();
        cands.extend(pent);
        for f in cands {
            let fld = F2m { m, f, tr_mask: 0 };
            // irreducible iff x^(2^m) = x mod f and no root in GF(2) (m prime)
            let mut t = 2u32; // x
            for _ in 0..m {
                t = fld.sq(t);
            }
            let roots = (f & 1 == 0) || (f.count_ones() % 2 == 0);
            if t == 2 && !roots {
                let mut fld = fld;
                let mut mask = 0;
                for i in 0..m {
                    if fld.tr_slow(1 << i) == 1 {
                        mask |= 1 << i;
                    }
                }
                fld.tr_mask = mask;
                return fld;
            }
        }
        unreachable!()
    }
}
#[derive(Clone)]
pub struct BCurve {
    pub k: F2m,
    pub a: u32,
    pub b: u32,
    pub order: u64,
    pub n: u64,
    pub h: u64,
    pub g: (u32, u32),
    pub label: String,
}
pub type BPt = Option<(u32, u32)>;
pub fn badd(c: &BCurve, p1: BPt, p2: BPt) -> BPt {
    let k = &c.k;
    match (p1, p2) {
        (None, q) | (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 {
                if y1 ^ y2 == x1 || x1 == 0 {
                    // P2 = -P1 (or doubling of a 2-torsion point)
                    if y1 ^ y2 == x1 {
                        return None;
                    }
                }
                if y1 != y2 {
                    return None;
                }
                // doubling
                if x1 == 0 {
                    return None;
                }
                let l = x1 ^ k.mul(y1, k.inv(x1));
                let x3 = k.sq(l) ^ l ^ c.a;
                let y3 = k.sq(x1) ^ k.mul(l ^ 1, x3);
                return Some((x3, y3));
            }
            let l = k.mul(y1 ^ y2, k.inv(x1 ^ x2));
            let x3 = k.sq(l) ^ l ^ x1 ^ x2 ^ c.a;
            let y3 = k.mul(l, x1 ^ x3) ^ x3 ^ y1;
            Some((x3, y3))
        }
    }
}
pub fn bmul(c: &BCurve, mut e: u64, mut q: BPt) -> BPt {
    let mut r = None;
    while e > 0 {
        if e & 1 == 1 {
            r = badd(c, r, q);
        }
        q = badd(c, q, q);
        e >>= 1;
    }
    r
}
pub fn count_binary(k: &F2m, a: u32, b: u32) -> u64 {
    // O + (x=0: one point) + sum_{x!=0} 2[Tr(x + a + b/x^2)=0]
    let mut cnt: u64 = 2;
    for x in 1..(1u32 << k.m) {
        let xi = k.inv(x);
        let c = x ^ a ^ k.mul(b, k.sq(xi));
        if k.tr(c) == 0 {
            cnt += 2;
        }
    }
    cnt
}
/// points with this x (x != 0): Some((y0, y1))
pub fn bpoints_at(k: &F2m, a: u32, b: u32, x: u32) -> Option<(u32, u32)> {
    let xi = k.inv(x);
    let c = x ^ a ^ k.mul(b, k.sq(xi));
    if k.tr(c) != 0 {
        return None;
    }
    let z = k.half_trace(c);
    let y = k.mul(x, z);
    Some((y, y ^ x))
}
pub fn make_bcurve(k: &F2m, a: u32, b: u32, h: u64, label: &str) -> Option<BCurve> {
    let order = count_binary(k, a, b);
    if order % h != 0 || !is_prime(order / h) {
        return None;
    }
    let n = order / h;
    let mut c = BCurve { k: k.clone(), a, b, order, n, h, g: (0, 0), label: label.to_string() };
    for x in 1..(1u32 << k.m) {
        if let Some((y, _)) = bpoints_at(k, a, b, x) {
            if let Some(g) = bmul(&c, h, Some((x, y))) {
                if bmul(&c, n, Some(g)).is_none() {
                    c.g = g;
                    return Some(c);
                }
            }
        }
    }
    None
}
pub fn family_b(m: u32) -> Vec<BCurve> {
    let k = F2m::new(m);
    let mut out = vec![];
    if let Some(c) = make_bcurve(&k, 0, 1, 4, &format!("koblitz_a0_m{m}")) {
        out.push(c);
    }
    if let Some(c) = make_bcurve(&k, 1, 1, 2, &format!("koblitz_a1_m{m}")) {
        out.push(c);
    }
    for b in 2..(1u32 << m) {
        if let Some(c) = make_bcurve(&k, 1, b, 2, &format!("randb_a1_m{m}_b{b}")) {
            out.push(c);
            break;
        }
    }
    out
}
pub fn enum_binary(c: &BCurve) -> (Vec<u32>, Vec<u32>) {
    let n = c.n as usize;
    let (mut xs, mut ys) = (vec![INF; n], vec![INF; n]);
    let mut cur: BPt = Some(c.g);
    for k in 1..n {
        let (x, y) = cur.expect("order too small");
        xs[k] = x;
        ys[k] = y;
        cur = badd(c, cur, Some(c.g));
    }
    assert!(cur.is_none(), "nG != O");
    (xs, ys)
}
