//! Independent syntax, Buchberger and physical-set checks for raw msolve output.
//! Input containment plus S-pairs does NOT prove reverse ideal containment.
use super::*;
use std::time::Instant;

fn digest(path: &Path, expected: &str) -> Vec<u8> {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(hex::encode(hash::sha256::sha256(&bytes)), expected);
    bytes
}

fn source_block_contains(case: &Case, arm: usize, terms: &Terms, t: u64) -> bool {
    terms.iter().any(|&(id, _)| {
        if !case.peels[arm].contains(&id) {
            return false;
        }
        let source = case.pool[id].source_id;
        fpow(case.pool[source].p.x[0], 12, case.p) == t
    })
}

fn parse(text: &str, p: u64, names: &[&str]) -> Vec<f4_fp::Poly> {
    assert!(text
        .lines()
        .any(|l| l.trim() == format!("#field characteristic: {p}")));
    let body: String = text
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("");
    let body = body.trim();
    assert!(
        body.starts_with('[') && body.ends_with("]:"),
        "incomplete raw basis"
    );
    body[1..body.len() - 2]
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let chars: Vec<_> = s.chars().filter(|c| !c.is_whitespace()).collect();
            let mut i = 0;
            let mut terms = Vec::new();
            while i < chars.len() {
                let negative = chars[i] == '-';
                if chars[i] == '-' || chars[i] == '+' {
                    i += 1;
                }
                let mut coefficient = 1u64;
                let mut exponent = vec![0u32; names.len()];
                let mut factor = false;
                while i < chars.len() && chars[i] != '-' && chars[i] != '+' {
                    if chars[i] == '*' {
                        assert!(factor);
                        i += 1;
                        continue;
                    }
                    if chars[i].is_ascii_digit() {
                        let mut n = 0u64;
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            n = n
                                .checked_mul(10)
                                .unwrap()
                                .checked_add(chars[i] as u64 - '0' as u64)
                                .unwrap();
                            i += 1;
                        }
                        coefficient = fm(coefficient, n % p, p);
                    } else {
                        let start = i;
                        while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                            i += 1;
                        }
                        assert!(i > start, "unsupported polynomial token");
                        let name: String = chars[start..i].iter().collect();
                        let variable = names
                            .iter()
                            .position(|&v| v == name)
                            .expect("unknown basis variable");
                        let mut power = 1u32;
                        if i < chars.len() && chars[i] == '^' {
                            i += 1;
                            let start = i;
                            power = 0;
                            while i < chars.len() && chars[i].is_ascii_digit() {
                                power = power
                                    .checked_mul(10)
                                    .unwrap()
                                    .checked_add(chars[i] as u32 - '0' as u32)
                                    .unwrap();
                                i += 1;
                            }
                            assert!(i > start, "missing exponent");
                        }
                        exponent[variable] = exponent[variable].checked_add(power).unwrap();
                    }
                    factor = true;
                }
                assert!(factor);
                terms.push((
                    exponent,
                    if negative && coefficient != 0 {
                        p - coefficient
                    } else {
                        coefficient
                    },
                ));
            }
            f4_fp::normalise(&terms, p, f4_fp::Ordering::Grevlex)
        })
        .filter(|g| !g.is_empty())
        .collect()
}

fn consistency(input: &[f4_fp::Poly], basis: &[f4_fp::Poly], p: u64) -> bool {
    let order = f4_fp::Ordering::Grevlex;
    if basis.is_empty() {
        return input.iter().all(|g| g.is_empty());
    }
    if input
        .iter()
        .any(|g| !f4_fp::reduce(g, basis, p, order).is_empty())
    {
        return false;
    }
    for i in 0..basis.len() {
        for j in 0..i {
            let a = &basis[i];
            let b = &basis[j];
            let lcm: Vec<_> = a[0]
                .0
                .iter()
                .zip(&b[0].0)
                .map(|(&x, &y)| x.max(y))
                .collect();
            let mut terms = Vec::new();
            for (g, sign) in [(a, 1u64), (b, p - 1)] {
                let shift: Vec<_> = lcm.iter().zip(&g[0].0).map(|(&x, &y)| x - y).collect();
                let scale = fm(sign, fpow(g[0].1, p - 2, p), p);
                for (e, c) in g {
                    terms.push((
                        e.iter().zip(&shift).map(|(&x, &y)| x + y).collect(),
                        fm(*c, scale, p),
                    ));
                }
            }
            let s = f4_fp::normalise(&terms, p, order);
            if !f4_fp::reduce(&s, basis, p, order).is_empty() {
                return false;
            }
        }
    }
    true
}

fn specialise(basis: &[f4_fp::Poly], values: &[Option<u64>], p: u64) -> Vec<f4_fp::Poly> {
    basis
        .iter()
        .map(|g| {
            let terms: Vec<_> = g
                .iter()
                .map(|(e, c)| {
                    let mut exponent = e.clone();
                    let mut coefficient = *c;
                    for (i, &value) in values.iter().enumerate() {
                        if let Some(value) = value {
                            coefficient = fm(coefficient, fpow(value, exponent[i] as u64, p), p);
                            exponent[i] = 0;
                        }
                    }
                    (exponent, coefficient)
                })
                .collect();
            f4_fp::normalise(&terms, p, f4_fp::Ordering::Grevlex)
        })
        .filter(|g| !g.is_empty())
        .collect()
}

// Enumerate finite-field branches directly, without calling a GB/root engine.
// A univariate constraint restricts a branch only after testing every Fp value.
fn enumerate(
    basis: &[f4_fp::Poly],
    values: &mut [Option<u64>],
    p: u64,
    deadline: Instant,
    roots: &mut Vec<Vec<u64>>,
    nodes: &mut u64,
) -> bool {
    if Instant::now() >= deadline {
        return false;
    }
    *nodes += 1;
    let reduced = specialise(basis, values, p);
    let mut univariate = None;
    for g in &reduced {
        let active: BTreeSet<_> = g
            .iter()
            .flat_map(|(e, _)| {
                e.iter()
                    .enumerate()
                    .filter_map(|(i, &k)| if k > 0 { Some(i) } else { None })
            })
            .collect();
        if active.is_empty() {
            return true;
        } // nonzero constant: impossible branch
        if active.len() == 1 {
            let variable = *active.iter().next().unwrap();
            let degree = g.iter().map(|(e, _)| e[variable]).max().unwrap();
            if univariate.as_ref().is_none_or(|&(_, _, d)| degree < d) {
                univariate = Some((variable, g, degree));
            }
        }
    }
    let Some(default) = values.iter().rposition(|v| v.is_none()) else {
        assert!(reduced.is_empty());
        roots.push(values.iter().map(|v| v.unwrap()).collect());
        return true;
    };
    let (variable, candidates) = if let Some((variable, g, _)) = univariate {
        assert!(values[variable].is_none());
        (
            variable,
            (0..p)
                .filter(|&x| {
                    g.iter().fold(0u64, |sum, (e, c)| {
                        (sum + fm(*c, fpow(x, e[variable] as u64, p), p)) % p
                    }) == 0
                })
                .collect::<Vec<_>>(),
        )
    } else {
        (default, (0..p).collect())
    };
    for x in candidates {
        values[variable] = Some(x);
        let complete = enumerate(&reduced, values, p, deadline, roots, nodes);
        values[variable] = None;
        if !complete {
            return false;
        }
    }
    true
}

fn physical(
    case: &Case,
    spec: &Value,
    input: &[f4_fp::Poly],
    roots: &[Vec<u64>],
) -> BTreeSet<Terms> {
    let arm = arm_index(spec["arm"].as_str().unwrap());
    let qi = spec["query_index"].as_u64().unwrap() as usize;
    let residual = case.residual(&case.raw["query_trace"][qi]);
    let f = case.c.f;
    let by_x: BTreeMap<_, _> = case.pool[..case.l0]
        .iter()
        .enumerate()
        .map(|(i, p)| (p.p.x, i))
        .collect();
    let by_source: BTreeMap<_, _> = case.peels[arm]
        .iter()
        .map(|&i| (case.pool[i].source_id, i))
        .collect();
    let mut relations = BTreeSet::new();
    for root in roots {
        assert!(root.iter().all(|&x| x < case.p));
        assert!(input.iter().all(|g| f4_fp::eval(g, root, case.p) == 0));
        let degree = if spec["root_encoding"] == "elementary-four" {
            4
        } else {
            3
        };
        let xs: Vec<_> = (0..case.p)
            .filter(|&x| {
                let mut y = 1u64;
                for (i, &coefficient) in root.iter().take(degree).enumerate() {
                    y = (fm(y, x, case.p)
                        + if i % 2 == 0 {
                            (case.p - coefficient) % case.p
                        } else {
                            coefficient
                        })
                        % case.p;
                }
                y == 0
            })
            .collect();
        if xs.len() != degree {
            continue;
        }
        let Some(mut ids) = xs
            .iter()
            .map(|&x| by_x.get(&f.scalar(x)).copied())
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        if degree == 3 {
            let Some(&source) = by_x.get(&f.scalar(root[3])) else {
                continue;
            };
            let Some(&fourth) = by_source.get(&source) else {
                continue;
            };
            if ids.contains(&fourth) {
                continue;
            }
            ids.push(fourth);
        } else {
            assert_eq!(arm, 8);
        }
        ids.sort();
        assert_eq!(ids.iter().collect::<BTreeSet<_>>().len(), 4);
        for mask in 0..16 {
            let terms: Terms = ids
                .iter()
                .enumerate()
                .map(|(i, &id)| (id, if mask & (1 << i) == 0 { 1 } else { -1 }))
                .collect();
            let sum = terms.iter().fold(INF, |s, &(id, sign)| {
                case.c.add(s, signed(&case.c, case.pool[id].p.into(), sign))
            });
            if sum == residual {
                case.verify_terms(&terms, residual);
                relations.insert(terms);
            }
        }
    }
    relations
}

// A verifier mode: the independent oracle is allowed here, not in a candidate.
pub(super) fn audit(
    public: &Path,
    public_sha: &str,
    system: &Path,
    system_sha: &str,
    raw_basis: &Path,
    basis_sha: &str,
    reference: &Path,
    out: &Path,
) -> bool {
    assert!(!out.exists());
    let public = checked_public(public, public_sha);
    let spec: Value = serde_json::from_slice(&digest(system, system_sha)).unwrap();
    assert_eq!(spec["schema"], "joint-image-system/v1");
    assert_eq!(spec["p"], public["p"]);
    assert_eq!(spec["seed"], public["seed"]);
    let p = spec["p"].as_u64().unwrap();
    let n = spec["variables"].as_u64().unwrap() as usize;
    assert!([4, 5, 6].contains(&n));
    let input: Vec<f4_fp::Poly> = serde_json::from_value(spec["equations"].clone()).unwrap();
    assert!(input.iter().flatten().all(|(e, c)| e.len() == n && *c < p));
    let input: Vec<_> = input
        .iter()
        .map(|g| f4_fp::normalise(g, p, f4_fp::Ordering::Grevlex))
        .collect();
    let bytes = digest(raw_basis, basis_sha);
    let basis = parse(
        std::str::from_utf8(&bytes).unwrap(),
        p,
        &["e1", "e2", "e3", "u", "v", "s"][..n],
    );
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("parsed-basis.json"),
        &json!({"basis":basis,"raw_sha256":basis_sha}),
    );
    assert!(
        consistency(&input, &basis, p),
        "input containment or Buchberger check failed"
    );
    let mut roots = Vec::new();
    let mut nodes = 0u64;
    let complete_basis_variety = enumerate(
        &basis,
        &mut vec![None; n],
        p,
        Instant::now() + Duration::from_secs(1180),
        &mut roots,
        &mut nodes,
    );
    roots.sort();
    roots.dedup();
    assert!(roots
        .iter()
        .all(|r| basis.iter().all(|g| f4_fp::eval(g, r, p) == 0)));
    let case = Case::new(public, out);
    let relations = physical(&case, &spec, &input, &roots);
    let arm = arm_index(spec["arm"].as_str().unwrap());
    let qi = spec["query_index"].as_u64().unwrap() as usize;
    let cols = case.columns(arm);
    let rows: Vec<_> = relations
        .iter()
        .map(|t| case.row(t, &case.raw["query_trace"][qi], arm))
        .collect();
    let unique: BTreeSet<_> = rows
        .iter()
        .filter_map(|r| canonical_row(r.clone(), case.n))
        .collect();
    let (rank, solution) = independent_rref(&rows, cols, case.n);
    if let Some(s) = &solution {
        assert!(case.verify_solution(s, arm));
    }
    let ref_bytes = std::fs::read(reference).unwrap();
    let oracle: Value = serde_json::from_slice(&ref_bytes).unwrap();
    assert_eq!(oracle["p"], case.p);
    assert_eq!(oracle["seed"], case.seed);
    assert_eq!(oracle["arm"], spec["arm"]);
    let query = &oracle["trace"][qi];
    assert_eq!(query["query_index"], qi);
    assert_eq!(query["a"], case.raw["query_trace"][qi][0]);
    assert_eq!(query["b"], case.raw["query_trace"][qi][1]);
    let mut expected: BTreeSet<Terms> =
        serde_json::from_value(query["signed_relations"].clone()).unwrap();
    if let Some(t) = spec["orbit_value"].as_u64() {
        expected.retain(|terms| source_block_contains(&case, arm, terms, t));
    }
    let all_physical = complete_basis_variety && relations == expected;
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-external-audit/v1",
        "p":case.p,"seed":case.seed,"arm":spec["arm"],"query_index":qi,"orbit_value":spec["orbit_value"],
        "public_input_sha256":public_sha,"system_sha256":system_sha,"basis_sha256":basis_sha,
        "reference_sha256":hex::encode(hash::sha256::sha256(&ref_bytes)),
        "input_reductions_and_s_pairs_zero":true,"reverse_ideal_containment_verified":false,
        "basis_variety_complete_exhaustive_enumeration":complete_basis_variety,
        "root_method":"Independent finite-field branching with exhaustive univariate scans; no Groebner/root engine call",
        "root_enumeration_nodes":nodes,
        "original_system_field_root_completeness_verified":false,
        "basis_polynomials":basis.len(),"field_roots":roots,"signed_relations":relations,
        "expected_physical_relations":expected.len(),"unique_projective_augmented_rows":unique.len(),
        "rank":rank,"columns":cols,"all_physical_sets_equal":relations==expected,
        "status":if all_physical {"physical-set-pass"} else {"partial"},
        "scope":"Separate verifier. Complete physical sets are established only by equality with the independent group oracle on this fixture. Buchberger and input containment alone do not certify equality of ideals. No timing or DLP cost claim."}),
    );
    all_physical
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_physical_relations_use_the_correct_source_block_scope() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let scratch =
            std::env::temp_dir().join(format!("joint-external-scope-check-{}", std::process::id()));
        assert!(!scratch.exists());
        std::fs::create_dir(&scratch).unwrap();
        let case = Case::new(public_record(&load(&tape, INPUT_SHA, 37, 0)), &scratch);
        let residual = case.residual(&case.raw["query_trace"][2]);
        // Frozen positive fixture, independently audited in regression-v2.
        let pure = vec![(5, -1), (21, -1), (23, 1), (24, 1)];
        let phi = vec![(7, -1), (13, 1), (29, 1), (71, 1)];
        case.verify_terms(&pure, residual);
        case.verify_terms(&phi, residual);
        // One pure physical tuple has presentations in all three blocks.
        for t in [1, 10, 26] {
            assert!(source_block_contains(&case, 8, &pure, t));
        }
        assert!(!source_block_contains(&case, 8, &pure, 2));
        // The image's source, rather than an arbitrary ordinary factor,
        // fixes the mixed block. The saved independent audit puts it in T=10.
        assert!(source_block_contains(&case, 7, &phi, 10));
        assert!(!source_block_contains(&case, 7, &phi, 1));
        assert!(!source_block_contains(&case, 7, &phi, 26));
        std::fs::remove_dir_all(scratch).unwrap();
    }
    #[test]
    fn multicharacter_variable_parser_handles_all_six_exported_names() {
        let basis = parse(
            "#field characteristic: 37\n[e1^2-e2,e3*u+36*v,s-2]:",
            37,
            &["e1", "e2", "e3", "u", "v", "s"],
        );
        assert_eq!(basis.len(), 3);
        assert_eq!(f4_fp::eval(&basis[0], &[3, 9, 2, 4, 8, 2], 37), 0);
        assert_eq!(f4_fp::eval(&basis[1], &[3, 9, 2, 4, 8, 2], 37), 0);
        assert_eq!(f4_fp::eval(&basis[2], &[3, 9, 2, 4, 8, 2], 37), 0);
    }
    #[test]
    fn buchberger_and_input_reduction_do_not_prove_reverse_containment() {
        let g = parse("#field characteristic: 37\n[e1^2-e1]:", 37, &["e1"]);
        let valid = parse("#field characteristic: 37\n[e1^2-e1]:", 37, &["e1"]);
        let strict_superideal = parse("#field characteristic: 37\n[1]:", 37, &["e1"]);
        assert!(consistency(&g, &valid, 37));
        assert!(consistency(&g, &strict_superideal, 37));
        assert_ne!(
            f4_fp::eval(&g[0], &[0], 37),
            f4_fp::eval(&strict_superideal[0], &[0], 37)
        );
        let wrong = parse("#field characteristic: 37\n[e1-2]:", 37, &["e1"]);
        assert!(!consistency(&g, &wrong, 37));
    }
    #[test]
    #[should_panic(expected = "incomplete raw basis")]
    fn truncated_basis_is_rejected() {
        parse("#field characteristic: 37\n[e1-2", 37, &["e1"]);
    }
    #[test]
    fn independent_branch_enumeration_keeps_both_roots_and_free_variables() {
        let g = parse(
            "#field characteristic: 37\n[e1+e2,e2^2-1,u]:",
            37,
            &["e1", "e2", "u"],
        );
        let mut roots = Vec::new();
        let mut nodes = 0;
        assert!(enumerate(
            &g,
            &mut vec![None; 3],
            37,
            Instant::now() + Duration::from_secs(10),
            &mut roots,
            &mut nodes
        ));
        roots.sort();
        assert_eq!(roots, vec![vec![1, 36, 0], vec![36, 1, 0]]);
        let g = parse("#field characteristic: 37\n[e1-2]:", 37, &["e1", "e2"]);
        let mut roots = Vec::new();
        let mut nodes = 0;
        assert!(enumerate(
            &g,
            &mut vec![None; 2],
            37,
            Instant::now() + Duration::from_secs(10),
            &mut roots,
            &mut nodes
        ));
        assert_eq!(roots.len(), 37);
        assert!(roots.iter().all(|r| r[0] == 2));
    }
}
