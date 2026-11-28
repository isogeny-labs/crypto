use super::*;
fn packed_terms(poly: &Poly, n: usize) -> Option<(Vec<(u128, u64)>, usize)> {
    if n == 0 || n > 16 {
        return None;
    }
    let mut maximum = 0;
    let mut terms = Vec::with_capacity(poly.len());
    for (m, c) in poly {
        if m.len() != n {
            return None;
        }
        let degree = m.iter().try_fold(0usize, |a, b| a.checked_add(*b))?;
        if degree > 64 {
            return None;
        }
        maximum = maximum.max(degree);
        let bits = m
            .iter()
            .enumerate()
            .fold(0u128, |value, (i, e)| value | ((*e as u128) << (7 * i)));
        terms.push((bits, *c));
    }
    Some((terms, maximum))
}
fn unpack(bits: u128, n: usize) -> Mono {
    (0..n).map(|i| ((bits >> (7 * i)) & 127) as usize).collect()
}
fn packed_product(a: &Poly, b: &Poly, p: u64) -> Option<Poly> {
    let n = a.keys().next().or_else(|| b.keys().next())?.len();
    let (a, da) = packed_terms(a, n)?;
    let (b, db) = packed_terms(b, n)?;
    if da + db > 64 {
        return None;
    }
    let mut out = BTreeMap::new();
    for (m, c) in a {
        for (e, d) in &b {
            axpy_owned(&mut out, m + e, *d, c, p);
        }
    }
    Some(out.into_iter().map(|(m, c)| (unpack(m, n), c)).collect())
}
fn packed_times(row: &Row, f: &Poly, p: u64) -> Option<Row> {
    let n = row.poly.keys().next().or_else(|| f.keys().next())?.len();
    let (a, da) = packed_terms(&row.poly, n)?;
    let (f, df) = packed_terms(f, n)?;
    let mut proof = Vec::with_capacity(row.proof.len());
    let mut maximum = 0;
    for ((i, m), c) in &row.proof {
        if m.len() != n {
            return None;
        }
        let degree = m.iter().try_fold(0usize, |a, b| a.checked_add(*b))?;
        if degree > 64 {
            return None;
        }
        maximum = maximum.max(degree);
        let bits = m
            .iter()
            .enumerate()
            .fold(0u128, |value, (i, e)| value | ((*e as u128) << (7 * i)));
        proof.push(((*i, bits), *c));
    }
    if da + df > 64 || maximum + df > 64 {
        return None;
    }
    let mut poly = BTreeMap::new();
    let mut coefficients = BTreeMap::new();
    for (m, c) in f {
        for (e, d) in &a {
            axpy_owned(&mut poly, m + e, *d, c, p);
        }
        for ((i, e), d) in &proof {
            axpy_owned(&mut coefficients, (*i, m + e), *d, c, p);
        }
    }
    Some(Row {
        poly: poly.into_iter().map(|(m, c)| (unpack(m, n), c)).collect(),
        proof: coefficients
            .into_iter()
            .map(|((i, m), c)| ((i, unpack(m, n)), c))
            .collect(),
    })
}

pub(super) fn product(a: &Poly, b: &Poly, p: u64) -> Poly {
    if Mono::PACKED {
        let mut out = Poly::new();
        for (m, c) in a { for (e, d) in b { axpy_owned(&mut out, plus(m,e), *d, *c, p); } }
        return out;
    }
    if native_kernel_bits() & 128 != 0 {
        if let Some(out) = packed_product(a, b, p) {
            return out;
        }
    }
    let mut out = Poly::new();
    for (m, c) in a {
        axpy(&mut out, &shift(b, m), *c, p);
    }
    out
}
pub(super) fn times(row: &Row, f: &Poly, p: u64) -> Row {
    if Mono::PACKED {
        let mut out = Row {poly: Poly::new(), proof: Proof::new()};
        for (m,c) in f {
            for (e,d) in &row.poly {axpy_owned(&mut out.poly,plus(m,e),*d,*c,p);}
            for ((i,e),d) in &row.proof {axpy_owned(&mut out.proof,(*i,plus(m,e)),*d,*c,p);}
        }
        return out;
    }
    if native_kernel_bits() & 128 != 0 {
        if let Some(out) = packed_times(row, f, p) {
            return out;
        }
    }
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
        return BTreeMap::from([(Mono::zero(n), 1)]);
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
            let mut m = Mono::zero(n);
            m.set_exp(y, k);
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
                x.set_exp(y, 0);
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
/// Quadratic resultant and a division-free Bezout identity over Fp[x].
/// f=A*y^2+B*y+C, g=D*y^2+E*y+F, U=AE-BD, V=AF-CD, W=BF-CE.
/// R=V^2-UW=(DU*y+EU-DV)f+(-AU*y+AV-BU)g.
fn quadratic_resultant(f: &Row, g: &Row, variables: [usize; 2], n: usize, p: u64) -> Row {
    let y = variables[1];
    let coefficients = |poly: &Poly| {
        let mut result: [Poly; 3] = std::array::from_fn(|_| Poly::new());
        for (m, c) in poly {
            assert!(m[y] <= 2);
            let mut exponent = m.clone();
            exponent.set_exp(y, 0);
            result[m[y]].insert(exponent, *c);
        }
        result
    };
    let [c, b, a] = coefficients(&f.poly);
    let [ff, e, d] = coefficients(&g.poly);
    let difference = |a: &Poly, b: &Poly, c: &Poly, d: &Poly| {
        let mut out = product(a, b, p);
        axpy(&mut out, &product(c, d, p), p - 1, p);
        out
    };
    let u = difference(&a, &e, &b, &d);
    let v = difference(&a, &ff, &c, &d);
    let w = difference(&b, &ff, &c, &e);
    let mut ym = Mono::zero(n);
    ym.set_exp(y, 1);
    let mut left = shift(&product(&d, &u, p), &ym);
    axpy(&mut left, &difference(&e, &u, &d, &v), 1, p);
    let mut right = difference(&a, &v, &b, &u);
    axpy(&mut right, &shift(&product(&a, &u, p), &ym), p - 1, p);
    let mut out = times(f, &left, p);
    let second = times(g, &right, p);
    axpy(&mut out.poly, &second.poly, 1, p);
    axpy(&mut out.proof, &second.proof, 1, p);
    assert_eq!(out.poly, difference(&v, &v, &u, &w));
    assert!(out.poly.keys().all(|m| m
        .iter()
        .enumerate()
        .all(|(i, e)| i == variables[0] || *e == 0)));
    monic(out, p)
}
fn swap(row: &Row, a: usize, b: usize) -> Row {
    let change = |m: &Mono| {
        let mut m = m.clone();
        m.swap_exp(a, b);
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
        let mut m = Mono::zero(am.len());
        m.set_exp(axis, am[axis] - bm[axis]);
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
/// Kernel invocation counts, not field-operation counts or timing samples.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Work {
    pub edge_resultants: usize,
    pub vertex_gcd_folds: usize,
}
pub fn augment(inputs: &[Poly], n: usize, p: u64) -> (Vec<Poly>, Vec<Vertex>) {
    let (rows, vertices, _) = augment_impl(inputs, n, p, false, None);
    (rows, vertices)
}
/// Alternative construction from original quadratic equations. No solver,
/// witness, division by a polynomial, or saturated-ideal replacement is used.
pub fn augment_quadratic(inputs: &[Poly], n: usize, p: u64) -> (Vec<Poly>, Vec<Vertex>) {
    let (rows, vertices, _) = augment_impl(inputs, n, p, true, None);
    (rows, vertices)
}
/// Preserve the same augmented rows' original-input certificates for quotient
/// construction. The polynomial-only interfaces retain their previous output.
pub fn certified_quadratic(inputs: &[Poly], n: usize, p: u64) -> (Vec<Row>, Vec<Vertex>) {
    let (rows, vertices, _) = all_quadratic(inputs, n, p);
    (rows, vertices)
}
pub(crate) fn all_quadratic(inputs: &[Poly], n: usize, p: u64) -> (Vec<Row>, Vec<Vertex>, Work) {
    let mut rows = Vec::new();
    let (_, info, work) = augment_impl(inputs, n, p, true, Some(&mut rows));
    (rows, info, work)
}
/// Same highest eligible vertex and ordered GCD as all_quadratic, without
/// constructing projections for vertices that selection never examines.
pub(crate) fn selected_quadratic(
    inputs: &[Poly],
    n: usize,
    p: u64,
    pairs: &[cheap::PairCandidate],
) -> Option<(Row, Vertex, Work)> {
    let mut cache: Vec<Option<Row>> = vec![None; pairs.len()];
    let mut work = Work::default();
    for axis in (0..n).rev() {
        let incident: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.variables.contains(&axis))
            .collect();
        if incident.len() < 2 {
            continue;
        }
        let mut projections = Vec::new();
        for (index, pair) in incident {
            let canonical = cache[index].get_or_insert_with(|| {
                work.edge_resultants += 1;
                let row = |i: usize| Row {
                    poly: inputs[i].clone(),
                    proof: BTreeMap::from([((i, Mono::zero(n)), 1)]),
                };
                quadratic_resultant(
                    &row(pair.inputs[0]),
                    &row(pair.inputs[1]),
                    pair.variables,
                    n,
                    p,
                )
            });
            if canonical.poly.is_empty() {
                continue;
            }
            let oriented = if pair.variables[0] == axis {
                canonical.clone()
            } else {
                swap(canonical, pair.variables[0], pair.variables[1])
            };
            assert!(verify(&oriented, inputs, p));
            projections.push(oriented);
        }
        if projections.len() < 2 {
            continue;
        }
        work.vertex_gcd_folds += 1;
        let count = projections.len();
        let mut iter = projections.into_iter();
        let mut root = iter.next().unwrap();
        for other in iter {
            root = gcd(root, other, axis, p);
        }
        let d = degree(&root.poly);
        if root.poly.is_empty() || !(2..=8).contains(&d) {
            continue;
        }
        assert!(verify(&root, inputs, p));
        let vertex = Vertex {
            variable: axis,
            incident_projections: count,
            degree: d,
            certificate_terms: root.proof.len(),
            certificate_product_degree: certificate_degree(&root, inputs),
        };
        return Some((root, vertex, work));
    }
    None
}
/// Opt-in sufficient subset. Full ideal containment is the caller's separate
/// mandatory obligation; a degree bound alone never certifies completeness.
pub(crate) fn subset_quadratic(
    inputs: &[Poly],
    n: usize,
    p: u64,
    pairs: &[cheap::PairMetadata],
) -> Option<(Row, Vertex, Work)> {
    subset_quadratic_impl(inputs, n, p, pairs, false)
        .map(|(row, vertex, work, _)| (row, vertex, work))
}
pub(crate) fn subset_quadratic_dag(
    inputs: &[Poly],
    n: usize,
    p: u64,
    pairs: &[cheap::PairMetadata],
) -> Option<(Row, Vertex, Work, Vec<Row>)> {
    subset_quadratic_impl(inputs, n, p, pairs, true)
}
fn subset_quadratic_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    pairs: &[cheap::PairMetadata],
    retain: bool,
) -> Option<(Row, Vertex, Work, Vec<Row>)> {
    let mut cache: Vec<Option<Row>> = vec![None; pairs.len()];
    let mut work = Work::default();
    let mut definitions = Vec::new();
    for axis in (0..n).rev() {
        let incident: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.variables.contains(&axis))
            .collect();
        if incident.len() < 2 {
            continue;
        }
        let mut root: Option<Row> = None;
        let mut count = 0;
        for (index, pair) in incident {
            let canonical = cache[index].get_or_insert_with(|| {
                work.edge_resultants += 1;
                let row = |i: usize| Row {
                    poly: inputs[i].clone(),
                    proof: BTreeMap::from([((i, Mono::zero(n)), 1)]),
                };
                quadratic_resultant(
                    &row(pair.inputs[0]),
                    &row(pair.inputs[1]),
                    pair.variables,
                    n,
                    p,
                )
            });
            if canonical.poly.is_empty() {
                continue;
            }
            let oriented = if pair.variables[0] == axis {
                canonical.clone()
            } else {
                swap(canonical, pair.variables[0], pair.variables[1])
            };
            assert!(verify(&oriented, inputs, p));
            let oriented = if retain {
                if definitions.len() >= 512 {
                    return None;
                }
                let local = Row {
                    poly: oriented.poly.clone(),
                    proof: BTreeMap::from([((inputs.len() + definitions.len(), Mono::zero(n)), 1)]),
                };
                definitions.push(oriented);
                local
            } else {
                oriented
            };
            count += 1;
            root = Some(if let Some(previous) = root {
                gcd(previous, oriented, axis, p)
            } else {
                oriented
            });
            let r = root.as_ref().unwrap();
            if count >= 2 && (2..=4).contains(&degree(&r.poly)) {
                break;
            }
        }
        let Some(root) = root else { continue };
        let d = degree(&root.poly);
        if count < 2 || root.poly.is_empty() || !(2..=8).contains(&d) {
            continue;
        }
        work.vertex_gcd_folds += 1;
        let extended;
        let generators = if retain {
            extended = inputs
                .iter()
                .cloned()
                .chain(definitions.iter().map(|row| row.poly.clone()))
                .collect::<Vec<_>>();
            extended.as_slice()
        } else {
            inputs
        };
        assert!(verify(&root, generators, p));
        let vertex = Vertex {
            variable: axis,
            incident_projections: count,
            degree: d,
            certificate_terms: root.proof.len(),
            certificate_product_degree: certificate_degree(&root, generators),
        };
        return Some((root, vertex, work, definitions));
    }
    None
}
fn augment_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    quadratic: bool,
    mut certified: Option<&mut Vec<Row>>,
) -> (Vec<Poly>, Vec<Vertex>, Work) {
    let mut work = Work::default();
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
        work.edge_resultants += 1;
        let r = if quadratic {
            let row = |i: usize| Row {
                poly: inputs[i].clone(),
                proof: BTreeMap::from([((i, Mono::zero(n)), 1)]),
            };
            quadratic_resultant(
                &row(pair.inputs[0]),
                &row(pair.inputs[1]),
                pair.variables,
                n,
                p,
            )
        } else {
            resultant(
                &pair.cubic_difference,
                &pair.cubic_syzygy,
                pair.variables,
                n,
                p,
            )
        };
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
        work.vertex_gcd_folds += 1;
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
        if let Some(rows) = certified.as_mut() {
            rows.push(r.clone());
        }
        prefix.push(r.poly);
    }
    for pair in pairs {
        if let Some(rows) = certified.as_mut() {
            rows.push(pair.cubic_difference.clone());
            rows.push(pair.cubic_syzygy.clone());
        }
        prefix.push(pair.cubic_difference.poly);
        prefix.push(pair.cubic_syzygy.poly);
    }
    prefix.extend_from_slice(inputs);
    if let Some(rows) = certified.as_mut() {
        rows.extend(input_rows(inputs, n));
    }
    (prefix, info, work)
}
