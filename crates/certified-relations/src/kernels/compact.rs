use super::*;

const BASIS_CAP: usize = 256;
const IDENTITY_WORK_CAP: usize = 1_000_000;

#[derive(Clone, Debug)]
pub struct Certificate {
    /// Definition j may reference only originals and definitions before j.
    pub definitions: Vec<Row>,
    /// Final rows may reference all already-certified definitions.
    pub basis: Vec<Row>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub root_projection_definitions: usize,
    pub batch_groups_attempted: usize,
    pub batch_groups_completed: usize,
    pub coordinates_batched: usize,
    pub batch_groups_fallback: usize,
    pub root_variable: usize,
    pub quotient_dimension: usize,
    pub definition_nodes: usize,
    pub stored_proof_terms: usize,
    pub stored_definition_proof_terms: usize,
    pub stored_basis_proof_terms: usize,
    pub max_local_certificate_product_degree: usize,
    /// A bound after recursive substitution, before possible cancellation.
    pub expanded_original_product_degree_upper_bound: usize,
    pub basis_polynomial_degree: usize,
    pub coordinate_parents: Vec<Option<usize>>,
    pub derivation_depths: Vec<usize>,
    pub projection_work: projection::Work,
}
pub struct Output {
    pub certificate: Certificate,
    pub info: Info,
}
fn mono_degree(m: &Mono) -> Option<usize> {
    m.iter().try_fold(0usize, |s, e| s.checked_add(*e))
}
fn valid_poly(f: &Poly, n: usize, p: u64, max_degree: usize, max_terms: usize) -> bool {
    f.len() <= max_terms
        && f.iter().all(|(m, c)| {
            m.len() == n && *c > 0 && *c < p && mono_degree(m).is_some_and(|d| d <= max_degree)
        })
}
fn admitted(inputs: &[Poly], n: usize, p: u64) -> bool {
    (3..=MAX_VARIABLES).contains(&n)
        && p >= 3
        && p <= i64::MAX as u64
        && prime(p)
        && !inputs.is_empty()
        && inputs.len() <= 32
        && inputs.iter().all(|f| valid_poly(f, n, p, 4, 32))
}
/// Validate public rows before trusted-shape arithmetic. Returned work counts
/// referenced polynomial terms for one identity, not total field operations.
fn row_shape(row: &Row, generators: &[Poly], n: usize, p: u64) -> Option<(usize, usize)> {
    if row.poly.is_empty()
        || !valid_poly(&row.poly, n, p, 64, POLY_CAP)
        || row.proof.len() > PROOF_CAP
    {
        return None;
    }
    let mut work = 0usize;
    let mut local_degree = 0;
    for ((i, m), c) in &row.proof {
        let input = generators.get(*i)?;
        let product_degree = mono_degree(m)?.checked_add(degree(input))?;
        if m.len() != n || *c == 0 || *c >= p || product_degree > 64 {
            return None;
        }
        work = work.checked_add(input.len())?;
        if work > IDENTITY_WORK_CAP {
            return None;
        }
        local_degree = local_degree.max(product_degree);
    }
    Some((work, local_degree))
}
fn reduced(basis: &[Row]) -> bool {
    basis.iter().enumerate().all(|(i, row)| {
        row.poly[lead(&row.poly)] == 1
            && row.poly.keys().all(|m| {
                basis
                    .iter()
                    .enumerate()
                    .all(|(j, other)| i == j || !divides(lead(&other.poly), m))
            })
    })
}
/// Check acyclic membership identities, both ideal containments, reducedness
/// and every S-pair. This is exact arithmetic, not an independent signature.
pub fn verify(certificate: &Certificate, inputs: &[Poly], n: usize, p: u64) -> bool {
    verify_impl(certificate, inputs, n, p, false, n)
}
pub fn verify_product(certificate: &Certificate, inputs: &[Poly], n: usize, p: u64) -> bool {
    verify_impl(certificate, inputs, n, p, true, n)
}
/// Explicit v2 definition budget; all identity and whole-basis checks remain.
pub fn verify_dag_product(certificate: &Certificate, inputs: &[Poly], n: usize, p: u64) -> bool {
    verify_impl(certificate, inputs, n, p, true, 512)
}
fn verify_impl(
    certificate: &Certificate,
    inputs: &[Poly],
    n: usize,
    p: u64,
    product: bool,
    definition_cap: usize,
) -> bool {
    if !admitted(inputs, n, p)
        || certificate.definitions.len() > definition_cap
        || certificate.basis.is_empty()
        || certificate.basis.len() > BASIS_CAP
    {
        return false;
    }
    let mut generators = inputs.to_vec();
    let mut stored_terms = 0usize;
    let mut identity_work = 0usize;
    for row in &certificate.definitions {
        let Some((work, _)) = row_shape(row, &generators, n, p) else {
            return false;
        };
        stored_terms += row.proof.len();
        identity_work += work;
        if stored_terms > PROOF_CAP
            || identity_work > IDENTITY_WORK_CAP
            || !super::verify(row, &generators, p)
        {
            return false;
        }
        // No forward/self reference can reach this append: row_shape used
        // only the strictly earlier generator list.
        generators.push(row.poly.clone());
    }
    for row in &certificate.basis {
        let Some((work, _)) = row_shape(row, &generators, n, p) else {
            return false;
        };
        stored_terms += row.proof.len();
        identity_work += work;
        if stored_terms > PROOF_CAP || identity_work > IDENTITY_WORK_CAP {
            return false;
        }
    }
    // check_reference verifies final row identities as well as all inputs
    // and S-pairs; definitions already have inductive original membership.
    reduced(&certificate.basis)
        && if product {
            check_reference_product(&certificate.basis, &generators, n, p)
        } else {
            check_reference(&certificate.basis, &generators, n, p)
        }
}
struct Chain {
    definition_cap: usize,
    generators: Vec<Poly>,
    definitions: Vec<Row>,
    stored_terms: usize,
    identity_work: usize,
    n: usize,
    p: u64,
}
impl Chain {
    fn push(&mut self, row: Row) -> Result<Row, String> {
        let (work, _) = row_shape(&row, &self.generators, self.n, self.p)
            .ok_or("compact: invalid local identity shape/reference/budget")?;
        if self.definitions.len() >= self.definition_cap
            || self.stored_terms + row.proof.len() > PROOF_CAP
            || self.identity_work + work > IDENTITY_WORK_CAP
        {
            return Err("compact: definition/term/identity-work budget".into());
        }
        if !super::verify(&row, &self.generators, self.p) {
            return Err("compact: false local identity".into());
        }
        let index = self.generators.len();
        let polynomial = row.poly.clone();
        self.stored_terms += row.proof.len();
        self.identity_work += work;
        self.generators.push(polynomial.clone());
        self.definitions.push(row);
        Ok(Row {
            poly: polynomial,
            proof: BTreeMap::from([((index, Mono::zero(self.n)), 1)]),
        })
    }
}
fn original_bound(row: &Row, bounds: &[usize]) -> usize {
    row.proof
        .keys()
        .map(|(i, m)| mono_degree(m).unwrap() + bounds[*i])
        .max()
        .unwrap_or(0)
}
/// Same polynomial construction as the expanded control, with a different
/// retained certificate representation. No oracle, witness or hidden fallback.
pub fn solve(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    solve_impl(inputs, n, p, false, false, false, false)
}
pub fn solve_product(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    solve_impl(inputs, n, p, false, false, true, false)
}
/// Share checked edge-resultant nodes through root GCD; explicit v2 only.
pub fn solve_sparse_dag_product(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    solve_impl(inputs, n, p, false, true, true, true)
}
pub struct SparseOutput {
    pub output: Output,
    pub subset_succeeded: bool,
    pub subset_error: Option<String>,
}
/// Try a short certified root consequence, then the unchanged full constructor.
/// Both attempts run in the caller's interval. No output bypasses final checks.
pub fn solve_sparse(inputs: &[Poly], n: usize, p: u64) -> Result<SparseOutput, String> {
    solve_sparse_impl(inputs, n, p, false)
}
pub fn solve_sparse_product(inputs: &[Poly], n: usize, p: u64) -> Result<SparseOutput, String> {
    solve_sparse_impl(inputs, n, p, true)
}
fn solve_sparse_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    product: bool,
) -> Result<SparseOutput, String> {
    match solve_impl(inputs, n, p, false, true, product, false) {
        Ok(output) => Ok(SparseOutput {
            output,
            subset_succeeded: true,
            subset_error: None,
        }),
        Err(error) => Ok(SparseOutput {
            output: if product {
                solve_product(inputs, n, p)?
            } else {
                solve(inputs, n, p)?
            },
            subset_succeeded: false,
            subset_error: Some(error),
        }),
    }
}
/// Explicit ablation: eagerly construct every vertex before selecting a root.
/// Polynomial kernels, proof representation and final checks match solve.
pub fn solve_all_vertices(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    solve_impl(inputs, n, p, true, false, false, false)
}
fn solve_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    all_vertices: bool,
    subset: bool,
    product: bool,
    dag: bool,
) -> Result<Output, String> {
    if !admitted(inputs, n, p) {
        return Err("compact: unsupported prime/input shape".into());
    }
    let mut root_definitions = Vec::new();
    let (root_row, root, projection_work, pairs) = if dag {
        let pairs = cheap::pair_metadata(inputs, n, p);
        let (row, root, work, definitions) = projection::subset_quadratic_dag(inputs, n, p, &pairs)
            .ok_or("compact-dag: no supported subset vertex")?;
        root_definitions = definitions;
        (row, root, work, pairs)
    } else if subset {
        let pairs = cheap::pair_metadata(inputs, n, p);
        let (row, root, work) = projection::subset_quadratic(inputs, n, p, &pairs)
            .ok_or("compact: no supported subset vertex")?;
        (row, root, work, pairs)
    } else if all_vertices {
        let (projected, vertices, work) = projection::all_quadratic(inputs, n, p);
        let root = vertices
            .into_iter()
            .filter(|v| (2..=8).contains(&v.degree))
            .max_by_key(|v| v.variable)
            .ok_or("compact: no supported certified vertex")?;
        let row = projected
            .into_iter()
            .find(|r| is_univariate(&r.poly, root.variable) && degree(&r.poly) == root.degree)
            .ok_or("compact: missing root polynomial")?;
        (
            row,
            root,
            work,
            cheap::consequences(inputs, n, p)
                .iter()
                .map(|pair| cheap::PairMetadata {
                    inputs: pair.inputs,
                    variables: pair.variables,
                })
                .collect(),
        )
    } else {
        let pairs = cheap::consequences(inputs, n, p);
        let (row, root, work) = projection::selected_quadratic(inputs, n, p, &pairs)
            .ok_or("compact: no supported certified vertex")?;
        (
            row,
            root,
            work,
            pairs
                .iter()
                .map(|pair| cheap::PairMetadata {
                    inputs: pair.inputs,
                    variables: pair.variables,
                })
                .collect(),
        )
    };
    let t = root.variable;
    let root_projection_definitions = root_definitions.len();
    let mut chain = Chain {
        definition_cap: if dag { 512 } else { n },
        generators: inputs.to_vec(),
        definitions: Vec::new(),
        stored_terms: 0,
        identity_work: 0,
        n,
        p,
    };
    for definition in root_definitions {
        chain.push(definition)?;
    }
    let h = chain.push(root_row)?;
    let originals = input_rows(inputs, n);
    let mut rules = vec![None; n];
    let mut known = vec![false; n];
    let mut parents = vec![None; n];
    let mut depths = vec![0; n];
    known[t] = true;
    let mut batch_groups_attempted = 0usize;
    let mut batch_groups_completed = 0usize;
    let mut coordinates_batched = 0usize;
    loop {
        let mut progressed = false;
        let wave = known.clone();
        if native_kernel_bits() & 256 != 0 {
            let mut selected = Vec::new();
            let mut seen = BTreeSet::new();
            for pair in &pairs {
                let [a, b] = pair.variables;
                let candidate = if wave[a] && !known[b] {
                    Some((a, b))
                } else if wave[b] && !known[a] {
                    Some((b, a))
                } else {
                    None
                };
                if let Some((parent, y)) = candidate {
                    if seen.insert(y) {
                        selected.push((pair.inputs, parent, y));
                    }
                }
            }
            if selected.len() >= 2 {
                batch_groups_attempted += 1;
                let candidates = selected
                    .iter()
                    .map(|(indices, _, y)| {
                        (
                            originals[indices[0]].clone(),
                            originals[indices[1]].clone(),
                            *y,
                        )
                    })
                    .collect();
                if let Some(rows) = coordinate_batch(candidates, &h, &rules, t, p)? {
                    batch_groups_completed += 1;
                    coordinates_batched += rows.len();
                    for (row, (_, parent, y)) in rows.into_iter().zip(selected) {
                        rules[y] = Some(chain.push(row)?);
                        parents[y] = Some(parent);
                        depths[y] = depths[parent] + 1;
                        known[y] = true;
                        progressed = true;
                    }
                }
            }
        }
        for pair in &pairs {
            let [a, b] = pair.variables;
            let (parent, y) = if wave[a] && !known[b] {
                (a, b)
            } else if wave[b] && !known[a] {
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
            .map_err(|e| format!("{e}; compact phase=coordinate; variable={y}"))?
            {
                rules[y] = Some(chain.push(row)?);
                parents[y] = Some(parent);
                depths[y] = depths[parent] + 1;
                known[y] = true;
                progressed = true;
            }
        }
        if known.iter().all(|v| *v) {
            break;
        }
        if !progressed {
            return Err("compact: disconnected graph or nonunit subresultant".into());
        }
    }
    let bare = |row: &Row| Row {
        poly: row.poly.clone(),
        proof: Proof::new(),
    };
    let bare_rules: Vec<_> = rules.iter().map(|r| r.as_ref().map(bare)).collect();
    for row in &originals {
        if !rewrite(bare(row), &bare(&h), &bare_rules, t, p)?
            .poly
            .is_empty()
        {
            return Err("compact: original input not in representation".into());
        }
    }
    let basis =
        change_order(&h, &rules, t, p).map_err(|e| format!("{e}; compact phase=change-order"))?;
    let certificate = Certificate {
        definitions: chain.definitions,
        basis,
    };
    if !(if dag {
        verify_dag_product(&certificate, inputs, n, p)
    } else if product {
        verify_product(&certificate, inputs, n, p)
    } else {
        verify(&certificate, inputs, n, p)
    }) {
        return Err("compact: final chain/basis verification or budget failed".into());
    }
    let mut bounds: Vec<_> = inputs.iter().map(degree).collect();
    let mut generators = inputs.to_vec();
    let mut max_local = 0;
    let mut max_original_bound = 0;
    for row in certificate.definitions.iter().chain(&certificate.basis) {
        max_local = max_local.max(row_shape(row, &generators, n, p).unwrap().1);
        let bound = original_bound(row, &bounds);
        max_original_bound = max_original_bound.max(bound);
        bounds.push(bound);
        generators.push(row.poly.clone());
    }
    let definition_terms = certificate.definitions.iter().map(|r| r.proof.len()).sum();
    let basis_terms = certificate.basis.iter().map(|r| r.proof.len()).sum();
    let info = Info {
        root_projection_definitions,
        batch_groups_attempted,
        batch_groups_completed,
        coordinates_batched,
        batch_groups_fallback: batch_groups_attempted - batch_groups_completed,
        root_variable: t,
        quotient_dimension: root.degree,
        definition_nodes: certificate.definitions.len(),
        stored_proof_terms: definition_terms + basis_terms,
        stored_definition_proof_terms: definition_terms,
        stored_basis_proof_terms: basis_terms,
        max_local_certificate_product_degree: max_local,
        expanded_original_product_degree_upper_bound: max_original_bound,
        basis_polynomial_degree: certificate
            .basis
            .iter()
            .map(|r| degree(&r.poly))
            .max()
            .unwrap(),
        coordinate_parents: parents,
        derivation_depths: depths,
        projection_work,
    };
    Ok(Output { certificate, info })
}
