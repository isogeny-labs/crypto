//! Certified rational-univariate representation and grevlex change of order.
//! No witness or reference Groebner basis is used by construction.
use super::projection::times;
use super::*;
pub mod compact;
/// Explicit research capacity; the default remains the frozen eight-variable API.
pub const MAX_VARIABLES: usize = if cfg!(feature = "scale-screen") {
    16
} else {
    8
};
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
    pub propagation_policy: &'static str,
    pub coordinate_parents: Vec<Option<usize>>,
    pub derivation_depths: Vec<usize>,
}
fn bounded(row: &Row) -> Result<(), String> {
    let polynomial_degree = degree(&row.poly);
    if row.poly.len() > POLY_CAP || row.proof.len() > PROOF_CAP || polynomial_degree > 64 {
        Err(format!("quotient_budget: polynomial/proof expansion; polynomial_terms={}; proof_terms={}; polynomial_degree={polynomial_degree}; caps={POLY_CAP}/{PROOF_CAP}/64",row.poly.len(),row.proof.len()))
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
// The new path changes enumeration only; the original grid remains the reference.
fn change_order(h: &Row, rules: &[Option<Row>], t: usize, p: u64) -> Result<Vec<Row>, String> {
    if native_kernel_bits() & 64 != 0 {
        change_order_frontier(h, rules, t, p)
    } else {
        change_order_grid(h, rules, t, p)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct GrevlexMonomial(Mono);
impl Ord for GrevlexMonomial {
    fn cmp(&self, other: &Self) -> Ordering {
        super::cmp(&self.0, &other.0)
    }
}
impl PartialOrd for GrevlexMonomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct MonomialFrontier {
    pending: BTreeSet<GrevlexMonomial>,
    seen: BTreeSet<Mono>,
    cap: usize,
}
impl MonomialFrontier {
    fn new(n: usize, cap: usize) -> Self {
        let zero = vec![0; n];
        Self {
            pending: BTreeSet::from([GrevlexMonomial(zero.clone())]),
            seen: BTreeSet::from([zero]),
            cap,
        }
    }
    fn pop(&mut self) -> Option<Mono> {
        self.pending.pop_first().map(|m| m.0)
    }
    // Call only for an independent standard; proper divisors precede children.
    fn expand(&mut self, m: &Mono) {
        if m.iter().sum::<usize>() >= self.cap {
            return;
        }
        for i in 0..m.len() {
            let mut child = m.clone();
            child[i] = child[i].checked_add(1).expect("bounded frontier exponent");
            if self.seen.insert(child.clone()) {
                self.pending.insert(GrevlexMonomial(child));
            }
        }
    }
}
fn change_order_grid(h: &Row, rules: &[Option<Row>], t: usize, p: u64) -> Result<Vec<Row>, String> {
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
fn change_order_frontier(
    h: &Row,
    rules: &[Option<Row>],
    t: usize,
    p: u64,
) -> Result<Vec<Row>, String> {
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
    let mut frontier = MonomialFrontier::new(n, d);
    let mut leading_monomials: Vec<Mono> = Vec::new();
    while let Some(m) = frontier.pop() {
        if leading_monomials.iter().any(|lm| divides(lm, &m)) {
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
            leading_monomials.push(lead(&f).clone());
            basis.push(Row { poly: f, proof });
        } else {
            if columns.len() >= d {
                return Err("quotient: dimension overflow".into());
            }
            frontier.expand(&m);
            standards.push(m);
            columns.push(v);
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
    solve_impl(inputs, n, p, true, false)
}
pub fn solve_product(inputs: &[Poly], n: usize, p: u64) -> Result<(Vec<Row>, Info), String> {
    solve_impl(inputs, n, p, true, true)
}
/// Explicit legacy scheduling ablation: reuse each coordinate immediately in
/// the current edge scan. It has the same equations, checks and budgets.
pub fn solve_input_order(inputs: &[Poly], n: usize, p: u64) -> Result<(Vec<Row>, Info), String> {
    solve_impl(inputs, n, p, false, false)
}
fn solve_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    breadth_first: bool,
    product: bool,
) -> Result<(Vec<Row>, Info), String> {
    if !(3..=MAX_VARIABLES).contains(&n)
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
    let mut parents = vec![None; n];
    let mut depths = vec![0; n];
    known[t] = true;
    loop {
        let mut progressed = false;
        let wave = known.clone();
        for pair in &pairs {
            let [a, b] = pair.variables;
            let available = if breadth_first { &wave } else { &known };
            let (parent, y) = if available[a] && !known[b] {
                (a, b)
            } else if available[b] && !known[a] {
                (b, a)
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
            )
            .map_err(|e| {
                format!(
                    "{e}; phase=coordinate; variable={y}; projected_root_degree={}",
                    root.degree
                )
            })? {
                if !verify(&row, inputs, p) {
                    return Err("quotient: invalid coordinate certificate".into());
                }
                rules[y] = Some(row);
                parents[y] = Some(parent);
                depths[y] = depths[parent] + 1;
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
    // H and every coordinate row have already passed original-input
    // membership checks. This phase needs their polynomial rewrite only;
    // provenance of an input's zero remainder is never used as a certificate.
    let bare = |row: &Row| Row {
        poly: row.poly.clone(),
        proof: Proof::new(),
    };
    let polynomial_h = bare(&h);
    let polynomial_rules: Vec<_> = rules.iter().map(|r| r.as_ref().map(bare)).collect();
    for (i, row) in originals.into_iter().enumerate() {
        if !rewrite(
            Row {
                poly: row.poly,
                proof: Proof::new(),
            },
            &polynomial_h,
            &polynomial_rules,
            t,
            p,
        )
        .map_err(|e| format!("{e}; phase=input-rewrite; input={i}"))?
        .poly
        .is_empty()
        {
            return Err("quotient: original input not in representation".into());
        }
    }
    let basis = change_order(&h, &rules, t, p).map_err(|e| {
        format!(
            "{e}; phase=change-order; derivation_depths={depths:?}; coordinate_parents={parents:?}"
        )
    })?;
    let reduced = basis.iter().enumerate().all(|(i, r)| {
        r.poly[lead(&r.poly)] == 1
            && r.poly.keys().all(|m| {
                basis
                    .iter()
                    .enumerate()
                    .all(|(j, s)| i == j || !divides(lead(&s.poly), m))
            })
    });
    let checked = if product {
        check_reference_product(&basis, inputs, n, p)
    } else {
        check_reference(&basis, inputs, n, p)
    };
    if !reduced || !checked {
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
        propagation_policy: if breadth_first {
            "breadth-first"
        } else {
            "input-order"
        },
        coordinate_parents: parents,
        derivation_depths: depths,
    };
    Ok((basis, info))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::product;

    fn staircase_trace(
        n: usize,
        cap: usize,
        ideal: &[Mono],
        frontier: bool,
    ) -> (Vec<Mono>, Vec<Mono>, usize) {
        let mut border = MonomialFrontier::new(n, cap);
        let mut grid = if frontier {
            Vec::new()
        } else {
            (0..=cap)
                .flat_map(|d| monomials_legacy(n, d, true).into_iter().rev())
                .collect::<Vec<_>>()
        }
        .into_iter();
        let mut known: Vec<Mono> = Vec::new();
        let mut trace = Vec::new();
        let mut standards = Vec::new();
        loop {
            let next = if frontier { border.pop() } else { grid.next() };
            let Some(m) = next else {
                break;
            };
            if known.iter().any(|lm| divides(lm, &m)) {
                continue;
            }
            trace.push(m.clone());
            if ideal.iter().any(|lm| divides(lm, &m)) {
                known.push(m);
            } else {
                if frontier {
                    border.expand(&m);
                }
                standards.push(m);
            }
        }
        (trace, standards, border.seen.len())
    }
    #[test]
    fn frontier_matches_exact_grid_order_on_mixed_nonradical_staircases() {
        for n in [3, 8, 16] {
            for active in [(0, n - 1), (n - 2, n - 1)] {
                for mixed in [false, true] {
                    let mut ideal = Vec::new();
                    for i in 0..n {
                        let mut m = vec![0; n];
                        m[i] = if i == active.0 || i == active.1 { 2 } else { 1 };
                        ideal.push(m);
                    }
                    if mixed {
                        let mut m = vec![0; n];
                        m[active.0] = 1;
                        m[active.1] = 1;
                        ideal.push(m);
                    }
                    let dimension = if mixed { 3 } else { 4 };
                    let old = staircase_trace(n, dimension, &ideal, false);
                    let new = staircase_trace(n, dimension, &ideal, true);
                    assert_eq!(new.0, old.0);
                    assert_eq!(new.1, old.1);
                    assert_eq!(new.1.len(), dimension);
                    assert!(new.2 <= 1 + n * dimension);
                }
            }
        }
        let ideal: Vec<_> = (0..3)
            .map(|i| {
                let mut m = vec![0; 3];
                m[i] = 2;
                m
            })
            .collect();
        let old = staircase_trace(3, 8, &ideal, false);
        let new = staircase_trace(3, 8, &ideal, true);
        assert_eq!(new.0, old.0);
        assert_eq!(new.1, old.1);
        assert_eq!(new.1.len(), 8);
        assert!(new.2 <= 25);
    }
    #[test]
    fn frontier_full_bases_provenance_and_field_counters_equal_grid_on_both_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, MAX_VARIABLES] {
                for root in [0, n - 1] {
                    let edges: Vec<_> = (0..n).filter(|i| *i != root).map(|i| (root, i)).collect();
                    #[cfg(feature = "scale-screen")]
                    let system = workload::generate_scale_graph(p, n, 7, &edges)
                        .unwrap()
                        .system;
                    #[cfg(not(feature = "scale-screen"))]
                    let system = workload::generate_graph(p, n, 7, &edges).unwrap().system;
                    let inputs = system.decode().unwrap();
                    reset_counters();
                    let old = with_native_kernel_bits(39, || solve_product(&inputs, n, p)).unwrap();
                    let counts = serde_json::to_value(counters()).unwrap();
                    reset_counters();
                    let new =
                        with_native_kernel_bits(103, || solve_product(&inputs, n, p)).unwrap();
                    assert_eq!(new.0, old.0);
                    assert_eq!(
                        serde_json::to_value(new.1).unwrap(),
                        serde_json::to_value(old.1).unwrap()
                    );
                    assert_eq!(serde_json::to_value(counters()).unwrap(), counts);
                    assert_eq!(native_kernel_bits(), 0);
                }
            }
        }
    }
    #[test]
    fn frontier_keeps_coordinate_failure_and_forgery_checks() {
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
        let old = change_order_grid(&rows[2], &rules, 2, p).unwrap();
        assert_eq!(change_order_frontier(&rows[2], &rules, 2, p).unwrap(), old);
        rules[0] = None;
        assert_eq!(
            change_order_frontier(&rows[2], &rules, 2, p).unwrap_err(),
            change_order_grid(&rows[2], &rules, 2, p).unwrap_err()
        );
        rules[0] = Some(rows[0].clone());
        axpy(&mut rules[0].as_mut().unwrap().poly, &one(3), 1, p);
        let forged = change_order_frontier(&rows[2], &rules, 2, p).unwrap();
        assert_eq!(forged, change_order_grid(&rows[2], &rules, 2, p).unwrap());
        assert!(!check_reference(&forged, &inputs, 3, p));
    }
    #[test]
    fn polynomial_containment_keeps_full_remainders_and_rejects_nonzero_inputs() {
        let p = 101;
        let t = 2;
        let mut r0 = variable(3, 0);
        axpy(&mut r0, &variable(3, t), p - 1, p);
        let mut r1 = variable(3, 1);
        axpy(&mut r1, &variable(3, t), p - 1, p);
        axpy(&mut r1, &one(3), p - 1, p);
        let h = BTreeMap::from([(vec![0, 0, 2], 1), (vec![0, 0, 0], 100)]);
        let inputs = vec![r0, r1, h];
        let rows = input_rows(&inputs, 3);
        let rules = vec![Some(rows[0].clone()), Some(rows[1].clone()), None];
        let bare = |r: &Row| Row {
            poly: r.poly.clone(),
            proof: Proof::new(),
        };
        let bare_rules: Vec<_> = rules.iter().map(|r| r.as_ref().map(bare)).collect();
        for f in inputs.iter().chain(std::iter::once(&one(3))) {
            let full = rewrite(
                Row {
                    poly: f.clone(),
                    proof: Proof::new(),
                },
                &rows[2],
                &rules,
                t,
                p,
            )
            .unwrap();
            let lean = rewrite(
                Row {
                    poly: f.clone(),
                    proof: Proof::new(),
                },
                &bare(&rows[2]),
                &bare_rules,
                t,
                p,
            )
            .unwrap();
            assert_eq!(full.poly, lean.poly);
            if f == &one(3) {
                assert_eq!(lean.poly, one(3));
            } else {
                assert!(lean.poly.is_empty());
            }
            assert!(lean.proof.is_empty());
        }
    }
    #[test]
    fn six_variable_development_basis_fits_unchanged_caps() {
        for p in [2147483647, 9007199254740881] {
            let system = workload::generate(p, 6, 7).unwrap().system;
            let inputs = system.decode().unwrap();
            let (basis, info) = solve(&inputs, 6, p).unwrap();
            assert_eq!(info.quotient_dimension, 4);
            assert_eq!(info.propagation_policy, "breadth-first");
            assert_eq!(info.derivation_depths.iter().copied().max(), Some(3));
            assert!(basis.iter().all(|r| r.proof.len() <= PROOF_CAP));
            assert!(check_reference_legacy(&basis, &inputs, 6, p));
            if p == 2147483647 {
                let legacy = solve_input_order(&inputs, 6, p).unwrap_err();
                assert!(legacy.contains("phase=change-order"));
                assert!(legacy.contains("[1, 2, 3, 4, 1, 0]"));
            }
        }
    }
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
