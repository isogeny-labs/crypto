//! Development adapter, reusing the actual Rust tower-F4 source, not a copy.
//! All costs below are diagnostics, not calibrated GAE or an ecbench result.
// The shared module's `any_between` is used by other lab engines, which
// are intentionally not linked into this focused adapter crate.
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/f4_fp_tower.rs"]
pub mod f4_fp_tower;
#[path = "../../../src/cryptanalysis/sig_fp_tower.rs"]
pub mod sig_fp_tower;
// The reused modules' own tests use the library's usual qualified path.
pub mod cryptanalysis {
    pub use crate::{f4_fp_tower, sig_fp_tower};
}

use f4_fp_tower::{f4_tower, mask_of, RPoly, SquareRule, TowerF4Options, TowerRing};
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::Duration;

pub type Point = Option<(u64, u64)>;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub g: (u64, u64),
}

pub const PRIME31: Curve = Curve {
    p: 2_147_483_647,
    a: 847_514_846,
    b: 2_087_444_724,
    n: 2_147_446_831,
    g: (625_037_715, 2_100_627_251),
};

#[derive(Clone, Copy, Default, Debug, Serialize, PartialEq, Eq)]
pub struct Counts {
    pub group_additions: u64,
    pub fp_multiplications: u64,
    pub fp_inversions: u64,
    pub scalar_multiplications: u64,
    pub scalar_inversions: u64,
    pub polynomial_evaluations: u64,
    pub extraction_nodes: u64,
}

#[derive(Default)]
pub struct Arithmetic {
    pub counts: Counts,
}

impl Arithmetic {
    fn mul(&mut self, x: u64, y: u64, p: u64) -> u64 {
        self.counts.fp_multiplications += 1;
        ((x as u128 * y as u128) % p as u128) as u64
    }
    fn pow(&mut self, mut x: u64, mut e: u64, p: u64) -> u64 {
        let mut out = 1;
        while e > 0 {
            if e & 1 != 0 {
                out = self.mul(out, x, p);
            }
            x = self.mul(x, x, p);
            e >>= 1;
        }
        out
    }
    fn inv(&mut self, x: u64, p: u64) -> u64 {
        assert!(x > 0 && x < p);
        self.counts.fp_inversions += 1;
        self.pow(x, p - 2, p)
    }
    pub fn sqrt(&mut self, x: u64, p: u64) -> Option<u64> {
        assert_eq!(p % 4, 3);
        let r = self.pow(x, (p + 1) / 4, p);
        (self.mul(r, r, p) == x).then_some(r)
    }
    pub fn rhs(&mut self, e: Curve, x: u64) -> u64 {
        let xx = self.mul(x, x, e.p);
        let xxx = self.mul(xx, x, e.p);
        (xxx + self.mul(e.a, x, e.p) + e.b) % e.p
    }
    pub fn on_curve(&mut self, e: Curve, q: Point) -> bool {
        q.is_none_or(|(x, y)| x < e.p && y < e.p && self.mul(y, y, e.p) == self.rhs(e, x))
    }
    pub fn neg(&self, e: Curve, q: Point) -> Point {
        q.map(|(x, y)| (x, (e.p - y) % e.p))
    }
    pub fn add(&mut self, e: Curve, u: Point, v: Point) -> Point {
        self.counts.group_additions += 1;
        let (Some((x, y)), Some((z, w))) = (u, v) else {
            return u.or(v);
        };
        if x == z && (y + w) % e.p == 0 {
            return None;
        }
        let (num, den) = if x == z {
            ((3 * self.mul(x, x, e.p) + e.a) % e.p, 2 * y % e.p)
        } else {
            (sub(w, y, e.p), sub(z, x, e.p))
        };
        let id = self.inv(den, e.p);
        let slope = self.mul(num, id, e.p);
        let xx = sub(sub(self.mul(slope, slope, e.p), x, e.p), z, e.p);
        let yy = sub(self.mul(slope, sub(x, xx, e.p), e.p), y, e.p);
        Some((xx, yy))
    }
    pub fn scalar(&mut self, e: Curve, mut k: u64, mut q: Point) -> Point {
        let mut out = None;
        while k > 0 {
            if k & 1 != 0 {
                out = self.add(e, out, q);
            }
            q = self.add(e, q, q);
            k >>= 1;
        }
        out
    }
    fn smul(&mut self, x: u64, y: u64, n: u64) -> u64 {
        self.counts.scalar_multiplications += 1;
        ((x as u128 * y as u128) % n as u128) as u64
    }
    fn sinv(&mut self, x: u64, n: u64) -> u64 {
        assert_ne!(x, 0);
        self.counts.scalar_inversions += 1;
        let (mut z, mut b, mut k) = (1, x, n - 2);
        while k > 0 {
            if k & 1 != 0 {
                z = self.smul(z, b, n);
            }
            b = self.smul(b, b, n);
            k >>= 1;
        }
        z
    }
}

fn sub(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}

fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return n == d;
        }
        d += 1;
    }
    true
}

pub fn check_curve(e: Curve, ar: &mut Arithmetic) -> Result<(), String> {
    if e.p >= 1 << 32
        || e.n >= 1 << 32
        || e.a >= e.p
        || e.b >= e.p
        || e.p % 4 != 3
        || !prime(e.p)
        || !prime(e.n)
    {
        return Err("requires prime p<2^32, p=3 mod 4 and prime subgroup order".into());
    }
    let a2 = ar.mul(e.a, e.a, e.p);
    let a3 = ar.mul(a2, e.a, e.p);
    let b2 = ar.mul(e.b, e.b, e.p);
    if (4 * a3 + 27 * b2).is_multiple_of(e.p)
        || !ar.on_curve(e, Some(e.g))
        || ar.scalar(e, e.n, Some(e.g)).is_some()
    {
        return Err("singular curve, invalid generator or subgroup certificate".into());
    }
    // A prime n annihilating a nonzero G with 2n above the Hasse upper
    // bound certifies #E=n. Use a conservative integer sqrt upper bound.
    let root = (e.p as f64).sqrt() as u64 + 1;
    if 2 * e.n <= e.p + 1 + 2 * root {
        return Err("full prime-order certificate not established".into());
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct Base {
    pub depth: usize,
    pub terminal: u64,
    pub values: Vec<u64>,
    pub points: Vec<(u64, u64)>,
}

impl Base {
    pub fn build(e: Curve, depth: usize, ar: &mut Arithmetic) -> Result<Self, String> {
        if !(1..=10).contains(&depth) || !(e.p + 1).is_multiple_of(1 << depth) {
            return Err("Dickson fibre requires 2^depth | p+1, depth=1..10".into());
        }
        for seed in 2..e.p.min(10_000) {
            let mut c = seed;
            for _ in 0..depth {
                c = sub(ar.mul(c, c, e.p), 2, e.p);
            }
            if c == 2 || c == e.p - 2 {
                continue;
            }
            let mut values = vec![c];
            for _ in 0..depth {
                let mut next = Vec::new();
                for y in values {
                    if let Some(x) = ar.sqrt((y + 2) % e.p, e.p) {
                        next.push(x);
                        next.push((e.p - x) % e.p);
                    }
                }
                next.sort_unstable();
                next.dedup();
                values = next;
            }
            if values.len() != 1 << depth {
                continue;
            }
            let mut points = Vec::new();
            for &x in &values {
                let rhs = ar.rhs(e, x);
                if let Some(y) = ar.sqrt(rhs, e.p) {
                    if y != 0 {
                        points.push((x, y.min(e.p - y)));
                    }
                }
            }
            if points.len() >= 2 {
                return Ok(Self {
                    depth,
                    terminal: c,
                    values,
                    points,
                });
            }
        }
        Err("no fully split fibre with >=2 curve points in frozen seed scan".into())
    }
    fn ring(&self, p: u64) -> TowerRing {
        let mut rules = Vec::new();
        for block in 0..2 {
            for level in 0..self.depth {
                let terminal = level + 1 == self.depth;
                rules.push(SquareRule {
                    next: (!terminal).then_some((block * self.depth + level + 1) as u8),
                    a: 0,
                    b: u64::from(!terminal),
                    c: 0,
                    d: if terminal { (self.terminal + 2) % p } else { 2 },
                });
            }
        }
        TowerRing {
            p,
            rules,
            n_free: 0,
        }
    }
}

fn s3_input(e: Curve, base: &Base, xr: u64, ar: &mut Arithmetic) -> Vec<(Vec<u32>, u64)> {
    let rr = ar.mul(xr, xr, e.p);
    let aa = ar.mul(e.a, e.a, e.p);
    let arx = ar.mul(e.a, xr, e.p);
    let brx = ar.mul(e.b, xr, e.p);
    let mixed = sub(0, 2 * ((rr + e.a) % e.p) % e.p, e.p);
    let linear = sub(0, (2 * arx + 4 * e.b) % e.p, e.p);
    [
        ([2, 2], 1),
        ([2, 1], sub(0, 2 * xr % e.p, e.p)),
        ([1, 2], sub(0, 2 * xr % e.p, e.p)),
        ([2, 0], rr),
        ([0, 2], rr),
        ([1, 1], mixed),
        ([1, 0], linear),
        ([0, 1], linear),
        ([0, 0], sub(aa, 4 * brx % e.p, e.p)),
    ]
    .into_iter()
    .map(|(ex, c)| {
        let mut v = vec![0; 2 * base.depth];
        v[0] = ex[0];
        v[base.depth] = ex[1];
        (v, c)
    })
    .collect()
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relation {
    pub entries: [(usize, i8); 2],
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Limits {
    pub degree: u32,
    pub seconds: u64,
    pub dense_entries: u64,
    pub extraction_nodes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            degree: 16,
            seconds: 5,
            dense_entries: 2_000_000,
            extraction_nodes: 100_000,
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct OracleResult {
    pub relations: Vec<Relation>,
    pub status: String,
    pub f4_muladds: u64,
    pub solving_degree: u32,
    pub max_columns: usize,
    pub extraction_nodes: u64,
}

fn leaf_relations(
    e: Curve,
    base: &Base,
    r: Point,
    xs: &[u64],
    ar: &mut Arithmetic,
) -> Vec<Relation> {
    let mut out = Vec::new();
    let Some(i) = base.points.iter().position(|q| q.0 == xs[0]) else {
        return out;
    };
    let Some(j) = base.points.iter().position(|q| q.0 == xs[base.depth]) else {
        return out;
    };
    for si in [-1, 1] {
        for sj in [-1, 1] {
            let u = Some(base.points[i]);
            let v = Some(base.points[j]);
            let u = if si == 1 { u } else { ar.neg(e, u) };
            let v = if sj == 1 { v } else { ar.neg(e, v) };
            if ar.add(e, u, v) == r {
                let mut entries = [(i, si), (j, sj)];
                entries.sort_unstable();
                out.push(Relation { entries });
            }
        }
    }
    out
}

// Recover Fp points of a finite quadratic tower. At each node, evaluate
// assigned basis equations, use a remaining linear equation when present,
// otherwise split the next quadratic into its two actual field roots.
// Every branch and polynomial evaluation is exposed; no scan over Fp.
struct Extract<'a> {
    ring: &'a TowerRing,
    basis: &'a [RPoly],
    input: &'a RPoly,
    ar: &'a mut Arithmetic,
    nodes: u64,
    cap: u64,
    stopped: bool,
    solutions: Vec<Vec<u64>>,
}
impl Extract<'_> {
    fn visit(&mut self, x: &mut [u64], assigned: u64, left: usize) {
        if self.nodes == self.cap {
            self.stopped = true;
            return;
        }
        self.nodes += 1;
        self.ar.counts.extraction_nodes += 1;
        let p = self.ring.p;
        let mut forced: BTreeMap<usize, u64> = BTreeMap::new();
        for g in self.basis {
            self.ar.counts.polynomial_evaluations += 1;
            let mut constant = 0;
            let mut unknown: BTreeMap<u64, u64> = BTreeMap::new();
            for (&m, &c) in g.monos.iter().zip(&g.coefs) {
                let mask = mask_of(m);
                let mut val = u64::from(c);
                let mut bits = mask & assigned;
                while bits != 0 {
                    let j = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    val = self.ar.mul(val, x[j], p);
                }
                let rest = mask & !assigned;
                if rest == 0 {
                    constant = (constant + val) % p;
                } else {
                    let v = unknown.entry(rest).or_default();
                    *v = (*v + val) % p;
                }
            }
            unknown.retain(|_, v| *v != 0);
            if unknown.is_empty() && constant != 0 {
                return;
            }
            if unknown.len() == 1 {
                let (&mask, &coef) = unknown.iter().next().unwrap();
                if mask.count_ones() == 1 {
                    let j = mask.trailing_zeros() as usize;
                    let inv = self.ar.inv(coef, p);
                    let value = self.ar.mul(sub(0, constant, p), inv, p);
                    if forced.insert(j, value).is_some_and(|old| old != value) {
                        return;
                    }
                }
            }
        }
        if left == 0 {
            self.ar.counts.polynomial_evaluations += 1;
            if self.ring.eval(self.input, x) == 0 {
                self.solutions.push(x.to_vec());
            }
            return;
        }
        // Descending order makes every quadratic's next level known.
        let j = left - 1;
        let rule = self.ring.rules[j];
        let rhs = rule.next.map_or(rule.d, |k| (x[k as usize] + rule.d) % p);
        let candidates = if let Some(&v) = forced.get(&j) {
            if self.ar.mul(v, v, p) != rhs {
                return;
            }
            vec![v]
        } else {
            let Some(v) = self.ar.sqrt(rhs, p) else {
                return;
            };
            if v == 0 {
                vec![0]
            } else {
                vec![v, p - v]
            }
        };
        for v in candidates {
            x[j] = v;
            self.visit(x, assigned | (1 << j), left - 1);
            if self.stopped {
                return;
            }
        }
    }
}

pub fn decompose(
    e: Curve,
    base: &Base,
    r: Point,
    limits: Limits,
    ar: &mut Arithmetic,
) -> OracleResult {
    if !ar.on_curve(e, r) {
        return OracleResult {
            status: "invalid_input".into(),
            ..Default::default()
        };
    }
    if r.is_none() {
        // Explicit identity and inverse-pair exception to affine S3.
        return OracleResult {
            relations: (0..base.points.len())
                .map(|i| Relation {
                    entries: [(i, -1), (i, 1)],
                })
                .collect(),
            status: "complete".into(),
            ..Default::default()
        };
    }
    let ring = base.ring(e.p);
    ring.check();
    let raw = s3_input(e, base, r.unwrap().0, ar);
    let input = ring.from_raw(&raw);
    let opts = TowerF4Options::new(limits.degree)
        .with_budget(Duration::from_secs(limits.seconds))
        .with_max_dense(limits.dense_entries)
        .with_max_nnz(limits.dense_entries);
    let rep = f4_tower(std::slice::from_ref(&input), &ring, &opts);
    let mut out = OracleResult {
        f4_muladds: rep.muladds,
        solving_degree: rep.solving_degree_max,
        max_columns: rep.max_cols,
        ..Default::default()
    };
    if rep.inconsistent {
        out.status = "complete".into();
        return out;
    }
    if rep.timed_out || rep.oversize || rep.pairs_above_bound > 0 {
        out.status = "undetermined_f4".into();
        return out;
    }
    let mut extract = Extract {
        ring: &ring,
        basis: &rep.basis,
        input: &input,
        ar,
        nodes: 0,
        cap: limits.extraction_nodes,
        stopped: false,
        solutions: Vec::new(),
    };
    extract.visit(&mut vec![0; 2 * base.depth], 0, 2 * base.depth);
    out.extraction_nodes = extract.nodes;
    out.status = if extract.stopped {
        "undetermined_extraction"
    } else {
        "complete"
    }
    .into();
    let sols = std::mem::take(&mut extract.solutions);
    for x in sols {
        out.relations.extend(leaf_relations(e, base, r, &x, ar));
    }
    out.relations.sort_unstable();
    out.relations.dedup();
    out
}

#[derive(Default)]
pub struct Rank {
    rows: Vec<Option<Vec<u64>>>,
    pub rank: usize,
}
impl Rank {
    fn new(base: usize) -> Self {
        Self {
            rows: vec![None; base + 1],
            rank: 0,
        }
    }
    fn add(
        &mut self,
        mut row: Vec<u64>,
        n: u64,
        ar: &mut Arithmetic,
    ) -> Result<Option<u64>, String> {
        let cols = self.rows.len();
        for j in 0..cols {
            if row[j] == 0 {
                continue;
            }
            if let Some(pivot) = &self.rows[j] {
                let k = row[j];
                for c in j..=cols {
                    row[c] = sub(row[c], ar.smul(k, pivot[c], n), n);
                }
            } else {
                let inv = ar.sinv(row[j], n);
                for v in &mut row[j..] {
                    *v = ar.smul(*v, inv, n);
                }
                let answer = (j + 1 == cols).then_some(row[cols]);
                self.rows[j] = Some(row);
                self.rank += 1;
                return Ok(answer);
            }
        }
        if row[cols] != 0 {
            return Err("inconsistent independently replayed relation rows".into());
        }
        Ok(None)
    }
}

#[derive(Debug, Serialize)]
pub struct SolveResult {
    pub status: String,
    pub scalar: Option<u64>,
    pub curve: Curve,
    pub target: Point,
    pub depth: usize,
    pub factor_base: Base,
    pub calls: u64,
    pub misses: u64,
    pub undetermined: u64,
    pub relations: u64,
    pub rank: usize,
    pub f4_muladds: u64,
    pub arithmetic: Counts,
    pub setup_arithmetic: Counts,
    pub limits: Limits,
    pub accounting: &'static str,
}

pub fn solve(
    e: Curve,
    q: Point,
    depth: usize,
    seed: u64,
    max_calls: u64,
    limits: Limits,
) -> Result<SolveResult, String> {
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar)?;
    if !ar.on_curve(e, q) {
        return Err("off-curve public target".into());
    }
    let base = Base::build(e, depth, &mut ar)?;
    let setup = ar.counts;
    let mut rank = Rank::new(base.points.len());
    let mut out = SolveResult { status: "censored_call_limit".into(), scalar: None, curve: e,
        target: q, depth, factor_base: base.clone(), calls: 0, misses: 0, undetermined: 0,
        relations: 0, rank: 0, f4_muladds: 0, arithmetic: Counts::default(),
        setup_arithmetic: setup, limits,
        accounting: "development counters only: F4 muladds exclude symbolic work, allocation, hashing, ring normalisation; no GAE/S/rho ranking" };
    if q.is_none() {
        out.scalar = Some(0);
        out.status = "solved".into();
    }
    let mut rng = seed;
    let mut next = || {
        rng = rng.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31)) % e.n
    };
    for _ in 0..max_calls {
        if out.scalar.is_some() {
            break;
        }
        let a = next();
        let b = next();
        let ag = ar.scalar(e, a, Some(e.g));
        let bq = ar.scalar(e, b, q);
        let r = ar.add(e, ag, bq);
        let found = decompose(e, &base, r, limits, &mut ar);
        out.calls += 1;
        out.f4_muladds += found.f4_muladds;
        if found.status != "complete" {
            out.undetermined += 1;
        }
        if found.relations.is_empty() && found.status == "complete" {
            out.misses += 1;
        }
        for rel in found.relations {
            // A second group-law check at the consumer boundary.
            let mut sum = None;
            for (j, sign) in rel.entries {
                let pt = Some(base.points[j]);
                let pt = if sign == 1 { pt } else { ar.neg(e, pt) };
                sum = ar.add(e, sum, pt);
            }
            if sum != r {
                return Err("relation replay failed".into());
            }
            let size = base.points.len();
            let mut row = vec![0; size + 2];
            for (j, s) in rel.entries {
                row[j] = (row[j] + if s > 0 { 1 } else { e.n - 1 }) % e.n;
            }
            row[size] = sub(0, b, e.n);
            row[size + 1] = a;
            out.relations += 1;
            if let Some(k) = rank.add(row, e.n, &mut ar)? {
                if ar.scalar(e, k, Some(e.g)) != q {
                    return Err("final scalar replay failed".into());
                }
                out.scalar = Some(k);
                out.status = "solved".into();
                break;
            }
        }
    }
    out.rank = rank.rank;
    out.arithmetic = ar.counts;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn exhaustive(e: Curve, b: &Base, r: Point) -> Vec<Relation> {
        let mut ar = Arithmetic::default();
        let mut out = Vec::new();
        for &x in &b.values {
            for &y in &b.values {
                let mut xs = vec![0; 2 * b.depth];
                xs[0] = x;
                xs[b.depth] = y;
                out.extend(leaf_relations(e, b, r, &xs, &mut ar));
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
    #[test]
    fn prime31_certificate_and_positive_oracle() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let b = Base::build(PRIME31, 2, &mut ar).unwrap();
        for i in 0..b.points.len() {
            for j in i..b.points.len() {
                let q = ar.add(PRIME31, Some(b.points[i]), Some(b.points[j]));
                let r = decompose(PRIME31, &b, q, Limits::default(), &mut ar);
                assert_eq!(r.status, "complete");
                assert!(!r.relations.is_empty());
                assert_eq!(r.relations, exhaustive(PRIME31, &b, q));
            }
        }
    }
    #[test]
    fn prime31_natural_misses_identity_and_limits() {
        let mut ar = Arithmetic::default();
        let b = Base::build(PRIME31, 2, &mut ar).unwrap();
        for k in [1, 17, 137, 20261115] {
            let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
            let r = decompose(PRIME31, &b, q, Limits::default(), &mut ar);
            assert_eq!(r.status, "complete");
            assert_eq!(r.relations, exhaustive(PRIME31, &b, q));
        }
        let r = decompose(PRIME31, &b, None, Limits::default(), &mut ar);
        assert_eq!(r.relations.len(), b.points.len());
        let q = ar.add(PRIME31, Some(b.points[0]), Some(b.points[1]));
        let limits = Limits {
            extraction_nodes: 0,
            ..Limits::default()
        };
        assert_eq!(
            decompose(PRIME31, &b, q, limits, &mut ar).status,
            "undetermined_extraction"
        );
        assert!(Base::build(PRIME31, 0, &mut ar).is_err());
        let mut invalid = PRIME31;
        invalid.g.1 ^= 1;
        assert!(check_curve(invalid, &mut ar).is_err());
    }
    #[test]
    fn complete_toy_ic_uses_public_target_only() {
        let mut ar = Arithmetic::default();
        let mut e = Curve {
            p: 1031,
            a: 447,
            b: 997,
            n: 991,
            g: (0, 0),
        };
        let rhs = ar.rhs(e, 0);
        e.g = (0, ar.sqrt(rhs, e.p).unwrap());
        check_curve(e, &mut ar).unwrap();
        let q = ar.scalar(e, 137, Some(e.g));
        let r = solve(e, q, 2, 4188, 4000, Limits::default()).unwrap();
        assert_eq!(r.status, "solved");
        assert_eq!(r.scalar, Some(137));
        let r = solve(PRIME31, Some(PRIME31.g), 2, 4188, 0, Limits::default()).unwrap();
        assert_eq!(r.status, "censored_call_limit");
        assert_eq!(r.scalar, None);
    }
}
