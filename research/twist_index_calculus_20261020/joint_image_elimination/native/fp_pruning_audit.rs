//! Counterfactual S4 rejection audit only, never a candidate or cost measurement.
//! A deployment additionally needs the square certificate and structural proof.
use super::*;

fn proposed_rejection(point: Point) -> bool {
    !point.inf && point.x[1..].iter().all(|&x| x == 0)
}
fn read(path: &Path) -> (Vec<u8>, Value) {
    let bytes = std::fs::read(path).unwrap();
    let value = serde_json::from_slice(&bytes).unwrap();
    (bytes, value)
}
pub(super) fn audit(
    public: &Path,
    public_sha: &str,
    workload: &Path,
    refs: &Path,
    double_refs: &Path,
    out: &Path,
) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let raw = checked_public(public, public_sha);
    let case = Case::new(raw, out);
    let (workload_bytes, workload) = read(workload);
    assert_eq!(workload["schema"], "joint-image-workload/v1");
    let tasks = workload["tasks"].as_array().unwrap();
    let mut arms = Vec::new();
    let mut all_pass = true;
    for arm in [7usize, 9, 8] {
        let root = if arm == 9 { double_refs } else { refs };
        let reference = root.join(format!("p{}-seed{}/{}.json", case.p, case.seed, ARMS[arm]));
        let (bytes, oracle) = read(&reference);
        let expected: BTreeSet<_> = tasks
            .iter()
            .filter(|t| t["p"] == case.p && t["seed"] == case.seed && t["arm"] == ARMS[arm])
            .map(|t| t["reference_sha256"].as_str().unwrap())
            .collect();
        assert_eq!(expected.len(), 1);
        let reference_sha = hex::encode(hash::sha256::sha256(&bytes));
        assert!(
            expected.contains(reference_sha.as_str()),
            "reference differs from frozen construction workload"
        );
        assert_eq!(oracle["p"], case.p);
        assert_eq!(oracle["seed"], case.seed);
        assert_eq!(oracle["arm"], ARMS[arm]);
        let trace = oracle["trace"].as_array().unwrap();
        assert_eq!(
            trace.len(),
            case.raw["query_trace"].as_array().unwrap().len()
        );
        let allowed: BTreeSet<_> = case.peels[arm].iter().copied().collect();
        let cols = case.columns(arm);
        let mut tested = 0u64;
        let mut finite = 0u64;
        let mut infinite = 0u64;
        let mut skips = 0u64;
        let mut false_rejections = Vec::new();
        let mut query_records = Vec::new();
        let mut all_rows = Vec::new();
        let mut retained_rows = Vec::new();
        let mut all_physical = BTreeSet::new();
        let mut retained_physical = BTreeSet::new();
        let mut first_rank = None;
        let mut first_retained_rank = None;
        let mut prefix = None;
        for (qi, record) in trace.iter().enumerate() {
            assert_eq!(record["query_index"], qi);
            let query = &case.raw["query_trace"][qi];
            assert_eq!(record["a"], query[0]);
            assert_eq!(record["b"], query[1]);
            let residual = case.residual(query);
            let relations: BTreeSet<Terms> =
                serde_json::from_value(record["signed_relations"].clone()).unwrap();
            let mut pruned = BTreeSet::new();
            let mut qfinite = 0u64;
            let mut qinf = 0u64;
            for &id in &case.peels[arm] {
                for sign in [-1, 1] {
                    tested += 1;
                    let peeled = case.c.add(
                        residual,
                        case.c.neg(signed(&case.c, case.pool[id].p.into(), sign)),
                    );
                    if peeled.inf {
                        infinite += 1;
                        qinf += 1;
                    } else {
                        finite += 1;
                        qfinite += 1;
                    }
                    if proposed_rejection(peeled) {
                        skips += 1;
                        pruned.insert((id, sign));
                    }
                }
            }
            let mut added = false;
            let mut retained_added = false;
            for terms in &relations {
                assert_eq!(terms.len(), 4);
                assert_eq!(terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len(), 4);
                case.verify_terms(terms, residual);
                let presentations: Vec<_> = terms
                    .iter()
                    .filter(|t| allowed.contains(&t.0))
                    .copied()
                    .collect();
                assert!(!presentations.is_empty());
                for &(id, sign) in &presentations {
                    if pruned.contains(&(id, sign)) {
                        false_rejections.push(json!({"query_index":qi,"peel_id":id,"sign":sign,"physical_relation":terms}));
                    }
                }
                let kept = presentations.iter().any(|t| !pruned.contains(t));
                let row = case.row(terms, query, arm);
                all_physical.insert(terms.clone());
                all_rows.push(row.clone());
                added = true;
                if kept {
                    retained_physical.insert(terms.clone());
                    retained_rows.push(row);
                    retained_added = true;
                }
            }
            if first_rank.is_none() && added && all_rows.len() >= cols {
                if independent_rref(&all_rows, cols, case.n).0 == cols {
                    first_rank = Some(qi);
                    prefix = Some(
                        json!({"query_index":qi,"queries_consumed":qi+1,"tested_signed_peels":tested,
                        "finite_signed_peels":finite,"infinite_peels_retained":infinite,
                        "proposed_finite_rejections":skips,"projected_remaining_finite_peels":finite-skips}),
                    );
                }
            }
            if first_retained_rank.is_none()
                && retained_added
                && retained_rows.len() >= cols
                && independent_rref(&retained_rows, cols, case.n).0 == cols
            {
                first_retained_rank = Some(qi);
            }
            query_records.push(json!({"query_index":qi,"finite_signed_peels":qfinite,"infinite_peels_retained":qinf,
                "proposed_rejections":pruned.len(),"physical_relations":relations.len()}));
        }
        let (rank, solution) = independent_rref(&all_rows, cols, case.n);
        let (kept_rank, kept_solution) = independent_rref(&retained_rows, cols, case.n);
        let original_verified = solution
            .as_ref()
            .map(|s| case.verify_solution(s, arm))
            .unwrap_or(false);
        let retained_verified = kept_solution
            .as_ref()
            .map(|s| case.verify_solution(s, arm))
            .unwrap_or(false);
        let unique: BTreeSet<_> = all_rows
            .iter()
            .filter_map(|r| canonical_row(r.clone(), case.n))
            .collect();
        let kept_unique: BTreeSet<_> = retained_rows
            .iter()
            .filter_map(|r| canonical_row(r.clone(), case.n))
            .collect();
        let pass = false_rejections.is_empty()
            && all_physical == retained_physical
            && rank == cols
            && kept_rank == rank
            && first_rank == first_retained_rank
            && original_verified
            && retained_verified
            && unique == kept_unique;
        all_pass &= pass;
        arms.push(json!({"arm":ARMS[arm],"status":if pass {"hypothesis-audit-pass"}else{"partial"},
            "reference_sha256":reference_sha,"queries":trace.len(),"tested_signed_peels":tested,
            "finite_signed_peels":finite,"infinite_peels_retained":infinite,"proposed_finite_rejections":skips,
            "projected_remaining_finite_peels":finite-skips,"false_rejected_presentations":false_rejections,
            "original_distinct_physical_tuples":all_physical.len(),"retained_distinct_physical_tuples":retained_physical.len(),
            "original_projective_rows":unique.len(),"retained_projective_rows":kept_unique.len(),
            "original_rank":rank,"retained_rank":kept_rank,"columns":cols,
            "original_first_full_rank_query":first_rank,"retained_first_full_rank_query":first_retained_rank,
            "original_recovery_verified":original_verified,"retained_recovery_verified":retained_verified,
            "through_original_full_rank":prefix,"query_records":query_records}));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-fp-pruning-audit/v1","p":case.p,"seed":case.seed,
        "public_input_sha256":public_sha,"workload_sha256":hex::encode(hash::sha256::sha256(&workload_bytes)),
        "status":if all_pass {"hypothesis-audit-pass"}else{"partial"},"arms":arms,
        "predicate":"finite peeled S4 residual whose abscissa has only Fp component; infinity is retained",
        "scope":"Counterfactual audit against retained, independently verified group-oracle sets. No BSGS or polynomial collector is repeated, no prefilter deployed, no elapsed speedup or complete price claimed. Square ideal certificate and structural proof are separate prerequisites."}),
    );
    all_pass
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_cm_triangle_with_distinct_peel_requires_retaining_infinity() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let scratch =
            std::env::temp_dir().join(format!("fp-prune-infinity-{}", std::process::id()));
        assert!(!scratch.exists());
        std::fs::create_dir(&scratch).unwrap();
        let case = Case::new(public_record(&load(&tape, INPUT_SHA, 37, 0)), &scratch);
        let p: Point = case.pool[0].p.into();
        let w = case.raw["omega"].as_u64().unwrap();
        let p1 = psi(&case.c, p, w);
        let p2 = psi(&case.c, p1, w);
        assert!(p.x != p1.x && p.x != p2.x && p1.x != p2.x);
        for point in [p, p1, p2] {
            assert!(case.pool[..case.l0].iter().any(|v| v.p.x == point.x));
        }
        assert_eq!(case.c.add(case.c.add(p, p1), p2), INF);
        let peel: Point = case.pool[..case.l0]
            .iter()
            .map(|v| Point::from(v.p))
            .find(|q| q.x != p.x && q.x != p1.x && q.x != p2.x)
            .unwrap();
        assert_eq!(case.c.add(case.c.add(case.c.add(p, p1), p2), peel), peel);
        let residual = case.c.add(peel, case.c.neg(peel));
        assert!(residual.inf);
        assert!(
            !proposed_rejection(residual),
            "a real distinct-factor CM presentation was pruned"
        );
        std::fs::remove_dir_all(scratch).unwrap();
    }
}
