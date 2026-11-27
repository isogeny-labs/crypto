//! Bounded architecture qualification, not an APS/F4 implementation or benchmark.
//! All fixtures are public, tiny and known-answer. Semantic counters are incomplete.
pub mod partials;
pub mod partitions;
pub mod recursive;

use aps_partition_lab::curve::{Curve, Point};
use serde::Serialize;

pub type Affine = Option<Point>;
pub type PointKey = Option<(u64, u64)>;
pub fn key(p: Affine) -> PointKey {
    p.map(|p| (p.x, p.y))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum FixtureId {
    P17,
    P101,
}

#[derive(Clone, Copy, Debug)]
pub struct Fixture {
    pub(crate) id: FixtureId,
    pub(crate) curve: Curve,
    pub(crate) generator: Point,
    pub(crate) order: u64,
}
impl Fixture {
    pub fn new(id: FixtureId) -> Self {
        let (p, a, b, x, y, order) = match id {
            FixtureId::P17 => (17, 2, 2, 5, 1, 19),
            FixtureId::P101 => (101, 59, 6, 78, 4, 107),
        };
        let curve = Curve::new(p, a, b).expect("fixed nonsingular prime fixture");
        let generator = Point { x, y };
        assert!(curve.contains(generator));
        assert_eq!(curve.mul(Some(generator), order), None);
        Self {
            id,
            curve,
            generator,
            order,
        }
    }
    pub fn point(self, scalar: u64) -> Affine {
        self.curve.mul(Some(self.generator), scalar % self.order)
    }
    pub fn id(self) -> FixtureId {
        self.id
    }
    pub fn order(self) -> u64 {
        self.order
    }
    pub fn neg(self, p: Affine) -> Affine {
        p.map(|p| self.curve.neg(p))
    }
}

/// API events only. Field arithmetic, comparisons, allocation, instructions and
/// serialization are NOT completely counted. No GAE or timing is inferred.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Work {
    pub group_add_calls: u64,
    pub group_mul_calls: u64,
    pub draws: u64,
    pub lookup_calls: u64,
    pub pair_entries_built: u64,
    pub join_candidates: u64,
    pub direct_tuples: u64,
    pub witness_checks: u64,
}
impl Work {
    pub fn add(&mut self, f: Fixture, a: Affine, b: Affine) -> Affine {
        self.group_add_calls += 1;
        f.curve.add(a, b)
    }
    pub fn mul(&mut self, f: Fixture, a: Affine, n: u64) -> Affine {
        self.group_mul_calls += 1;
        f.curve.mul(a, n)
    }
}

pub fn valid_points(f: Fixture, points: &[Affine]) -> bool {
    points.iter().all(|p| p.is_none_or(|p| f.curve.contains(p)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_fixture_groups_and_identity_cases() {
        for id in [FixtureId::P17, FixtureId::P101] {
            let f = Fixture::new(id);
            for i in 0..f.order {
                assert_eq!(f.curve.add(f.point(i), f.neg(f.point(i))), None);
                assert_eq!(f.curve.add(f.point(i), f.point(1)), f.point(i + 1));
            }
        }
    }
}
