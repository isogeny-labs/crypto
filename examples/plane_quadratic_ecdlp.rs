//! Toy ECDLP from a line relation containing one degree-two closed point.
//! The quadratic target is supplied by its curve coordinates and independently
//! checked by elliptic addition over F_25 before accepting the sparse solve.

use std::collections::BTreeMap;

use crypto_lib::cryptanalysis::koblitz_sparse_la::{
    solve_sparse_system, SparseSolveOptions, SparseSolveOutcome,
};
use crypto_lib::cryptanalysis::plane_curve_geometry::{
    PlaneCurve, PlaneLinePlace, PlaneTerm, QuadraticLinePlace,
};
use crypto_lib::cryptanalysis::plane_curve_sparse::MixedPlaneRelationColumns;
use crypto_lib::cryptanalysis::residual_walk::{Curve, Pt};

const P: u64 = 5;
const N: u64 = 7;

/// F_25 = F_5[s]/(s²+2), so s²=3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct F2([u64; 2]);

impl F2 {
    fn constant(value: u64) -> Self {
        Self([value % P, 0])
    }

    fn add(self, other: Self) -> Self {
        Self([(self.0[0] + other.0[0]) % P, (self.0[1] + other.0[1]) % P])
    }

    fn sub(self, other: Self) -> Self {
        Self([
            (self.0[0] + P - other.0[0]) % P,
            (self.0[1] + P - other.0[1]) % P,
        ])
    }

    fn mul(self, other: Self) -> Self {
        let [a, b] = self.0;
        let [c, d] = other.0;
        Self([((a * c) + 3 * b * d) % P, (a * d + b * c) % P])
    }

    fn pow(mut self, mut exponent: u64) -> Self {
        let mut result = Self::constant(1);
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = result.mul(self);
            }
            self = self.mul(self);
            exponent >>= 1;
        }
        result
    }

    fn inv(self) -> Self {
        assert_ne!(self, Self::constant(0));
        self.pow(P * P - 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum E2 {
    Infinity,
    Affine(F2, F2),
}

fn add(a: E2, b: E2) -> E2 {
    let (E2::Affine(x1, y1), E2::Affine(x2, y2)) = (a, b) else {
        return if a == E2::Infinity { b } else { a };
    };
    if x1 == x2 && y1.add(y2) == F2::constant(0) {
        return E2::Infinity;
    }
    let slope = if x1 == x2 {
        x1.mul(x1)
            .mul(F2::constant(3))
            .add(F2::constant(2))
            .mul(y1.mul(F2::constant(2)).inv())
    } else {
        y2.sub(y1).mul(x2.sub(x1).inv())
    };
    let x3 = slope.mul(slope).sub(x1).sub(x2);
    let y3 = slope.mul(x1.sub(x3)).sub(y1);
    E2::Affine(x3, y3)
}

fn affine_quadratic_point(place: &QuadraticLinePlace) -> E2 {
    assert_eq!(place.polynomial, [2, 0, 1]);
    let [x, y, z] = place
        .point
        .map(|coordinate| F2([u64::from(coordinate[0]), u64::from(coordinate[1])]));
    let inverse = z.inv();
    let x = x.mul(inverse);
    let y = y.mul(inverse);
    assert_eq!(
        y.mul(y),
        x.mul(x)
            .mul(x)
            .add(x.mul(F2::constant(2)))
            .add(F2::constant(1))
    );
    E2::Affine(x, y)
}

fn main() {
    let curve = PlaneCurve::new(
        P as u32,
        3,
        &[
            PlaneTerm {
                x_exp: 0,
                y_exp: 2,
                z_exp: 1,
                coeff: 1,
            },
            PlaneTerm {
                x_exp: 3,
                y_exp: 0,
                z_exp: 0,
                coeff: 4,
            },
            PlaneTerm {
                x_exp: 1,
                y_exp: 0,
                z_exp: 2,
                coeff: 3,
            },
            PlaneTerm {
                x_exp: 0,
                y_exp: 0,
                z_exp: 3,
                coeff: 4,
            },
        ],
    )
    .unwrap();
    assert_eq!(curve.certify_geometric_smoothness(), Ok(true));
    let infinity = [0, 1, 0];
    let generator = [0, 1, 1];
    let line_y_one = ([1, 0, 0], [0, 1, 1]);
    let line_infinity = ([1, 0, 0], [0, 1, 0]);
    let relation = curve
        .certified_line_relation_degree_two(line_y_one, line_infinity)
        .unwrap()
        .unwrap();
    let quadratic = relation
        .keys()
        .find_map(|place| match place {
            PlaneLinePlace::Quadratic(key) => Some(*key),
            PlaneLinePlace::Rational(_) => None,
        })
        .expect("line has a quadratic closed point");
    assert_eq!(relation.get(&PlaneLinePlace::Rational(generator)), Some(&1));
    assert_eq!(
        relation.get(&PlaneLinePlace::Quadratic(quadratic)),
        Some(&1)
    );
    assert_eq!(relation.get(&PlaneLinePlace::Rational(infinity)), Some(&-3));

    let q = affine_quadratic_point(&quadratic);
    let conjugate = match q {
        E2::Affine(x, y) => E2::Affine(x.pow(P), y.pow(P)),
        E2::Infinity => unreachable!(),
    };
    assert_ne!(q, conjugate);
    let trace = add(q, conjugate);
    let E2::Affine(trace_x, trace_y) = trace else {
        panic!("quadratic trace is infinity");
    };
    assert_eq!(trace_x.0[1], 0);
    assert_eq!(trace_y.0[1], 0);
    let target = Pt::affine(trace_x.0[0], trace_y.0[0]);

    let columns = MixedPlaneRelationColumns::new_certified(
        &curve,
        infinity,
        vec![
            PlaneLinePlace::Rational(generator),
            PlaneLinePlace::Quadratic(quadratic),
        ],
    )
    .unwrap();
    let rows = vec![
        columns.encode_line_relation(&relation, N).unwrap(),
        columns
            .encode(
                &BTreeMap::from([(PlaneLinePlace::Rational(generator), 1)]),
                0,
                1,
                N,
            )
            .unwrap(),
        columns
            .encode(
                &BTreeMap::from([(PlaneLinePlace::Quadratic(quadratic), 1)]),
                -1,
                0,
                N,
            )
            .unwrap(),
    ];
    let (outcome, _) =
        solve_sparse_system(&rows, columns.columns(), N, &SparseSolveOptions::default());
    let SparseSolveOutcome::Solved(solution) = outcome else {
        panic!("sparse solve failed: {outcome:?}");
    };
    let recovered = solution[columns.target_column() as usize];
    let group = Curve::new(P, 2, 1, N, Pt::affine(0, 1));
    assert_eq!(group.mul(&Pt::affine(0, 1), N), Pt::INFINITY);
    assert_eq!(group.mul(&Pt::affine(0, 1), recovered), target);
    assert_eq!(recovered, 6);
    println!(
        "quadratic plane DLP: degree=2 rows={} columns={} log={recovered} verified=true",
        rows.len(),
        columns.columns()
    );
}
