//! Public-input one-target pipeline, with an optional stopped zero precomputation.
//! All elapsed values here are shared-host diagnostics, not admitted prices.
use super::*;
#[path = "baseline.rs"]
mod baseline;
#[path = "pipeline_fixtures.rs"]
mod fixtures;
#[path = "pipeline_audit.rs"]
mod replay;

pub(crate) fn select_baseline(csv: &Path, sha: &str, out: &Path) {
    baseline::select_baseline(csv, sha, out)
}
pub(crate) fn test_fixtures(zero: &Path, nine: &Path, double: &Path, out: &Path) {
    fixtures::prepare(zero, nine, double, out)
}
pub(crate) fn audit(
    public: &Path,
    sha: &str,
    candidate: &Path,
    zero: &Path,
    zero_sha: &str,
    target: &Path,
    target_sha: &str,
    out: &Path,
) -> bool {
    replay::audit(
        public, sha, candidate, zero, zero_sha, target, target_sha, out,
    )
}

const SETUP: usize = 0;
const ZERO: usize = 1;
const POINT_QUERIES: usize = 2;
const DECOMPOSITION: usize = 3;
const MATRIX: usize = 4;
const RECOVERY: usize = 5;
const VERIFICATION: usize = 6;
const JOURNAL: usize = 7;
const PHASES: [&str; 8] = [
    "setup",
    "zero_precomputation",
    "target_point_queries",
    "target_decomposition",
    "target_matrix",
    "target_recovery",
    "target_verification",
    "journal",
];

struct Clock {
    last: Instant,
    current: usize,
    ns: [u64; 8],
    work: [meter::Counts; 8],
    prior_work: meter::Counts,
}

impl Clock {
    fn new() -> Self {
        Self {
            last: Instant::now(),
            current: SETUP,
            ns: [0; 8],
            work: [meter::Counts::default(); 8],
            prior_work: meter::snapshot(),
        }
    }
    fn enter(&mut self, phase: usize) {
        let now = Instant::now();
        let elapsed: u64 = now.duration_since(self.last).as_nanos().try_into().unwrap();
        self.ns[self.current] += elapsed;
        let work = meter::snapshot();
        let delta = meter::difference(self.prior_work, work);
        self.work[self.current].fp_muls += delta.fp_muls;
        self.work[self.current].subgroup_muls += delta.subgroup_muls;
        self.work[self.current].group_add_calls += delta.group_add_calls;
        self.last = now;
        self.prior_work = work;
        self.current = phase;
    }
    fn total(&self) -> u64 {
        self.ns.iter().sum()
    }
}

struct Journals {
    inputs: std::fs::File,
    results: std::fs::File,
}

impl Journals {
    fn event(&mut self, clock: &mut Clock, input: bool, record: &Value) {
        let prior = clock.current;
        clock.enter(JOURNAL);
        append_journal(
            if input {
                &mut self.inputs
            } else {
                &mut self.results
            },
            record,
        );
        clock.enter(prior);
    }
}

fn physical_sum(case: &Case, terms: &Terms) -> Point {
    terms.iter().fold(INF, |sum, &(id, sign)| {
        case.c
            .add(sum, signed(&case.c, case.pool[id].p.into(), sign))
    })
}

fn solve(
    case: &Case,
    h: &integer::ZPoly,
    by_x: &BTreeMap<E, usize>,
    prefix: &Terms,
    target: Point,
    remaining: Point,
    context: Value,
    deadline: Instant,
    journals: &mut Journals,
    clock: &mut Clock,
) -> Value {
    let equations = if remaining.inf {
        None
    } else {
        Some(restrict_exact(case.c.f, case.c.b, h, remaining.x))
    };
    let input: Vec<_> = equations
        .as_ref()
        .map(|equations| {
            equations
                .iter()
                .map(|poly| {
                    f4_fp::normalise(
                        &poly
                            .iter()
                            .map(|(e, &v)| (e.iter().map(|&x| x as u32).collect(), v))
                            .collect::<Vec<_>>(),
                        case.p,
                        f4_fp::Ordering::Grevlex,
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    let equation_record = equations.as_ref().map(|polys| {
        polys
            .iter()
            .map(|p| p.iter().map(|(e, c)| json!([e, c])).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });
    journals.event(
        clock,
        true,
        &json!({"context":context,"prefix":prefix,
        "remaining_point":PointRecord::from(remaining),"equations":equation_record,
        "phase":"solve-pending"}),
    );
    let budget = deadline.saturating_duration_since(Instant::now());
    let mut accepted = BTreeSet::new();
    let mut roots = Vec::new();
    let mut f4_muls = 0;
    let mut max_rows = 0;
    let mut max_cols = 0;
    let mut attempted = false;
    let mut rejected_decode = 0;
    let verdict = if budget.is_zero() {
        "deadline-before-solve"
    } else if remaining.inf {
        // The same explicitly checked branch used in the verified parent prefix.
        assert_eq!(prefix.len(), 1);
        for triple in &case.zero_triples {
            if triple.iter().any(|t| prefix.iter().any(|v| v.0 == t.0)) {
                continue;
            }
            let mut terms = prefix.clone();
            terms.extend(triple);
            terms.sort();
            case.verify_terms(&terms, target);
            accepted.insert(terms);
        }
        "checked-infinity-branch"
    } else {
        attempted = true;
        let options = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12).with_budget(budget);
        let solved = f4_fp::solve(&input, 3, case.p, &options);
        f4_muls = solved.field_ops;
        max_rows = solved.max_rows;
        max_cols = solved.max_cols;
        let verdict = match solved.verdict {
            f4_fp::Verdict::Solutions(v) => {
                roots = v;
                "complete-field-roots"
            }
            f4_fp::Verdict::Inconsistent => "inconsistent",
            f4_fp::Verdict::Undetermined => "undetermined",
        };
        roots.sort();
        roots.dedup();
        for e in &roots {
            assert_eq!(e.len(), 3);
            assert!(e.iter().all(|&v| v < case.p));
            assert!(equations
                .as_ref()
                .unwrap()
                .iter()
                .all(|p| polynomial_value(p, e, case.p) == 0));
            let xs: Vec<_> = (0..case.p)
                .filter(|&x| {
                    let v = (fm((x + case.p - e[0]) % case.p, x, case.p) + e[1]) % case.p;
                    (fm(v, x, case.p) + case.p - e[2]) % case.p == 0
                })
                .collect();
            if xs.len() != 3 {
                rejected_decode += 1;
                continue;
            }
            let ids: Option<Vec<_>> = xs
                .iter()
                .map(|&x| by_x.get(&case.c.f.scalar(x)).copied())
                .collect();
            let Some(ids) = ids else {
                rejected_decode += 1;
                continue;
            };
            if ids.iter().any(|id| prefix.iter().any(|v| v.0 == *id)) {
                rejected_decode += 1;
                continue;
            }
            for mask in 0..8 {
                let mut terms = prefix.clone();
                for (i, &id) in ids.iter().enumerate() {
                    terms.push((id, if mask & (1 << i) == 0 { 1 } else { -1 }));
                }
                terms.sort();
                if physical_sum(case, &terms) != target {
                    continue;
                }
                if prefix.len() == 2 {
                    assert!(zero_sum(case, &terms));
                } else {
                    case.verify_terms(&terms, target);
                }
                accepted.insert(terms);
            }
        }
        verdict
    };
    let result = json!({"context":context,"prefix":prefix,
        "remaining_point":PointRecord::from(remaining),"equations":equation_record,
        "verdict":verdict,"field_roots":roots,"accepted_relations":accepted,
        "f4_attempted":attempted,"f4_row_reduction_fp_muls":f4_muls,
        "rejected_decode":rejected_decode,"max_rows":max_rows,"max_cols":max_cols});
    journals.event(clock, false, &result);
    result
}

fn complete(record: &Value) -> bool {
    matches!(
        record["verdict"].as_str(),
        Some("complete-field-roots" | "inconsistent" | "checked-infinity-branch")
    )
}

fn factor_solution(matrix: &Matrix) -> Option<Vec<u64>> {
    let factor_cols = matrix.cols - 1;
    if matrix.rank() != factor_cols {
        return None;
    }
    let rows = matrix
        .rows
        .iter()
        .map(|r| {
            let mut row = r.clone();
            assert_eq!(row.remove(factor_cols), 0);
            row
        })
        .collect();
    Matrix {
        rows,
        n: matrix.n,
        cols: factor_cols,
    }
    .solution()
}

fn verify_factor_logs(case: &Case, arm: usize, logs: &[u64]) -> bool {
    let mut values = vec![case.anchor];
    values.extend(logs);
    values.len() == case.columns(arm)
        && case.usable(arm).iter().all(|&id| {
            let factor = &case.pool[id];
            case.c
                .mul(case.g, mm(values[factor.col], factor.coef, case.n))
                == factor.p.into()
        })
}

pub(crate) fn execute(public: &Path, sha: &str, arm_name: &str, mode: &str, out: &Path) -> bool {
    let raw = checked_public(public, sha);
    let arm = ARMS.iter().position(|&name| name == arm_name).unwrap();
    let use_zero = match mode {
        "plain" => false,
        "zero" => true,
        _ => panic!("unregistered mode"),
    };
    assert!(!use_zero || [7, 9].contains(&arm));
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    write(
        &out.join("started.json"),
        &json!({"schema":"twist-zero-pipeline-started/v1",
        "public_input_sha256":sha,"arm":arm_name,"mode":mode}),
    );
    let mut journals = Journals {
        inputs: std::fs::File::create(out.join("system-inputs.jsonl")).unwrap(),
        results: std::fs::File::create(out.join("system-results.jsonl")).unwrap(),
    };
    meter::configure(raw["p"].as_u64().unwrap(), raw["n"].as_u64().unwrap());
    let memory_start = cold::allocation_begin();
    let mut clock = Clock::new();
    let deadline = clock.last + Duration::from_secs(1180);
    let case = cold::build_reusable(raw, arm);
    let (_, h) = integer::derive();
    let by_x: BTreeMap<_, _> = (0..case.l0).map(|id| (case.pool[id].p.x, id)).collect();
    let mut matrix = Matrix::new(case.n, case.columns(arm));
    let factor_cols = matrix.cols - 1;
    let mut zero_records = Vec::new();
    let mut zero_found = BTreeSet::new();
    let mut target_found = BTreeSet::new();
    let mut zero_projective = BTreeSet::new();
    let mut target_projective = BTreeSet::new();
    let mut zero_rows = 0usize;
    let mut undetermined = 0usize;
    let mut deadline_hit = false;
    let mut zero_f4_muls = 0u64;
    let mut target_f4_muls = 0u64;
    let mut zero_total_pairs = 0usize;
    let mut zero_first_full = None;
    clock.enter(ZERO);
    if use_zero {
        let group = actions(&case, arm);
        let pairs = pair_representatives(case.l0, &case.peels[arm], &group);
        zero_total_pairs = pairs.len();
        for (i, &(ordinary, os, image, ims)) in pairs.iter().enumerate() {
            if matrix.rank() == factor_cols {
                break;
            }
            if Instant::now() >= deadline {
                deadline_hit = true;
                break;
            }
            let prefix = vec![(ordinary, os), (image, ims)];
            let remaining = case.c.neg(physical_sum(&case, &prefix));
            assert!(!remaining.inf);
            let mut record = solve(
                &case,
                &h,
                &by_x,
                &prefix,
                INF,
                remaining,
                json!({"phase":"zero","system_index":i,"pair":[ordinary,os,image,ims]}),
                deadline,
                &mut journals,
                &mut clock,
            );
            if !complete(&record) {
                undetermined += 1;
            }
            zero_f4_muls += record["f4_row_reduction_fp_muls"].as_u64().unwrap();
            let accepted: BTreeSet<Terms> =
                serde_json::from_value(record["accepted_relations"].clone()).unwrap();
            let mut transported = BTreeSet::new();
            for terms in &accepted {
                for action in &group {
                    let mut image: Terms = terms
                        .iter()
                        .map(|&(id, sign)| {
                            let (to, s) = action[id];
                            (to, sign * s)
                        })
                        .collect();
                    image.sort();
                    assert!(zero_sum(&case, &image));
                    transported.insert(image);
                }
            }
            for terms in &transported {
                let new_tuple = zero_found.insert(terms.clone());
                let row = case.row(terms, &json!([0, 0]), arm);
                assert_eq!(row[factor_cols], 0);
                if let Some(key) = canonical_row(row.clone(), case.n) {
                    zero_projective.insert(key);
                } else if new_tuple {
                    zero_rows += 1;
                }
                if matrix.rank() < factor_cols {
                    matrix.insert(row);
                }
            }
            record["transported_relations"] = json!(transported);
            record["rank_after"] = json!(matrix.rank());
            journals.event(
                &mut clock,
                false,
                &json!({"phase":"zero-matrix-checkpoint",
                "system_index":i,"rank_after":matrix.rank(),"transported_relations":transported}),
            );
            zero_records.push(record);
            if matrix.rank() == factor_cols {
                zero_first_full = Some(i);
                break;
            }
        }
    }
    let zero_rank = matrix.rank();
    let factor_logs = if use_zero {
        factor_solution(&matrix)
    } else {
        None
    };
    let factor_verified = factor_logs
        .as_ref()
        .map(|logs| verify_factor_logs(&case, arm, logs));
    if factor_logs.is_some() {
        assert_eq!(factor_verified, Some(true));
    }
    clock.enter(POINT_QUERIES);
    let reusable_wall_ns = clock.total();
    // First target-dependent arithmetic: base construction above never checked Q.
    assert!(case.c.on_curve(case.q));
    assert_eq!(case.c.mul(case.q, case.n), INF);
    let mut query_records = Vec::new();
    let mut first_full = None;
    for (qi, item) in case.raw["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        if Instant::now() >= deadline {
            deadline_hit = true;
            break;
        }
        clock.enter(POINT_QUERIES);
        let target = case.residual(item);
        clock.enter(DECOMPOSITION);
        let mut got = BTreeSet::new();
        let mut systems = Vec::new();
        let mut query_complete = true;
        'peels: for &id in &case.peels[arm] {
            for sign in [-1, 1] {
                if Instant::now() >= deadline {
                    deadline_hit = true;
                    query_complete = false;
                    break 'peels;
                }
                let prefix = vec![(id, sign)];
                let remaining = case.c.add(
                    target,
                    case.c.neg(signed(&case.c, case.pool[id].p.into(), sign)),
                );
                let record = solve(
                    &case,
                    &h,
                    &by_x,
                    &prefix,
                    target,
                    remaining,
                    json!({"phase":"target","query_index":qi,"peel_id":id,"sign":sign}),
                    deadline,
                    &mut journals,
                    &mut clock,
                );
                if !complete(&record) {
                    undetermined += 1;
                    query_complete = false;
                }
                target_f4_muls += record["f4_row_reduction_fp_muls"].as_u64().unwrap();
                let relations: BTreeSet<Terms> =
                    serde_json::from_value(record["accepted_relations"].clone()).unwrap();
                got.extend(relations);
                systems.push(record);
            }
        }
        clock.enter(MATRIX);
        for terms in &got {
            target_found.insert(terms.clone());
            let row = case.row(terms, item, arm);
            if let Some(key) = canonical_row(row.clone(), case.n) {
                target_projective.insert(key);
            }
            if matrix.rank() < matrix.cols {
                matrix.insert(row);
            }
        }
        let record = json!({"query_index":qi,"a":item[0],"b":item[1],
            "residual":PointRecord::from(target),"complete":query_complete,
            "signed_relations":got,"rank_after":matrix.rank(),"systems":systems});
        journals.event(
            &mut clock,
            false,
            &json!({"phase":"target-matrix-checkpoint",
            "query_index":qi,"rank_after":matrix.rank(),"complete":query_complete,
            "signed_relations":got}),
        );
        query_records.push(record);
        if matrix.rank() == matrix.cols {
            first_full = Some(qi);
            break;
        }
        if deadline_hit {
            break;
        }
    }
    clock.enter(RECOVERY);
    let solution = matrix.solution();
    clock.enter(VERIFICATION);
    let verified = solution
        .as_ref()
        .map(|logs| case.verify_solution(logs, arm));
    if solution.is_some() {
        assert_eq!(verified, Some(true));
    }
    clock.enter(VERIFICATION);
    let cold_wall_ns = clock.total();
    let memory = cold::allocation_end(memory_start);
    let status = if undetermined > 0 || deadline_hit {
        "partial"
    } else if first_full.is_none() {
        "censored"
    } else {
        "pass"
    };
    let deterministic = json!({"p":case.p,"seed":case.seed,"n":case.n,"arm":arm_name,"mode":mode,
        "public_input_sha256":sha,"status":status,"geometry":case.geometry,
        "columns":matrix.cols,"unanchored_factor_columns":factor_cols,
        "zero_pair_representatives":zero_total_pairs,"zero_systems":zero_records,
        "zero_factor_log_rank":zero_rank,"first_full_factor_system":zero_first_full,
        "zero_factor_log_solution":factor_logs,"zero_factor_log_points_verified":factor_verified,
        "zero_signed_physical_set":zero_found,"zero_projective_rows":zero_projective,
        "zero_augmented_rows":zero_rows,"all_zero_target_coefficients_checked":true,
        "query_trace_cap":case.raw["query_trace"].as_array().unwrap().len(),
        "target_queries":query_records,"first_full_rank_query":first_full,
        "target_signed_physical_set":target_found,"target_projective_rows":target_projective,
        "rank":matrix.rank(),"solved_logs":solution,
        "recovered":solution.as_ref().map(|v|v[matrix.cols-1]),
        "all_usable_factor_logs_and_target_verified":verified,
        "target_recovery_attempted":true,"undetermined_systems":undetermined,
        "cooperative_deadline_hit":deadline_hit,
        "work_by_phase":PHASES.iter().zip(clock.work).collect::<BTreeMap<_,_>>(),
        "zero_f4_row_reduction_fp_muls":zero_f4_muls,
        "target_f4_row_reduction_fp_muls":target_f4_muls});
    let value = json!({"schema":"twist-zero-one-target-pipeline/v1","deterministic":deterministic,
        "diagnostic_wall_ns":{"phases":PHASES.iter().zip(clock.ns).collect::<BTreeMap<_,_>>(),
            "reusable_with_zero_journals":reusable_wall_ns,"online_with_target_journals":cold_wall_ns-reusable_wall_ns,
            "cold_including_journals":cold_wall_ns,"journal":clock.ns[JOURNAL]},"memory":memory,
        "scope":"Actual public-only base reconstruction, stopped zero precomputation and target descent through point verification. System-boundary zero stop and query-boundary target stop. Journal time retained separately; final record serialization, input decoding and process startup excluded. Shared-host durations are not admitted timing. Fp/group and F4 row-reduction counters are partial; complete calibrated IC/rho price remains open."});
    write(&out.join("record.json"), &value);
    println!(
        "{}",
        json!({"status":status,"p":case.p,"seed":case.seed,"arm":arm_name,"mode":mode,
        "zero_systems":value["deterministic"]["zero_systems"].as_array().unwrap().len(),
        "target_queries":value["deterministic"]["target_queries"].as_array().unwrap().len(),
        "rank":matrix.rank(),"columns":matrix.cols})
    );
    status == "pass"
}

pub(crate) fn parity(first: &Path, second: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let a = std::fs::read(first.join("record.json")).unwrap();
    let b = std::fs::read(second.join("record.json")).unwrap();
    let av: Value = serde_json::from_slice(&a).unwrap();
    let bv: Value = serde_json::from_slice(&b).unwrap();
    let pass = av["schema"] == "twist-zero-one-target-pipeline/v1"
        && av["schema"] == bv["schema"]
        && av["deterministic"] == bv["deterministic"];
    write(
        out,
        &json!({"schema":"twist-zero-pipeline-aa-parity/v1","status":if pass{"pass"}else{"fail"},
        "first_sha256":hex::encode(hash::sha256::sha256(&a)),"second_sha256":hex::encode(hash::sha256::sha256(&b)),
        "scope":"Exact deterministic record equality including every system, root, physical row, rank, solution and scoped work counter. Both raw timing/memory observations retained."}),
    );
    pass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reusable_setup_does_not_validate_or_use_target_coordinates() {
        let raw: Value =
            serde_json::from_str(include_str!("../fixtures/p37-seed0-public.json")).unwrap();
        let mut altered = raw.clone();
        altered["target_x"] = json!([0, 0, 0, 0]);
        altered["target_y"] = json!([0, 0, 0, 0]);
        let a = cold::build_reusable(raw, 7);
        let a_work = meter::snapshot();
        let b = cold::build_reusable(altered.clone(), 7);
        let b_work = meter::snapshot();
        assert_eq!(a.geometry, b.geometry);
        assert_eq!(
            serde_json::to_value(a_work).unwrap(),
            serde_json::to_value(b_work).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&a.pool).unwrap(),
            serde_json::to_value(&b.pool).unwrap()
        );
        assert_ne!(a.q, b.q);
        assert!(!b.c.on_curve(b.q));
        assert!(std::panic::catch_unwind(|| cold::build(altered, 7)).is_err());
    }
}
