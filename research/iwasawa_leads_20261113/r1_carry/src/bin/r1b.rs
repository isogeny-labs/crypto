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

// ---- R1b (PROTOCOL_R1B.md): non-affine leakage ----

fn table(c: &Curve, g: (i128, i128)) -> (Vec<i128>, Vec<(i128, i128)>) {
    let n = c.n;
    let mut t = vec![0i128; n as usize];
    let mut pts = vec![(0i128, 0i128); n as usize];
    let mut q: Pt = None;
    for x in 1..n {
        q = add(c, q, Some(g));
        let qq = q.expect("hit O early");
        pts[x as usize] = qq;
        t[x as usize] = psi(c, qq);
    }
    assert!(add(c, q, Some(g)).is_none(), "order check failed");
    (t, pts)
}

fn bitlen(v: i128) -> u32 { 128 - (v as u128).leading_zeros() }

/// S4: max chi^2 (1 dof) over (bit i of x, bit j of value) 2x2 tables, x = 1..n-1.
fn s4(vals: &[i128], n: i128, p: i128) -> f64 {
    let (bx, bp) = (bitlen(n - 1), bitlen(p - 1));
    let tot = (n - 1) as f64;
    let mut best = 0.0f64;
    for i in 0..bx {
        for j in 0..bp {
            let mut c = [[0f64; 2]; 2];
            for x in 1..n {
                c[((x >> i) & 1) as usize][((vals[x as usize] >> j) & 1) as usize] += 1.0;
            }
            let r = [c[0][0] + c[0][1], c[1][0] + c[1][1]];
            let k = [c[0][0] + c[1][0], c[0][1] + c[1][1]];
            if r[0] == 0.0 || r[1] == 0.0 || k[0] == 0.0 || k[1] == 0.0 { continue; }
            let mut chi = 0.0;
            for a in 0..2 { for b in 0..2 { let e = r[a] * k[b] / tot; chi += (c[a][b] - e).powi(2) / e; } }
            if chi > best { best = chi; }
        }
    }
    best
}

/// Degree of the Newton interpolating polynomial of f on distinct nodes xs over F_p.
fn interp_degree(xs: &[i128], fs: &[i128], p: i128) -> usize {
    let m = xs.len();
    let mut d: Vec<i128> = fs.to_vec();
    let mut deg = 0usize;
    if d[0] != 0 { deg = 0; }
    for k in 1..m {
        for i in (k..m).rev() {
            d[i] = md((d[i] - d[i - 1]) * inv(xs[i] - xs[i - k], p), p);
        }
        if d[k] != 0 { deg = k; }
    }
    deg
}

fn main() {
    let seed: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20261114);
    let mut rng = Rng(seed);
    // S4 positive control: anomalous, psi(Q)/psi(P) == x
    for _ in 0..4 {
        let (c, g) = find_curve(&mut rng, true, 1 << 13);
        let (t, _) = table(&c, g);
        let ip = inv(t[1], c.p);
        let norm: Vec<i128> = t.iter().map(|v| md(v * ip, c.p)).collect();
        for x in 1..c.n { assert_eq!(norm[x as usize], x % c.p); }
        println!("{{\"stat\":\"S4\",\"arm\":\"positive\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"max_chi2\":{:.2}}}", c.p, c.a, c.b, c.n, s4(&norm, c.n, c.p));
    }
    for _ in 0..12 {
        let (c, g) = find_curve(&mut rng, false, 1 << 13);
        let (t, _) = table(&c, g);
        let test = s4(&t, c.n, c.p);
        let mut perm = t.clone();
        for i in (2..c.n as usize).rev() { let j = 1 + rng.below(i as u64) as usize; perm.swap(i, j); }
        let sh = s4(&perm, c.n, c.p);
        println!("{{\"stat\":\"S4\",\"arm\":\"test\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"G\":[{},{}],\"max_chi2\":{:.2},\"shuffled_max_chi2\":{:.2}}}", c.p, c.a, c.b, c.n, g.0, g.1, test, sh);
    }
    for _ in 0..12 {
        let (c, g) = find_curve(&mut rng, false, 1 << 11);
        let (t, pts) = table(&c, g);
        // one representative per +-pair: keep points with y < p/2 (y != 0 since n odd)
        let mut xs = Vec::new(); let mut fs = Vec::new(); let mut cub = Vec::new();
        for x in 1..c.n {
            let (px, py) = pts[x as usize];
            if py <= c.p / 2 {
                xs.push(px);
                fs.push(md(t[x as usize] * inv(py, c.p), c.p));
                cub.push(md(px * px % c.p * px + c.a * px, c.p));
            }
        }
        // oddness check: psi(-Q) == -psi(Q)
        for x in 1..c.n { assert_eq!(md(t[x as usize] + t[(c.n - x) as usize], c.p), 0, "psi not odd"); }
        let m = xs.len();
        let dg = interp_degree(&xs, &fs, c.p);
        let dc = interp_degree(&xs, &cub, c.p);
        println!("{{\"stat\":\"S5\",\"arm\":\"test\",\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"m\":{},\"deg\":{},\"deficit\":{},\"positive_cubic_deg\":{}}}", c.p, c.a, c.b, c.n, m, dg, m - 1 - dg, dc);
    }
}
