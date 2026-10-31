//! Rank of conjugate relations in the plain and in the Frobenius-orbit basis (claim C4, second half).
//!
//! A relation of target class a over a base is a signed set  sum_i eps_i d_i = a (mod r)  (d_i = class logarithms).
//! Its n-1 conjugates are the relations of sigma^j R over sigma^j(base): legs x_i^(2^j), the sign eps_i * tau_i
//! (tau_i = +-1 compares lam^j d_i with the class logarithm of the conjugate leg), right-hand side lam^j a.
//! Plain basis: one unknown per class x. Orbit basis: one unknown per sigma-orbit, v_O = log of the orbit
//! representative point; a class x = sigma^k(rep) has point +- lam^k P_rep. The conjugate row is built from the
//! tables for the conjugate legs only (never by scaling the original row), then compared.

use crate::census::for_each_subset;
use crate::curve::{pow_mod, Group};
use crate::field::Fq;
use crate::rng::Rng;
use std::collections::HashMap;

pub struct Orbits {
    pub orbit: Vec<u32>,
    pub k: Vec<u32>,
    pub sgn: Vec<i8>,
    pub rep_d: Vec<u32>,
    pub n_orbits: usize,
}

pub fn build_orbits(f: &Fq, g: &Group) -> Orbits {
    let half = g.half() as usize;
    let mut orbit = vec![u32::MAX; half];
    let mut kk = vec![0u32; half];
    let mut sgn = vec![1i8; half];
    let mut rep_d: Vec<u32> = vec![];
    for c in 0..half {
        if orbit[c] != u32::MAX {
            continue;
        }
        // orbit of x: find the minimal x in it as representative
        let x0 = g.xs[c];
        let mut members = vec![x0];
        let mut z = f.sq(x0);
        while z != x0 {
            members.push(z);
            z = f.sq(z);
        }
        let rep = *members.iter().min().unwrap();
        let d_rep = g.dlog[rep as usize];
        let oid = rep_d.len() as u32;
        rep_d.push(d_rep);
        let mut x = rep;
        for k in 0..members.len() as u32 {
            let id = (g.dlog[x as usize] - 1) as usize;
            let e = g.mulmod(g.lam_pow(k), d_rep);
            let d_x = g.dlog[x as usize];
            orbit[id] = oid;
            kk[id] = k;
            sgn[id] = if e == d_x {
                1
            } else {
                assert_eq!(e, g.r - d_x, "orbit point is not +- the class point");
                -1
            };
            x = f.sq(x);
        }
    }
    let n_orbits = rep_d.len();
    Orbits {
        orbit,
        k: kk,
        sgn,
        rep_d,
        n_orbits,
    }
}

#[derive(Clone, Debug)]
pub struct Rel {
    pub a: u32,
    pub ds: Vec<u32>,
    pub eps: Vec<i8>,
}

pub struct Ech {
    r: u64,
    pub cols: usize,
    piv: Vec<Option<Vec<u32>>>,
    pub rank: usize,
    pub incons: usize,
}

impl Ech {
    pub fn new(r: u32, cols: usize) -> Ech {
        Ech {
            r: r as u64,
            cols,
            piv: vec![None; cols],
            rank: 0,
            incons: 0,
        }
    }
    /// row has length cols + 1 (last = right-hand side). Returns true if the rank grew.
    pub fn insert(&mut self, mut row: Vec<u32>) -> bool {
        if self.rank == self.cols {
            return false;
        }
        let r = self.r;
        for c in 0..self.cols {
            let v = row[c] as u64;
            if v == 0 {
                continue;
            }
            if let Some(p) = &self.piv[c] {
                let neg = r - v;
                for k in c..=self.cols {
                    row[k] = ((row[k] as u64 + neg * p[k] as u64) % r) as u32;
                }
            } else {
                let inv = pow_mod(v, r - 2, r);
                for k in c..=self.cols {
                    row[k] = ((row[k] as u64 * inv) % r) as u32;
                }
                self.piv[c] = Some(row);
                self.rank += 1;
                return true;
            }
        }
        if row[self.cols] != 0 {
            self.incons += 1;
        }
        false
    }
}

fn sgn_mod(r: u32, e: i8, v: u32) -> u32 {
    if e >= 0 {
        v % r
    } else {
        (r - v % r) % r
    }
}

pub type Sparse = Vec<(u32, u32)>;

/// Plain-basis row of a relation conjugated by sigma^j (j = 0: the relation itself): (columns = class ids d-1).
pub fn plain_row(f: &Fq, g: &Group, rel: &Rel, j: u32) -> (Sparse, u32, bool) {
    let r = g.r;
    let lj = g.lam_pow(j);
    let mut row: Sparse = vec![];
    let mut check: u64 = 0;
    let mut tau_ok = true;
    for (i, &d) in rel.ds.iter().enumerate() {
        let x = g.xs[(d - 1) as usize];
        let xj = f.frob(x, j);
        let dj = g.dlog[xj as usize];
        let want = g.mulmod(lj, d);
        let tau: i8 = if want == dj {
            1
        } else if want == r - dj {
            -1
        } else {
            tau_ok = false;
            1
        };
        let e = rel.eps[i] * tau;
        row.push((dj - 1, sgn_mod(r, e, 1)));
        check += sgn_mod(r, e, dj) as u64;
    }
    let rhs = g.mulmod(lj, rel.a);
    let ok = tau_ok && (check % r as u64) as u32 == rhs;
    (row, rhs, ok)
}

/// Orbit-basis row of the relation conjugated by sigma^j, built from the tables of the conjugate legs.
/// `lam_used` is lam for the honest row; a wrong multiplier gives the negative control.
pub fn orbit_row(
    f: &Fq,
    g: &Group,
    ob: &Orbits,
    rel: &Rel,
    j: u32,
    lam_used: u32,
) -> (Sparse, u32, bool) {
    let r = g.r;
    let lj = g.lam_pow(j);
    let mut tmp: HashMap<u32, u32> = HashMap::new();
    let mut check: u64 = 0;
    let mut tau_ok = true;
    for (i, &d) in rel.ds.iter().enumerate() {
        let x = g.xs[(d - 1) as usize];
        let xj = f.frob(x, j);
        let dj = g.dlog[xj as usize];
        let want = g.mulmod(lj, d);
        let tau: i8 = if want == dj {
            1
        } else if want == r - dj {
            -1
        } else {
            tau_ok = false;
            1
        };
        let e = rel.eps[i] * tau;
        let id = (dj - 1) as usize;
        let o = ob.orbit[id];
        let coef_base = pow_mod(lam_used as u64, ob.k[id] as u64, r as u64) as u32;
        let s = e * ob.sgn[id];
        let coef = sgn_mod(r, s, coef_base);
        let ent = tmp.entry(o).or_insert(0);
        *ent = (*ent + coef) % r;
        check += (coef as u64 * ob.rep_d[o as usize] as u64) % r as u64;
    }
    let rhs = g.mulmod(lj, rel.a);
    let ok = tau_ok && (check % r as u64) as u32 == rhs;
    let mut row: Sparse = tmp.into_iter().filter(|&(_, c)| c != 0).collect();
    row.sort_unstable();
    (row, rhs, ok)
}

/// Find all relations (sign patterns) of the labelled target classes over the base `ds`.
/// lab[a] = 0 not wanted, else a label; relations are returned per label (index label-1).
pub fn find_relations(ds: &[u32], r: u32, m: usize, lab: &[u8], nlabels: usize) -> Vec<Vec<Rel>> {
    let half = ((r - 1) / 2) as usize;
    let mut stamp = vec![0u32; half + 1];
    let mut sid = 0u32;
    let mut out: Vec<Vec<Rel>> = vec![vec![]; nlabels];
    let mut visit = |idx: &[usize], leaf: &[u32]| {
        sid += 1;
        let mut hits: Vec<u32> = vec![];
        for &v in leaf {
            let c = v.min(r - v);
            if c != 0 && stamp[c as usize] != sid {
                stamp[c as usize] = sid;
                if lab[c as usize] != 0 {
                    hits.push(c);
                }
            }
        }
        for &a in &hits {
            let label = lab[a as usize] as usize - 1;
            // all sign patterns (first sign + in `mask` = 0) whose signed sum is +-a; flip to make it +a
            for mask in 0u32..(1u32 << (m - 1)) {
                let mut s: u64 = 0;
                for (i, &k) in idx.iter().enumerate() {
                    let d = ds[k] as u64;
                    let neg = i > 0 && (mask >> (i - 1)) & 1 == 1;
                    s += if neg { r as u64 - d } else { d };
                }
                let s = (s % r as u64) as u32;
                if s == a || s == r - a {
                    let flip = s != a;
                    let eps: Vec<i8> = (0..m)
                        .map(|i| {
                            let neg = i > 0 && (mask >> (i - 1)) & 1 == 1;
                            let e: i8 = if neg { -1 } else { 1 };
                            if flip {
                                -e
                            } else {
                                e
                            }
                        })
                        .collect();
                    out[label].push(Rel {
                        a,
                        ds: idx.iter().map(|&k| ds[k]).collect(),
                        eps,
                    });
                }
            }
        }
    };
    for_each_subset(ds, r, m, &mut visit);
    out
}

#[derive(Debug, Default, Clone)]
pub struct RankOut {
    pub n_rel: usize,
    pub n_rel_indep: usize,
    pub cols_orbit: usize,
    pub cols_plain_n: usize,
    pub rank_plain_1: usize,
    pub rank_plain_n: usize,
    pub rank_orbit_1: usize,
    pub rank_orbit_n: usize,
    pub rank_orbit_wrong: usize,
    pub rank_orbit_indep: usize,
    pub rows_checked: usize,
    pub rows_bad: usize,
    pub incons: usize,
    /// conjugate orbit rows that are NOT the lam^j multiple of the original row
    pub nonproportional: usize,
    pub orbits_touched: usize,
    /// effective row cap used (at most a third of the orbits the base touches, so the rank can still grow)
    pub row_cap_eff: usize,
    /// rows wanted for the independent-target control ((n-1) times the number of original relations)
    pub indep_need: usize,
}

fn dense(row: &Sparse, rhs: u32, colmap: &HashMap<u32, usize>, cols: usize) -> Vec<u32> {
    let mut v = vec![0u32; cols + 1];
    for &(c, x) in row {
        v[colmap[&c]] = x;
    }
    v[cols] = rhs;
    v
}

/// Rank census of one base. t1 targets (label 1) give N relations; their n-1 conjugates are added;
/// the independent control uses t1 * n fresh targets outside the conjugate orbits.
pub fn rank_census(
    f: &Fq,
    g: &Group,
    ob: &Orbits,
    ds: &[u32],
    m: usize,
    t1: usize,
    row_cap: usize,
    rng: &mut Rng,
) -> RankOut {
    let r = g.r;
    let half = g.half() as usize;
    let n = g.n;
    // The orbit-basis column count is at most the number of sigma-orbits the base touches. With more original rows
    // than a third of that, rank_orbit(N) saturates by pigeonhole and the controls cannot grow, so the cap is lowered.
    let touched = {
        let mut set = std::collections::HashSet::new();
        for &d in ds {
            set.insert(ob.orbit[(d - 1) as usize]);
        }
        set.len()
    };
    let row_cap = row_cap.min((touched / 3).max(2));
    let mut lab = vec![0u8; half + 1];
    let mut forb = vec![false; half + 1];
    let mut picked = 0;
    while picked < t1.min(half / (n as usize + 2)) {
        let a = 1 + rng.below(half as u64) as usize;
        if lab[a] == 0 && !forb[a] {
            lab[a] = 1;
            picked += 1;
            for j in 0..n {
                let b = g.class(g.mulmod(g.lam_pow(j), a as u32)) as usize;
                forb[b] = true;
            }
        }
    }
    let mut picked2 = 0;
    let want2 = (row_cap * n as usize).min(half / 4);
    while picked2 < want2 {
        let a = 1 + rng.below(half as u64) as usize;
        if lab[a] == 0 && !forb[a] {
            lab[a] = 2;
            picked2 += 1;
        }
    }
    let mut rels = find_relations(ds, r, m, &lab, 2);
    // enumeration order is lexicographic in the base, which would concentrate the kept rows on the first orbits
    for v in rels.iter_mut() {
        rng.shuffle(v);
    }
    // cap per target (round-robin by target) so that the kept relations cover many targets, not the first few
    let take = |all: &Vec<Rel>, per: usize, total: usize| -> Vec<Rel> {
        let mut cnt: HashMap<u32, usize> = HashMap::new();
        let mut out = vec![];
        for rl in all {
            let c = cnt.entry(rl.a).or_insert(0);
            if *c < per && out.len() < total {
                *c += 1;
                out.push(rl.clone());
            }
        }
        out
    };
    let per1 = (row_cap + t1 - 1) / t1.max(1);
    let r1 = take(&rels[0], per1.max(1), row_cap);
    // the independent control is row-matched to the conjugate rows: (n-1) times as many rows as original relations
    let indep_need = (n as usize - 1) * r1.len();
    let r2 = take(&rels[1], per1.max(1), indep_need);
    let mut out = RankOut {
        n_rel: r1.len(),
        n_rel_indep: r2.len(),
        row_cap_eff: row_cap,
        indep_need,
        ..Default::default()
    };
    if r1.is_empty() {
        return out;
    }
    // sparse rows
    let mut plain_rows: Vec<(Sparse, u32)> = vec![]; // j = 0 rows first, then conjugates (j = 1..n-1)
    let mut plain_j0 = 0usize;
    let mut orb_rows_1: Vec<(Sparse, u32)> = vec![];
    let mut orb_rows_conj: Vec<(Sparse, u32)> = vec![];
    let mut orb_rows_wrong: Vec<(Sparse, u32)> = vec![];
    let wrong_lam = {
        // a unit that is not a power of lam: pick the smallest g with g^n != 1
        let mut c = 2u32;
        while pow_mod(c as u64, n as u64, r as u64) == 1 {
            c += 1;
        }
        c
    };
    for rel in &r1 {
        let (pr, rhs, ok) = plain_row(f, g, rel, 0);
        out.rows_checked += 1;
        if !ok {
            out.rows_bad += 1;
        }
        plain_rows.push((pr, rhs));
        plain_j0 += 1;
        let (or, orhs, ook) = orbit_row(f, g, ob, rel, 0, g.lam);
        out.rows_checked += 1;
        if !ook {
            out.rows_bad += 1;
        }
        orb_rows_1.push((or, orhs));
    }
    for rel in &r1 {
        let (o0, rhs0, _) = orbit_row(f, g, ob, rel, 0, g.lam);
        for j in 1..n {
            let (pr, rhs, ok) = plain_row(f, g, rel, j);
            out.rows_checked += 1;
            if !ok {
                out.rows_bad += 1;
            }
            plain_rows.push((pr, rhs));
            let (or, orhs, ook) = orbit_row(f, g, ob, rel, j, g.lam);
            out.rows_checked += 1;
            if !ook {
                out.rows_bad += 1;
            }
            // proportional to the original orbit row by lam^j ?
            let lj = g.lam_pow(j);
            let scaled: Sparse = o0.iter().map(|&(c, x)| (c, g.mulmod(lj, x))).collect();
            if scaled != or || g.mulmod(lj, rhs0) != orhs {
                out.nonproportional += 1;
            }
            orb_rows_conj.push((or, orhs));
            let (ow, orw, _) = orbit_row(f, g, ob, rel, j, wrong_lam);
            orb_rows_wrong.push((ow, orw));
        }
    }
    let mut orb_rows_indep: Vec<(Sparse, u32)> = vec![];
    for rel in &r2 {
        let (or, orhs, ook) = orbit_row(f, g, ob, rel, 0, g.lam);
        out.rows_checked += 1;
        if !ook {
            out.rows_bad += 1;
        }
        orb_rows_indep.push((or, orhs));
    }
    // column maps
    let mk = |sets: &[&Vec<(Sparse, u32)>]| -> HashMap<u32, usize> {
        let mut m: HashMap<u32, usize> = HashMap::new();
        for s in sets {
            for (row, _) in s.iter() {
                for &(c, _) in row {
                    let l = m.len();
                    m.entry(c).or_insert(l);
                }
            }
        }
        m
    };
    let run = |sets: &[&Vec<(Sparse, u32)>], cap: Option<usize>| -> (usize, usize, usize) {
        let cm = mk(sets);
        let cols = cm.len();
        let mut e = Ech::new(r, cols);
        let mut r_at_cap = 0;
        let mut cnt = 0usize;
        for s in sets {
            for (row, rhs) in s.iter() {
                e.insert(dense(row, *rhs, &cm, cols));
                cnt += 1;
                if Some(cnt) == cap {
                    r_at_cap = e.rank;
                }
            }
        }
        let _ = r_at_cap;
        (e.rank, cols, e.incons)
    };
    let plain_first: Vec<(Sparse, u32)> = plain_rows[..plain_j0].to_vec();
    let (rp1, _, i1) = run(&[&plain_first], None);
    let (rpn, cpn, i2) = run(&[&plain_rows], None);
    let (ro1, co1, i3) = run(&[&orb_rows_1], None);
    let (ron, _, i4) = run(&[&orb_rows_1, &orb_rows_conj], None);
    let (row_, _, _) = run(&[&orb_rows_1, &orb_rows_wrong], None);
    let (roi, _, i5) = run(&[&orb_rows_1, &orb_rows_indep], None);
    out.rank_plain_1 = rp1;
    out.rank_plain_n = rpn;
    out.cols_plain_n = cpn;
    out.rank_orbit_1 = ro1;
    out.cols_orbit = co1;
    out.rank_orbit_n = ron;
    out.rank_orbit_wrong = row_;
    out.rank_orbit_indep = roi;
    out.incons = i1 + i2 + i3 + i4 + i5;
    out.orbits_touched = touched;
    out
}
