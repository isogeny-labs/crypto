//! Fixed-zero pair quotient and exact S4 collector. No reference input enters collect.
use super::*;
use std::time::Instant;

type Pair = (usize, i8, usize, i8);
type Action = Vec<(usize, i8)>;

fn transform(pair: Pair, action: &Action) -> Pair {
    let (i, si, j, sj) = pair;
    let (a, sa) = action[i];
    let (b, sb) = action[j];
    (a, si * sa, b, sj * sb)
}

fn pair_representatives(l0: usize, images: &[usize], actions: &[Action]) -> Vec<Pair> {
    let mut reps = BTreeSet::new();
    let image_set: BTreeSet<_> = images.iter().copied().collect();
    assert_eq!(image_set.len(), images.len());
    for i in 0..l0 {
        for &j in images {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let orbit: BTreeSet<_> = actions
                        .iter()
                        .map(|h| transform((i, si, j, sj), h))
                        .collect();
                    assert_eq!(orbit.len(), actions.len(), "pair action is not free");
                    assert!(orbit.iter().all(|&(a, sa, b, sb)| a < l0
                        && image_set.contains(&b)
                        && [-1, 1].contains(&sa)
                        && [-1, 1].contains(&sb)));
                    reps.insert(*orbit.first().unwrap());
                }
            }
        }
    }
    assert_eq!(reps.len() * actions.len(), 4 * l0 * images.len());
    reps.into_iter().collect()
}

fn actions(case: &Case, arm: usize) -> Vec<Action> {
    let usable = case.usable(arm);
    let by_x: BTreeMap<_, _> = usable.iter().map(|&id| (case.pool[id].p.x, id)).collect();
    assert_eq!(by_x.len(), usable.len(), "signed factor domains overlap");
    let mut result = Vec::new();
    let mut scalars = BTreeSet::new();
    for outer in [-1, 1] {
        for j in 0..4 {
            for k in 0..3 {
                let mut scalar = mm(fpow(case.lf, j, case.n), fpow(case.lc, k, case.n), case.n);
                if outer == -1 {
                    scalar = negn(scalar, case.n);
                }
                assert_ne!(scalar, 0);
                assert!(scalars.insert(scalar), "duplicate subgroup action");
                let mut table = vec![(usize::MAX, 0); case.pool.len()];
                for &id in &usable {
                    let mut image: Point = case.pool[id].p.into();
                    for _ in 0..j {
                        image = frob(&case.c, image);
                    }
                    for _ in 0..k {
                        image = psi(&case.c, image, case.omega);
                    }
                    image = signed(&case.c, image, outer);
                    assert!(!image.inf && case.c.on_curve(image));
                    let to = by_x[&image.x];
                    let sign = matches_sign(&case.c, case.pool[to].p.into(), image);
                    assert_eq!(
                        id < case.l0,
                        to < case.l0,
                        "action changed factor component"
                    );
                    let from = &case.pool[id];
                    let target = &case.pool[to];
                    assert_eq!(from.col, target.col);
                    assert_eq!(
                        mm(from.coef, scalar, case.n),
                        if sign == 1 {
                            target.coef
                        } else {
                            negn(target.coef, case.n)
                        }
                    );
                    table[id] = (to, sign);
                }
                result.push(table);
            }
        }
    }
    assert_eq!(result.len(), 24);
    result
}

fn zero_sum(case: &Case, terms: &Terms) -> bool {
    terms.len() == 5
        && terms
            .iter()
            .all(|&(id, s)| id < case.pool.len() && [-1, 1].contains(&s))
        && terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len() == 5
        && terms
            .iter()
            .map(|t| case.pool[t.0].p.x)
            .collect::<BTreeSet<_>>()
            .len()
            == 5
        && terms.iter().fold(INF, |s, &(id, sign)| {
            case.c.add(s, signed(&case.c, case.pool[id].p.into(), sign))
        }) == INF
}

pub(super) fn collect(public: &Path, sha: &str, arm_name: &str, out: &Path) -> bool {
    let raw = checked_public(public, sha);
    let arm = ARMS.iter().position(|&name| name == arm_name).unwrap();
    assert!(
        [7, 9].contains(&arm),
        "only matched mixed arms are registered"
    );
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let clock = Instant::now();
    let deadline = clock + Duration::from_secs(1180);
    let case = cold::build(raw, arm);
    let (_, h) = integer::derive();
    let group = actions(&case, arm);
    let representatives = pair_representatives(case.l0, &case.peels[arm], &group);
    let by_x: BTreeMap<_, _> = (0..case.l0).map(|id| (case.pool[id].p.x, id)).collect();
    let setup_work = meter::snapshot();
    let setup_ns = clock.elapsed().as_nanos();
    meter::reset();
    let mut found = BTreeSet::new();
    let mut records = Vec::new();
    let mut undetermined = 0usize;
    let mut f4_muls = 0u64;
    let mut deadline_hit = false;
    for &(ordinary, os, image, ims) in &representatives {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            deadline_hit = true;
            break;
        }
        let r = case.c.neg(case.c.add(
            signed(&case.c, case.pool[ordinary].p.into(), os),
            signed(&case.c, case.pool[image].p.into(), ims),
        ));
        assert!(
            !r.inf,
            "disjoint signed components unexpectedly gave infinity"
        );
        let equations = restrict_exact(case.c.f, case.c.b, &h, r.x);
        let input: Vec<_> = equations
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
            .collect();
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            deadline_hit = true;
            break;
        }
        let options = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12).with_budget(remaining);
        let solved = f4_fp::solve(&input, 3, case.p, &options);
        f4_muls += solved.field_ops;
        let (mut roots, verdict) = match solved.verdict {
            f4_fp::Verdict::Solutions(v) => (v, "complete-field-roots"),
            f4_fp::Verdict::Inconsistent => (Vec::new(), "inconsistent"),
            f4_fp::Verdict::Undetermined => {
                undetermined += 1;
                (Vec::new(), "undetermined")
            }
        };
        roots.sort();
        roots.dedup();
        let mut accepted = BTreeSet::new();
        let mut rejected_decode = 0usize;
        for e in &roots {
            assert_eq!(e.len(), 3);
            assert!(e.iter().all(|&v| v < case.p));
            assert!(equations
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
            if ids.contains(&ordinary) {
                rejected_decode += 1;
                continue;
            }
            for mask in 0..8 {
                let mut terms = vec![(ordinary, os), (image, ims)];
                for (k, &id) in ids.iter().enumerate() {
                    terms.push((id, if mask & (1 << k) == 0 { 1 } else { -1 }));
                }
                terms.sort();
                if !zero_sum(&case, &terms) {
                    continue;
                }
                accepted.insert(terms.clone());
                for action in &group {
                    let mut transported: Terms = terms
                        .iter()
                        .map(|&(id, sign)| {
                            let (to, s) = action[id];
                            (to, sign * s)
                        })
                        .collect();
                    transported.sort();
                    assert!(zero_sum(&case, &transported));
                    found.insert(transported);
                }
            }
        }
        records.push(json!({"pair":[ordinary,os,image,ims],"remaining_point":PointRecord::from(r),
            "equations":equations.iter().map(|p|p.iter().map(|(e,c)|json!([e,c])).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "verdict":verdict,"field_roots":roots,"accepted_representative_relations":accepted,
            "rejected_decode":rejected_decode,"f4_row_reduction_fp_muls":solved.field_ops,
            "max_rows":solved.max_rows,"max_cols":solved.max_cols}));
    }
    let cols = case.columns(arm) - 1;
    let mut matrix = Matrix::new(case.n, cols);
    let mut projective = BTreeSet::new();
    let mut zero_rows = 0usize;
    for terms in &found {
        let mut row = case.row(terms, &json!([0, 0]), arm);
        assert_eq!(
            row.remove(cols),
            0,
            "zero relation acquired a target coefficient"
        );
        if let Some(key) = canonical_row(row.clone(), case.n) {
            projective.insert(key);
        } else {
            zero_rows += 1;
        }
        matrix.insert(row);
    }
    let solution = matrix.solution();
    let point_verified = solution
        .as_ref()
        .map(|logs| {
            let mut reps = vec![case.anchor];
            reps.extend(logs);
            case.usable(arm).iter().all(|&id| {
                let f = &case.pool[id];
                case.c.mul(case.g, mm(reps[f.col], f.coef, case.n)) == f.p.into()
            })
        })
        .unwrap_or(false);
    assert_eq!(point_verified, matrix.rank() == cols);
    let complete = records.len() == representatives.len() && undetermined == 0 && !deadline_hit;
    let total_ns = clock.elapsed().as_nanos();
    let value = json!({"schema":"twist-zero-five-polynomial/v1",
        "status":if complete {"complete-polynomial-enumeration"} else {"partial"},
        "p":case.p,"seed":case.seed,"arm":ARMS[arm],"public_input_sha256":sha,
        "ordinary_abscissae":case.l0,"geometry":case.geometry,"checked_actions":group.len(),
        "signed_pairs":4*case.l0*case.peels[arm].len(),"pair_representatives":representatives.len(),
        "systems_attempted":records.len(),"undetermined_systems":undetermined,"cooperative_deadline_hit":deadline_hit,
        "signed_physical_relations":found.len(),"signed_set_sha256":hex::encode(hash::sha256::sha256(&serde_json::to_vec(&found).unwrap())),
        "signed_physical_set":found,"zero_augmented_rows":zero_rows,"unique_projective_augmented_rows":projective.len(),
        "factor_log_rank":matrix.rank(),"unanchored_factor_columns":cols,"factor_log_solution":solution,
        "factor_log_solution_point_verified":point_verified,"target_column_zero":true,"target_recovery_attempted":false,
        "setup_work":setup_work,"collection_and_matrix_work":meter::snapshot(),"f4_row_reduction_fp_muls":f4_muls,
        "diagnostic_elapsed_ns":{"setup":setup_ns,"remaining":total_ns-setup_ns,"total":total_ns},
        "systems":records,"scope":"Complete zero-only polynomial enumeration if every system is complete; actual lifts, map, action quotient, roots, groups and matrix charged in diagnostics. Shared migrated host: elapsed values are not admitted timing or complete calibrated IC prices. No reference logs, BSGS or physical sets read by collector."});
    write(&out.join("record.json"), &value);
    println!(
        "{}",
        json!({"status":value["status"],"p":case.p,"seed":case.seed,"arm":ARMS[arm],
        "systems":value["systems_attempted"],"relations":value["signed_physical_relations"],"rank":value["factor_log_rank"]})
    );
    complete
}

fn inspect(
    case: &Case,
    arm: usize,
    sha: &str,
    value: &Value,
    expected: &BTreeSet<Terms>,
) -> Result<Value, &'static str> {
    if value["p"] != case.p
        || value["seed"] != case.seed
        || value["arm"] != ARMS[arm]
        || value["public_input_sha256"] != sha
    {
        return Err("public-seal");
    }
    if value["status"] != "complete-polynomial-enumeration"
        || value["undetermined_systems"] != 0
        || value["cooperative_deadline_hit"] != false
        || value["systems_attempted"] != value["pair_representatives"]
    {
        return Err("incomplete-systems");
    }
    let actual: BTreeSet<Terms> = serde_json::from_value(value["signed_physical_set"].clone())
        .map_err(|_| "invalid-physical-set")?;
    if actual != *expected {
        return Err("physical-set");
    }
    if value["signed_physical_relations"] != actual.len()
        || value["signed_set_sha256"]
            != hex::encode(hash::sha256::sha256(&serde_json::to_vec(&actual).unwrap()))
    {
        return Err("physical-set-seal");
    }
    let usable = case.usable(arm);
    let cols = case.columns(arm) - 1;
    let mut rows = Vec::new();
    let mut projective = BTreeSet::new();
    let mut zero = 0usize;
    for terms in &actual {
        if terms.iter().filter(|t| t.0 < case.l0).count() != 4
            || terms.iter().any(|t| !usable.contains(&t.0))
            || !zero_sum(case, terms)
        {
            return Err("physical-group-relation");
        }
        let mut row = case.row(terms, &json!([0, 0]), arm);
        if row.remove(cols) != 0 {
            return Err("nonzero-target-column");
        }
        if let Some(key) = canonical_row(row.clone(), case.n) {
            projective.insert(key);
        } else {
            zero += 1;
        }
        rows.push(row);
    }
    let (rank, solution) = independent_rref(&rows, cols, case.n);
    let verified = solution
        .as_ref()
        .map(|logs| {
            let mut reps = vec![case.anchor];
            reps.extend(logs);
            usable.iter().all(|&id| {
                let f = &case.pool[id];
                case.c.mul(case.g, mm(reps[f.col], f.coef, case.n)) == f.p.into()
            })
        })
        .unwrap_or(false);
    if value["factor_log_rank"] != rank
        || value["unanchored_factor_columns"] != cols
        || value["zero_augmented_rows"] != zero
        || value["unique_projective_augmented_rows"] != projective.len()
        || value["target_column_zero"] != true
        || value["target_recovery_attempted"] != false
        || value["factor_log_solution_point_verified"] != verified
        || verified != (rank == cols)
        || value["factor_log_solution"] != json!(solution)
    {
        return Err("independent-matrix");
    }
    if value["checked_actions"] != 24
        || value["signed_pairs"] != 4 * case.l0 * case.l0
        || value["pair_representatives"].as_u64().unwrap_or(0) * 24
            != (4 * case.l0 * case.l0) as u64
    {
        return Err("quotient-accounting");
    }
    Ok(
        json!({"relations":actual.len(),"factor_log_rank":rank,"unanchored_factor_columns":cols,
        "zero_augmented_rows":zero,"unique_projective_augmented_rows":projective.len(),
        "factor_log_solution_point_verified":verified}),
    )
}

fn reseal_set(value: &mut Value) {
    let actual: BTreeSet<Terms> =
        serde_json::from_value(value["signed_physical_set"].clone()).unwrap();
    value["signed_physical_relations"] = json!(actual.len());
    value["signed_set_sha256"] = json!(hex::encode(hash::sha256::sha256(
        &serde_json::to_vec(&actual).unwrap()
    )));
}

pub(super) fn audit(
    public: &Path,
    sha: &str,
    candidate: &Path,
    reference: &Path,
    out: &Path,
) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(checked_public(public, sha), out);
    let bytes = std::fs::read(candidate.join("record.json")).unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    let arm = ARMS.iter().position(|&a| value["arm"] == a).unwrap();
    assert!([7, 9].contains(&arm));
    let prior_bytes = std::fs::read(reference.join(format!("{}.json", ARMS[arm]))).unwrap();
    let prior: Value = serde_json::from_slice(&prior_bytes).unwrap();
    let prior_summary: Value =
        serde_json::from_slice(&std::fs::read(reference.join("summary.json")).unwrap()).unwrap();
    assert_eq!(prior_summary["public_input_sha256"], sha);
    assert_eq!(prior["status"], "complete-oracle-census");
    let expected: BTreeSet<Terms> =
        serde_json::from_value(prior["signed_physical_set"].clone()).unwrap();
    assert_eq!(
        prior["signed_set_sha256"],
        hex::encode(hash::sha256::sha256(
            &serde_json::to_vec(&expected).unwrap()
        ))
    );
    let result = inspect(&case, arm, sha, &value, &expected);
    let mut controls = Vec::new();
    if result.is_ok() {
        let mut changed = value.clone();
        changed["public_input_sha256"] = json!("corrupted-public-seal");
        assert_eq!(
            inspect(&case, arm, sha, &changed, &expected).unwrap_err(),
            "public-seal"
        );
        controls.push(json!({"control":"changed-public-binding","rejected_as":"public-seal"}));
        let mut changed = value.clone();
        changed["undetermined_systems"] = json!(1);
        assert_eq!(
            inspect(&case, arm, sha, &changed, &expected).unwrap_err(),
            "incomplete-systems"
        );
        controls.push(json!({"control":"unresolved-system","rejected_as":"incomplete-systems"}));
        if !expected.is_empty() {
            let mut changed = value.clone();
            changed["signed_physical_set"].as_array_mut().unwrap().pop();
            reseal_set(&mut changed);
            assert_eq!(
                inspect(&case, arm, sha, &changed, &expected).unwrap_err(),
                "physical-set"
            );
            controls.push(json!({"control":"omitted-tuple-with-consistent-set-hash","rejected_as":"physical-set"}));
            let mut changed = value.clone();
            let sign = changed["signed_physical_set"][0][0][1].as_i64().unwrap();
            changed["signed_physical_set"][0][0][1] = json!(-sign);
            reseal_set(&mut changed);
            assert_eq!(
                inspect(&case, arm, sha, &changed, &expected).unwrap_err(),
                "physical-set"
            );
            controls.push(json!({"control":"changed-sign-with-consistent-set-hash","rejected_as":"physical-set"}));
        }
    }
    let pass = result.is_ok();
    write(
        &out.join("summary.json"),
        &json!({"schema":"twist-zero-five-polynomial-audit/v1",
        "status":if pass {"pass"} else {"fail"},"p":case.p,"seed":case.seed,"arm":ARMS[arm],
        "public_input_sha256":sha,"candidate_sha256":hex::encode(hash::sha256::sha256(&bytes)),
        "reference_sha256":hex::encode(hash::sha256::sha256(&prior_bytes)),"independent_checks":result.ok(),
        "corruption_controls":controls,"scope":"Exact physical equality to immutable census independently confirmed by the prior public-point enumeration; new point-sum checks and separate RREF against the candidate's incremental matrix. Shared public arithmetic and orbit builder. No BSGS or census repeated; no admitted timing or target-recovery claim."}),
    );
    pass
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_direct_six_action_pair_partition_has_no_missing_orbit() {
        let mut group = Vec::new();
        for sign in [-1, 1] {
            for rotation in 0..3 {
                group.push(
                    (0..6)
                        .map(|id| (id / 3 * 3 + (id % 3 + rotation) % 3, sign))
                        .collect(),
                );
            }
        }
        let reps = pair_representatives(3, &[3, 4, 5], &group);
        assert_eq!(reps.len(), 6);
        let covered: BTreeSet<_> = reps
            .iter()
            .flat_map(|&p| group.iter().map(move |h| transform(p, h)))
            .collect();
        let direct: BTreeSet<_> = (0..3)
            .flat_map(|i| {
                (3..6).flat_map(move |j| {
                    [-1, 1]
                        .into_iter()
                        .flat_map(move |s| [-1, 1].into_iter().map(move |t| (i, s, j, t)))
                })
            })
            .collect();
        assert_eq!(covered, direct);
    }
    #[test]
    fn actual_p37_cold_lifts_and_both_images_have_free_24_action_pair_quotient() {
        let raw: Value =
            serde_json::from_str(include_str!("../fixtures/p37-seed0-public.json")).unwrap();
        for arm in [7, 9] {
            let case = cold::build(raw.clone(), arm);
            let h = actions(&case, arm);
            let reps = pair_representatives(case.l0, &case.peels[arm], &h);
            assert_eq!(case.l0, 36);
            assert_eq!(reps.len(), 216);
        }
    }
}
