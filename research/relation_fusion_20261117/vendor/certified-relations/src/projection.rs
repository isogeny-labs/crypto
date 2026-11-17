//! Certified edge resultants and shared-vertex univariate gcd consequences.
//! No Groebner basis or witness is used in construction.
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

#[cfg(test)]
mod packed_product_tests {
    use super::*;
    #[test]
    fn packed_products_and_local_coefficients_equal_legacy_with_counters() {
        let mut state = 0x73ae19u64;
        for p in [2147483647u64, 9007199254740881] {
            for n in [3, 4, 6, 16, 17] {
                for _ in 0..12 {
                    let mut make = || {
                        let mut f = Poly::new();
                        for _ in 0..12 {
                            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                            let mut m = vec![0; n];
                            m[state as usize % n] += 1;
                            m[(state >> 17) as usize % n] += 1;
                            f.insert(m, state % p);
                        }
                        f
                    };
                    let a = make();
                    let b = make();
                    let row = Row {
                        poly: a.clone(),
                        proof: a
                            .iter()
                            .enumerate()
                            .map(|(i, (m, c))| ((i % 3, m.clone()), *c))
                            .collect(),
                    };
                    reset_counters();
                    let old = with_native_kernel_bits(103, || product(&a, &b, p));
                    let count = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new = with_native_kernel_bits(231, || product(&a, &b, p));
                    assert_eq!(old, new);
                    assert_eq!(count, serde_json::to_value(counters()).unwrap());
                    reset_counters();
                    let old = with_native_kernel_bits(103, || times(&row, &b, p));
                    let count = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new = with_native_kernel_bits(231, || times(&row, &b, p));
                    assert_eq!(old, new);
                    assert_eq!(count, serde_json::to_value(counters()).unwrap());
                }
            }
        }
    }
    #[test]
    fn cancellation_empty_and_outside_packing_domain_keep_exact_semantics() {
        let p = 2147483647;
        let a = Poly::from([(vec![1, 0, 0], 1), (vec![0, 1, 0], 1)]);
        let b = Poly::from([(vec![1, 0, 0], 1), (vec![0, 1, 0], p - 1)]);
        let expected = with_native_kernel_bits(103, || product(&a, &b, p));
        let actual = with_native_kernel_bits(231, || product(&a, &b, p));
        assert_eq!(expected, actual);
        assert!(!actual.contains_key(&vec![1, 1, 0]));
        for left in [
            Poly::new(),
            Poly::from([(vec![65, 0, 0], 1)]),
            Poly::from([(vec![1], 1)]),
        ] {
            assert_eq!(
                with_native_kernel_bits(103, || product(&left, &b, p)),
                with_native_kernel_bits(231, || product(&left, &b, p))
            );
        }
    }
}
pub(super) fn product(a: &Poly, b: &Poly, p: u64) -> Poly {
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
            exponent[y] = 0;
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
    let mut ym = vec![0; n];
    ym[y] = 1;
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
                    proof: BTreeMap::from([((i, vec![0; n]), 1)]),
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
        let mut root: Option<Row> = None;
        let mut count = 0;
        for (index, pair) in incident {
            let canonical = cache[index].get_or_insert_with(|| {
                work.edge_resultants += 1;
                let row = |i: usize| Row {
                    poly: inputs[i].clone(),
                    proof: BTreeMap::from([((i, vec![0; n]), 1)]),
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
                proof: BTreeMap::from([((i, vec![0; n]), 1)]),
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
#[cfg(test)]
mod tests {
    use super::*;
    fn embedded_cycle(n: usize, p: u64) -> Vec<Poly> {
        workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap()
            .into_iter()
            .map(|f| {
                f.into_iter()
                    .map(|(mut m, c)| {
                        m.resize(n, 0);
                        (m, c)
                    })
                    .collect()
            })
            .collect()
    }
    fn compare_selected(inputs: &[Poly], n: usize, p: u64) -> (Vertex, Work) {
        let pairs = cheap::consequences(inputs, n, p);
        let (selected, vertex, work) = selected_quadratic(inputs, n, p, &pairs).unwrap();
        let (all, vertices, all_work) = all_quadratic(inputs, n, p);
        let expected = vertices
            .iter()
            .filter(|v| (2..=8).contains(&v.degree))
            .max_by_key(|v| v.variable)
            .unwrap();
        assert_eq!(vertex.variable, expected.variable);
        assert_eq!(vertex.degree, expected.degree);
        let row = all
            .iter()
            .find(|r| {
                r.poly.keys().all(|m| {
                    m.iter()
                        .enumerate()
                        .all(|(i, e)| i == expected.variable || *e == 0)
                }) && degree(&r.poly) == expected.degree
            })
            .unwrap();
        assert_eq!(&selected, row, "original cofactor row must match exactly");
        assert!(verify(&selected, inputs, p));
        assert!(work.edge_resultants <= all_work.edge_resultants);
        (vertex, work)
    }
    #[test]
    fn selected_root_skips_unused_and_single_incident_vertices() {
        for p in [2147483647, 9007199254740881] {
            for n in [4, 6] {
                let mut inputs = embedded_cycle(n, p);
                if n == 6 {
                    for z in [0, 1] {
                        inputs.push(cheap::f3(n, [4, 5], 2, 3, z, p));
                    }
                }
                let (root, work) = compare_selected(&inputs, n, p);
                assert_eq!(root.variable, 2);
                assert_eq!(work.edge_resultants, 2);
                assert_eq!(work.vertex_gcd_folds, 1);
                assert!(quotient::compact::solve(&inputs, n, p).is_err());
                assert!(quotient::compact::solve_all_vertices(&inputs, n, p).is_err());
            }
        }
    }
    #[test]
    fn selected_root_caches_edges_when_higher_vertices_are_ineligible() {
        let p = 2147483647;
        let mut inputs = embedded_cycle(6, p);
        for (edge, a, b) in [([3, 4], 2, 3), ([4, 5], 5, 7), ([3, 5], 9, 11)] {
            for z in [0, 1] {
                inputs.push(cheap::f3(6, edge, a, b, z, p));
            }
        }
        let (root, work) = compare_selected(&inputs, 6, p);
        assert_eq!(root.variable, 2);
        // Three upper vertices rejected, then the lower cycle supplies a root.
        // Shared upper edges are each constructed once despite that fallback.
        assert_eq!(work.edge_resultants, 5);
        assert_eq!(work.vertex_gcd_folds, 4);
    }
    #[test]
    fn direct_quadratic_identity_agrees_with_sylvester_in_degenerate_and_scaled_cases() {
        for p in [5, 7, 101] {
            for (a, b) in [(0, 0), (0, 3 % p), (2, 3 % p), (p - 1, p - 1)] {
                for (u, v) in [(0, 1), (1, 2), (1, p - 1)] {
                    let mut f = vec![
                        cheap::f3(2, [0, 1], a, b, u, p),
                        cheap::f3(2, [0, 1], a, b, v, p),
                    ];
                    for c in f[1].values_mut() {
                        *c = mul(*c, p - 1, p);
                    }
                    let rows = input_rows(&f, 2);
                    let direct = quadratic_resultant(&rows[0], &rows[1], [0, 1], 2, p);
                    let determinant = resultant(&rows[0], &rows[1], [0, 1], 2, p);
                    assert_eq!(direct.poly, determinant.poly);
                    assert!(verify(&direct, &f, p));
                    assert!(certificate_degree(&direct, &f) <= 11);
                }
            }
        }
    }
    #[test]
    fn quadratic_projection_preserves_full_ideal_on_both_requested_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4] {
                let fixture = workload::generate(p, n, 7).unwrap();
                let f = fixture.system.decode().unwrap();
                let (new, info) = augment_quadratic(&f, n, p);
                let (old, _) = augment(&f, n, p);
                assert_eq!(new,old,"construction refactor must preserve the ordered augmented inputs on these fixtures");
                assert_eq!(info.len(), n);
                assert!(info.iter().all(|v| v.degree == 4));
                let original = reference(&f, n, p).unwrap();
                let augmented = reference_criteria(&new, n, p).unwrap();
                assert_eq!(
                    original
                        .iter()
                        .map(|r| r.poly.clone())
                        .collect::<BTreeSet<_>>(),
                    augmented.iter().map(|r| r.poly.clone()).collect()
                );
            }
        }
    }
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
