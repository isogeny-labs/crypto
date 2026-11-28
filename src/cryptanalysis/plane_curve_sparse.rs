//! Sparse modular rows from rational plane-curve divisor relations.
//!
//! The reserved last column is the logarithm of a public target class.
//! Line-divisor relations have target coefficient and right-hand side zero;
//! descent rows can fill them later. A modular solution is only a linear
//! candidate until checked in the curve's Jacobian.

use std::collections::BTreeMap;

use super::koblitz_sparse_la::SparseRow;
use super::plane_curve_geometry::{PlaneCurve, PlaneLinePlace};
use super::residual_walk::is_prime_u64;

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
    /// Construct columns only after the bounded geometric smoothness
    /// certificate succeeds. This keeps certified line rows and their
    /// factor-base points on the same checked plane model.
    pub fn new_certified(
        curve: &PlaneCurve,
        base_point: RationalPlanePoint,
        points: Vec<RationalPlanePoint>,
    ) -> Result<Self, &'static str> {
        if !curve.certify_geometric_smoothness()? {
            return Err("plane curve is geometrically singular");
        }
        Self::new(curve, base_point, points)
    }

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
        if modulus < 3 || modulus >= (1u64 << 63) || !is_prime_u64(modulus) {
            return Err("modulus must be an odd prime below 2^63");
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

/// Sparse columns for rational and degree-two places. A quadratic place
/// represents the class `[Q - 2P0]`; a rational place represents `[P - P0]`.
/// Principal line-ratio relations therefore use their original divisor
/// multiplicities as logarithm coefficients.
#[derive(Clone, Debug)]
pub struct MixedPlaneRelationColumns {
    base_point: RationalPlanePoint,
    places: Vec<PlaneLinePlace>,
    index: BTreeMap<PlaneLinePlace, u32>,
}

impl MixedPlaneRelationColumns {
    pub fn new_certified(
        curve: &PlaneCurve,
        base_point: RationalPlanePoint,
        places: Vec<PlaneLinePlace>,
    ) -> Result<Self, &'static str> {
        if !curve.certify_geometric_smoothness()? {
            return Err("plane curve is geometrically singular");
        }
        Self::new(curve, base_point, places)
    }

    pub fn new(
        curve: &PlaneCurve,
        base_point: RationalPlanePoint,
        places: Vec<PlaneLinePlace>,
    ) -> Result<Self, &'static str> {
        PlaneRelationColumns::new(curve, base_point, Vec::new())?;
        if places.len() >= u32::MAX as usize {
            return Err("too many factor-base columns");
        }
        let canonical_rational = |point: &[u32; 3]| {
            point.iter().all(|&coordinate| coordinate < curve.p)
                && point.iter().find(|&&coordinate| coordinate != 0) == Some(&1)
                && curve.eval(*point) == 0
                && curve.gradient(*point) != [0; 3]
        };
        let mut index = BTreeMap::new();
        for (i, &place) in places.iter().enumerate() {
            let valid = match place {
                PlaneLinePlace::Rational(point) => {
                    point != base_point && canonical_rational(&point)
                }
                PlaneLinePlace::Quadratic(key) => curve.valid_quadratic_line_place(&key),
            };
            if !valid || index.insert(place, i as u32).is_some() {
                return Err("invalid or repeated factor-base place");
            }
        }
        Ok(Self {
            base_point,
            places,
            index,
        })
    }

    pub fn columns(&self) -> usize {
        self.places.len() + 1
    }

    pub fn target_column(&self) -> u32 {
        self.places.len() as u32
    }

    pub fn encode(
        &self,
        coefficients: &BTreeMap<PlaneLinePlace, i64>,
        target_coeff: i64,
        rhs: i128,
        modulus: u64,
    ) -> Result<SparseRow<u64>, &'static str> {
        if modulus < 3 || modulus >= (1u64 << 63) || !is_prime_u64(modulus) {
            return Err("modulus must be an odd prime below 2^63");
        }
        let reduced = |value: i128| value.rem_euclid(modulus as i128) as u64;
        let mut entries = Vec::with_capacity(coefficients.len() + 1);
        for (&place, &coefficient) in coefficients {
            if place == PlaneLinePlace::Rational(self.base_point) {
                continue;
            }
            let Some(&column) = self.index.get(&place) else {
                return Err("relation uses a place outside the factor base");
            };
            entries.push((column, reduced(coefficient as i128)));
        }
        entries.push((self.target_column(), reduced(target_coeff as i128)));
        Ok(SparseRow::new(entries, reduced(rhs), modulus))
    }

    pub fn encode_line_relation(
        &self,
        relation: &BTreeMap<PlaneLinePlace, i32>,
        modulus: u64,
    ) -> Result<SparseRow<u64>, &'static str> {
        let coefficients = relation
            .iter()
            .map(|(&place, &coefficient)| (place, i64::from(coefficient)))
            .collect();
        self.encode(&coefficients, 0, 0, modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::plane_curve_geometry::PlaneTerm;

    #[test]
    fn mixed_conic_relation_maps_to_sparse_columns() {
        let curve = PlaneCurve::new(
            3,
            2,
            &[
                PlaneTerm {
                    x_exp: 2,
                    y_exp: 0,
                    z_exp: 0,
                    coeff: 1,
                },
                PlaneTerm {
                    x_exp: 0,
                    y_exp: 2,
                    z_exp: 0,
                    coeff: 1,
                },
                PlaneTerm {
                    x_exp: 0,
                    y_exp: 0,
                    z_exp: 2,
                    coeff: 1,
                },
            ],
        )
        .unwrap();
        let relation = curve
            .certified_line_relation_degree_two(([1, 0, 0], [0, 1, 0]), ([1, 1, 0], [0, 0, 1]))
            .unwrap()
            .unwrap();
        let base_point = relation
            .keys()
            .find_map(|place| match place {
                PlaneLinePlace::Rational(point) => Some(*point),
                PlaneLinePlace::Quadratic(_) => None,
            })
            .unwrap();
        let places: Vec<_> = relation
            .keys()
            .copied()
            .filter(|place| *place != PlaneLinePlace::Rational(base_point))
            .collect();
        let columns =
            MixedPlaneRelationColumns::new_certified(&curve, base_point, places.clone()).unwrap();
        assert_eq!(columns.columns(), 3);
        let row = columns.encode_line_relation(&relation, 7).unwrap();
        assert_eq!(row.rhs, 0);
        assert_eq!(row.entries, vec![(0, 6), (1, 1)]);
        assert!(columns.encode_line_relation(&relation, 9).is_err());
        assert!(
            MixedPlaneRelationColumns::new(&curve, base_point, places[1..].to_vec())
                .unwrap()
                .encode_line_relation(&relation, 7)
                .is_err()
        );
        let mut forged = places.clone();
        for place in &mut forged {
            if let PlaneLinePlace::Quadratic(key) = place {
                key.point[0][0] = 0;
            }
        }
        assert!(MixedPlaneRelationColumns::new(&curve, base_point, forged).is_err());
    }
}
