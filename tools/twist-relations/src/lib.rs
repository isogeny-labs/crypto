//! Toy-curve experiment contracts, with complete phase accounting.
pub mod benchmark;
pub mod core;
pub mod evidence;
pub mod exhaustive;
pub mod tr04;
pub mod tr04_protocol;
pub mod tr04_relation;
pub mod tr04_report;
pub mod tr04_select;
pub mod tr04_verify;

pub fn valid_prime(p: u64, maximum: u64) -> bool {
    p >= 7 && p <= maximum && p % 4 == 3 && core::prime(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_full_finite_scope_matches_historical_correctness_fixture() {
        // This is an untimed correctness regression, not a performance result.
        let audit = exhaustive::run(&[7, 11, 19, 31, 43]);
        assert_eq!(audit.curves.len(), 457);
        assert_eq!(
            audit.curves.iter().map(|c| c.factor_bases).sum::<u64>(),
            439435
        );
        assert_eq!(
            audit.curves.iter().map(|c| c.f3_triples).sum::<u64>(),
            4215282
        );
        assert_eq!(
            audit
                .curves
                .iter()
                .map(|c| c.f3_zero_counts_by_original_points.values().sum::<u64>())
                .sum::<u64>(),
            106820
        );
        assert!(audit.curves.iter().all(|c| c.f3_mixed_zero_count == 0));
    }

    #[test]
    fn solver_replays_and_phase_counts_reconcile() {
        let rows = benchmark::run(&[43, 67], 2, 9);
        assert_eq!(rows.len(), 16);
        benchmark::verify(&rows).unwrap();
        assert!(rows.iter().all(|r| r.seed == 9 || r.seed == 10));
        for row in &rows {
            if row.method != "bsgs" {
                assert_eq!(row.rank, row.factor_base_size + 1);
                assert_eq!(row.factor_base_logs.len(), row.factor_base_size);
                assert!(row.table_entries <= row.factor_base_size * (row.factor_base_size + 1) / 2);
            }
        }
        let mut bad = rows.clone();
        bad[0].recovered_scalar = 0;
        assert!(benchmark::verify(&bad).is_err());
        let mut bad = rows.clone();
        bad[0].counts.group_add_calls += 1;
        assert!(benchmark::verify(&bad).is_err());
    }

    #[test]
    fn arithmetic_handles_identity_inverse_and_two_torsion() {
        let mut e = core::Curve::new(7, 0, 1);
        let p = Some(core::Point { x: 0, y: 1 });
        assert!(e.on_curve(p));
        assert_eq!(e.add(None, p), p);
        assert_eq!(e.add(p, e.neg(p)), None);
        assert_eq!(e.mul(0, p), None);
        assert_eq!(e.mul(3, p), None);
        assert_eq!(
            e.add(
                Some(core::Point { x: 3, y: 0 }),
                Some(core::Point { x: 3, y: 0 })
            ),
            None
        );
    }

    #[test]
    fn default_native_known_answer_panel_is_replayable() {
        // Validate every default fixture without making a timing comparison.
        let rows = benchmark::run(&[1019, 4091, 16319], 30, 0);
        assert_eq!(rows.len(), 360);
        assert_eq!(
            rows.iter().map(|r| r.factor_base_logs.len()).sum::<usize>(),
            4590
        );
        benchmark::verify(&rows).unwrap();
    }

    #[test]
    fn evidence_sink_error_stops_before_the_next_case() {
        let mut started = 0;
        let mut failed = 0;
        let failure =
            benchmark::run_observed(&[43], 1, 0, benchmark::Options::default(), |event| {
                match event {
                    benchmark::Event::Started { .. } => started += 1,
                    benchmark::Event::Completed { .. } => {
                        return Err("deliberate evidence sink failure".into())
                    }
                    benchmark::Event::Failed { .. } => failed += 1,
                    _ => {}
                }
                Ok(())
            })
            .unwrap_err();
        assert_eq!((started, failed), (1, 1));
        assert_eq!(failure.case.unwrap().method, "bsgs");
        assert_eq!(failure.progress.phase, "evidence_publication");
        assert!(failure.progress.counts.group_add_calls > 0);
        assert!(failure.error.contains("deliberate evidence sink failure"));
    }
}
