//! Tian §3.2 symmetric tensors evaluated in the degree-three residue algebra.
//! Traces avoid assuming that the closed support splits over the base field.
use crate::coordinate_ring::{Ring, Space};
use crate::divisor::points_space;
use crate::linear::{self, Kernel, Row, Span};
use quotient_calibration_preflight::field::F;
use serde_json::{json, Value};

fn trace(a: &[Row]) -> F {
    (0..a.len()).fold(F::ZERO, |s, i| s + a[i][i])
}
fn tensor_indices() -> Vec<[usize; 3]> {
    (0..8)
        .flat_map(|i| (i..8).flat_map(move |j| (j..8).map(move |k| [i, j, k])))
        .collect()
}

fn degree_two_ideal(ring: &Ring, d: &Space) -> Span {
    assert_eq!(d.degree, 3);
    assert_eq!(d.divisor_degree, 3);
    let mut kernel = Kernel::new(ring.dimension(2));
    let free = d.span.free_columns();
    for axis in 0..3 {
        let products: Vec<_> = (0..ring.dimension(2))
            .map(|i| {
                d.span
                    .remainder(ring.multiply(&ring.unit(2, i), 2, &ring.unit(1, axis), 1))
            })
            .collect();
        for index in &free {
            kernel.constrain(&products.iter().map(|p| p[*index]).collect::<Row>());
        }
    }
    let result = Span::from_rows(ring.dimension(2), kernel.rows);
    assert_eq!(result.rows.len(), 3, "degree-two divisor ideal dimension");
    result
}

pub fn evaluate(ring: &Ring, d: &Space, coefficients: &[Vec<u8>]) -> Result<Row, String> {
    let ideal = degree_two_ideal(ring, d);
    let basis = ideal.free_columns();
    let operators: Vec<Vec<Row>> = (0..3)
        .map(|axis| {
            let columns: Vec<_> = basis
                .iter()
                .map(|i| {
                    d.span
                        .quotient(ring.multiply(&ring.unit(2, *i), 2, &ring.unit(1, axis), 1))
                })
                .collect();
            (0..3)
                .map(|i| columns.iter().map(|r| r[i]).collect())
                .collect()
        })
        .collect();
    // The ring orders degree-one monomials X,Y,Z.
    assert_eq!(ring.monomials[1], vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    if Span::from_rows(3, operators[2].clone()).rows.len() != 3 {
        return Err("published Z=1 divisor chart meets infinity; no point replaced".into());
    }
    let z_inverse = linear::inverse(&operators[2]);
    let x = linear::multiply(&z_inverse, &operators[0]);
    let y = linear::multiply(&z_inverse, &operators[1]);
    assert_eq!(
        linear::multiply(&x, &y),
        linear::multiply(&y, &x),
        "residue algebra must commute"
    );
    let identity = Span::full(3).rows;
    let x2 = linear::multiply(&x, &x);
    let xy = linear::multiply(&x, &y);
    let y2 = linear::multiply(&y, &y);
    let x3 = linear::multiply(&x2, &x);
    let x2y = linear::multiply(&x2, &y);
    let functions = [identity, x, y, x2, xy, y2, x3, x2y];
    let traces: Vec<_> = functions.iter().map(|f| trace(f)).collect();
    let pair: Vec<Vec<_>> = functions
        .iter()
        .map(|f| {
            functions
                .iter()
                .map(|g| trace(&linear::multiply(f, g)))
                .collect()
        })
        .collect();
    let symmetric: Row = tensor_indices()
        .iter()
        .map(|[i, j, k]| {
            traces[*i] * traces[*j] * traces[*k]
                - pair[*i][*j] * traces[*k]
                - pair[*i][*k] * traces[*j]
                - pair[*j][*k] * traces[*i]
                + F::c(2)
                    * trace(&linear::multiply(
                        &linear::multiply(&functions[*i], &functions[*j]),
                        &functions[*k],
                    ))
        })
        .collect();
    let result: Row = coefficients
        .iter()
        .map(|row| {
            assert_eq!(row.len(), 120);
            row.iter().zip(&symmetric).fold(
                F::ZERO,
                |s, (c, v)| if *c == 0 { s } else { s + F::c(*c) * *v },
            )
        })
        .collect();
    if result.iter().all(|v| *v == F::ZERO) {
        return Err(
            "all-zero raw zeta evaluation: diagonal/collinear chart boundary; no point replaced"
                .into(),
        );
    }
    Ok(linear::normalized(result))
}

fn direct(points: &[[F; 3]], coefficients: &[Vec<u8>]) -> Row {
    assert_eq!(points.len(), 3);
    let functions: Vec<_> = points
        .iter()
        .map(|p| {
            let x = p[0] / p[2];
            let y = p[1] / p[2];
            vec![F::c(1), x, y, x * x, x * y, y * y, x * x * x, x * x * y]
        })
        .collect();
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let values: Row = tensor_indices()
        .iter()
        .map(|indices| {
            permutations.iter().fold(F::ZERO, |s, p| {
                s + functions[0][indices[p[0]]]
                    * functions[1][indices[p[1]]]
                    * functions[2][indices[p[2]]]
            })
        })
        .collect();
    linear::normalized(
        coefficients
            .iter()
            .map(|r| {
                r.iter()
                    .zip(&values)
                    .fold(F::ZERO, |s, (c, v)| s + F::c(*c) * *v)
            })
            .collect(),
    )
}

pub fn historical_controls(ring: &Ring, coefficients: &[Vec<u8>], controls: &Value) -> Value {
    let w = F::c(6).sqrt().expect("F49 embeds in L");
    assert_eq!(w * w + F::c(1), F::ZERO);
    assert_eq!(w.frob(2), w);
    let convert = |value: &Value| {
        let a = value.as_array().unwrap();
        assert_eq!(a.len(), 2);
        F::c(a[0].as_u64().unwrap() as u8) + w * F::c(a[1].as_u64().unwrap() as u8)
    };
    let mut results = vec![];
    for (i, case) in controls["cases"].as_array().unwrap().iter().enumerate() {
        let points: Vec<[F; 3]> = case["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| [convert(&p[0]), convert(&p[1]), F::c(1)])
            .collect();
        let divisor = points_space(ring, 3, &points.iter().map(|p| (*p, 1)).collect::<Vec<_>>());
        let expected: Row = case["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(convert)
            .collect();
        let actual = evaluate(ring, &divisor, coefficients).expect("historical control chart");
        assert!(
            linear::projective_equal(&actual, &expected),
            "historical Kummer control {i}"
        );
        assert_eq!(
            actual,
            direct(&points, coefficients),
            "residue-algebra trace versus direct six-permutation evaluation"
        );
        results.push(json!({"case":i,"matches_historical_values":true,"matches_direct_six_permutation_evaluation":true}));
    }
    json!({"F49_generator_embedding_coefficients":w.0,"cases":results,"completed":results.len()})
}
