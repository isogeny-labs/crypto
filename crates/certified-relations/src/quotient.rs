//! Certified rational-univariate representation and grevlex change of order.
//! No witness or reference Groebner basis is used by construction.
use super::projection::times;
use super::*;
const POLY_CAP: usize = 5000;
const PROOF_CAP: usize = 50000;
const REDUCTION_CAP: usize = 100000;

#[derive(Clone, Debug, Serialize)]
pub struct Coordinate {
    pub variable: usize,
    pub certificate_product_degree: usize,
    pub certificate_terms: usize,
    pub polynomial_degree: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub root_variable: usize,
    pub quotient_dimension: usize,
    pub coordinates_constructed: usize,
    pub coordinate_certificates: Vec<Coordinate>,
    pub basis_certificate_product_degree: usize,
    pub basis_polynomial_degree: usize,
}
fn bounded(row: &Row) -> Result<(), String> {
    if row.poly.len() > POLY_CAP || row.proof.len() > PROOF_CAP || degree(&row.poly) > 64 {
        Err("quotient_budget: polynomial/proof expansion".into())
    } else {
        Ok(())
    }
}
fn one(n: usize) -> Poly {
    BTreeMap::from([(vec![0; n], 1)])
}
fn variable(n: usize, i: usize) -> Poly {
    let mut m = vec![0; n];
    m[i] = 1;
    BTreeMap::from([(m, 1)])
}
fn is_univariate(f: &Poly, t: usize) -> bool {
    f.keys()
        .all(|m| m.iter().enumerate().all(|(i, e)| i == t || *e == 0))
}
/// Reduce by xi-ri(t), then by a monic H(t). This is a terminating substitution
/// order, not a grevlex normal form. Every subtraction propagates provenance.
fn rewrite(mut row: Row, h: &Row, rules: &[Option<Row>], t: usize, p: u64) -> Result<Row, String> {
    bounded(&row)?;
    let mut steps = 0;
    for (i, rule) in rules.iter().enumerate() {
        if i == t {
            continue;
        }
        let Some(rule) = rule else {
            continue;
        };
        while let Some((m, c)) = row
            .poly
            .iter()
            .find(|(m, _)| m[i] > 0)
            .map(|(m, c)| (m.clone(), *c))
        {
            let mut shift_by = m;
            shift_by[i] -= 1;
            subtract_row(&mut row, &row_shift(rule, &shift_by), c, p);
            steps += 1;
            bounded(&row)?;
            if steps > REDUCTION_CAP {
                return Err("quotient_budget: substitutions".into());
            }
        }
    }
    let d = degree(&h.poly);
    while let Some((m, c)) = row
        .poly
        .iter()
        .find(|(m, _)| m[t] >= d)
        .map(|(m, c)| (m.clone(), *c))
    {
        let mut shift_by = m;
        shift_by[t] -= d;
        subtract_row(&mut row, &row_shift(h, &shift_by), c, p);
        steps += 1;
        bounded(&row)?;
        if steps > REDUCTION_CAP {
            return Err("quotient_budget: substitutions".into());
        }
    }
    Ok(row)
}
/// Extended Euclid in Fp[t], with a checked Bezout identity. Nonunits fail.
fn inverse_mod(u: &Poly, h: &Poly, n: usize, t: usize, p: u64) -> Option<Poly> {
    if u.is_empty() || h.is_empty() || !is_univariate(u, t) || !is_univariate(h, t) {
        return None;
    }
    let mut a = Row {
        poly: u.clone(),
        proof: BTreeMap::from([((0, vec![0; n]), 1)]),
    };
    let mut b = Row {
        poly: h.clone(),
        proof: BTreeMap::from([((1, vec![0; n]), 1)]),
    };
    while !b.poly.is_empty() {
        while !a.poly.is_empty() && degree(&a.poly) >= degree(&b.poly) {
            let am = lead(&a.poly).clone();
            let bm = lead(&b.poly);
            let scalar = mul(a.poly[&am], inv(b.poly[bm], p), p);
            let shift_by = difference(&am, bm);
            subtract_row(&mut a, &row_shift(&b, &shift_by), scalar, p);
            if bounded(&a).is_err() {
                return None;
            }
        }
        (a, b) = (b, a);
    }
    a = monic(a, p);
    if a.poly != one(n) || !verify(&a, &[u.clone(), h.clone()], p) {
        return None;
    }
    Some(
        a.proof
            .iter()
            .filter(|((i, _), _)| *i == 0)
            .map(|((_, m), c)| (m.clone(), *c))
            .collect(),
    )
}
fn coefficient(f: &Poly, y: usize, k: usize) -> Poly {
    f.iter()
        .filter(|(m, _)| m[y] == k)
        .map(|(m, c)| {
            let mut m = m.clone();
            m[y] = 0;
            (m, *c)
        })
        .collect()
}
fn coordinate(
    f: Row,
    g: Row,
    h: &Row,
    rules: &[Option<Row>],
    t: usize,
    y: usize,
    p: u64,
) -> Result<Option<Row>, String> {
    let f = rewrite(f, h, rules, t, p)?;
    let g = rewrite(g, h, rules, t, p)?;
    if f.poly.keys().chain(g.poly.keys()).any(|m| {
        m[y] > 2
            || m.iter()
                .enumerate()
                .any(|(i, e)| i != t && i != y && *e != 0)
    }) {
        return Ok(None);
    }
    let a = coefficient(&f.poly, y, 2);
    let d = coefficient(&g.poly, y, 2);
    let mut linear = times(&g, &a, p);
    subtract_row(&mut linear, &times(&f, &d, p), 1, p);
    linear = rewrite(linear, h, rules, t, p)?;
    if linear.poly.keys().any(|m| m[y] > 1) {
        return Ok(None);
    }
    let u = coefficient(&linear.poly, y, 1);
    let Some(inverse) = inverse_mod(&u, &h.poly, rules.len(), t, p) else {
        return Ok(None);
    };
    let result = rewrite(times(&linear, &inverse, p), h, rules, t, p)?;
    let ypoly = variable(rules.len(), y);
    let mut tail = result.poly.clone();
    axpy(&mut tail, &ypoly, p - 1, p);
    if !is_univariate(&tail, t) || degree(&tail) >= degree(&h.poly) {
        return Ok(None);
    }
    Ok(Some(result))
}
fn qmul(a: &[u64], b: &[u64], h: &[u64], p: u64) -> Vec<u64> {
    let d = h.len() - 1;
    let mut out = vec![0; 2 * d - 1];
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            out[i + j] = ((out[i + j] as u128 + mul(*a, *b, p) as u128) % p as u128) as u64;
        }
    }
    for k in (d..out.len()).rev() {
        let c = out[k];
        out[k] = 0;
        for j in 0..d {
            out[k - d + j] =
                ((out[k - d + j] as u128 + minus(mul(c, h[j], p), p) as u128) % p as u128) as u64;
        }
    }
    out.truncate(d);
    out
}
/// Return coefficients in the independent column vectors, or None for a new
/// independent vector. No dimension/rank heuristic substitutes for final checks.
fn dependence(v: &[u64], columns: &[Vec<u64>], p: u64) -> Option<Vec<u64>> {
    let k = columns.len();
    let mut matrix: Vec<Vec<u64>> = (0..v.len())
        .map(|i| columns.iter().map(|c| c[i]).chain([v[i]]).collect())
        .collect();
    for j in 0..k {
        let r = (j..matrix.len()).find(|r| matrix[*r][j] != 0)?;
        matrix.swap(j, r);
        let scalar = inv(matrix[j][j], p);
        for value in matrix[j].iter_mut().skip(j) {
            *value = mul(*value, scalar, p);
        }
        for r in 0..matrix.len() {
            if r == j {
                continue;
            }
            let scalar = matrix[r][j];
            for c in j..=k {
                matrix[r][c] = ((matrix[r][c] as u128
                    + minus(mul(scalar, matrix[j][c], p), p) as u128)
                    % p as u128) as u64;
            }
        }
    }
    if matrix.iter().skip(k).any(|r| r[k] != 0) {
        None
    } else {
        Some((0..k).map(|j| matrix[j][k]).collect())
    }
}
fn change_order(h: &Row, rules: &[Option<Row>], t: usize, p: u64) -> Result<Vec<Row>, String> {
    let n = rules.len();
    let d = degree(&h.poly);
    let mut hd = vec![0; d + 1];
    for (m, c) in &h.poly {
        hd[m[t]] = *c;
    }
    let mut coords = Vec::new();
    for (i, rule) in rules.iter().enumerate() {
        let mut c = vec![0; d];
        if i == t {
            c[1] = 1;
        } else {
            let rule = rule.as_ref().ok_or("quotient: missing coordinate")?;
            let mut tail = rule.poly.clone();
            axpy(&mut tail, &variable(n, i), p - 1, p);
            for (m, value) in tail {
                c[m[t]] = minus(value, p);
            }
        }
        coords.push(c);
    }
    let mut standards: Vec<Mono> = Vec::new();
    let mut columns = Vec::new();
    let mut basis: Vec<Row> = Vec::new();
    for degree_bound in 0..=d {
        for m in monomials(n, degree_bound, true).into_iter().rev() {
            if basis.iter().any(|r| divides(lead(&r.poly), &m)) {
                continue;
            }
            let mut v = vec![0; d];
            v[0] = 1;
            for (i, e) in m.iter().enumerate() {
                for _ in 0..*e {
                    v = qmul(&v, &coords[i], &hd, p);
                }
            }
            if let Some(coeffs) = dependence(&v, &columns, p) {
                let mut f = BTreeMap::from([(m.clone(), 1)]);
                for (b, c) in standards.iter().zip(coeffs) {
                    if c != 0 {
                        f.insert(b.clone(), minus(c, p));
                    }
                }
                let reduced = rewrite(
                    Row {
                        poly: f.clone(),
                        proof: Proof::new(),
                    },
                    h,
                    rules,
                    t,
                    p,
                )?;
                if !reduced.poly.is_empty() {
                    return Err("quotient: uncertified linear dependence".into());
                }
                let proof = reduced
                    .proof
                    .into_iter()
                    .map(|(key, c)| (key, minus(c, p)))
                    .collect();
                basis.push(Row { poly: f, proof });
            } else {
                if columns.len() >= d {
                    return Err("quotient: dimension overflow".into());
                }
                standards.push(m);
                columns.push(v);
            }
        }
    }
    if columns.len() != d {
        return Err("quotient: dimension mismatch".into());
    }
    basis.sort_by(|a, b| cmp(lead(&b.poly), lead(&a.poly)));
    Ok(basis)
}
/// Caller supplies a validated prime-field system (as with reference()).
/// Unsupported shape/zero divisors/budgets return Err; there is no hidden GB
/// fallback and no use of fixture witnesses or a reference completion oracle.
pub fn solve(inputs: &[Poly], n: usize, p: u64) -> Result<(Vec<Row>, Info), String> {
    if !(3..=8).contains(&n)
        || p < 3
        || inputs.len() > 32
        || inputs
            .iter()
            .any(|f| degree(f) > 4 || f.len() > 32 || f.keys().any(|m| m.len() != n))
    {
        return Err("quotient: unsupported input shape".into());
    }
    let (rows, vertices) = projection::certified_quadratic(inputs, n, p);
    let root = vertices
        .iter()
        .filter(|v| (2..=8).contains(&v.degree))
        .max_by_key(|v| v.variable)
        .ok_or("quotient: no supported certified vertex")?;
    let t = root.variable;
    let h = rows
        .iter()
        .find(|r| is_univariate(&r.poly, t) && degree(&r.poly) == root.degree)
        .ok_or("quotient: missing root polynomial")?
        .clone();
    if !verify(&h, inputs, p) {
        return Err("quotient: invalid root certificate".into());
    }
    let pairs = cheap::consequences(inputs, n, p);
    let originals = input_rows(inputs, n);
    let mut rules = vec![None; n];
    let mut known = vec![false; n];
    known[t] = true;
    loop {
        let mut progressed = false;
        for pair in &pairs {
            let [a, b] = pair.variables;
            let y = if known[a] && !known[b] {
                b
            } else if known[b] && !known[a] {
                a
            } else {
                continue;
            };
            if let Some(row) = coordinate(
                originals[pair.inputs[0]].clone(),
                originals[pair.inputs[1]].clone(),
                &h,
                &rules,
                t,
                y,
                p,
            )? {
                if !verify(&row, inputs, p) {
                    return Err("quotient: invalid coordinate certificate".into());
                }
                rules[y] = Some(row);
                known[y] = true;
                progressed = true;
            }
        }
        if known.iter().all(|x| *x) {
            break;
        }
        if !progressed {
            return Err("quotient: disconnected graph or nonunit subresultant".into());
        }
    }
    for row in originals {
        if !rewrite(row, &h, &rules, t, p)?.poly.is_empty() {
            return Err("quotient: original input not in representation".into());
        }
    }
    let basis = change_order(&h, &rules, t, p)?;
    let reduced = basis.iter().enumerate().all(|(i, r)| {
        r.poly[lead(&r.poly)] == 1
            && r.poly.keys().all(|m| {
                basis
                    .iter()
                    .enumerate()
                    .all(|(j, s)| i == j || !divides(lead(&s.poly), m))
            })
    });
    if !reduced || !check_reference(&basis, inputs, n, p) {
        return Err("quotient: full final basis check failed".into());
    }
    let info = Info {
        root_variable: t,
        quotient_dimension: root.degree,
        coordinates_constructed: rules.iter().flatten().count(),
        coordinate_certificates: rules
            .iter()
            .enumerate()
            .filter_map(|(variable, r)| {
                r.as_ref().map(|r| Coordinate {
                    variable,
                    certificate_product_degree: certificate_degree(r, inputs),
                    certificate_terms: r.proof.len(),
                    polynomial_degree: degree(&r.poly),
                })
            })
            .collect(),
        basis_certificate_product_degree: basis
            .iter()
            .map(|r| certificate_degree(r, inputs))
            .max()
            .unwrap_or(0),
        basis_polynomial_degree: basis.iter().map(|r| degree(&r.poly)).max().unwrap_or(0),
    };
    Ok((basis, info))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::product;
    #[test]
    fn missing_rules_fail_and_forged_rules_cannot_pass_final_check() {
        let p = 101;
        let mut r0 = variable(3, 0);
        axpy(&mut r0, &variable(3, 2), p - 1, p);
        let mut r1 = variable(3, 1);
        axpy(&mut r1, &variable(3, 2), p - 1, p);
        axpy(&mut r1, &one(3), p - 1, p);
        let h = BTreeMap::from([(vec![0, 0, 2], 1), (vec![0, 0, 0], 100)]);
        let inputs = vec![r0, r1, h];
        let rows = input_rows(&inputs, 3);
        let mut rules = vec![Some(rows[0].clone()), Some(rows[1].clone()), None];
        let valid = change_order(&rows[2], &rules, 2, p).unwrap();
        assert!(check_reference(&valid, &inputs, 3, p));
        rules[0] = None;
        assert!(change_order(&rows[2], &rules, 2, p).is_err());
        rules[0] = Some(rows[0].clone());
        axpy(&mut rules[0].as_mut().unwrap().poly, &one(3), 1, p);
        let forged = change_order(&rows[2], &rules, 2, p).unwrap();
        assert!(!check_reference(&forged, &inputs, 3, p));
    }
    #[test]
    fn nonunits_are_not_inverted_and_unit_identity_is_exact() {
        let h = BTreeMap::from([(vec![2], 1), (vec![0], 100)]);
        let nonunit = BTreeMap::from([(vec![1], 1), (vec![0], 100)]);
        assert!(inverse_mod(&nonunit, &h, 1, 0, 101).is_none());
        let unit = BTreeMap::from([(vec![1], 1), (vec![0], 2)]);
        let inverse = inverse_mod(&unit, &h, 1, 0, 101).unwrap();
        let actual = normal_form(
            Row {
                poly: product(&unit, &inverse, 101),
                proof: Proof::new(),
            },
            &[Row {
                poly: h,
                proof: Proof::new(),
            }],
            101,
        );
        assert_eq!(actual.poly, one(1));
    }
    #[test]
    fn quotient_basis_matches_full_reference_on_both_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4] {
                let fixture = workload::generate(p, n, 7).unwrap();
                let inputs = fixture.system.decode().unwrap();
                let (basis, info) = solve(&inputs, n, p).unwrap();
                let reference = reference(&inputs, n, p).unwrap();
                assert_eq!(
                    basis.iter().map(|r| &r.poly).collect::<Vec<_>>(),
                    reference.iter().map(|r| &r.poly).collect::<Vec<_>>()
                );
                assert_eq!(info.quotient_dimension, 4);
                assert_eq!(info.coordinates_constructed, n - 1);
                assert!(check_reference_legacy(&basis, &inputs, n, p));
            }
        }
    }
    #[test]
    fn dependence_requires_exact_linear_membership() {
        let cols = vec![vec![1, 0, 0], vec![0, 1, 0]];
        assert_eq!(dependence(&[3, 7, 0], &cols, 101), Some(vec![3, 7]));
        assert_eq!(dependence(&[3, 7, 1], &cols, 101), None);
    }
    #[test]
    fn unsupported_shapes_are_explicit_failures() {
        assert!(solve(&[], 2, 101).is_err());
        assert!(solve(&[Poly::new()], 3, 101).is_err());
        let f = vec![
            cheap::f3(3, [0, 1], 2, 3, 1, 101),
            cheap::f3(3, [0, 1], 2, 3, 2, 101),
        ];
        assert!(solve(&f, 3, 101).is_err());
    }
}
