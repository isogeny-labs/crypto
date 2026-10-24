//! Exact Fp root extraction and physical filtering for sealed joint systems.
use super::*;
use std::time::Instant;

fn system(path: &Path, expected: &str) -> (Value, Vec<f4_fp::Poly>) {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(
        hex::encode(hash::sha256::sha256(&bytes)),
        expected,
        "joint system hash changed"
    );
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["schema"], "joint-image-system/v1");
    let p = v["p"].as_u64().unwrap();
    assert!(PRIMES.contains(&p));
    let variables = v["variables"].as_u64().unwrap() as usize;
    assert!([4, 5].contains(&variables));
    let raw: Vec<Vec<(Vec<u32>, u64)>> = serde_json::from_value(v["equations"].clone()).unwrap();
    assert!(raw.iter().all(|p| p
        .iter()
        .all(|(e, c)| e.len() == variables && *c < p_u64(&v))));
    let input = raw
        .iter()
        .map(|p0| f4_fp::normalise(p0, p, f4_fp::Ordering::Grevlex))
        .collect();
    (v, input)
}
fn p_u64(v: &Value) -> u64 {
    v["p"].as_u64().unwrap()
}

pub(super) fn solve(
    public: &Path,
    public_sha: &str,
    system_path: &Path,
    system_sha: &str,
    out: &Path,
) -> bool {
    solve_until(
        public,
        public_sha,
        system_path,
        system_sha,
        out,
        Instant::now() + Duration::from_secs(1180),
    )
}

fn solve_until(
    public: &Path,
    public_sha: &str,
    system_path: &Path,
    system_sha: &str,
    out: &Path,
    deadline: Instant,
) -> bool {
    let start = Instant::now();
    let raw = checked_public(public, public_sha);
    let (spec, input) = system(system_path, system_sha);
    assert_eq!(spec["p"], raw["p"]);
    assert_eq!(spec["seed"], raw["seed"]);
    let arm_name = spec["arm"].as_str().unwrap();
    let arm = arm_index(arm_name);
    let qi = spec["query_index"].as_u64().unwrap() as usize;
    assert!(qi < raw["query_trace"].as_array().unwrap().len());
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let f = case.c.f;
    let residual = case.residual(&case.raw["query_trace"][qi]);
    write(
        &out.join("started.json"),
        &json!({"schema":"joint-image-solve-start/v1","public_input_sha256":public_sha,"system_sha256":system_sha,"p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"degree_cap":64,"outer_cap_seconds":1200,"state":"started"}),
    );
    // Leave time for preserving a cooperative failure and physical filtering.
    let remaining = deadline.saturating_duration_since(Instant::now());
    let options = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 64).with_budget(remaining);
    let solved = f4_fp::solve(
        &input,
        spec["variables"].as_u64().unwrap() as usize,
        case.p,
        &options,
    );
    let complete = !matches!(solved.verdict, f4_fp::Verdict::Undetermined);
    let mut roots = match &solved.verdict {
        f4_fp::Verdict::Solutions(v) => v.clone(),
        _ => Vec::new(),
    };
    roots.sort();
    roots.dedup();
    // Preserve the field result before potentially lengthy group filtering.
    write(
        &out.join("field-roots.json"),
        &json!({"roots_complete":complete,"roots":roots,"timed_out":solved.timed_out,"f4_reported_ms":solved.ms,"solving_degree":solved.solving_degree,"degree_reached":solved.degree_reached,"max_rows":solved.max_rows,"max_cols":solved.max_cols,"f4_runs":solved.f4_runs,"partial_row_reduction_field_ops":solved.field_ops}),
    );
    let by_x: BTreeMap<_, _> = case.pool[..case.l0]
        .iter()
        .enumerate()
        .map(|(i, p)| (p.p.x, i))
        .collect();
    let by_source: BTreeMap<_, _> = case.peels[arm]
        .iter()
        .map(|&i| (case.pool[i].source_id, i))
        .collect();
    let mut got = BTreeSet::<Terms>::new();
    let mut rejected = BTreeMap::<String, usize>::new();
    let mut group_assignments = 0;
    for root in &roots {
        assert!(root.iter().all(|&v| v < case.p));
        assert!(
            input.iter().all(|p| f4_fp::eval(p, root, case.p) == 0),
            "false Fp root"
        );
        let Some(&source) = by_x.get(&f.scalar(root[3])) else {
            *rejected.entry("source_outside_base".into()).or_default() += 1;
            continue;
        };
        let image_id = by_source[&source];
        let xs: Vec<_> = (0..case.p)
            .filter(|&x| {
                (((x + case.p - root[0]) % case.p * x + root[1]) % case.p * x + case.p - root[2])
                    % case.p
                    == 0
            })
            .collect();
        if xs.len() != 3 {
            *rejected
                .entry("cubic_not_three_distinct_Fp_roots".into())
                .or_default() += 1;
            continue;
        }
        let ordinary: Option<Vec<_>> = xs
            .iter()
            .map(|&x| by_x.get(&f.scalar(x)).copied())
            .collect();
        let Some(mut ids) = ordinary else {
            *rejected
                .entry("ordinary_factor_outside_base".into())
                .or_default() += 1;
            continue;
        };
        if ids.contains(&image_id) {
            *rejected.entry("repeated_fourth_factor".into()).or_default() += 1;
            continue;
        }
        ids.push(image_id);
        for mask in 0..16 {
            group_assignments += 1;
            let mut sum = INF;
            let mut terms = Vec::new();
            for (i, &id) in ids.iter().enumerate() {
                let sign = if mask & (1 << i) == 0 { 1 } else { -1 };
                sum = case
                    .c
                    .add(sum, signed(&case.c, case.pool[id].p.into(), sign));
                terms.push((id, sign));
            }
            if sum == residual {
                terms.sort();
                case.verify_terms(&terms, residual);
                got.insert(terms);
            }
        }
    }
    let mut rows = BTreeSet::new();
    let mut matrix = Matrix::new(case.n, case.columns(arm));
    for terms in &got {
        let row = case.row(terms, &case.raw["query_trace"][qi], arm);
        if let Some(&first) = row.iter().find(|&&x| x != 0) {
            let inverse = fpow(first, case.n - 2, case.n);
            rows.insert(
                row.iter()
                    .map(|&x| mm(x, inverse, case.n))
                    .collect::<Vec<_>>(),
            );
        }
        if matrix.rank() < matrix.cols {
            matrix.insert(row);
        }
    }
    let solution = matrix.solution();
    if let Some(logs) = &solution {
        assert!(case.verify_solution(logs, arm));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-query-solve/v1","status":if complete {"complete"}else{"undetermined"},"p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"orbit_value":spec["orbit_value"],"public_input_sha256":public_sha,"system_sha256":system_sha,"roots_complete":complete,"field_roots":roots.len(),"rejections":rejected,"group_sign_assignments":group_assignments,"signed_relations":got,"unique_projective_augmented_rows":rows.len(),"rank":matrix.rank(),"columns":matrix.cols,"recovered":solution.as_ref().map(|s|s[matrix.cols-1]),"all_target_and_factor_logs_verified":solution.as_ref().map(|_|true),"f4_reported_ms":solved.ms,"diagnostic_process_ms":start.elapsed().as_secs_f64()*1000.0,"cost_scope":"Unadmitted diagnostic process duration; supplied point encodings, no complete lifting/cold/rho price; not an end-to-end speedup"}),
    );
    println!(
        "{}",
        json!({"p":case.p,"seed":case.seed,"arm":arm_name,"query":qi,"roots_complete":complete,"signed_relations":got.len(),"rank":matrix.rank()})
    );
    complete
}

pub(super) fn solve_orbits(
    public: &Path,
    public_sha: &str,
    manifest_path: &Path,
    manifest_sha: &str,
    out: &Path,
) -> bool {
    let start = Instant::now();
    let deadline = start + Duration::from_secs(1180);
    let raw = checked_public(public, public_sha);
    let bytes = std::fs::read(manifest_path).unwrap();
    assert_eq!(hex::encode(hash::sha256::sha256(&bytes)), manifest_sha);
    let manifest: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(manifest["schema"], "joint-image-orbit-inputs/v1");
    assert_eq!(manifest["public_input_sha256"], public_sha);
    assert_eq!(manifest["p"], raw["p"]);
    assert_eq!(manifest["seed"], raw["seed"]);
    let arm_name = manifest["arm"].as_str().unwrap();
    let arm = arm_index(arm_name);
    let qi = manifest["query_index"].as_u64().unwrap() as usize;
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut complete = true;
    let mut blocks = Vec::new();
    let mut got = BTreeSet::<Terms>::new();
    for block in manifest["blocks"].as_array().unwrap() {
        let id = block["id"].as_str().unwrap();
        assert!(
            id.starts_with("orbit-") && id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
        );
        if deadline.saturating_duration_since(Instant::now()) < Duration::from_secs(5) {
            complete = false;
            blocks.push(json!({"id":id,"status":"not-run-query-budget-exhausted"}));
            continue;
        }
        let relative = Path::new(block["system_path"].as_str().unwrap());
        assert!(relative
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_))));
        let system = manifest_path.parent().unwrap().join(relative);
        let dest = out.join(id);
        let pass = solve_until(
            public,
            public_sha,
            &system,
            block["system_sha256"].as_str().unwrap(),
            &dest,
            deadline,
        );
        complete &= pass;
        let s: Value =
            serde_json::from_slice(&std::fs::read(dest.join("summary.json")).unwrap()).unwrap();
        let terms: BTreeSet<Terms> = serde_json::from_value(s["signed_relations"].clone()).unwrap();
        got.extend(terms);
        blocks.push(json!({"id":id,"status":s["status"],"signed_relations":s["signed_relations"].as_array().unwrap().len()}));
        write(
            &out.join("progress.json"),
            &json!({"blocks":blocks,"completed_signed_union":got,"roots_complete_so_far":complete}),
        );
    }
    let case = Case::new(raw, out);
    let residual = case.residual(&case.raw["query_trace"][qi]);
    let mut rows = BTreeSet::new();
    let mut matrix = Matrix::new(case.n, case.columns(arm));
    for terms in &got {
        case.verify_terms(terms, residual);
        let row = case.row(terms, &case.raw["query_trace"][qi], arm);
        if let Some(r) = canonical_row(row.clone(), case.n) {
            rows.insert(r);
        }
        if matrix.rank() < matrix.cols {
            matrix.insert(row);
        }
    }
    let solution = matrix.solution();
    if let Some(s) = &solution {
        assert!(case.verify_solution(s, arm));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-query-solve/v1","status":if complete{"complete"}else{"undetermined"},"p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"public_input_sha256":public_sha,"manifest_sha256":manifest_sha,"roots_complete":complete,"blocks":blocks,"signed_relations":got,"unique_projective_augmented_rows":rows.len(),"rank":matrix.rank(),"columns":matrix.cols,"recovered":solution.as_ref().map(|s|s[matrix.cols-1]),"all_target_and_factor_logs_verified":solution.as_ref().map(|_|true),"diagnostic_process_ms":start.elapsed().as_secs_f64()*1000.0,"cost_scope":"Unadmitted query component; repeated supplied-geometry setup included, lifting and full-DLP/rho price missing"}),
    );
    complete
}

pub(super) fn check(
    public: &Path,
    public_sha: &str,
    candidate: &Path,
    reference: &Path,
    out: &Path,
) -> bool {
    let raw = checked_public(public, public_sha);
    let s: Value =
        serde_json::from_slice(&std::fs::read(candidate.join("summary.json")).unwrap()).unwrap();
    let bytes = std::fs::read(reference).unwrap();
    let r: Value = serde_json::from_slice(&bytes).unwrap();
    for key in ["p", "seed", "arm"] {
        assert_eq!(s[key], r[key]);
    }
    assert_eq!(s["p"], raw["p"]);
    assert_eq!(s["seed"], raw["seed"]);
    let qi = s["query_index"].as_u64().unwrap() as usize;
    let actual: BTreeSet<Terms> = serde_json::from_value(s["signed_relations"].clone()).unwrap();
    let mut expected: BTreeSet<Terms> =
        serde_json::from_value(r["trace"][qi]["signed_relations"].clone()).unwrap();
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let arm = arm_index(s["arm"].as_str().unwrap());
    if let Some(t) = s["orbit_value"].as_u64() {
        expected.retain(|terms| {
            terms.iter().any(|&(id, _)| {
                if !case.peels[arm].contains(&id) {
                    return false;
                }
                let source = case.pool[id].source_id;
                fpow(case.pool[source].p.x[0], 12, case.p) == t
            })
        });
    }
    let residual = case.residual(&case.raw["query_trace"][qi]);
    for terms in &actual {
        case.verify_terms(terms, residual);
    }
    let rows: Vec<_> = actual
        .iter()
        .map(|terms| case.row(terms, &case.raw["query_trace"][qi], arm))
        .collect();
    let (rank, solution) = independent_rref(&rows, case.columns(arm), case.n);
    if let Some(solution) = solution {
        assert!(case.verify_solution(&solution, arm));
    }
    assert_eq!(s["rank"], rank);
    let pass = s["roots_complete"] == true && actual == expected;
    write(
        &out.join("audit.json"),
        &json!({"schema":"joint-image-query-audit/v1","status":if pass{"pass"}else{"partial"},"p":case.p,"seed":case.seed,"arm":s["arm"],"query_index":qi,"orbit_value":s["orbit_value"],"candidate_roots_complete":s["roots_complete"],"reference_sha256":hex::encode(hash::sha256::sha256(&bytes)),"expected_signed_relations":expected.len(),"actual_signed_relations":actual.len(),"missing":expected.difference(&actual).collect::<Vec<_>>(),"extra":actual.difference(&expected).collect::<Vec<_>>(),"rank":rank,"columns":case.columns(arm),"scope":"Exact physical signed-set comparison against the earlier independently audited group/BSGS reference and separate RREF; not a complete-GB or performance certificate"}),
    );
    pass
}
