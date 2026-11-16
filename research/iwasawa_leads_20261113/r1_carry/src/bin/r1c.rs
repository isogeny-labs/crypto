//! R1c (PROTOCOL_R1C.md): scale-up and multi-bit leakage of psi. Exact; no timing.
//! R1: does psi(Q) = log(n * L(Q)) / (n p) mod p leak the discrete log? See PROTOCOL.md.
//! Exact toy computation; no timing is measured.

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 { self.next() % m }
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d * d <= n { if n % d == 0 { return false; } d += 1; }
    true
}

fn md(a: i128, m: i128) -> i128 { ((a % m) + m) % m }

fn inv(a: i128, m: i128) -> i128 {
    let (mut r0, mut r1) = (md(a, m), m);
    let (mut s0, mut s1) = (1i128, 0i128);
    while r1 != 0 { let q = r0 / r1; (r0, r1) = (r1, r0 - q * r1); (s0, s1) = (s1, s0 - q * s1); }
    assert_eq!(r0, 1, "non-invertible");
    md(s0, m)
}

#[derive(Clone)]
struct Curve { p: i128, a: i128, b: i128, n: i128 }

fn rhs(c: &Curve, x: i128, m: i128) -> i128 { md(x * x % m * x + c.a * x + c.b, m) }

fn count(p: i128, a: i128, b: i128, nsq: &[u32]) -> i128 {
    let mut n: i128 = 1;
    for x in 0..p { n += nsq[md(x * x % p * x + a * x + b, p) as usize] as i128; }
    n
}

// affine addition over F_p, None = point at infinity
type Pt = Option<(i128, i128)>;
fn add(c: &Curve, pp: Pt, qq: Pt) -> Pt {
    let p = c.p;
    match (pp, qq) {
        (None, q) => q,
        (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let l = if x1 == x2 {
                if md(y1 + y2, p) == 0 { return None; }
                md((3 * x1 * x1 + c.a) % p * inv(2 * y1, p), p)
            } else { md((y2 - y1) * inv(x2 - x1, p), p) };
            let x3 = md(l * l - x1 - x2, p);
            Some((x3, md(l * (x1 - x3) - y1, p)))
        }
    }
}

// homogeneous projective arithmetic mod M = p^2
type P3 = (i128, i128, i128);
fn padd(a: i128, m: i128, p1: P3, p2: P3) -> P3 {
    let _ = a;
    let (x1, y1, z1) = p1; let (x2, y2, z2) = p2;
    let u = md(y2 * z1 - y1 * z2, m);
    let v = md(x2 * z1 - x1 * z2, m);
    let v2 = v * v % m; let v3 = v2 * v % m;
    let z1z2 = z1 * z2 % m;
    let aa = md(u * u % m * z1z2 - v3 - 2 * v2 % m * x1 % m * z2, m);
    let x3 = v * aa % m;
    let y3 = md(u * md(v2 * x1 % m * z2 - aa, m) - v3 * y1 % m * z2, m);
    let z3 = v3 * z1z2 % m;
    (x3, y3, z3)
}
fn pdbl(a: i128, m: i128, p1: P3) -> P3 {
    let (x, y, z) = p1;
    let w = md(a * z % m * z + 3 * x % m * x, m);
    let s = y * z % m;
    let bb = x * y % m * s % m;
    let h = md(w * w - 8 * bb, m);
    let x3 = 2 * h % m * s % m;
    let y3 = md(w * md(4 * bb - h, m) - 8 * (y * y % m) % m * (s * s % m), m);
    let z3 = 8 * (s * s % m) % m * s % m;
    (x3, y3, z3)
}

fn psi(c: &Curve, q: (i128, i128)) -> i128 {
    let p = c.p; let m = p * p;
    let (x, y) = q;
    let f = rhs(c, x, m);
    let d = md(f - y * y, m);
    assert_eq!(d % p, 0);
    let t = md((d / p) * inv(2 * y, p), p);
    let yl = y + p * t;
    let l: P3 = (x, yl, 1);
    let nn = c.n;
    let bits = 128 - (nn as u128).leading_zeros();
    let mut r = l;
    for i in (0..bits - 1).rev() {
        r = pdbl(c.a, m, r);
        if (nn >> i) & 1 == 1 { r = padd(c.a, m, r, l); }
    }
    let (xx, yy, zz) = r;
    assert_eq!(zz % p, 0, "n*L not in kernel of reduction");
    assert_eq!(xx % p, 0);
    let tt = md(-xx * inv(yy, m), m);
    assert_eq!(tt % p, 0);
    let tp = tt / p;
    if nn == p { tp } else { md(tp * inv(nn % p, p), p) }
}

fn find_curve(rng: &mut Rng, anomalous: bool, lo: i128) -> (Curve, (i128, i128)) {
    loop {
        let p = loop { let c = lo + rng.below(lo as u64) as i128; if is_prime(c as u64) { break c; } };
        let mut nsq = vec![0u32; p as usize];
        let mut root = vec![-1i128; p as usize];
        for y in 0..p { let v = (y * y % p) as usize; nsq[v] += 1; root[v] = y; }
        for _ in 0..4000 {
            let a = rng.below(p as u64) as i128; let b = rng.below(p as u64) as i128;
            if md(4 * a * a % p * a + 27 * b * b, p) == 0 { continue; }
            let n = count(p, a, b, &nsq);
            let ok = if anomalous { n == p } else { n != p && is_prime(n as u64) };
            if !ok { continue; }
            let c = Curve { p, a, b, n };
            loop {
                let x = rng.below(p as u64) as i128;
                let f = rhs(&c, x, p);
                if f != 0 && nsq[f as usize] > 0 { return (c, (x, root[f as usize])); }
            }
        }
    }
}


// ---- R1c ----

fn mul_pt(c: &Curve, g: (i128, i128), k: i128) -> Pt {
    let mut r: Pt = None; let mut base: Pt = Some(g); let mut e = k;
    while e > 0 { if e & 1 == 1 { r = add(c, r, base); } base = add(c, base, base); e >>= 1; }
    r
}

/// psi table over x = 1..n-1, multi-threaded (x = 0 slot unused).
fn psi_table(c: &Curve, g: (i128, i128), threads: usize) -> Vec<u32> {
    let n = c.n as usize;
    let mut t = vec![0u32; n];
    let chunk = (n + threads - 1) / threads;
    std::thread::scope(|s| {
        for (ci, slice) in t.chunks_mut(chunk).enumerate() {
            let c = c.clone();
            s.spawn(move || {
                let x0 = ci * chunk;
                // q holds [x]P for the current x; chunk 0 starts at x = 0 (slot unused)
                let mut q: Pt = if x0 == 0 { None } else { mul_pt(&c, g, x0 as i128 - 1) };
                for (off, slot) in slice.iter_mut().enumerate() {
                    let x = x0 + off;
                    if x == 0 { continue; }
                    q = add(&c, q, Some(g));
                    *slot = psi(&c, q.expect("hit O early")) as u32;
                }
            });
        }
    });
    t
}

fn bitlen(v: u64) -> u32 { 64 - v.leading_zeros() }

fn masks(bits: u32) -> Vec<u64> {
    let mut m = Vec::new();
    for i in 0..bits { m.push(1u64 << i); }
    for i in 0..bits { for j in i + 1..bits { m.push((1u64 << i) | (1u64 << j)); } }
    m
}

/// parity bitsets over the half range h = 1..=hmax, indexed by x-1.
fn parity_sets(vals: &(dyn Fn(usize) -> u64 + Sync), hmax: usize, ms: &[u64], threads: usize) -> Vec<Vec<u64>> {
    let words = (hmax + 63) / 64;
    let mut out = vec![vec![0u64; words]; ms.len()];
    let chunk = (ms.len() + threads - 1) / threads;
    std::thread::scope(|s| {
        for (ci, part) in out.chunks_mut(chunk).enumerate() {
            s.spawn(move || {
                for (k, set) in part.iter_mut().enumerate() {
                    let m = ms[ci * chunk + k];
                    for x in 1..=hmax {
                        if (vals(x) & m).count_ones() & 1 == 1 { set[(x - 1) / 64] |= 1u64 << ((x - 1) % 64); }
                    }
                }
            });
        }
    });
    out
}

/// S6: max chi^2 over (x-mask, value-mask) pairs; returns (T, max, ia, ib).
fn s6(xs: &[Vec<u64>], vs: &[Vec<u64>], hmax: usize, threads: usize) -> (usize, f64, usize, usize) {
    let h = hmax as f64;
    let chunk = (xs.len() + threads - 1) / threads;
    let best = std::sync::Mutex::new((0.0f64, 0usize, 0usize));
    std::thread::scope(|s| {
        for ci in 0..threads {
            let best = &best;
            s.spawn(move || {
                let mut loc = (0.0f64, 0usize, 0usize);
                for ia in ci * chunk..((ci + 1) * chunk).min(xs.len()) {
                    let na: u64 = xs[ia].iter().map(|w| w.count_ones() as u64).sum();
                    for (ib, vb) in vs.iter().enumerate() {
                        let nb: u64 = vb.iter().map(|w| w.count_ones() as u64).sum();
                        if na == 0 || nb == 0 || na == hmax as u64 || nb == hmax as u64 { continue; }
                        let mut n11 = 0u64;
                        for (wa, wb) in xs[ia].iter().zip(vb.iter()) { n11 += (wa & wb).count_ones() as u64; }
                        // 2x2 independence chi^2 with the observed marginals
                        let (a, b) = (na as f64, nb as f64);
                        let o = [n11 as f64, a - n11 as f64, b - n11 as f64, h - a - b + n11 as f64];
                        let e = [a * b / h, a * (h - b) / h, (h - a) * b / h, (h - a) * (h - b) / h];
                        let chi: f64 = o.iter().zip(e.iter()).map(|(o, e)| (o - e) * (o - e) / e).sum();
                        if chi > loc.0 { loc = (chi, ia, ib); }
                    }
                }
                let mut b = best.lock().unwrap(); if loc.0 > b.0 { *b = loc; }
            });
        }
    });
    let b = best.into_inner().unwrap();
    (xs.len() * vs.len(), b.0, b.1, b.2)
}

/// S7: largest bin of psi_{x+k} - psi_x mod p over k = 1..16 with x, x+k in the half range.
fn s7(t: &[u32], hmax: usize, p: i128) -> u32 {
    let mut hist = vec![0u32; p as usize];
    let mut best = 0u32;
    for k in 1..=16usize {
        for x in 1..=hmax - k {
            let d = ((t[x + k] as i128 - t[x] as i128) % p + p) % p;
            let h = &mut hist[d as usize]; *h += 1; if *h > best { best = *h; }
        }
        for x in 1..=hmax - k { let d = ((t[x + k] as i128 - t[x] as i128) % p + p) % p; hist[d as usize] = 0; }
    }
    best
}

fn s3(t: &[u32], n: i128, p: i128, rng: &mut Rng) -> u64 {
    let mut zero = 0u64; let mut done = 0;
    while done < 1_000_000 {
        let i = 1 + rng.below((n - 1) as u64) as i128;
        let j = 1 + rng.below((n - 1) as u64) as i128;
        let k = (i + j) % n; if k == 0 { continue; }
        let c = ((t[k as usize] as i128 - t[i as usize] as i128 - t[j as usize] as i128) % p + p) % p;
        if c == 0 { zero += 1; }
        done += 1;
    }
    zero
}

fn analyse(label: &str, c: &Curve, t: &[u32], xs: &[Vec<u64>], xm: &[u64], vm: &[u64], hmax: usize, threads: usize, extra: &str) {
    let tv = |x: usize| t[x] as u64;
    let vs = parity_sets(&tv, hmax, vm, threads);
    let (tt, chi, ia, ib) = s6(xs, &vs, hmax, threads);
    let l7 = s7(t, hmax, c.p);
    println!("{{\"arm\":\"{}\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"H\":{},\"S6_T\":{},\"S6_max_chi2\":{:.2},\"S6_xmask\":{},\"S6_vmask\":{},\"S7_max_bin\":{}{}}}",
        label, c.p, c.a, c.b, c.n, hmax, tt, chi, xm[ia], vm[ib], l7, extra);
}

fn main() {
    let seed: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20261115);
    let ncurves: usize = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(6);
    let lo: i128 = std::env::args().nth(3).map(|s| s.parse().unwrap()).unwrap_or(1 << 21);
    let threads = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
    let mut rng = Rng(seed);
    for ci in 0..ncurves {
        let (c, g) = find_curve(&mut rng, false, lo);
        let t = psi_table(&c, g, threads);
        // spot-check the parallel table against the sequential definition
        let mut q: Pt = None;
        for x in 1..c.n.min(2000) { q = add(&c, q, Some(g)); assert_eq!(t[x as usize] as i128, psi(&c, q.unwrap()), "table mismatch at x={}", x); }
        let q2 = mul_pt(&c, g, c.n - 1).unwrap();
        assert_eq!(t[(c.n - 1) as usize] as i128, psi(&c, q2), "table mismatch at n-1");
        assert!(mul_pt(&c, g, c.n).is_none(), "order check failed");
        for x in 1..c.n.min(5000) { assert_eq!((t[x as usize] as i128 + t[(c.n - x) as usize] as i128) % c.p, 0, "psi not odd"); }
        let hmax = ((c.n - 1) / 2) as usize;
        let xm = masks(bitlen((c.n - 1) as u64));
        let vm = masks(bitlen((c.p - 1) as u64));
        let xid = |x: usize| x as u64;
        let xs = parity_sets(&xid, hmax, &xm, threads);
        let z = s3(&t, c.n, c.p, &mut rng);
        analyse("test", &c, &t, &xs, &xm, &vm, hmax, threads, &format!(",\"curve\":{},\"G\":[{},{}],\"S3_zero_of_1e6\":{}", ci, g.0, g.1, z));
        // negative control: seeded permutation of x over 1..n-1
        let mut perm = t.clone();
        for i in (2..c.n as usize).rev() { let j = 1 + rng.below(i as u64) as usize; perm.swap(i, j); }
        analyse("shuffled", &c, &perm, &xs, &xm, &vm, hmax, threads, &format!(",\"curve\":{}", ci));
        // positive P1: affine plant
        let alpha = 1 + rng.below((c.p - 1) as u64) as i128; let beta = rng.below(c.p as u64) as i128;
        let aff: Vec<u32> = (0..c.n).map(|x| ((alpha * x + beta) % c.p) as u32).collect();
        analyse("positive_affine", &c, &aff, &xs, &xm, &vm, hmax, threads, &format!(",\"curve\":{}", ci));
        // positive P2: weak pairwise plant on bit 0 at x = 0 mod 100
        let mut pl = t.clone();
        for x in (100..c.n as usize).step_by(100) {
            let bit = ((x as u64) ^ ((x as u64) >> 1)) & 1;
            pl[x] = (pl[x] & !1) | bit as u32;
        }
        analyse("positive_weakbit", &c, &pl, &xs, &xm, &vm, hmax, threads, &format!(",\"curve\":{},\"planted_xmask\":3,\"planted_vmask\":1", ci));
    }
}
