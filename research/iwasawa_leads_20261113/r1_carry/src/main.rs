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

struct Stats { s1: u64, s2: u64, s3_zero: u64, s3_chi2: f64 }

fn stats(psi_t: &[i128], p: i128, n: i128, rng: &mut Rng) -> Stats {
    let p_u = p as usize;
    let a1 = psi_t[1];
    let mut s1 = 0u64;
    for x in 1..n { if psi_t[x as usize] == md(x * a1, p) { s1 += 1; } }
    // S2: max over alpha, beta of #{x : psi_x == alpha x + beta}
    let mut hist = vec![0u32; p_u];
    let mut s2 = 0u32;
    for alpha in 0..p {
        let mut ax = 0i128; // alpha * x mod p
        for x in 1..n {
            ax += alpha; if ax >= p { ax -= p; }
            let mut v = psi_t[x as usize] - ax; if v < 0 { v += p; }
            let h = &mut hist[v as usize]; *h += 1; if *h > s2 { s2 = *h; }
        }
        let mut ax = 0i128;
        for x in 1..n {
            ax += alpha; if ax >= p { ax -= p; }
            let mut v = psi_t[x as usize] - ax; if v < 0 { v += p; }
            hist[v as usize] = 0;
        }
    }
    // S3 cocycle
    let pairs = 20000;
    let mut zero = 0u64; let mut bins = [0u64; 16]; let mut done = 0;
    while done < pairs {
        let i = 1 + rng.below((n - 1) as u64) as i128;
        let j = 1 + rng.below((n - 1) as u64) as i128;
        let k = (i + j) % n; if k == 0 { continue; }
        let c = md(psi_t[k as usize] - psi_t[i as usize] - psi_t[j as usize], p);
        if c == 0 { zero += 1; }
        bins[((c * 16) / p) as usize] += 1;
        done += 1;
    }
    // expected counts per bin (bins are nearly equal width)
    let mut chi2 = 0.0;
    for bi in 0..16i128 {
        let lo = (bi * p + 15) / 16; let hi = ((bi + 1) * p + 15) / 16;
        let e = pairs as f64 * (hi - lo) as f64 / p as f64;
        let o = bins[bi as usize] as f64; chi2 += (o - e) * (o - e) / e;
    }
    Stats { s1, s2: s2 as u64, s3_zero: zero, s3_chi2: chi2 }
}

fn find_curve(rng: &mut Rng, anomalous: bool) -> (Curve, (i128, i128)) {
    loop {
        let p = loop { let c = (1 << 13) + rng.below(1 << 13) as i128; if is_prime(c as u64) { break c; } };
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

fn run(label: &str, c: &Curve, g: (i128, i128), rng: &mut Rng) {
    let n = c.n;
    let mut table = vec![0i128; n as usize];
    let mut q: Pt = None;
    for x in 1..n {
        q = add(c, q, Some(g));
        table[x as usize] = psi(c, q.expect("hit O early"));
    }
    assert!(add(c, q, Some(g)).is_none(), "order check failed");
    let s = stats(&table, c.p, n, rng);
    println!("{{\"arm\":\"{}\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"G\":[{},{}],\"S1\":{},\"S2\":{},\"S3_zero\":{},\"S3_chi2\":{:.2}}}",
        label, c.p, c.a, c.b, n, g.0, g.1, s.s1, s.s2, s.s3_zero, s.s3_chi2);
    if label == "test" {
        // negative control: seeded permutation of x
        let mut perm = table.clone();
        for i in (2..n as usize).rev() { let j = 1 + rng.below(i as u64) as usize; perm.swap(i, j); }
        let s = stats(&perm, c.p, n, rng);
        println!("{{\"arm\":\"shuffled\",\"p\":{},\"n\":{},\"S1\":{},\"S2\":{},\"S3_zero\":{},\"S3_chi2\":{:.2}}}",
            c.p, n, s.s1, s.s2, s.s3_zero, s.s3_chi2);
    }
}

fn main() {
    let seed: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20261113);
    let ntest: usize = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(12);
    let nanom: usize = std::env::args().nth(3).map(|s| s.parse().unwrap()).unwrap_or(4);
    let mut rng = Rng(seed);
    for _ in 0..nanom { let (c, g) = find_curve(&mut rng, true); run("anomalous", &c, g, &mut rng); }
    for _ in 0..ntest { let (c, g) = find_curve(&mut rng, false); run("test", &c, g, &mut rng); }
}
