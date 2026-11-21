//! Exact enumeration of decompositions R = Q_1 + ... + Q_m with pairwise
//! distinct x(Q_i), every x(Q_i) in the union of blocks (x != 0), grouped by
//! shift pattern. Meet-in-the-middle over strictly x-increasing index tuples.

use std::collections::HashMap;

use crate::combi::{pack, PatternTable};
use crate::curve::{Curve, INF, P};
use crate::family::Family;
use crate::field::Fq;

#[derive(Clone, Copy)]
struct Ent {
    sum: P,
    rmin: u32,
    rmax: u32,
    idx: [u32; 3],
}

pub struct Counts {
    /// per pattern: decompositions for which the pattern is compatible
    pub cover: Vec<u32>,
    /// per pattern: 1/(number of compatible patterns), summed (a partition of `total`)
    pub frac: Vec<f64>,
    pub total: u64,
    /// decompositions with exactly one compatible pattern
    pub unamb: u64,
    /// decompositions skipped because the compatible-pattern product exceeded the cap
    pub overflow: u64,
}

pub struct Enumerator<'a> {
    f: &'a Fq,
    c: &'a Curve,
    pub pts: Vec<P>,
    pmask: Vec<u32>,
    rank: Vec<u32>,
    pub m: usize,
    a: usize,
    b: usize,
    left: Vec<Ent>,
    right: HashMap<u64, Vec<Ent>>,
}

pub const PRODUCT_CAP: usize = 4096;

impl<'a> Enumerator<'a> {
    pub fn new(f: &'a Fq, c: &'a Curve, fam: &Family, m: usize, max_left: usize) -> Enumerator<'a> {
        assert!((2..=6).contains(&m));
        let (a, b) = ((m + 1) / 2, m / 2);
        let mut pts = vec![];
        let mut pmask = vec![];
        let mut rank = vec![];
        let mut r = 0u32;
        for x in 1..f.size() as u32 {
            let km = fam.kmask[x as usize];
            if km == 0 {
                continue;
            }
            if let Some((p, q)) = c.lift(f, x) {
                for pt in [p, q] {
                    pts.push(pt);
                    pmask.push(km);
                    rank.push(r);
                }
                r += 1;
            }
        }
        let mut e = Enumerator {
            f,
            c,
            pts,
            pmask,
            rank,
            m,
            a,
            b,
            left: vec![],
            right: HashMap::new(),
        };
        let est = e.tuple_count(a);
        assert!(
            est <= max_left as f64,
            "left table too large: {est:.3e} > {max_left}"
        );
        let mut left = vec![];
        e.gen(a, &mut |ent| left.push(ent));
        let mut right: HashMap<u64, Vec<Ent>> = HashMap::new();
        e.gen(b, &mut |ent| {
            right.entry(ent.sum.key()).or_default().push(ent)
        });
        e.left = left;
        e.right = right;
        e
    }

    /// Upper estimate of the number of strictly x-increasing s-tuples of points.
    pub fn tuple_count(&self, s: usize) -> f64 {
        let nx = (self.rank.last().map(|&r| r + 1).unwrap_or(0)) as u64;
        crate::combi::binom(nx, s as u64) as f64 * (1u64 << s) as f64
    }

    pub fn n_points(&self) -> usize {
        self.pts.len()
    }
    pub fn n_left(&self) -> usize {
        self.left.len()
    }

    fn gen(&self, s: usize, out: &mut dyn FnMut(Ent)) {
        let mut cur = [0u32; 3];
        self.rec(0, s, 0, &mut cur, INF, out);
    }

    fn rec(
        &self,
        depth: usize,
        s: usize,
        start: usize,
        cur: &mut [u32; 3],
        sum: P,
        out: &mut dyn FnMut(Ent),
    ) {
        if depth == s {
            out(Ent {
                sum,
                rmin: self.rank[cur[0] as usize],
                rmax: self.rank[cur[s - 1] as usize],
                idx: *cur,
            });
            return;
        }
        for j in start..self.pts.len() {
            if depth > 0 && self.rank[j] <= self.rank[cur[depth - 1] as usize] {
                continue;
            }
            cur[depth] = j as u32;
            let ns = self.c.add(self.f, sum, self.pts[j]);
            self.rec(depth + 1, s, j + 1, cur, ns, out);
        }
    }

    /// Count all decompositions of r, grouped by compatible shift pattern.
    pub fn count_target(&self, r: P, pt: &PatternTable) -> Counts {
        let mut cnt = Counts {
            cover: vec![0; pt.pats.len()],
            frac: vec![0.0; pt.pats.len()],
            total: 0,
            unamb: 0,
            overflow: 0,
        };
        let mut ids = [0u32; 6];
        let mut pats: Vec<usize> = Vec::with_capacity(16);
        for l in &self.left {
            let need = self.c.add(self.f, r, self.c.neg(l.sum));
            if let Some(rs) = self.right.get(&need.key()) {
                for rr in rs {
                    if rr.rmin <= l.rmax {
                        continue;
                    }
                    ids[..self.a].copy_from_slice(&l.idx[..self.a]);
                    ids[self.a..self.m].copy_from_slice(&rr.idx[..self.b]);
                    cnt.total += 1;
                    if !self.patterns_of(&ids[..self.m], pt, &mut pats) {
                        cnt.overflow += 1;
                        continue;
                    }
                    if pats.len() == 1 {
                        cnt.unamb += 1;
                    }
                    let w = 1.0 / pats.len() as f64;
                    for &p in &pats {
                        cnt.cover[p] += 1;
                        cnt.frac[p] += w;
                    }
                }
            }
        }
        cnt
    }

    /// Distinct compatible patterns of a decomposition (assignments of one
    /// block to each point). Returns false if the assignment product exceeds the cap.
    fn patterns_of(&self, ids: &[u32], pt: &PatternTable, out: &mut Vec<usize>) -> bool {
        out.clear();
        let m = ids.len();
        let mut prod = 1usize;
        for &i in ids {
            prod = prod.saturating_mul(self.pmask[i as usize].count_ones() as usize);
        }
        if prod > PRODUCT_CAP {
            return false;
        }
        let mut shifts = [0u8; 6];
        let mut keys: Vec<u64> = Vec::with_capacity(prod);
        self.assign(ids, 0, &mut shifts, &mut keys);
        keys.sort_unstable();
        keys.dedup();
        for k in keys {
            out.push(pt.index[&k]);
        }
        let _ = m;
        true
    }

    fn assign(&self, ids: &[u32], d: usize, shifts: &mut [u8; 6], keys: &mut Vec<u64>) {
        if d == ids.len() {
            let mut s = shifts[..d].to_vec();
            s.sort_unstable();
            keys.push(pack(&s));
            return;
        }
        let mut mask = self.pmask[ids[d] as usize];
        while mask != 0 {
            let k = mask.trailing_zeros();
            mask &= mask - 1;
            shifts[d] = k as u8;
            self.assign(ids, d + 1, shifts, keys);
        }
    }

    /// Brute-force reference (m nested loops, no meet-in-the-middle, no
    /// tuple tables) for tiny instances: returns the number of decompositions.
    pub fn brute_total(&self, r: P) -> u64 {
        fn rec(
            e: &Enumerator,
            depth: usize,
            start: usize,
            last_rank: Option<u32>,
            sum: P,
            r: P,
        ) -> u64 {
            if depth == e.m {
                return (sum == r) as u64;
            }
            let mut t = 0;
            for j in start..e.pts.len() {
                if let Some(lr) = last_rank {
                    if e.rank[j] <= lr {
                        continue;
                    }
                }
                t += rec(
                    e,
                    depth + 1,
                    j + 1,
                    Some(e.rank[j]),
                    e.c.add(e.f, sum, e.pts[j]),
                    r,
                );
            }
            t
        }
        rec(self, 0, 0, None, INF, r)
    }
}
