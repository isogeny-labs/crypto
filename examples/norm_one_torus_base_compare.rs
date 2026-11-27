//! Matched Rust comparison of the production Prime31 Dickson/T2 factor
//! base and the equally sized ordinary small-abscissa factor base.
//!
//! The retained side is the canonical three-sum quotient. The two-sum side
//! is regenerated in fixed blocks for every target and is never stored across
//! targets. Full-scan operation counts on common targets are primary; local
//! wall time is a producer diagnostic only.

#[path = "support/norm_one_torus_base_compare.rs"]
mod common;

#[allow(dead_code)]
mod engine {
    include!("support/norm_one_torus_compare_kernel.rs");

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub(super) struct PublicOps {
        pub multiplications: u64,
        pub squarings: u64,
        pub inversions: u64,
        pub additions: u64,
        pub doublings: u64,
    }

    impl From<OpCounts> for PublicOps {
        fn from(ops: OpCounts) -> Self {
            Self {
                multiplications: ops.multiplications,
                squarings: ops.squarings,
                inversions: ops.inversions,
                additions: ops.additions,
                doublings: ops.doublings,
            }
        }
    }

    impl Jacobian {
        fn negated(self, modulus: u64) -> Self {
            if self.z == 0 || self.y == 0 {
                self
            } else {
                Self {
                    y: modulus - self.y,
                    ..self
                }
            }
        }
    }

    struct StreamingSolver {
        base: Vec<PrimePoint>,
        right: HashMap<XKey, QuotientEntry>,
        neg_index: Vec<usize>,
        curve_a: u64,
        modulus: u64,
        right_states: usize,
    }

    struct QueryBatch {
        elapsed: Duration,
        ops: OpCounts,
        pair_candidates: u64,
        table_probes: u64,
        results: Vec<Option<Vec<usize>>>,
    }

    #[derive(Clone, Debug)]
    pub(super) struct StreamingRun {
        pub setup: Duration,
        pub setup_ops: PublicOps,
        pub right_states: usize,
        pub right_keys: usize,
        pub random_full_elapsed: Duration,
        pub random_full_ops: PublicOps,
        pub random_full_pairs: u64,
        pub random_full_probes: u64,
        pub random_full_results: Vec<Option<Vec<usize>>>,
        pub random_first_elapsed: Duration,
        pub random_first_ops: PublicOps,
        pub random_first_pairs: u64,
        pub random_first_probes: u64,
        pub random_first_results: Vec<Option<Vec<usize>>>,
        pub planted_elapsed: Duration,
        pub planted_ops: PublicOps,
        pub planted_pairs: u64,
        pub planted_probes: u64,
        pub planted_results: Vec<Option<Vec<usize>>>,
        pub certificate_results: Vec<Option<Vec<usize>>>,
    }

    impl StreamingSolver {
        fn new(fixture: &Fixture) -> (Self, OpCounts) {
            let mut ops = OpCounts::default();
            let right_states = triple_states(fixture, true, &mut ops);
            let right_state_count = right_states.len();
            let right_points = normalize_batch(&right_states, fixture.instance.curve.p, &mut ops);
            let mut right = HashMap::with_capacity(right_points.len());
            for (point, witness) in right_points {
                right
                    .entry(point.into())
                    .or_insert(QuotientEntry { point, witness });
            }
            (
                Self {
                    base: fixture.base.clone(),
                    right,
                    neg_index: fixture.neg_index.clone(),
                    curve_a: fixture.instance.curve.a,
                    modulus: fixture.instance.curve.p,
                    right_states: right_state_count,
                },
                ops,
            )
        }

        fn probe_block(
            &self,
            block: &[(Jacobian, [u16; 2])],
            stop_on_hit: bool,
            ops: &mut OpCounts,
            probes: &mut u64,
            result: &mut Option<Vec<usize>>,
        ) -> bool {
            for (point, left_witness) in normalize_batch(block, self.modulus, ops) {
                *probes += 1;
                let Some(entry) = self.right.get(&point.into()) else {
                    continue;
                };
                if result.is_none() {
                    let right_witness = if point == entry.point {
                        entry.witness
                    } else {
                        assert_eq!(point, negate(entry.point, self.modulus));
                        negated_witness(entry.witness, &self.neg_index)
                    };
                    *result = Some(
                        left_witness
                            .into_iter()
                            .chain(right_witness)
                            .map(usize::from)
                            .collect(),
                    );
                }
                if stop_on_hit {
                    return true;
                }
            }
            false
        }

        fn decomposition(
            &self,
            target: PrimePoint,
            block_size: usize,
            stop_on_hit: bool,
            ops: &mut OpCounts,
            pair_candidates: &mut u64,
            probes: &mut u64,
        ) -> Option<Vec<usize>> {
            assert!(block_size > 0);
            let mut block = Vec::with_capacity(block_size);
            let mut result = None;
            for first in 0..self.base.len() {
                let left = Jacobian::from_affine(self.base[first]);
                for second in first..self.base.len() {
                    let pair = left.add_mixed(self.base[second], self.curve_a, self.modulus, ops);
                    let complement = pair.negated(self.modulus).add_mixed(
                        target,
                        self.curve_a,
                        self.modulus,
                        ops,
                    );
                    block.push((complement, [first as u16, second as u16]));
                    *pair_candidates += 1;
                    if block.len() == block_size {
                        let stop = self.probe_block(&block, stop_on_hit, ops, probes, &mut result);
                        block.clear();
                        if stop {
                            return result;
                        }
                    }
                }
            }
            if !block.is_empty() {
                self.probe_block(&block, stop_on_hit, ops, probes, &mut result);
            }
            result
        }

        fn query(
            &self,
            targets: &[PrimePoint],
            block_size: usize,
            stop_on_hit: bool,
        ) -> QueryBatch {
            let started = Instant::now();
            let mut ops = OpCounts::default();
            let mut pair_candidates = 0;
            let mut table_probes = 0;
            let results = targets
                .iter()
                .copied()
                .map(|target| {
                    self.decomposition(
                        target,
                        block_size,
                        stop_on_hit,
                        &mut ops,
                        &mut pair_candidates,
                        &mut table_probes,
                    )
                })
                .collect();
            QueryBatch {
                elapsed: started.elapsed(),
                ops,
                pair_candidates,
                table_probes,
                results,
            }
        }
    }

    pub(super) fn time_streaming(
        source: &super::common::Fixture,
        block_size: usize,
    ) -> StreamingRun {
        let started = Instant::now();
        let (solver, setup_ops) = StreamingSolver::new(source);
        let setup = started.elapsed();
        let (random, planted) = source.targets.split_at(source.random_targets);
        let random_full = solver.query(random, block_size, false);
        let random_first = solver.query(random, block_size, true);
        let planted = solver.query(planted, block_size, true);
        assert_eq!(
            random_full
                .results
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>(),
            random_first
                .results
                .iter()
                .map(Option::is_some)
                .collect::<Vec<_>>()
        );
        let certificate_results = random_full
            .results
            .iter()
            .cloned()
            .chain(planted.results.iter().cloned())
            .collect::<Vec<_>>();
        assert!(super::common::verify_results(source, &certificate_results));
        StreamingRun {
            setup,
            setup_ops: setup_ops.into(),
            right_states: solver.right_states,
            right_keys: solver.right.len(),
            random_full_elapsed: random_full.elapsed,
            random_full_ops: random_full.ops.into(),
            random_full_pairs: random_full.pair_candidates,
            random_full_probes: random_full.table_probes,
            random_full_results: random_full.results,
            random_first_elapsed: random_first.elapsed,
            random_first_ops: random_first.ops.into(),
            random_first_pairs: random_first.pair_candidates,
            random_first_probes: random_first.table_probes,
            random_first_results: random_first.results,
            planted_elapsed: planted.elapsed,
            planted_ops: planted.ops.into(),
            planted_pairs: planted.pair_candidates,
            planted_probes: planted.table_probes,
            planted_results: planted.results,
            certificate_results,
        }
    }
}

use std::fs::{self, OpenOptions};
use std::io::Write;

use common::{base_digest, certificate_bundle, result_digest, BaseKind, Fixture};
use engine::{PublicOps, StreamingRun};

fn millis(duration: std::time::Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn successes(results: &[Option<Vec<usize>>]) -> usize {
    results.iter().filter(|result| result.is_some()).count()
}

fn write_certificate(path: &str, bytes: &[u8]) {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut output) => output.write_all(bytes).expect("write certificate"),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            assert_eq!(fs::read(path).expect("read retained certificate"), bytes);
        }
        Err(error) => panic!("create certificate {path}: {error}"),
    }
}

fn push_ops(fields: &mut Vec<String>, ops: PublicOps) {
    fields.extend([
        ops.inversions.to_string(),
        ops.multiplications.to_string(),
        ops.squarings.to_string(),
        ops.additions.to_string(),
        ops.doublings.to_string(),
    ]);
}

fn print_run(round: usize, label: &str, fixture: &Fixture, run: &StreamingRun) {
    let mut fields = vec![
        round.to_string(),
        label.to_owned(),
        fixture.kind.as_str().to_owned(),
        format!("{:.3}", millis(run.setup)),
        format!("{:.3}", millis(run.random_full_elapsed)),
        format!("{:.3}", millis(run.random_first_elapsed)),
        format!("{:.3}", millis(run.planted_elapsed)),
        run.right_states.to_string(),
        run.right_keys.to_string(),
    ];
    push_ops(&mut fields, run.setup_ops);
    push_ops(&mut fields, run.random_full_ops);
    fields.extend([
        run.random_full_pairs.to_string(),
        run.random_full_probes.to_string(),
        successes(&run.random_full_results).to_string(),
    ]);
    push_ops(&mut fields, run.random_first_ops);
    fields.extend([
        run.random_first_pairs.to_string(),
        run.random_first_probes.to_string(),
        successes(&run.random_first_results).to_string(),
    ]);
    push_ops(&mut fields, run.planted_ops);
    fields.extend([
        run.planted_pairs.to_string(),
        run.planted_probes.to_string(),
        successes(&run.planted_results).to_string(),
        fixture.build_counts.abscissae_scanned.to_string(),
        fixture.build_counts.legendre_symbols.to_string(),
        fixture.build_counts.selector_sqrt_solves.to_string(),
        fixture.build_counts.curve_sqrt_solves.to_string(),
        base_digest(fixture),
        result_digest(&run.certificate_results),
    ]);
    println!("{}", fields.join(","));
}

fn fixture(fixtures: &[Fixture], kind: BaseKind) -> &Fixture {
    fixtures
        .iter()
        .find(|fixture| fixture.kind == kind)
        .expect("both fixed arms")
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_base_compare --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");
    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let fixtures = common::load_fixtures(&manifest);
    let configuration = &manifest.configuration;
    assert_eq!(fixtures.len(), 2);
    assert_eq!(fixtures[0].abscissae, fixtures[1].abscissae);
    assert_eq!(fixtures[0].base.len(), fixtures[1].base.len());
    assert_eq!(
        &fixtures[0].targets[..configuration.random_targets],
        &fixtures[1].targets[..configuration.random_targets]
    );

    let warm: Vec<_> = BaseKind::ALL
        .into_iter()
        .map(|kind| {
            let fixture = fixture(&fixtures, kind);
            (
                kind,
                engine::time_streaming(fixture, configuration.query_block_size),
            )
        })
        .collect();
    for (kind, run) in &warm {
        assert_eq!(
            successes(&run.planted_results),
            configuration.planted_targets_per_base,
            "{} positive controls",
            kind.as_str()
        );
    }

    println!(
        "round,label,base,setup_ms,random_full_ms,random_first_ms,planted_first_ms,right_states,right_keys,setup_inversions,setup_multiplications,setup_squarings,setup_additions,setup_doublings,random_full_inversions,random_full_multiplications,random_full_squarings,random_full_additions,random_full_doublings,random_full_pairs,random_full_probes,random_full_successes,random_first_inversions,random_first_multiplications,random_first_squarings,random_first_additions,random_first_doublings,random_first_pairs,random_first_probes,random_first_successes,planted_inversions,planted_multiplications,planted_squarings,planted_additions,planted_doublings,planted_pairs,planted_probes,planted_successes,base_abscissae_scanned,base_legendre_symbols,base_selector_sqrt_solves,base_curve_sqrt_solves,factor_base_sha256,result_digest"
    );
    for round in 0..configuration.rounds {
        let order = if round.is_multiple_of(2) {
            BaseKind::ALL
        } else {
            [BaseKind::OrdinarySmallX, BaseKind::DicksonTorus]
        };
        for kind in order {
            let fixture = fixture(&fixtures, kind);
            let expected = &warm
                .iter()
                .find(|(warm_kind, _)| *warm_kind == kind)
                .unwrap()
                .1
                .certificate_results;
            for suffix in ["primary", "aa"] {
                let run = engine::time_streaming(fixture, configuration.query_block_size);
                assert_eq!(&run.certificate_results, expected);
                print_run(round, suffix, fixture, &run);
            }
        }
    }

    let results: Vec<_> = fixtures
        .iter()
        .map(|fixture| {
            warm.iter()
                .find(|(kind, _)| *kind == fixture.kind)
                .unwrap()
                .1
                .certificate_results
                .clone()
        })
        .collect();
    let bundle = certificate_bundle(&fixtures, &manifest_bytes, &results);
    let bytes = serde_json::to_vec_pretty(&bundle).expect("serialize certificates");
    write_certificate(&arguments[4], &bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manifest() -> common::Manifest {
        serde_json::from_value(serde_json::json!({
            "schema": "isogeny-labs/experiment-seeds/v1",
            "study_id": "unit-test-only",
            "master_seed_hex": "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            "stream_derivation": {"domain": "unit-test/not-a-measurement"},
            "configuration": {
                "curve": crypto_lib::cryptanalysis::ic_boundary::fp_p31_subexp_mersenne_instance().name,
                "dickson_height": 6,
                "dickson_lo": 0,
                "dickson_c_top": 0,
                "random_targets": 3,
                "planted_targets_per_base": 2,
                "arity": 5,
                "rounds": 1,
                "query_block_size": 1024
            }
        }))
        .unwrap()
    }

    #[test]
    fn production_bases_are_exactly_cardinality_matched() {
        let fixtures = common::load_fixtures(&test_manifest());
        assert_eq!(fixtures.len(), 2);
        assert_eq!(fixtures[0].abscissae, 34);
        assert_eq!(fixtures[0].abscissae, fixtures[1].abscissae);
        assert_eq!(fixtures[0].base.len(), 68);
        assert_eq!(fixtures[0].base.len(), fixtures[1].base.len());
        assert_ne!(base_digest(&fixtures[0]), base_digest(&fixtures[1]));
    }

    #[test]
    fn streaming_solver_recovers_every_planted_control() {
        let fixtures = common::load_fixtures(&test_manifest());
        for fixture in &fixtures {
            let run = engine::time_streaming(fixture, 1024);
            assert_eq!(successes(&run.planted_results), 2);
            assert_eq!(run.random_full_pairs, 3 * 68 * 69 / 2);
            assert_eq!(run.random_full_pairs, run.random_full_probes);
            assert_eq!(
                run.random_full_results
                    .iter()
                    .map(Option::is_some)
                    .collect::<Vec<_>>(),
                run.random_first_results
                    .iter()
                    .map(Option::is_some)
                    .collect::<Vec<_>>()
            );
            assert!(common::verify_results(fixture, &run.certificate_results));
        }
    }
}
