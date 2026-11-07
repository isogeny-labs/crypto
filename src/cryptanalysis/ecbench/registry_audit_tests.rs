//! The registry-wide accounting check: every registered method is run on a
//! small curve it applies to, and its phases must add up to its total.
//!
//! `methods::tests::the_listed_methods_phases_sum_to_their_totals` checked a
//! fixed list, and a method registered later was in it only if its author
//! remembered (ten methods registered after 2026-10-28 were not).  This test
//! walks [`registry`] itself.  Every id must be exactly one of
//!
//! * a **case** ([`CASES`]): run on the named fixture with the named
//!   parameters, which must be `applicable`, recover the planted logarithm,
//!   and report phases that sum to `total_gae`;
//! * a **skip** ([`SKIPS`]): not run, with the reason (an external backend,
//!   or no instance small enough to build in a unit test);
//! * an **exemption** ([`EXEMPT`]): run like a case, but the sum is known to
//!   fall short by design, so the id is frozen (README §9) and a `_v2` id
//!   carries the fix.  An exemption that starts summing fails the test, so
//!   the list cannot go stale.
//!
//! A newly registered id that is none of these fails the test and names
//! itself: add a case (preferred), or a skip with its reason.

use super::accounting_rules_tests::{j0, j1728};
use super::*;
use crate::cryptanalysis::ecbench::workload::{CurveSpec, HashWork};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Fx {
    /// `PrimeSearch { 18, 59297 }`: prime order, r about 2^18.
    Prime18,
    /// The APS instrument's own `p = 101` development curve (prime order 107).
    Aps101,
    /// The registry-optimization study's frozen `p = 739` Scholten fixture.
    Scholten,
    /// Its binary `n = 6` subfield fixture (the decomposition adapters').
    TlSubfield,
    /// `PrimeSearch { 14, 5 }`: has a rational 3-isogeny and a quartic cover.
    Prime14,
    /// `PrimeSearch { 10, 59297 }`: small enough for the tensor toy.
    Prime10,
    J0,
    J1728,
    /// `Koblitz { a: 0, n: 19 }` and `{ a: 1, n: 17 }`: one-word Koblitz.
    K0n19,
    K1n17,
    /// `Koblitz { a: 0, n: 7 }`: the Jacobian-image adapters' size.
    K0n7,
    /// `BinaryExplicit`, n = 11, a non-Koblitz ordinary curve.
    Binary11,
    F2m4,
    Genus2,
    Ghs,
    Hyper,
    Gls,
    Subfield,
    Fp4,
    /// `PrimeExplicit` p = 4999, order 5006 = 2 · 2503: cofactor two, so the
    /// cofactor multiplication of a hashed public point is a real doubling.
    Cofactor2,
}

fn spec(fx: Fx) -> CurveSpec {
    match fx {
        Fx::Prime18 => CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        },
        Fx::Aps101 => CurveSpec::PrimeExplicit {
            p: 101,
            a: 59,
            b: 6,
            group_order: 107,
            r: 107,
            gx: 78,
            gy: 4,
        },
        Fx::Scholten => CurveSpec::ThreeLeads {
            p: 739,
            n: 2,
            modulus: 1,
            a: 493102,
            b: 193650,
            group_order: 546352,
            r: 34147,
            gx: 433246,
            gy: 59741,
            family: "scholten".into(),
            parameter: 0,
        },
        Fx::TlSubfield => CurveSpec::ThreeLeads {
            p: 2,
            n: 6,
            modulus: 67,
            a: 3,
            b: 3,
            group_order: 56,
            r: 7,
            gx: 1,
            gy: 58,
            family: "subfield".into(),
            parameter: 3,
        },
        Fx::Cofactor2 => CurveSpec::PrimeExplicit {
            p: 4999,
            a: 1,
            b: 10,
            group_order: 5006,
            r: 2503,
            gx: 3060,
            gy: 144,
        },
        Fx::Prime14 => CurveSpec::PrimeSearch { bits: 14, seed: 5 },
        Fx::Prime10 => CurveSpec::PrimeSearch {
            bits: 10,
            seed: 59297,
        },
        Fx::J0 => j0(),
        Fx::J1728 => j1728(),
        Fx::K0n19 => CurveSpec::Koblitz { a: 0, n: 19 },
        Fx::K1n17 => CurveSpec::Koblitz { a: 1, n: 17 },
        Fx::K0n7 => CurveSpec::Koblitz { a: 0, n: 7 },
        Fx::Binary11 => CurveSpec::BinaryExplicit {
            n: 11,
            modulus: 2323,
            a: 1,
            b: 1,
            group_order: 1982,
            r: 991,
            gx: 1193,
            gy: 1007,
        },
        Fx::F2m4 => CurveSpec::F2m4Torsion {
            m: 5,
            seed: 1,
            max_cofactor: 16,
        },
        Fx::Genus2 => CurveSpec::Genus2Search {
            p_bits: 8,
            seed: 3,
            max_cofactor: 16,
        },
        Fx::Ghs => CurveSpec::GhsFp6Search { p: 13, seed: 1 },
        Fx::Hyper => CurveSpec::HyperellipticThread { p: 23, genus: 3 },
        Fx::Gls => CurveSpec::GlsFp2 {
            p_bits: 14,
            seed: 1,
            max_cofactor: 64,
        },
        Fx::Subfield => CurveSpec::SubfieldFp3 {
            p_bits: 14,
            seed: 1,
            max_cofactor_ratio: 64,
        },
        Fx::Fp4 => CurveSpec::Fp4Translation { p: 29, seed: 1 },
    }
}

type P = &'static [(&'static str, &'static str)];

const IC_PRIME: P = &[
    ("factor_base", "prime-abscissa:size=32"),
    ("oracle", "mitm:negation_folded=1"),
];
const IC_BINARY: P = &[
    ("factor_base", "koblitz-orbit:divisor=0;1"),
    ("oracle", "mitm-frobenius:m=2"),
];
const WWALK: P = &[
    ("factor_base", "prime-abscissa:size=2"),
    ("oracle", "wwalk:x=1,g=1,grow=2,seed=run"),
];
const WSUM: P = &[
    ("factor_base", "prime-abscissa:size=3"),
    ("oracle", "wsum:coef=3"),
];
const INTERVAL: P = &[
    ("factor_base", "prime-interval:t=6"),
    ("oracle", "mitm:negation_folded=1"),
];
const TL_RHO: P = &[("trial_cap", "2000")];
const TL_GENUS2: P = &[
    ("trial_cap", "2000"),
    ("independent", "1"),
    ("unique_relations", "1"),
];
const TL_DECOMP: P = &[
    ("trial_cap", "2000"),
    ("oracle", "subtract"),
    ("random_base", "0"),
    ("base_seed", "17"),
];
const CLAW: P = &[("base_scale", "4.0"), ("table_scale", "1.0")];
const MODEL: P = &[
    ("jumps", "16"),
    ("walks", "8"),
    ("dp_bits", "2"),
    ("cap_multiple", "10000"),
];
const BATCH: P = &[("batch", "4")];
const PENCIL: P = &[
    ("small_denominator", "2"),
    ("matrix_coefficient_cap", "1000000"),
];
const APS: P = &[
    ("encoding", "line3"),
    ("roots", "exact"),
    ("stop", "first_informative"),
    ("labels", "guaranteed"),
    ("subset_size", "2"),
    ("attempts", "8"),
    ("degree", "24"),
    ("budget_ms", "10000"),
    ("sampling_draws", "1000"),
    ("orbit", "none"),
    ("beta", "0"),
    ("lambda", "0"),
    ("prune", "false"),
];
const LARGE_PRIME: P = &[
    ("small_dimension", "3"),
    ("envelope_dimension", "4"),
    ("summands", "n-1"),
    ("large_primes", "2"),
    ("max_trials", "20000"),
    ("max_states", "100000"),
];

/// Every (id, fixture, parameters) that is run.  An id may be listed on
/// several fixtures.
const CASES: &[(&str, Fx, P)] = &[
    ("rho.three_leads_negation_v1", Fx::Scholten, TL_RHO),
    ("rho.three_leads_plain_v1", Fx::Scholten, TL_RHO),
    ("rho.three_leads_jacobian_v1", Fx::Scholten, TL_RHO),
    (
        "ic.three_leads_genus2_pencil_symbolic_v2",
        Fx::Scholten,
        PENCIL,
    ),
    (
        "ic.three_leads_genus2_pencil_symbolic_v3",
        Fx::Scholten,
        PENCIL,
    ),
    ("ic.three_leads_genus2_choice2_v1", Fx::Scholten, TL_RHO),
    (
        "ic.three_leads_genus2_projective_chart_v1",
        Fx::Scholten,
        TL_RHO,
    ),
    ("ic.three_leads_genus2_target_rank_v1", Fx::Scholten, TL_RHO),
    ("ic.three_leads_genus2_v1", Fx::Scholten, TL_GENUS2),
    ("ic.three_leads_genus2_v2", Fx::Scholten, TL_GENUS2),
    ("ic.three_leads_decomposition_v1", Fx::TlSubfield, TL_DECOMP),
    ("ic.three_leads_decomposition_v2", Fx::TlSubfield, TL_DECOMP),
    ("rho.frozen_reference", Fx::Prime18, &[]),
    ("rho.plain", Fx::Prime18, &[]),
    ("rho.negation", Fx::Prime18, &[]),
    ("rho.negation_v2", Fx::Prime18, &[]),
    ("rho.negation_v3", Fx::Prime18, &[]),
    ("rho.plain_v3", Fx::Prime18, &[]),
    ("rho.negation_v4", Fx::Prime18, &[]),
    ("rho.plain_v4", Fx::Prime18, &[]),
    ("bsgs.textbook", Fx::Prime18, &[]),
    ("bsgs.interleaved", Fx::Prime18, &[]),
    ("bsgs.negation", Fx::Prime18, &[]),
    ("bsgs.negation_table", Fx::Prime18, &[]),
    ("bsgs.textbook_v2", Fx::Prime18, &[]),
    ("bsgs.interleaved_v2", Fx::Prime18, &[]),
    ("bsgs.negation_v2", Fx::Prime18, &[]),
    ("kangaroo.vow", Fx::Prime18, &[]),
    ("kangaroo.vow_v2", Fx::Prime18, &[]),
    ("rho.plus_model_v1", Fx::Prime18, MODEL),
    ("rho.best2_model_v1", Fx::Prime18, MODEL),
    ("rho.multitarget_pooled", Fx::Prime18, BATCH),
    ("ic.residual_batch", Fx::Prime18, BATCH),
    ("ic.residual_batch_v2", Fx::Prime18, BATCH),
    ("ic.residual_batch_v2", Fx::Cofactor2, BATCH),
    ("rho.preprocessing", Fx::Prime18, &[("entries", "64")]),
    ("ic.pipeline", Fx::Prime18, IC_PRIME),
    ("ic.pipeline", Fx::Prime18, WWALK),
    ("ic.pipeline", Fx::Prime18, WSUM),
    ("ic.pipeline", Fx::Prime18, INTERVAL),
    ("ic.pipeline_counted", Fx::Prime18, IC_PRIME),
    (
        "ic.known_log_mitm2_v1",
        Fx::Prime18,
        &[
            ("size", "8"),
            ("base_seed", "1"),
            ("max_pair_work", "1000"),
            ("max_trials", "100000"),
        ],
    ),
    (
        "ic.known_log_ksum_v1",
        Fx::Prime18,
        &[
            ("size", "12"),
            ("k", "3"),
            ("base_seed", "1"),
            ("max_sums", "100000"),
            ("max_trials", "100000"),
        ],
    ),
    ("ic.tensor_contraction_v1", Fx::Prime10, &[("m", "3")]),
    (
        "ic.tensor_contraction_cofactor_v1",
        Fx::Cofactor2,
        &[("m", "2")],
    ),
    (
        "ic.prime_orbit_relations_v1",
        Fx::Prime18,
        &[("coordinates", "affine")],
    ),
    ("ic.aps_v1", Fx::Aps101, APS),
    ("ic.aps_v2", Fx::Aps101, APS),
    ("transfer.isogeny", Fx::Prime14, &[("ell", "3")]),
    ("transfer.quartic_cover", Fx::Prime14, &[]),
    ("transfer.quartic_cover_v2", Fx::Prime14, &[]),
    ("rho.cubic", Fx::J0, &[]),
    ("rho.cubic_v2", Fx::J0, &[]),
    ("rho.quartic", Fx::J1728, &[]),
    ("rho.quartic_v2", Fx::J1728, &[]),
    ("rho.signed_frobenius", Fx::K0n19, &[]),
    ("rho.signed_frobenius_v2", Fx::K0n19, &[]),
    ("rho.signed_frobenius_v3", Fx::K0n19, &[]),
    ("rho.signed_frobenius_strong", Fx::K0n19, &[]),
    ("rho.signed_frobenius_strong_legacy", Fx::K0n19, &[]),
    ("rho.signed_frobenius_strong_escape", Fx::K0n19, &[]),
    ("rho.signed_frobenius_strong_v2", Fx::K0n19, &[]),
    ("claw.pair_table", Fx::K0n19, CLAW),
    ("claw.pair_table_v2", Fx::K0n19, CLAW),
    ("ic.pipeline", Fx::K1n17, IC_BINARY),
    (
        "ic.koblitz_descent",
        Fx::K1n17,
        &[
            ("descent_summands", "2"),
            ("collection_window", "64"),
            ("factor_base_points", "256"),
            ("unit_trials", "64"),
            ("units", "1"),
            ("max_trials", "100000"),
            ("seed", "7"),
        ],
    ),
    (
        "ic.koblitz_batched",
        Fx::K1n17,
        &[("factor_base_points", "102")],
    ),
    (
        "ic.shared_rank",
        Fx::K1n17,
        &[
            (
                "factor_base",
                "compact-orbit-scan:columns=4,raw_x_cap=10000",
            ),
            ("rank_seed", "7"),
            ("rank_max_trials", "10000"),
            ("target_max_attempts", "64"),
        ],
    ),
    ("jacobian_image.rho3", Fx::K0n7, &[]),
    ("jacobian_image.bsgs", Fx::K0n7, &[]),
    ("jacobian_image.rho3_field_tally", Fx::K0n7, &[]),
    ("jacobian_image.bsgs_field_tally", Fx::K0n7, &[]),
    ("jacobian_image.rho3", Fx::Binary11, &[]),
    ("jacobian_image.bsgs", Fx::Binary11, &[]),
    ("jacobian_image.rho3_field_tally", Fx::Binary11, &[]),
    ("jacobian_image.bsgs_field_tally", Fx::Binary11, &[]),
    ("ic.large_prime", Fx::Binary11, LARGE_PRIME),
    ("ic.large_prime_v2", Fx::Binary11, LARGE_PRIME),
    (
        "ic.known_log_mitm2_v1",
        Fx::K0n19,
        &[
            ("size", "8"),
            ("base_seed", "1"),
            ("max_pair_work", "1000"),
            ("max_trials", "100000"),
        ],
    ),
    (
        "rho.negation_strong",
        Fx::F2m4,
        &[("lanes", "8"), ("dp_bits", "auto")],
    ),
    ("ic.hyperelliptic", Fx::Genus2, &[]),
    ("ic.hyperelliptic_v2", Fx::Genus2, &[]),
    ("transfer.ghs_cover", Fx::Ghs, &[]),
    (
        "ic.hyperelliptic_thread",
        Fx::Hyper,
        &[
            ("fb_size", "all"),
            ("large_primes", "0"),
            ("extra_relations", "8"),
            ("branches", "64"),
            ("d_step_in", "4"),
        ],
    ),
    ("rho.gls", Fx::Gls, &[]),
    ("rho.frobenius_orbit", Fx::Subfield, &[]),
    (
        "ic.k4_gaudry",
        Fx::Fp4,
        &[("base", "xline"), ("large_primes", "0"), ("c4", "97")],
    ),
];

/// Ids that are not run, each with why.  Only an external backend or the
/// lack of any instance small enough for a unit test belongs here.
const SKIPS: &[(&str, &str)] = &[
    ("ic.cm_folded_native_v1", "needs the external native backend binary"),
    ("ic.cm_folded_recurrence_v1", "needs the external native backend binary"),
    ("ic.cm_folded_native_d43_v1", "needs the external native backend binary"),
    ("ic.cm_folded_native_v2", "needs the external native backend binary"),
    ("ic.cm_folded_recurrence_v2", "needs the external native backend binary"),
    ("ic.cm_folded_native_d43_v2", "needs the external native backend binary"),
    ("ic.pipeline_full", "reprices a recorded ic.pipeline session (price-units, full_price); no solve of its own"),
    ("ic.pipeline_wordram", "reprices a recorded ic.pipeline session (price-units, wordram_price); no solve of its own"),
    ("rho.signed_frobenius_full", "reprices a recorded session (full_price)"),
    ("rho.signed_frobenius_v2_full", "reprices a recorded session (full_price)"),
    ("rho.signed_frobenius_strong_full", "reprices a recorded session (full_price)"),
    ("rho.signed_frobenius_wordram", "reprices a recorded session (wordram_price)"),
    ("rho.signed_frobenius_strong_wordram", "reprices a recorded session (wordram_price)"),
    ("ic.trace_line_2lp", "needs a binary curve with n = 4d on a trace-selected subfield line; no instance small enough to build in a unit test"),
    ("cheon.pminus1_comb", "DLP with auxiliary inputs: needs the runner's static-DH oracle and a curve with d | n-1 or n+1, so a plain solve refuses; covered by tests/ecbench_cheon.rs"),
    ("cheon.pminus1_naive", "DLP with auxiliary inputs: needs the runner's static-DH oracle and a curve with d | n-1 or n+1, so a plain solve refuses; covered by tests/ecbench_cheon.rs"),
    ("cheon.pplus1_comb", "DLP with auxiliary inputs: needs the runner's static-DH oracle and a curve with d | n-1 or n+1, so a plain solve refuses; covered by tests/ecbench_cheon.rs"),
];

/// Frozen ids whose phases do not sum to their total on purpose, each with
/// the id that carries the fix.  Run like a case; the sum must still fail.
const EXEMPT: &[(&str, &str)] = &[];

/// Work that the Koblitz index-calculus adapters declare unpriced by name
/// without counting it (field arithmetic outside the group law, hashing,
/// the descent's modular combination, the host-dependent vector decline
/// count): honest declarations, not counters.
const UNCOUNTED_NAMES: &[&str] = &[
    "field_arithmetic_uncharged",
    "hash_and_allocation_uncharged",
    "modular_combination_uncharged",
    "vector_declined_lanes_uncharged",
];

/// Methods whose development caps may end a run unsolved on these fixtures
/// (their records keep the exhaustion), or whose answer depends on a wall
/// deadline: the recovered logarithm is not asserted, the accounting is.
fn may_exhaust(id: &str) -> bool {
    id.contains(".three_leads_") || crate::cryptanalysis::ecbench::aps::is_aps(id)
}

/// Frozen ids whose `unpriced` list does not meet rule 3 (counter names, one
/// for every `_uncharged` counter with a count), each with why it stays: a
/// corrected list is a change to a frozen id's record, which README §9
/// reserves for a new id.  The entry is checked like an exemption: it fails
/// once the id meets the rule.
const UNPRICED_EXEMPT: &[(&str, &str)] = &[
    (
        "ic.aps_v1",
        "one prose entry beside two counter names (\"APS core group/field operations, ... not completely counted\")",
    ),
    (
        "ic.three_leads_genus2_pencil_symbolic_v2",
        "names three work labels that are no counter and omits the counted group_field_adds_uncharged and group_bit_ops_uncharged",
    ),
];

/// Rule 2: the phase that holds the final `[d]G = Q` check, which must
/// charge something inside the total (a recovered run performed the check).
/// `jacobian_image.*` is absent: it converts no operation to GAE at all
/// (its check is counted in `jacobian_group_operations_uncharged`).
const CHECK_PHASE: &[(&str, &str)] = &[
    ("ic.known_log_ksum_v1", "target_recovery_check"),
    ("ic.known_log_mitm2_v1", "target_recovery_check"),
    ("ic.tensor_contraction_v1", "recovery_check"),
    ("ic.tensor_contraction_cofactor_v1", "recovery_check"),
    ("ic.residual_batch", "internal_verification"),
    ("ic.residual_batch_v2", "internal_verification"),
    ("rho.multitarget_pooled", "internal_verification"),
    ("rho.preprocessing", "internal_verification"),
    ("rho.negation_strong", "check"),
    ("ic.koblitz_batched", "verify"),
];

fn phase_gap(rep: &SolveReport) -> f64 {
    let sum: f64 = rep.phases.iter().map(|p| p.gae).sum();
    (sum - rep.total_gae).abs() / sum.abs().max(rep.total_gae.abs()).max(1.0)
}

#[test]
fn every_registered_method_is_run_skipped_or_exempt_and_its_phases_sum() {
    let ids: Vec<&str> = registry().iter().map(|d| d.id).collect();
    let mut problems: Vec<String> = Vec::new();
    // Each id is exactly one of case, skip; exemptions are cases.
    for id in &ids {
        let cases = CASES.iter().filter(|c| c.0 == *id).count();
        let skips = SKIPS.iter().filter(|s| s.0 == *id).count();
        match (cases, skips) {
            (0, 0) => problems.push(format!(
                "{id}: registered but neither a case nor a skip (add it to CASES, or to SKIPS with a reason)"
            )),
            (c, s) if c > 0 && s > 0 => {
                problems.push(format!("{id}: both a case ({c}) and a skip ({s})"))
            }
            _ => {}
        }
    }
    for (id, _, _) in CASES {
        if decl(id).is_none() {
            problems.push(format!("{id}: a case, but not registered"));
        }
    }
    for (id, why) in SKIPS.iter().chain(EXEMPT).chain(UNPRICED_EXEMPT) {
        if decl(id).is_none() {
            problems.push(format!("{id}: named ({why}) but not registered"));
        }
        if why.is_empty() {
            problems.push(format!("{id}: no reason recorded"));
        }
    }
    for (id, _) in EXEMPT {
        if !CASES.iter().any(|c| c.0 == *id) {
            problems.push(format!("{id}: exempt but has no case to run"));
        }
    }

    let mut built: BTreeMap<String, (Instance, CurveFacts)> = BTreeMap::new();
    for (id, fx, params) in CASES {
        let tag = format!("{id} on {fx:?}");
        let s = spec(*fx);
        let key = format!("{fx:?}");
        if !built.contains_key(&key) {
            let inst = match s.build() {
                Ok(i) => i,
                Err(e) => {
                    problems.push(format!("{tag}: the fixture does not build: {e}"));
                    continue;
                }
            };
            let facts = inst.facts(&s);
            built.insert(key.clone(), (inst, facts));
        }
        let (inst, facts) = &built[&key];
        let m = match resolve(&MethodSpec {
            id: (*id).into(),
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }) {
            Ok(m) => m,
            Err(e) => {
                problems.push(format!("{tag}: does not resolve: {e}"));
                continue;
            }
        };
        if let Err(e) = applicable(&m, inst, facts) {
            problems.push(format!("{tag}: refused at plan time: {e}"));
            continue;
        }
        let k = 1 + 4242 % (facts.r - 1);
        let Some(target) = inst.mul_generator_hex(k) else {
            problems.push(format!("{tag}: no target for k = {k}"));
            continue;
        };
        let rep = match solve(&m, inst, facts, &target, 6) {
            Ok(r) => r,
            Err(e) => {
                problems.push(format!(
                    "{tag}: applicable, but solve errors at run time: {e}"
                ));
                continue;
            }
        };
        // Transfers and walks recover the planted log on these fixtures.
        if rep.recovered != Some(ExactScalar::from(k)) && !may_exhaust(id) {
            problems.push(format!("{tag}: recovered {:?}, planted {k}", rep.recovered));
        }
        let gap = phase_gap(&rep);
        let exempt = EXEMPT.iter().any(|e| e.0 == *id);
        if gap > 1e-9 && !exempt {
            problems.push(format!(
                "{tag}: phases sum to {} but total_gae is {} ({:?})",
                rep.phases.iter().map(|p| p.gae).sum::<f64>(),
                rep.total_gae,
                rep.phases
                    .iter()
                    .map(|p| (p.name.as_str(), p.gae))
                    .collect::<Vec<_>>()
            ));
        }
        if gap <= 1e-9 && exempt {
            problems.push(format!(
                "{tag}: exempt, but its phases now sum to the total: remove it from EXEMPT"
            ));
        }
        if let Some((_, name)) = CHECK_PHASE.iter().find(|c| c.0 == *id) {
            let charged = rep.phases.iter().find(|p| p.name == *name).map(|p| p.gae);
            if !charged.is_some_and(|g| g > 0.0) {
                problems.push(format!(
                    "{tag}: the final-check phase `{name}` is missing or charges nothing ({charged:?})"
                ));
            }
        }
        // Rule 3: `unpriced` names counters.  A name is a counter of the
        // report, a native counter of a phase, or the `_uncharged` form of
        // one (the pipeline family derives its list that way); the work
        // counted nowhere is declared by name in `UNCOUNTED_NAMES`.  An
        // `_uncharged` counter with a count must be listed, so the record
        // is a lower bound.
        let mut rule3: Vec<String> = Vec::new();
        for name in &rep.unpriced {
            let stem = name.strip_suffix("_uncharged").unwrap_or(name);
            let named = rep.counters.contains_key(name)
                || rep.phases.iter().any(|p| {
                    p.native.contains_key(name)
                        || p.work_native.contains_key(name)
                        || p.native.contains_key(stem)
                        || p.work_native.contains_key(stem)
                })
                || UNCOUNTED_NAMES.contains(&name.as_str());
            if !named {
                rule3.push(format!(
                    "unpriced entry `{name}` is neither a counter nor a phase's native counter"
                ));
            }
        }
        let unlisted: Vec<&String> = rep
            .counters
            .iter()
            .filter(|(k, v)| k.ends_with("_uncharged") && **v > 0 && !rep.unpriced.contains(k))
            .map(|(k, _)| k)
            .collect();
        if !unlisted.is_empty() {
            rule3.push(format!(
                "`_uncharged` counters missing from `unpriced`: {unlisted:?}"
            ));
        }
        let exempt3 = UNPRICED_EXEMPT.iter().any(|e| e.0 == *id);
        if exempt3 && rule3.is_empty() {
            problems.push(format!(
                "{tag}: exempt from rule 3, but its `unpriced` is now in order: remove it from UNPRICED_EXEMPT"
            ));
        }
        if !exempt3 {
            problems.extend(rule3.into_iter().map(|p| format!("{tag}: {p}")));
        }
    }
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

/// Rule 4 on the audited methods: on every small curve of their family, an
/// arm the planner accepts (`applicable`) must not error when it runs, for
/// several targets.  (A refusal is always fine; the other half of the rule,
/// that what the method handles is accepted, is the cases above.)
#[test]
fn what_the_planner_accepts_the_new_methods_run_without_error() {
    let primes: Vec<CurveSpec> = (8..=11u32)
        .flat_map(|bits| (1..=2u64).map(move |seed| CurveSpec::PrimeSearch { bits, seed }))
        .collect();
    let koblitz: Vec<CurveSpec> = [3u32, 5, 7, 9, 11, 13]
        .into_iter()
        .flat_map(|n| [0u8, 1].map(move |a| CurveSpec::Koblitz { a, n }))
        .collect();
    let f2m4: Vec<CurveSpec> = (4..=7u32)
        .flat_map(|m| {
            (1..=2u64).map(move |seed| CurveSpec::F2m4Torsion {
                m,
                seed,
                max_cofactor: 16,
            })
        })
        .collect();
    let batched: &[(&str, &str)] = &[("factor_base_points", "36")];
    let sweeps: Vec<(&str, P, &Vec<CurveSpec>)> = vec![
        ("jacobian_image.rho3", &[], &koblitz),
        ("jacobian_image.bsgs", &[], &koblitz),
        ("jacobian_image.rho3_field_tally", &[], &koblitz),
        ("jacobian_image.bsgs_field_tally", &[], &koblitz),
        ("ic.koblitz_batched", batched, &koblitz),
        (
            "rho.negation_strong",
            &[("lanes", "4"), ("dp_bits", "auto")],
            &f2m4,
        ),
        (
            "rho.negation_strong",
            &[("lanes", "1"), ("dp_bits", "2")],
            &f2m4,
        ),
        ("rho.multitarget_pooled", &[("batch", "3")], &primes),
        ("ic.residual_batch", &[("batch", "3")], &primes),
        ("rho.preprocessing", &[("entries", "16")], &primes),
        (
            "rho.preprocessing",
            &[("entries", "16"), ("dp_bits", "1")],
            &primes,
        ),
        ("ic.tensor_contraction_v1", &[("m", "3")], &primes),
        (
            "ic.known_log_mitm2_v1",
            &[
                ("size", "6"),
                ("base_seed", "1"),
                ("max_pair_work", "1000"),
                ("max_trials", "100000"),
            ],
            &primes,
        ),
        (
            "ic.known_log_ksum_v1",
            &[
                ("size", "6"),
                ("k", "3"),
                ("base_seed", "1"),
                ("max_sums", "100000"),
                ("max_trials", "100000"),
            ],
            &primes,
        ),
        (
            "ic.known_log_ksum_v1",
            &[
                ("size", "9"),
                ("k", "5"),
                ("base_seed", "2"),
                ("base_multiplier", "3"),
                ("max_sums", "100000"),
                ("max_trials", "100000"),
            ],
            &primes,
        ),
        ("ic.pipeline", WWALK, &primes),
        ("ic.pipeline", WSUM, &primes),
        ("ic.pipeline", INTERVAL, &primes),
    ];
    let mut problems = Vec::new();
    let mut ran = 0usize;
    for (id, params, curves) in sweeps {
        let started = std::time::Instant::now();
        let m = resolve(&MethodSpec {
            id: id.into(),
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        })
        .unwrap();
        for s in curves {
            let Ok(inst) = s.build() else { continue };
            let facts = inst.facts(s);
            if applicable(&m, &inst, &facts).is_err() {
                continue;
            }
            for k in [1u128, 2, facts.r / 2, facts.r - 1] {
                if k == 0 || k >= facts.r {
                    continue;
                }
                let Some(target) = inst.mul_generator_hex(k) else {
                    continue;
                };
                ran += 1;
                match solve(&m, &inst, &facts, &target, 3) {
                    Err(e) => problems.push(format!(
                        "{id} {params:?} on {} k={k}: accepted at plan time, errors at run time: {e}",
                        facts.slug
                    )),
                    Ok(rep) => {
                        if phase_gap(&rep) > 1e-9 {
                            problems.push(format!(
                                "{id} on {} k={k}: phases do not sum to the total",
                                facts.slug
                            ));
                        }
                    }
                }
            }
        }
        eprintln!(
            "sweep {id} {params:?}: {} ms",
            started.elapsed().as_millis()
        );
    }
    assert!(ran > 100, "the sweep ran only {ran} solves");
    assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
}

// ---------------------------------------------------------------------------
// The successor ids of the `unpriced` audit (README §9, "Unpriced-list ids").
//
// `ic.aps_v1`, `ic.three_leads_genus2_pencil_symbolic_v2` and
// `ic.residual_batch` have committed sessions, so their records are frozen and
// named in `UNPRICED_EXEMPT` / left as they are.  Each successor is a case
// above (no exemption) and has a test here that the frozen id still produces
// exactly its old record shape and that `successor − frozen` is exactly the
// intended difference.
// ---------------------------------------------------------------------------

fn run_report(id: &str, fx: Fx, params: P) -> SolveReport {
    let s = spec(fx);
    let inst = s.build().unwrap();
    let facts = inst.facts(&s);
    let m = resolve(&MethodSpec {
        id: id.into(),
        params: params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    })
    .unwrap();
    applicable(&m, &inst, &facts).unwrap();
    let k = 1 + 4242 % (facts.r - 1);
    let target = inst.mul_generator_hex(k).unwrap();
    let mut rep = solve(&m, &inst, &facts, &target, 6).unwrap();
    rep.solve_wall_ns = 0;
    for p in &mut rep.phases {
        p.wall_ns = None;
    }
    rep
}

fn shape(p: &PhaseRecord) -> (String, u64, u64, u64, f64) {
    (p.name.clone(), p.adds, p.doubles, p.scalar_mults, p.gae)
}

fn keys(m: &BTreeMap<String, u64>) -> Vec<&str> {
    m.keys().map(String::as_str).collect()
}

/// The frozen `ic.aps_v1` `unpriced` prose entry, verbatim.
const APS_V1_PROSE: &str = "APS core group/field operations, polynomial construction, allocation and hashing are not completely counted; absent solver reports leave diagnostic counts unknown. No complete S or bitop ranking is supported";

const APS_V2_NEW: [&str; 5] = [
    "aps_core_combines_uncharged",
    "aps_core_expression_multiplications_uncharged",
    "aps_core_point_adds_uncharged",
    "aps_core_scalar_mults_uncharged",
    "aps_core_square_roots_uncharged",
];

#[test]
fn aps_v2_adds_exactly_the_core_counters_and_names_only_counters() {
    let a = run_report("ic.aps_v1", Fx::Aps101, APS);
    let b = run_report("ic.aps_v2", Fx::Aps101, APS);

    // The frozen id keeps its record: seven counters, the prose entry, v1.
    assert_eq!(
        a.unpriced,
        [
            "aps_f4_multiplications_uncharged",
            "aps_root_multiplications_uncharged",
            APS_V1_PROSE
        ]
    );
    assert_eq!(
        keys(&a.counters),
        [
            "aps_attempts",
            "aps_deadline_attempts",
            "aps_f4_multiplications_uncharged",
            "aps_failed_attempts",
            "aps_legacy_root_count_unknown_attempts",
            "aps_root_multiplications_uncharged",
            "aps_unreported_algebra_attempts",
        ]
    );
    assert_eq!(a.detail["schema"], "ecbench.aps/v1");
    assert!(a.detail.get("uncounted_work").is_none());

    // v2 − frozen in the counters: exactly the five core counters.
    let mut rest = b.counters.clone();
    for name in APS_V2_NEW {
        assert!(rest.remove(name).is_some(), "{name} missing from v2");
    }
    assert_eq!(rest, a.counters);
    // They count real work on this fixture (the others are 0 here or not).
    for name in [
        "aps_core_scalar_mults_uncharged",
        "aps_core_combines_uncharged",
        "aps_core_square_roots_uncharged",
        "aps_core_expression_multiplications_uncharged",
    ] {
        assert!(b.counters[name] > 0, "{name} counted nothing");
    }
    // Everything charged is identical: recovery, total, phases (with native).
    assert_eq!(a.recovered, b.recovered);
    assert_eq!(a.total_gae, b.total_gae);
    assert_eq!(a.phases.len(), b.phases.len());
    for (pa, pb) in a.phases.iter().zip(&b.phases) {
        assert_eq!(shape(pa), shape(pb));
        assert_eq!(pa.native, pb.native);
    }
    // Rule 3, stated directly: counter names only, none of them prose.
    assert_eq!(
        b.unpriced,
        [
            "aps_core_combines_uncharged",
            "aps_core_expression_multiplications_uncharged",
            "aps_core_point_adds_uncharged",
            "aps_core_scalar_mults_uncharged",
            "aps_core_square_roots_uncharged",
            "aps_f4_multiplications_uncharged",
            "aps_legacy_root_count_unknown_attempts",
            "aps_root_multiplications_uncharged",
            "aps_unreported_algebra_attempts",
        ]
    );
    for name in &b.unpriced {
        assert!(b.counters.contains_key(name), "{name} is not a counter");
    }
    assert!(!b.unpriced.iter().any(|n| n.contains(' ')));
    // What no counter can hold is a detail field, not an `unpriced` entry.
    assert_eq!(b.detail["schema"], "ecbench.aps/v2");
    assert_eq!(
        b.detail["uncounted_work"]["allocation_observer_available"],
        false
    );
    assert_eq!(b.detail["native_work_accounting_complete"], false);
    // The tallies are a function of the run, not of the thread's history.
    let c = run_report("ic.aps_v2", Fx::Aps101, APS);
    assert_eq!(b.counters, c.counters);
}

#[test]
fn aps_core_work_counts_the_calls_the_core_makes() {
    use crate::cryptanalysis::aps_single_relation as core;
    use crate::cryptanalysis::prime_fast::FastCurve;
    // p = 101, order 107, the instrument's development curve.
    let curve = FastCurve::new("t", 101, 59, 6, 107);
    let g = curve.point(78, 4);
    core::reset_core_work();
    // validate_instance: two subgroup-order checks, no other group call.
    core::validate_instance(&curve, g, g).unwrap();
    let w = core::core_work();
    assert_eq!(
        (w.scalar_mults, w.combines, w.point_adds, w.lifts),
        (2, 0, 0, 0)
    );
    // One more reset starts from zero.
    core::reset_core_work();
    assert_eq!(core::core_work(), core::CoreWork::default());
}

const PENCIL_V2_LABELS: [&str; 3] = [
    "pencil_native_and_symbolic_arithmetic",
    "pencil_logical_work",
    "allocation_observer_unavailable",
];

#[test]
fn pencil_v3_lifts_the_ledger_into_counters_and_names_only_counters() {
    let a = run_report(
        "ic.three_leads_genus2_pencil_symbolic_v2",
        Fx::Scholten,
        PENCIL,
    );
    let b = run_report(
        "ic.three_leads_genus2_pencil_symbolic_v3",
        Fx::Scholten,
        PENCIL,
    );

    // The frozen id keeps its record: three labels, five group counters and
    // the eight replay words, a `v2` detail schema.
    assert_eq!(a.unpriced, PENCIL_V2_LABELS);
    let frozen_counters: Vec<&str> = keys(&a.counters)
        .into_iter()
        .filter(|k| !k.contains("sha256_word"))
        .collect();
    assert_eq!(
        frozen_counters,
        [
            "group_bit_ops_uncharged",
            "group_field_adds_uncharged",
            "group_field_inversions",
            "group_field_multiplications",
            "group_field_squarings",
        ]
    );
    assert_eq!(a.detail["schema"], "ecbench-pencil-symbolic/v2");

    // v3 − frozen in the counters: only the lifted `pencil_*_uncharged` ones.
    let lifted = |k: &str| {
        ["pencil_native_", "pencil_symbolic_", "pencil_logical_"]
            .iter()
            .any(|p| k.starts_with(p))
    };
    let mut rest = b.counters.clone();
    rest.retain(|k, _| !lifted(k));
    assert_eq!(rest, a.counters);
    assert!(b
        .counters
        .keys()
        .filter(|k| lifted(k.as_str()))
        .all(|k| k.ends_with("_uncharged")));
    // Everything charged is identical.
    assert_eq!(a.recovered, b.recovered);
    assert_eq!(a.total_gae, b.total_gae);
    assert_eq!(a.phases.len(), b.phases.len());
    for (pa, pb) in a.phases.iter().zip(&b.phases) {
        assert_eq!(shape(pa), shape(pb));
    }

    // `unpriced` is exactly the `_uncharged` counters, each a real counter,
    // the two the frozen id omitted among them, the three labels gone.
    let uncharged: Vec<String> = b
        .counters
        .keys()
        .filter(|k| k.ends_with("_uncharged"))
        .cloned()
        .collect();
    assert_eq!(b.unpriced, uncharged);
    for name in ["group_field_adds_uncharged", "group_bit_ops_uncharged"] {
        assert!(b.unpriced.iter().any(|n| n == name), "{name} not listed");
    }
    for label in PENCIL_V2_LABELS {
        assert!(!b.unpriced.iter().any(|n| n == label));
        assert!(!b.counters.contains_key(label));
    }

    // The lifted counters are the ledger's own numbers.
    let candidate = &b.detail["candidate"];
    let arithmetic = candidate["arithmetic"].as_object().unwrap();
    assert!(!arithmetic.is_empty());
    for (k, v) in arithmetic {
        assert_eq!(
            b.counters[&format!("pencil_native_{k}_uncharged")],
            v.as_u64().unwrap(),
            "arithmetic {k}"
        );
    }
    let setup = b.detail["cold_setup"]["semantic_counts"]
        .as_object()
        .unwrap();
    let symbolic = candidate["symbolic"]["semantic_counts"]
        .as_object()
        .unwrap();
    assert_eq!(
        setup.keys().collect::<Vec<_>>(),
        symbolic.keys().collect::<Vec<_>>()
    );
    for (k, v) in symbolic {
        assert_eq!(
            b.counters[&format!("pencil_symbolic_{k}_uncharged")],
            v.as_u64().unwrap() + setup[k].as_u64().unwrap(),
            "semantic {k}"
        );
    }
    for k in ["pivot_lookups", "coefficient_updates", "submitted_rows"] {
        assert_eq!(
            b.counters[&format!("pencil_logical_{k}_uncharged")],
            candidate["symbolic"][k].as_u64().unwrap(),
            "logical {k}"
        );
    }
    assert_eq!(
        b.counters["pencil_logical_solver_nodes_uncharged"],
        candidate["solver_nodes"].as_u64().unwrap()
    );
    assert!(b.counters["pencil_symbolic_collection_entry_visits_uncharged"] > 0);
    assert!(b.counters["pencil_logical_solver_nodes_uncharged"] > 0);
    // No allocation observer: a detail field, not a counter name.
    assert_eq!(b.detail["schema"], "ecbench-pencil-symbolic/v3");
    assert_eq!(b.detail["allocation_observer_available"], false);
}

#[test]
fn residual_batch_v2_charges_the_base_point_derivation_and_nothing_else() {
    for (fx, charged) in [(Fx::Prime18, false), (Fx::Cofactor2, true)] {
        let a = run_report("ic.residual_batch", fx, BATCH);
        let b = run_report("ic.residual_batch_v2", fx, BATCH);
        let tag = format!("{fx:?}");

        // The frozen id keeps its three phases and no base-point counter.
        assert_eq!(
            a.phases.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
            ["setup", "search", "internal_verification"],
            "{tag}"
        );
        for name in [
            "base_point_hash_draws_uncharged",
            "base_point_on_curve_checks_uncharged",
            "base_point_sqrt_field_multiplications_uncharged",
        ] {
            assert!(!a.counters.contains_key(name), "{name} in the frozen id");
        }
        assert!(a.detail.get("base_point").is_some());

        // v2 − frozen: one leading phase, three counters, the phase's GAE.
        assert_eq!(
            b.phases.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
            ["base_point", "setup", "search", "internal_verification"],
            "{tag}"
        );
        for (pa, pb) in a.phases.iter().zip(&b.phases[1..]) {
            assert_eq!(shape(pa), shape(pb), "{tag}");
        }
        let base = &b.phases[0];
        assert_eq!(base.gae, (base.adds + base.doubles) as f64);
        assert_eq!(b.total_gae, a.total_gae + base.gae, "{tag}");
        assert_eq!(b.total_gae, b.phases.iter().map(|p| p.gae).sum::<f64>());
        assert_eq!(a.recovered, b.recovered);
        // The charge is the cofactor multiplication: nothing on a cofactor-one
        // curve (still a derivation, counted below), a real doubling at h = 2.
        assert_eq!(
            base.gae > 0.0,
            charged,
            "{tag}: base_point gae {}",
            base.gae
        );

        let new = [
            "base_point_hash_draws_uncharged",
            "base_point_on_curve_checks_uncharged",
            "base_point_sqrt_field_multiplications_uncharged",
        ];
        let mut rest = b.counters.clone();
        for name in new {
            assert!(rest.remove(name).is_some_and(|n| n > 0), "{tag}: {name}");
        }
        assert_eq!(rest, a.counters, "{tag}");
        let mut expected = a.unpriced.clone();
        expected.extend(new.map(String::from));
        expected.sort();
        assert_eq!(b.unpriced, expected, "{tag}");

        // The phase and counters are the derivation's own tally.
        let s = spec(fx);
        let inst = s.build().unwrap();
        let facts = inst.facts(&s);
        let mut work = HashWork::default();
        let counted = inst
            .hash_to_subgroup_counted(&facts, 6, (1 << 40) - 1, &mut work)
            .unwrap();
        assert_eq!(
            Some(counted.clone()),
            inst.hash_to_subgroup(&facts, 6, (1 << 40) - 1)
        );
        assert_eq!(
            (base.adds, base.doubles, base.scalar_mults),
            (
                work.group_ops.adds,
                work.group_ops.doubles,
                work.group_ops.scalar_mults
            )
        );
        assert_eq!(b.counters["base_point_hash_draws_uncharged"], work.draws);
        assert_eq!(
            b.counters["base_point_sqrt_field_multiplications_uncharged"],
            work.sqrt_field_muls
        );
        assert_eq!(b.detail["base_point"], serde_json::json!(counted.0));
    }
}

#[test]
fn counted_square_root_is_the_square_root_and_counts_its_multiplications() {
    use crate::cryptanalysis::ic_boundary::PrimeCurve;
    // 1019 and 4999 are 3 mod 4 (one exponentiation); 1009 and 4993 are 1 mod 4
    // (Tonelli–Shanks).
    for p in [1019u64, 4999, 1009, 4993] {
        let curve = PrimeCurve { p, a: 1, b: 10 };
        let mut total = 0u64;
        for n in 0..p {
            let mut muls = 0;
            let counted = curve.sqrt_counted(n, &mut muls);
            assert_eq!(counted, curve.sqrt(n), "p {p} n {n}");
            match counted {
                Some(r) => assert_eq!(r * r % p, n),
                None => assert!(n != 0),
            }
            if n != 0 {
                assert!(muls > 0, "p {p} n {n}");
            }
            total += muls;
        }
        // About log2 p squarings plus half as many multiplications each.
        assert!(total as f64 / p as f64 > 10.0, "p {p}: {total}");
    }
}
