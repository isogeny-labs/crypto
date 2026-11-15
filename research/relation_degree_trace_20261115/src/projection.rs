//! Certified edge resultants and shared-vertex univariate gcd consequences.
//! No Groebner basis or witness is used in construction.
use super::*;
fn product(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut out = Poly::new();
    for (m, c) in a {
        axpy(&mut out, &shift(b, m), *c, p);
    }
    out
}
fn times(row: &Row, f: &Poly, p: u64) -> Row {
    let mut out = Row {
        poly: Poly::new(),
        proof: Proof::new(),
    };
    for (m, c) in f {
        axpy(&mut out.poly, &shift(&row.poly, m), *c, p);
        axpy(&mut out.proof, &proof_shift(&row.proof, m), *c, p);
    }
    out
}
fn determinant(
    matrix: &[Vec<Poly>],
    rows: u8,
    cols: u8,
    n: usize,
    p: u64,
    memo: &mut BTreeMap<(u8, u8), Poly>,
) -> Poly {
    if rows == 0 {
        return BTreeMap::from([(vec![0; n], 1)]);
    }
    if let Some(f) = memo.get(&(rows, cols)) {
        return f.clone();
    }
    let i = rows.trailing_zeros() as usize;
    let mut out = Poly::new();
    let mut position = 0;
    for j in 0..matrix.len() {
        if cols & (1 << j) == 0 {
            continue;
        }
        let minor = determinant(matrix, rows & !(1 << i), cols & !(1 << j), n, p, memo);
        let f = product(&matrix[i][j], &minor, p);
        axpy(&mut out, &f, if position % 2 == 0 { 1 } else { p - 1 }, p);
        position += 1;
    }
    memo.insert((rows, cols), out.clone());
    out
}
fn resultant(g: &Row, c: &Row, variables: [usize; 2], n: usize, p: u64) -> Row {
    let y = variables[1];
    let dy = |f: &Poly| f.keys().map(|m| m[y]).max().unwrap_or(0);
    let dg = dy(&g.poly);
    let dc = dy(&c.poly);
    let size = dg + dc;
    assert!(size <= 5);
    let mut rows = Vec::new();
    for (r, count) in [(g, dc), (c, dg)] {
        for k in 0..count {
            let mut m = vec![0; n];
            m[y] = k;
            rows.push(row_shift(r, &m));
        }
    }
    let matrix: Vec<Vec<Poly>> = rows
        .iter()
        .map(|r| {
            let mut coefficients = vec![Poly::new(); size];
            for (m, c) in &r.poly {
                let mut x = m.clone();
                let k = x[y];
                x[y] = 0;
                coefficients[k].insert(x, *c);
            }
            coefficients
        })
        .collect();
    let mask = (1u8 << size) - 1;
    let mut memo = BTreeMap::new();
    let mut out = Row {
        poly: Poly::new(),
        proof: Proof::new(),
    };
    for (i, r) in rows.iter().enumerate() {
        let mut cofactor = determinant(&matrix, mask & !(1 << i), mask & !1, n, p, &mut memo);
        if i % 2 == 1 {
            for c in cofactor.values_mut() {
                *c = minus(*c, p);
            }
        }
        let term = times(r, &cofactor, p);
        axpy(&mut out.poly, &term.poly, 1, p);
        axpy(&mut out.proof, &term.proof, 1, p);
    }
    assert!(out.poly.keys().all(|m| m
        .iter()
        .enumerate()
        .all(|(i, e)| i == variables[0] || *e == 0)));
    monic(out, p)
}
fn swap(row: &Row, a: usize, b: usize) -> Row {
    let change = |m: &Mono| {
        let mut m = m.clone();
        m.swap(a, b);
        m
    };
    Row {
        poly: row.poly.iter().map(|(m, c)| (change(m), *c)).collect(),
        proof: row
            .proof
            .iter()
            .map(|((i, m), c)| ((*i, change(m)), *c))
            .collect(),
    }
}
fn remainder(mut a: Row, b: &Row, axis: usize, p: u64) -> Row {
    while !a.poly.is_empty() && degree(&a.poly) >= degree(&b.poly) {
        let am = lead(&a.poly);
        let bm = lead(&b.poly);
        let scalar = mul(a.poly[am], inv(b.poly[bm], p), p);
        let mut m = vec![0; am.len()];
        m[axis] = am[axis] - bm[axis];
        subtract_row(&mut a, &row_shift(b, &m), scalar, p);
    }
    a
}
fn gcd(mut a: Row, mut b: Row, axis: usize, p: u64) -> Row {
    a = monic(a, p);
    b = monic(b, p);
    while !b.poly.is_empty() {
        let r = remainder(a, &b, axis, p);
        a = b;
        b = monic(r, p);
    }
    monic(a, p)
}
#[derive(Clone, Debug, Serialize)]
pub struct Vertex {
    pub variable: usize,
    pub incident_projections: usize,
    pub degree: usize,
    pub certificate_terms: usize,
    pub certificate_product_degree: usize,
}
pub fn augment(inputs: &[Poly], n: usize, p: u64) -> (Vec<Poly>, Vec<Vertex>) {
    let pairs = cheap::consequences(inputs, n, p);
    let mut incidence = BTreeMap::<usize, usize>::new();
    for pair in &pairs {
        for v in pair.variables {
            *incidence.entry(v).or_default() += 1;
        }
    }
    let mut projections = BTreeMap::<usize, Vec<Row>>::new();
    for pair in &pairs {
        if pair.variables.iter().all(|v| incidence[v] < 2) {
            continue;
        }
        let r = resultant(
            &pair.cubic_difference,
            &pair.cubic_syzygy,
            pair.variables,
            n,
            p,
        );
        if r.poly.is_empty() {
            continue;
        }
        assert!(verify(&r, inputs, p));
        let other = swap(&r, pair.variables[0], pair.variables[1]);
        assert!(verify(&other, inputs, p));
        projections.entry(pair.variables[0]).or_default().push(r);
        projections
            .entry(pair.variables[1])
            .or_default()
            .push(other);
    }
    let mut prefix = Vec::new();
    let mut info = Vec::new();
    for (axis, rows) in projections {
        if rows.len() < 2 {
            continue;
        }
        let count = rows.len();
        let mut iter = rows.into_iter();
        let mut r = iter.next().unwrap();
        for other in iter {
            r = gcd(r, other, axis, p);
        }
        if r.poly.is_empty() {
            continue;
        }
        assert!(verify(&r, inputs, p));
        info.push(Vertex {
            variable: axis,
            incident_projections: count,
            degree: degree(&r.poly),
            certificate_terms: r.proof.len(),
            certificate_product_degree: certificate_degree(&r, inputs),
        });
        prefix.push(r.poly);
    }
    for pair in pairs {
        prefix.push(pair.cubic_difference.poly);
        prefix.push(pair.cubic_syzygy.poly);
    }
    prefix.extend_from_slice(inputs);
    (prefix, info)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edge_projection_is_certified_and_reused_vertex_gcds_are_quartic() {
        for p in [2147483647, 9007199254740881] {
            let fixture = workload::generate(p, 3, 7).unwrap();
            let f = fixture.system.decode().unwrap();
            let (a, info) = augment(&f, 3, p);
            assert_eq!(info.len(), 3);
            assert!(info.iter().all(|v| v.degree == 4));
            let gb = reference(&f, 3, p).unwrap();
            let ab = reference(&a, 3, p).unwrap();
            assert_eq!(
                gb.iter().map(|r| r.poly.clone()).collect::<BTreeSet<_>>(),
                ab.iter().map(|r| r.poly.clone()).collect()
            );
        }
    }
    #[test]
    fn no_shared_vertex_falls_back_to_cubics() {
        let fixture = workload::generate(2147483647, 2, 7).unwrap();
        let f = fixture.system.decode().unwrap();
        let (a, info) = augment(&f, 2, fixture.system.prime);
        assert!(info.is_empty());
        assert_eq!(a.len(), 4);
    }
}
