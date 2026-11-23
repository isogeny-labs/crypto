//! Cold, complete two-point decomposition baselines for tiny binary fixtures.
//! Every counter includes setup and output work at the API boundary; inversions
//! additionally expose their multiplication/squaring work. This is not a timer.
#![forbid(unsafe_code)]
use std::collections::HashMap;

pub type Point = Option<(u32, u32)>;
pub type Relation = (usize, usize);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counts {
    pub mul: u64,
    pub square: u64,
    pub inverse: u64,
    pub additions: u64,
    pub inserts: u64,
    pub lookups: u64,
    pub rows: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    degree: u32,
    modulus: u32,
}
impl Field {
    /// Only these fixed irreducible fixtures are supported.
    pub fn new(degree: u32) -> Self {
        let modulus = match degree {
            2 => 7,
            3 => 11,
            4 => 19,
            5 => 37,
            8 => 0x11b,
            _ => panic!("unsupported fixture field"),
        };
        Self { degree, modulus }
    }
    pub fn q(self) -> u32 {
        1 << self.degree
    }
    fn raw(self, mut a: u32, mut b: u32) -> u32 {
        let mut z = 0;
        while b != 0 {
            if b & 1 != 0 {
                z ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & self.q() != 0 {
                a ^= self.modulus;
            }
        }
        z
    }
    fn mul(self, a: u32, b: u32, c: &mut Counts) -> u32 {
        c.mul += 1;
        self.raw(a, b)
    }
    fn sq(self, a: u32, c: &mut Counts) -> u32 {
        c.square += 1;
        self.raw(a, a)
    }
    fn inv(self, a: u32, c: &mut Counts) -> u32 {
        assert_ne!(a, 0);
        c.inverse += 1;
        let mut z = 1;
        let mut x = a;
        let mut k = self.q() - 2;
        while k != 0 {
            if k & 1 != 0 {
                z = self.mul(z, x, c);
            }
            k >>= 1;
            if k != 0 {
                x = self.sq(x, c);
            }
        }
        z
    }
    fn batch_inv(self, input: &[u32], c: &mut Counts) -> Vec<u32> {
        // Zero entries are exceptional group-law branches, never inverted.
        let mut prefix = Vec::with_capacity(input.len());
        let mut product = 1;
        for &x in input {
            prefix.push(product);
            if x != 0 {
                product = self.mul(product, x, c);
            }
        }
        let mut out = vec![0; input.len()];
        if input.iter().all(|&x| x == 0) {
            return out;
        }
        let mut suffix = self.inv(product, c);
        for i in (0..input.len()).rev() {
            let x = input[i];
            if x != 0 {
                out[i] = self.mul(prefix[i], suffix, c);
                suffix = self.mul(suffix, x, c);
            }
        }
        out
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Curve {
    pub field: Field,
    pub a: u32,
    pub b: u32,
}
impl Curve {
    pub fn new(degree: u32, a: u32, b: u32) -> Self {
        let field = Field::new(degree);
        assert!(a < field.q() && b > 0 && b < field.q());
        Self { field, a, b }
    }
    pub fn contains(self, p: Point) -> bool {
        let Some((x, y)) = p else {
            return true;
        };
        let f = self.field;
        x < f.q()
            && y < f.q()
            && f.raw(y, y)
                ^ f.raw(x, y)
                ^ f.raw(f.raw(x, x), x)
                ^ f.raw(self.a, f.raw(x, x))
                ^ self.b
                == 0
    }
    pub fn points(self) -> Vec<Point> {
        let mut out = vec![None];
        for x in 0..self.field.q() {
            for y in 0..self.field.q() {
                if self.contains(Some((x, y))) {
                    out.push(Some((x, y)));
                }
            }
        }
        out
    }
    pub fn neg(self, p: Point) -> Point {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn finish(self, p: (u32, u32), q: (u32, u32), slope: u32, c: &mut Counts) -> Point {
        let f = self.field;
        let z = f.sq(slope, c) ^ slope ^ p.0 ^ q.0 ^ self.a;
        Some((z, f.mul(slope, p.0 ^ z, c) ^ z ^ p.1))
    }
    pub fn add(self, p: Point, q: Point, c: &mut Counts) -> Point {
        c.additions += 1;
        let (Some(p), Some(q)) = (p, q) else {
            return p.or(q);
        };
        let f = self.field;
        let slope = if p.0 == q.0 {
            if p.1 != q.1 || p.0 == 0 {
                return None;
            }
            p.0 ^ f.mul(p.1, f.inv(p.0, c), c)
        } else {
            f.mul(p.1 ^ q.1, f.inv(p.0 ^ q.0, c), c)
        };
        self.finish(p, q, slope, c)
    }
    fn check(self, base: &[Point], target: Point, c: &mut Counts) {
        for p in std::iter::once(target).chain(base.iter().copied()) {
            if let Some((x, y)) = p {
                let f = self.field;
                assert!(x < f.q() && y < f.q());
                let x2 = f.sq(x, c);
                let residual =
                    f.sq(y, c) ^ f.mul(x, y, c) ^ f.mul(x2, x, c) ^ f.mul(self.a, x2, c) ^ self.b;
                assert_eq!(residual, 0, "off-curve input");
            }
        }
    }
}

/// Exhaustive reference: no setup table, every ordered pair is tested.
pub fn pair_scan(curve: Curve, base: &[Point], target: Point) -> (Vec<Relation>, Counts) {
    let mut c = Counts::default();
    curve.check(base, target, &mut c);
    let mut out = Vec::new();
    for (i, &p) in base.iter().enumerate() {
        for (j, &q) in base.iter().enumerate() {
            if curve.add(p, q, &mut c) == target {
                out.push((i, j));
            }
        }
    }
    c.rows = out.len() as u64;
    (out, c)
}

fn index(base: &[Point], c: &mut Counts) -> HashMap<Point, Vec<usize>> {
    let mut table = HashMap::<Point, Vec<usize>>::with_capacity(base.len());
    for (j, &p) in base.iter().enumerate() {
        table.entry(p).or_default().push(j);
        c.inserts += 1;
    }
    table
}
fn emit(
    table: &HashMap<Point, Vec<usize>>,
    i: usize,
    rest: Point,
    out: &mut Vec<Relation>,
    c: &mut Counts,
) {
    c.lookups += 1;
    if let Some(js) = table.get(&rest) {
        for &j in js {
            out.push((i, j));
            c.rows += 1;
        }
    }
}

/// Cold baseline: rebuild the full-point membership index for this target.
pub fn subtract(curve: Curve, base: &[Point], target: Point) -> (Vec<Relation>, Counts) {
    let mut c = Counts::default();
    curve.check(base, target, &mut c);
    let table = index(base, &mut c);
    let mut out = Vec::new();
    for (i, &p) in base.iter().enumerate() {
        let rest = curve.add(target, curve.neg(p), &mut c);
        emit(&table, i, rest, &mut out, &mut c);
    }
    (out, c)
}

/// Batch all ordinary subtraction denominators and all doubling denominators
/// together. Identity/inverse/two-torsion branches need no inverse. No pair
/// table or cross-target cache; the index and buffers are charged cold setup.
pub fn subtract_batched(curve: Curve, base: &[Point], target: Point) -> (Vec<Relation>, Counts) {
    let mut c = Counts::default();
    curve.check(base, target, &mut c);
    let f = curve.field;
    let table = index(base, &mut c);
    let mut out = Vec::new();
    let denominators: Vec<_> = base
        .iter()
        .map(|&p| match (target, curve.neg(p)) {
            (Some(t), Some(q)) if t.0 != q.0 => t.0 ^ q.0,
            (Some(t), Some(q)) if t == q && t.0 != 0 => t.0,
            _ => 0,
        })
        .collect();
    let inverses = f.batch_inv(&denominators, &mut c);
    for (i, &p) in base.iter().enumerate() {
        c.additions += 1;
        let neg = curve.neg(p);
        let rest = match (target, neg) {
            (None, q) => q,
            (t, None) => t,
            (Some(t), Some(q)) if t.0 == q.0 && (t.1 != q.1 || t.0 == 0) => None,
            (Some(t), Some(q)) => {
                let slope = if t.0 == q.0 {
                    t.0 ^ f.mul(t.1, inverses[i], &mut c)
                } else {
                    f.mul(t.1 ^ q.1, inverses[i], &mut c)
                };
                curve.finish(t, q, slope, &mut c)
            }
        };
        emit(&table, i, rest, &mut out, &mut c);
    }
    (out, c)
}

#[cfg(test)]
#[path = "../orientation_yx_validation_20261115/validation.rs"]
mod prior_controls;

#[cfg(test)]
mod tests {
    use super::*;
    fn agree(c: Curve, base: &[Point], t: Point) -> Counts {
        let (expected, scan) = pair_scan(c, base, t);
        let (scalar, sc) = subtract(c, base, t);
        let (batch, bc) = subtract_batched(c, base, t);
        assert_eq!(expected, scalar, "scalar {c:?} target {t:?}");
        assert_eq!(expected, batch, "batch {c:?} target {t:?}");
        assert_eq!(scan.additions, (base.len() * base.len()) as u64);
        assert_eq!(sc.additions, base.len() as u64);
        assert_eq!(bc.additions, base.len() as u64);
        assert_eq!(bc.inserts, base.len() as u64);
        assert_eq!(bc.lookups, base.len() as u64);
        assert_eq!(sc.rows, bc.rows);
        assert!(bc.inverse <= 1);
        bc
    }
    #[test]
    fn field_and_batch_inverse_controls() {
        for n in [2, 3, 4, 5, 8] {
            let f = Field::new(n);
            let mut c = Counts::default();
            let input: Vec<_> = (0..f.q()).chain([0, 1, 0]).collect();
            let inverses = f.batch_inv(&input, &mut c);
            assert_eq!(c.inverse, 1);
            for (&x, &inv) in input.iter().zip(&inverses) {
                if x == 0 {
                    assert_eq!(inv, 0);
                } else {
                    assert_eq!(f.raw(x, inv), 1);
                }
            }
            let mut c = Counts::default();
            assert_eq!(f.batch_inv(&[0, 0], &mut c), vec![0, 0]);
            assert_eq!(c.inverse, 0);
            assert!(f.batch_inv(&[], &mut c).is_empty());
        }
    }
    #[test]
    fn complete_relations_and_exceptional_branches() {
        let mut cases = 0;
        for n in [2, 3, 4] {
            for a in [0, 1, 2] {
                for b in [1, 2] {
                    let c = Curve::new(n, a, b);
                    let points = c.points();
                    let mut sparse: Vec<_> = points.iter().step_by(3).copied().collect();
                    // Preserve distinct labels for equal points, and no assumed sign closure.
                    sparse.extend_from_within(..sparse.len().min(3));
                    for &t in &points {
                        for base in [&points[..], &sparse[..], &[][..]] {
                            agree(c, base, t);
                            cases += 1;
                        }
                    }
                }
            }
        }
        eprintln!("exact complete-output cases across 18 small curves: {cases}");
    }
    #[test]
    fn counted_cold_scaling_controls() {
        let c = Curve::new(8, 0, 1);
        let points = c.points();
        let mut targets = vec![
            None,
            *points.iter().find(|p| matches!(p, Some((0, _)))).unwrap(),
        ];
        targets.extend(points.iter().skip(2).step_by(37).take(6).copied());
        assert_eq!(targets.len(), 8);
        for size in [8, 16, 32, 64] {
            let base = &points[..size];
            let mut sums = [Counts::default(); 3];
            for &t in &targets {
                let arms = [
                    pair_scan(c, base, t),
                    subtract(c, base, t),
                    subtract_batched(c, base, t),
                ];
                assert_eq!(arms[0].0, arms[1].0);
                assert_eq!(arms[0].0, arms[2].0);
                agree(c, base, t);
                for (i, (_, counts)) in arms.into_iter().enumerate() {
                    sums[i].mul += counts.mul;
                    sums[i].square += counts.square;
                    sums[i].inverse += counts.inverse;
                    sums[i].additions += counts.additions;
                    sums[i].inserts += counts.inserts;
                    sums[i].lookups += counts.lookups;
                    sums[i].rows += counts.rows;
                }
            }
            // Inversion internals are already included in mul/square. This
            // inequality is fixture-specific; it is not a wall-time assertion.
            assert!(sums[2].mul + sums[2].square < sums[1].mul + sums[1].square);
            eprintln!(
                "B={size} targets=8 pair_scan={:?} scalar={:?} batch={:?}",
                sums[0], sums[1], sums[2]
            );
        }
    }
}
