//! Section-space divisor arithmetic on the pinned nonhyperelliptic curve.
//! Khuri-Makdisi, math/0105182, Lemmas 2.1--2.3.
use crate::coordinate_ring::{Ring, Space};
use crate::linear::{combination, Kernel, Row, Span};
use quotient_calibration_preflight::field::F;
use serde_json::{json, Value};

fn pmul(a: &[F], b: &[F], length: usize) -> Vec<F> {
    let mut out = vec![F::ZERO; length];
    for (i, x) in a.iter().enumerate().take(length) {
        if *x != F::ZERO {
            for (j, y) in b.iter().enumerate().take(length - i) {
                if *y != F::ZERO {
                    out[i + j] = out[i + j] + *x * *y;
                }
            }
        }
    }
    out
}
fn ppow(a: &[F], n: usize, length: usize) -> Vec<F> {
    let mut out = vec![F::ZERO; length];
    out[0] = F::c(1);
    for _ in 0..n {
        out = pmul(&out, a, length);
    }
    out
}
fn curve_series(ring: &Ring, p: &[Vec<F>; 3], length: usize) -> Vec<F> {
    let mut out = vec![F::ZERO; length];
    for (m, c) in &ring.quartic {
        let term = pmul(
            &pmul(
                &ppow(&p[0], m[0], length),
                &ppow(&p[1], m[1], length),
                length,
            ),
            &ppow(&p[2], m[2], length),
            length,
        );
        for (a, b) in out.iter_mut().zip(term) {
            *a = *a + *c * b;
        }
    }
    out
}
fn point_jets(ring: &Ring, point: [F; 3], length: usize) -> [Vec<F>; 3] {
    assert_eq!(
        ring.curve_value(point),
        F::ZERO,
        "point outside fixed quartic"
    );
    let pivot = (0..3).rev().find(|i| point[*i] != F::ZERO).unwrap();
    let p = point.map(|x| x / point[pivot]);
    let free: Vec<_> = (0..3).filter(|i| *i != pivot).collect();
    let derivative = |index: usize| {
        ring.quartic.iter().fold(F::ZERO, |sum, (m, c)| {
            if m[index] == 0 {
                sum
            } else {
                let mut e = *m;
                e[index] -= 1;
                sum + *c
                    * F::c(m[index] as u8)
                    * p[0].pow(e[0] as u64)
                    * p[1].pow(e[1] as u64)
                    * p[2].pow(e[2] as u64)
            }
        })
    };
    let dependent = *free
        .iter()
        .find(|i| derivative(**i) != F::ZERO)
        .expect("singular affine chart");
    let parameter = *free.iter().find(|i| **i != dependent).unwrap();
    let mut out: [Vec<F>; 3] = std::array::from_fn(|i| {
        let mut v = vec![F::ZERO; length];
        v[0] = p[i];
        v
    });
    if length > 1 {
        out[parameter][1] = F::c(1);
    }
    let inverse = derivative(dependent).inv();
    for n in 1..length {
        let coefficient = curve_series(ring, &out, n + 1)[n];
        out[dependent][n] = -coefficient * inverse;
        assert_eq!(curve_series(ring, &out, n + 1)[n], F::ZERO);
    }
    out
}

pub fn points_space(ring: &Ring, degree: usize, points: &[([F; 3], usize)]) -> Space {
    let mut equations = vec![];
    for (point, multiplicity) in points {
        let jets = point_jets(ring, *point, *multiplicity);
        let columns: Vec<_> = ring.monomials[degree]
            .iter()
            .map(|m| {
                pmul(
                    &pmul(
                        &ppow(&jets[0], m[0], *multiplicity),
                        &ppow(&jets[1], m[1], *multiplicity),
                        *multiplicity,
                    ),
                    &ppow(&jets[2], m[2], *multiplicity),
                    *multiplicity,
                )
            })
            .collect();
        for n in 0..*multiplicity {
            equations.push(columns.iter().map(|p| p[n]).collect());
        }
    }
    let kernel = Span::from_rows(ring.dimension(degree), equations).kernel();
    let result = Space::from_rows(ring, degree, points.iter().map(|(_, m)| m).sum(), kernel);
    result.assert_dimension(ring);
    result
}

pub fn contact_space(ring: &Ring, line: [F; 3]) -> Space {
    let pivot = (0..3).find(|i| line[*i] != F::ZERO).unwrap();
    let free: Vec<_> = (0..3).filter(|i| *i != pivot).collect();
    let mut forms = vec![vec![F::ZERO; 2]; 3];
    forms[free[0]][1] = F::c(1);
    forms[free[1]][0] = F::c(1);
    forms[pivot][0] = -line[free[1]] / line[pivot];
    forms[pivot][1] = -line[free[0]] / line[pivot];
    let mut binary = vec![F::ZERO; 5];
    for (m, c) in &ring.quartic {
        let p = pmul(
            &pmul(&ppow(&forms[0], m[0], 5), &ppow(&forms[1], m[1], 5), 5),
            &ppow(&forms[2], m[2], 5),
            5,
        );
        for (a, b) in binary.iter_mut().zip(p) {
            *a = *a + *c * b;
        }
    }
    let degree = binary.iter().rposition(|c| *c != F::ZERO).unwrap();
    assert!(degree.is_multiple_of(2), "line is not a bitangent");
    let inverse = binary[degree].inv();
    for x in &mut binary {
        *x = *x * inverse;
    }
    let mut square_root = vec![F::ZERO; 3];
    if degree == 4 {
        square_root[2] = F::c(1);
        square_root[1] = binary[3] / F::c(2);
        square_root[0] = (binary[2] - square_root[1] * square_root[1]) / F::c(2);
    } else if degree == 2 {
        square_root[1] = F::c(1);
        square_root[0] = binary[1] / F::c(2);
    } else {
        square_root[0] = F::c(1);
    }
    assert_eq!(
        pmul(&square_root, &square_root, 5),
        binary,
        "contact square identity"
    );
    let mut contact = vec![F::ZERO; ring.dimension(2)];
    for (i, c) in square_root.into_iter().enumerate() {
        let mut m = [0; 3];
        m[free[0]] = i;
        m[free[1]] = 2 - i;
        let index = ring.indices[2][&m];
        contact[index] = c;
    }
    let line_form: Vec<_> = ring.monomials[1]
        .iter()
        .map(|m| line[m.iter().position(|e| *e == 1).unwrap()])
        .collect();
    let rows = (0..ring.dimension(2))
        .map(|i| ring.multiply(&line_form, 1, &ring.unit(2, i), 2))
        .chain((0..3).map(|i| ring.multiply(&contact, 2, &ring.unit(1, i), 1)));
    let result = Space::from_rows(ring, 3, 2, rows);
    result.assert_dimension(ring);
    result
}

/// Intersect candidate sections with (s):I(positive), checking every basis
/// product in the exact homogeneous coordinate ring.
fn divide_sections(
    ring: &Ring,
    candidates: &Space,
    positive: &Space,
    s: &[F],
    s_degree: usize,
) -> Vec<Row> {
    assert!(
        4 * positive.degree - positive.divisor_degree >= 6,
        "division requires base-point freeness"
    );
    let total = candidates.degree + positive.degree;
    assert!(total >= s_degree);
    let multiple_degree = total - s_degree;
    let multiples = Span::from_rows(
        ring.dimension(total),
        (0..ring.dimension(multiple_degree))
            .map(|i| ring.multiply(s, s_degree, &ring.unit(multiple_degree, i), multiple_degree)),
    );
    assert_eq!(
        multiples.rows.len(),
        ring.dimension(multiple_degree),
        "multiplication by nonzero section must be injective"
    );
    let free = multiples.free_columns();
    let mut kernel = Kernel::new(candidates.span.rows.len());
    for divisor_section in &positive.span.rows {
        let products: Vec<_> = candidates
            .span
            .rows
            .iter()
            .map(|candidate| {
                multiples.remainder(ring.multiply(
                    candidate,
                    candidates.degree,
                    divisor_section,
                    positive.degree,
                ))
            })
            .collect();
        for i in &free {
            let equation: Row = products.iter().map(|r| r[*i]).collect();
            kernel.constrain(&equation);
        }
    }
    kernel
        .rows
        .into_iter()
        .map(|r| combination(&r, &candidates.span.rows, ring.dimension(candidates.degree)))
        .collect()
}

pub struct Jacobian {
    pub ring: Ring,
    pub contacts: Space,
    pub t: Space,
    pub zero: Space,
    pub generator: Space,
    pub receipts: Vec<Value>,
}
impl Jacobian {
    pub fn new(ring: Ring, lines: &[[F; 3]]) -> Self {
        let spaces: Vec<_> = lines
            .iter()
            .map(|line| contact_space(&ring, *line))
            .collect();
        let contacts = spaces[0]
            .product(&spaces[1], &ring)
            .product(&spaces[2], &ring);
        let a = [F::ZERO, F::c(1), F::ZERO];
        let infinity = [F::c(1), F::ZERO, F::ZERO];
        let t = points_space(&ring, 3, &[(a, 2), (infinity, 1)]);
        let placeholder = t.clone();
        let mut result = Self {
            ring,
            contacts,
            t,
            zero: placeholder.clone(),
            generator: placeholder,
            receipts: vec![],
        };
        // B = D9+D19+D26 - (2*A+P_infinity), with H=2*A+2*P_infinity.
        let pos = result.contacts.clone();
        let neg = result.t.promote(9, &result.ring);
        result.zero = result.riemann_roch(&pos, &neg, "origin B");
        // D*+B = Q+N -3*O-P_infinity; no scalar label enters this construction.
        let q = [F::c(4), F::c(6), F::c(3)];
        let o = [F::c(1), F::c(5), F::c(5)];
        let qspace = points_space(&result.ring, 3, &[(q, 1)]);
        let pos = qspace.product(&result.contacts, &result.ring);
        let neg = points_space(&result.ring, 3, &[(o, 3), (infinity, 1)]).promote(12, &result.ring);
        result.generator = result.riemann_roch(&pos, &neg, "certified anchor plus B");
        result
    }
    pub fn riemann_roch(&mut self, positive: &Space, negative: &Space, label: &str) -> Space {
        assert_eq!(positive.divisor_degree - negative.divisor_degree, 3);
        let d = positive.degree;
        let s = positive.span.rows[0].clone();
        let candidate = negative.promote(d, &self.ring);
        let rr = divide_sections(&self.ring, &candidate, positive, &s, d);
        assert!(!rr.is_empty(), "degree-three Riemann--Roch space is empty");
        let h = rr[0].clone();
        let effective_degree = 4 * d - positive.divisor_degree + negative.divisor_degree;
        let k = (effective_degree + 7).div_ceil(4);
        let e_rows = divide_sections(
            &self.ring,
            &negative.promote(k, &self.ring),
            positive,
            &s,
            d,
        );
        let effective = Space::from_rows(&self.ring, k, effective_degree, e_rows);
        effective.assert_dimension(&self.ring);
        assert!(4 * k - effective_degree >= 7);
        let residual_rows = divide_sections(&self.ring, &self.ring.full(3), &effective, &h, d);
        let result = Space::from_rows(&self.ring, 3, 3, residual_rows);
        result.assert_dimension(&self.ring);
        self.receipts.push(json!({"label":label,"positive_degree":positive.divisor_degree,"negative_degree":negative.divisor_degree,"section_degree":d,"riemann_roch_dimension":rr.len(),"residual_test_degree":k,"residual_ideal_dimension":result.span.rows.len(),"all_basis_product_constraints_verified":true}));
        eprintln!(
            "target RR {}: {} (dimension {})",
            self.receipts.len(),
            label,
            rr.len()
        );
        result
    }
    pub fn add(&mut self, left: &Space, right: &Space) -> Space {
        let positive = left.product(right, &self.ring).product(&self.t, &self.ring);
        self.riemann_roch(
            &positive,
            &self.contacts.clone(),
            "Jacobian addition relative to B",
        )
    }
    pub fn negative(&mut self, divisor: &Space) -> Space {
        let a = [F::ZERO, F::c(1), F::ZERO];
        let infinity = [F::c(1), F::ZERO, F::ZERO];
        // 2*B ~ H+2*P_infinity = 2*A+4*P_infinity.
        let positive = points_space(&self.ring, 4, &[(a, 2), (infinity, 4)]);
        let negative = divisor.promote(4, &self.ring);
        self.riemann_roch(&positive, &negative, "residual negation")
    }
    pub fn equal_class(&mut self, left: &Space, right: &Space) -> bool {
        // L(D_left-D_right) is nonzero exactly for linear equivalence.
        let d = left.degree;
        let s = left.span.rows[0].clone();
        let space = divide_sections(&self.ring, right, left, &s, d);
        !space.is_empty()
    }
}
