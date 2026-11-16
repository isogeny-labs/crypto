use crate::curve::Curve;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Signature {
    pub support: Vec<[u8; 2]>,
    pub degree_x: u8,
    pub degree_y: u8,
}

pub type Polynomial = BTreeMap<[u8; 2], u64>;

/// f3(X,Y,c), expanded directly from its standard short-Weierstrass formula.
pub fn specialize(curve: Curve, c: u64) -> Result<Polynomial, String> {
    if c >= curve.p {
        return Err("specialization coordinate outside field".into());
    }
    let a = i128::from(curve.a);
    let b = i128::from(curve.b);
    let c = i128::from(c);
    let terms = [
        ([2, 2], 1),
        ([2, 1], -2 * c),
        ([1, 2], -2 * c),
        ([2, 0], c * c),
        ([0, 2], c * c),
        ([1, 1], -2 * c * c - 2 * a),
        ([1, 0], -2 * a * c - 4 * b),
        ([0, 1], -2 * a * c - 4 * b),
        ([0, 0], a * a - 4 * b * c),
    ];
    Ok(terms
        .into_iter()
        .filter_map(|(exp, coefficient)| {
            let value = coefficient.rem_euclid(i128::from(curve.p)) as u64;
            (value != 0).then_some((exp, value))
        })
        .collect())
}

pub fn signature(poly: &Polynomial) -> Signature {
    Signature {
        support: poly.keys().copied().collect(),
        degree_x: poly.keys().map(|e| e[0]).max().unwrap_or(0),
        degree_y: poly.keys().map(|e| e[1]).max().unwrap_or(0),
    }
}

pub fn evaluate(curve: Curve, poly: &Polynomial, x: u64, y: u64) -> u64 {
    let power = |mut base: u64, mut exponent: u8| {
        base %= curve.p;
        let mut out = 1;
        while exponent != 0 {
            if exponent & 1 == 1 {
                out = out * base % curve.p;
            }
            base = base * base % curve.p;
            exponent >>= 1;
        }
        out
    };
    poly.iter().fold(0, |sum, (exponent, coefficient)| {
        (sum + (coefficient % curve.p) * power(x, exponent[0]) % curve.p * power(y, exponent[1]))
            % curve.p
    })
}

/// Separate literal expression, with no sparse-polynomial traversal.
pub fn literal(curve: Curve, x: u64, y: u64, c: u64) -> u64 {
    let x = i128::from(x % curve.p);
    let y = i128::from(y % curve.p);
    let c = i128::from(c % curve.p);
    let a = i128::from(curve.a);
    let b = i128::from(curve.b);
    ((x - y).pow(2) * c * c - 2 * ((x + y) * (x * y + a) + 2 * b) * c + (x * y - a).pow(2)
        - 4 * b * (x + y))
        .rem_euclid(i128::from(curve.p)) as u64
}

#[derive(Serialize)]
pub struct Bucket {
    pub signature: Signature,
    pub coordinates: Vec<u64>,
}

#[derive(Serialize)]
pub struct SignatureReport {
    pub curve: Curve,
    pub coordinate_count: usize,
    pub checked_specialized_pairs: usize,
    pub relation_incidents: usize,
    pub buckets: Vec<Bucket>,
    pub trace_reuse_tested: bool,
    pub same_signature_different_roots: Option<Counterexample>,
}

#[derive(Serialize)]
pub struct Counterexample {
    pub left_coordinate: u64,
    pub right_coordinate: u64,
    pub pair: [u64; 2],
    pub left_residual: u64,
    pub right_residual: u64,
}

pub fn experiment(curve: Curve) -> Result<SignatureReport, String> {
    let points = curve.representatives();
    let mut buckets: BTreeMap<Signature, Vec<u64>> = BTreeMap::new();
    let mut checked = 0;
    let mut relations = 0;
    for c in &points {
        let poly = specialize(curve, c.x)?;
        buckets.entry(signature(&poly)).or_default().push(c.x);
        for (i, x) in points.iter().enumerate() {
            for y in &points[i + 1..] {
                if x.x == c.x || y.x == c.x {
                    continue;
                }
                checked += 1;
                let value = evaluate(curve, &poly, x.x, y.x);
                if value != literal(curve, x.x, y.x, c.x) {
                    return Err("sparse expansion disagrees with literal f3".into());
                }
                let group_relation = curve.has_signed_zero(&[*x, *y, *c]);
                if (value == 0) != group_relation {
                    return Err("complete polynomial roots disagree with group-sign oracle".into());
                }
                relations += usize::from(group_relation);
            }
        }
    }
    let mut counterexample = None;
    'search: for u in &points {
        for v in &points {
            if u.x >= v.x
                || signature(&specialize(curve, u.x)?) != signature(&specialize(curve, v.x)?)
            {
                continue;
            }
            for (i, x) in points.iter().enumerate() {
                for y in &points[i + 1..] {
                    if [u.x, v.x].contains(&x.x) || [u.x, v.x].contains(&y.x) {
                        continue;
                    }
                    let left = literal(curve, x.x, y.x, u.x);
                    let right = literal(curve, x.x, y.x, v.x);
                    if (left == 0) != (right == 0) {
                        counterexample = Some(Counterexample {
                            left_coordinate: u.x,
                            right_coordinate: v.x,
                            pair: [x.x, y.x],
                            left_residual: left,
                            right_residual: right,
                        });
                        break 'search;
                    }
                }
            }
        }
    }
    Ok(SignatureReport {
        curve,
        coordinate_count: points.len(),
        checked_specialized_pairs: checked,
        relation_incidents: relations,
        buckets: buckets
            .into_iter()
            .map(|(signature, coordinates)| Bucket {
                signature,
                coordinates,
            })
            .collect(),
        trace_reuse_tested: false,
        same_signature_different_roots: counterexample,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn expanded_formula_matches_literal_on_every_field_triple() {
        for curve in [
            Curve::new(5, 1, 1).unwrap(),
            Curve::new(7, 2, 3).unwrap(),
            Curve::new(17, 2, 2).unwrap(),
        ] {
            for c in 0..curve.p {
                let poly = specialize(curve, c).unwrap();
                for x in 0..curve.p {
                    for y in 0..curve.p {
                        assert_eq!(evaluate(curve, &poly, x, y), literal(curve, x, y, c));
                    }
                }
            }
        }
    }

    #[test]
    fn complete_distinct_coordinate_roots_match_independent_group_signs() {
        for (p, a, b) in [(17, 2, 2), (19, 2, 3), (31, 2, 3)] {
            let c = Curve::new(p, a, b).unwrap();
            let report = experiment(c).unwrap();
            assert!(report.checked_specialized_pairs > 0);
            assert!(!report.trace_reuse_tested);
            let coords: BTreeSet<_> = report
                .buckets
                .iter()
                .flat_map(|b| b.coordinates.iter().copied())
                .collect();
            assert_eq!(coords.len(), report.coordinate_count);
        }
    }

    #[test]
    fn same_signature_is_not_a_certificate_for_reusing_roots() {
        let c = Curve::new(17, 2, 2).unwrap();
        let points = c.representatives();
        let roots = |z| -> BTreeSet<_> {
            let f = specialize(c, z).unwrap();
            (0..c.p)
                .flat_map(|x| (0..c.p).map(move |y| (x, y)))
                .filter(|&(x, y)| evaluate(c, &f, x, y) == 0)
                .collect()
        };
        let mut negative = false;
        for u in &points {
            for v in &points {
                if u.x != v.x
                    && signature(&specialize(c, u.x).unwrap())
                        == signature(&specialize(c, v.x).unwrap())
                    && roots(u.x) != roots(v.x)
                {
                    negative = true;
                }
            }
        }
        assert!(
            negative,
            "fixture must expose incompatible roots within one feature bucket"
        );
        assert!(specialize(c, c.p).is_err());
    }
}
