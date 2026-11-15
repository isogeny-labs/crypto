//! Independent bounded affine reference. No F4, constructor, factoring or
//! Montgomery arithmetic is used. Enumeration never supplies oracle answers.
use super::{Point, Request, Witness};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Default, Debug, serde::Serialize)]
pub(super) struct Counts {
    pub field_additions: u64,
    pub field_subtractions: u64,
    pub field_multiplications: u64,
    pub inverse_calls: u64,
    pub point_additions: u64,
    pub point_tests: u64,
    pub membership_tests: u64,
    pub signed_tuples: u64,
}

pub(super) struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub counts: Counts,
}

impl Curve {
    pub fn new(r: &Request) -> Self {
        Self {
            p: r.p,
            a: r.a,
            b: r.b,
            counts: Counts::default(),
        }
    }
    fn add_f(&mut self, a: u64, b: u64) -> u64 {
        self.counts.field_additions += 1;
        ((u128::from(a) + u128::from(b)) % u128::from(self.p)) as u64
    }
    fn sub_f(&mut self, a: u64, b: u64) -> u64 {
        self.counts.field_subtractions += 1;
        (a + self.p - b) % self.p
    }
    fn mul_f(&mut self, a: u64, b: u64) -> u64 {
        self.counts.field_multiplications += 1;
        (u128::from(a) * u128::from(b) % u128::from(self.p)) as u64
    }
    fn pow(&mut self, mut a: u64, mut n: u64) -> u64 {
        let mut acc = 1;
        while n != 0 {
            if n & 1 != 0 {
                acc = self.mul_f(acc, a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul_f(a, a);
            }
        }
        acc
    }
    fn inv(&mut self, a: u64) -> u64 {
        assert_ne!(a, 0, "reference inversion requires nonzero input");
        self.counts.inverse_calls += 1;
        self.pow(a, self.p - 2)
    }
    pub fn on_curve(&mut self, q: Point) -> bool {
        self.counts.point_tests += 1;
        if q.x >= self.p || q.y >= self.p {
            return false;
        }
        let lhs = self.mul_f(q.y, q.y);
        let x2 = self.mul_f(q.x, q.x);
        let x3 = self.mul_f(x2, q.x);
        let ax = self.mul_f(self.a, q.x);
        let rhs = self.add_f(x3, ax);
        let rhs = self.add_f(rhs, self.b);
        lhs == rhs
    }
    pub fn member(&mut self, q: Point, r: &Request) -> bool {
        self.counts.membership_tests += 1;
        let power = 1u64 << r.t;
        self.pow(q.x, power) == self.pow(r.g, power)
    }
    pub fn add(&mut self, a: Option<Point>, b: Option<Point>) -> Option<Point> {
        self.counts.point_additions += 1;
        let (Some(a), Some(b)) = (a, b) else {
            return a.or(b);
        };
        let slope = if a.x == b.x {
            if self.add_f(a.y, b.y) == 0 {
                return None;
            }
            let x2 = self.mul_f(a.x, a.x);
            let num = self.mul_f(3, x2);
            let num = self.add_f(num, self.a);
            let den = self.mul_f(2, a.y);
            let inv = self.inv(den);
            self.mul_f(num, inv)
        } else {
            let num = self.sub_f(b.y, a.y);
            let den = self.sub_f(b.x, a.x);
            let inv = self.inv(den);
            self.mul_f(num, inv)
        };
        let s2 = self.mul_f(slope, slope);
        let x = self.sub_f(s2, a.x);
        let x = self.sub_f(x, b.x);
        let dx = self.sub_f(a.x, x);
        let y = self.mul_f(slope, dx);
        let y = self.sub_f(y, a.y);
        Some(Point { x, y })
    }
    pub fn verify(&mut self, r: &Request, witness: &[Point]) -> bool {
        if witness.len() != r.m || !witness.windows(2).all(|w| w[0].x < w[1].x) {
            return false;
        }
        let mut sum = None;
        for &q in witness {
            if !self.on_curve(q) || !self.member(q, r) {
                return false;
            }
            sum = self.add(sum, Some(q));
        }
        sum == Some(r.target)
    }
    pub fn mul(&mut self, mut q: Option<Point>, mut k: u64) -> Option<Point> {
        let mut acc = None;
        while k != 0 {
            if k & 1 != 0 {
                acc = self.add(acc, q);
            }
            k >>= 1;
            if k != 0 {
                q = self.add(q, q);
            }
        }
        acc
    }
}

#[derive(Debug, serde::Serialize)]
pub(super) struct Certificate {
    pub schema: &'static str,
    pub expected_witnesses: usize,
    pub submitted_witnesses: usize,
    pub independently_valid: bool,
    pub complete_set_match: Option<bool>,
    pub reference_counts: Counts,
    pub scope: &'static str,
}

pub(super) fn all(r: &Request) -> Result<(BTreeSet<Witness>, Counts), String> {
    if r.p > 257 {
        return Err("independent exhaustive control is limited to p<=257".into());
    }
    let mut curve = Curve::new(r);
    let mut groups = Vec::new();
    for x in 0..r.p {
        // Scan the field independently: no root-of-unity generator or sqrt API.
        if !curve.member(Point { x, y: 0 }, r) {
            continue;
        }
        let mut ys = Vec::new();
        for y in 0..r.p {
            let q = Point { x, y };
            if curve.on_curve(q) {
                ys.push(q);
            }
        }
        if !ys.is_empty() {
            groups.push(ys);
        }
    }
    let mut combinations = 1u128;
    if groups.len() < r.m {
        return Ok((BTreeSet::new(), curve.counts));
    }
    for i in 0..r.m {
        combinations = combinations * (groups.len() - i) as u128 / (i + 1) as u128;
    }
    if combinations * (1u128 << r.m) > 1_000_000 {
        return Err("independent exhaustive control exceeds one million signed tuples".into());
    }
    fn visit(
        curve: &mut Curve,
        groups: &[Vec<Point>],
        r: &Request,
        from: usize,
        chosen: &mut Witness,
        sum: Option<Point>,
        found: &mut BTreeSet<Witness>,
    ) {
        if chosen.len() == r.m {
            curve.counts.signed_tuples += 1;
            if sum == Some(r.target) {
                found.insert(chosen.clone());
            }
            return;
        }
        let remaining = r.m - chosen.len();
        if groups.len() - from < remaining {
            return;
        }
        for i in from..=groups.len() - remaining {
            for &q in &groups[i] {
                let next = curve.add(sum, Some(q));
                chosen.push(q);
                visit(curve, groups, r, i + 1, chosen, next, found);
                chosen.pop();
            }
        }
    }
    let mut found = BTreeSet::new();
    visit(&mut curve, &groups, r, 0, &mut Vec::new(), None, &mut found);
    Ok((found, curve.counts))
}

pub(super) fn check(
    r: &Request,
    witnesses: &[Witness],
    complete: bool,
) -> Result<Certificate, String> {
    let mut curve = Curve::new(r);
    let actual: BTreeSet<_> = witnesses.iter().cloned().collect();
    if actual.len() != witnesses.len() {
        return Err("duplicate witness in submitted output".into());
    }
    if !witnesses.iter().all(|w| curve.verify(r, w)) {
        return Err("independent original-curve witness verification failed".into());
    }
    let (expected, counts) = all(r)?;
    if !actual.is_subset(&expected) {
        return Err("submitted witness outside exhaustive control".into());
    }
    if complete && actual != expected {
        return Err("complete output lost supported decompositions".into());
    }
    Ok(Certificate {
        schema: "pkm.tiny-reference/v1",
        expected_witnesses: expected.len(),
        submitted_witnesses: actual.len(),
        independently_valid: true,
        complete_set_match: complete.then_some(actual == expected),
        reference_counts: counts,
        scope: "bounded independent correctness control; no performance comparison",
    })
}
