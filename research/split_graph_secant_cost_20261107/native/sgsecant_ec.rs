//! Curve arithmetic, SHA-256 (for reproducing the historical ladder selection
//! and for domain-separated deterministic seeding), graph/control construction.
use crate::ops::*;

// ------------------------------------------------------------- SHA-256
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[4 * i..4 * i + 4].try_into().unwrap());
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
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut out = [0u8; 32];
    for i in 0..8 {
        out[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}
/// Same preimage layout as the historical ladder: domain, 0x00, big-endian u64 words.
pub fn hash_words(domain: &str, words: &[u64]) -> [u8; 32] {
    native_n(1);
    let mut b = domain.as_bytes().to_vec();
    b.push(0);
    for w in words {
        b.extend_from_slice(&w.to_be_bytes());
    }
    sha256(&b)
}
pub fn word(b: &[u8]) -> u64 {
    u64::from_be_bytes(b[..8].try_into().unwrap())
}

// ------------------------------------------------------------- curve
pub type Pt = (u64, u64);
#[derive(Clone, Debug)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64, // group order (prime)
    pub points: Vec<Pt>, // affine points, x ascending then y ascending
}
pub fn rhs(c: (u64, u64), p: u64, x: u64) -> u64 {
    let xx = fmul(x, x, p);
    let xxx = fmul(xx, x, p);
    let ax = fmul(c.0, x, p);
    fadd(fadd(xxx, ax, p), c.1, p)
}
pub fn enumerate_points(a: u64, b: u64, p: u64) -> Vec<Pt> {
    let mut roots = vec![Vec::new(); p as usize];
    for y in 0..p {
        let s = fmul(y, y, p);
        roots[s as usize].push(y);
    }
    let mut pts = Vec::new();
    for x in 0..p {
        let v = rhs((a, b), p, x);
        for y in &roots[v as usize] {
            pts.push((x, *y));
        }
    }
    pts
}
pub fn is_prime_td(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        native_n(1);
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
/// Affine addition; None = identity. Charged.
pub fn add(c: &Curve, p1: Option<Pt>, p2: Option<Pt>) -> Option<Pt> {
    let (Some((x, y)), Some((z, w))) = (p1, p2) else { return p1.or(p2) };
    let p = c.p;
    if x == z && fadd(y, w, p) == 0 {
        return None;
    }
    let (num, den) = if x == z {
        let xx = fmul(x, x, p);
        let t = fmul(3, xx, p);
        (fadd(t, c.a, p), fmul(2, y, p))
    } else {
        (fsub(w, y, p), fsub(z, x, p))
    };
    let iv = finv(den, p);
    let m = fmul(num, iv, p);
    let m2 = fmul(m, m, p);
    let rx = fsub(fsub(m2, x, p), z, p);
    let d = fsub(x, rx, p);
    let ry = fsub(fmul(m, d, p), y, p);
    Some((rx, ry))
}
pub fn neg(c: &Curve, q: Option<Pt>) -> Option<Pt> {
    q.map(|(x, y)| (x, fneg(y, c.p)))
}
pub fn smul(c: &Curve, mut k: u64, q: Pt) -> Option<Pt> {
    let mut acc: Option<Pt> = None;
    let mut base = Some(q);
    while k > 0 {
        if k & 1 == 1 {
            acc = add(c, acc, base);
        }
        k >>= 1;
        if k > 0 {
            base = add(c, base, base);
        }
    }
    acc
}

pub struct Spec {
    pub domain: String, // hash domain prefix, e.g. "sgsecant/v1" or the legacy ladder prefix
    pub seed: u64,
    pub curve_cap: u64,
    pub graph_cap: u64,
}
pub struct CurveSearch {
    pub curve: Option<Curve>,
    pub attempts: u64,
    pub statuses: Vec<String>,
}
/// Curve selection with the historical ladder's rules (prime full order >= 23, not anomalous,
/// j not in {0,1728}). Charged to the current phase.
pub fn select_curve(s: &Spec, p: u64, curve_label: &str) -> CurveSearch {
    let mut st = Vec::new();
    for attempt in 0..s.curve_cap {
        let h = hash_words(&format!("{}/{}", s.domain, curve_label), &[s.seed, p, attempt]);
        let a = 1 + word(&h[..8]) % (p - 1);
        let b = 1 + word(&h[8..16]) % (p - 1);
        let aa = fmul(a, a, p);
        let aaa = fmul(aa, a, p);
        let a4 = fmul(4, aaa, p);
        let bb = fmul(b, b, p);
        let b27 = fmul(27, bb, p);
        let disc = fadd(a4, b27, p);
        if disc == 0 {
            st.push("singular".into());
            continue;
        }
        let pts = enumerate_points(a, b, p);
        let n = pts.len() as u64 + 1;
        if !is_prime_td(n) {
            st.push("composite".into());
            continue;
        }
        if n == p {
            st.push("anomalous".into());
            continue;
        }
        if n < 23 {
            st.push("small".into());
            continue;
        }
        // j-invariant 1728*4a^3/(4a^3+27b^2)
        let inv = finv(disc, p);
        let j = fmul(fmul(1728 % p, a4, p), inv, p);
        if j == 0 || j == 1728 % p {
            st.push("exceptional_j".into());
            continue;
        }
        st.push("accepted".into());
        return CurveSearch { curve: Some(Curve { p, a, b, n, points: pts }), attempts: attempt + 1, statuses: st };
    }
    CurveSearch { curve: None, attempts: s.curve_cap, statuses: st }
}

// -------------------------------------------------- univariate helpers (low->high)
pub fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}
pub fn upmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, c) in a.iter().enumerate() {
        for (j, d) in b.iter().enumerate() {
            let t = fmul(*c, *d, p);
            out[i + j] = fadd(out[i + j], t, p);
        }
    }
    trim(out)
}
pub fn membership(xs: &[u64], p: u64) -> Vec<u64> {
    xs.iter().fold(vec![1u64], |acc, x| upmul(&acc, &[fneg(*x, p), 1], p))
}
pub fn ueval(h: &[u64], x: u64, p: u64) -> u64 {
    h.iter().rev().fold(0, |a, c| fadd(fmul(a, x, p), *c, p))
}
pub fn interpolate(pts: &[Pt], p: u64) -> Vec<u64> {
    let mut out = vec![0u64; pts.len()];
    for (i, (xi, yi)) in pts.iter().enumerate() {
        let mut basis = vec![1u64];
        let mut den = 1u64;
        for (j, (xj, _)) in pts.iter().enumerate() {
            if i == j {
                continue;
            }
            basis = upmul(&basis, &[fneg(*xj, p), 1], p);
            den = fmul(den, fsub(*xi, *xj, p), p);
        }
        let iv = finv(den, p);
        let sc = fmul(*yi, iv, p);
        for (j, c) in basis.into_iter().enumerate() {
            let v = fmul(c, sc, p);
            out[j] = fadd(out[j], v, p);
        }
    }
    trim(out)
}

#[derive(Clone, Debug)]
pub struct Base {
    pub name: String,
    pub xs: Vec<u64>,        // abscissae, ascending (2d of them)
    pub h: Vec<u64>,         // oriented ordinate polynomial (low->high)
    pub ys: Vec<u64>,        // h(x) for each abscissa (oriented ordinate)
    pub graph: bool,         // true: h has degree d and h^2-f = a^2 prod(T-v)
    pub memb: Vec<u64>,      // membership polynomial (low->high), monic
}
pub fn make_base(c: &Curve, name: &str, xs: Vec<u64>, h: Vec<u64>, graph: bool) -> Base {
    let p = c.p;
    let ys: Vec<u64> = xs.iter().map(|x| ueval(&h, *x, p)).collect();
    let memb = membership(&xs, p);
    Base { name: name.into(), xs, h, ys, graph, memb }
}

pub struct GraphSearch {
    pub base: Option<Base>,
    pub attempts: u64,
    pub log: Vec<(u64, String)>,
}
fn curve_abscissae(c: &Curve) -> Vec<u64> {
    let mut v: Vec<u64> = c.points.iter().map(|q| q.0).collect();
    v.dedup();
    v
}
fn low_y(c: &Curve, x: u64) -> u64 {
    c.points.iter().find(|q| q.0 == x).unwrap().1
}
/// Hash-forced graph proposals, exactly the historical ladder's recipe
/// (d+1 hash-selected abscissae with hash-selected ordinate signs, interpolate,
/// accept iff h^2-f has exactly 2d distinct rational roots). `legacy` selects the
/// historical domain strings and the historical root test (scan all x).
pub fn graph_search(s: &Spec, c: &Curve, d: usize, forced_label: &str) -> GraphSearch {
    let p = c.p;
    let mut log = Vec::new();
    let mut seen: std::collections::BTreeSet<Vec<u64>> = Default::default();
    let abs = curve_abscissae(c);
    for attempt in 0..s.graph_cap {
        let mut order: Vec<([u8; 32], u64)> = abs
            .iter()
            .map(|x| (hash_words(&format!("{}/{}", s.domain, forced_label), &[s.seed, p, c.a, c.b, d as u64, attempt, *x]), *x))
            .collect();
        native_n(order.len() as u64);
        order.sort_unstable();
        let forced: Vec<Pt> = order
            .into_iter()
            .take(d + 1)
            .map(|(hh, x)| {
                let y = low_y(c, x);
                (x, if hh[31] & 1 == 0 { y } else { p - y })
            })
            .collect();
        let h = interpolate(&forced, p);
        let degree = h.len().saturating_sub(1);
        native_n(1); // polynomial hash for duplicate detection
        if degree != d {
            log.push((attempt, "degree_drop".into()));
            continue;
        }
        if seen.contains(&h) {
            log.push((attempt, "duplicate_polynomial".into()));
            continue;
        }
        seen.insert(h.clone());
        let roots: Vec<u64> = (0..p)
            .filter(|x| {
                let y = ueval(&h, *x, p);
                let yy = fmul(y, y, p);
                let r = rhs((c.a, c.b), p, *x);
                yy == r
            })
            .collect();
        if roots.len() == 2 * d {
            // reject graphs with a vanishing ordinate (not possible at prime order; checked, charged)
            if roots.iter().any(|x| ueval(&h, *x, p) == 0) {
                log.push((attempt, "zero_ordinate".into()));
                continue;
            }
            log.push((attempt, "accepted".into()));
            let base = make_base(c, "graph", roots, h, true);
            return GraphSearch { base: Some(base), attempts: attempt + 1, log };
        }
        log.push((attempt, format!("nonsplit_{}", roots.len())));
    }
    GraphSearch { base: None, attempts: s.graph_cap, log }
}

/// Nongraph controls of size 2d: "low-x" (the 2d smallest abscissae) or "hash-x";
/// ordinate signs chosen by hash; h interpolates the 2d oriented points (degree <= 2d-1).
pub fn control_base(s: &Spec, c: &Curve, d: usize, name: &str, label: &str) -> Base {
    let p = c.p;
    let mut xs = curve_abscissae(c);
    if name == "hash-x" {
        let mut o: Vec<([u8; 32], u64)> = xs
            .into_iter()
            .map(|x| (hash_words(&format!("{}/{}-x", s.domain, label), &[s.seed, p, c.a, c.b, d as u64, x]), x))
            .collect();
        native_n(o.len() as u64);
        o.sort_unstable();
        xs = o.into_iter().map(|t| t.1).collect();
    }
    xs.truncate(2 * d);
    xs.sort_unstable();
    let pts: Vec<Pt> = xs
        .iter()
        .map(|x| {
            let hh = hash_words(&format!("{}/{}-y/{}", s.domain, label, name), &[s.seed, p, c.a, c.b, d as u64, *x]);
            let y = low_y(c, *x);
            (*x, if hh[31] & 1 == 0 { y } else { p - y })
        })
        .collect();
    let h = interpolate(&pts, p);
    make_base(c, name, xs, h, false)
}
