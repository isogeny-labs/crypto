use super::*;
fn event(phase: Phase, work: usize, state: usize, t: Transition, report: &Report) -> Event {
    Event {
        sequence: 0,
        reservation: Reservation::new(phase, work, state),
        transition: t,
        report_after: report.clone(),
    }
}
fn retotal(run: &mut Run) {
    let mut work = 0;
    let mut inv = 0;
    let mut bytes = 0;
    let mut rows = Vec::new();
    let mut report = Report {
        input_relations: run.input_rows,
        ..Report::default()
    };
    for (i, e) in run.events.iter_mut().enumerate() {
        e.sequence = i as u64;
        if let Transition::Begin { limits, .. } = &mut e.transition {
            *limits = run.limits.clone();
        }
        work += e.reservation.work;
        inv += u64::from(matches!(e.transition, Transition::Reduction { .. }));
        bytes += serde_json::to_vec(e).unwrap().len() as u64;
        report = e.report_after.clone();
        if let Transition::CompleteRow { row, .. } = &e.transition {
            rows.push(row.complete());
        }
    }
    run.acknowledged_work = work;
    run.acknowledged_inverse_calls = inv;
    run.acknowledged_bytes = bytes;
    run.report = report;
    run.rows = rows;
    if let Some(e) = &mut run.unacknowledged {
        e.sequence = run.events.len() as u64;
        if let Transition::Begin { limits, .. } = &mut e.transition {
            *limits = run.limits.clone();
        }
        work += e.reservation.work;
        inv += u64::from(matches!(e.transition, Transition::Reduction { .. }));
        run.unacknowledged_bytes = serde_json::to_vec(e).unwrap().len() as u64;
    } else {
        run.unacknowledged_bytes = 0;
    }
    run.computed_work = work;
    run.inverse_calls = inv;
}
/// Hand-derived equations x0+L=3, x1+L=4 over F7, hence -x0+x1=1.
fn two_rows() -> (Vec<Partial>, Run) {
    let input = vec![
        Partial {
            id: 10,
            small: vec![(0, 1)],
            large: vec![(99, 1)],
            rhs: 3,
        },
        Partial {
            id: 20,
            small: vec![(1, 1)],
            large: vec![(99, 1)],
            rhs: 4,
        },
    ];
    let options = Options {
        modulus: 7,
        max_small_weight: 4096,
        max_sources: 4096,
    };
    let limits = Limits::policy();
    let sha = digest(&serde_json::to_vec(&input).unwrap());
    let mut report = Report {
        input_relations: 2,
        ..Report::default()
    };
    let mut events = vec![event(
        Phase::Begin,
        0,
        0,
        Transition::Begin {
            input_sha256: sha.clone(),
            input_rows: 2,
            input_terms: 4,
            options: options.clone(),
            limits: limits.clone(),
        },
        &report,
    )];
    for (position, row) in input.iter().enumerate() {
        events.push(event(
            Phase::Normalize,
            3,
            4,
            Transition::Normalized {
                position,
                row: row.clone(),
            },
            &report,
        ));
    }
    report.input_large_vertices = 1;
    report.core_relations = 2;
    report.core_large_vertices = 1;
    events.push(event(
        Phase::Core,
        2,
        0,
        Transition::CoreReady {
            row_ids: vec![10, 20],
            smooth_ids: vec![],
            degrees: vec![(99, 2)],
        },
        &report,
    ));
    let first = Row {
        small: vec![(0, 1)],
        large: vec![(99, 1)],
        rhs: 3,
        sources: vec![(10, 1)],
    };
    let second = Row {
        small: vec![(1, 1)],
        large: vec![(99, 1)],
        rhs: 4,
        sources: vec![(20, 1)],
    };
    let pr = RowRef {
        input_id: 10,
        version: 0,
    };
    let sr = RowRef {
        input_id: 20,
        version: 0,
    };
    events.push(event(
        Phase::Row,
        0,
        4,
        Transition::RowBegin {
            reference: pr.clone(),
            row: first.clone(),
        },
        &report,
    ));
    events.push(event(
        Phase::Select,
        1,
        4,
        Transition::PivotSelected {
            reference: pr.clone(),
            column: 99,
            pivot: None,
            score: 0,
        },
        &report,
    ));
    report.pivots = 1;
    events.push(event(
        Phase::Store,
        0,
        4,
        Transition::PivotStored {
            reference: pr.clone(),
            column: 99,
            row: first,
        },
        &report,
    ));
    events.push(event(
        Phase::Row,
        0,
        4,
        Transition::RowBegin {
            reference: sr.clone(),
            row: second,
        },
        &report,
    ));
    events.push(event(
        Phase::Select,
        1,
        4,
        Transition::PivotSelected {
            reference: sr.clone(),
            column: 99,
            pivot: Some(pr.clone()),
            score: 0,
        },
        &report,
    ));
    let result = Row {
        small: vec![(0, 6), (1, 1)],
        large: vec![],
        rhs: 1,
        sources: vec![(10, 6), (20, 1)],
    };
    report.merges = 1;
    events.push(event(
        Phase::Reduce,
        7,
        7,
        Transition::Reduction {
            reference: sr,
            column: 99,
            pivot: pr,
            inverse: 1,
            scale: 6,
            candidate: result.clone(),
            accepted: true,
        },
        &report,
    ));
    report.complete_relations = 1;
    report.cycles_closed = 1;
    events.push(event(
        Phase::Complete,
        0,
        5,
        Transition::CompleteRow {
            reference: RowRef {
                input_id: 20,
                version: 1,
            },
            row: result.clone(),
        },
        &report,
    ));
    events.push(event(
        Phase::Finish,
        0,
        0,
        Transition::Finish {
            rows: vec![result.complete()],
        },
        &report,
    ));
    let mut run = Run {
        schema: "large-prime-filter-trace/v1".into(),
        input_sha256: Some(sha),
        input_rows: 2,
        input_terms: 4,
        options,
        limits,
        stop: Stop::Completed,
        events,
        unacknowledged: None,
        next_reservation: None,
        sink_error: None,
        computed_work: 0,
        acknowledged_work: 0,
        inverse_calls: 0,
        acknowledged_inverse_calls: 0,
        acknowledged_bytes: 0,
        unacknowledged_bytes: 0,
        rows: vec![],
        report: Report::default(),
        report_final: true,
        execution_trace_complete_verified: false,
        total_operation_count_verified: false,
        dependency_basis_complete_verified: false,
        work_scope: WORK_SCOPE.into(),
        byte_scope: BYTE_SCOPE.into(),
    };
    retotal(&mut run);
    (input, run)
}
fn check(input: &[Partial], run: &Run) -> Result<()> {
    filter::verify(input, &run.options, run, &mut Budget::default())
}
#[test]
fn hand_derived_two_source_cancellation() {
    let (i, r) = two_rows();
    check(&i, &r).unwrap();
    assert_eq!(r.rows[0].sources, vec![(10, 6), (20, 1)]);
    assert_eq!(r.computed_work, 17);
}
#[test]
fn every_event_boundary_is_a_valid_censor_including_finish() {
    let (i, full) = two_rows();
    for end in 0..full.events.len() {
        let mut r = full.clone();
        r.limits.events = end as u64;
        r.stop = Stop::EventLimit;
        r.report_final = false;
        r.next_reservation = Some(r.events[end].reservation.clone());
        r.events.truncate(end);
        retotal(&mut r);
        check(&i, &r).unwrap();
        r.report_final = true;
        assert!(check(&i, &r).is_err());
    }
}
#[test]
fn admission_priority_and_computed_unacknowledged_bytes() {
    let (i, full) = two_rows();
    let mut r = full.clone();
    r.limits.row_terms = 0;
    r.limits.events = 1;
    r.limits.work = 0;
    r.stop = Stop::StateLimit;
    r.report_final = false;
    r.next_reservation = Some(r.events[1].reservation.clone());
    r.events.truncate(1);
    retotal(&mut r);
    check(&i, &r).unwrap();
    r.stop = Stop::EventLimit;
    assert!(check(&i, &r).is_err());
    let mut r = full.clone();
    r.limits.work = 0;
    r.stop = Stop::WorkLimit;
    r.report_final = false;
    r.next_reservation = Some(r.events[1].reservation.clone());
    r.events.truncate(1);
    retotal(&mut r);
    check(&i, &r).unwrap();
    let mut r = full.clone();
    r.limits.bytes = 1;
    r.stop = Stop::ByteLimit;
    r.report_final = false;
    r.unacknowledged = Some(r.events[0].clone());
    r.events.clear();
    retotal(&mut r);
    check(&i, &r).unwrap();
    r.acknowledged_bytes = r.unacknowledged_bytes;
    assert!(check(&i, &r).is_err());
    let mut r = full;
    r.stop = Stop::SinkFailure;
    r.report_final = false;
    r.sink_error = Some("retained I/O failure".into());
    r.unacknowledged = Some(r.events.remove(0));
    r.events.clear();
    retotal(&mut r);
    check(&i, &r).unwrap();
}
#[test]
fn input_limit_has_no_hash_or_events() {
    let (i, mut r) = two_rows();
    r.limits.input_rows = 1;
    r.stop = Stop::InputLimit;
    r.input_sha256 = None;
    r.report_final = false;
    r.events.clear();
    retotal(&mut r);
    check(&i, &r).unwrap();
    r.input_sha256 = Some("a".repeat(64));
    assert!(check(&i, &r).is_err());
}
#[test]
fn rehashed_transition_and_total_tampering_fails() {
    let (i, r) = two_rows();
    for j in 0..r.events.len() {
        let mut bad = r.clone();
        bad.events[j].report_after.merges += 1;
        retotal(&mut bad);
        assert!(check(&i, &bad).is_err(), "event {j}");
    }
    for variant in 0..5 {
        let mut bad = r.clone();
        if let Transition::Reduction {
            inverse,
            scale,
            candidate,
            accepted,
            reference,
            ..
        } = &mut bad.events[9].transition
        {
            match variant {
                0 => *inverse = 2,
                1 => *scale = 5,
                2 => candidate.rhs = 2,
                3 => *accepted = false,
                _ => reference.version = 1,
            }
        }
        retotal(&mut bad);
        assert!(check(&i, &bad).is_err());
    }
    let mut bad = r;
    bad.dependency_basis_complete_verified = true;
    assert!(check(&i, &bad).is_err());
}
#[test]
fn rank_determination_matches_exhaustive_f3_solutions() {
    for a in 0..81usize {
        for rhs in 0..9usize {
            let coeff = [a % 3, (a / 3) % 3, (a / 9) % 3, (a / 27) % 3];
            let vals = [rhs % 3, (rhs / 3) % 3];
            let rows: Vec<_> = (0..2)
                .map(|i| Complete {
                    small: (0..2)
                        .filter_map(|j| {
                            (coeff[2 * i + j] != 0).then_some((j as u32, coeff[2 * i + j] as u64))
                        })
                        .collect(),
                    rhs: vals[i] as u64,
                    sources: vec![],
                })
                .collect();
            let solutions: Vec<_> = (0..9)
                .map(|s| [s % 3, s / 3])
                .filter(|s| {
                    (0..2).all(|i| (coeff[2 * i] * s[0] + coeff[2 * i + 1] * s[1]) % 3 == vals[i])
                })
                .collect();
            let answer = rank::reduce(&rows, 2, 3, &mut Budget::default());
            if solutions.is_empty() {
                assert!(answer.is_err());
            } else {
                let x = answer.unwrap();
                for col in 0..2 {
                    assert_eq!(
                        x.determined[col],
                        solutions.iter().all(|s| s[col] == solutions[0][col])
                    );
                }
                assert!(solutions.contains(&[x.values[0] as usize, x.values[1] as usize]));
            }
        }
    }
}
#[test]
fn raw_high_word_coefficients_and_memory_work_admission() {
    let r = (1u64 << 63) - 25;
    let mut b = Budget::default();
    assert_eq!(
        filter::normalize(&[(u64::MAX, u64::MAX), (u64::MAX, 1)], r, &mut b).unwrap(),
        vec![(u64::MAX, 50)]
    );
    assert_eq!(filter::mul(r - 1, r - 1, r), 1);
    assert!(b.matrix(4096, 4097).is_err());
    let mut b = Budget::default();
    assert!(b.work(128_000_001).is_err());
    assert!(b.curves(1_048_577).is_err());
}
#[test]
fn strict_schema_and_distinct_hash_encodings() {
    let (i, r) = two_rows();
    let mut v = serde_json::to_value(&r).unwrap();
    v["surprise"] = json!(1);
    assert!(serde_json::from_value::<Run>(v).is_err());
    let mut v = serde_json::to_value(&r).unwrap();
    v["input_rows"] = json!(2.0);
    assert!(serde_json::from_value::<Run>(v).is_err());
    assert_ne!(hash(&i).unwrap(), digest(&serde_json::to_vec(&i).unwrap()));
    assert_eq!(
        hash(&json!({"z":{"b":2,"a":1},"a":0})).unwrap(),
        hash(&json!({"a":0,"z":{"a":1,"b":2}})).unwrap()
    );
}
#[test]
fn cumulative_caps_charge_attempts_computation_and_refusals() {
    let (_, r) = two_rows();
    let mut t = Totals::default();
    t.charge(&r).unwrap();
    t.charge(&r).unwrap();
    assert_eq!(t.remaining().unwrap().work, 1_000_000 - 2 * r.computed_work);
    assert_eq!(
        t.remaining().unwrap().events,
        100_000 - 2 * r.events.len() as u64
    );
    let mut refusal = r;
    refusal.unacknowledged = Some(refusal.events.pop().unwrap());
    refusal.stop = Stop::ByteLimit;
    refusal.report_final = false;
    retotal(&mut refusal);
    t.charge(&refusal).unwrap();
    assert_eq!(t.refused_event_bytes, refusal.unacknowledged_bytes);
    assert_eq!(t.attempted_events, t.acknowledged_events + 1);
}

#[test]
fn rejected_fill_finishes_without_committing_candidate() {
    let (input, mut run) = two_rows();
    run.options.max_small_weight = 1;
    if let Transition::Begin { options, .. } = &mut run.events[0].transition {
        *options = run.options.clone();
    }
    if let Transition::Reduction { accepted, .. } = &mut run.events[9].transition {
        *accepted = false;
    }
    run.events.remove(10);
    if let Transition::Finish { rows } = &mut run.events[10].transition {
        rows.clear();
    }
    for event in &mut run.events[9..] {
        event.report_after.merges = 0;
        event.report_after.rejected_fill = 1;
        event.report_after.complete_relations = 0;
        event.report_after.cycles_closed = 0;
    }
    retotal(&mut run);
    check(&input, &run).unwrap();
    assert!(run.rows.is_empty());
}

#[test]
fn duplicate_normalization_smooth_rows_and_two_valid_singletons() {
    let (_, template) = two_rows();
    let inputs = vec![
        Partial {
            id: 1,
            small: vec![(2, 1), (2, 6)],
            large: vec![],
            rhs: 7,
        },
        Partial {
            id: 2,
            small: vec![],
            large: vec![],
            rhs: 0,
        },
        Partial {
            id: 3,
            small: vec![],
            large: vec![(8, 1), (9, 1)],
            rhs: 0,
        },
    ];
    for choice in [8, 9] {
        let mut run = template.clone();
        run.input_rows = 3;
        run.input_terms = 4;
        let sha = digest(&serde_json::to_vec(&inputs).unwrap());
        run.input_sha256 = Some(sha.clone());
        let mut report = Report {
            input_relations: 3,
            ..Report::default()
        };
        let mut events = vec![event(
            Phase::Begin,
            0,
            0,
            Transition::Begin {
                input_sha256: sha,
                input_rows: 3,
                input_terms: 4,
                options: run.options.clone(),
                limits: run.limits.clone(),
            },
            &report,
        )];
        let normalized = Partial {
            id: 1,
            small: vec![],
            large: vec![],
            rhs: 0,
        };
        events.push(event(
            Phase::Normalize,
            3,
            4,
            Transition::Normalized {
                position: 0,
                row: normalized.clone(),
            },
            &report,
        ));
        events.push(event(
            Phase::Normalize,
            1,
            2,
            Transition::Normalized {
                position: 1,
                row: inputs[1].clone(),
            },
            &report,
        ));
        report.duplicate_relations = 1;
        events.push(event(
            Phase::Duplicate,
            0,
            0,
            Transition::DuplicateDropped {
                input_id: 2,
                representative: 1,
            },
            &report,
        ));
        events.push(event(
            Phase::Normalize,
            3,
            4,
            Transition::Normalized {
                position: 2,
                row: inputs[2].clone(),
            },
            &report,
        ));
        report.input_large_vertices = 2;
        report.peeled_relations = 1;
        let mut peel = event(
            Phase::Peel,
            2,
            4,
            Transition::Peel {
                column: choice,
                input_id: 3,
                degrees_before: vec![(8, 1), (9, 1)],
                degrees_after: vec![(8, 0), (9, 0)],
            },
            &report,
        );
        peel.reservation.choice = Some(choice);
        events.push(peel);
        events.push(event(
            Phase::Core,
            0,
            0,
            Transition::CoreReady {
                row_ids: vec![],
                smooth_ids: vec![1],
                degrees: vec![],
            },
            &report,
        ));
        report.complete_relations = 1;
        let row = Row {
            small: vec![],
            large: vec![],
            rhs: 0,
            sources: vec![(1, 1)],
        };
        events.push(event(
            Phase::Complete,
            0,
            2,
            Transition::CompleteRow {
                reference: RowRef {
                    input_id: 1,
                    version: 0,
                },
                row: row.clone(),
            },
            &report,
        ));
        events.push(event(
            Phase::Finish,
            0,
            0,
            Transition::Finish {
                rows: vec![row.complete()],
            },
            &report,
        ));
        run.events = events;
        retotal(&mut run);
        check(&inputs, &run).unwrap();
    }
}

#[test]
fn exact_v2_file_boundary_and_known_query_stream() {
    let temp = tempfile::tempdir().unwrap();
    let mut bytes = b"{}".to_vec();
    bytes.resize(FILE_MAX as usize, b' ');
    std::fs::write(temp.path().join("bounded.json"), &bytes).unwrap();
    let mut artifact = json!({"path":"bounded.json","role":ROLE,"encoding":"json","bytes":bytes.len(),"sha256":digest(&bytes)});
    assert_eq!(
        json::load(temp.path(), &artifact, budget::MEMORY)
            .unwrap()
            .0,
        json!({})
    );
    bytes.push(b' ');
    std::fs::write(temp.path().join("bounded.json"), &bytes).unwrap();
    artifact["bytes"] = json!(bytes.len());
    artifact["sha256"] = json!(digest(&bytes));
    assert!(json::load(temp.path(), &artifact, budget::MEMORY).is_err());
    // Pinned58bd first n12/m3 software fixture, seed42/gen1/key2, scalar modulus397.
    let mut rng = StdRng::seed_from_u64(
        42 ^ 1u64.rotate_left(17) ^ 2u64.rotate_left(31) ^ 0x8a01_731f_94b0_d6c3,
    );
    assert_eq!(
        (
            rng.gen_range(0u64..397),
            rng.gen_range(1u64..397),
            rng.gen_range(0usize..2)
        ),
        (10, 48, 0)
    );
    assert_eq!(
        (
            rng.gen_range(0u64..397),
            rng.gen_range(1u64..397),
            rng.gen_range(0usize..2)
        ),
        (208, 125, 1)
    );
}
