//! Post-run prefix replay against sealed, previously independently checked sets.
//! Scalar maps and RREF differ from the candidate's action tables and matrix.
use super::*;

fn require(ok: bool, why: &'static str) -> Result<(), &'static str> {
    if ok {
        Ok(())
    } else {
        Err(why)
    }
}
fn terms(value: &Value) -> Result<BTreeSet<Terms>, &'static str> {
    serde_json::from_value(value.clone()).map_err(|_| "physical-set-format")
}
fn number(value: &Value) -> Result<u64, &'static str> {
    value.as_u64().ok_or("numeric-field")
}

fn physical(case: &Case, signed_terms: &Terms, target: Point) -> bool {
    !signed_terms.is_empty()
        && signed_terms
            .iter()
            .all(|&(id, s)| id < case.pool.len() && [-1, 1].contains(&s))
        && signed_terms
            .iter()
            .map(|t| t.0)
            .collect::<BTreeSet<_>>()
            .len()
            == signed_terms.len()
        && signed_terms
            .iter()
            .map(|t| case.pool[t.0].p.x)
            .collect::<BTreeSet<_>>()
            .len()
            == signed_terms.len()
        && physical_sum(case, signed_terms) == target
}

fn root_records(
    case: &Case,
    h: &integer::ZPoly,
    system: &Value,
    prefix: &Terms,
    target: Point,
    remaining: Point,
    expected: &BTreeSet<Terms>,
) -> Result<u64, &'static str> {
    require(complete(system), "incomplete-system")?;
    require(system["prefix"] == json!(prefix), "system-prefix")?;
    require(
        system["remaining_point"] == json!(PointRecord::from(remaining)),
        "system-residual",
    )?;
    let got = terms(&system["accepted_relations"])?;
    require(&got == expected, "per-system-physical-set")?;
    for tuple in &got {
        require(physical(case, tuple, target), "physical-point-sum")?;
        require(prefix.iter().all(|t| tuple.contains(t)), "system-prefix")?;
    }
    let work = number(&system["f4_row_reduction_fp_muls"])?;
    if remaining.inf {
        require(
            system["f4_attempted"] == false
                && system["verdict"] == "checked-infinity-branch"
                && work == 0
                && system["equations"].is_null(),
            "infinity-work",
        )?;
        return Ok(work);
    }
    require(system["f4_attempted"] == true, "missing-finite-system")?;
    let equations = restrict_exact(case.c.f, case.c.b, h, remaining.x);
    let encoded = json!(equations
        .iter()
        .map(|p| p.iter().map(|(e, c)| json!([e, c])).collect::<Vec<_>>())
        .collect::<Vec<_>>());
    require(system["equations"] == encoded, "system-equations")?;
    let roots: Vec<Vec<u64>> =
        serde_json::from_value(system["field_roots"].clone()).map_err(|_| "root-format")?;
    require(roots.windows(2).all(|w| w[0] < w[1]), "root-order")?;
    for e in &roots {
        require(e.len() == 3 && e.iter().all(|&v| v < case.p), "root-format")?;
        require(
            equations
                .iter()
                .all(|p| polynomial_value(p, e, case.p) == 0),
            "false-field-root",
        )?;
    }
    for tuple in &got {
        let triple: Vec<_> = tuple.iter().filter(|t| !prefix.contains(t)).collect();
        require(
            triple.len() == 3 && triple.iter().all(|t| t.0 < case.l0),
            "ordinary-triple",
        )?;
        let xs: Vec<_> = triple.iter().map(|t| case.pool[t.0].p.x[0]).collect();
        let e = vec![
            (xs[0] + xs[1] + xs[2]) % case.p,
            (fm(xs[0], xs[1], case.p) + fm(xs[0], xs[2], case.p) + fm(xs[1], xs[2], case.p))
                % case.p,
            fm(fm(xs[0], xs[1], case.p), xs[2], case.p),
        ];
        require(
            roots.binary_search(&e).is_ok(),
            "accepted-tuple-without-root",
        )?;
    }
    Ok(work)
}

fn inspect(
    case: &Case,
    sha: &str,
    value: &Value,
    zero: Option<&Value>,
    target_reference: &Value,
) -> Result<Value, &'static str> {
    require(
        value["schema"] == "twist-zero-one-target-pipeline/v1",
        "schema",
    )?;
    let d = &value["deterministic"];
    require(d["public_input_sha256"] == sha, "public-seal")?;
    require(
        d["p"] == case.p && d["seed"] == case.seed && d["n"] == case.n,
        "case-identity",
    )?;
    let arm = ARMS.iter().position(|&a| d["arm"] == a).ok_or("arm")?;
    require(
        target_reference["p"] == case.p
            && target_reference["seed"] == case.seed
            && target_reference["arm"] == ARMS[arm],
        "target-reference-identity",
    )?;
    let use_zero = match d["mode"].as_str() {
        Some("zero") => true,
        Some("plain") => false,
        _ => return Err("mode"),
    };
    require(!use_zero || [7, 9].contains(&arm), "mode-arm")?;
    require(
        number(&d["undetermined_systems"])? == 0 && d["cooperative_deadline_hit"] == false,
        "incomplete-pipeline",
    )?;
    require(
        d["target_recovery_attempted"] == true,
        "target-not-attempted",
    )?;
    require(
        d["all_zero_target_coefficients_checked"] == true,
        "zero-target-coefficients",
    )?;
    let cols = case.columns(arm);
    require(
        d["columns"] == cols && d["unanchored_factor_columns"] == cols - 1,
        "matrix-dimension",
    )?;
    let (_, h) = integer::derive();
    let zero_systems = d["zero_systems"].as_array().ok_or("zero-record-format")?;
    let mut rows = Vec::new();
    let mut zero_set = BTreeSet::new();
    let mut zero_projective = BTreeSet::new();
    let mut target_set = BTreeSet::new();
    let mut target_projective = BTreeSet::new();
    let mut zero_rows = 0usize;
    let mut rank = 0usize;
    let mut zero_work = 0u64;
    let mut first_factor = None;
    let mut zero_solution = None;
    if use_zero {
        let reference = zero.ok_or("missing-zero-reference")?;
        require(
            reference["public_input_sha256"] == sha
                && reference["p"] == case.p
                && reference["seed"] == case.seed
                && reference["arm"] == ARMS[arm]
                && reference["status"] == "complete-polynomial-enumeration",
            "zero-reference-identity",
        )?;
        let complete_systems = reference["systems"]
            .as_array()
            .ok_or("zero-reference-format")?;
        let pairs: Vec<Pair> = complete_systems
            .iter()
            .map(|r| serde_json::from_value(r["pair"].clone()).map_err(|_| "pair-format"))
            .collect::<Result<_, _>>()?;
        let maps = independent_audit::scalar_maps(case, arm)?;
        let ordinary: Vec<_> = (0..case.l0).collect();
        independent_audit::partition(&pairs, &ordinary, &case.peels[arm], &maps)?;
        require(
            d["zero_pair_representatives"] == pairs.len() && zero_systems.len() <= pairs.len(),
            "pair-count",
        )?;
        for (i, system) in zero_systems.iter().enumerate() {
            require(rank < cols - 1, "late-zero-stop")?;
            let old = &complete_systems[i];
            let pair = pairs[i];
            require(
                system["context"]
                    == json!({"phase":"zero","system_index":i,
                "pair":[pair.0,pair.1,pair.2,pair.3]}),
                "zero-pair-order",
            )?;
            let expected = terms(&old["accepted_representative_relations"])?;
            let prefix = vec![(pair.0, pair.1), (pair.2, pair.3)];
            let remaining = case.c.neg(physical_sum(case, &prefix));
            zero_work += root_records(case, &h, system, &prefix, INF, remaining, &expected)?;
            for key in [
                "field_roots",
                "f4_row_reduction_fp_muls",
                "max_rows",
                "max_cols",
            ] {
                require(system[key] == old[key], "zero-reference-work-or-roots")?;
            }
            let mut transported = BTreeSet::new();
            for tuple in &expected {
                for map in &maps {
                    let mut image: Terms = tuple
                        .iter()
                        .map(|t| map.get(t).copied().ok_or("transport-domain"))
                        .collect::<Result<_, _>>()?;
                    image.sort();
                    require(physical(case, &image, INF), "transported-point-sum")?;
                    transported.insert(image);
                }
            }
            require(
                terms(&system["transported_relations"])? == transported,
                "zero-transported-set",
            )?;
            for tuple in &transported {
                let new_tuple = zero_set.insert(tuple.clone());
                let row = case.row(tuple, &json!([0, 0]), arm);
                require(row[cols - 1] == 0, "zero-target-coefficient")?;
                if let Some(key) = canonical_row(row.clone(), case.n) {
                    zero_projective.insert(key);
                } else if new_tuple {
                    zero_rows += 1;
                }
                rows.push(row);
            }
            rank = independent_rref(&rows, cols, case.n).0;
            require(system["rank_after"] == rank, "zero-rank")?;
            if rank == cols - 1 {
                first_factor = Some(i);
            }
        }
        require(
            rank == cols - 1 || zero_systems.len() == pairs.len(),
            "early-zero-stop",
        )?;
        if rank == cols - 1 {
            let factor_rows: Vec<_> = rows
                .iter()
                .map(|r| {
                    let mut row = r.clone();
                    assert_eq!(row.remove(cols - 1), 0);
                    row
                })
                .collect();
            zero_solution = independent_rref(&factor_rows, cols - 1, case.n).1;
        }
    } else {
        require(
            zero_systems.is_empty() && d["zero_pair_representatives"] == 0,
            "plain-has-zero-work",
        )?;
    }
    require(
        d["zero_factor_log_rank"] == rank && d["first_full_factor_system"] == json!(first_factor),
        "factor-stop",
    )?;
    require(
        terms(&d["zero_signed_physical_set"])? == zero_set,
        "zero-prefix-set",
    )?;
    require(
        d["zero_projective_rows"] == json!(zero_projective)
            && d["zero_augmented_rows"] == zero_rows,
        "zero-row-counts",
    )?;
    require(
        d["zero_f4_row_reduction_fp_muls"] == zero_work,
        "zero-f4-work-sum",
    )?;
    require(
        d["zero_factor_log_solution"] == json!(zero_solution),
        "factor-solution",
    )?;
    let zero_verified = zero_solution
        .as_ref()
        .map(|logs| verify_factor_logs(case, arm, logs));
    require(
        zero_verified != Some(false)
            && d["zero_factor_log_points_verified"] == json!(zero_verified),
        "factor-point-verification",
    )?;
    let queries = d["target_queries"].as_array().ok_or("query-format")?;
    let cap = case.raw["query_trace"].as_array().unwrap().len();
    require(
        d["query_trace_cap"] == cap && queries.len() <= cap,
        "query-cap",
    )?;
    let ref_trace = target_reference["trace"]
        .as_array()
        .ok_or("target-reference-format")?;
    require(queries.len() <= ref_trace.len(), "reference-query-cap")?;
    let mut first_full = None;
    let mut target_work = 0u64;
    let mut target_systems = 0usize;
    for (qi, query) in queries.iter().enumerate() {
        require(rank < cols, "late-target-stop")?;
        let item = &case.raw["query_trace"][qi];
        let target = case.residual(item);
        require(
            query["query_index"] == qi
                && query["a"] == item[0]
                && query["b"] == item[1]
                && query["residual"] == json!(PointRecord::from(target)),
            "query-binding",
        )?;
        require(query["complete"] == true, "incomplete-query")?;
        let expected = terms(&ref_trace[qi]["signed_relations"])?;
        let got = terms(&query["signed_relations"])?;
        require(got == expected, "target-physical-set")?;
        let systems = query["systems"].as_array().ok_or("target-system-format")?;
        require(
            systems.len() == 2 * case.peels[arm].len(),
            "target-system-count",
        )?;
        let mut per_system_union = BTreeSet::new();
        let mut finite = 0usize;
        for (j, system) in systems.iter().enumerate() {
            let id = case.peels[arm][j / 2];
            let sign = if j % 2 == 0 { -1 } else { 1 };
            let prefix = vec![(id, sign)];
            require(
                system["context"]
                    == json!({"phase":"target","query_index":qi,
                "peel_id":id,"sign":sign}),
                "target-peel-order",
            )?;
            let remaining = case.c.add(
                target,
                case.c.neg(signed(&case.c, case.pool[id].p.into(), sign)),
            );
            let per_peel: BTreeSet<_> = expected
                .iter()
                .filter(|tuple| tuple.contains(&(id, sign)))
                .cloned()
                .collect();
            target_work += root_records(case, &h, system, &prefix, target, remaining, &per_peel)?;
            if !remaining.inf {
                finite += 1;
            }
            per_system_union.extend(per_peel);
        }
        require(per_system_union == expected, "target-peel-union")?;
        require(
            ref_trace[qi]["finite_signed_peels"] == finite,
            "target-finite-system-count",
        )?;
        target_systems += finite;
        for tuple in &got {
            require(
                tuple.len() == 4 && physical(case, tuple, target),
                "target-point-sum",
            )?;
            target_set.insert(tuple.clone());
            let row = case.row(tuple, item, arm);
            if let Some(key) = canonical_row(row.clone(), case.n) {
                target_projective.insert(key);
            }
            rows.push(row);
        }
        rank = independent_rref(&rows, cols, case.n).0;
        require(query["rank_after"] == rank, "target-rank")?;
        if rank == cols {
            first_full = Some(qi);
        }
    }
    require(rank == cols || queries.len() == cap, "early-target-stop")?;
    require(
        d["first_full_rank_query"] == json!(first_full) && d["rank"] == rank,
        "target-stop",
    )?;
    require(
        terms(&d["target_signed_physical_set"])? == target_set
            && d["target_projective_rows"] == json!(target_projective),
        "target-row-counts",
    )?;
    require(
        d["target_f4_row_reduction_fp_muls"] == target_work,
        "target-f4-work-sum",
    )?;
    let solution = independent_rref(&rows, cols, case.n).1;
    require(
        d["solved_logs"] == json!(solution)
            && d["recovered"] == json!(solution.as_ref().map(|v| v[cols - 1])),
        "target-solution",
    )?;
    let verified = solution
        .as_ref()
        .map(|logs| case.verify_solution(logs, arm));
    require(
        verified != Some(false)
            && d["all_usable_factor_logs_and_target_verified"] == json!(verified),
        "target-point-verification",
    )?;
    require(
        d["status"] == if rank == cols { "pass" } else { "censored" },
        "pipeline-status",
    )?;
    let wall = &value["diagnostic_wall_ns"];
    let phase_ns = wall["phases"].as_object().ok_or("phase-clock-format")?;
    let sum = PHASES
        .iter()
        .map(|name| number(&phase_ns[*name]))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<u64>();
    require(
        number(&wall["cold_including_journals"])? == sum
            && number(&wall["reusable_with_zero_journals"])?
                + number(&wall["online_with_target_journals"])?
                == sum
            && wall["journal"] == wall["phases"]["journal"],
        "phase-clock-sum",
    )?;
    Ok(
        json!({"zero_systems_checked":zero_systems.len(),"target_queries_checked":queries.len(),
        "target_finite_systems_checked":target_systems,"zero_factor_rank":d["zero_factor_log_rank"],
        "rank":rank,"columns":cols,"first_full_factor_system":first_factor,
        "first_full_rank_query":first_full,"factor_points_verified":zero_verified,
        "target_and_factor_points_verified":verified}),
    )
}

pub(super) fn audit(
    public: &Path,
    sha: &str,
    candidate: &Path,
    zero_path: &Path,
    zero_sha: &str,
    target_path: &Path,
    target_sha: &str,
    out: &Path,
) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(checked_public(public, sha), out);
    let bytes = std::fs::read(candidate.join("record.json")).unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    let target_bytes = std::fs::read(target_path).unwrap();
    assert_eq!(hex::encode(hash::sha256::sha256(&target_bytes)), target_sha);
    let target: Value = serde_json::from_slice(&target_bytes).unwrap();
    let zero_bytes = if value["deterministic"]["mode"] == "zero" {
        let data = std::fs::read(zero_path).unwrap();
        assert_eq!(hex::encode(hash::sha256::sha256(&data)), zero_sha);
        Some(data)
    } else {
        None
    };
    let zero: Option<Value> = zero_bytes
        .as_ref()
        .map(|v| serde_json::from_slice(v).unwrap());
    let result = inspect(&case, sha, &value, zero.as_ref(), &target);
    let mut controls = Vec::new();
    if result.is_ok() {
        let mut changed = value.clone();
        changed["deterministic"]["public_input_sha256"] = json!("corrupted");
        require(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err() == "public-seal",
            "control",
        )
        .unwrap();
        controls.push("public-binding");
        let mut changed = value.clone();
        changed["deterministic"]["target_queries"][0]["systems"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert_eq!(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
            "target-system-count"
        );
        controls.push("omitted-target-system");
        let mut changed = value.clone();
        changed["deterministic"]["target_queries"][0]["systems"][1]["context"] =
            changed["deterministic"]["target_queries"][0]["systems"][0]["context"].clone();
        assert_eq!(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
            "target-peel-order"
        );
        controls.push("duplicated-target-peel");
        let mut changed = value.clone();
        let count = number(&changed["deterministic"]["target_f4_row_reduction_fp_muls"]).unwrap();
        changed["deterministic"]["target_f4_row_reduction_fp_muls"] = json!(count + 1);
        assert_eq!(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
            "target-f4-work-sum"
        );
        controls.push("altered-target-work-sum");
        let mut changed = value.clone();
        changed["deterministic"]["all_usable_factor_logs_and_target_verified"] = json!(false);
        assert_eq!(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
            "target-point-verification"
        );
        controls.push("forged-point-verification");
        let mut changed = value.clone();
        changed["deterministic"]["first_full_rank_query"] = json!(usize::MAX);
        assert_eq!(
            inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
            "target-stop"
        );
        controls.push("wrong-target-stop");
        let nonempty = value["deterministic"]["target_queries"]
            .as_array()
            .unwrap()
            .iter()
            .position(|q| {
                q["signed_relations"]
                    .as_array()
                    .is_some_and(|v| !v.is_empty())
            });
        if let Some(qi) = nonempty {
            let mut changed = value.clone();
            changed["deterministic"]["target_queries"][qi]["signed_relations"]
                .as_array_mut()
                .unwrap()
                .pop();
            assert_eq!(
                inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
                "target-physical-set"
            );
            controls.push("omitted-target-tuple");
            let mut changed = value.clone();
            let s = changed["deterministic"]["target_queries"][qi]["signed_relations"][0][0][1]
                .as_i64()
                .unwrap();
            changed["deterministic"]["target_queries"][qi]["signed_relations"][0][0][1] = json!(-s);
            assert_eq!(
                inspect(&case, sha, &changed, zero.as_ref(), &target).unwrap_err(),
                "target-physical-set"
            );
            controls.push("altered-target-sign");
        }
        if zero.is_some() {
            let mut changed = value.clone();
            changed["deterministic"]["zero_systems"]
                .as_array_mut()
                .unwrap()
                .pop();
            assert!(inspect(&case, sha, &changed, zero.as_ref(), &target).is_err());
            controls.push("omitted-zero-system");
        }
    }
    let pass = result.is_ok();
    write(
        &out.join("summary.json"),
        &json!({"schema":"twist-zero-pipeline-independent-audit/v1",
        "status":if pass{"pass"}else{"fail"},"public_input_sha256":sha,
        "candidate_sha256":hex::encode(hash::sha256::sha256(&bytes)),
        "zero_reference_sha256":zero_bytes.as_ref().map(|v|hex::encode(hash::sha256::sha256(v))),
        "target_reference_sha256":target_sha,"independent_checks":result.as_ref().ok(),
        "failure":result.as_ref().err(),"negative_controls":controls,
        "scope":"Every processed system/pair/peel and physical set, scalar-map transport, separate RREF and exact stopping prefix are checked against sealed prior complete sets. Target/factor points are verified without a known scalar. Shared field/curve and equation arithmetic; no BSGS or old census rerun. Elapsed clocks are not admitted and scoped counters are not a complete GAE price."}),
    );
    pass
}
