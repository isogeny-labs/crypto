//! Line relation search for arms Q_full / C_full / P_full / P_orbit.
use crate::curves::{eval_f, Family};
use crate::enumerate::Orbits;
use crate::fp::{Cnt, Fp};
use crate::plane_quartic_pic::{xorshift_next, Pt};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Full(Vec<(u32, i64)>),
    Orb(Vec<(u32, [u8; 3])>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    QFull,
    CFull,
    PFull,
    POrbit,
}
impl Arm {
    pub fn name(&self) -> &'static str {
        match self {
            Arm::QFull => "Q_full",
            Arm::CFull => "C_full",
            Arm::PFull => "P_full",
            Arm::POrbit => "P_orbit",
        }
    }
    pub fn family(&self) -> Family {
        match self {
            Arm::QFull => Family::Q,
            Arm::CFull => Family::C,
            _ => Family::P,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Counters {
    pub line_mul: u64,
    pub line_inv: u64,
    pub line_setup_mul: u64,
    pub trials: u64,
    pub accepted: u64,
    pub dup: u64,
    pub trivial: u64,
    pub nonsquare: u64,
    pub tangent: u64,
}

pub struct SearchResult {
    pub counters: Counters,
    pub rows: Vec<Row>,
    /// point indices of the multiset M for each accepted row
    pub rel_pts: Vec<[usize; 4]>,
    pub n_cols: usize,
    pub rows_target: usize,
    pub ok: bool,
}

pub struct SearchInput<'a> {
    pub arm: Arm,
    pub p: u32,
    pub f: &'a [u32; 15],
    pub pts: &'a [Pt],
    pub idx: &'a HashMap<Pt, usize>,
    pub p0: Pt,
    pub orbits: Option<&'a Orbits>,
    pub curve_seed: u64,
    pub solver_seed: u64,
    pub rows_target: usize,
    pub cap: u64,
}

pub fn n_cols_for(arm: Arm, n_points: usize, orbits: Option<&Orbits>) -> usize {
    match arm {
        Arm::QFull => n_points,
        Arm::CFull | Arm::PFull => n_points - 1,
        Arm::POrbit => orbits.unwrap().n_orbits,
    }
}

fn normalise(fp: &Fp, v: [u64; 3], c: &mut Cnt) -> Pt {
    let i = (0..3).find(|&i| v[i] != 0).expect("zero vector");
    let iz = fp.inv(v[i], c);
    let mut o = [0u32; 3];
    for k in 0..3 {
        if k == i {
            o[k] = 1;
        } else {
            o[k] = fp.mul(v[k], iz, c) as u32;
        }
    }
    o
}

fn comb(fp: &Fp, s: u64, t: u64, pp: Pt, qq: Pt, c: &mut Cnt) -> [u64; 3] {
    let mut r = [0u64; 3];
    for k in 0..3 {
        let a = fp.mul(s, pp[k] as u64, c);
        let b = fp.mul(t, qq[k] as u64, c);
        r[k] = fp.add(a, b);
    }
    r
}

pub fn run_search(inp: &SearchInput) -> SearchResult {
    let p = inp.p as u64;
    let fp = Fp::new(p);
    let n_pts = inp.pts.len();
    let n_cols = n_cols_for(inp.arm, n_pts, inp.orbits);
    let mut cnt = Cnt::default();
    let mut ct = Counters::default();
    // sqrt table (counted)
    let mut sqrt_of: Vec<i64> = vec![-1; p as usize];
    sqrt_of[0] = 0;
    for y in 1..=(p - 1) / 2 {
        let s = fp.mul(y, y, &mut cnt);
        sqrt_of[s as usize] = y as i64;
    }
    ct.line_setup_mul = (p - 1) / 2;
    let i2 = fp.inv_u(2);
    let i3 = fp.inv_u(3);
    // column map for full arms
    let p0idx = *inp.idx.get(&inp.p0).expect("p0 in points");
    let mut col_of: Vec<Option<u32>> = vec![None; n_pts];
    {
        let mut c = 0u32;
        for i in 0..n_pts {
            if i != p0idx {
                col_of[i] = Some(c);
                c += 1;
            }
        }
    }
    let mut state: u64 = 7_000_003u64
        .wrapping_mul(inp.solver_seed)
        .wrapping_add(1_000u64.wrapping_mul(inp.curve_seed))
        .wrapping_add(p);
    let mut seen: HashSet<Vec<u64>> = HashSet::new();
    let mut rows: Vec<Row> = Vec::new();
    let mut rel_pts: Vec<[usize; 4]> = Vec::new();
    let mut ok = true;
    let nn = n_pts as u64;
    while (ct.accepted as usize) < inp.rows_target {
        if ct.trials > inp.cap {
            ok = false;
            break;
        }
        let (i, j) = loop {
            let i = xorshift_next(&mut state) % nn;
            let j = xorshift_next(&mut state) % nn;
            if i != j {
                break (i as usize, j as usize);
            }
        };
        ct.trials += 1;
        let pp = inp.pts[i];
        let qq = inp.pts[j];
        let mut e = [0u64; 3];
        let mut ev = [[0u64; 3]; 3];
        for k in 0..3 {
            ev[0][k] = fp.add(pp[k] as u64, qq[k] as u64);
            ev[1][k] = fp.sub(pp[k] as u64, qq[k] as u64);
            ev[2][k] = fp.add(pp[k] as u64, fp.add(qq[k] as u64, qq[k] as u64));
        }
        for k in 0..3 {
            e[k] = eval_f(inp.f, ev[k], &fp, &mut cnt);
        }
        let (e1, e2, e3) = (e[0], e[1], e[2]);
        let c2 = fp.mul(fp.add(e1, e2), i2, &mut cnt);
        let u = fp.mul(fp.sub(e1, e2), i2, &mut cnt);
        let w = fp.sub(fp.mul(e3, i2, &mut cnt), fp.add(c2, c2));
        let c3 = fp.mul(fp.sub(w, u), i3, &mut cnt);
        let c1 = fp.sub(u, c3);
        assert!(!(c1 == 0 && c2 == 0 && c3 == 0), "line inside curve");
        let c2sq = fp.mul(c2, c2, &mut cnt);
        let c1c3 = fp.mul(c1, c3, &mut cnt);
        let c1c3x4 = fp.add(fp.add(c1c3, c1c3), fp.add(c1c3, c1c3));
        let delta = fp.sub(c2sq, c1c3x4);
        let res: [Pt; 2];
        if c1 == 0 {
            if c2 != 0 {
                let r = comb(&fp, fp.neg(c3), c2, pp, qq, &mut cnt);
                let r = normalise(&fp, r, &mut cnt);
                res = [pp, r];
            } else {
                ct.tangent += 1;
                res = [pp, pp];
            }
        } else {
            let sq = sqrt_of[delta as usize];
            if sq < 0 {
                ct.nonsquare += 1;
                continue;
            }
            if delta == 0 {
                ct.tangent += 1;
                let r = comb(&fp, fp.neg(c2), fp.add(c1, c1), pp, qq, &mut cnt);
                let r = normalise(&fp, r, &mut cnt);
                res = [r, r];
            } else {
                let d = sq as u64;
                let t = fp.add(c1, c1);
                let r1 = comb(&fp, fp.add(fp.neg(c2), d), t, pp, qq, &mut cnt);
                let r1 = normalise(&fp, r1, &mut cnt);
                let r2 = comb(&fp, fp.sub(fp.neg(c2), d), t, pp, qq, &mut cnt);
                let r2 = normalise(&fp, r2, &mut cnt);
                res = [r1, r2];
            }
        }
        let ra = *inp.idx.get(&res[0]).expect("residual not on curve");
        let rb = *inp.idx.get(&res[1]).expect("residual not on curve");
        let m = [i, j, ra, rb];
        let (key, row, empty): (Vec<u64>, Row, bool) = match inp.arm {
            Arm::QFull | Arm::CFull | Arm::PFull => {
                let mut k: Vec<u64> = m.iter().map(|&x| x as u64).collect();
                k.sort();
                let mut cols: Vec<(u32, i64)> = Vec::new();
                for &x in m.iter() {
                    if let Some(c) = col_of[x] {
                        match cols.iter_mut().find(|e| e.0 == c) {
                            Some(e) => e.1 += 1,
                            None => cols.push((c, 1)),
                        }
                    }
                }
                if inp.arm == Arm::QFull {
                    cols.push(((n_pts - 1) as u32, -1));
                }
                cols.sort();
                let em = cols.is_empty();
                (k, Row::Full(cols), em)
            }
            Arm::POrbit => {
                let orb = inp.orbits.unwrap();
                let mut trip: Vec<(u32, [u8; 3])> = Vec::new();
                for &x in m.iter() {
                    if let Some((o, k)) = orb.of[x] {
                        match trip.iter_mut().find(|e| e.0 == o) {
                            Some(e) => e.1[k as usize] += 1,
                            None => {
                                let mut t = [0u8; 3];
                                t[k as usize] = 1;
                                trip.push((o, t));
                            }
                        }
                    }
                }
                let mut red: Vec<(u32, [u8; 3])> = Vec::new();
                for (o, t) in trip.into_iter() {
                    let mn = *t.iter().min().unwrap();
                    let t2 = [t[0] - mn, t[1] - mn, t[2] - mn];
                    if t2 != [0, 0, 0] {
                        red.push((o, t2));
                    }
                }
                red.sort();
                let key = orbit_key(&red);
                let em = red.is_empty();
                (key, Row::Orb(red), em)
            }
        };
        if seen.contains(&key) {
            ct.dup += 1;
            continue;
        }
        seen.insert(key);
        if empty {
            ct.trivial += 1;
            continue;
        }
        ct.accepted += 1;
        rows.push(row);
        rel_pts.push(m);
    }
    ct.line_mul = cnt.mul;
    ct.line_inv = cnt.inv;
    SearchResult { counters: ct, rows, rel_pts, n_cols, rows_target: inp.rows_target, ok }
}

/// Lexicographic minimum over the three rotations r^0, r^1, r^2, flattened.
pub fn orbit_key(row: &[(u32, [u8; 3])]) -> Vec<u64> {
    let mut best: Option<Vec<u64>> = None;
    for rot in 0..3 {
        let mut v: Vec<u64> = Vec::with_capacity(row.len() * 4);
        for &(o, t) in row.iter() {
            let mut t2 = t;
            for _ in 0..rot {
                t2 = [t2[2], t2[0], t2[1]];
            }
            v.push(o as u64);
            v.push(t2[0] as u64);
            v.push(t2[1] as u64);
            v.push(t2[2] as u64);
        }
        if best.as_ref().map_or(true, |b| v < *b) {
            best = Some(v);
        }
    }
    best.unwrap_or_default()
}
