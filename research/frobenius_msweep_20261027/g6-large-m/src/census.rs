//! Exact census of all ordered m-fold sums: H[R] = #{(P_1..P_m) in S_1 x .. x S_m : P_1+..+P_m = R} for EVERY group
//! element R at once (meet-in-the-middle halves, all pairs), plus the statistics of H.

use crate::curve::{Curve, Pt};
use crate::rng::Rng;

/// Greedy balanced split of the slots into two sides by log-size (largest slot first onto the lighter side).
pub fn split_sides(sizes: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by(|&a, &b| sizes[b].cmp(&sizes[a]).then(a.cmp(&b)));
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let (mut la, mut lb) = (0.0f64, 0.0f64);
    for i in order {
        let l = (sizes[i] as f64).log2();
        if la <= lb {
            a.push(i);
            la += l;
        } else {
            b.push(i);
            lb += l;
        }
    }
    a.sort();
    b.sort();
    (a, b)
}

/// Multiset of all sums of one point from each set; returns the list and the number of group additions performed.
pub fn sumset(c: &Curve, sets: &[&Vec<Pt>]) -> (Vec<Pt>, u64) {
    let mut cur = vec![Pt::Inf];
    let mut adds = 0u64;
    for s in sets {
        let mut next = Vec::with_capacity(cur.len() * s.len());
        for &p in &cur {
            for &q in s.iter() {
                next.push(c.add(p, q));
            }
        }
        adds += (cur.len() * s.len()) as u64;
        cur = next;
    }
    (cur, adds)
}

pub struct Census {
    /// total number of tuples = product of slot sizes
    pub t: u128,
    pub a_len: usize,
    pub b_len: usize,
    /// group additions spent building the two half tables
    pub adds_build: u64,
}

/// Fill `h` (resized to the key space and zeroed) with H. Panics if a slot is empty.
pub fn histogram_into(c: &Curve, slots: &[Vec<Pt>], h: &mut Vec<u32>) -> Census {
    let sz: Vec<usize> = slots.iter().map(|s| s.len()).collect();
    assert!(sz.iter().all(|&s| s > 0));
    let (ia, ib) = split_sides(&sz);
    let sa: Vec<&Vec<Pt>> = ia.iter().map(|&i| &slots[i]).collect();
    let sb: Vec<&Vec<Pt>> = ib.iter().map(|&i| &slots[i]).collect();
    let (la, ca) = sumset(c, &sa);
    let (lb, cb) = sumset(c, &sb);
    h.clear();
    h.resize(c.key_space(), 0);
    for &a in &la {
        for &b in &lb {
            h[c.key(c.add(a, b))] += 1;
        }
    }
    Census {
        t: sz.iter().map(|&s| s as u128).product(),
        a_len: la.len(),
        b_len: lb.len(),
        adds_build: ca + cb,
    }
}

/// Plant `count` random tuples and return how many of their sums have H >= 1 (must be all of them).
pub fn planted_hits(c: &Curve, slots: &[Vec<Pt>], h: &[u32], count: usize, rng: &mut Rng) -> usize {
    let mut ok = 0;
    for _ in 0..count {
        let mut s = Pt::Inf;
        for sl in slots {
            s = c.add(s, sl[rng.below(sl.len() as u64) as usize]);
        }
        if h[c.key(s)] >= 1 {
            ok += 1;
        }
    }
    ok
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub ne: u32,
    pub mu: f64,
    /// fraction of group elements with H >= 1
    pub p: f64,
    /// p / (1 - exp(-mu)): realised yield against the Poisson ideal at the same mean
    pub eff: f64,
    /// Var(H) / mean(H) over all group elements (Poisson: 1)
    pub disp: f64,
    pub conserved: bool,
}

pub fn stats(h: &[u32], ne: u32, t: u128) -> Stats {
    let (mut nz, mut s1, mut s2) = (0u64, 0u128, 0f64);
    for &v in h {
        if v > 0 {
            nz += 1;
            s1 += v as u128;
            s2 += (v as f64) * (v as f64);
        }
    }
    let e = ne as f64;
    let mu = t as f64 / e;
    let p = nz as f64 / e;
    let disp = (s2 / e - mu * mu) / mu;
    Stats {
        ne,
        mu,
        p,
        eff: p / (1.0 - (-mu).exp()),
        disp,
        conserved: s1 == t,
    }
}

/// Frobenius orbits of size n on the points of E (fixed points excluded), as flat key lists.
pub struct OrbitMap {
    pub n: usize,
    pub keys: Vec<u32>,
    pub fixed: usize,
}

pub fn build_orbits(c: &Curve) -> OrbitMap {
    let f = c.f;
    let n = f.n as usize;
    let words = (c.key_space() + 63) / 64;
    let mut seen = vec![0u64; words];
    let mut keys: Vec<u32> = Vec::new();
    let mut fixed = 0usize;
    for x in 0..f.q {
        let mut pts: Vec<Pt> = Vec::new();
        if let Some(y) = c.lift_x(x) {
            pts.push(Pt::Aff(x, y));
            if x != 0 {
                pts.push(Pt::Aff(x, x ^ y));
            }
        }
        for p in pts {
            let k0 = c.key(p);
            if (seen[k0 / 64] >> (k0 % 64)) & 1 == 1 {
                continue;
            }
            let mut cur = p;
            let mut orbit = Vec::with_capacity(n);
            for _ in 0..n {
                orbit.push(c.key(cur));
                cur = c.frob_pt(cur);
            }
            if orbit[1] == orbit[0] {
                fixed += 1;
                seen[k0 / 64] |= 1 << (k0 % 64);
                continue;
            }
            for &k in &orbit {
                assert_eq!((seen[k / 64] >> (k % 64)) & 1, 0, "orbit overlap");
                seen[k / 64] |= 1 << (k % 64);
            }
            keys.extend(orbit.iter().map(|&k| k as u32));
        }
    }
    // O is a fixed point too
    fixed += 1;
    OrbitMap { n, keys, fixed }
}

/// Fano factor of K_O = #{j : H[sigma^j R] >= 1} over Frobenius orbits: Var(K)/(n p (1-p)); 1 for independent attempts.
/// `tamper` replaces every orbit by n copies of its first member (perfect dependence; the detector's positive control).
pub fn orbit_fano(h: &[u32], om: &OrbitMap, tamper: bool) -> f64 {
    let n = om.n;
    let (mut s1, mut s2, mut norb) = (0f64, 0f64, 0f64);
    for chunk in om.keys.chunks(n) {
        let k = if tamper {
            if h[chunk[0] as usize] >= 1 {
                n as f64
            } else {
                0.0
            }
        } else {
            chunk.iter().filter(|&&k| h[k as usize] >= 1).count() as f64
        };
        s1 += k;
        s2 += k * k;
        norb += 1.0;
    }
    if norb == 0.0 {
        return f64::NAN;
    }
    let mean = s1 / norb;
    let var = s2 / norb - mean * mean;
    let p = mean / n as f64;
    if p <= 0.0 || p >= 1.0 {
        return f64::NAN;
    }
    var / (n as f64 * p * (1.0 - p))
}
