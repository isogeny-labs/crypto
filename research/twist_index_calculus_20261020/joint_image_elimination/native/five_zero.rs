//! Oracle-only higher-arity diagnostic. Never called by a polynomial candidate.
use super::*;
use crate::fx_hash::FxMap;

fn addn(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + b as u128) % n as u128) as u64
}
fn signed_log(logs: &[u64], id: usize, sign: i8, n: u64) -> u64 {
    if sign == 1 {
        logs[id]
    } else {
        negn(logs[id], n)
    }
}
fn pairs(logs: &[u64], l0: usize, n: u64) -> FxMap<u64, Vec<[(usize, i8); 2]>> {
    let mut table = FxMap::<u64, Vec<[(usize, i8); 2]>>::default();
    for j in 1..l0 {
        for i in 0..j {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let key = addn(signed_log(logs, i, si, n), signed_log(logs, j, sj, n), n);
                    table.entry(key).or_default().push([(i, si), (j, sj)]);
                }
            }
        }
    }
    table
}
fn join(
    logs: &[u64],
    l0: usize,
    images: Option<&[usize]>,
    n: u64,
    table: &FxMap<u64, Vec<[(usize, i8); 2]>>,
) -> (BTreeMap<Terms, u64>, u64) {
    let mut result = BTreeMap::<Terms, u64>::new();
    let mut queries = 0u64;
    let mut visit = |triple: [(usize, i8); 3]| {
        queries += 1;
        let sum = triple.iter().fold(0, |sum, &(id, sign)| {
            addn(sum, signed_log(logs, id, sign, n), n)
        });
        if let Some(postings) = table.get(&negn(sum, n)) {
            for pair in postings {
                if pair.iter().any(|t| triple.iter().any(|s| s.0 == t.0)) {
                    continue;
                }
                let mut terms = triple.to_vec();
                terms.extend(pair);
                terms.sort();
                assert_eq!(terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len(), 5);
                *result.entry(terms).or_default() += 1;
            }
        }
    };
    if let Some(images) = images {
        for j in 1..l0 {
            for i in 0..j {
                for &k in images {
                    assert!(k >= l0);
                    for si in [-1, 1] {
                        for sj in [-1, 1] {
                            for sk in [-1, 1] {
                                visit([(i, si), (j, sj), (k, sk)]);
                            }
                        }
                    }
                }
            }
        }
    } else {
        for k in 2..l0 {
            for j in 1..k {
                for i in 0..j {
                    for si in [-1, 1] {
                        for sj in [-1, 1] {
                            for sk in [-1, 1] {
                                visit([(i, si), (j, sj), (k, sk)]);
                            }
                        }
                    }
                }
            }
        }
    }
    let expected = if images.is_some() { 6 } else { 10 };
    assert!(result
        .values()
        .all(|&multiplicity| multiplicity == expected));
    (result, queries)
}

pub(super) fn audit(public: &Path, sha: &str, replay: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(checked_public(public, sha), out);
    let replay_bytes = std::fs::read(replay).unwrap();
    let saved: Value = serde_json::from_slice(&replay_bytes).unwrap();
    assert_eq!(saved["schema"], "joint-image-fp-pruning-audit/v1");
    assert_eq!(saved["status"], "hypothesis-audit-pass");
    assert_eq!(saved["p"], case.p);
    assert_eq!(saved["seed"], case.seed);
    assert_eq!(saved["public_input_sha256"], sha);
    let cols = case.columns(8);
    let mut solutions = Vec::new();
    for arm in [8usize, 7, 9] {
        let record = saved["arms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["arm"] == ARMS[arm])
            .unwrap();
        assert_eq!(record["status"], "hypothesis-audit-pass");
        assert_eq!(record["original_recovery_verified"], true);
        let solution: Vec<u64> =
            serde_json::from_value(record["oracle_replay_verified_solution"].clone()).unwrap();
        assert_eq!(solution.len(), cols);
        assert!(case.verify_solution(&solution, arm));
        solutions.push(solution);
    }
    assert!(solutions.iter().all(|s| s == &solutions[0]));
    let mut reps = vec![case.anchor];
    reps.extend_from_slice(&solutions[0][..cols - 1]);
    let mut logs = vec![0; case.pool.len()];
    let usable: BTreeSet<_> = (0..case.l0)
        .chain(case.peels[7].iter().copied())
        .chain(case.peels[9].iter().copied())
        .collect();
    for &id in &usable {
        let factor = &case.pool[id];
        assert!(factor.col < cols);
        logs[id] = mm(reps[factor.col], factor.coef, case.n);
        assert_eq!(case.c.mul(case.g, logs[id]), factor.p.into());
    }
    let table = pairs(&logs, case.l0, case.n);
    let pair_entries: usize = table.values().map(Vec::len).sum();
    let mut arms = Vec::new();
    for arm in [8usize, 7, 9] {
        let images = if arm == 8 {
            None
        } else {
            Some(case.peels[arm].as_slice())
        };
        let (physical, queries) = join(&logs, case.l0, images, case.n, &table);
        let mut rows = Vec::new();
        let mut unique = BTreeSet::new();
        let mut zero = 0usize;
        for terms in physical.keys() {
            assert_eq!(terms.len(), 5);
            assert_eq!(
                terms
                    .iter()
                    .map(|&(id, _)| case.pool[id].p.x)
                    .collect::<BTreeSet<_>>()
                    .len(),
                5
            );
            let sum = terms.iter().fold(INF, |sum, &(id, sign)| {
                case.c
                    .add(sum, signed(&case.c, case.pool[id].p.into(), sign))
            });
            assert_eq!(sum, INF, "false five-factor zero relation");
            let augmented = case.row(terms, &json!([0, 0]), arm);
            assert_eq!(
                augmented[cols - 1],
                0,
                "zero relation acquired target coefficient"
            );
            let mut row = augmented[..cols - 1].to_vec();
            row.push(augmented[cols]);
            if let Some(key) = canonical_row(row.clone(), case.n) {
                unique.insert(key);
            } else {
                zero += 1;
            }
            rows.push(row);
        }
        let (rank, solution) = independent_rref(&rows, cols - 1, case.n);
        let verified = solution
            .as_ref()
            .map(|solution| {
                let mut recovered = vec![case.anchor];
                recovered.extend(solution);
                usable.iter().all(|&id| {
                    let f = &case.pool[id];
                    case.c.mul(case.g, mm(recovered[f.col], f.coef, case.n)) == f.p.into()
                }) && solution == &solutions[0][..cols - 1]
            })
            .unwrap_or(false);
        assert_eq!(verified, rank == cols - 1);
        let signed: Vec<_> = physical.keys().cloned().collect();
        let set_bytes = serde_json::to_vec(&signed).unwrap();
        write(
            &out.join(format!("{}.json", ARMS[arm])),
            &json!({"arm":ARMS[arm],"signed_physical_set":signed,
            "signed_set_sha256":hex::encode(hash::sha256::sha256(&set_bytes)),"signed_physical_relations":physical.len(),
            "presentation_multiplicity":if arm==8 {10}else{6},"presentation_multiplicity_verified":true,
            "triple_queries":queries,"zero_augmented_rows":zero,"unique_projective_augmented_rows":unique.len(),
            "factor_log_rank":rank,"unanchored_factor_columns":cols-1,"factor_log_solution_point_verified":verified,
            "target_column_zero":true,"target_recovery_attempted":false,"status":"complete-oracle-census"}),
        );
        arms.push(json!({"arm":ARMS[arm],"signed_physical_relations":physical.len(),"zero_augmented_rows":zero,
            "unique_projective_augmented_rows":unique.len(),"factor_log_rank":rank,"unanchored_factor_columns":cols-1,
            "factor_log_solution_point_verified":verified,"triple_queries":queries}));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-five-zero-census/v1","status":"complete-oracle-census",
        "p":case.p,"seed":case.seed,"public_input_sha256":sha,
        "historical_replay_sha256":hex::encode(hash::sha256::sha256(&replay_bytes)),"ordinary_abscissae":case.l0,
        "signed_pair_entries":pair_entries,"pair_keys":table.len(),"arms":arms,
        "scope":"Oracle census using old point-verified matrix solutions; no collector or BSGS repeated. All returned tuples verified by point addition, partition counts and independent RREF. Full point-keyed completeness replay remains separate. No polynomial solve, target recovery or timing/price claim."}),
    );
    true
}

pub(super) fn summarize(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let mut records = Vec::new();
    let mut missing = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let id = format!("p{p}-seed{seed}");
            let path = run.join("cases").join(&id).join("summary.json");
            if !path.is_file() {
                missing.push(id);
                continue;
            }
            let bytes = std::fs::read(&path).unwrap();
            let value: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(value["schema"], "joint-image-five-zero-census/v1");
            assert_eq!(value["status"], "complete-oracle-census");
            assert_eq!(value["p"], p);
            assert_eq!(value["seed"], seed);
            records.push(json!({"id":id,"summary_sha256":hex::encode(hash::sha256::sha256(&bytes)),"summary":value}));
        }
    }
    let arms:Vec<_>=[8usize,7,9].iter().map(|&arm| {
        let mut totals=BTreeMap::<&str,u64>::new();let mut ranks=Vec::new();let mut full=0usize;
        for r in &records {
            let a=r["summary"]["arms"].as_array().unwrap().iter().find(|a|a["arm"]==ARMS[arm]).unwrap();
            for key in ["signed_physical_relations","zero_augmented_rows","unique_projective_augmented_rows","triple_queries"] {
                *totals.entry(key).or_default()+=a[key].as_u64().unwrap();
            }
            if a["factor_log_solution_point_verified"]==true {full+=1;}
            ranks.push(json!({"case":r["id"],"rank":a["factor_log_rank"],"columns":a["unanchored_factor_columns"],
                "all_factor_points_verified":a["factor_log_solution_point_verified"]}));
        }
        json!({"arm":ARMS[arm],"case_local_count_sums":totals,"cases_with_all_factor_logs":full,"per_case_rank":ranks})
    }).collect();
    let pass = missing.is_empty() && records.len() == 21;
    write(
        out,
        &json!({"schema":"joint-image-five-zero-summary/v1","status":if pass {"complete-oracle-census"}else{"partial"},
        "completed_cases":records.len(),"missing_cases":missing,"arms":arms,"records":records,
        "scope":"Native sums of case-local counts, with rank kept per case. Oracle relation generation reuses old verified logs; no polynomial or target-recovery price. Full independent point-keyed completeness replay pending."}),
    );
    pass
}

#[cfg(test)]
mod tests {
    use super::*;
    // A different enumeration and integer arithmetic from the pair/triple
    // join. Includes genuine zero tuples and wraparound modulo101.
    fn direct(logs: &[u64], l0: usize, mixed: bool) -> BTreeSet<Terms> {
        let mut subsets = Vec::new();
        for d in 3..l0 {
            for c in 2..d {
                for b in 1..c {
                    for a in 0..b {
                        if mixed {
                            for e in l0..logs.len() {
                                subsets.push(vec![a, b, c, d, e]);
                            }
                        } else {
                            for e in d + 1..l0 {
                                subsets.push(vec![a, b, c, d, e]);
                            }
                        }
                    }
                }
            }
        }
        let mut result = BTreeSet::new();
        for ids in subsets {
            for mask in 0..32 {
                let terms: Terms = ids
                    .iter()
                    .enumerate()
                    .map(|(i, &id)| (id, if mask & (1 << i) == 0 { -1 } else { 1 }))
                    .collect();
                let sum: i128 = terms
                    .iter()
                    .map(|&(id, sign)| logs[id] as i128 * sign as i128)
                    .sum();
                if sum.rem_euclid(101) == 0 {
                    result.insert(terms);
                }
            }
        }
        result
    }
    #[test]
    fn pure_join_matches_direct_five_sum_and_ten_partitions() {
        let logs = [1, 3, 7, 11, 22, 29];
        let expected = direct(&logs, 6, false);
        assert!(!expected.is_empty());
        let (actual, _) = join(&logs, 6, None, 101, &pairs(&logs, 6, 101));
        assert_eq!(actual.keys().cloned().collect::<BTreeSet<_>>(), expected);
        assert!(actual.values().all(|&multiplicity| multiplicity == 10));
    }
    #[test]
    fn mixed_join_matches_direct_five_sum_and_six_partitions() {
        let logs = [1, 3, 7, 11, 22, 29, 44, 48];
        let expected = direct(&logs, 6, true);
        assert!(!expected.is_empty());
        let (actual, _) = join(&logs, 6, Some(&[6, 7]), 101, &pairs(&logs, 6, 101));
        assert_eq!(actual.keys().cloned().collect::<BTreeSet<_>>(), expected);
        assert!(actual.values().all(|&multiplicity| multiplicity == 6));
    }
}
