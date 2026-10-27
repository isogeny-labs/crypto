//! The method registry: every ECDLP algorithm `ecbench` can run, behind
//! one interface, charged in one unit.
//!
//! A method is an id from [`registry`] plus string parameters.  Defaults
//! are written into the parameters before the method id is hashed, so an
//! omitted default and an explicit one are the same method, and an
//! unknown parameter is refused rather than ignored.
//!
//! Every method runs on the instance's [`CountedGroup`], so an addition
//! costs the same in every row: rho's walk step, a BSGS giant step, a
//! kangaroo jump and an index-calculus relation trial are all charged
//! through `GroupOps`.  Native work the unit does not price is counted
//! under a name ending `_uncharged` and listed in
//! [`SolveReport::unpriced`]; a report with any unpriced work is a lower
//! bound and says so.

use std::collections::BTreeMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::canonical::{derive_u64, sha256_hex, short_id};
use crate::cryptanalysis::ecbench::claw::{pair_claw, pair_claw_wide, ClawOutcome, ClawShape};
use crate::cryptanalysis::ecbench::f2m4_price::Mulmod;
use crate::cryptanalysis::ecbench::generic::{
    bsgs_interleaved, bsgs_negation, bsgs_textbook, kangaroo, GenericOutcome,
};
use crate::cryptanalysis::ecbench::workload::{CurveFacts, Instance, WideInstance};
use crate::cryptanalysis::ecbench_large_prime::{
    self as large_prime, SolveConfig as LargePrimeConfig,
};
use crate::cryptanalysis::gaudry_quartic::Pt4;
use crate::cryptanalysis::gaudry_quartic_bin::{
    fq_muls_per_add, fq_muls_per_add_ref, fq_muls_per_add_ref_poly, BinCurve, BinD4Oracle,
    BinOptions, OracleNative, PtB, TLineBase,
};
use crate::cryptanalysis::gaudry_quartic_d4::{
    fp_muls_per_add, unpack_e4, D4Curve, D4Options, D4Oracle, YLineBase,
};
use crate::cryptanalysis::ic_boundary::{
    price_phase, rho_cap, rho_reference, rho_walk_with, signed_frobenius_rho_tuned, BinaryGroup,
    BinaryInstance, Calibration, CountedGroup, CubicClasses, FieldOps, GroupOps, NegationClasses,
    PhaseCost, PointClasses, PrimeInstance, PrimePoint, QuarticClasses, RhoResult, RhoWalk,
};
use crate::cryptanalysis::ic_framework::fiber::{
    FiberDescentOracle, FiberFrobeniusMitmOracle, FiberMitmOracle,
};
use crate::cryptanalysis::ic_framework::plugins::{
    BinarySubspaceBase, CompactOrbitScanBase, CyclotomicBase, DescentAlgebraicOracle,
    FrobeniusMitmOracle, GlvOrbitBase, KoblitzOrbitBase, MitmOracle, PrimeAbscissaBase,
    PrimeBatchMitmOracle, RootfindSubtractOracle, SubtractOracle,
};
use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
use crate::cryptanalysis::ic_framework::rootfind::{RootfindMitmOracle, RootfindPairsOracle};
use crate::cryptanalysis::ic_framework::shared_rank::{
    run_shared_rank_targets, SharedRankSpec, SharedTargetSpec,
};
use crate::cryptanalysis::ic_framework::solvers::solver_by_name;
use crate::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, Targets,
};
use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase, Snapshot};
use crate::cryptanalysis::koblitz_fast::FastPoint;
use crate::cryptanalysis::koblitz_strong_rho::{
    RawPoint as StrongPoint, RawPointG, RhoField, RhoScalar, StrongRho, StrongRhoCharges,
    StrongRhoG, StrongRhoParams,
};
use rand::{rngs::StdRng, SeedableRng};

/// A parameter a method reads.  `default: None` means required: a value
/// that changes the search is never left to a producer default.
#[derive(Clone, Copy, Debug)]
pub struct ParamDecl {
    pub name: &'static str,
    pub default: Option<&'static str>,
    pub help: &'static str,
}

/// Which curves a method runs on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Applies {
    Any,
    KoblitzOnly,
    BinaryOnly,
}

/// One registered method.
#[derive(Clone, Copy, Debug)]
pub struct MethodDecl {
    pub id: &'static str,
    /// `rho`, `bsgs`, `kangaroo` or `ic`.
    pub family: &'static str,
    pub summary: &'static str,
    /// The code the method runs, for the record.
    pub entry: &'static str,
    pub applies: Applies,
    pub params: &'static [ParamDecl],
}

const RHO_PARAMS: &[ParamDecl] = &[ParamDecl {
    name: "cap_multiple",
    default: Some("64"),
    help: "step budget as a multiple of √r before the run counts as exhausted",
}];

/// `rho.negation_batched`: the unbatched walk's budget, plus the lockstep width.
const RHO_BATCHED_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "cap_multiple",
        default: Some("64"),
        help: "step budget as a multiple of √r before the run counts as exhausted",
    },
    ParamDecl {
        name: "walks",
        default: Some("256"),
        help: "walks advanced in lockstep; one Montgomery batch inversion per round",
    },
    ParamDecl {
        name: "dp_bits",
        default: Some("0"),
        help: "distinguished-point bits; 0 takes bits(r)/4 as the unbatched walk does",
    },
];

/// `ic.pipeline`'s parameters, shared by `ic.pipeline_f2m4_full`.
const IC_PIPELINE_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "factor_base",
        default: None,
        help: "plug-in spec: prime-abscissa:size=N, glv-orbit:size=N, binary-subspace:dimension=D, koblitz-orbit:divisor=1;2, compact-orbit-scan:columns=N,raw_x_cap=M, cyclotomic:w=W[,columns=N], koblitz-standard-subspace:dimension=D",
    },
    ParamDecl {
        name: "oracle",
        default: None,
        help: "subtract, mitm, mitm-frobenius, mitm-frobenius-counted or descent-algebraic, with :m=2|3; mitm-prime-batch:m=2 (prime folded table with batched inversions); rootfind-subtract:m=2|3 (prime curves: prime_fast's direct-subtraction root finding, one batched inversion a target); the cofactor-fiber-aware mitm-fiber:fiber=closed|lifts|blind, mitm-frobenius-fiber:fiber=closed|lifts|blind and descent-algebraic-fiber:fiber=combined|lifts|blind (ic_framework::fiber); or pdp3-koblitz:m=3,engine=inherited-f4|f6-ic,degree=D,node_budget=N, pdp3-koblitz:m=2|3,engine=rootfind (PairSumTable), pdp3-koblitz:m=3,engine=rootfind-window (cyclic ⌈|F|/3⌉ window), pdp3-koblitz:m=2|3,engine=rootfind-negfold (PairTable negation fold, ~half set-up), and pdp3-koblitz:m=3,engine=rootfind-window-negfold on koblitz-standard-subspace",
    },
    ParamDecl {
        name: "solver",
        default: Some(""),
        help: "descent-algebraic and descent-algebraic-fiber only: f4-f2, buchberger-f2, sat-cdcl, exhaustive (with :k=v options)",
    },
    ParamDecl {
        name: "linalg",
        default: Some("incremental-gauss"),
        help: "incremental-gauss (stop at target pin), incremental-gauss-full-rank (all columns independent), incremental-gauss-full-rank-checked (also point-check each base column), or structured-gauss",
    },
    ParamDecl {
        name: "targets",
        default: Some("walk"),
        help: "random ([a]G+[b]Q per trial), walk, or walk-prime-fast (same guarded stream using native Montgomery additions on prime curves)",
    },
    ParamDecl {
        name: "max_trials",
        default: Some("100000000"),
        help: "relation trials before the run counts as exhausted",
    },
    ParamDecl {
        name: "solver_budget_seconds",
        default: Some("0"),
        help: "per-call wall budget for an algebraic solver; nonzero makes the run nondeterministic",
    },
];

// Keep the defaults of the frozen `ic.pipeline_counted` sessions. The
// existing `ic.pipeline` declaration below retains main's newer options.
const IC_COUNTED_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "factor_base",
        default: None,
        help: "factor-base plug-in spec",
    },
    ParamDecl {
        name: "oracle",
        default: None,
        help: "decomposition oracle plug-in spec",
    },
    ParamDecl {
        name: "solver",
        default: Some(""),
        help: "algebraic solver plug-in spec",
    },
    ParamDecl {
        name: "linalg",
        default: Some("incremental-gauss"),
        help: "linear algebra plug-in",
    },
    ParamDecl {
        name: "targets",
        default: Some("walk"),
        help: "random, walk, or walk-prime-fast on single-word prime curves",
    },
    ParamDecl {
        name: "max_trials",
        default: Some("100000000"),
        help: "relation trial cap",
    },
    ParamDecl {
        name: "solver_budget_seconds",
        default: Some("0"),
        help: "per-call solver wall budget",
    },
];

/// Every method `ecbench` knows.  Adding one is an entry here and an arm
/// in [`solve`]; nothing else changes.
pub fn registry() -> &'static [MethodDecl] {
    &[
        MethodDecl {
            id: "rho.frozen_reference",
            family: "rho",
            summary: "the frozen r-adding walk every ledger row was priced against through §17 (A = 1); the historical \"before\" mark",
            entry: "ic_boundary::rho_reference",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.plain",
            family: "rho",
            summary: "tuned r-adding walk on points (A = 1), distinguished points, stride starts",
            entry: "ic_boundary::rho_walk_with(PointClasses, RhoWalk::plain())",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.negation",
            family: "rho",
            summary: "tuned walk on {P, −P} (A = 2) with Wiener–Zuccherato look-ahead and cycle escape",
            entry: "ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_batched",
            family: "rho",
            summary: "negation-map r-adding walk with distinguished points, many walks in lockstep with Montgomery batched inversion (one F_q^4 inversion per round, 3(n-1) extra multiplications), canonicalisation counted; priced in F_q operations at the curve's fq_muls_per_add; curves over F_(2^m)^4 only (one-word and u128 orders)",
            entry: "ic_wide::rho_negation_batched",
            applies: Applies::Any,
            params: RHO_BATCHED_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_batched_full",
            family: "rho",
            summary: "rho.negation_batched with every native counter priced in F_q-multiplication equivalents from src/cryptanalysis/ecbench/f2m4_full_prices.json (ecbench price-units-f2m4): canonicalisation, hashing, recent-point window, distinguished-point table, scalar arithmetic modulo r, walk bookkeeping; the same walk, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_batched + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_BATCHED_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_f2m4_full",
            family: "rho",
            summary: "rho.negation (the unbatched one-affine-inversion walk, one-word and u128) with every native counter priced in F_q-multiplication equivalents from src/cryptanalysis/ecbench/f2m4_full_prices.json: key, hash and negation evaluations, window, distinguished-point table, scalar arithmetic modulo r; the same walk as rho.negation, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_tallied + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_batched_ref",
            family: "rho",
            summary: "rho.negation_batched_full in the reference pricing: the walk's F_q operations are priced with Karatsuba/tower F_(q^4) arithmetic (gaudry_quartic_bin::RefCosts) and divided by the per-m reference affine addition (50 at odd m, 64 at even m, where u^2+u+1 is reducible: reference_pricing.reference_model records the model, the per-primitive costs and the divisor; reference_pricing.poly_free_const the polynomial-basis free-constant-multiplication model, divisor 42, beside it), native work by the same divisor; detail.reference_pricing carries the current-unit total and divisor side by side; the same walk, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_batched + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_BATCHED_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_f2m4_ref",
            family: "rho",
            summary: "rho.negation_f2m4_full with its native work divided by the per-m reference affine addition (50 F_q multiplications at odd m, 64 at even m, gaudry_quartic_bin::RefCosts; reference_pricing.poly_free_const records the 42-divisor second model) instead of the curve's schoolbook fq_muls_per_add; group operations are one GAE each in both; detail.reference_pricing carries the current-unit total and divisor side by side; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_tallied + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_batched_refmm",
            family: "rho",
            summary: "rho.negation_batched_ref and mod-r multiplications (scalar_mulmod, row_op, lp_mulmod) priced from a one- or two-limb Montgomery multiplication (f2m4_mulmod_prices.json, ecbench price-units-f2m4 --mulmod) at every m, so the unit has no step where the subgroup order crosses a word; detail.reference_pricing carries the same record with the code's mulmod beside it; the same run, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_batched + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_BATCHED_PARAMS,
        },
        MethodDecl {
            id: "rho.negation_f2m4_refmm",
            family: "rho",
            summary: "rho.negation_f2m4_ref and mod-r multiplications (scalar_mulmod, row_op, lp_mulmod) priced from a one- or two-limb Montgomery multiplication (f2m4_mulmod_prices.json, ecbench price-units-f2m4 --mulmod) at every m, so the unit has no step where the subgroup order crosses a word; detail.reference_pricing carries the same record with the code's mulmod beside it; the same run, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_wide::rho_negation_tallied + ecbench::f2m4_price",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.cubic",
            family: "rho",
            summary: "tuned walk on {±ζ₃^i(P)} (A = 6), the rational order-3 automorphism of a j = 0 curve with p ≡ 1 (mod 3) and 3 | r − 1; Wiener–Zuccherato look-ahead and cycle escape; refuses any curve without the structure",
            entry: "ic_boundary::CubicClasses with RhoWalk::negation()",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.quartic",
            family: "rho",
            summary: "tuned walk on {±ι^i(P)} (A = 4), the rational order-4 automorphism of a j = 1728 curve with p ≡ 1 (mod 4) and 4 | r − 1; Wiener–Zuccherato look-ahead and cycle escape; refuses any curve without the structure",
            entry: "ic_boundary::QuarticClasses with RhoWalk::negation()",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius",
            family: "rho",
            summary: "tuned walk on signed Frobenius orbits {±φ^t P} (A = 2n), Koblitz curves only",
            entry: "ic_boundary::signed_frobenius_rho_tuned",
            applies: Applies::KoblitzOnly,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong",
            family: "rho",
            summary: "the strong single-target reference of the IC measurement rules (docs/ic/boundary_targets.json, since 2026-10-01): lockstep walks with batched inversion on signed Frobenius orbits (A = 2n), Koblitz curves only",
            entry: "koblitz_strong_rho::StrongRho::solve",
            applies: Applies::KoblitzOnly,
            params: &[
                ParamDecl {
                    name: "lanes",
                    default: Some("32"),
                    help: "walks advanced in lockstep (batch-inversion width); the reference measured 32",
                },
                ParamDecl {
                    name: "dp_bits",
                    default: Some("4"),
                    help: "distinguished-point bits; the reference measured 4",
                },
                ParamDecl {
                    name: "step_cap_factor",
                    default: Some("2000"),
                    help: "give up after this multiple of the ideal step count",
                },
            ],
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong_escape",
            family: "rho",
            summary: "the strong reference with fruitless cycles escaped by doubling instead of abandoned: identical walks until a cycle, so long distinguished-point walks (the large dp_bits a table at m = 83 needs) survive; A = 2n, Koblitz curves only",
            entry: "koblitz_strong_rho::StrongRho::solve (escape_fruitless)",
            applies: Applies::KoblitzOnly,
            params: &[
                ParamDecl {
                    name: "lanes",
                    default: Some("32"),
                    help: "walks advanced in lockstep (batch-inversion width)",
                },
                ParamDecl {
                    name: "dp_bits",
                    default: Some("4"),
                    help: "distinguished-point bits; the table holds about steps / 2^dp_bits entries",
                },
                ParamDecl {
                    name: "step_cap_factor",
                    default: Some("2000"),
                    help: "give up after this multiple of the ideal step count",
                },
            ],
        },
        MethodDecl {
            id: "bsgs.textbook",
            family: "bsgs",
            summary: "baby table of m = ⌈√r⌉ steps first, then giant steps; worst 2√r, mean 1.5√r",
            entry: "ecbench::generic::bsgs_textbook",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "bsgs.interleaved",
            family: "bsgs",
            summary: "Pollard's interleaved baby and giant steps, two tables; mean (4/3)√r",
            entry: "ecbench::generic::bsgs_interleaved",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "bsgs.negation",
            family: "bsgs",
            summary: "baby table keyed by {P, −P}, giant stride 2m + 1; mean about √r",
            entry: "ecbench::generic::bsgs_negation",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "kangaroo.vow",
            family: "kangaroo",
            summary: "van Oorschot–Wiener tame/wild kangaroo on [0, r), distinguished points; mean about 2√r",
            entry: "ecbench::generic::kangaroo",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "claw.pair_table",
            family: "claw",
            summary: "four-point signed-Frobenius pair claw of aburan28/cryptanalysis#175: known-log orbit base, target-independent quotient pair table of M = c·√(r/n) classes, unique query pairs Q − (F_k + F_l); generic, mean S ≈ (c + 1/c)/√n, Koblitz curves only",
            entry: "ecbench::claw::pair_claw",
            applies: Applies::KoblitzOnly,
            params: &[
                ParamDecl {
                    name: "base_scale",
                    default: None,
                    help: "seed orbits S = ⌈base_scale · r^(1/4) · n^(−3/4)⌉; the PR's n=83 base is ≈ 1.06",
                },
                ParamDecl {
                    name: "table_scale",
                    default: None,
                    help: "table classes M = ⌈table_scale · √(r/n)⌉; 1 minimises additions, the PR's n=83 tables are ≈ 1/32",
                },
                ParamDecl {
                    name: "cap_multiple",
                    default: Some("64"),
                    help: "charged-addition budget as a multiple of √r before the run counts as exhausted",
                },
            ],
        },
        MethodDecl {
            id: "ic.pipeline",
            family: "ic",
            summary: "index calculus through ic_framework::run_pipeline: factor base, oracle set-up, relations, linear algebra, verification, all charged",
            entry: "ic_framework::run_pipeline",
            applies: Applies::Any,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "ic.pipeline_f2m4_full",
            family: "ic",
            summary: "ic.pipeline on a curve over F_(2^m)^4 (bin-t-line + gaudry-d4-bin, one-word and u128) with every native counter priced in F_q-multiplication equivalents from src/cryptanalysis/ecbench/f2m4_full_prices.json: row operations, guard and witness lookups, the solver's table inversions and the matrix elements its passes touch, monomial hash-map operations, large-prime arithmetic, relation rows, factor-base construction; the same run, so the same counts",
            entry: "ic_framework::run_pipeline + ecbench::f2m4_price",
            applies: Applies::Any,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "ic.pipeline_f2m4_ref",
            family: "ic",
            summary: "ic.pipeline_f2m4_full in the reference pricing: the solver's F_q operations and native work are divided by the per-m reference affine addition (50 at odd m, 64 at even m; reference_pricing.reference_model and .poly_free_const record both models), and the base lifts, x_of_t, the Weil-restriction system builds and the interpolation precomputation are priced with Karatsuba/tower F_(q^4) arithmetic (gaudry_quartic_bin::RefCosts; reference_pricing.weil_restriction has weil_muls and weil_ref_muls, solver.extra the same per record); detail.reference_pricing carries the current-unit total, both divisors and IC's F_(q^4) share side by side; the same run, so the same counts",
            entry: "ic_framework::run_pipeline + ecbench::f2m4_price",
            applies: Applies::Any,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "ic.pipeline_f2m4_refmm",
            family: "ic",
            summary: "ic.pipeline_f2m4_ref and mod-r multiplications (scalar_mulmod, row_op, lp_mulmod) priced from a one- or two-limb Montgomery multiplication (f2m4_mulmod_prices.json, ecbench price-units-f2m4 --mulmod) at every m, so the unit has no step where the subgroup order crosses a word; detail.reference_pricing carries the same record with the code's mulmod beside it; the same run, so the same counts; curves over F_(2^m)^4 only",
            entry: "ic_framework::run_pipeline + ecbench::f2m4_price",
            applies: Applies::Any,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "ic.pipeline_counted",
            family: "ic",
            summary: "index calculus with solver wall pricing excluded by deterministic phase recomputation; unpriced solver work remains explicit",
            entry: "ic_framework::run_pipeline + ecbench::ic_report(counted)",
            applies: Applies::Any,
            params: IC_COUNTED_PARAMS,
        },
        MethodDecl {
            id: "ic.large_prime",
            family: "ic",
            summary: "bounded exact m-summand binary index calculus with zero, single, or double large primes and exact modular elimination",
            entry: "ecbench_large_prime::solve",
            applies: Applies::BinaryOnly,
            params: &[
                ParamDecl {
                    name: "small_dimension",
                    default: None,
                    help: "dimension of the nested small x-coordinate subspace",
                },
                ParamDecl {
                    name: "envelope_dimension",
                    default: None,
                    help: "dimension of the full x-coordinate factor-base envelope",
                },
                ParamDecl {
                    name: "summands",
                    default: None,
                    help: "exact summand count, or n-1 (never silently downscaled)",
                },
                ParamDecl {
                    name: "large_primes",
                    default: None,
                    help: "maximum accepted large-prime columns: 0, 1, or 2",
                },
                ParamDecl {
                    name: "max_trials",
                    default: None,
                    help: "relation trials before an exhausted result",
                },
                ParamDecl {
                    name: "max_states",
                    default: None,
                    help: "hard cap on exact meet-in-the-middle combination states",
                },
            ],
        },
        MethodDecl {
            id: "ic.shared_rank",
            family: "ic",
            summary: "target-blind full-rank Koblitz relation log followed by point-only signed-Frobenius m3 target descent; native work remains visible and unpriced",
            entry: "ic_framework::shared_rank::run_shared_rank_targets",
            applies: Applies::KoblitzOnly,
            params: &[
                ParamDecl {
                    name: "factor_base",
                    default: None,
                    help: "compact-orbit-scan:columns=N,raw_x_cap=M; exact base construction",
                },
                ParamDecl {
                    name: "rank_seed",
                    default: None,
                    help: "seed of the target-blind relation search",
                },
                ParamDecl {
                    name: "rank_max_trials",
                    default: None,
                    help: "target-blind relation trial cap",
                },
                ParamDecl {
                    name: "target_max_attempts",
                    default: None,
                    help: "direct Q query followed by at most this many total residual attempts",
                },
            ],
        },
    ]
}

/// The leading-order expected `S` of a generic method on a curve whose
/// generic automorphism group has order `a_available`, or `None` when
/// the method has no such constant (index calculus).  Derived, not
/// measured: these are the boundaries a calibration run is read against.
///
/// - rho on points: `√(π/2)` (the birthday bound on `r` points);
///   on classes of size `A`: `√(π/2A)` (negation `A = 2`, signed
///   Frobenius `A = 2m`).
/// - BSGS, textbook: `m = ⌈√r⌉` baby steps, then a uniform target's
///   giant steps, mean `m/2`: `1.5`.  Interleaved: `E[2·max(i, j)]` for
///   uniform `i, j < m`, `(4/3)`.  Negation-folded: `m = √r/2` baby
///   steps and a mean of `√r/2` giant steps of stride `2m + 1`: `1`.
/// - Kangaroo (van Oorschot–Wiener, one tame and one wild, mean jump
///   `√r/2` on the interval `[0, r)`): `2`.
///
/// Every one omits `O(log r)` set-up, so measured `S` approaches it from
/// above as `r` grows.
pub fn expected_s(id: &str, a_available: u32) -> Option<f64> {
    use std::f64::consts::PI;
    match id {
        "rho.frozen_reference" | "rho.plain" => Some((PI / 2.0).sqrt()),
        "rho.negation"
        | "rho.negation_batched"
        | "rho.negation_batched_full"
        | "rho.negation_batched_ref"
        | "rho.negation_batched_refmm"
        | "rho.negation_f2m4_refmm"
        | "rho.negation_f2m4_ref"
        | "rho.negation_f2m4_full" => Some((PI / 4.0).sqrt()),
        "rho.cubic" => Some((PI / 12.0).sqrt()),
        "rho.quartic" => Some((PI / 8.0).sqrt()),
        "rho.signed_frobenius"
        | "rho.signed_frobenius_strong"
        | "rho.signed_frobenius_strong_escape" => {
            Some((PI / (2.0 * a_available.max(1) as f64)).sqrt())
        }
        "bsgs.textbook" => Some(1.5),
        "bsgs.interleaved" => Some(4.0 / 3.0),
        "bsgs.negation" => Some(1.0),
        "kangaroo.vow" => Some(2.0),
        _ => None,
    }
}

/// A method as a spec names it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MethodSpec {
    pub id: String,
    #[serde(default)]
    pub params: BTreeMap<String, String>,
}

/// A method with its defaults written out and its identity computed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedMethod {
    pub id: String,
    pub family: String,
    pub params: BTreeMap<String, String>,
    /// `ECM1h` + 12 hex of `{id, params}`.  The configuration, not the
    /// code: the binary's hash is recorded beside it.
    pub method_id: String,
    pub method_sha256: String,
    pub entry: String,
}

pub fn decl(id: &str) -> Option<&'static MethodDecl> {
    registry().iter().find(|m| m.id == id)
}

/// Fill defaults, refuse unknown and missing parameters, hash.
pub fn resolve(spec: &MethodSpec) -> Result<ResolvedMethod, String> {
    let d = decl(&spec.id).ok_or_else(|| {
        let ids: Vec<&str> = registry().iter().map(|m| m.id).collect();
        format!("unknown method `{}`; known: {}", spec.id, ids.join(", "))
    })?;
    for k in spec.params.keys() {
        if !d.params.iter().any(|p| p.name == k) {
            return Err(format!("method `{}` has no parameter `{k}`", d.id));
        }
    }
    let mut params = BTreeMap::new();
    for p in d.params {
        match (spec.params.get(p.name), p.default) {
            (Some(v), _) => {
                params.insert(p.name.to_string(), v.clone());
            }
            (None, Some(def)) => {
                params.insert(p.name.to_string(), def.to_string());
            }
            (None, None) => {
                return Err(format!(
                    "method `{}` needs `{}` ({}); it has no default because it changes the search",
                    d.id, p.name, p.help
                ))
            }
        }
    }
    let (method_id, method_sha256) = short_id(
        "ECM1",
        &json!({"schema": "ecbench.method/v1", "id": d.id, "params": params}),
    )?;
    Ok(ResolvedMethod {
        id: d.id.into(),
        family: d.family.into(),
        params,
        method_id,
        method_sha256,
        entry: d.entry.into(),
    })
}

/// One phase of a solve, in the unit and natively.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PhaseRecord {
    pub name: String,
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
    /// Group-addition equivalents charged to this phase.
    pub gae: f64,
    /// In-process wall time of the phase where the producer clocks it.
    pub wall_ns: Option<u64>,
    #[serde(default)]
    pub native: BTreeMap<String, u64>,
}

/// The factor base an index-calculus run used, identified by its points.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactorBaseFacts {
    /// `FB1h` + 12 hex of the curve, family, parameters and the sorted
    /// point keys: two runs with the same id used the same points.
    pub fb_id: String,
    pub fb_sha256: String,
    pub family: String,
    pub params: BTreeMap<String, String>,
    pub description: String,
    pub signed_points: u64,
    pub abscissae: u64,
    pub columns: u64,
    pub dimension: Option<u32>,
    /// SHA-256 of the sorted group keys of every point.
    pub points_sha256: String,
}

/// The one-target online window: from the first target-dependent
/// computation to the verified recovery, with its exclusive phases, as the
/// IC measurement rules define it (AGENTS.md "IC measurements").
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnlineWindow {
    pub wall_ns: u64,
    /// Exclusive phases inside the window, ns, under the claim schema's
    /// names: `target_query`, `target_PDP`, `target_relation_check`,
    /// `target_descent`, `target_recovery_check` for index calculus;
    /// `rho_solve` (and `recovery_check` where a walk marks it) for a
    /// generic method.  They sum to `wall_ns`.
    pub phases_ns: BTreeMap<String, u64>,
    /// Phases of the window this method never entered, each with the
    /// reason its cost is zero by construction.
    pub zero_phases: BTreeMap<String, String>,
    pub start_event: String,
    pub stop_event: String,
    pub included_stages: Vec<String>,
    /// How the method's work maps onto the stage names.
    pub mapping: String,
}

/// What an algebraic or SAT decomposition oracle's solver did over a
/// run: the shape of the system it solved, the work it counted in its
/// own unit, and the matrix and search statistics the ICMS registry
/// names (`docs/ic/measurement/registry.json`, `pdp_metrics`).
///
/// Informational: nothing here is priced into `S` beyond what the
/// relations phase already carries as `solver_*` counters, and the audit
/// replay does not compare it (a budgeted solver is nondeterministic by
/// construction, and the block records exactly that).  A table oracle
/// has no solver and the record's block is `null`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SolverStats {
    pub name: String,
    pub calls: u64,
    pub ops: u64,
    pub op_unit: String,
    pub wall_ns: u64,
    pub peak_bytes: u64,
    /// Calls that ran out of budget, counted apart from refutations.
    pub budget_exceeded: u64,
    /// The system every call solved on this base: variables, equations,
    /// their degrees, and the semi-regular degree such a system reaches
    /// without exploitable structure.
    pub n_vars: Option<u64>,
    pub n_equations: Option<u64>,
    pub degrees: Vec<u32>,
    pub semi_regular_degree: Option<u32>,
    pub solving_degree_mean: Option<f64>,
    pub solving_degree_max: Option<u32>,
    /// Mean solving degree over the semi-regular bound.
    pub degree_over_bound: Option<f64>,
    /// Macaulay (or F4/F5) matrix figures, the maximum over calls where
    /// the solver reports a maximum and the sum where it reports a sum.
    pub macaulay_rows: Option<u64>,
    pub macaulay_columns: Option<u64>,
    pub macaulay_degree: Option<u32>,
    pub macaulay_rank: Option<u64>,
    /// SAT figures: variables and clauses are the maximum over calls,
    /// conflicts, decisions, propagations, restarts and learnt clauses
    /// are totals over the run.
    pub sat_variables: Option<u64>,
    pub sat_clauses: Option<u64>,
    pub sat_conflicts: Option<u64>,
    pub sat_decisions: Option<u64>,
    pub sat_propagations: Option<u64>,
    pub sat_restarts: Option<u64>,
    pub sat_learnt_clauses: Option<u64>,
    /// Every solver-specific counter, verbatim.
    pub extra: BTreeMap<String, u64>,
}

impl SolverStats {
    /// Build the block from the framework's run report.
    pub fn from_report(
        solver: &crate::cryptanalysis::ic_framework::SolverReport,
        system: Option<&crate::cryptanalysis::ic_framework::stages::SystemShape>,
    ) -> Self {
        let x = &solver.extra;
        let get = |keys: &[&str]| keys.iter().find_map(|k| x.get(*k).copied());
        let sat = solver.op_unit == "conflicts";
        SolverStats {
            name: solver.name.clone(),
            calls: solver.calls,
            ops: solver.ops,
            op_unit: solver.op_unit.clone(),
            wall_ns: solver.wall_ns,
            peak_bytes: solver.peak_bytes,
            budget_exceeded: solver.budget_exceeded,
            n_vars: system.map(|s| s.n_vars as u64),
            n_equations: system.map(|s| s.n_equations as u64),
            degrees: system.map(|s| s.degrees.clone()).unwrap_or_default(),
            semi_regular_degree: solver
                .semi_regular_degree
                .or_else(|| system.and_then(|s| s.semi_regular_degree)),
            solving_degree_mean: solver.solving_degree_mean,
            solving_degree_max: (solver.solving_degree_max > 0)
                .then_some(solver.solving_degree_max),
            degree_over_bound: solver.degree_over_bound,
            macaulay_rows: get(&["matrix_rows_max", "macaulay_rows"]),
            macaulay_columns: get(&["matrix_cols_max", "macaulay_cols"]),
            macaulay_degree: get(&["max_degree", "max_poly_degree"])
                .map(|d| d as u32)
                .or((solver.solving_degree_max > 0).then_some(solver.solving_degree_max)),
            macaulay_rank: get(&["matrix_rank_max", "macaulay_rank"]),
            sat_variables: if sat { get(&["variables_max"]) } else { None },
            sat_clauses: if sat { get(&["clauses_max"]) } else { None },
            sat_conflicts: sat.then_some(solver.ops),
            sat_decisions: if sat { get(&["decisions"]) } else { None },
            sat_propagations: if sat { get(&["propagations"]) } else { None },
            sat_restarts: if sat { get(&["restarts"]) } else { None },
            sat_learnt_clauses: if sat { get(&["learnt_clauses"]) } else { None },
            extra: x.clone(),
        }
    }
}

/// What a solve reports.  The runner adds verification and timing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveReport {
    /// The candidate logarithm, unverified.  Wide enough for a subgroup
    /// order past `2^64`; written as a number while it fits a `u64`.
    #[serde(
        default,
        with = "crate::cryptanalysis::ecbench::canonical::compat_u128::option"
    )]
    pub recovered: Option<u128>,
    /// The budget ran out before an answer.
    pub exhausted: bool,
    pub phases: Vec<PhaseRecord>,
    pub total_gae: f64,
    /// Automorphism group order the method's walk or table uses.
    pub automorphisms_used: u32,
    /// Counters by name; shape descriptors and `*_uncharged` work.
    pub counters: BTreeMap<String, u64>,
    /// Work counted but not priced in the unit.  Nonempty makes the
    /// total a lower bound.
    pub unpriced: Vec<String>,
    /// The same inputs give the same counts.  False when any price came
    /// from host wall time or a solver ran under a wall-clock budget.
    pub deterministic: bool,
    pub nondeterminism: Vec<String>,
    /// Wall time of the algorithm alone, inside the measured process.
    pub solve_wall_ns: u64,
    pub factor_base: Option<FactorBaseFacts>,
    /// Method-specific detail kept whole (the IC report's structure).
    pub detail: Value,
    /// The one-target online window, when the phase clock produced one.
    #[serde(default)]
    pub online: Option<OnlineWindow>,
    #[serde(default)]
    pub online_error: Option<String>,
    /// The decomposition solver's statistics, for algebraic and SAT
    /// oracles; `None` for table oracles and generic methods.
    #[serde(default)]
    pub solver: Option<SolverStats>,
    /// The field operations behind the group operations — modular
    /// multiplications, squarings and inversions — summed over every
    /// phase of the solve, when the group counted them
    /// ([`CountedGroup::counts_field_ops`]: the prime-field curve
    /// arithmetic the generic walks and tables run on).  `None` is
    /// unknown, never zero: binary and Koblitz curves do not count, and
    /// the index-calculus pipeline does field arithmetic outside the
    /// group law (square roots, Legendre symbols, the oracles, the
    /// elimination) that this tally would not see, so it reports none.
    /// Kept out of `counters` and `phases`, so what a committed record's
    /// replay compares is unchanged; the audit compares the block only
    /// when both the record and the replay carry it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ops: Option<FieldOps>,
}

fn ops_phase(name: &str, ops: GroupOps) -> PhaseRecord {
    PhaseRecord {
        name: name.into(),
        adds: ops.adds,
        doubles: ops.doubles,
        scalar_mults: ops.scalar_mults,
        gae: ops.gae(),
        wall_ns: None,
        native: BTreeMap::new(),
    }
}

fn unpriced_of(counters: &BTreeMap<String, u64>) -> Vec<String> {
    counters
        .iter()
        .filter(|(k, v)| k.ends_with("_uncharged") && **v > 0)
        .map(|(k, _)| k.clone())
        .collect()
}

fn param_u64(m: &ResolvedMethod, name: &str) -> Result<u64, String> {
    m.params[name]
        .parse()
        .map_err(|_| format!("parameter `{name}` is not an integer: `{}`", m.params[name]))
}

/// The field operations of a solve from its ledgers, when the group
/// counts them; `None` (unknown) when it does not.
fn field_ops_of<G: CountedGroup>(g: &G, ledgers: &[GroupOps]) -> Option<FieldOps> {
    g.counts_field_ops().then(|| FieldOps::total(ledgers))
}

fn generic_report(
    out: GenericOutcome,
    wall: u64,
    a: u32,
    field_ops: Option<FieldOps>,
) -> SolveReport {
    let total = out.setup.gae() + out.search.gae();
    SolveReport {
        recovered: out.recovered.map(u128::from),
        exhausted: out.exhausted,
        phases: vec![
            ops_phase("setup", out.setup),
            ops_phase("search", out.search),
        ],
        total_gae: total,
        automorphisms_used: a,
        unpriced: unpriced_of(&out.counters),
        counters: out.counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: Value::Null,
        online: None,
        online_error: None,
        solver: None,
        field_ops,
    }
}

/// Split a tuned rho run into set-up, walk and verification from its own
/// counters; the frozen walk has none and stays one phase.
fn rho_report(res: RhoResult, wall: u64, field_ops: Option<FieldOps>) -> SolveReport {
    let c = &res.counters;
    let get = |k: &str| c.get(k).copied().unwrap_or(0);
    let phases = if c.is_empty() {
        vec![ops_phase("search", res.group_ops)]
    } else {
        let setup = get("setup_additions") + get("setup_doublings");
        let verify = get("verification_operations");
        let total = res.group_ops.adds + res.group_ops.doubles;
        let search = total.saturating_sub(setup + verify);
        let mk = |name: &str, gae: u64| PhaseRecord {
            name: name.into(),
            gae: gae as f64,
            ..Default::default()
        };
        let mut s = mk("setup", setup);
        s.adds = get("setup_additions");
        s.doubles = get("setup_doublings");
        let mut w = mk("search", search);
        w.scalar_mults = res.group_ops.scalar_mults;
        vec![s, w, mk("internal_verification", verify)]
    };
    let mut counters = res.counters.clone();
    counters.insert("steps".into(), res.steps);
    counters.insert("walks".into(), res.walks);
    counters.insert("distinguished_points".into(), res.distinguished_points);
    SolveReport {
        recovered: res.recovered.map(u128::from),
        exhausted: res.recovered.is_none(),
        total_gae: res.gae,
        automorphisms_used: res.automorphisms,
        unpriced: unpriced_of(&counters),
        counters,
        phases,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: json!({"method": res.method, "expected_steps": res.expected_steps}),
        online: None,
        online_error: None,
        solver: None,
        field_ops,
    }
}

/// Run the generic and rho methods on any counted group.
fn solve_generic<G: CountedGroup>(
    m: &ResolvedMethod,
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    let cap = || -> Result<u64, String> { Ok(rho_cap(r, param_u64(m, "cap_multiple")? as f64)) };
    let t = Instant::now();
    // Everything these methods do depends on the target (rho's jump table
    // is [a]G + [b]Q), so the whole solve is the online window.
    measurement::begin_online(Phase::RhoSolve);
    // The rho walks keep one ledger for the whole solve; the tables and
    // the kangaroo keep set-up and search apart.  Either way the field
    // operations reported are the solve's total, known only when the
    // group counts them.
    let rho = |res: RhoResult, wall: u64| {
        let f = field_ops_of(g, &[res.group_ops]);
        rho_report(res, wall, f)
    };
    let generic = |o: GenericOutcome, wall: u64, a: u32| {
        let f = field_ops_of(g, &[o.setup, o.search]);
        generic_report(o, wall, a, f)
    };
    let rep = match m.id.as_str() {
        "rho.frozen_reference" => {
            let res = rho_reference(g, gen, target, r, seed, cap()?);
            rho(res, t.elapsed().as_nanos() as u64)
        }
        "rho.plain" => {
            let res = rho_walk_with(
                g,
                &PointClasses,
                gen,
                target,
                r,
                seed,
                cap()?,
                RhoWalk::plain(),
            );
            rho(res, t.elapsed().as_nanos() as u64)
        }
        "rho.negation" => {
            let res = rho_walk_with(
                g,
                &NegationClasses { r },
                gen,
                target,
                r,
                seed,
                cap()?,
                RhoWalk::negation(),
            );
            rho(res, t.elapsed().as_nanos() as u64)
        }
        "bsgs.textbook" => {
            let o = bsgs_textbook(g, gen, target, r);
            generic(o, t.elapsed().as_nanos() as u64, 1)
        }
        "bsgs.interleaved" => {
            let o = bsgs_interleaved(g, gen, target, r);
            generic(o, t.elapsed().as_nanos() as u64, 1)
        }
        "bsgs.negation" => {
            let o = bsgs_negation(g, gen, target, r);
            generic(o, t.elapsed().as_nanos() as u64, 2)
        }
        "kangaroo.vow" => {
            let o = kangaroo(g, gen, target, r, seed, cap()?);
            generic(o, t.elapsed().as_nanos() as u64, 1)
        }
        other => return Err(format!("`{other}` is not a generic method")),
    };
    measurement::end_online();
    Ok(rep)
}

/// A `u128` order on a one-word path: the wide curves route to
/// `ic_wide` before they get here.
fn one_word(v: u128) -> Result<u64, String> {
    u64::try_from(v).map_err(|_| format!("{v} needs the wide (u128) path"))
}

fn parse_hex_wide(s: &str) -> Result<u128, String> {
    u128::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex `{s}`"))
}

/// The methods that have a wide-field (`64 ≤ n ≤ 127`) implementation.
/// Any other method refuses a wide curve rather than run on a truncation.
fn solve_wide(
    m: &ResolvedMethod,
    wi: &WideInstance,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<SolveReport, String> {
    match m.id.as_str() {
        "rho.signed_frobenius_strong" | "rho.signed_frobenius_strong_escape" => {
            solve_strong_wide(m, wi, tx, ty, seed)
        }
        "claw.pair_table" => solve_claw_wide(m, wi, tx, ty, seed),
        other => Err(format!(
            "`{other}` has no wide-field implementation; on a curve past one word (n = {}) only the strong rho (with or without the escape) and claw.pair_table run",
            wi.kc.n
        )),
    }
}

fn parse_hex(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex `{s}`"))
}

/// Solve for `target` with method `m` on `inst`, under algorithm seed
/// `seed`.  The planted logarithm is not an argument.
pub fn solve(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    target: &[String; 2],
    seed: u64,
) -> Result<SolveReport, String> {
    let d = decl(&m.id).ok_or("unregistered method")?;
    if d.applies == Applies::KoblitzOnly && curve.family != "koblitz" {
        return Err(format!(
            "`{}` runs on Koblitz curves only; {} is {}",
            m.id, curve.slug, curve.family
        ));
    }
    if d.applies == Applies::BinaryOnly && !matches!(inst, Instance::Binary(_)) {
        return Err(format!(
            "`{}` runs on binary curves only; {} is {}",
            m.id, curve.slug, curve.family
        ));
    }
    // The phase clock: the methods open and close the online window, the
    // session turns it into exclusive phase times.
    let session = measurement::Session::begin().ok();
    if matches!(
        m.id.as_str(),
        "rho.negation_batched"
            | "rho.negation_batched_full"
            | "rho.negation_f2m4_full"
            | "ic.pipeline_f2m4_full"
            | "rho.negation_batched_ref"
            | "rho.negation_f2m4_ref"
            | "ic.pipeline_f2m4_ref"
            | "rho.negation_batched_refmm"
            | "rho.negation_f2m4_refmm"
            | "ic.pipeline_f2m4_refmm"
    ) && !matches!(inst, Instance::F2m4(_))
    {
        return Err(format!(
            "`{}` runs on curves over F_(2^m)^4 only; {} is {}",
            m.id, curve.slug, curve.family
        ));
    }
    let mut rep = if let Instance::Wide(wi) = inst {
        let (tx, ty) = (parse_hex_wide(&target[0])?, parse_hex_wide(&target[1])?);
        solve_wide(m, wi, tx, ty, seed)?
    } else if let Instance::F2m4(i) = inst {
        let (tx, ty) = (parse_hex_wide(&target[0])?, parse_hex_wide(&target[1])?);
        if m.id == "rho.negation_batched" {
            solve_f2m4_rho_batched(m, i, tx, ty, seed, false, false, Mulmod::Code)?
        } else if m.id == "rho.negation_batched_full" {
            solve_f2m4_rho_batched(m, i, tx, ty, seed, true, false, Mulmod::Code)?
        } else if m.id == "rho.negation_batched_ref" {
            solve_f2m4_rho_batched(m, i, tx, ty, seed, true, true, Mulmod::Code)?
        } else if m.id == "rho.negation_batched_refmm" {
            solve_f2m4_rho_batched(m, i, tx, ty, seed, true, true, Mulmod::Montgomery)?
        } else if m.id == "rho.negation_f2m4_full" {
            solve_f2m4_rho_negation_full(m, i, tx, ty, seed, false, Mulmod::Code)?
        } else if m.id == "rho.negation_f2m4_ref" {
            solve_f2m4_rho_negation_full(m, i, tx, ty, seed, true, Mulmod::Code)?
        } else if m.id == "rho.negation_f2m4_refmm" {
            solve_f2m4_rho_negation_full(m, i, tx, ty, seed, true, Mulmod::Montgomery)?
        } else if m.id == "ic.pipeline_f2m4_full" {
            solve_f2m4_ic_full(m, i, curve, tx, ty, seed)?
        } else if m.id == "ic.pipeline_f2m4_ref" {
            solve_f2m4_ic_full_with(
                m,
                i,
                curve,
                tx,
                ty,
                seed,
                u64::try_from(i.r).is_err(),
                true,
                Mulmod::Code,
            )?
        } else if m.id == "ic.pipeline_f2m4_refmm" {
            solve_f2m4_ic_full_with(
                m,
                i,
                curve,
                tx,
                ty,
                seed,
                u64::try_from(i.r).is_err(),
                true,
                Mulmod::Montgomery,
            )?
        } else if u64::try_from(i.r).is_err() {
            // `m ≥ 16` with a small cofactor: the subgroup order needs two
            // words; the u128 implementations (`ic_wide`, `rho_wide`).
            solve_f2m4_wide(m, i, curve, tx, ty, seed)?
        } else {
            solve_inner(m, inst, curve, d, tx as u64, ty as u64, seed)?
        }
    } else {
        let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
        solve_inner(m, inst, curve, d, tx, ty, seed)?
    };
    match session.map(|s| s.finish()) {
        Some(Ok(snap)) if rep.online.is_none() => {
            rep.online = online_window(d.family, &m.id, &snap)
        }
        Some(Ok(_)) => {} // A method with its own five-phase online clock supplied the window.
        Some(Err(e)) => rep.online_error = Some(e.to_string()),
        None => rep.online_error = Some("a measurement session was already active".into()),
    }
    Ok(rep)
}

fn solve_inner(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    d: &MethodDecl,
    tx: u64,
    ty: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    if let Instance::Wide(_) = inst {
        return Err("a wide instance goes through solve_wide".into());
    }
    match (inst, d.family) {
        (Instance::Binary(i), _) if m.id == "ic.large_prime" => {
            solve_large_prime(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _) if m.id == "ic.shared_rank" => {
            solve_shared_rank(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), "ic") => solve_ic_prime(m, i, curve, PrimePoint::affine(tx, ty), seed),
        (Instance::Fp4(i), "ic") if m.id == "ic.pipeline" => {
            let p = i.p();
            let target = Pt4 {
                x: unpack_e4(p, tx),
                y: unpack_e4(p, ty),
                inf: false,
            };
            solve_ic_fp4(m, i, curve, target, seed)
        }
        (Instance::Fp4(_), "ic") => Err(format!(
            "`{}` does not run on a curve over F_(p^4); ic.pipeline does",
            m.id
        )),
        (Instance::F2m4(i), "ic") if m.id == "ic.pipeline" => {
            let f = &i.curve.f;
            let target = PtB {
                x: f.unpack(u128::from(tx)),
                y: f.unpack(u128::from(ty)),
                inf: false,
            };
            solve_ic_f2m4(m, i, curve, target, seed)
        }
        (Instance::F2m4(_), "ic") => Err(format!(
            "`{}` does not run on a curve over F_(2^m)^4; ic.pipeline does",
            m.id
        )),
        (Instance::Binary(i), "ic") => {
            solve_ic_binary(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _)
            if m.id == "rho.signed_frobenius_strong"
                || m.id == "rho.signed_frobenius_strong_escape" =>
        {
            solve_strong(m, i, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), "claw") => solve_claw(m, i, FastPoint::affine(tx, ty), seed),
        (Instance::Binary(i), _) if m.id == "rho.signed_frobenius" => {
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let res = signed_frobenius_rho_tuned(
                i,
                FastPoint::affine(tx, ty),
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
            )
            .ok_or("not a Koblitz instance")?;
            measurement::end_online();
            // A Koblitz walk does not count its field operations: unknown.
            Ok(rho_report(res, t.elapsed().as_nanos() as u64, None))
        }
        (Instance::Prime(i), _) if m.id == "rho.quartic" => {
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let mut setup = GroupOps::default();
            let classes =
                QuarticClasses::new(&i.curve, i.generator_point(), i.r, &mut setup).ok_or(
                    format!(
                        "`rho.quartic` needs a j = 1728 curve over p ≡ 1 (mod 4) with 4 | r − 1 and a nontrivial ι on the subgroup; {} lacks the structure",
                        curve.slug
                    ),
                )?;
            let mut res = rho_walk_with(
                &i.curve,
                &classes,
                i.generator_point(),
                PrimePoint::affine(tx, ty),
                i.r,
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
                RhoWalk::negation(),
            );
            res.group_ops.merge(setup);
            measurement::end_online();
            let f = FieldOps::total(&[res.group_ops]);
            Ok(rho_report(res, t.elapsed().as_nanos() as u64, Some(f)))
        }
        (Instance::Prime(i), _) if m.id == "rho.cubic" => {
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let mut setup = GroupOps::default();
            let classes = CubicClasses::new(&i.curve, i.generator_point(), i.r, &mut setup).ok_or(
                format!(
                    "`rho.cubic` needs a j = 0 curve over p ≡ 1 (mod 3) with 3 | r − 1 and a nontrivial ζ₃ on the subgroup; {} lacks the structure",
                    curve.slug
                ),
            )?;
            let mut res = rho_walk_with(
                &i.curve,
                &classes,
                i.generator_point(),
                PrimePoint::affine(tx, ty),
                i.r,
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
                RhoWalk::negation(),
            );
            res.group_ops.merge(setup);
            measurement::end_online();
            let f = FieldOps::total(&[res.group_ops]);
            Ok(rho_report(res, t.elapsed().as_nanos() as u64, Some(f)))
        }
        (Instance::Prime(i), _) => solve_generic(
            m,
            &i.curve,
            i.generator_point(),
            PrimePoint::affine(tx, ty),
            i.r,
            seed,
        ),
        (Instance::Binary(i), _) => {
            let g = BinaryGroup(&i.fast);
            solve_generic(m, &g, i.generator, FastPoint::affine(tx, ty), i.r, seed)
        }
        (Instance::Fp4(i), _) => {
            let p = i.p();
            let target = Pt4 {
                x: unpack_e4(p, tx),
                y: unpack_e4(p, ty),
                inf: false,
            };
            solve_generic(m, &i.inst.curve, i.generator, target, i.r, seed)
        }
        (Instance::F2m4(i), _) => {
            let f = &i.curve.f;
            let target = PtB {
                x: f.unpack(u128::from(tx)),
                y: f.unpack(u128::from(ty)),
                inf: false,
            };
            solve_generic(m, &i.curve, i.generator, target, one_word(i.r)?, seed)
        }
        (Instance::Wide(_), _) => unreachable!("returned above"),
    }
}

fn solve_large_prime(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let summands = match m.params["summands"].as_str() {
        "n-1" => inst.n.checked_sub(1).ok_or("field degree has no n-1")?,
        value => value
            .parse::<u32>()
            .map_err(|_| format!("parameter `summands` is not an integer or n-1: `{value}`"))?,
    };
    let config = LargePrimeConfig {
        small_dimension: u32::try_from(param_u64(m, "small_dimension")?)
            .map_err(|_| "small_dimension does not fit u32")?,
        envelope_dimension: u32::try_from(param_u64(m, "envelope_dimension")?)
            .map_err(|_| "envelope_dimension does not fit u32")?,
        summands,
        max_large_primes: u8::try_from(param_u64(m, "large_primes")?)
            .map_err(|_| "large_primes does not fit u8")?,
        max_trials: param_u64(m, "max_trials")?,
        max_states: param_u64(m, "max_states")?,
        seed,
    };
    // The method returns an algebraic candidate; ecbench's runner verifies it.
    let report = large_prime::solve_candidate(inst, target, config)?;
    let mut phases = Vec::new();
    for name in ["factor_base", "oracle_setup", "relations", "verification"] {
        let mut phase = ops_phase(
            name,
            report
                .phase_group_ops
                .get(name)
                .copied()
                .unwrap_or_default(),
        );
        match name {
            "oracle_setup" => {
                phase.native.insert(
                    "combination_states_uncharged".into(),
                    report.counters.combination_states_uncharged,
                );
            }
            "relations" => {
                phase.native.insert(
                    "mitm_lookups_uncharged".into(),
                    report.counters.mitm_lookups_uncharged,
                );
                phase.native.insert(
                    "lp_merge_ops_uncharged".into(),
                    report.counters.lp_merge_ops_uncharged,
                );
                phase.native.insert(
                    "row_ops_uncharged".into(),
                    report.counters.row_ops_uncharged,
                );
            }
            _ => {}
        }
        phases.push(phase);
    }
    let total_gae = phases.iter().map(|p| p.gae).sum();
    let mut counters = BTreeMap::new();
    for (name, value) in [
        ("trials", report.counters.trials),
        ("decompositions", report.counters.decompositions),
        ("accepted_relations", report.counters.accepted_relations),
        (
            "eliminated_full_relations",
            report.counters.eliminated_full_relations,
        ),
        ("matrix_rank", report.counters.matrix_rank),
        ("large_prime_pivots", report.counters.lp_pivots),
        (
            "combination_states_uncharged",
            report.counters.combination_states_uncharged,
        ),
        (
            "mitm_lookups_uncharged",
            report.counters.mitm_lookups_uncharged,
        ),
        (
            "lp_merge_ops_uncharged",
            report.counters.lp_merge_ops_uncharged,
        ),
        ("row_ops_uncharged", report.counters.row_ops_uncharged),
    ] {
        counters.insert(name.into(), value);
    }
    for (i, value) in report.counters.relation_histogram.iter().enumerate() {
        counters.insert(format!("relations_with_{}_large_primes", i.min(3)), *value);
    }
    let mut point_bytes = Vec::with_capacity(report.factor_base.column_keys.len() * 8);
    for key in &report.factor_base.column_keys {
        point_bytes.extend_from_slice(&key.to_be_bytes());
    }
    let points_sha256 = sha256_hex(&point_bytes);
    let (fb_id, fb_sha256) = short_id(
        "FB1",
        &json!({
            "schema": "ecbench.factor_base/v1",
            "curve": curve.slug,
            "family": "binary-nested-large-prime",
            "params": m.params,
            "columns": report.factor_base.columns,
            "signed_points": report.factor_base.raw_points,
            "points_sha256": points_sha256,
        }),
    )?;
    let factor_base = FactorBaseFacts {
        fb_id,
        fb_sha256,
        family: "binary-nested-large-prime".into(),
        params: m.params.clone(),
        description: format!(
            "x < 2^{} nested at x < 2^{}, projected by cofactor and folded by negation",
            report.config.envelope_dimension, report.config.small_dimension
        ),
        signed_points: report.factor_base.raw_points as u64,
        abscissae: 1u64 << report.config.envelope_dimension,
        columns: report.factor_base.columns as u64,
        dimension: Some(report.config.envelope_dimension),
        points_sha256,
    };
    let detail =
        serde_json::to_value(&report).map_err(|e| format!("serialise large-prime report: {e}"))?;
    Ok(SolveReport {
        recovered: report.recovered.map(u128::from),
        exhausted: report.exhausted,
        phases,
        total_gae,
        automorphisms_used: 2,
        counters,
        unpriced: vec![
            "combination_states_uncharged".into(),
            "mitm_lookups_uncharged".into(),
            "lp_merge_ops_uncharged".into(),
            "row_ops_uncharged".into(),
        ],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: report.solve_wall_ns,
        factor_base: Some(factor_base),
        detail,
        online: None,
        online_error: None,
        solver: None,
        // Binary-field index calculus: the field work is outside the group law
        // this tally sees, so it stays unknown rather than understated.
        field_ops: None,
    })
}

/// The online window from the phase clock's snapshot, mapped to the
/// claim schema's stage names.
fn online_window(family: &str, id: &str, snap: &Snapshot) -> Option<OnlineWindow> {
    let wall_ns = snap.online_wall_ns?;
    let get = |name: &str| snap.online_phases_ns.get(name).copied().flatten();
    let mut w = OnlineWindow {
        wall_ns,
        ..Default::default()
    };
    if family == "ic" {
        w.start_event = "first target-dependent query ([a]G + [b]Q, or the walk's first jump), after the factor base and the oracle's tables are built".into();
        w.stop_event = "[d]G = Q verified inside the pipeline".into();
        w.mapping = "classic index calculus: every relation carries the target, so query generation is target_query, decomposition attempts are target_PDP, the oracle's own witness checks are target_relation_check, the elimination that pins the target's column is target_descent, and the final [d]G = Q is target_recovery_check".into();
        for (clock, claim, why) in [
            ("target_query", "target_query", "no query was drawn"),
            ("target_pdp", "target_PDP", "no decomposition was attempted"),
            (
                "target_relation_check",
                "target_relation_check",
                "the oracle returns exact decompositions (pair-table hits on exact point keys, or solutions it checks under its own relation_check scope); this pipeline has no separate relation check, so any checking cost is inside target_PDP",
            ),
            ("target_descent", "target_descent", "no relation reached the elimination"),
            ("recovery_check", "target_recovery_check", "no candidate logarithm was pinned"),
        ] {
            match get(clock) {
                Some(ns) => {
                    w.phases_ns.insert(claim.into(), ns);
                }
                None => {
                    w.phases_ns.insert(claim.into(), 0);
                    w.zero_phases.insert(claim.into(), why.into());
                }
            }
            w.included_stages.push(claim.into());
        }
    } else {
        for (name, ns) in &snap.online_phases_ns {
            if let Some(ns) = ns {
                w.phases_ns.insert((*name).to_string(), *ns);
            }
        }
        let strong =
            id == "rho.signed_frobenius_strong" || id == "rho.signed_frobenius_strong_escape";
        w.start_event = if strong {
            "first walk start [c]G + Q (the jump table [a]G is target-independent set-up)".into()
        } else {
            "first target-dependent operation (the jump table [a]G + [b]Q, or the first table step)"
                .into()
        };
        w.stop_event = "[d]G = Q verified inside the method".into();
        w.included_stages = match family {
            "rho" => vec!["walk".into(), "collision".into(), "recovery_check".into()],
            "kangaroo" => vec!["jumps".into(), "collision".into(), "recovery_check".into()],
            "claw" => vec!["queries".into(), "collision".into(), "recovery".into()],
            _ => vec![
                "baby_steps".into(),
                "giant_steps".into(),
                "recovery_check".into(),
            ],
        };
        w.mapping = format!("{family}: the whole target-dependent solve, one exclusive phase");
    }
    Some(w)
}

fn param_f64(m: &ResolvedMethod, name: &str) -> Result<f64, String> {
    let v: f64 = m.params[name]
        .parse()
        .map_err(|_| format!("parameter `{name}` is not a number: `{}`", m.params[name]))?;
    if v.is_finite() && v > 0.0 {
        Ok(v)
    } else {
        Err(format!("parameter `{name}` must be positive: `{v}`"))
    }
}

/// The pair claw ([`crate::cryptanalysis::ecbench::claw`]).  The base and
/// the table are target-independent set-up; the online window opens at
/// the first query `Q − (F_k + F_l)`.
fn solve_claw(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let shape = ClawShape {
        base_scale: param_f64(m, "base_scale")?,
        table_scale: param_f64(m, "table_scale")?,
        max_adds: rho_cap(inst.r, param_u64(m, "cap_multiple")? as f64),
    };
    let t = Instant::now();
    let o = pair_claw(inst, target, seed, shape).ok_or("not a Koblitz instance")?;
    let wall = t.elapsed().as_nanos() as u64;
    Ok(claw_report(o, shape, u128::from(inst.r), inst.n, wall))
}

/// The claw on a wide Koblitz curve; the step budget is [`rho_cap`]'s rule
/// in floating point, since the order does not fit its `u64`.
fn solve_claw_wide(
    m: &ResolvedMethod,
    wi: &WideInstance,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<SolveReport, String> {
    let multiple = param_u64(m, "cap_multiple")? as f64;
    let shape = ClawShape {
        base_scale: param_f64(m, "base_scale")?,
        table_scale: param_f64(m, "table_scale")?,
        max_adds: (crate::cryptanalysis::ic_boundary::generic_floor_ops(wi.kc.r as f64, 1.0)
            * multiple) as u64
            + 4096,
    };
    let t = Instant::now();
    let o = pair_claw_wide(wi, RawPointG::Affine { x: tx, y: ty }, seed, shape);
    let wall = t.elapsed().as_nanos() as u64;
    Ok(claw_report(o, shape, wi.kc.r, wi.kc.n, wall))
}

fn claw_report(o: ClawOutcome, shape: ClawShape, r: u128, n: u32, wall: u64) -> SolveReport {
    let total = o.base.gae() + o.table.gae() + o.search.gae() + o.recover.gae();
    let detail = json!({
        "source": "aburan28/cryptanalysis#175, experiments/koblitz-pair-claw-20260929 (known-log orbit base, quotient pair table, unique query schedule)",
        "seed_orbits": shape.seeds(r, n),
        "table_classes_target": shape.table(r, n),
        "expected_s": (shape.table_scale + 1.0 / shape.table_scale) / (n as f64).sqrt(),
        "expected_s_law": "(c + 1/c)/√n with c = table_scale, from a hit probability of 2nM/r per query at two additions each; finite base support and duplicate classes not modelled",
    });
    SolveReport {
        recovered: o.recovered,
        exhausted: o.exhausted,
        phases: vec![
            ops_phase("base", o.base),
            ops_phase("table", o.table),
            ops_phase("search", o.search),
            ops_phase("recover", o.recover),
        ],
        total_gae: total,
        automorphisms_used: 2 * n,
        unpriced: unpriced_of(&o.counters),
        counters: o.counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    }
}

/// The strong reference: `koblitz_strong_rho` as the admissible fixture
/// drives it (`examples/koblitz_rho_fixture.rs`, backend `strong`), seeded
/// from the algorithm seed.  Group additions are exact; each scalar
/// multiplication is charged `1.5·log₂ r` additions, the convention
/// `ic_boundary::signed_frobenius_rho` uses.
fn solve_strong(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let kc = inst.koblitz.as_ref().ok_or("not a Koblitz instance")?;
    let t = Instant::now();
    let rho = StrongRho::new(kc);
    let q = StrongPoint::from_binary(&inst.fast.lower(target));
    strong_run(m, &rho, q, inst.r as f64, inst.n, seed, t)
}

/// The strong reference on a wide Koblitz curve: the same walk, on the
/// instance's `u128` arithmetic.  The clock starts where the one-word path
/// starts it, before the reference's own precomputation.
fn solve_strong_wide(
    m: &ResolvedMethod,
    wi: &WideInstance,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<SolveReport, String> {
    let t = Instant::now();
    let rho = wi.kc.strong_rho();
    let q = RawPointG::Affine { x: tx, y: ty };
    strong_run(m, &rho, q, wi.kc.r as f64, wi.kc.n, seed, t)
}

/// Seed, walk and report one strong-reference solve, at either width.
fn strong_run<F: RhoField, S: RhoScalar>(
    m: &ResolvedMethod,
    rho: &StrongRhoG<F, S>,
    q: RawPointG<F::E>,
    r: f64,
    n: u32,
    seed: u64,
    t: Instant,
) -> Result<SolveReport, String> {
    let profile_online = std::env::var("ECBENCH_CALLGRIND_TARGET").as_deref() == Ok("1");
    let params = StrongRhoParams {
        lanes: param_u64(m, "lanes")?.max(1) as usize,
        dp_bits: param_u64(m, "dp_bits")? as u32,
        step_cap_factor: param_u64(m, "step_cap_factor")?,
        escape_fruitless: m.id == "rho.signed_frobenius_strong_escape",
    };
    if params.dp_bits >= 32 {
        return Err("dp_bits must be below 32".into());
    }
    let mul_gae = 1.5 * r.log2();
    let mut charges = StrongRhoCharges::default();
    let jump_seed = derive_u64("ecbench.strong_rho.jumps", &[seed]);
    let jumps = rho.jumps(jump_seed, &mut charges);
    let setup_mults = charges.scalar_multiplications;
    let mut start_rng = StdRng::seed_from_u64(seed);
    if profile_online {
        measurement::callgrind_dump(b"ecbench_before_online\0");
    }
    measurement::begin_online(Phase::RhoSolve);
    let outcome = rho.solve(q, &jumps, &mut start_rng, &params, charges);
    measurement::end_online();
    if profile_online {
        measurement::callgrind_dump(b"ecbench_online\0");
    }
    let wall = t.elapsed().as_nanos() as u64;
    let detail = json!({
        "reference": "koblitz_strong_rho::StrongRho, the single-target reference of docs/ic/boundary_targets.json since 2026-10-01",
        "lanes": params.lanes,
        "dp_bits": params.dp_bits,
        "jump_seed": jump_seed.to_string(),
        "scalar_multiplication_charge": "1.5·log2(r) additions each (ic_boundary::signed_frobenius_rho's convention)",
    });
    let Some(o) = outcome else {
        return Ok(SolveReport {
            recovered: None,
            exhausted: true,
            phases: vec![],
            total_gae: 0.0,
            automorphisms_used: 2 * n,
            counters: BTreeMap::new(),
            unpriced: vec!["counts_lost_at_the_step_cap_uncharged".into()],
            deterministic: true,
            nondeterminism: vec![],
            solve_wall_ns: wall,
            factor_base: None,
            detail,
            online: None,
            online_error: None,
            solver: None,
            field_ops: None,
        });
    };
    let c = o.charges;
    let search_mults = c.scalar_multiplications - setup_mults;
    let setup = PhaseRecord {
        name: "setup".into(),
        scalar_mults: setup_mults,
        gae: setup_mults as f64 * mul_gae,
        ..Default::default()
    };
    let search = PhaseRecord {
        name: "search".into(),
        adds: c.group_additions,
        scalar_mults: search_mults,
        gae: c.group_additions as f64 + search_mults as f64 * mul_gae,
        ..Default::default()
    };
    let mut counters = BTreeMap::new();
    for (k, v) in [
        ("walk_steps", o.walk_steps),
        ("walks", o.walks),
        ("fruitless_walks", o.fruitless),
        ("capped_walks", o.capped),
        ("wasted_merges", o.wasted_merges),
        ("table_entries", o.table_entries as u64),
        ("failed_collisions", c.failed_collisions),
        ("scalar_multiplications", c.scalar_multiplications),
        ("canonicalisations_uncharged", c.canonicalizations),
        ("partition_hashes_uncharged", c.partition_hashes),
        ("table_queries_uncharged", c.table_queries),
        ("table_inserts_uncharged", c.table_inserts),
    ] {
        counters.insert(k.to_string(), v);
    }
    // Only the escaping variant has escapes; the reference's counter set is
    // left exactly as its committed records have it.
    if params.escape_fruitless {
        counters.insert("fruitless_escapes".to_string(), o.escapes);
    }
    Ok(SolveReport {
        recovered: Some(o.scalar.to_u128()),
        exhausted: false,
        total_gae: setup.gae + search.gae,
        phases: vec![setup, search],
        automorphisms_used: o.automorphisms as u32,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

// ── Index calculus ─────────────────────────────────────────────────

/// A calibration that prices only what the repository has pinned for
/// this curve, and nothing from the host: `ns_per_add = 1`, and every
/// unit without a pinned ratio left at zero (and reported unpriced).
/// The total is then a function of the counts alone.
fn pinned_only_calibration(regime: &str, slug: &str) -> Calibration {
    let mut c = Calibration {
        ns_per_add: 1.0,
        ns_per_double: 1.0,
        ..Default::default()
    };
    c.pin(regime, slug);
    c
}

/// Native counters the calibration prices, with the unit field that
/// prices each (`price_phase`'s table).
const PRICED_NATIVE: &[(&str, &str)] = &[
    ("sqrt_solves", "ns_per_sqrt"),
    ("as_solves", "ns_per_as_solve"),
    ("s4_pairs", "ns_per_s4_pair"),
    ("lookups", "ns_per_lookup"),
    ("row_ops", "ns_per_row_op"),
    ("legendre_symbols", "ns_per_legendre"),
    ("inversions", "ns_per_inversion"),
    ("frobenius_maps", "ns_per_frobenius"),
    ("canonicalisations", "ns_per_canon"),
    ("target_guard_probes", "ns_per_lookup"),
];

fn phase_of(name: &str, p: &PhaseCost) -> PhaseRecord {
    PhaseRecord {
        name: name.into(),
        adds: p.group_ops.adds,
        doubles: p.group_ops.doubles,
        scalar_mults: p.group_ops.scalar_mults,
        gae: p.gae,
        wall_ns: Some(p.wall_ns),
        native: p.native.clone(),
    }
}

fn ic_report(
    m: &ResolvedMethod,
    rep: RunReport,
    calib: &Calibration,
    fb: FactorBaseFacts,
    wall: u64,
    automorphisms: u32,
) -> SolveReport {
    let mut phases = vec![
        phase_of("factor_base", &rep.factor_base.cost),
        phase_of("oracle_setup", &rep.decomposition.setup),
        phase_of("relations", &rep.decomposition.cost),
        phase_of("linear_algebra", &rep.linear_algebra.cost),
        phase_of("verify", &rep.verify),
    ];
    let mut unpriced = Vec::new();
    let mut nondeterminism = Vec::new();
    // Solver work: run_pipeline prices it from this host's wall time,
    // which is not a count.  Take it back out and list it unpriced.  The
    // figure is rebuilt from the pre-solver phase, never by subtracting
    // the wall term: `(x + w) - w` rounds `x` to `w`'s binade and so
    // leaked host timing into the low bits of a "deterministic" record
    // (`audit::legacy_solver_rounding` reproduces those records).
    let mut total = rep.total_gae;
    if let Some(s) = &rep.decomposition.solver {
        // `counted` (field multiplications priced at the curve's own
        // multiplications per addition, `gaudry-d4`) is a count, like
        // `pinned`, and stays in the total.
        if (s.gae > 0.0 || (m.id == "ic.pipeline_counted" && s.calls > 0))
            && !matches!(s.priced_by.as_str(), "pinned" | "counted")
        {
            phases[2].gae = rep.decomposition.gae_before_solver;
            total = phases.iter().fold(0.0, |acc, p| acc + p.gae);
            unpriced.push(format!("solver_{}_uncharged", s.op_unit.replace(' ', "_")));
        }
        if param_u64(m, "solver_budget_seconds").unwrap_or(0) > 0 {
            nondeterminism.push("solver ran under a wall-clock budget".into());
        }
    }
    if m.id == "ic.pipeline_counted" {
        // The frozen counted method reconstructs its relation charge from
        // integer group/native work, independent of solver wall time.
        let mut counted = rep.decomposition.cost.clone();
        price_phase(&mut counted, calib);
        phases[2].gae = counted.gae
            + rep
                .decomposition
                .solver
                .as_ref()
                .filter(|s| s.priced_by == "pinned")
                .map_or(0.0, |s| s.gae);
        total = phases.iter().map(|p| p.gae).sum();
    }
    // Native counters with no pinned ratio for this curve.
    for (counter, unit) in PRICED_NATIVE {
        let present = phases
            .iter()
            .any(|p| p.native.get(*counter).copied().unwrap_or(0) > 0);
        if present && !calib.is_pinned(unit) {
            let name = format!("{counter}_uncharged");
            if !unpriced.contains(&name) {
                unpriced.push(name);
            }
        }
    }
    for phase in &phases {
        for (name, &count) in &phase.native {
            if count > 0 && name.ends_with("_uncharged") && !unpriced.contains(name) {
                unpriced.push(name.clone());
            }
        }
    }
    let mut counters = BTreeMap::new();
    counters.insert("targets_tried".into(), rep.decomposition.targets_tried);
    counters.insert("relations_found".into(), rep.decomposition.relations_found);
    counters.insert("matrix_rows".into(), rep.linear_algebra.rows);
    counters.insert("matrix_rank".into(), rep.linear_algebra.rank);
    counters.insert("matrix_dependent".into(), rep.linear_algebra.dependent);
    counters.insert("matrix_work".into(), rep.linear_algebra.work);
    let solver = rep
        .decomposition
        .solver
        .as_ref()
        .map(|sr| SolverStats::from_report(sr, rep.decomposition.system.as_ref()));
    let detail = json!({
        "label": rep.label,
        "decomposition": {
            "name": rep.decomposition.name,
            "summands": rep.decomposition.summands,
            "hit_rate": format!("{:.6e}", rep.decomposition.hit_rate),
            "system": rep.decomposition.system,
            "solver": rep.decomposition.solver,
        },
        "linear_algebra": {"name": rep.linear_algebra.name, "work_unit": rep.linear_algebra.work_unit},
        "calibration_pinned_units": calib.pinned_units,
        "framework_total_gae_before_solver_removal": rep.total_gae,
    });
    SolveReport {
        recovered: rep.recovered.map(u128::from),
        exhausted: rep.exhausted,
        phases,
        total_gae: total,
        automorphisms_used: automorphisms,
        counters,
        unpriced,
        deterministic: nondeterminism.is_empty(),
        nondeterminism,
        solve_wall_ns: wall,
        factor_base: Some(fb),
        detail,
        online: None,
        online_error: None,
        solver,
        // The pipeline's field arithmetic outside the group law (square
        // roots, Legendre symbols, oracle inversions, the elimination) is
        // counted natively and priced apart; the group-law tally alone
        // would understate it, so the figure stays unknown.
        field_ops: None,
    }
}

fn factor_base_facts<E: Copy>(
    slug: &str,
    spec: &str,
    params: &Params,
    fb: &crate::cryptanalysis::ic_boundary::FactorBase<E>,
    key: impl Fn(&E) -> u64,
) -> Result<FactorBaseFacts, String> {
    let mut keys: Vec<u64> = fb.points.iter().map(key).collect();
    keys.sort_unstable();
    let mut bytes = Vec::with_capacity(keys.len() * 8);
    for k in &keys {
        bytes.extend_from_slice(&k.to_be_bytes());
    }
    let points_sha256 = sha256_hex(&bytes);
    let family = spec.split(':').next().unwrap_or(spec).to_string();
    let (fb_id, fb_sha256) = short_id(
        "FB1",
        &json!({
            "schema": "ecbench.factor_base/v1",
            "curve": slug,
            "family": family,
            "params": params.0,
            "columns": fb.columns,
            "signed_points": fb.points.len(),
            "points_sha256": points_sha256,
        }),
    )?;
    Ok(FactorBaseFacts {
        fb_id,
        fb_sha256,
        family,
        params: params.0.clone(),
        description: fb.description.clone(),
        signed_points: fb.points.len() as u64,
        abscissae: fb.abscissae as u64,
        columns: fb.columns as u64,
        dimension: fb.dimension,
        points_sha256,
    })
}

fn pipeline_spec(m: &ResolvedMethod, seed: u64) -> Result<(PipelineSpec, String, String), String> {
    let (fb_name, fb_params) = Params::parse_spec(&m.params["factor_base"])?;
    let (or_name, or_params) = Params::parse_spec(&m.params["oracle"])?;
    let solver = m.params["solver"].clone();
    let (solver_name, solver_params) = if solver.is_empty() {
        (None, Params::default())
    } else {
        let (n, p) = Params::parse_spec(&solver)?;
        (Some(n), p)
    };
    let spec = PipelineSpec {
        factor_base: fb_name.clone(),
        factor_base_params: fb_params,
        oracle: or_name.clone(),
        oracle_params: or_params,
        solver: solver_name,
        solver_params,
        targets: Targets::parse(&m.params["targets"])?,
        linalg: m.params["linalg"].clone(),
        max_trials: param_u64(m, "max_trials")?,
        seed,
        solver_budget_seconds: param_u64(m, "solver_budget_seconds")?,
    };
    Ok((spec, fb_name, or_name))
}

fn solve_ic_prime(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    curve: &CurveFacts,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    let abscissa = PrimeAbscissaBase { instance: inst };
    let orbit = GlvOrbitBase { instance: inst };
    let kummer = PrimeKummerBase { instance: inst };
    let base: &dyn FactorBaseBuilder<_> = match fb_name.as_str() {
        "prime-abscissa" => &abscissa,
        "glv-orbit" => &orbit,
        "prime-kummer" => &kummer,
        other => {
            return Err(format!(
                "factor base `{other}` does not run on a prime curve"
            ))
        }
    };
    let ms = spec.oracle_params.u64_or("m", 2)? as u32;
    let mut subtract = SubtractOracle;
    let mut mitm = MitmOracle::new(ms);
    let mut prime_batch = PrimeBatchMitmOracle::new();
    let mut mitm_fiber = FiberMitmOracle::new(ms);
    let mut rootfind_mitm = RootfindMitmOracle::new(ms);
    let mut rootfind_pairs = RootfindPairsOracle::new(ms);
    let mut rootfind = RootfindSubtractOracle::new(ms, inst);
    let oracle: &mut dyn DecompositionOracle<_> = match or_name.as_str() {
        "subtract" => &mut subtract,
        "mitm" => &mut mitm,
        "mitm-prime-batch" => &mut prime_batch,
        "mitm-fiber" => &mut mitm_fiber,
        "rootfind-mitm" => &mut rootfind_mitm,
        "rootfind-pairs" => &mut rootfind_pairs,
        "rootfind-subtract" => &mut rootfind,
        other => return Err(format!("oracle `{other}` does not run on a prime curve")),
    };
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: curve.slug.clone(),
        field_degree: None,
    };
    let calib = pinned_only_calibration("prime", &curve.slug);
    let t = Instant::now();
    // The runner, not the pipeline, checks the answer: pass a planted
    // value that cannot match so `verified` carries no information.
    let rep = run_pipeline(&ctx, &spec, base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |p| inst.curve.key(p),
    )?;
    Ok(ic_report(m, rep, &calib, facts, wall, 2))
}

/// `ic.pipeline` on a curve over `F_{p⁴}` with rational 2-torsion: the
/// `fghr-y-line` base and the `gaudry-d4` oracle
/// (`gaudry_quartic_d4`), the only pair that runs there.  Everything is
/// priced in one unit: the oracle's `F_p` multiplications at the curve's
/// own multiplications per addition, and the pipeline's square roots and
/// row operations at the calibration below, also in `F_p` multiplications
/// (a square root by its measured count, a modular multiply-subtract as
/// one word multiplication).
fn solve_ic_fp4(
    m: &ResolvedMethod,
    inst: &D4Curve,
    curve: &CurveFacts,
    target: Pt4,
    seed: u64,
) -> Result<SolveReport, String> {
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    if fb_name != "fghr-y-line" {
        return Err(format!(
            "factor base `{fb_name}` does not run on a curve over F_(p^4); fghr-y-line does"
        ));
    }
    if or_name != "gaudry-d4" {
        return Err(format!(
            "oracle `{or_name}` does not run on a curve over F_(p^4); gaudry-d4 does"
        ));
    }
    let base = YLineBase { curve: inst };
    let macaulay = spec.oracle_params.u64("macaulay")?;
    let macaulay = u8::try_from(macaulay)
        .ok()
        .filter(|&d| d >= 2)
        .ok_or_else(|| format!("gaudry-d4: macaulay is a degree from 2 to 255, not {macaulay}"))?;
    let mut oracle = D4Oracle::new(inst, macaulay, seed);
    oracle.charge_unsolved = match spec.oracle_params.u64_or("charge_unsolved", 0)? {
        0 => false,
        1 => true,
        v => return Err(format!("gaudry-d4: charge_unsolved is 0 or 1, not {v}")),
    };
    let flag = |k: &str| -> Result<bool, String> {
        match spec.oracle_params.u64_or(k, 0)? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(format!("gaudry-d4: {k} is 0 or 1, not {v}")),
        }
    };
    oracle.opts = D4Options {
        weighted: flag("weighted")?,
        chains: flag("chains")?,
        sparsest: flag("sparsest")?,
    };
    oracle.replay = flag("replay")?;
    let c = &inst.inst.curve;
    let ctx = InstanceCtx {
        group: c,
        generator: inst.generator,
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: curve.slug.clone(),
        field_degree: None,
    };
    let per_add = fp_muls_per_add(c) as f64;
    let calib = Calibration {
        ns_per_add: per_add,
        ns_per_double: per_add,
        ns_per_row_op: 1.0,
        ns_per_lookup: 1.0,
        ns_per_sqrt: Some(fp4_sqrt_muls(inst)),
        pinned_units: vec![
            "ns_per_double".into(),
            "ns_per_row_op".into(),
            "ns_per_lookup".into(),
            "ns_per_sqrt".into(),
        ],
        ..Default::default()
    };
    let t = Instant::now();
    let rep = run_pipeline(&ctx, &spec, &base, &mut oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )?;
    Ok(ic_report(m, rep, &calib, facts, wall, 2))
}

/// `ic.pipeline` on a curve over `F_{(2^m)⁴}` with `bin-t-line` and
/// `gaudry-d4-bin`, priced as [`solve_ic_fp4`] prices its curve: the
/// oracle's `F_q` multiplications at the curve's own multiplications per
/// addition, lifts at their measured count, row operations at one each.
fn solve_ic_f2m4(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    target: PtB,
    seed: u64,
) -> Result<SolveReport, String> {
    solve_ic_f2m4_native(m, inst, curve, target, seed).map(|(rep, _)| rep)
}

/// [`solve_ic_f2m4`], and the native work the oracle did that no count
/// includes (read by `ic.pipeline_f2m4_full` only).
fn solve_ic_f2m4_native(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    target: PtB,
    seed: u64,
) -> Result<(SolveReport, OracleNative), String> {
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    if fb_name != "bin-t-line" {
        return Err(format!(
            "factor base `{fb_name}` does not run on a curve over F_(2^m)^4; bin-t-line does"
        ));
    }
    if or_name != "gaudry-d4-bin" {
        return Err(format!(
            "oracle `{or_name}` does not run on a curve over F_(2^m)^4; gaudry-d4-bin does"
        ));
    }
    let base = TLineBase { curve: inst };
    let mut oracle = bin_oracle(&spec, inst, seed)?;
    let c = &inst.curve;
    let ctx = InstanceCtx {
        group: c,
        generator: inst.generator,
        target,
        r: one_word(inst.r)?,
        cofactor: one_word(inst.cofactor)?,
        group_order: one_word(inst.group_order)?,
        name: curve.slug.clone(),
        field_degree: None,
    };
    let per_add = fq_muls_per_add(c) as f64;
    let calib = Calibration {
        ns_per_add: per_add,
        ns_per_double: per_add,
        ns_per_row_op: 1.0,
        ns_per_lookup: 1.0,
        ns_per_sqrt: Some(f2m4_lift_muls(inst)),
        pinned_units: vec![
            "ns_per_double".into(),
            "ns_per_row_op".into(),
            "ns_per_lookup".into(),
            "ns_per_sqrt".into(),
        ],
        ..Default::default()
    };
    let t = Instant::now();
    let rep = run_pipeline(&ctx, &spec, &base, &mut oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )?;
    let native = oracle.native.clone();
    Ok((ic_report(m, rep, &calib, facts, wall, 2), native))
}

/// A curve over `F_{(2^m)^4}` whose subgroup order needs two words
/// (`m ≥ 16`): `rho.negation` and `ic.pipeline` through `ic_wide`, the
/// same algorithms at `u128` (its tests pin them to the one-word code).
fn solve_f2m4_wide(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ic_wide::rho_negation;
    let f = &inst.curve.f;
    let target = PtB {
        x: f.unpack(tx),
        y: f.unpack(ty),
        inf: false,
    };
    let r = inst.r;
    match m.id.as_str() {
        "rho.negation" => {
            let multiple = param_u64(m, "cap_multiple")? as f64;
            let cap = (crate::cryptanalysis::ic_boundary::generic_floor_ops(r as f64, 1.0)
                * multiple) as u64
                + 4096;
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let (res, d) =
                rho_negation::<crate::cryptanalysis::gaudry_quartic_bin::CurveB, u128>(&inst.curve, inst.generator, target, r, seed, cap);
            measurement::end_online();
            let fops = field_ops_of(&inst.curve, &[res.group_ops]);
            let mut rep = rho_report(res, t.elapsed().as_nanos() as u64, fops);
            rep.recovered = d;
            rep.exhausted = d.is_none();
            Ok(rep)
        }
        "ic.pipeline" => solve_f2m4_wide_ic(m, inst, curve, tx, ty, seed).map(|(rep, _)| rep),
        other => Err(format!(
            "`{other}` has no implementation past one word on a curve over F_(2^m)^4; rho.negation and ic.pipeline do"
        )),
    }
}

/// The wide `ic.pipeline`, and the native work its oracle did that no
/// count includes.
fn solve_f2m4_wide_ic(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<(SolveReport, OracleNative), String> {
    use crate::cryptanalysis::ic_wide::run_f2m4;
    let f = &inst.curve.f;
    let target = PtB {
        x: f.unpack(tx),
        y: f.unpack(ty),
        inf: false,
    };
    let r = inst.r;
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    if fb_name != "bin-t-line" || or_name != "gaudry-d4-bin" {
        return Err("on a wide curve over F_(2^m)^4 only bin-t-line + gaudry-d4-bin run".into());
    }
    let mut oracle = bin_oracle(&spec, inst, seed)?;
    let c = &inst.curve;
    // The one-word fields are not read on this path (the base and
    // the oracle ignore them); r and h travel as u128.
    let ctx = InstanceCtx {
        group: c,
        generator: inst.generator,
        target,
        r: 0,
        cofactor: 0,
        group_order: 0,
        name: curve.slug.clone(),
        field_degree: None,
    };
    let per_add = fq_muls_per_add(c) as f64;
    let calib = Calibration {
        ns_per_add: per_add,
        ns_per_double: per_add,
        ns_per_row_op: 1.0,
        ns_per_lookup: 1.0,
        ns_per_sqrt: Some(f2m4_lift_muls(inst)),
        pinned_units: vec![
            "ns_per_double".into(),
            "ns_per_row_op".into(),
            "ns_per_lookup".into(),
            "ns_per_sqrt".into(),
        ],
        ..Default::default()
    };
    let t = Instant::now();
    let run = run_f2m4::<u128>(&ctx, inst, &spec, &mut oracle, r, inst.cofactor, &calib)?;
    let wall = t.elapsed().as_nanos() as u64;
    let mut scratch = GroupOps::default();
    let base = TLineBase { curve: inst };
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )?;
    let oracle_native = oracle.native.clone();
    let mut rep = ic_report(m, run.report, &calib, facts, wall, 2);
    rep.recovered = run.recovered;
    rep.exhausted = run.recovered.is_none();
    Ok((rep, oracle_native))
}

/// `rho.negation_batched` on a curve over `F_{(2^m)^4}`, one word or `u128`.
///
/// Pricing, in the unit `ic.pipeline` uses on this curve: every `F_q`
/// operation the `Gf2q` counter sees (multiplication, squaring and
/// inversion one each: set-up, the Montgomery batches, the batched
/// addition formula, the unbatched starts, escapes and special cases, and
/// the verification) divided by `fq_muls_per_add(curve)`.  The native work
/// with no `F_q` multiplication in it (canonicalisation, hashing, table
/// inserts and queries, coefficient arithmetic mod `r`) is reported as
/// `*_uncharged` counters, so the total is a lower bound until a pricing
/// step prices them.
fn solve_f2m4_rho_batched(
    m: &ResolvedMethod,
    inst: &BinCurve,
    tx: u128,
    ty: u128,
    seed: u64,
    full: bool,
    reference: bool,
    mulmod: Mulmod,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ic_wide::{rho_negation_batched, BatchedRhoParams};
    let c = &inst.curve;
    let f = &c.f;
    let target = PtB {
        x: f.unpack(tx),
        y: f.unpack(ty),
        inf: false,
    };
    let par = BatchedRhoParams {
        walks: param_u64(m, "walks")?.max(1) as usize,
        dp_bits: u32::try_from(param_u64(m, "dp_bits")?).map_err(|_| "dp_bits too large")?,
    };
    let multiple = param_u64(m, "cap_multiple")? as f64;
    let cap = (crate::cryptanalysis::ic_boundary::generic_floor_ops(inst.r as f64, 1.0) * multiple)
        as u64
        + 4096;
    let per_cur = fq_muls_per_add(c) as f64;
    let per_ref = fq_muls_per_add_ref(c) as f64;
    // The divisor the record is priced by.
    let per_add = if reference { per_ref } else { per_cur };
    let fq4_before = c.f.fq4_current_muls();
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let (res, recovered) = if let Ok(r64) = u64::try_from(inst.r) {
        let (res, d) = rho_negation_batched::<u64>(c, inst.generator, target, r64, seed, cap, par);
        (res, d.map(u128::from))
    } else {
        rho_negation_batched::<u128>(c, inst.generator, target, inst.r, seed, cap, par)
    };
    measurement::end_online();
    let wall = t.elapsed().as_nanos() as u64;
    let fq4_muls = c.f.fq4_current_muls() - fq4_before;
    let g = |fq: u64| fq as f64 / per_add;
    // The F_q operations by region, in the pricing of the record.
    let (r_setup, r_total, r_verif) = if reference {
        (res.ref_fq_setup, res.ref_fq_total, res.ref_fq_verification)
    } else {
        (res.fq_setup, res.fq_total, res.fq_verification)
    };
    let walk_fq = if reference {
        res.ref_fq_batch_inversion + res.ref_fq_batched_formula + res.ref_fq_single_ops
    } else {
        res.fq_batch_inversion + res.fq_batched_formula + res.fq_single_ops
    };
    let mut counters = BTreeMap::new();
    for (k, v) in [
        ("walks_parallel", par.walks as u64),
        ("rounds", res.rounds),
        ("walks_started", res.walks_started),
        ("distinguished_points", res.distinguished_points),
        ("useless_collisions", res.useless_collisions),
        ("steps", res.steps),
        ("look_ahead_additions", res.look_ahead_adds),
        ("cycle_escape_doublings", res.escapes),
        ("cycles_length_2", res.cycles_2),
        ("cycles_length_4", res.cycles_4),
        ("cycles_other_length", res.cycles_other),
        ("walks_abandoned_in_a_cycle", res.abandoned),
        ("walks_capped", res.capped),
        ("batched_additions", res.batched_adds),
        ("unbatched_operations", res.single_ops),
        ("batch_inversions", res.batch_inversions),
        ("batch_fq4_multiplications", res.batch_fq4_muls),
        ("jumps", res.jumps),
        ("distinguished_point_bits", res.dp_bits as u64),
        ("setup_additions", res.setup_ops.adds),
        ("setup_doublings", res.setup_ops.doubles),
        ("verification_operations", res.verification_ops),
        ("fq_operations_total", res.fq_total),
        ("fq_operations_setup", res.fq_setup),
        ("fq_operations_batch_inversion", res.fq_batch_inversion),
        ("fq_operations_batched_formula", res.fq_batched_formula),
        ("fq_operations_unbatched", res.fq_single_ops),
        ("fq_operations_verification", res.fq_verification),
        ("fq_muls_per_add", per_add as u64),
        ("canonicalisations_uncharged", res.canonicalisations),
        ("canonical_flips", res.canonical_flips),
        (
            "canonicalisation_fq_additions_uncharged",
            res.canon_fq_additions,
        ),
        ("hash_evaluations_uncharged", res.hash_evals),
        (
            "distinguished_point_table_queries_uncharged",
            res.table_queries,
        ),
        (
            "distinguished_point_table_inserts_uncharged",
            res.table_inserts,
        ),
        ("scalar_mod_r_operations_uncharged", res.scalar_mod_ops),
    ] {
        counters.insert(k.to_string(), v);
    }
    if full {
        // The full method prices these, so they are not `_uncharged`; and
        // it exposes the rest of the native work.
        counters = counters
            .into_iter()
            .map(|(k, v)| (k.trim_end_matches("_uncharged").to_string(), v))
            .collect();
        for (k, v) in [
            ("round_jobs", res.steps + res.look_ahead_adds),
            ("window_scans", res.window_scans),
            ("window_scan_elements", res.window_scan_elems),
            ("window_pushes", res.window_pushes),
            ("scalar_mulmods", res.scalar_mulmods),
            ("scalar_invmods", res.scalar_invmods),
            ("rng_draws", res.rng_draws),
        ] {
            counters.insert(k.to_string(), v);
        }
    }
    let mk = |name: &str, fq: u64, adds: u64, doubles: u64| PhaseRecord {
        name: name.into(),
        adds,
        doubles,
        scalar_mults: 0,
        gae: g(fq),
        wall_ns: None,
        native: BTreeMap::from([("fq_operations".to_string(), fq)]),
    };
    let mut search = mk("search", walk_fq, res.batched_adds + res.single_ops, 0);
    for (k, v) in [
        ("canonicalisations", res.canonicalisations),
        ("hash_evaluations", res.hash_evals),
        ("table_queries", res.table_queries),
        ("table_inserts", res.table_inserts),
        ("scalar_mod_r_operations", res.scalar_mod_ops),
    ] {
        search.native.insert(k.to_string(), v);
    }
    let phases = vec![
        mk("setup", r_setup, res.setup_ops.adds, res.setup_ops.doubles),
        search,
        mk("internal_verification", r_verif, res.verification_ops, 0),
    ];
    let mut rep = SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        total_gae: g(r_total),
        automorphisms_used: 2,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: json!({
            "method": "negation-map r-adding walk, distinguished points, lockstep walks with Montgomery batched inversion",
            "unit": "GAE = F_q operations / fq_muls_per_add",
            "fq_muls_per_add": per_add,
            "expected_steps": res.expected_steps,
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    };
    let code_mulmod_total = if full && mulmod == Mulmod::Montgomery {
        let mut alt = rep.clone();
        crate::cryptanalysis::ecbench::f2m4_price::reprice_rho_mm(
            &mut alt,
            c.f.f.m,
            u64::try_from(inst.r).is_err(),
            per_add,
            true,
            Mulmod::Code,
        )?;
        Some(alt.total_gae)
    } else {
        None
    };
    if full {
        crate::cryptanalysis::ecbench::f2m4_price::reprice_rho_mm(
            &mut rep,
            c.f.f.m,
            u64::try_from(inst.r).is_err(),
            per_add,
            true,
            mulmod,
        )?;
    }
    if reference {
        let fme = rep.detail["full_pricing"]["native_gae"]
            .as_f64()
            .unwrap_or(0.0)
            * per_add;
        let current = res.fq_total as f64 / per_cur + fme / per_cur;
        let total_ref = rep.total_gae;
        let per_poly = fq_muls_per_add_ref_poly(c) as f64;
        let total_poly = (res.poly_fq_total as f64 + fme) / per_poly;
        add_reference_block(
            &mut rep,
            json!({
                "pricing": "reference",
                "fq_muls_per_add_current": per_cur,
                "fq_muls_per_add_reference": per_ref,
                "reference_model": reference_model_note(c, per_ref),
                "poly_free_const": json!({
                    "model": "polynomial-basis F_q, free constant multiplication (second model, not this id's total)",
                    "fq_muls_per_add": per_poly,
                    "fq_operations_total": res.poly_fq_total,
                    "total_gae": total_poly,
                    "ratio_over_primary": total_poly / total_ref,
                }),
                "total_gae_current_pricing": current,
                "total_gae_reference_pricing": total_ref,
                "ratio_reference_over_current": total_ref / current,
                "fq_operations_total_current": res.fq_total,
                "fq_operations_total_reference": res.ref_fq_total,
                "fq4_share_of_current_fq_operations": fq4_muls as f64 / res.fq_total.max(1) as f64,
                "phases_note": "setup, search and verification are the walk's own F_q operations in the reference pricing; the natives phase is native work divided by the reference divisor",
                "mulmod": mulmod_note(mulmod, code_mulmod_total, total_ref),
            }),
        );
    }
    Ok(rep)
}

/// Which reference model priced a `*_ref` / `*_refmm` record, and the
/// per-primitive costs and divisor it used at this `m`.  The comparison uses
/// the log-table tower model (`RefModel::Tower`, divisor 50 at odd `m`, 64
/// at even `m`) because the field the code runs in is a log-table field,
/// where a constant multiple costs a multiplication; the polynomial-basis
/// model with free constant multiplication (42 at every `m`) is recorded
/// beside it so a reader can see both.  Both are applied identically to the
/// rho and IC arms.
fn reference_model_note(
    c: &crate::cryptanalysis::gaudry_quartic_bin::CurveB,
    per_ref: f64,
) -> Value {
    let (a, b) = (c.f.ref_costs(), c.f.ref_costs_poly());
    let costs = |k: crate::cryptanalysis::gaudry_quartic_bin::RefCosts| {
        json!({"mul": k.mul, "sq": k.sq, "scale": k.scale, "frob": k.frob,
               "inv": k.inv, "sqrt": k.sqrt, "add": k.add()})
    };
    json!({
        "used": "log_table_tower",
        "m": c.f.f.m,
        "m_parity": if c.f.f.m % 2 == 0 { "even (u^2+u+1 reducible, epsilon != 1)" } else { "odd (epsilon = 1)" },
        "divisor": per_ref,
        "tower_costs": costs(a),
        "poly_free_const_costs": costs(b),
    })
}

/// The record of which mod-`r` multiplication priced a `*_ref`/`*_refmm`
/// record, with the total under the code's mulmod beside it when the
/// Montgomery price was used (reference unit, same run).
fn mulmod_note(mulmod: Mulmod, code_total: Option<f64>, total: f64) -> Value {
    match mulmod {
        Mulmod::Code => json!({"pricing": "code", "units": "f2m4_full_prices.json"}),
        Mulmod::Montgomery => json!({
            "pricing": "montgomery",
            "units": "f2m4_mulmod_prices.json (scalar_mulmod, row_op, lp_mulmod)",
            "total_gae_reference_unit_montgomery_mulmod": total,
            "total_gae_reference_unit_code_mulmod": code_total,
            "ratio_montgomery_over_code": code_total.map(|c| total / c),
        }),
    }
}

/// Attach the side-by-side pricing block of a `*_ref` record.
fn add_reference_block(rep: &mut SolveReport, block: Value) {
    match &mut rep.detail {
        Value::Object(d) => {
            d.insert("reference_pricing".into(), block);
        }
        other => {
            let old = std::mem::take(other);
            *other = json!({"base": old, "reference_pricing": block});
        }
    }
}

/// `rho.negation_f2m4_full`: `rho.negation`'s walk (`ic_wide::rho_negation`
/// is pinned to it step for step, at one word and at `u128`), run for its
/// native tallies, and priced in `F_q`-multiplication equivalents.  The
/// group operations are charged one affine addition each, as
/// `rho.negation` charges them.
fn solve_f2m4_rho_negation_full(
    m: &ResolvedMethod,
    inst: &BinCurve,
    tx: u128,
    ty: u128,
    seed: u64,
    reference: bool,
    mulmod: Mulmod,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ic_wide::rho_negation_tallied;
    let c = &inst.curve;
    let f = &c.f;
    let target = PtB {
        x: f.unpack(tx),
        y: f.unpack(ty),
        inf: false,
    };
    let multiple = param_u64(m, "cap_multiple")? as f64;
    let cap = (crate::cryptanalysis::ic_boundary::generic_floor_ops(inst.r as f64, 1.0) * multiple)
        as u64
        + 4096;
    let per_cur = fq_muls_per_add(c) as f64;
    let per_ref = fq_muls_per_add_ref(c) as f64;
    let per_add = if reference { per_ref } else { per_cur };
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let (res, recovered, native) = if let Ok(r64) = u64::try_from(inst.r) {
        let (res, d, n) = rho_negation_tallied::<_, u64>(c, inst.generator, target, r64, seed, cap);
        (res, d.map(u128::from), n)
    } else {
        rho_negation_tallied::<_, u128>(c, inst.generator, target, inst.r, seed, cap)
    };
    measurement::end_online();
    let wall = t.elapsed().as_nanos() as u64;
    let mut rep = rho_report(res, wall, None);
    rep.recovered = recovered;
    rep.exhausted = recovered.is_none();
    let mut counters: BTreeMap<String, u64> = rep
        .counters
        .iter()
        .map(|(k, v)| (k.trim_end_matches("_uncharged").to_string(), *v))
        .collect();
    for (k, v) in [
        ("key_evaluations", native.key_evals),
        ("mix_evaluations", native.mix_evals),
        ("negation_evaluations", native.neg_evals),
        ("canonical_flips", native.canonical_flips),
        ("scalar_mod_r_operations", native.scalar_mod_ops),
        ("window_scans", native.window_scans),
        ("window_scan_elements", native.window_scan_elems),
        ("window_pushes", native.window_pushes),
        ("distinguished_point_table_queries", native.table_queries),
        ("distinguished_point_table_inserts", native.table_inserts),
        ("scalar_mulmods", native.scalar_mulmods),
        ("scalar_invmods", native.scalar_invmods),
        ("rng_draws", native.rng_draws),
        ("walks_started", native.starts),
        ("fq_muls_per_add", per_add as u64),
    ] {
        counters.insert(k.to_string(), v);
    }
    rep.counters = counters;
    rep.unpriced.clear();
    let code_mulmod_total = if mulmod == Mulmod::Montgomery {
        let mut alt = rep.clone();
        crate::cryptanalysis::ecbench::f2m4_price::reprice_rho_mm(
            &mut alt,
            c.f.f.m,
            u64::try_from(inst.r).is_err(),
            per_add,
            false,
            Mulmod::Code,
        )?;
        Some(alt.total_gae)
    } else {
        None
    };
    crate::cryptanalysis::ecbench::f2m4_price::reprice_rho_mm(
        &mut rep,
        c.f.f.m,
        u64::try_from(inst.r).is_err(),
        per_add,
        false,
        mulmod,
    )?;
    if reference {
        // The group operations are one GAE each in either unit; only the
        // native work's divisor moves.
        let fme = rep.detail["full_pricing"]["native_gae"]
            .as_f64()
            .unwrap_or(0.0)
            * per_add;
        let counted = rep.detail["full_pricing"]["counted_gae"]
            .as_f64()
            .unwrap_or(0.0);
        let current = counted + fme / per_cur;
        let total_ref = rep.total_gae;
        let per_poly = fq_muls_per_add_ref_poly(c) as f64;
        let total_poly = counted + fme / per_poly;
        add_reference_block(
            &mut rep,
            json!({
                "pricing": "reference",
                "fq_muls_per_add_current": per_cur,
                "fq_muls_per_add_reference": per_ref,
                "reference_model": reference_model_note(c, per_ref),
                "poly_free_const": json!({
                    "model": "polynomial-basis F_q, free constant multiplication (second model, not this id's total)",
                    "fq_muls_per_add": per_poly,
                    "total_gae": total_poly,
                    "ratio_over_primary": total_poly / total_ref,
                }),
                "total_gae_current_pricing": current,
                "total_gae_reference_pricing": total_ref,
                "ratio_reference_over_current": total_ref / current,
                "note": "rho.negation charges one GAE per group operation whatever the arithmetic; only native work (FME / divisor) moves",
                "mulmod": mulmod_note(mulmod, code_mulmod_total, total_ref),
            }),
        );
    }
    Ok(rep)
}

/// `ic.pipeline_f2m4_full`: the `ic.pipeline` run on a curve over
/// `F_{(2^m)^4}` (one word or `u128`), unchanged, then every phase repriced
/// with the native work its oracle tallied.
fn solve_f2m4_ic_full(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    tx: u128,
    ty: u128,
    seed: u64,
) -> Result<SolveReport, String> {
    solve_f2m4_ic_full_with(
        m,
        inst,
        curve,
        tx,
        ty,
        seed,
        u64::try_from(inst.r).is_err(),
        false,
        Mulmod::Code,
    )
}

/// [`solve_f2m4_ic_full`], on the `u128` pipeline when `wide` (which any
/// order may take; the one-word run is `run_pipeline`).
fn solve_f2m4_ic_full_with(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    tx: u128,
    ty: u128,
    seed: u64,
    wide: bool,
    reference: bool,
    mulmod: Mulmod,
) -> Result<SolveReport, String> {
    let (mut rep, native) = if wide {
        solve_f2m4_wide_ic(m, inst, curve, tx, ty, seed)?
    } else {
        let f = &inst.curve.f;
        let target = PtB {
            x: f.unpack(tx),
            y: f.unpack(ty),
            inf: false,
        };
        solve_ic_f2m4_native(m, inst, curve, target, seed)?
    };
    let c = &inst.curve;
    let facts = rep.factor_base.clone().ok_or("no factor base facts")?;
    let lp_estimate = rep.detail["decomposition"]["solver"]["extra"]["lp_merge_mulmods"]
        .as_u64()
        .unwrap_or(0);
    let solver_extra = |k: &str| -> u64 {
        rep.detail["decomposition"]["solver"]["extra"][k]
            .as_u64()
            .unwrap_or(0)
    };
    // The Weil-restriction system builds and the precomputation, which the
    // pipeline counted in schoolbook F_{q^4} multiplications: their
    // counted size and the reference-priced size under each model.
    let (weil_cur, pre_cur) = (
        solver_extra("weil_fq_muls"),
        solver_extra("precompute_fq_muls"),
    );
    let (weil_ref, pre_ref) = (
        solver_extra("weil_ref_fq_muls"),
        solver_extra("precompute_ref_fq_muls"),
    );
    let (weil_poly, pre_poly) = (
        solver_extra("weil_ref_poly_fq_muls"),
        solver_extra("precompute_ref_poly_fq_muls"),
    );
    // The `x_of_t` before each base lift: its counted multiplications.
    let (mut muls, mut rmuls, mut pmuls, mut n) = (0u64, 0u64, 0u64, 0u64);
    for t in (2..c.f.f.q)
        .step_by(((c.f.f.q / 64).max(1)) as usize)
        .take(64)
    {
        let before = (c.f.f.muls(), c.f.ref_muls(), c.f.ref_muls_poly());
        let _ = c.x_of_t(&c.f.from_fq(t));
        muls += c.f.f.muls() - before.0;
        rmuls += c.f.ref_muls().wrapping_sub(before.1);
        pmuls += c.f.ref_muls_poly().wrapping_sub(before.2);
        n += 1;
    }
    let ctx_mm =
        |ref_unit: bool, mulmod: Mulmod| crate::cryptanalysis::ecbench::f2m4_price::IcContext {
            mulmod,
            m: c.f.f.m,
            wide: u64::try_from(inst.r).is_err(),
            per_add: if ref_unit {
                fq_muls_per_add_ref(c)
            } else {
                fq_muls_per_add(c)
            } as f64,
            lift_muls: f2m4_lift_muls_with(inst, ref_unit),
            x_of_t_muls: (if ref_unit { rmuls } else { muls }) as f64 / n.max(1) as f64,
            oracle: &native,
            fb_points: facts.signed_points,
            fb_columns: facts.columns,
            fb_abscissae: facts.abscissae,
            lp_estimate,
            f4_in_solver: if ref_unit { weil_cur + pre_cur } else { 0 },
            f4_in_solver_priced: if ref_unit {
                (weil_ref + pre_ref) as f64
            } else {
                0.0
            },
        };
    // The unit's own mulmod applies to the primary record only; the
    // current-unit comparison block always prices the code's.
    let ctx_of = |ref_unit: bool| ctx_mm(ref_unit, if ref_unit { mulmod } else { Mulmod::Code });
    if !reference {
        crate::cryptanalysis::ecbench::f2m4_price::reprice_ic(&mut rep, &ctx_of(false))?;
        return Ok(rep);
    }
    // The current-unit total, from a copy of the unpriced report.
    let mut current = rep.clone();
    crate::cryptanalysis::ecbench::f2m4_price::reprice_ic(&mut current, &ctx_of(false))?;
    let (cur, refc) = (ctx_of(false), ctx_of(true));
    let natives = |k: &str| -> u64 {
        rep.phases
            .iter()
            .map(|p| p.native.get(k).copied().unwrap_or(0))
            .sum()
    };
    let (solver_ops, lifts) = (natives("solver_ops"), natives("sqrt_solves"));
    let solver_fq = (solver_ops - lp_estimate.min(solver_ops)) as f64;
    // The F_{q^4} share of the F_q operations: lifts, x_of_t, and the
    // Weil system builds and precomputation in the context's own unit
    // (the current unit leaves the latter inside the solver count).
    let share = |c: &crate::cryptanalysis::ecbench::f2m4_price::IcContext| {
        let f4 = lifts as f64 * (c.lift_muls + c.x_of_t_muls) + c.f4_in_solver_priced;
        let rest = solver_fq - c.f4_in_solver.min(solver_fq as u64) as f64;
        f4 / (f4 + rest).max(1.0)
    };
    let code_mulmod_total = if mulmod == Mulmod::Montgomery {
        let mut alt = rep.clone();
        crate::cryptanalysis::ecbench::f2m4_price::reprice_ic(
            &mut alt,
            &ctx_mm(true, Mulmod::Code),
        )?;
        Some(alt.total_gae)
    } else {
        None
    };
    crate::cryptanalysis::ecbench::f2m4_price::reprice_ic(&mut rep, &refc)?;
    let total_ref = rep.total_gae;
    // The second model, the same run and the same mod-r pricing.
    let polyc = crate::cryptanalysis::ecbench::f2m4_price::IcContext {
        per_add: fq_muls_per_add_ref_poly(c) as f64,
        lift_muls: f2m4_lift_muls_poly(inst),
        x_of_t_muls: pmuls as f64 / n.max(1) as f64,
        f4_in_solver_priced: (weil_poly + pre_poly) as f64,
        ..ctx_of(true)
    };
    let mut poly_rep = rep.clone();
    crate::cryptanalysis::ecbench::f2m4_price::reprice_ic(&mut poly_rep, &polyc)?;
    let total_poly = poly_rep.total_gae;
    add_reference_block(
        &mut rep,
        json!({
            "pricing": "reference",
            "fq_muls_per_add_current": cur.per_add,
            "fq_muls_per_add_reference": refc.per_add,
            "reference_model": reference_model_note(c, refc.per_add),
            "poly_free_const": json!({
                "model": "polynomial-basis F_q, free constant multiplication (second model, not this id's total)",
                "fq_muls_per_add": polyc.per_add,
                "lift_plus_x_of_t_fq_muls": polyc.lift_muls + polyc.x_of_t_muls,
                "fq4_share_of_fq_operations": share(&polyc),
                "total_gae": total_poly,
                "ratio_over_primary": total_poly / total_ref,
            }),
            "weil_restriction": json!({
                "note": "the Weil-restriction system builds and the interpolation precomputation are F_{q^4} work: the pipeline counts them in this file's schoolbook multiplications; this reference unit prices the same operations at the reference costs before dividing by the divisor",
                "weil_muls": weil_cur,
                "weil_ref_muls": weil_ref,
                "weil_ref_poly_muls": weil_poly,
                "precompute_muls": pre_cur,
                "precompute_ref_muls": pre_ref,
                "precompute_ref_poly_muls": pre_poly,
                "ratio_ref_over_counted": (weil_ref + pre_ref) as f64 / (weil_cur + pre_cur).max(1) as f64,
                "gae_counted_at_reference_divisor": (weil_cur + pre_cur) as f64 / refc.per_add,
                "gae_reference_priced": (weil_ref + pre_ref) as f64 / refc.per_add,
                "gae_poly_free_const": (weil_poly + pre_poly) as f64 / polyc.per_add,
            }),
            "lift_plus_x_of_t_fq_muls_current": cur.lift_muls + cur.x_of_t_muls,
            "lift_plus_x_of_t_fq_muls_reference": refc.lift_muls + refc.x_of_t_muls,
            "base_lifts": lifts,
            "solver_fq_operations": solver_fq,
            "fq4_share_of_fq_operations_current": share(&cur),
            "fq4_share_of_fq_operations_reference": share(&refc),
            "total_gae_current_pricing": current.total_gae,
            "total_gae_reference_pricing": total_ref,
            "ratio_reference_over_current": total_ref / current.total_gae,
            "mulmod": mulmod_note(mulmod, code_mulmod_total, total_ref),
        }),
    );
    Ok(rep)
}

/// `gaudry-d4-bin`'s parameters, checked, as an oracle on `inst`; shared
/// by the one-word and the wide pipelines.
fn bin_oracle<'a>(
    spec: &crate::cryptanalysis::ic_framework::PipelineSpec,
    inst: &'a BinCurve,
    seed: u64,
) -> Result<BinD4Oracle<'a>, String> {
    let macaulay = spec.oracle_params.u64("macaulay")?;
    let macaulay = u8::try_from(macaulay)
        .ok()
        .filter(|&d| d >= 2)
        .ok_or_else(|| {
            format!("gaudry-d4-bin: macaulay is a degree from 2 to 255, not {macaulay}")
        })?;
    let flag = |k: &str| -> Result<bool, String> {
        match spec.oracle_params.u64_or(k, 0)? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(format!("gaudry-d4-bin: {k} is 0 or 1, not {v}")),
        }
    };
    let frobenius = flag("frobenius")?;
    let opts = BinOptions {
        weighted: flag("weighted")? || frobenius,
        frobenius,
        sparsest: flag("sparsest")?,
        replay: flag("replay")?,
    };
    let mut oracle = BinD4Oracle::new(inst, macaulay, seed, opts);
    oracle.charge_unsolved = flag("charge_unsolved")?;
    let large_primes = spec.oracle_params.u64_or("large_primes", 0)?;
    if large_primes > 2 {
        return Err(format!(
            "gaudry-d4-bin: large_primes is 0, 1 or 2, not {large_primes}"
        ));
    }
    let small_permille = spec.oracle_params.u64_or("small_permille", 1000)?;
    if !(1..=1000).contains(&small_permille) {
        return Err(format!(
            "gaudry-d4-bin: small_permille is 1 to 1000, not {small_permille}"
        ));
    }
    oracle.lp.max_large = large_primes as u8;
    oracle.lp.small_permille = small_permille;
    Ok(oracle)
}

/// `F_q` multiplications in one lift (`lift_x`) of a random abscissa
/// that lifts, averaged over 32.
fn f2m4_lift_muls(inst: &BinCurve) -> f64 {
    f2m4_lift_muls_with(inst, false)
}

/// [`f2m4_lift_muls`], in the reference pricing when `reference`
/// (`Fq4::ref_muls` instead of the counted multiplications).
fn f2m4_lift_muls_with(inst: &BinCurve, reference: bool) -> f64 {
    let c = &inst.curve;
    f2m4_lift_muls_tally(inst, || {
        if reference {
            c.f.ref_muls()
        } else {
            c.f.f.muls()
        }
    })
}

/// [`f2m4_lift_muls_with`] under the polynomial-basis model.
fn f2m4_lift_muls_poly(inst: &BinCurve) -> f64 {
    f2m4_lift_muls_tally(inst, || inst.curve.f.ref_muls_poly())
}

fn f2m4_lift_muls_tally(inst: &BinCurve, tally: impl Fn() -> u64) -> f64 {
    let c = &inst.curve;
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0x5e);
    let (mut muls, mut n) = (0u64, 0u64);
    while n < 32 {
        let x = c.f.random(&mut rng);
        let before = tally();
        if c.lift_x(&x, &mut rng).is_some() {
            muls += tally().wrapping_sub(before);
            n += 1;
        }
    }
    muls as f64 / n as f64
}

/// `F_p` multiplications of one square root in `F_{p⁴}`, the mean over a
/// fixed sample (the sampler is seeded): the price of a base lift.
fn fp4_sqrt_muls(inst: &D4Curve) -> f64 {
    let c = &inst.inst.curve;
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0x5e);
    let (mut muls, mut n) = (0u64, 0u64);
    while n < 32 {
        let x = c.f.random(&mut rng);
        let before = c.f.muls();
        if c.lift_x(&x, &mut rng).is_some() {
            muls += c.f.muls() - before;
            n += 1;
        }
    }
    muls as f64 / n as f64
}

/// A binary-curve factor base that a linking binary adds to `ic.pipeline`
/// by name, or `None` when the name is not its.
pub type BinaryBasePlugin = for<'i> fn(
    &str,
    &'i BinaryInstance,
) -> Option<Box<dyn FactorBaseBuilder<BinaryGroup<'i>> + 'i>>;

/// A binary-curve decomposition oracle that a linking binary adds to
/// `ic.pipeline` by name, given the summand count `m`, or `None`.
pub type BinaryOraclePlugin =
    for<'i> fn(
        &str,
        u32,
        &'i BinaryInstance,
    ) -> Option<Box<dyn DecompositionOracle<BinaryGroup<'i>> + 'i>>;

/// Plug-ins a binary registers at start-up.
///
/// The library cannot name everything a binary may link. The `ecbench`
/// binary's `pdp3-koblitz` oracle calls a `koblitz_index_calculus`
/// function that the frozen-source replays' snapshots of that file lack,
/// and those replays rebuild this library (never the `ecbench` binary)
/// against them. Keeping such an oracle in the binary and resolving it here
/// by name lets both builds compile, without a Cargo feature (`Cargo.toml`
/// is pinned by other frozen evaluations) or a build script.
#[derive(Clone, Copy)]
pub struct BinaryPlugins {
    pub base: BinaryBasePlugin,
    pub oracle: BinaryOraclePlugin,
}

static BINARY_PLUGINS: std::sync::OnceLock<BinaryPlugins> = std::sync::OnceLock::new();

/// Register a binary's plug-ins; the first registration wins.
pub fn register_binary_plugins(plugins: BinaryPlugins) {
    let _ = BINARY_PLUGINS.set(plugins);
}

fn binary_plugins() -> Option<&'static BinaryPlugins> {
    BINARY_PLUGINS.get()
}

fn solve_ic_binary(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    let g = BinaryGroup(&inst.fast);
    let subspace = BinarySubspaceBase { instance: inst };
    let orbit = KoblitzOrbitBase { instance: inst };
    let compact = CompactOrbitScanBase { instance: inst };
    let cyclotomic = CyclotomicBase { instance: inst };
    let plugin_base = binary_plugins().and_then(|p| (p.base)(fb_name.as_str(), inst));
    let base: &dyn FactorBaseBuilder<BinaryGroup> = match fb_name.as_str() {
        "binary-subspace" => &subspace,
        "koblitz-orbit" => &orbit,
        "compact-orbit-scan" => &compact,
        "cyclotomic" => &cyclotomic,
        other => match plugin_base.as_deref() {
            Some(b) => b,
            None => {
                return Err(format!(
                    "factor base `{other}` does not run on a binary curve"
                ))
            }
        },
    };
    let ms = spec.oracle_params.u64_or("m", 2)? as u32;
    let mut subtract = SubtractOracle;
    let mut mitm = MitmOracle::new(ms);
    let mut frob = FrobeniusMitmOracle::new(ms, inst);
    let mut frob_counted = FrobeniusMitmOracle::new_counted(ms, inst);
    let mut mitm_fiber = FiberMitmOracle::new(ms);
    let mut frob_fiber = FiberFrobeniusMitmOracle::new(ms, inst);
    let mut rootfind_mitm = RootfindMitmOracle::new(ms);
    let mut rootfind_pairs = RootfindPairsOracle::new(ms);
    let mut descent_fiber = match or_name.as_str() {
        "descent-algebraic-fiber" => {
            let name = spec
                .solver
                .as_deref()
                .ok_or("descent-algebraic-fiber needs `solver`")?;
            let budget = (spec.solver_budget_seconds > 0)
                .then(|| std::time::Duration::from_secs(spec.solver_budget_seconds));
            Some(FiberDescentOracle::new(
                ms,
                inst,
                solver_by_name(name)?,
                spec.solver_params.clone(),
                budget,
            ))
        }
        _ => None,
    };
    let mut plugin_oracle = binary_plugins().and_then(|p| (p.oracle)(or_name.as_str(), ms, inst));
    let mut algebraic = match or_name.as_str() {
        "descent-algebraic" => {
            let name = spec
                .solver
                .as_deref()
                .ok_or("descent-algebraic needs `solver`")?;
            let budget = (spec.solver_budget_seconds > 0)
                .then(|| std::time::Duration::from_secs(spec.solver_budget_seconds));
            Some(DescentAlgebraicOracle::new(
                ms,
                inst,
                solver_by_name(name)?,
                spec.solver_params.clone(),
                budget,
            ))
        }
        _ => None,
    };
    let oracle: &mut dyn DecompositionOracle<BinaryGroup> = match or_name.as_str() {
        "subtract" => &mut subtract,
        "mitm" => &mut mitm,
        "mitm-frobenius" => &mut frob,
        "mitm-frobenius-counted" => &mut frob_counted,
        "mitm-fiber" => &mut mitm_fiber,
        "mitm-frobenius-fiber" => &mut frob_fiber,
        "rootfind-mitm" => &mut rootfind_mitm,
        "rootfind-pairs" => &mut rootfind_pairs,
        "descent-algebraic" => algebraic.as_mut().expect("built above"),
        "descent-algebraic-fiber" => descent_fiber.as_mut().expect("built above"),
        other => match plugin_oracle.as_deref_mut() {
            Some(o) => o,
            None => return Err(format!("oracle `{other}` does not run on a binary curve")),
        },
    };
    let ctx = InstanceCtx {
        group: &g,
        generator: inst.generator,
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: curve.slug.clone(),
        field_degree: Some(inst.n),
    };
    let regime = if inst.koblitz.is_some() {
        "koblitz"
    } else {
        "char2"
    };
    let calib = pinned_only_calibration(regime, &curve.slug);
    let t = Instant::now();
    let rep = run_pipeline(&ctx, &spec, base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |p| g.key(p),
    )?;
    let a = if inst.koblitz.is_some() {
        2 * inst.n
    } else {
        2
    };
    Ok(ic_report(m, rep, &calib, facts, wall, a))
}

/// The reusable, target-blind rank variant.  The library owns its exclusive
/// five-phase target clock; `ecbench` retains that clock instead of trying to
/// infer the online interval from the whole (mostly setup) process.
fn solve_shared_rank(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    if inst.koblitz.is_none() {
        return Err("ic.shared_rank needs a Koblitz instance".into());
    }
    let (name, params) = Params::parse_spec(&m.params["factor_base"])?;
    if name != "compact-orbit-scan"
        || params.0.len() != 2
        || !params.0.contains_key("columns")
        || !params.0.contains_key("raw_x_cap")
    {
        return Err("ic.shared_rank needs exactly compact-orbit-scan:columns=N,raw_x_cap=M".into());
    }
    let rank_spec = SharedRankSpec {
        columns: usize::try_from(params.u64("columns")?).map_err(|_| "columns do not fit usize")?,
        raw_x_cap: params.u64("raw_x_cap")?,
        rank_seed: param_u64(m, "rank_seed")?,
        max_trials: param_u64(m, "rank_max_trials")?,
    };
    let target_spec = SharedTargetSpec {
        residual_seed: seed,
        max_attempts: u32::try_from(param_u64(m, "target_max_attempts")?)
            .map_err(|_| "target_max_attempts do not fit u32")?,
    };
    let calib = pinned_only_calibration("koblitz", &curve.slug);
    let mut point_bytes = b"ecbench-shared-rank-point-v1\0".to_vec();
    point_bytes.extend_from_slice(&target.x.to_le_bytes());
    point_bytes.extend_from_slice(&target.y.to_le_bytes());
    let started = Instant::now();
    let report = run_shared_rank_targets(
        inst,
        &rank_spec,
        &target_spec,
        &[target],
        sha256_hex(&point_bytes),
        &calib,
    )?;
    let wall = started.elapsed().as_nanos() as u64;
    let row = report
        .targets
        .first()
        .ok_or("shared rank returned no target")?;
    let recovered = row.recovered_log.map(u128::from);
    let phases = vec![
        phase_of("factor_base", &report.rank.base),
        phase_of("oracle_setup", &report.rank.table),
        phase_of("relations", &report.rank.rank_search),
        phase_of("linear_algebra", &report.rank.linear_algebra),
        phase_of("verify", &report.rank.verification),
        phase_of("target_query", &row.query),
        phase_of("target_PDP", &row.pdp),
        phase_of("target_relation_check", &row.relation_check),
        phase_of("target_descent", &row.descent),
        phase_of("target_recovery_check", &row.recovery_check),
    ];
    let mut counters = BTreeMap::new();
    counters.insert("rank_trials".into(), report.rank.trials);
    counters.insert("rank_hits".into(), report.rank.hits);
    counters.insert("matrix_rank".into(), report.rank.rank as u64);
    counters.insert("target_attempts".into(), row.attempts.len() as u64);
    let mut unpriced = Vec::new();
    for phase in &phases {
        for (name, &count) in &phase.native {
            if count == 0 {
                continue;
            }
            let priced = PRICED_NATIVE
                .iter()
                .find(|(counter, _)| *counter == name)
                .is_some_and(|(_, unit)| calib.is_pinned(unit));
            if !priced {
                let label = if name.ends_with("_uncharged") {
                    name.clone()
                } else {
                    format!("{name}_uncharged")
                };
                *counters.entry(label.clone()).or_insert(0) += count;
                unpriced.push(label);
            }
        }
    }
    // The current group-addition-equivalent calibration does not account
    // for these operations, even if every named native counter is pinned.
    unpriced.extend([
        "field_arithmetic_uncharged".into(),
        "hash_and_allocation_uncharged".into(),
        "modular_combination_uncharged".into(),
    ]);
    unpriced.sort();
    unpriced.dedup();
    let builder = CompactOrbitScanBase { instance: inst };
    let group = BinaryGroup(&inst.fast);
    let ctx = InstanceCtx {
        group: &group,
        generator: inst.generator,
        target: group.identity(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: curve.slug.clone(),
        field_degree: Some(inst.n),
    };
    let mut scratch = GroupOps::default();
    let fb = builder.build(&ctx, &params, &mut scratch)?;
    let facts = factor_base_facts(&curve.slug, &m.params["factor_base"], &params, &fb, |p| {
        group.key(p)
    })?;
    if facts.columns != report.rank.columns as u64
        || facts.signed_points != report.rank.signed_points as u64
    {
        return Err("shared rank factor-base inventory differs from solver setup".into());
    }
    let online_names = [
        ("target_query", row.query.wall_ns),
        ("target_PDP", row.pdp.wall_ns),
        ("target_relation_check", row.relation_check.wall_ns),
        ("target_descent", row.descent.wall_ns),
        ("target_recovery_check", row.recovery_check.wall_ns),
    ];
    let online = OnlineWindow {
        wall_ns: row.online_wall_ns,
        phases_ns: online_names
            .iter()
            .map(|(name, ns)| ((*name).into(), *ns))
            .collect(),
        zero_phases: online_names
            .iter()
            .filter(|(_, ns)| *ns == 0)
            .map(|(name, _)| ((*name).into(), "phase performed no work".into()))
            .collect(),
        start_event: "Q subgroup validation after target-blind base, table and rank logs are ready"
            .into(),
        stop_event: "[d]G = Q verified by full-point scalar replay".into(),
        included_stages: online_names.iter().map(|(name, _)| (*name).into()).collect(),
        mapping: "direct Q, then run-seeded Q+[a]G residuals; exact m3 folded-table PDP, full-point witness check, column-log combination and full-point scalar replay".into(),
    };
    if online.phases_ns.values().sum::<u64>() != online.wall_ns {
        return Err("shared rank online phases do not sum to online wall interval".into());
    }
    Ok(SolveReport {
        recovered,
        exhausted: !report.verified,
        phases,
        total_gae: report.total_gae,
        automorphisms_used: 2 * inst.n,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: Vec::new(),
        solve_wall_ns: wall,
        factor_base: Some(facts),
        detail: serde_json::to_value(&report).map_err(|e| e.to_string())?,
        online: Some(online),
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

#[cfg(test)]
mod shared_rank_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::CurveSpec;

    #[test]
    fn shared_rank_adapter_keeps_the_one_target_window_and_inventory() {
        let spec = CurveSpec::Koblitz { a: 1, n: 17 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Binary(binary) = &inst else {
            unreachable!()
        };
        let target = binary.fast.mul_u64(binary.generator, 113);
        let method = resolve(&MethodSpec {
            id: "ic.shared_rank".into(),
            params: [
                (
                    "factor_base".into(),
                    "compact-orbit-scan:columns=4,raw_x_cap=10000".into(),
                ),
                ("rank_seed".into(), "7".into()),
                ("rank_max_trials".into(), "10000".into()),
                ("target_max_attempts".into(), "64".into()),
            ]
            .into_iter()
            .collect(),
        })
        .unwrap();
        let report = solve(
            &method,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            19,
        )
        .unwrap();
        assert_eq!(report.recovered, Some(u128::from(113 % binary.r)));
        assert_eq!(report.factor_base.as_ref().unwrap().columns, 4);
        assert!(report
            .unpriced
            .contains(&"field_arithmetic_uncharged".into()));
        let online = report
            .online
            .expect("library online clock survives ecbench");
        assert!(online.wall_ns > 0);
        assert_eq!(online.phases_ns.len(), 5);
        assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
    }
}

// ── Factor-base dumps ──────────────────────────────────────────────

pub const FB_DUMP_SCHEMA: &str = "ecbench.factor_base_dump/v1";

/// One point of a dumped factor base.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FbPoint {
    pub x: String,
    pub y: String,
    /// The relation-matrix column the point contributes to.
    pub col: u64,
    /// Its coefficient in that column (mod r).
    pub coef: u64,
}

/// A factor base with its points: what `ecbench fb` writes and the
/// database's `factor_base_points` table holds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactorBaseDump {
    pub schema: String,
    pub curve: CurveFacts,
    pub factor_base: FactorBaseFacts,
    /// Group operations the builder charged.
    pub build_adds: u64,
    pub build_doubles: u64,
    pub points: Vec<FbPoint>,
}

fn dump_points<E: Copy>(
    fb: &crate::cryptanalysis::ic_boundary::FactorBase<E>,
    xy: impl Fn(&E) -> (u64, u64),
) -> Vec<FbPoint> {
    fb.points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let (x, y) = xy(p);
            FbPoint {
                x: format!("0x{x:x}"),
                y: format!("0x{y:x}"),
                col: fb.col_of.get(i).copied().unwrap_or(0) as u64,
                coef: fb.coef_of.get(i).copied().unwrap_or(0),
            }
        })
        .collect()
}

/// Build the factor base `fb_spec` (a plug-in spec such as
/// `koblitz-orbit:divisor=1`) on `curve` and return it whole.  The id is
/// the one an `ic.pipeline` run with the same `factor_base` reports.
pub fn dump_factor_base(
    curve: &crate::cryptanalysis::ecbench::workload::CurveSpec,
    fb_spec: &str,
) -> Result<FactorBaseDump, String> {
    let inst = curve.build()?;
    let facts = inst.facts(curve);
    let (name, params) = Params::parse_spec(fb_spec)?;
    if let Instance::Wide(_) = &inst {
        return Err("no factor-base plug-in runs on a wide (n > 62) curve yet".into());
    }
    if let Instance::Fp4(_) | Instance::F2m4(_) = &inst {
        return Err("factor-base dumps cover prime and binary curves".into());
    }
    match &inst {
        Instance::Prime(i) => {
            let abscissa = PrimeAbscissaBase { instance: i };
            let orbit = GlvOrbitBase { instance: i };
            let kummer = PrimeKummerBase { instance: i };
            let base: &dyn FactorBaseBuilder<_> = match name.as_str() {
                "prime-abscissa" => &abscissa,
                "glv-orbit" => &orbit,
                "prime-kummer" => &kummer,
                other => {
                    return Err(format!(
                        "factor base `{other}` does not run on a prime curve"
                    ))
                }
            };
            let ctx = InstanceCtx {
                group: &i.curve,
                generator: i.generator_point(),
                target: i.generator_point(),
                r: i.r,
                cofactor: i.cofactor,
                group_order: i.group_order,
                name: facts.slug.clone(),
                field_degree: None,
            };
            let mut ops = GroupOps::default();
            let fb = base.build(&ctx, &params, &mut ops)?;
            let fbf = factor_base_facts(&facts.slug, fb_spec, &params, &fb, |p| i.curve.key(p))?;
            let total = {
                let mut t = fb.cost.group_ops;
                t.merge(ops);
                t
            };
            Ok(FactorBaseDump {
                schema: FB_DUMP_SCHEMA.into(),
                points: dump_points(&fb, |p| (p.x, p.y)),
                curve: facts,
                factor_base: fbf,
                build_adds: total.adds,
                build_doubles: total.doubles,
            })
        }
        Instance::Binary(i) => {
            let g = BinaryGroup(&i.fast);
            let subspace = BinarySubspaceBase { instance: i };
            let orbit = KoblitzOrbitBase { instance: i };
            let compact = CompactOrbitScanBase { instance: i };
            let cyclotomic = CyclotomicBase { instance: i };
            let plugin_base = binary_plugins().and_then(|p| (p.base)(name.as_str(), i));
            let base: &dyn FactorBaseBuilder<BinaryGroup> = match name.as_str() {
                "binary-subspace" => &subspace,
                "koblitz-orbit" => &orbit,
                "compact-orbit-scan" => &compact,
                "cyclotomic" => &cyclotomic,
                other => match plugin_base.as_deref() {
                    Some(b) => b,
                    None => {
                        return Err(format!(
                            "factor base `{other}` does not run on a binary curve"
                        ))
                    }
                },
            };
            let ctx = InstanceCtx {
                group: &g,
                generator: i.generator,
                target: i.generator,
                r: i.r,
                cofactor: i.cofactor,
                group_order: i.group_order,
                name: facts.slug.clone(),
                field_degree: Some(i.n),
            };
            let mut ops = GroupOps::default();
            let fb = base.build(&ctx, &params, &mut ops)?;
            let fbf = factor_base_facts(&facts.slug, fb_spec, &params, &fb, |p| g.key(p))?;
            let total = {
                let mut t = fb.cost.group_ops;
                t.merge(ops);
                t
            };
            Ok(FactorBaseDump {
                schema: FB_DUMP_SCHEMA.into(),
                points: dump_points(&fb, |p| (p.x, p.y)),
                curve: facts,
                factor_base: fbf,
                build_adds: total.adds,
                build_doubles: total.doubles,
            })
        }
        Instance::Wide(_) | Instance::Fp4(_) | Instance::F2m4(_) => unreachable!("returned above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_written_before_hashing() {
        let a = resolve(&MethodSpec {
            id: "rho.negation".into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        let mut p = BTreeMap::new();
        p.insert("cap_multiple".to_string(), "64".to_string());
        let b = resolve(&MethodSpec {
            id: "rho.negation".into(),
            params: p,
        })
        .unwrap();
        assert_eq!(a.method_id, b.method_id);
        assert!(a.method_id.starts_with("ECM1h"));
    }

    #[test]
    fn solver_stats_read_the_sat_and_macaulay_figures_by_their_names() {
        use crate::cryptanalysis::ic_framework::stages::SystemShape;
        use crate::cryptanalysis::ic_framework::SolverReport;
        let shape = SystemShape {
            n_vars: 10,
            n_equations: 13,
            degrees: vec![2; 13],
            semi_regular_degree: Some(3),
        };
        let mut extra = BTreeMap::new();
        extra.insert("variables_max".to_string(), 132);
        extra.insert("clauses_max".to_string(), 2157);
        extra.insert("decisions".to_string(), 53_360);
        extra.insert("propagations".to_string(), 2_214_306);
        let sat = SolverReport {
            name: "sat-cdcl".into(),
            calls: 179,
            ops: 48_872,
            op_unit: "conflicts".into(),
            extra,
            ..Default::default()
        };
        let s = SolverStats::from_report(&sat, Some(&shape));
        assert_eq!(s.sat_conflicts, Some(48_872));
        assert_eq!(s.sat_variables, Some(132));
        assert_eq!(s.sat_clauses, Some(2157));
        assert_eq!(s.sat_decisions, Some(53_360));
        assert_eq!(
            (s.n_vars, s.n_equations, s.semi_regular_degree),
            (Some(10), Some(13), Some(3))
        );
        assert_eq!(s.macaulay_rows, None);
        // A round trip through the record's JSON keeps every field.
        let back: SolverStats = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(back, s);

        let mut extra = BTreeMap::new();
        extra.insert("matrix_rows_max".to_string(), 4096);
        extra.insert("matrix_cols_max".to_string(), 8192);
        extra.insert("max_poly_degree".to_string(), 4);
        let f4 = SolverReport {
            name: "f4-f2".into(),
            calls: 3,
            ops: 10,
            op_unit: "word XORs".into(),
            solving_degree_max: 3,
            extra,
            ..Default::default()
        };
        let s = SolverStats::from_report(&f4, None);
        assert_eq!(
            (s.macaulay_rows, s.macaulay_columns, s.macaulay_degree),
            (Some(4096), Some(8192), Some(4))
        );
        assert_eq!(s.sat_conflicts, None);
        assert_eq!(s.n_vars, None);
    }

    /// A prime-curve solve reports the field operations behind its group
    /// operations; a Koblitz solve, whose arithmetic does not count them,
    /// reports none — and never a zero.
    #[test]
    fn field_operations_are_reported_on_prime_curves_and_unknown_elsewhere() {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        let m = resolve(&MethodSpec {
            id: "bsgs.negation".into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        let spec = CurveSpec::PrimeSearch {
            bits: 12,
            seed: 59297,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Prime(prime) = &inst else {
            unreachable!()
        };
        let mut scratch = GroupOps::default();
        let target = prime.curve.mul(&mut scratch, prime.generator_point(), 97);
        let rep = solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            5,
        )
        .unwrap();
        assert_eq!(rep.recovered, Some(97));
        let f = rep
            .field_ops
            .expect("a prime curve counts its field operations");
        let group_ops = rep.total_gae;
        // Every addition or doubling that did field work did exactly one
        // inversion, at least two multiplications and at least one
        // squaring; the special cases (∞ + P, P + (−P)) did none.
        assert!(
            f.invs > 0 && f.invs as f64 <= group_ops,
            "{f:?} over {group_ops} group operations"
        );
        assert!(f.muls >= 2 * f.invs && f.sqrs >= f.invs, "{f:?}");
        // The block is its own: counters and phases, which committed
        // replays compare, do not carry it.
        assert!(rep.counters.keys().all(|k| !k.contains("field")));
        let v = serde_json::to_value(&rep).unwrap();
        assert_eq!(v["field_ops"]["invs"], f.invs);

        let spec = CurveSpec::Koblitz { a: 1, n: 17 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Binary(binary) = &inst else {
            unreachable!()
        };
        let target = binary.fast.mul_u64(binary.generator, 113);
        let rep = solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            5,
        )
        .unwrap();
        assert_eq!(rep.recovered, Some(u128::from(113 % binary.r)));
        assert!(rep.field_ops.is_none(), "unknown is not zero");
        let v = serde_json::to_value(&rep).unwrap();
        assert!(v.get("field_ops").is_none(), "absent, not null");
    }

    /// `rootfind-subtract` is `subtract`'s search on `prime_fast`'s
    /// arithmetic: the same candidates in the same order, so the two arms
    /// issue the same queries and find the same relations on one seed,
    /// with one batched inversion a target instead of one per candidate.
    /// Its hits are re-added on the ledger (one addition each), which is
    /// the only charge the two differ by.
    #[test]
    fn rootfind_subtract_follows_subtract_query_stream_on_a_prime_curve() {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        let spec = CurveSpec::PrimeSearch {
            bits: 20,
            seed: 59297,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Prime(prime) = &inst else {
            unreachable!()
        };
        let mut scratch = GroupOps::default();
        let target = prime.curve.mul(&mut scratch, prime.generator_point(), 4099);
        let coords = [format!("0x{:x}", target.x), format!("0x{:x}", target.y)];
        let run = |oracle: &str| {
            let m = resolve(&MethodSpec {
                id: "ic.pipeline".into(),
                params: [
                    (
                        "factor_base".to_string(),
                        "prime-abscissa:size=32".to_string(),
                    ),
                    ("oracle".to_string(), oracle.to_string()),
                ]
                .into_iter()
                .collect(),
            })
            .unwrap();
            solve(&m, &inst, &facts, &coords, 11).unwrap()
        };
        let sub = run("subtract");
        let root = run("rootfind-subtract:m=2");
        assert_eq!(sub.recovered, Some(4099 % u128::from(prime.r)));
        assert_eq!(root.recovered, sub.recovered);
        for key in ["targets_tried", "relations_found", "matrix_rank"] {
            assert_eq!(sub.counters[key], root.counters[key], "{key}");
        }
        let relations = |r: &SolveReport| {
            r.phases
                .iter()
                .find(|p| p.name == "relations")
                .cloned()
                .expect("a relations phase")
        };
        let (s, r) = (relations(&sub), relations(&root));
        // The same candidate points, each one addition; the root finder
        // adds one re-addition per relation and nothing else.
        assert_eq!(
            r.adds,
            s.adds + root.counters["relations_found"],
            "{s:?} {r:?}"
        );
        assert_eq!(r.native["lookups"], s.native["lookups"]);
        // One batched inversion per oracle call; the relation loop's
        // target guard skips a few trials before the oracle sees them.
        let inversions = r.native["inversions"];
        assert!(inversions > 0 && inversions <= root.counters["targets_tried"]);
        assert_eq!(s.native.get("inversions").copied().unwrap_or(0), 0);
        assert_eq!(r.native.get("frobfold_mismatches").copied().unwrap_or(0), 0);
        assert!(root.solver.is_none(), "a table oracle solves no system");
        assert!(root.deterministic);

        // A knob the engine does not read is refused, as is a summand
        // count it does not take.
        for bad in [
            "rootfind-subtract:m=2,negation_folded=1",
            "rootfind-subtract:m=4",
        ] {
            let m = resolve(&MethodSpec {
                id: "ic.pipeline".into(),
                params: [
                    (
                        "factor_base".to_string(),
                        "prime-abscissa:size=32".to_string(),
                    ),
                    ("oracle".to_string(), bad.to_string()),
                ]
                .into_iter()
                .collect(),
            })
            .unwrap();
            assert!(solve(&m, &inst, &facts, &coords, 11).is_err(), "{bad}");
        }
        let m = resolve(&MethodSpec {
            id: "ic.pipeline".into(),
            params: [
                (
                    "factor_base".to_string(),
                    "prime-abscissa:size=32".to_string(),
                ),
                ("oracle".to_string(), "rootfind-subtract:m=3".to_string()),
            ]
            .into_iter()
            .collect(),
        })
        .unwrap();
        let three = solve(&m, &inst, &facts, &coords, 11).unwrap();
        assert_eq!(three.recovered, sub.recovered);
        assert!(three
            .phases
            .iter()
            .any(|p| p.name == "oracle_setup" && p.adds > 0));
    }

    #[test]
    fn unknown_and_missing_parameters_are_refused() {
        let mut p = BTreeMap::new();
        p.insert("jumps".to_string(), "8".to_string());
        assert!(resolve(&MethodSpec {
            id: "bsgs.textbook".into(),
            params: p
        })
        .is_err());
        assert!(resolve(&MethodSpec {
            id: "ic.pipeline".into(),
            params: BTreeMap::new()
        })
        .is_err());
        assert!(resolve(&MethodSpec {
            id: "rho.nope".into(),
            params: BTreeMap::new()
        })
        .is_err());
    }

    // ── Full pricing of the f2m4 / gaudry-d4-bin comparison ─────────────

    fn f2m4_case(m: u32, d: u128) -> (Instance, CurveFacts, [String; 2]) {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        let spec = CurveSpec::F2m4Torsion {
            m,
            seed: 1,
            max_cofactor: 16,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::F2m4(b) = &inst else {
            unreachable!()
        };
        let q = b.curve.mul(&b.generator, d % b.r);
        let f = &b.curve.f;
        let t = [
            format!("0x{:x}", f.pack(&q.x)),
            format!("0x{:x}", f.pack(&q.y)),
        ];
        (inst, facts, t)
    }

    fn method(id: &str, kv: &[(&str, &str)]) -> ResolvedMethod {
        resolve(&MethodSpec {
            id: id.into(),
            params: kv
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        })
        .unwrap()
    }

    fn ic_params(extra: &str) -> Vec<(String, String)> {
        vec![
            ("factor_base".into(), "bin-t-line".into()),
            (
                "oracle".into(),
                format!("gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1{extra}"),
            ),
            ("linalg".into(), "wiedemann".into()),
            ("max_trials".into(), "2000000".into()),
        ]
    }

    fn ic_method(id: &str, extra: &str) -> ResolvedMethod {
        let p = ic_params(extra);
        let kv: Vec<(&str, &str)> = p.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        method(id, &kv)
    }

    /// The block every full record carries: the counted part, the native
    /// part, the band, and a sum that adds up.
    fn check_full_block(rep: &SolveReport) -> (f64, f64, u64) {
        let b = &rep.detail["full_pricing"];
        let counted = b["counted_gae"].as_f64().unwrap();
        let native = b["native_gae"].as_f64().unwrap();
        let ops = b["native_operations"].as_u64().unwrap();
        assert!((rep.total_gae - (counted + native)).abs() <= 1e-9 * rep.total_gae.max(1.0));
        let by: f64 = b["by_counter"]
            .as_object()
            .unwrap()
            .values()
            .map(|v| v["gae"].as_f64().unwrap())
            .sum();
        assert!(
            (by - native).abs() <= 1e-9 * native.max(1.0),
            "{by} {native}"
        );
        let ops_by: u64 = b["by_counter"]
            .as_object()
            .unwrap()
            .values()
            .map(|v| v["count"].as_u64().unwrap())
            .sum();
        assert_eq!(ops_by, ops);
        // The §5a band: native work at 0, 0.1 and 1 GAE per operation.
        let band = &b["band"];
        assert_eq!(
            band["native_gae_per_operation_0"].as_f64().unwrap(),
            counted
        );
        assert!(
            (band["native_gae_per_operation_1"].as_f64().unwrap() - (counted + ops as f64)).abs()
                < 1e-6
        );
        // The comparable band: one unit per rule application, per counter.
        let ra = &b["band_rule_applications"];
        let apps = ra["native_rule_applications"].as_f64().unwrap();
        assert!(apps >= ops as f64 - 1e-9, "{apps} {ops}");
        let apps_by: f64 = ra["by_counter"]
            .as_object()
            .unwrap()
            .values()
            .map(|v| v["applications"].as_f64().unwrap())
            .sum();
        assert!((apps_by - apps).abs() <= 1e-9 * apps.max(1.0));
        assert_eq!(
            ra["native_gae_per_application_0"].as_f64().unwrap(),
            counted
        );
        assert!(
            (ra["native_gae_per_application_1"].as_f64().unwrap() - (counted + apps)).abs() < 1e-6
        );
        assert!(
            (ra["native_gae_weighted"].as_f64().unwrap() - (counted + native)).abs()
                <= 1e-9 * rep.total_gae.max(1.0)
        );
        assert!(ra["legacy_ic"].as_str().unwrap().contains("no band"));
        assert!(rep.unpriced.is_empty(), "{:?}", rep.unpriced);
        (counted, native, ops)
    }

    /// `rho.negation_f2m4_full` walks `rho.negation`'s walk (same counts)
    /// and prices every native counter: no `_uncharged`, no lower bound.
    #[test]
    fn rho_negation_f2m4_full_is_the_same_walk_fully_priced() {
        for m in [6u32, 7] {
            let (inst, facts, t) = f2m4_case(m, 4321);
            let base = solve(&method("rho.negation", &[]), &inst, &facts, &t, 5).unwrap();
            let full = solve(&method("rho.negation_f2m4_full", &[]), &inst, &facts, &t, 5).unwrap();
            assert_eq!(base.recovered, full.recovered);
            assert!(base
                .unpriced
                .contains(&"canonicalisations_uncharged".to_string()));
            for k in [
                "steps",
                "walk_operations",
                "look_ahead_additions",
                "cycle_escape_doublings",
                "cycles_length_2",
                "cycles_length_4",
                "distinguished_points",
                "useless_collisions",
                "walks_capped",
            ] {
                assert_eq!(base.counters.get(k), full.counters.get(k), "{k}");
            }
            assert_eq!(
                base.counters["canonicalisations_uncharged"],
                full.counters["canonicalisations"]
            );
            let (counted, native, ops) = check_full_block(&full);
            // The counted part is exactly what rho.negation charged.
            assert_eq!(counted, base.total_gae);
            assert!(native > 0.0 && ops > 0);
            // Invariants of the tallies: a canonicalisation is a negation
            // and two keys; the table is probed once per distinguished
            // point; every start is a walk.
            let c = &full.counters;
            assert_eq!(c["negation_evaluations"], c["canonicalisations"]);
            assert!(c["key_evaluations"] >= 2 * c["canonicalisations"]);
            assert_eq!(
                c["distinguished_point_table_queries"],
                c["distinguished_points"]
            );
            assert!(c["distinguished_point_table_inserts"] <= c["distinguished_points"]);
            assert!(c["scalar_mod_r_operations"] >= 2 * (c["steps"] + c["look_ahead_additions"]));
        }
    }

    /// `rho.negation_batched_full` is `rho.negation_batched`'s walk (same
    /// counts, same F_q operations) with the native work priced on top.
    #[test]
    fn rho_negation_batched_full_is_the_same_walk_fully_priced() {
        for m in [6u32, 7] {
            let (inst, facts, t) = f2m4_case(m, 777);
            let kv = [("walks", "16")];
            let base = solve(&method("rho.negation_batched", &kv), &inst, &facts, &t, 9).unwrap();
            let full = solve(
                &method("rho.negation_batched_full", &kv),
                &inst,
                &facts,
                &t,
                9,
            )
            .unwrap();
            assert_eq!(base.recovered, full.recovered);
            assert!(!base.unpriced.is_empty(), "the base is a lower bound");
            for k in [
                "steps",
                "rounds",
                "batched_additions",
                "batch_inversions",
                "fq_operations_total",
                "look_ahead_additions",
                "distinguished_points",
            ] {
                assert_eq!(base.counters.get(k), full.counters.get(k), "{k}");
            }
            let (counted, native, ops) = check_full_block(&full);
            assert_eq!(counted, base.total_gae, "the F_q part is unchanged");
            assert!(native > 0.0 && ops > 0);
            let c = &full.counters;
            assert_eq!(c["round_jobs"], c["steps"] + c["look_ahead_additions"]);
        }
    }

    /// The `*_refmm` methods are the `*_ref` methods with the mod-r
    /// multiplications priced from Montgomery: same counts and answer, the
    /// same reference divisor, a different price only for those units.
    #[test]
    fn the_refmm_methods_differ_from_the_ref_methods_in_the_mulmod_units_only() {
        let (inst, facts, t) = f2m4_case(6, 777);
        let kv = [("walks", "16")];
        for (ref_id, mm_id, kv) in [
            (
                "rho.negation_batched_ref",
                "rho.negation_batched_refmm",
                &kv[..],
            ),
            ("rho.negation_f2m4_ref", "rho.negation_f2m4_refmm", &[][..]),
        ] {
            let a = solve(&method(ref_id, kv), &inst, &facts, &t, 9).unwrap();
            let b = solve(&method(mm_id, kv), &inst, &facts, &t, 9).unwrap();
            assert_eq!(a.recovered, b.recovered);
            assert_eq!(a.counters, b.counters);
            let n = &b.detail["reference_pricing"]["mulmod"];
            assert_eq!(n["pricing"], "montgomery");
            assert!(
                (n["total_gae_reference_unit_code_mulmod"].as_f64().unwrap() - a.total_gae).abs()
                    <= 1e-9 * a.total_gae
            );
            assert_eq!(
                n["total_gae_reference_unit_montgomery_mulmod"].as_f64(),
                Some(b.total_gae)
            );
        }
        let a = solve(&ic_method("ic.pipeline_f2m4_ref", ""), &inst, &facts, &t, 3).unwrap();
        let b = solve(
            &ic_method("ic.pipeline_f2m4_refmm", ""),
            &inst,
            &facts,
            &t,
            3,
        )
        .unwrap();
        assert_eq!(a.recovered, b.recovered);
        let n = &b.detail["reference_pricing"]["mulmod"];
        assert!(
            (n["total_gae_reference_unit_code_mulmod"].as_f64().unwrap() - a.total_gae).abs()
                <= 1e-9 * a.total_gae
        );
        assert_eq!(
            n["total_gae_reference_unit_montgomery_mulmod"].as_f64(),
            Some(b.total_gae)
        );
    }

    /// The `*_ref` methods are the full methods in the reference unit: the
    /// same walk or run (same counts, same answer), the reference divisor
    /// 50 in place of the curve's schoolbook one, and the two totals side
    /// by side.
    #[test]
    fn the_reference_methods_are_the_full_methods_in_the_reference_unit() {
        // The reference divisor is per m: 64 at even m, 50 at odd m; the
        // second model's is 42 at both.
        for m in [6u32, 7] {
            let expect = if m % 2 == 0 { 64.0 } else { 50.0 };
            let (inst, facts, t) = f2m4_case(m, 777);
            let kv = [("walks", "16")];
            for (full_id, ref_id, kv) in [
                (
                    "rho.negation_batched_full",
                    "rho.negation_batched_ref",
                    &kv[..],
                ),
                ("rho.negation_f2m4_full", "rho.negation_f2m4_ref", &[][..]),
            ] {
                let full = solve(&method(full_id, kv), &inst, &facts, &t, 9).unwrap();
                let rf = solve(&method(ref_id, kv), &inst, &facts, &t, 9).unwrap();
                assert_eq!(full.recovered, rf.recovered);
                for k in ["steps", "distinguished_points", "look_ahead_additions"] {
                    assert_eq!(full.counters.get(k), rf.counters.get(k), "{k}");
                }
                check_full_block(&rf);
                let b = &rf.detail["reference_pricing"];
                assert_eq!(b["fq_muls_per_add_reference"].as_f64(), Some(expect));
                assert_eq!(b["reference_model"]["used"], "log_table_tower");
                assert_eq!(b["reference_model"]["divisor"].as_f64(), Some(expect));
                assert_eq!(
                    b["reference_model"]["tower_costs"]["add"].as_f64(),
                    Some(expect)
                );
                assert_eq!(b["poly_free_const"]["fq_muls_per_add"].as_f64(), Some(42.0));
                assert!(b["poly_free_const"]["total_gae"].as_f64().unwrap() > 0.0);
                assert!(b["fq_muls_per_add_current"].as_f64().unwrap() > expect);
                assert!(
                    (b["total_gae_current_pricing"].as_f64().unwrap() - full.total_gae).abs()
                        <= 1e-9 * full.total_gae
                );
                assert_eq!(
                    b["total_gae_reference_pricing"].as_f64(),
                    Some(rf.total_gae)
                );
            }
            let method = ic_method("ic.pipeline_f2m4_ref", "");
            let full = solve(
                &ic_method("ic.pipeline_f2m4_full", ""),
                &inst,
                &facts,
                &t,
                3,
            )
            .unwrap();
            let rf = solve(&method, &inst, &facts, &t, 3).unwrap();
            assert_eq!(full.recovered, rf.recovered);
            let b = &rf.detail["reference_pricing"];
            assert!(
                (b["total_gae_current_pricing"].as_f64().unwrap() - full.total_gae).abs()
                    <= 1e-9 * full.total_gae
            );
            assert_eq!(b["fq_muls_per_add_reference"].as_f64(), Some(expect));
            assert_eq!(b["reference_model"]["used"], "log_table_tower");
            assert_eq!(b["poly_free_const"]["fq_muls_per_add"].as_f64(), Some(42.0));
            assert!(b["poly_free_const"]["total_gae"].as_f64().unwrap() > 0.0);
            let (cs, rs) = (
                b["fq4_share_of_fq_operations_current"].as_f64().unwrap(),
                b["fq4_share_of_fq_operations_reference"].as_f64().unwrap(),
            );
            assert!((0.0..=1.0).contains(&cs) && (0.0..=1.0).contains(&rs));
            assert!(
                b["lift_plus_x_of_t_fq_muls_reference"].as_f64().unwrap()
                    < b["lift_plus_x_of_t_fq_muls_current"].as_f64().unwrap()
            );
            // The Weil-restriction system builds and the precomputation are
            // reference-priced: the counted size is recorded beside it and
            // the reference price is not above the schoolbook count.
            let w = &b["weil_restriction"];
            let wc = w["weil_muls"].as_u64().unwrap();
            let pc = w["precompute_muls"].as_u64().unwrap();
            assert!(wc > 0 && pc > 0);
            assert_eq!(
                rf.detail["decomposition"]["solver"]["extra"]["weil_fq_muls"].as_u64(),
                Some(wc)
            );
            let (wr, pr) = (
                w["weil_ref_muls"].as_u64().unwrap(),
                w["precompute_ref_muls"].as_u64().unwrap(),
            );
            assert!(
                wr > 0 && pr > 0 && wr + pr < wc + pc,
                "{wr}+{pr} vs {wc}+{pc}"
            );
            assert!(w["weil_ref_poly_muls"].as_u64().unwrap() > 0);
            // The substitution moves the total by exactly the difference
            // of the two prices over the divisor (the same run, so the same
            // counts), and not at all in the current unit.
            let moved = ((wc + pc) as f64 - (wr + pr) as f64) / expect;
            assert!(moved > 0.0);
            let nosub = {
                let mut alt = rf.clone();
                alt.total_gae += moved;
                alt.total_gae
            };
            assert!(nosub > rf.total_gae);
            assert!(b["fq4_share_of_fq_operations_reference"].as_f64().unwrap() > 0.0);
            eprintln!(
                "m={m} weil {wc}->{wr} pre {pc}->{pr} (gae {:.1} -> {:.1})",
                (wc + pc) as f64 / expect,
                (wr + pr) as f64 / expect
            );
            eprintln!(
                "m={m} ic ref/current {:.3} share cur {cs:.4} ref {rs:.4}",
                b["ratio_reference_over_current"]
            );
        }
    }

    /// `ic.pipeline_f2m4_full` is the same run as `ic.pipeline` (the same
    /// group operations and counts, one word and `u128`, with and without
    /// large primes) with every native counter priced.
    #[test]
    fn ic_pipeline_f2m4_full_prices_every_native_counter() {
        let d = 12_345u128;
        for extra in ["", ",large_primes=2,small_permille=500"] {
            let (inst, facts, t) = f2m4_case(6, d);
            let Instance::F2m4(b) = &inst else {
                unreachable!()
            };
            let base = solve(&ic_method("ic.pipeline", extra), &inst, &facts, &t, 3).unwrap();
            let method = ic_method("ic.pipeline_f2m4_full", extra);
            let (tx, ty) = (
                parse_hex_wide(&t[0]).unwrap(),
                parse_hex_wide(&t[1]).unwrap(),
            );
            let one = solve(&method, &inst, &facts, &t, 3).unwrap();
            let wide =
                solve_f2m4_ic_full_with(&method, b, &facts, tx, ty, 3, true, false, Mulmod::Code)
                    .unwrap();
            for full in [&one, &wide] {
                assert_eq!(base.recovered, full.recovered, "{extra}");
                assert!(base.recovered.is_some());
                assert_eq!(base.counters, full.counters);
                for (a, f) in base.phases.iter().zip(&full.phases) {
                    assert_eq!((a.adds, a.doubles), (f.adds, f.doubles), "{}", a.name);
                    for (k, v) in &a.native {
                        assert_eq!(f.native.get(k), Some(v), "{} {k}", a.name);
                    }
                }
                let (counted, native, ops) = check_full_block(full);
                assert!(native > 0.0 && ops > 0 && counted > 0.0);
                let rel = full.phases.iter().find(|p| p.name == "relations").unwrap();
                assert_eq!(
                    rel.native["solver_systems_built"],
                    rel.native["solver_calls"]
                );
                assert!(rel.native["solver_table_inversions"] > 0);
                assert!(rel.native["solver_element_scans"] > 0);
                assert!(rel.native["solver_element_fills"] > 0);
                assert!(rel.native["solver_monomial_hash_ops"] > 0);
                assert!(rel.native["witness_lookups"] > 0);
                assert_eq!(
                    extra.is_empty(),
                    !rel.native.contains_key("lp_modular_multiplications")
                );
                let fb = full
                    .phases
                    .iter()
                    .find(|p| p.name == "factor_base")
                    .unwrap();
                assert!(fb.native["factor_base_points"] > 0);
            }
            // The one-word and the u128 pipelines walk the same run.
            assert_eq!(one.counters, wide.counters);
            for (a, b) in one.phases.iter().zip(&wide.phases) {
                assert_eq!(a.native, b.native, "{}", a.name);
            }
            assert_eq!(one.total_gae, wide.total_gae);
        }
    }

    /// The native tallies do not depend on how rayon schedules the solver.
    #[test]
    fn ic_native_tallies_are_deterministic() {
        let (inst, facts, t) = f2m4_case(6, 999);
        let method = ic_method("ic.pipeline_f2m4_full", "");
        let a = solve(&method, &inst, &facts, &t, 4).unwrap();
        let b = solve(&method, &inst, &facts, &t, 4).unwrap();
        assert_eq!(a.total_gae, b.total_gae);
        assert_eq!(
            a.detail["full_pricing"]["by_counter"],
            b.detail["full_pricing"]["by_counter"]
        );
    }

    /// A counter without a rule stays unpriced: the record is a lower bound.
    #[test]
    fn a_counter_without_a_rule_stays_unpriced() {
        let (inst, facts, t) = f2m4_case(6, 55);
        let Instance::F2m4(b) = &inst else {
            unreachable!()
        };
        let mut rep = solve(&method("rho.negation_f2m4_full", &[]), &inst, &facts, &t, 2).unwrap();
        rep.counters.insert("mystery_work".into(), 3);
        crate::cryptanalysis::ecbench::f2m4_price::reprice_rho(
            &mut rep,
            b.curve.f.f.m,
            false,
            fq_muls_per_add(&b.curve) as f64,
            false,
        )
        .unwrap();
        assert_eq!(rep.unpriced, vec!["mystery_work_uncharged".to_string()]);
    }

    /// The full methods exist on curves over F_(2^m)^4 only.
    #[test]
    fn the_full_methods_refuse_other_curves() {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        let spec = CurveSpec::Koblitz { a: 1, n: 17 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        for id in ["rho.negation_f2m4_full", "rho.negation_batched_full"] {
            let r = solve(
                &method(id, &[]),
                &inst,
                &facts,
                &["0x1".into(), "0x1".into()],
                1,
            );
            assert!(r.unwrap_err().contains("F_(2^m)^4 only"));
        }
    }
}
