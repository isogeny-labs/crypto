//! Tests for `large_prime_graph::retention` (online admission under a cap).

use crypto_lib::cryptanalysis::large_prime_filter::{
    replay_complete_relation, CompleteRelation, PartialRelation,
};
use crypto_lib::cryptanalysis::large_prime_graph::eval::{synthetic_stream, SyntheticParams};
use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
use crypto_lib::cryptanalysis::large_prime_graph::retention::{
    simulate_admission, AdmissionRule, IncrementalRank, StreamRow,
};

#[path = "../examples/lp_retention_support/fp_toy.rs"]
#[allow(dead_code)]
mod fp_toy;

const RULES: [AdmissionRule; 6] = [
    AdmissionRule::All,
    AdmissionRule::CompleteOnly,
    AdmissionRule::FirstCome,
    AdmissionRule::SinglesOnly,
    AdmissionRule::MaxLp(400),
    AdmissionRule::SeenLp,
];

fn stream(seed: u64, rows: usize, fb: u32, ell: u64) -> Vec<StreamRow> {
    let p = SyntheticParams {
        seed,
        rows,
        lp_columns: 600,
        fb_columns: fb,
        fb_terms: 3,
        arity_weights: vec![1, 4, 8],
        ..SyntheticParams::default()
    };
    synthetic_stream(&p, ell)
        .into_iter()
        .enumerate()
        .map(|(i, relation)| StreamRow {
            draw: i as u64 + 1,
            relation,
        })
        .collect()
}

fn canonical<K: Ord + Copy>(v: &[(K, u64)], p: u64) -> Vec<(K, u64)> {
    let mut m = std::collections::BTreeMap::<K, u64>::new();
    for &(c, a) in v {
        let e = m.entry(c).or_default();
        *e = (*e + a % p) % p;
    }
    m.into_iter().filter(|&(_, a)| a != 0).collect()
}

/// The incremental rank equals the repository's exact sparse audit.
#[test]
fn incremental_rank_matches_projected_rank_audit() {
    for &ell in &[7u64, 101, 1_000_003] {
        for seed in 0..20u64 {
            let rows: Vec<CompleteRelation> = stream(seed, 40 + seed as usize, 24, ell)
                .into_iter()
                .map(|r| CompleteRelation {
                    small: canonical(&r.relation.small, ell),
                    rhs: 0,
                    sources: vec![],
                })
                .collect();
            let mut inc = IncrementalRank::new(24, ell).unwrap();
            for r in &rows {
                inc.add(&r.small).unwrap();
            }
            let audit = audit_projected_rank(&rows, ell).unwrap();
            assert_eq!(inc.rank(), audit.useful_small_rank, "ell={ell} seed={seed}");
        }
    }
}

/// Every harvested combination cancels its large primes and replays, through
/// `replay_complete_relation`, to exactly the row the simulator added.
#[test]
fn harvested_relations_replay_exactly() {
    let mut checked = 0;
    for &ell in &[101u64, 1_000_003] {
        for seed in 0..10u64 {
            let s = stream(seed, 800, 40, ell);
            let inputs: Vec<PartialRelation> = s.iter().map(|r| r.relation.clone()).collect();
            for rule in [
                AdmissionRule::All,
                AdmissionRule::FirstCome,
                AdmissionRule::SeenLp,
            ] {
                let out =
                    simulate_admission(&s, 40, ell, rule, Some(300), usize::MAX, true).unwrap();
                assert!(out.harvested_relations > 0);
                for h in &out.harvested {
                    let proposed = CompleteRelation {
                        small: vec![],
                        rhs: 0,
                        sources: h.sources.clone(),
                    };
                    let replayed = replay_complete_relation(&proposed, &inputs, ell)
                        .expect("cancels and replays");
                    assert_eq!(canonical(&replayed.small, ell), h.small);
                    assert_eq!(replayed.rhs % ell, h.rhs);
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 1000, "checked {checked}");
}

/// `All` is never below any rule and every rule is never below
/// `CompleteOnly`, at every draw; caps and rule semantics hold.
#[test]
fn ordering_guarantee_and_rule_semantics() {
    let ell = 1_000_003;
    let rank_at = |trace: &[(u64, usize)], d: u64| {
        trace
            .iter()
            .take_while(|&&(x, _)| x <= d)
            .last()
            .map_or(0, |t| t.1)
    };
    for seed in 0..12u64 {
        let s = stream(seed, 1200, 50, ell);
        let run =
            |rule, cap| simulate_admission(&s, 50, ell, rule, cap, usize::MAX, false).unwrap();
        let all = run(AdmissionRule::All, None);
        let none = run(AdmissionRule::CompleteOnly, None);
        assert_eq!(none.stored_partials, 0);
        for cap in [Some(10), Some(100), Some(100_000)] {
            for rule in RULES {
                let o = run(rule, cap);
                let capped = !matches!(rule, AdmissionRule::All | AdmissionRule::CompleteOnly);
                if let (Some(c), true) = (cap, capped) {
                    assert!(o.stored_partials <= c);
                }
                for d in (1..=1200).step_by(37) {
                    let (a, r, n) = (
                        rank_at(&all.trace, d),
                        rank_at(&o.trace, d),
                        rank_at(&none.trace, d),
                    );
                    assert!(
                        a >= r && r >= n,
                        "seed={seed} {rule:?} cap={cap:?} draw={d}: {a} {r} {n}"
                    );
                }
            }
        }
        // FirstCome with a cap above the stream stores every partial; SinglesOnly only singles.
        let fc = run(AdmissionRule::FirstCome, Some(100_000));
        assert_eq!(fc.stored_partials, all.stored_partials);
        let singles = s
            .iter()
            .filter(|r| canonical(&r.relation.large, ell).len() == 1)
            .count();
        assert_eq!(
            run(AdmissionRule::SinglesOnly, None).stored_partials,
            singles
        );
    }
}

/// Stopping at the target reports the first draw whose rank reached it.
#[test]
fn draws_to_target_is_first_crossing() {
    let ell = 1_000_003;
    let s = stream(4, 2000, 30, ell);
    let full =
        simulate_admission(&s, 30, ell, AdmissionRule::All, None, usize::MAX, false).unwrap();
    let t = full.final_rank;
    let stop = simulate_admission(&s, 30, ell, AdmissionRule::All, None, t, false).unwrap();
    let first = full.trace.iter().find(|&&(_, r)| r >= t).unwrap().0;
    assert_eq!(stop.draws_to_target, Some(first));
    assert_eq!(stop.final_rank, t);
}

#[test]
fn fp_toy_collects_and_replays() {
    let p = fp_toy::Instance::next_prime(1 << 31);
    assert_eq!(p, 2_147_483_659);
    let inst = fp_toy::Instance::new(p, 1001);
    assert!(fp_toy::is_prime(inst.ell) && (p - 1).is_multiple_of(inst.ell));
    let mut col = fp_toy::collect(&inst, 1024, 65536, 20_000, 0);
    assert!(col.stream.len() > 1000);
    assert_eq!(fp_toy::replay(&col), Ok(col.stream.len()));
    col.exponents[3] += 1;
    assert_eq!(fp_toy::replay(&col), Err(3));
}
