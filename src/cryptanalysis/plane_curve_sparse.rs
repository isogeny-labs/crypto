//! Sparse modular rows from rational plane-curve divisor relations.
//!
//! The reserved last column is the logarithm of a public target class.
//! Line-divisor relations have target coefficient and right-hand side zero;
//! descent rows can fill them later. A modular solution is only a linear
//! candidate until checked in the curve's Jacobian.

use std::collections::BTreeMap;

use super::koblitz_sparse_la::SparseRow;
use super::plane_curve_geometry::PlaneCurve;

pub type RationalPlanePoint = [u32; 3];

/// Stable columns for factor-base points, excluding the chosen base point.
#[derive(Clone, Debug)]
pub struct PlaneRelationColumns {
    p: u32,
    base_point: RationalPlanePoint,
    points: Vec<RationalPlanePoint>,
    index: BTreeMap<RationalPlanePoint, u32>,
}

impl PlaneRelationColumns {
    /// Points must use the first nonzero projective coordinate scaled to one.
    pub fn new(
        curve: &PlaneCurve,
        base_point: RationalPlanePoint,
        points: Vec<RationalPlanePoint>,
    ) -> Result<Self, &'static str> {
        let p = curve.p;
        let canonical = |point: &RationalPlanePoint| {
            point.iter().all(|&coordinate| coordinate < p)
                && point.iter().find(|&&coordinate| coordinate != 0) == Some(&1)
        };
        let valid = |point: &RationalPlanePoint| {
            canonical(point) && curve.eval(*point) == 0 && curve.gradient(*point) != [0; 3]
        };
        if !valid(&base_point) {
            return Err("base point is not a smooth canonical curve point");
        }
        if points.len() >= u32::MAX as usize {
            return Err("too many factor-base columns");
        }
        let mut index = BTreeMap::new();
        for (i, &point) in points.iter().enumerate() {
            if !valid(&point) || point == base_point || index.insert(point, i as u32).is_some() {
                return Err("invalid or repeated factor-base point");
            }
        }
        Ok(Self {
            p,
            base_point,
            points,
            index,
        })
    }

    /// Number of factor-base columns plus the reserved target column.
    pub fn columns(&self) -> usize {
        self.points.len() + 1
    }

    pub fn target_column(&self) -> u32 {
        self.points.len() as u32
    }

    pub fn field_order(&self) -> u32 {
        self.p
    }

    /// Encode `Σ c_P log(P−P0) + target_coeff·log(T) = rhs (mod r)`.
    /// The caller supplies an odd prime `r < 2^63` and must verify any
    /// recovered target scalar on the original Jacobian.
    pub fn encode(
        &self,
        coefficients: &BTreeMap<RationalPlanePoint, i64>,
        target_coeff: i64,
        rhs: i128,
        modulus: u64,
    ) -> Result<SparseRow<u64>, &'static str> {
        if modulus < 3 || modulus >= (1u64 << 63) || modulus.is_multiple_of(2) {
            return Err("modulus must be odd and below 2^63");
        }
        let reduced = |value: i128| value.rem_euclid(modulus as i128) as u64;
        let mut entries = Vec::with_capacity(coefficients.len() + 1);
        for (&point, &coefficient) in coefficients {
            if point == self.base_point {
                continue;
            }
            let Some(&column) = self.index.get(&point) else {
                return Err("relation uses a point outside the factor base");
            };
            entries.push((column, reduced(coefficient as i128)));
        }
        entries.push((self.target_column(), reduced(target_coeff as i128)));
        Ok(SparseRow::new(entries, reduced(rhs), modulus))
    }

    /// Encode a principal relation returned by `PlaneCurve::rational_line_relation`.
    pub fn encode_line_relation(
        &self,
        relation: &BTreeMap<RationalPlanePoint, i32>,
        modulus: u64,
    ) -> Result<SparseRow<u64>, &'static str> {
        let coefficients = relation
            .iter()
            .map(|(&point, &coefficient)| (point, i64::from(coefficient)))
            .collect();
        self.encode(&coefficients, 0, 0, modulus)
    }
}
