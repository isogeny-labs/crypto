use super::super::filter_large_primes;
use super::*;

fn row(id: u64, small: &[(u32, u64)], large: &[(u64, u64)], rhs: u64) -> PartialRelation {
    PartialRelation {
        id,
        small: small.to_vec(),
        large: large.to_vec(),
        rhs,
    }
}
fn options() -> LargePrimeFilterOptions {
    LargePrimeFilterOptions {
        modulus: 101,
        ..Default::default()
    }
}

#[test]
fn upper_modulus_bound_and_maximum_raw_words_cancel_exactly() {
    // Legacy admission permits composite odd moduli. Unit pivots make this
    // manually checked cancellation valid without a primality claim.
    let modulus = (1u64 << 63) - 1;
    let opts = LargePrimeFilterOptions {
        modulus,
        ..options()
    };
    let rows = vec![
        row(
            u64::MAX,
            &[(u32::MAX, u64::MAX)],
            &[(u64::MAX, 1)],
            u64::MAX,
        ),
        row(
            u64::MAX - 1,
            &[(u32::MAX - 1, u64::MAX)],
            &[(u64::MAX, u64::MAX)],
            u64::MAX - 1,
        ),
    ];
    let result = run(&rows, &opts, Default::default());
    assert_eq!(result.stop, FilterStop::Completed);
    assert_eq!(
        result.rows,
        vec![CompleteRelation {
            small: vec![(u32::MAX - 1, modulus - 1), (u32::MAX, 1)],
            rhs: 1,
            sources: vec![(u64::MAX - 1, modulus - 1), (u64::MAX, 1)],
        }]
    );
    assert_eq!(
        (result.rows, result.report),
        filter_large_primes(rows, &opts).unwrap()
    );
}
fn run(rows: &[PartialRelation], o: &LargePrimeFilterOptions, l: FilterTraceLimits) -> FilterRun {
    let result = filter_large_primes_with_trace(rows.to_vec(), o, l, &mut |_| Ok(())).unwrap();
    verify_filter_trace(rows, o, &result).unwrap();
    result
}
fn fixture() -> Vec<PartialRelation> {
    vec![
        row(0, &[(0, 1)], &[(10, 2)], 3),
        row(1, &[(1, 3)], &[(10, 5), (11, 1)], 4),
        row(2, &[(2, 1)], &[(11, 2)], 5),
        row(3, &[(1, 1)], &[(12, 1), (13, 2)], 9),
        row(4, &[(2, 1)], &[(13, 1)], 6),
        row(5, &[(0, 5)], &[], 7),
    ]
}
#[test]
fn legacy_outputs_reports_and_source_combinations_match() {
    let o = options();
    let cases = vec![
        vec![],
        fixture(),
        vec![
            row(1, &[(0, 1), (0, 100), (2, 205)], &[], 202),
            row(2, &[(2, 3)], &[], 0),
        ],
        vec![row(1, &[], &[(9, 1)], 0), row(2, &[], &[(9, 2)], 0)],
        vec![
            row(4, &[(0, 2)], &[(1, 1), (2, 1)], 3),
            row(5, &[(1, 3)], &[(2, 2), (3, 2)], 4),
            row(6, &[(2, 4)], &[(1, 3), (3, 3)], 5),
        ],
    ];
    for rows in cases {
        for max_sources in [0, 1, 2, 4096] {
            for max_small_weight in [0, 1, 4096] {
                let opts = LargePrimeFilterOptions {
                    max_sources,
                    max_small_weight,
                    ..o
                };
                let legacy = filter_large_primes(rows.clone(), &opts).unwrap();
                let traced = run(&rows, &opts, FilterTraceLimits::default());
                assert_eq!(traced.stop, FilterStop::Completed);
                assert_eq!((traced.rows.clone(), traced.report.clone()), legacy);
                for complete in &traced.rows {
                    // A separately expressed dense source combination, not the
                    // producer's replay helper or the transition checker.
                    let mut small = BTreeMap::new();
                    let mut large = BTreeMap::new();
                    let mut rhs = 0u128;
                    for &(id, c) in &complete.sources {
                        let src = rows.iter().find(|r| r.id == id).unwrap();
                        for &(k, v) in &src.small {
                            *small.entry(k).or_insert(0u128) += u128::from(c) * u128::from(v);
                        }
                        for &(k, v) in &src.large {
                            *large.entry(k).or_insert(0u128) += u128::from(c) * u128::from(v);
                        }
                        rhs += u128::from(c) * u128::from(src.rhs);
                    }
                    let small: Vec<_> = small
                        .into_iter()
                        .filter_map(|(k, v)| {
                            let v = (v % 101) as u64;
                            (v != 0).then_some((k, v))
                        })
                        .collect();
                    assert_eq!(small, complete.small);
                    assert_eq!((rhs % 101) as u64, complete.rhs);
                    assert!(large.values().all(|v| v % 101 == 0));
                }
            }
        }
    }
}

#[test]
fn every_event_and_sink_boundary_is_a_verified_prefix() {
    let mut rows = fixture();
    let mut duplicate = rows[0].clone();
    duplicate.id = u64::MAX;
    rows.push(duplicate);
    let o = options();
    let full = run(&rows, &o, FilterTraceLimits::default());
    for count in 0..full.events.len() {
        let limited = run(
            &rows,
            &o,
            FilterTraceLimits {
                events: count as u64,
                ..Default::default()
            },
        );
        assert_eq!(limited.stop, FilterStop::EventLimit);
        assert_eq!(limited.events.len(), count);
        assert!(!limited.report_final);
        let mut calls = 0;
        let failed = filter_large_primes_with_trace(
            rows.clone(),
            &o,
            FilterTraceLimits::default(),
            &mut |_| {
                let n = calls;
                calls += 1;
                if n == count {
                    Err("injected durable sink failure".into())
                } else {
                    Ok(())
                }
            },
        )
        .unwrap();
        assert_eq!(calls, count + 1);
        assert_eq!(failed.events.len(), count);
        assert_eq!(failed.stop, FilterStop::SinkFailure);
        assert!(!failed.report_final);
        assert!(failed.unacknowledged.is_some());
        verify_filter_trace(&rows, &o, &failed).unwrap();
        assert!(failed.rows.len() <= full.rows.len());
    }
    let failed_finish =
        filter_large_primes_with_trace(rows.clone(), &o, FilterTraceLimits::default(), &mut |e| {
            if matches!(e.transition, Transition::Finish { .. }) {
                Err("finish refused".into())
            } else {
                Ok(())
            }
        })
        .unwrap();
    assert_eq!(failed_finish.rows, full.rows);
    assert!(!failed_finish.report_final);
    verify_filter_trace(&rows, &o, &failed_finish).unwrap();
}

#[test]
fn refused_fill_decision_cannot_commit_or_replace_a_pivot() {
    let rows = fixture();
    let opts = LargePrimeFilterOptions {
        max_small_weight: 1,
        ..options()
    };
    let full = run(&rows, &opts, Default::default());
    let index = full
        .events
        .iter()
        .position(|event| {
            matches!(
                event.transition,
                Transition::Reduction {
                    accepted: false,
                    ..
                }
            )
        })
        .expect("fixture must reject fill");
    let mut calls = 0;
    let failed =
        filter_large_primes_with_trace(rows.clone(), &opts, Default::default(), &mut |_| {
            let current = calls;
            calls += 1;
            if current == index {
                Err("refused fill decision".into())
            } else {
                Ok(())
            }
        })
        .unwrap();
    assert_eq!(calls, index + 1);
    assert_eq!(failed.stop, FilterStop::SinkFailure);
    assert_eq!(failed.report, full.events[index - 1].report_after);
    assert_eq!(
        failed.report.rejected_fill + 1,
        full.events[index].report_after.rejected_fill
    );
    verify_filter_trace(&rows, &opts, &failed).unwrap();
    let mut forged = failed;
    forged.report = full.events[index].report_after.clone();
    assert!(verify_filter_trace(&rows, &opts, &forged).is_err());
}

#[test]
fn exact_byte_boundary_and_all_work_reservations_are_honest() {
    let rows = fixture();
    let o = options();
    let full = run(&rows, &o, FilterTraceLimits::default());
    let mut exact = full.acknowledged_bytes;
    for _ in 0..4 {
        let mut events = full.events.clone();
        let Transition::Begin { limits, .. } = &mut events[0].transition else {
            panic!()
        };
        limits.bytes = exact;
        exact = events
            .iter()
            .map(|e| serde_json::to_vec(e).unwrap().len() as u64)
            .sum();
    }
    for cap in [exact - 1, exact, exact + 1] {
        let result = run(
            &rows,
            &o,
            FilterTraceLimits {
                bytes: cap,
                ..Default::default()
            },
        );
        assert_eq!(
            result.stop,
            if cap < exact {
                FilterStop::ByteLimit
            } else {
                FilterStop::Completed
            }
        );
        assert!(result.acknowledged_bytes <= cap);
    }
    for cap in 0..=full.computed_work {
        let result = run(
            &rows,
            &o,
            FilterTraceLimits {
                work: cap,
                ..Default::default()
            },
        );
        assert!(result.computed_work <= cap);
        assert_eq!(
            result.stop,
            if cap == full.computed_work {
                FilterStop::Completed
            } else {
                FilterStop::WorkLimit
            }
        );
    }
}

#[test]
fn input_state_limits_and_invalid_native_inputs_preserve_claim_boundaries() {
    let rows = fixture();
    let o = options();
    for limits in [
        FilterTraceLimits {
            input_rows: 0,
            ..Default::default()
        },
        FilterTraceLimits {
            input_terms: 0,
            ..Default::default()
        },
    ] {
        let result = run(&rows, &o, limits);
        assert_eq!(result.stop, FilterStop::InputLimit);
        assert!(result.events.is_empty() && result.input_sha256.is_none());
    }
    let state = run(
        &rows,
        &o,
        FilterTraceLimits {
            row_terms: 0,
            ..Default::default()
        },
    );
    assert_eq!(state.stop, FilterStop::StateLimit);
    assert_eq!(state.events.len(), 1);
    let invalid = FilterTraceLimits {
        events: 100_001,
        ..Default::default()
    };
    assert_eq!(
        filter_large_primes_with_trace(vec![], &o, invalid, &mut |_| Ok(())).unwrap_err(),
        FilterTraceError::InvalidLimits
    );
    for modulus in [0, 1, 2, 4, 1u64 << 63] {
        let opts = LargePrimeFilterOptions { modulus, ..o };
        assert_eq!(
            filter_large_primes_with_trace(
                rows.clone(),
                &opts,
                Default::default(),
                &mut |_| Ok(())
            )
            .unwrap_err(),
            FilterTraceError::Native(filter_large_primes(rows.clone(), &opts).unwrap_err())
        );
    }
    let duplicated = vec![rows[0].clone(), rows[0].clone()];
    assert_eq!(
        filter_large_primes_with_trace(duplicated.clone(), &o, Default::default(), &mut |_| Ok(()))
            .unwrap_err(),
        FilterTraceError::Native(filter_large_primes(duplicated, &o).unwrap_err())
    );
    let opts = LargePrimeFilterOptions { modulus: 9, ..o };
    let rows = vec![
        row(0, &[(0, 1)], &[(1, 3)], 0),
        row(1, &[(1, 1)], &[(1, 3)], 0),
    ];
    let mut acknowledged = Vec::new();
    assert_eq!(
        filter_large_primes_with_trace(rows.clone(), &opts, Default::default(), &mut |e| {
            acknowledged.push(e.clone());
            Ok(())
        })
        .unwrap_err(),
        FilterTraceError::Native(filter_large_primes(rows, &opts).unwrap_err())
    );
    assert!(!acknowledged.is_empty());
    assert!(!matches!(
        acknowledged.last().unwrap().transition,
        Transition::Finish { .. }
    ));
}

#[test]
fn altered_bindings_transitions_and_counters_are_rejected() {
    let rows = fixture();
    let o = options();
    let full = run(&rows, &o, Default::default());
    let json = serde_json::to_vec(&full).unwrap();
    let decoded: FilterRun = serde_json::from_slice(&json).unwrap();
    assert_eq!(decoded, full);
    verify_filter_trace(&rows, &o, &decoded).unwrap();
    for index in 0..full.events.len() {
        let mut bad = full.clone();
        bad.events.remove(index);
        assert!(verify_filter_trace(&rows, &o, &bad).is_err());
        let mut bad = full.clone();
        bad.events[index].report_after.merges += 1;
        assert!(verify_filter_trace(&rows, &o, &bad).is_err());
        let mut bad = full.clone();
        bad.events[index].reservation.work += 1;
        assert!(verify_filter_trace(&rows, &o, &bad).is_err());
    }
    for mutate in 0..11 {
        let mut bad = full.clone();
        match mutate {
            0 => bad.options.modulus = 103,
            1 => bad.input_sha256 = Some("00".into()),
            2 => bad.computed_work += 1,
            3 => bad.acknowledged_bytes += 1,
            4 => bad.report_final = false,
            5 => bad.execution_trace_complete_verified = true,
            6 => bad.stop = FilterStop::WorkLimit,
            7 => bad.rows[0].rhs ^= 1,
            8 => bad.total_operation_count_verified = true,
            9 => bad.dependency_basis_complete_verified = true,
            _ => bad.sink_error = Some("invented".into()),
        }
        assert!(
            verify_filter_trace(&rows, &o, &bad).is_err(),
            "mutation {mutate}"
        );
    }
    for index in 0..full.events.len() {
        let mut bad = full.clone();
        match &mut bad.events[index].transition {
            Transition::Peel { degrees_after, .. } => degrees_after[0].1 += 1,
            Transition::CoreReady { row_ids, .. } => row_ids.reverse(),
            Transition::PivotSelected { score, .. } => *score += 1,
            Transition::Reduction { candidate, .. } => candidate.sources[0].1 ^= 1,
            Transition::PivotStored { row, .. } => row.rhs ^= 1,
            Transition::CompleteRow { row, .. } => row.rhs ^= 1,
            _ => continue,
        }
        if bad != full {
            assert!(verify_filter_trace(&rows, &o, &bad).is_err());
        }
    }
    let mut reordered = rows.clone();
    reordered.swap(0, 1);
    assert!(verify_filter_trace(&reordered, &o, &full).is_err());
}

// Independent dense elimination with exhaustive inverse search over tiny prime
// fields; it does not share producer/checker sparse arithmetic or scheduling.
fn dense(mut rows: Vec<Vec<u64>>, columns: usize, r: u64) -> Vec<Vec<u64>> {
    let mut next = 0;
    for col in 0..columns {
        let Some(index) = (next..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(next, index);
        let inv = (1..r).find(|v| v * rows[next][col] % r == 1).unwrap();
        for value in &mut rows[next] {
            *value = *value * inv % r;
        }
        for i in 0..rows.len() {
            if i != next {
                let scale = rows[i][col];
                for j in 0..rows[i].len() {
                    rows[i][j] = (rows[i][j] + r - scale * rows[next][j] % r) % r;
                }
            }
        }
        next += 1;
    }
    rows
}
fn rank(rows: Vec<Vec<u64>>, r: u64) -> usize {
    dense(rows, 2, r)
        .into_iter()
        .filter(|row| row.iter().any(|v| *v != 0))
        .count()
}
#[test]
fn exhaustive_small_projected_row_spaces_match_dense_reference() {
    // All 729 two-row systems with one large, one small and one RHS column
    // over F3, plus deterministic 3-row/two-large systems over F5 and F7.
    for r in [3u64, 5, 7] {
        let count = if r == 3 { 729 } else { 100 };
        for code in 0..count {
            let mut state = code as u64;
            let mut raw = Vec::new();
            let mut rows = Vec::new();
            let number = if r == 3 { 2 } else { 3 };
            let large = if r == 3 { 1 } else { 2 };
            for id in 0..number {
                let mut values = Vec::new();
                for _ in 0..large + 2 {
                    let v = if r == 3 {
                        let v = state % r;
                        state /= r;
                        v
                    } else {
                        state = state
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        state % r
                    };
                    values.push(v);
                }
                rows.push(row(
                    id,
                    &[(0, values[large])],
                    &(0..large)
                        .map(|i| (i as u64, values[i]))
                        .collect::<Vec<_>>(),
                    values[large + 1],
                ));
                raw.push(values);
            }
            let opts = LargePrimeFilterOptions {
                modulus: r,
                ..options()
            };
            let result = run(&rows, &opts, Default::default());
            assert_eq!(
                (result.rows.clone(), result.report.clone()),
                filter_large_primes(rows.clone(), &opts).unwrap()
            );
            let reference: Vec<Vec<u64>> = dense(raw, large, r)
                .into_iter()
                .filter(|v| v[..large].iter().all(|x| *x == 0))
                .map(|v| v[large..].to_vec())
                .collect();
            let actual: Vec<Vec<u64>> = result
                .rows
                .iter()
                .map(|row| vec![row.small.first().map_or(0, |t| t.1), row.rhs])
                .collect();
            assert_eq!(rank(reference.clone(), r), rank(actual.clone(), r));
            let mut joined = actual.clone();
            joined.extend(reference);
            assert_eq!(rank(joined, r), rank(actual, r));
        }
    }
}
