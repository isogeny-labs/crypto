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
use crate::cryptanalysis::ecbench::workload::CurveSpec;

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
        &[
            ("small_denominator", "2"),
            ("matrix_coefficient_cap", "1000000"),
        ],
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
        "ic.prime_orbit_relations_v1",
        Fx::Prime18,
        &[("coordinates", "affine")],
    ),
    (
        "ic.aps_v1",
        Fx::Aps101,
        &[
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
        ],
    ),
    ("transfer.isogeny", Fx::Prime14, &[("ell", "3")]),
    ("transfer.quartic_cover", Fx::Prime14, &[]),
    ("transfer.quartic_cover_v2", Fx::Prime14, &[]),
    ("rho.cubic", Fx::J0, &[]),
    ("rho.cubic_v2", Fx::J0, &[]),
    ("rho.quartic", Fx::J1728, &[]),
    ("rho.quartic_v2", Fx::J1728, &[]),
    ("rho.signed_frobenius", Fx::K0n19, &[]),
    ("rho.signed_frobenius_v2", Fx::K0n19, &[]),
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
    id.contains(".three_leads_") || id == "ic.aps_v1"
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
    ("ic.residual_batch", "internal_verification"),
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
