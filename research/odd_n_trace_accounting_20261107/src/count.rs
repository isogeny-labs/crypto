//! Exact N(R) = #ordered n-tuples of factor-base points summing to R,
//! by meet in the middle, and an independent brute-force tally.
use crate::ec::{Curve, Pt, O};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct MulHash(u64);
impl Hasher for MulHash {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }
    fn write_u64(&mut self, k: u64) {
        let h = k.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = h ^ (h >> 29);
    }
}
pub type Map = HashMap<u64, u64, BuildHasherDefault<MulHash>>;

pub struct Counter {
    pub n: usize,
    fb: Vec<Pt>,
    c2: Map,
    /// for n = 4: entries of c2; for n = 5: entries of c3 (as negated points, with multiplicity)
    outer: Vec<(Pt, u64)>,
}

fn tally(cv: &Curve, fb: &[Pt], k: usize) -> Map {
    // ordered k-sums by iterated convolution over the factor base
    let mut cur: Map = Map::default();
    cur.insert(O.key(), 1);
    let mut pts: HashMap<u64, Pt, BuildHasherDefault<MulHash>> = HashMap::default();
    pts.insert(O.key(), O);
    for _ in 0..k {
        let mut nxt: Map = Map::default();
        let mut npts: HashMap<u64, Pt, BuildHasherDefault<MulHash>> = HashMap::default();
        for (key, &c) in &cur {
            let s = pts[key];
            for p in fb {
                let t = cv.add(&s, p);
                *nxt.entry(t.key()).or_insert(0) += c;
                npts.entry(t.key()).or_insert(t);
            }
        }
        cur = nxt;
        pts = npts;
    }
    cur
}

fn tally_with_points(cv: &Curve, fb: &[Pt], k: usize) -> Vec<(Pt, u64)> {
    let mut cur: Vec<(Pt, u64)> = vec![(O, 1)];
    for _ in 0..k {
        let mut nxt: HashMap<u64, (Pt, u64), BuildHasherDefault<MulHash>> = HashMap::default();
        for (s, c) in &cur {
            for p in fb {
                let t = cv.add(s, p);
                nxt.entry(t.key()).or_insert((t, 0)).1 += c;
            }
        }
        cur = nxt.into_values().collect();
    }
    cur
}

impl Counter {
    pub fn build(cv: &Curve, fb: &[Pt], n: usize) -> Counter {
        assert!((3..=5).contains(&n));
        let c2 = tally(cv, fb, 2);
        let outer = match n {
            3 => vec![],
            4 => tally_with_points(cv, fb, 2),
            5 => tally_with_points(cv, fb, 3),
            _ => unreachable!(),
        };
        Counter { n, fb: fb.to_vec(), c2, outer }
    }
    pub fn count(&self, cv: &Curve, r: &Pt) -> u64 {
        if self.fb.is_empty() {
            return 0;
        }
        let mut s = 0u64;
        if self.n == 3 {
            for p in &self.fb {
                let t = cv.sub(r, p);
                if let Some(c) = self.c2.get(&t.key()) {
                    s += c;
                }
            }
        } else {
            for (p, m) in &self.outer {
                let t = cv.sub(r, p);
                if let Some(c) = self.c2.get(&t.key()) {
                    s += m * c;
                }
            }
        }
        s
    }
}

/// independent check: enumerate every ordered n-tuple with nested loops and
/// tally the sums (no pair table, no convolution)
pub fn brute(cv: &Curve, fb: &[Pt], n: usize) -> Map {
    fn rec(cv: &Curve, fb: &[Pt], left: usize, acc: Pt, out: &mut Map) {
        if left == 0 {
            *out.entry(acc.key()).or_insert(0) += 1;
            return;
        }
        for p in fb {
            rec(cv, fb, left - 1, cv.add(&acc, p), out);
        }
    }
    let mut out = Map::default();
    if !fb.is_empty() {
        rec(cv, fb, n, O, &mut out);
    }
    out
}
