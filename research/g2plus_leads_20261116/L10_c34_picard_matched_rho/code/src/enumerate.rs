//! Enumeration: uncounted brute force (lab order) and counted Picard table method; orbits.
use crate::fp::{Cnt, Fp};
use crate::plane_quartic_pic::{mons_of_degree, Pt};
use std::collections::HashMap;

/// Uncounted brute force in the lab order.
pub fn points_bruteforce(f: &[u32; 15], p: u32) -> Vec<Pt> {
    let mons = mons_of_degree(4);
    let pp = p as u64;
    let mut pts = Vec::new();
    // F(1,x1,X2) = sum_c X2^c * coef[c]
    for x1 in 0..p as u64 {
        let mut coef = [0u64; 5];
        for (i, &(_a, b, c)) in mons.iter().enumerate() {
            if f[i] == 0 {
                continue;
            }
            let mut t = f[i] as u64;
            for _ in 0..b {
                t = t * x1 % pp;
            }
            coef[c as usize] = (coef[c as usize] + t) % pp;
        }
        for x2 in 0..pp {
            let mut v = coef[4];
            for c in (0..4).rev() {
                v = (v * x2 + coef[c]) % pp;
            }
            if v == 0 {
                pts.push([1, x1 as u32, x2 as u32]);
            }
        }
    }
    // line at infinity X0=0: F(0,1,x2)
    for x2 in 0..pp {
        let pt = [0u64, 1, x2];
        if eval_plain(f, pt, pp) == 0 {
            pts.push([0, 1, x2 as u32]);
        }
    }
    if eval_plain(f, [0, 0, 1], pp) == 0 {
        pts.push([0, 0, 1]);
    }
    pts
}

pub fn eval_plain(f: &[u32; 15], pt: [u64; 3], pp: u64) -> u64 {
    let mons = mons_of_degree(4);
    let mut s = 0u64;
    for (i, &(a, b, c)) in mons.iter().enumerate() {
        if f[i] == 0 {
            continue;
        }
        let mut t = f[i] as u64;
        for _ in 0..a {
            t = t * pt[0] % pp;
        }
        for _ in 0..b {
            t = t * pt[1] % pp;
        }
        for _ in 0..c {
            t = t * pt[2] % pp;
        }
        s = (s + t) % pp;
    }
    s
}

/// Counted Picard enumeration (table method).
pub fn points_picard_counted(a2: u32, a1: u32, a0: u32, p: u32, c: &mut Cnt) -> Vec<Pt> {
    let fp = Fp::new(p as u64);
    let mut cubes: Vec<Vec<u32>> = vec![Vec::new(); p as usize];
    for y in 1..p as u64 {
        let y2 = fp.mul(y, y, c);
        let y3 = fp.mul(y2, y, c);
        cubes[y3 as usize].push(y as u32);
    }
    let mut pts = Vec::new();
    for x in 0..p as u64 {
        let x2 = fp.mul(x, x, c);
        let t = fp.add(x2, a2 as u64);
        let mut v = fp.mul(x2, t, c);
        let ax = fp.mul(a1 as u64, x, c);
        v = fp.add(v, ax);
        v = fp.add(v, a0 as u64);
        if v == 0 {
            pts.push([1, x as u32, 0]);
        } else {
            for &y in cubes[v as usize].iter() {
                pts.push([1, x as u32, y]);
            }
        }
    }
    pts.push([0, 0, 1]);
    pts
}

#[derive(Clone, Debug)]
pub struct Orbits {
    /// per point index: Some((orbit id, k)) or None
    pub of: Vec<Option<(u32, u8)>>,
    pub n_orbits: usize,
    /// rep point index per orbit
    pub rep: Vec<usize>,
    pub n_fixed: usize,
}

/// Orbit structure for a Picard curve (bookkeeping uncounted).
pub fn orbits_of(pts: &[Pt], idx: &HashMap<Pt, usize>, zeta: u64, p: u32) -> Orbits {
    let pp = p as u64;
    let mut of: Vec<Option<(u32, u8)>> = vec![None; pts.len()];
    let mut rep = Vec::new();
    let mut n_orbits = 0usize;
    for (i, pt) in pts.iter().enumerate() {
        if pt[0] != 1 || pt[2] == 0 || of[i].is_some() {
            continue;
        }
        let x = pt[1];
        let ys = [pt[2] as u64, pt[2] as u64 * zeta % pp, pt[2] as u64 * zeta % pp * zeta % pp];
        // rep = smallest y
        let mut ri = 0;
        for k in 1..3 {
            if ys[k] < ys[ri] {
                ri = k;
            }
        }
        let yrep = ys[ri];
        let ypts = [yrep, yrep * zeta % pp, yrep * zeta % pp * zeta % pp];
        let id = n_orbits as u32;
        let mut repidx = usize::MAX;
        for k in 0..3 {
            let q = [1u32, x, ypts[k] as u32];
            let j = *idx.get(&q).expect("orbit member missing");
            assert!(of[j].is_none());
            of[j] = Some((id, k as u8));
            if k == 0 {
                repidx = j;
            }
        }
        rep.push(repidx);
        n_orbits += 1;
    }
    let n_fixed = pts.len() - 3 * n_orbits;
    Orbits { of, n_orbits, rep, n_fixed }
}

pub fn orbits_identity(pts: &[Pt], p0: Pt) -> Orbits {
    let mut of = vec![None; pts.len()];
    let mut rep = Vec::new();
    let mut n = 0u32;
    for (i, pt) in pts.iter().enumerate() {
        if *pt == p0 {
            continue;
        }
        of[i] = Some((n, 0u8));
        rep.push(i);
        n += 1;
    }
    Orbits { of, n_orbits: n as usize, rep, n_fixed: 1 }
}
