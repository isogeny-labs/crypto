//! Acyclic local identities: checked shared consequences, without expanding
//! every later certificate back into the original input generators.
//! Polynomial construction kernels are shared with the expanded control.
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
fn mono_degree(m: &[usize]) -> Option<usize> {
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
    verify_impl(certificate, inputs, n, p, false)
}
pub fn verify_product(certificate: &Certificate, inputs: &[Poly], n: usize, p: u64) -> bool {
    verify_impl(certificate, inputs, n, p, true)
}
fn verify_impl(
    certificate: &Certificate,
    inputs: &[Poly],
    n: usize,
    p: u64,
    product: bool,
) -> bool {
    if !admitted(inputs, n, p)
        || certificate.definitions.len() > n
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
        if self.definitions.len() >= self.n
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
            proof: BTreeMap::from([((index, vec![0; self.n]), 1)]),
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
    solve_impl(inputs, n, p, false, false, false)
}
pub fn solve_product(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    solve_impl(inputs, n, p, false, false, true)
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
    match solve_impl(inputs, n, p, false, true, product) {
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
    solve_impl(inputs, n, p, true, false, false)
}
fn solve_impl(
    inputs: &[Poly],
    n: usize,
    p: u64,
    all_vertices: bool,
    subset: bool,
    product: bool,
) -> Result<Output, String> {
    if !admitted(inputs, n, p) {
        return Err("compact: unsupported prime/input shape".into());
    }
    let (root_row, root, projection_work, pairs) = if subset {
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
    let mut chain = Chain {
        generators: inputs.to_vec(),
        definitions: Vec::new(),
        stored_terms: 0,
        identity_work: 0,
        n,
        p,
    };
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
    if !(if product {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sufficient_subset_keeps_full_checked_star_ideals() {
        for hub in [0, 7] {
            let edges: Vec<_> = (0..8).filter(|v| *v != hub).map(|v| (hub, v)).collect();
            let system = workload::generate_graph(2147483647, 8, 7, &edges)
                .unwrap()
                .system;
            let input = system.decode().unwrap();
            let old = solve(&input, 8, system.prime).unwrap();
            let new = solve_sparse(&input, 8, system.prime).unwrap();
            assert!(new.subset_succeeded);
            assert_eq!(
                new.output
                    .certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>(),
                old.certificate
                    .basis
                    .iter()
                    .map(|r| &r.poly)
                    .collect::<Vec<_>>()
            );
            assert_eq!(old.info.projection_work.edge_resultants, 7);
            assert_eq!(new.output.info.projection_work.edge_resultants, 2);
            assert!(verify(&new.output.certificate, &input, 8, system.prime));
            let metadata = cheap::pair_metadata(&input, 8, system.prime);
            let cubic = cheap::consequences(&input, 8, system.prime);
            assert_eq!(
                metadata,
                cubic
                    .iter()
                    .map(|pair| cheap::PairMetadata {
                        inputs: pair.inputs,
                        variables: pair.variables
                    })
                    .collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn research_capacity_is_explicit_and_bounded() {
        for n in [3, 8, 9, 16, 17] {
            let input = vec![BTreeMap::from([(vec![0; n], 1)])];
            assert_eq!(admitted(&input, n, 2147483647), n <= MAX_VARIABLES);
        }
        assert_eq!(
            MAX_VARIABLES,
            if cfg!(feature = "scale-screen") {
                16
            } else {
                8
            }
        );
    }
    #[test]
    fn compact_chain_keeps_existing_fixed_case_bases_and_smaller_six_cycle_proofs() {
        for p in [2147483647, 9007199254740881] {
            for n in [3, 4, 6] {
                let inputs = workload::generate(p, n, 7)
                    .unwrap()
                    .system
                    .decode()
                    .unwrap();
                let compact = solve(&inputs, n, p).unwrap();
                let all = solve_all_vertices(&inputs, n, p).unwrap();
                assert_eq!(compact.certificate.definitions, all.certificate.definitions);
                assert_eq!(compact.certificate.basis, all.certificate.basis);
                assert_eq!(compact.info.root_variable, n - 1);
                assert_eq!(compact.info.projection_work.edge_resultants, 2);
                assert_eq!(compact.info.projection_work.vertex_gcd_folds, 1);
                assert_eq!(all.info.projection_work.edge_resultants, n);
                assert_eq!(all.info.projection_work.vertex_gcd_folds, n);
                assert!(verify(&all.certificate, &inputs, n, p));
                assert!(verify(&compact.certificate, &inputs, n, p));
                let (expanded, _) = super::super::solve(&inputs, n, p).unwrap();
                assert_eq!(
                    compact
                        .certificate
                        .basis
                        .iter()
                        .map(|r| &r.poly)
                        .collect::<Vec<_>>(),
                    expanded.iter().map(|r| &r.poly).collect::<Vec<_>>()
                );
                if n == 6 {
                    let expanded_terms: usize = expanded.iter().map(|r| r.proof.len()).sum();
                    assert!(compact.info.stored_proof_terms < expanded_terms);
                    eprintln!("compact fixed case p={p}: total_local_terms={}, expanded_final_basis_terms={expanded_terms}, local_product_degree={}, expanded_original_product_degree_upper_bound={}", compact.info.stored_proof_terms,compact.info.max_local_certificate_product_degree,compact.info.expanded_original_product_degree_upper_bound);
                }
            }
        }
    }
    #[test]
    fn rejects_circular_forged_missing_and_malformed_chain_data() {
        let p = 2147483647;
        let inputs = workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap();
        let output = solve(&inputs, 3, p).unwrap();
        let mut bad = output.certificate.clone();
        bad.definitions[0].proof = BTreeMap::from([((inputs.len(), vec![0; 3]), 1)]);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.definitions[0].proof.clear();
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.definitions.remove(0);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        bad.basis[0].proof = BTreeMap::from([((0, vec![usize::MAX, 1, 0]), 1)]);
        assert!(!verify(&bad, &inputs, 3, p));
        bad = output.certificate.clone();
        *bad.definitions[0].poly.values_mut().next().unwrap() = p;
        assert!(!verify(&bad, &inputs, 3, p));
        assert!(!verify(&output.certificate, &inputs, 3, 1u64 << 53));
    }
    #[test]
    fn wrong_final_ideal_and_shape_are_not_accepted() {
        let p = 2147483647;
        let inputs = workload::generate(p, 3, 7)
            .unwrap()
            .system
            .decode()
            .unwrap();
        let output = solve(&inputs, 3, p).unwrap();
        let mut bad = output.certificate;
        bad.basis.pop();
        assert!(!verify(&bad, &inputs, 3, p));
        assert!(solve(&inputs, 2, p).is_err());
        assert!(solve(&inputs, 3, 1u64 << 53).is_err());
    }
}
