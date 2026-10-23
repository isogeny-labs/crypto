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
use crate::cryptanalysis::ecbench::claw::{
    pair_claw, pair_claw_v2, pair_claw_wide, pair_claw_wide_v2, ClawOutcome, ClawShape,
};
use crate::cryptanalysis::ecbench::fp4::{self as fp4, Fp4Group, Fp4Instance, K4Base, K4Params};
use crate::cryptanalysis::ecbench::full_price;
use crate::cryptanalysis::ecbench::generic::{
    bsgs_interleaved, bsgs_negation, bsgs_textbook, kangaroo, kangaroo_with, verify_candidate,
    GenericOutcome,
};
use crate::cryptanalysis::ecbench::workload::{CurveFacts, Instance, WideInstance};
use crate::cryptanalysis::ecbench_large_prime::{
    self as large_prime, SolveConfig as LargePrimeConfig,
};
use crate::cryptanalysis::gaudry_quartic::Pt4;
use crate::cryptanalysis::gaudry_quartic_bin::{
    fq_muls_per_add, BinCurve, BinD4Oracle, BinOptions, PtB, TLineBase,
};
use crate::cryptanalysis::gaudry_quartic_d4::{
    fp_muls_per_add, unpack_e4, D4Curve, D4Options, D4Oracle, YLineBase,
};
use crate::cryptanalysis::ic_boundary::{
    price_phase, rho_cap, rho_reference, rho_walk_with, rho_walk_with_policy,
    signed_frobenius_rho_tuned, signed_frobenius_rho_tuned_policy, BinaryGroup, BinaryInstance,
    Calibration, CountedGroup, CubicClasses, FieldOps, GroupOps, NegationClasses, PhaseCost,
    PointClasses, PrimeInstance, PrimePoint, QuarticClasses, RhoResult, RhoWalk, WalkPolicy,
};
use crate::cryptanalysis::ic_framework::cm_orbit::CmOrbitBase;
use crate::cryptanalysis::ic_framework::fiber::{
    FiberDescentOracle, FiberFrobeniusMitmOracle, FiberMitmOracle,
};
use crate::cryptanalysis::ic_framework::plugins::{
    BinarySubspaceBase, CompactOrbitScanBase, CyclotomicBase, DescentAlgebraicOracle,
    FrobeniusMitmOracle, GlvOrbitBase, KoblitzOrbitBase, MitmOracle, OnbWeightBase,
    PrimeAbscissaBase, PrimeBatchMitmOracle, PrimeProjectedAbscissaBase, RootfindSubtractOracle,
    SubtractOracle,
};
use crate::cryptanalysis::ic_framework::prime_tower::{PrimeIsogenyBase, PrimeKummerBase};
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
    StrongRhoExtras, StrongRhoG, StrongRhoParams,
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

const K4_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "base",
        default: None,
        help: "`xline` ({P : x(P) ∈ F_p}) or `yline-d4` ({P : Y(P) ∈ F_p}, columns {±P, ±(P+T)})",
    },
    ParamDecl {
        name: "large_primes",
        default: None,
        help: "0 (full relations) or 2 (double large primes)",
    },
    ParamDecl {
        name: "c4",
        default: None,
        help: "F_p multiplications charged per trial: the algebraic solve's measured cost (§11.17 S₄: 1.213e12; D₄: 2.54e10)",
    },
    ParamDecl {
        name: "lp_exponent",
        default: Some("0.75"),
        help: "|F'| = ⌈|F|^lp_exponent⌉ when large_primes > 0 (Gaudry's |F|^{1−1/k})",
    },
    ParamDecl {
        name: "trial_cap",
        default: Some("2000"),
        help: "trials per column before the run counts as exhausted",
    },
];

const IC_PIPELINE_PARAMS: &[ParamDecl] = &[
        ParamDecl {
            name: "factor_base",
            default: None,
            help: "plug-in spec: prime-abscissa:size=N, glv-orbit:size=N, prime-kummer:t=T[,g=G], prime-isogeny:t=T[,seed=S] (auxiliary-curve search charged to the factor-base phase), binary-subspace:dimension=D, koblitz-orbit:divisor=1;2, compact-orbit-scan:columns=N,raw_x_cap=M, cyclotomic:w=W[,columns=N], koblitz-standard-subspace:dimension=D",
        },
        ParamDecl {
            name: "oracle",
            default: None,
            help: "subtract, mitm, mitm-frobenius, mitm-frobenius-counted or descent-algebraic, with :m=2|3; mitm-prime-batch:m=2 (prime folded table with batched inversions); mitm-prime-blocked:m=2 (the same table inserted in cache-sized pieces); rootfind-subtract:m=2|3 (prime curves: prime_fast's direct-subtraction root finding, one batched inversion a target); the cofactor-fiber-aware mitm-fiber:fiber=closed|lifts|blind, mitm-frobenius-fiber:fiber=closed|lifts|blind and descent-algebraic-fiber:fiber=combined|lifts|blind (ic_framework::fiber); or pdp3-koblitz:m=3,engine=inherited-f4|f6-ic,degree=D,node_budget=N, pdp3-koblitz:m=2|3,engine=rootfind (PairSumTable), pdp3-koblitz:m=3,engine=rootfind-window (cyclic ⌈|F|/3⌉ window), pdp3-koblitz:m=2|3,engine=rootfind-negfold (PairTable negation fold, ~half set-up), and pdp3-koblitz:m=3,engine=rootfind-window-negfold on koblitz-standard-subspace",
        },
        ParamDecl {
            name: "solver",
            default: Some(""),
            help: "descent-algebraic and descent-algebraic-fiber only: f4-f2, buchberger-f2, sat-cdcl, exhaustive (with :k=v options)",
        },
        ParamDecl {
            name: "linalg",
            default: Some("incremental-gauss"),
            help: "incremental-gauss (stop at target pin), incremental-gauss-full-rank (all columns independent), incremental-gauss-full-rank-checked (also point-check each base column), structured-gauss, sparse-cado = filter+wiedemann (CADO-style filter + block Wiedemann, full-rank rule), filter+lanczos, wiedemann or lanczos (no filter, block Wiedemann / block Lanczos, full-rank rule)",
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

const STRONG_PARAMS: &[ParamDecl] = &[
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
];

const RHO_PARAMS: &[ParamDecl] = &[ParamDecl {
    name: "cap_multiple",
    default: Some("64"),
    help: "step budget as a multiple of √r before the run counts as exhausted",
}];

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

const LARGE_PRIME_PARAMS: &[ParamDecl] = &[
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
];

const CLAW_PARAMS: &[ParamDecl] = &[
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
];

/// The `_v2` strong reference scales its walk with `r`: `auto` takes the
/// rule in [`strong_v2_shape`]; an integer pins the value.
const STRONG_V2_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "lanes",
        default: Some("auto"),
        help: "walks advanced in lockstep: auto = the power of two at most (ideal steps)/2^(dp_bits+3), from 1 to 32; or an integer",
    },
    ParamDecl {
        name: "dp_bits",
        default: Some("auto"),
        help: "distinguished-point bits: auto = round(log2(ideal steps)/2), from 1 to 20 (a walk of about √ideal steps); or an integer",
    },
    ParamDecl {
        name: "jumps",
        default: Some("auto"),
        help: "distinct jump scalars, a divisor of 32 repeated to fill the 32-entry table: auto = the largest of 32, 16, 8 whose fixed set-up (jump table, start multiplications, final check) is at most a tenth of the ideal steps (never fewer than 8); or 4, 8, 16, 32",
    },
    ParamDecl {
        name: "step_cap_factor",
        default: Some("2000"),
        help: "give up after this multiple of the ideal step count",
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
            id: "ic.k4_gaudry",
            family: "ic",
            summary: "Gaudry's k = 4 index calculus on E(F_{p⁴}) (fp4_translation curves only): x-line or D₄ Y-line base, optional double large primes (|F'| = ⌈|F|^lp_exponent⌉), filtering + Wiedemann; each trial charged the algebraic solve's measured cost c4 (F_p mults ÷ 97), its relations from an exact enumeration oracle whose own work is counted uncharged, so S is a lower bound; expected S ∝ r^{-1/2}·p·c4 until the linear algebra (∝ p², or p^{3/2} with large primes) dominates",
            entry: "ecbench::fp4::solve_k4",
            applies: Applies::Any,
            params: K4_PARAMS,
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
            params: STRONG_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong_legacy",
            family: "rho",
            summary: "the literal pre-2026-10-14 implementation of the strong single-target reference, retained only as a trajectory-identical within-binary performance control",
            entry: "koblitz_strong_rho::StrongRho::solve_legacy",
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
            id: "ic.cm_folded_native_v1",
            family: "ic",
            summary: "CM scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained",
            entry: "ecbench::cm_folded::solve",
            applies: Applies::Any,
            params: &[
                ParamDecl { name: "backend", default: None, help: "compiled native/folded.cpp executable" },
                ParamDecl { name: "backend_sha256", default: None, help: "required content hash of native backend" },
                ParamDecl { name: "mode", default: Some("scalar"), help: "scalar or velu (crater only)" },
                ParamDecl { name: "size", default: Some("8192"), help: "abscissae, four seed columns" },
                ParamDecl { name: "query_limit", default: Some("1000000000"), help: "injective point query cap below r" },
                ParamDecl { name: "seconds", default: Some("1800"), help: "cold wall watchdog; failures are censored" },
            ],
        },
        MethodDecl {
            id: "ic.cm_folded_recurrence_v1",
            family: "ic",
            summary: "Generic quadratic-recurrence scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained",
            entry: "ecbench::cm_folded::solve",
            applies: Applies::Any,
            params: &[
                ParamDecl { name: "backend", default: None, help: "compiled native/folded.cpp executable" },
                ParamDecl { name: "backend_sha256", default: None, help: "required content hash of native backend" },
                ParamDecl { name: "mode", default: Some("scalar"), help: "scalar or velu (crater only)" },
                ParamDecl { name: "size", default: Some("8192"), help: "abscissae, four seed columns" },
                ParamDecl { name: "query_limit", default: Some("1000000000"), help: "injective point query cap below r" },
                ParamDecl { name: "seconds", default: Some("1800"), help: "cold wall watchdog; failures are censored" },
            ],
        },
        MethodDecl {
            id: "ic.cm_folded_native_d43_v1",
            family: "ic",
            summary: "D=-43 degree-11 CM scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained",
            entry: "ecbench::cm_folded::solve",
            applies: Applies::Any,
            params: &[
                ParamDecl { name: "backend", default: None, help: "compiled native/folded.cpp executable" },
                ParamDecl { name: "backend_sha256", default: None, help: "required content hash of native backend" },
                ParamDecl { name: "mode", default: Some("scalar"), help: "scalar or velu (crater only)" },
                ParamDecl { name: "size", default: Some("8192"), help: "abscissae, four seed columns" },
                ParamDecl { name: "query_limit", default: Some("1000000000"), help: "injective point query cap below r" },
                ParamDecl { name: "seconds", default: Some("1800"), help: "cold wall watchdog; failures are censored" },
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
            id: "ic.pipeline_full",
            family: "ic",
            summary: "ic.pipeline unchanged, then every native counter priced from src/cryptanalysis/ecbench/full_prices.json (field multiplications, squarings, inversions, Artin-Schreier solves, hash probes, canonicalisations, row operations), so the factor base's construction is charged; unknown counters stay unpriced",
            entry: "ic_framework::run_pipeline + ecbench::full_price::reprice",
            applies: Applies::BinaryOnly,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_full",
            family: "rho",
            summary: "rho.signed_frobenius unchanged, with its canonicalisations and distinguished-point probes priced from src/cryptanalysis/ecbench/full_prices.json",
            entry: "ic_boundary::signed_frobenius_rho_tuned + ecbench::full_price::reprice",
            applies: Applies::KoblitzOnly,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong_full",
            family: "rho",
            summary: "rho.signed_frobenius_strong unchanged, with its canonicalisations, partition hashes and table probes priced from src/cryptanalysis/ecbench/full_prices.json",
            entry: "koblitz_strong_rho::StrongRho::solve + ecbench::full_price::reprice",
            applies: Applies::KoblitzOnly,
            params: STRONG_PARAMS,
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
            params: LARGE_PRIME_PARAMS,
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
        // ── `_v2`: the same algorithms under the accounting rules of README
        // §7 "Accounting v2".  The ids above keep the counts they were
        // recorded with; these are the ones to read new work against.
        MethodDecl {
            id: "rho.negation_v2",
            family: "rho",
            summary: "rho.negation with Brent cycle detection beside the 16-point window (long fruitless cycles are escaped, not run to the walk cap), the steps lost in capped, abandoned and identity walks as a charged counter, and the distinguished-point table's queries and inserts counted uncharged",
            entry: "ic_boundary::rho_walk_with_policy(NegationClasses, RhoWalk::negation(), WalkPolicy::v2())",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.cubic_v2",
            family: "rho",
            summary: "rho.cubic with the set-up [κ]G verification ledger of CubicClasses::new inside total_gae (phase automorphism_setup), plus the walk of rho.negation_v2; refuses any curve without the j = 0 structure",
            entry: "ic_boundary::CubicClasses with rho_walk_with_policy(WalkPolicy::v2())",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.quartic_v2",
            family: "rho",
            summary: "rho.quartic with the set-up [κ]G verification ledger of QuarticClasses::new inside total_gae (phase automorphism_setup), plus the walk of rho.negation_v2; refuses any curve without the j = 1728 structure",
            entry: "ic_boundary::QuarticClasses with rho_walk_with_policy(WalkPolicy::v2())",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_v2",
            family: "rho",
            summary: "rho.signed_frobenius with the walk of rho.negation_v2 (Brent cycle detection, lost-walk and table counters), Koblitz curves only",
            entry: "ic_boundary::signed_frobenius_rho_tuned_policy(WalkPolicy::v2())",
            applies: Applies::KoblitzOnly,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong_v2",
            family: "rho",
            summary: "the strong single-target reference with lanes, distinguished-point bits and jump table scaled to r so its fixed set-up is a small share of S where r allows (fixed-base jump table, counted exactly), the start multiples charged, the work spent by an exhausted solve recorded, and the candidate checks in a verification phase; A = 2n, Koblitz curves only",
            entry: "koblitz_strong_rho::StrongRho::solve_accounted (jumps_fixed_base)",
            applies: Applies::KoblitzOnly,
            params: STRONG_V2_PARAMS,
        },
        MethodDecl {
            id: "bsgs.textbook_v2",
            family: "bsgs",
            summary: "bsgs.textbook plus one charged final verification scalar multiplication [d]G = Q (phase verification)",
            entry: "ecbench::generic::bsgs_textbook + verify_candidate",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "bsgs.interleaved_v2",
            family: "bsgs",
            summary: "bsgs.interleaved plus one charged final verification scalar multiplication [d]G = Q (phase verification)",
            entry: "ecbench::generic::bsgs_interleaved + verify_candidate",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "bsgs.negation_v2",
            family: "bsgs",
            summary: "bsgs.negation plus one charged final verification scalar multiplication [d]G = Q (phase verification)",
            entry: "ecbench::generic::bsgs_negation + verify_candidate",
            applies: Applies::Any,
            params: &[],
        },
        MethodDecl {
            id: "kangaroo.vow_v2",
            family: "kangaroo",
            summary: "kangaroo.vow with restart scalar multiplications in their own phase (restarts) instead of setup, plus one charged final verification scalar multiplication",
            entry: "ecbench::generic::kangaroo_with(restarts_apart) + verify_candidate",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "claw.pair_table_v2",
            family: "claw",
            summary: "claw.pair_table with the recover phase inside the charged-addition budget and one charged final verification scalar multiplication (phase verification)",
            entry: "ecbench::claw::pair_claw_with(v2)",
            applies: Applies::KoblitzOnly,
            params: CLAW_PARAMS,
        },
        MethodDecl {
            id: "ic.large_prime_v2",
            family: "ic",
            summary: "ic.large_prime with the [r]Q subgroup-membership check of the target and the final [d]G = Q verification counted and charged to the verification phase",
            entry: "ecbench_large_prime::solve_candidate_v2",
            applies: Applies::BinaryOnly,
            params: LARGE_PRIME_PARAMS,
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
        "rho.negation" | "rho.negation_v2" => Some((PI / 4.0).sqrt()),
        "rho.cubic" | "rho.cubic_v2" => Some((PI / 12.0).sqrt()),
        "rho.quartic" | "rho.quartic_v2" => Some((PI / 8.0).sqrt()),
        "rho.signed_frobenius"
        | "rho.signed_frobenius_v2"
        | "rho.signed_frobenius_full"
        | "rho.signed_frobenius_strong"
        | "rho.signed_frobenius_strong_v2"
        | "rho.signed_frobenius_strong_full"
        | "rho.signed_frobenius_strong_escape" => {
            Some((PI / (2.0 * a_available.max(1) as f64)).sqrt())
        }
        "bsgs.textbook" | "bsgs.textbook_v2" => Some(1.5),
        "bsgs.interleaved" | "bsgs.interleaved_v2" => Some(4.0 / 3.0),
        "bsgs.negation" | "bsgs.negation_v2" => Some(1.0),
        "kangaroo.vow" | "kangaroo.vow_v2" => Some(2.0),
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

/// Required parameters that hold an integer (a parameter with a default is
/// integer-valued when its default is a canonical integer).  `summands`
/// also takes `n-1`, which the lexeme rule below leaves alone.
const INTEGER_PARAMS_WITHOUT_DEFAULT: &[&str] = &[
    "large_primes",
    "max_trials",
    "target_max_attempts",
    "small_dimension",
    "envelope_dimension",
    "summands",
    "rank_seed",
    "rank_max_trials",
    "max_states",
];

/// Whether `s` is a canonical decimal integer: `0`, or an optional minus
/// and digits with no leading zero.
fn is_canonical_integer(s: &str) -> bool {
    let digits = s.strip_prefix('-').unwrap_or(s);
    !digits.is_empty()
        && digits.bytes().all(|b| b.is_ascii_digit())
        && (digits == "0" || (!digits.starts_with('0') && s != "-0"))
}

/// The canonical text of an integer-valued parameter.  `64`, `064` and
/// `+64` are one method and so one id; a value that is not an integer
/// lexeme (`n-1`, `1e3`, `6.0`) is left as written for the method to read
/// or refuse.  Done on the text, so no width limit applies.
fn normalise_param(name: &str, default: Option<&str>, value: &str) -> String {
    let integer_valued = default.map_or_else(
        || INTEGER_PARAMS_WITHOUT_DEFAULT.contains(&name),
        is_canonical_integer,
    );
    if !integer_valued {
        return value.to_string();
    }
    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    if unsigned.is_empty() || !unsigned.bytes().all(|b| b.is_ascii_digit()) {
        return value.to_string();
    }
    let trimmed = unsigned.trim_start_matches('0');
    match (trimmed.is_empty(), negative) {
        (true, _) => "0".to_string(),
        (false, true) => format!("-{trimmed}"),
        (false, false) => trimmed.to_string(),
    }
}

/// Fill defaults, normalise integer-valued parameters to canonical
/// decimal, refuse unknown and missing parameters, hash.
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
            (Some(v), def) => {
                params.insert(p.name.to_string(), normalise_param(p.name, def, v));
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

/// The share of `total` that is fixed set-up, for the methods that have one:
/// the phases a rho or kangaroo pays before its first step whatever `r` is
/// (`setup`, `automorphism_setup`).  `None` for a method whose set-up scales
/// with `√r` (BSGS's table, the claw's base and table) or that has none.
/// [`row_flags`] flags a share above [`SETUP_SHARE_FLAG`]; `ecbench table`
/// does not print that flag yet (README §12).
pub fn setup_share(family: &str, phases: &[PhaseRecord], total: f64) -> Option<f64> {
    if !matches!(family, "rho" | "kangaroo")
        || total.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
    {
        return None;
    }
    let fixed: f64 = phases
        .iter()
        .filter(|p| matches!(p.name.as_str(), "setup" | "automorphism_setup"))
        .map(|p| p.gae)
        .sum();
    Some(fixed / total)
}

/// A row is flagged when fixed set-up is more than this share of `S`.
pub const SETUP_SHARE_FLAG: f64 = 0.10;

/// `log2 r` below which the calibration
/// (`research/ecbench_calibration_20261002`) does not reach: `S` has not
/// settled to its constant there, so a row below it is flagged (prime
/// `r < 2^25`, Koblitz `r < 2^39`).
pub fn calibrated_log2_r(family: &str) -> f64 {
    if family == "koblitz" {
        39.0
    } else {
        25.0
    }
}

/// Why `ecbench table` should flag a row, in words (empty: unflagged):
/// fixed set-up above a tenth of `S`, or `r` below the calibrated range.
/// `family` is the curve family (`prime`, `koblitz`, …) and `method_family`
/// the method's.
pub fn row_flags(
    curve_family: &str,
    log2_r: f64,
    method_family: &str,
    phases: &[PhaseRecord],
    total: f64,
) -> Vec<String> {
    let mut flags = Vec::new();
    if let Some(share) = setup_share(method_family, phases, total) {
        if share > SETUP_SHARE_FLAG {
            flags.push(format!(
                "fixed set-up is {:.0}% of S (over {:.0}%)",
                100.0 * share,
                100.0 * SETUP_SHARE_FLAG
            ));
        }
    }
    let floor = calibrated_log2_r(curve_family);
    if log2_r < floor {
        flags.push(format!(
            "log2 r = {log2_r:.1} is below the calibrated range (2^{floor:.0}); S has not reached its constant"
        ));
    }
    flags
}

/// A tuned rho run reported under the `_v2` accounting: the same phases as
/// [`rho_report`] plus `automorphism_setup`, the `[κ]G` ledger of
/// `CubicClasses::new` / `QuarticClasses::new` that the first rule merges
/// into `group_ops` after `gae` was read, so that it sat in a phase and not
/// in the total.  Here the total is the sum of the phases.
fn rho_report_v2(
    res: RhoResult,
    wall: u64,
    field_ops: Option<FieldOps>,
    class_setup: GroupOps,
) -> SolveReport {
    let c = &res.counters;
    let get = |k: &str| c.get(k).copied().unwrap_or(0);
    let setup = get("setup_additions") + get("setup_doublings");
    let verify = get("verification_operations");
    let walk_total = res.group_ops.adds + res.group_ops.doubles;
    let search = walk_total.saturating_sub(setup + verify);
    let mk = |name: &str, gae: u64| PhaseRecord {
        name: name.into(),
        gae: gae as f64,
        ..Default::default()
    };
    let mut s = mk("setup", setup);
    s.adds = get("setup_additions");
    s.doubles = get("setup_doublings");
    let mut a = mk("automorphism_setup", class_setup.adds + class_setup.doubles);
    a.adds = class_setup.adds;
    a.doubles = class_setup.doubles;
    a.scalar_mults = class_setup.scalar_mults;
    let mut w = mk("search", search);
    w.scalar_mults = res.group_ops.scalar_mults;
    let phases = vec![s, a, w, mk("internal_verification", verify)];
    let total_gae = phases.iter().map(|p| p.gae).sum::<f64>();
    let mut counters = res.counters.clone();
    counters.insert("steps".into(), res.steps);
    counters.insert("walks".into(), res.walks);
    counters.insert("distinguished_points".into(), res.distinguished_points);
    counters.insert("automorphism_setup_additions".into(), class_setup.adds);
    counters.insert("automorphism_setup_doublings".into(), class_setup.doubles);
    let share = setup_share("rho", &phases, total_gae);
    SolveReport {
        recovered: res.recovered.map(u128::from),
        exhausted: res.recovered.is_none(),
        total_gae,
        automorphisms_used: res.automorphisms,
        unpriced: unpriced_of(&counters),
        counters,
        phases,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: json!({
            "method": res.method,
            "expected_steps": res.expected_steps,
            "accounting": "v2",
            "setup_share": share,
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops,
    }
}

/// A BSGS or kangaroo run reported under the `_v2` accounting: the first
/// rule's phases, a `restarts` phase for a kangaroo that restarted, and the
/// `verification` phase of [`generic::verify_candidate`].  The total is the
/// sum of the phases.
fn generic_report_v2(
    out: GenericOutcome,
    wall: u64,
    a: u32,
    field_ops: Option<FieldOps>,
    family: &str,
) -> SolveReport {
    let mut phases = vec![
        ops_phase("setup", out.setup),
        ops_phase("search", out.search),
    ];
    if family == "kangaroo" {
        phases.push(ops_phase("restarts", out.restarts));
    }
    phases.push(ops_phase("verification", out.verification));
    let total_gae = phases.iter().map(|p| p.gae).sum::<f64>();
    let share = setup_share(family, &phases, total_gae);
    SolveReport {
        recovered: out.recovered.map(u128::from),
        exhausted: out.exhausted,
        phases,
        total_gae,
        automorphisms_used: a,
        unpriced: unpriced_of(&out.counters),
        counters: out.counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: json!({"accounting": "v2", "setup_share": share}),
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
        // The `_v2` ids: the same searches, then the one charged check.
        "rho.negation_v2" => {
            let res = rho_walk_with_policy(
                g,
                &NegationClasses { r },
                gen,
                target,
                r,
                seed,
                cap()?,
                RhoWalk::negation(),
                WalkPolicy::v2(),
            );
            let wall = t.elapsed().as_nanos() as u64;
            let f = field_ops_of(g, &[res.group_ops]);
            rho_report_v2(res, wall, f, GroupOps::default())
        }
        id @ ("bsgs.textbook_v2"
        | "bsgs.interleaved_v2"
        | "bsgs.negation_v2"
        | "kangaroo.vow_v2") => {
            let (mut o, a, family) = match id {
                "bsgs.textbook_v2" => (bsgs_textbook(g, gen, target, r), 1, "bsgs"),
                "bsgs.interleaved_v2" => (bsgs_interleaved(g, gen, target, r), 1, "bsgs"),
                "bsgs.negation_v2" => (bsgs_negation(g, gen, target, r), 2, "bsgs"),
                _ => (
                    kangaroo_with(g, gen, target, r, seed, cap()?, true),
                    1,
                    "kangaroo",
                ),
            };
            verify_candidate(g, gen, target, &mut o);
            let wall = t.elapsed().as_nanos() as u64;
            let f = field_ops_of(g, &[o.setup, o.search, o.restarts, o.verification]);
            generic_report_v2(o, wall, a, f, family)
        }
        other => return Err(format!("`{other}` is not a generic method")),
    };
    measurement::end_online();
    Ok(rep)
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
        "rho.signed_frobenius_strong"
        | "rho.signed_frobenius_strong_legacy"
        | "rho.signed_frobenius_strong_escape" => {
            solve_strong_wide(m, wi, tx, ty, seed)
        }
        "rho.signed_frobenius_strong_v2" => solve_strong_wide(m, wi, tx, ty, seed),
        "claw.pair_table" | "claw.pair_table_v2" => solve_claw_wide(m, wi, tx, ty, seed),
        other => Err(format!(
            "`{other}` has no wide-field implementation; on a curve past one word (n = {}) only the strong rho (with or without the escape, or its _v2) and claw.pair_table (or its _v2) run",
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
    let mut rep = solve_measured(m, inst, curve, target, seed)?;
    complete(m, inst, curve, target, &mut rep)?;
    Ok(rep)
}

/// [`solve`] without the factor-base inventory of an index-calculus run
/// ([`complete`] adds it).  What a measured child times, counts and profiles.
pub fn solve_measured(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    target: &[String; 2],
    seed: u64,
) -> Result<SolveReport, String> {
    let d = decl(&m.id).ok_or("unregistered method")?;
    if m.id.starts_with("ic.cm_folded_") && !matches!(inst, Instance::Prime(_)) {
        return Err("CM folded controls require a prime-field instance".into());
    }
    // A `*_full` method runs its base unchanged and is repriced after.
    let full = full_price::base_of(&m.id);
    let base_m;
    let (m, run_d) = match full {
        Some(base) => {
            base_m = ResolvedMethod {
                id: base.to_string(),
                ..m.clone()
            };
            (&base_m, decl(base).ok_or("unregistered base method")?)
        }
        None => (m, d),
    };
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
    let mut rep = if let Instance::Wide(wi) = inst {
        let (tx, ty) = (parse_hex_wide(&target[0])?, parse_hex_wide(&target[1])?);
        solve_wide(m, wi, tx, ty, seed)?
    } else {
        let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
        solve_inner(m, inst, curve, run_d, tx, ty, seed)?
    };
    if full.is_some() {
        full_price::reprice(&mut rep, d.family, &curve.slug)?;
    }
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
    if let Instance::Fp4(i) = inst {
        return solve_fp4(m, i, tx, ty, seed);
    }
    if m.id == "ic.k4_gaudry" {
        return Err(format!(
            "`ic.k4_gaudry` runs on fp4_translation curves only; {} is {}",
            curve.slug, curve.family
        ));
    }
    match (inst, d.family) {
        (Instance::Binary(i), _) if m.id == "ic.large_prime" || m.id == "ic.large_prime_v2" => {
            solve_large_prime(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _) if m.id == "ic.shared_rank" => {
            solve_shared_rank(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), _)
            if m.id == "ic.cm_folded_native_v1"
                || m.id == "ic.cm_folded_native_d43_v1"
                || m.id == "ic.cm_folded_recurrence_v1" =>
        {
            super::cm_folded::solve(m, i, PrimePoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), "ic") => solve_ic_prime(m, i, curve, PrimePoint::affine(tx, ty), seed),
        (Instance::Fp4Torsion(i), "ic") if m.id == "ic.pipeline" => {
            let p = i.p();
            let target = Pt4 {
                x: unpack_e4(p, tx),
                y: unpack_e4(p, ty),
                inf: false,
            };
            solve_ic_fp4(m, i, curve, target, seed)
        }
        (Instance::Fp4Torsion(_), "ic") => Err(format!(
            "`{}` does not run on a curve over F_(p^4); ic.pipeline does",
            m.id
        )),
        (Instance::F2m4(i), "ic") if m.id == "ic.pipeline" => {
            let f = &i.curve.f;
            let target = PtB {
                x: f.unpack(tx),
                y: f.unpack(ty),
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
                || m.id == "rho.signed_frobenius_strong_legacy"
                || m.id == "rho.signed_frobenius_strong_escape"
                || m.id == "rho.signed_frobenius_strong_v2" =>
        {
            solve_strong(m, i, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), "claw") => solve_claw(m, i, FastPoint::affine(tx, ty), seed),
        (Instance::Binary(i), _) if m.id == "rho.signed_frobenius_v2" => {
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let res = signed_frobenius_rho_tuned_policy(
                i,
                FastPoint::affine(tx, ty),
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
                WalkPolicy::v2(),
            )
            .ok_or("not a Koblitz instance")?;
            measurement::end_online();
            Ok(rho_report_v2(
                res,
                t.elapsed().as_nanos() as u64,
                None,
                GroupOps::default(),
            ))
        }
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
        (Instance::Prime(i), _) if m.id == "rho.quartic" || m.id == "rho.quartic_v2" => {
            let v2 = m.id == "rho.quartic_v2";
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
            let mut res = rho_walk_with_policy(
                &i.curve,
                &classes,
                i.generator_point(),
                PrimePoint::affine(tx, ty),
                i.r,
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
                RhoWalk::negation(),
                if v2 {
                    WalkPolicy::v2()
                } else {
                    WalkPolicy::legacy()
                },
            );
            if v2 {
                // The [κ]G check of the class set-up is part of the cost.
                let walk_ops = res.group_ops;
                res.group_ops.merge(setup);
                measurement::end_online();
                let f = FieldOps::total(&[res.group_ops]);
                return Ok(rho_report_v2(
                    RhoResult {
                        group_ops: walk_ops,
                        ..res
                    },
                    t.elapsed().as_nanos() as u64,
                    Some(f),
                    setup,
                ));
            }
            res.group_ops.merge(setup);
            measurement::end_online();
            let f = FieldOps::total(&[res.group_ops]);
            Ok(rho_report(res, t.elapsed().as_nanos() as u64, Some(f)))
        }
        (Instance::Prime(i), _) if m.id == "rho.cubic" || m.id == "rho.cubic_v2" => {
            let v2 = m.id == "rho.cubic_v2";
            let t = Instant::now();
            measurement::begin_online(Phase::RhoSolve);
            let mut setup = GroupOps::default();
            let classes = CubicClasses::new(&i.curve, i.generator_point(), i.r, &mut setup).ok_or(
                format!(
                    "`rho.cubic` needs a j = 0 curve over p ≡ 1 (mod 3) with 3 | r − 1 and a nontrivial ζ₃ on the subgroup; {} lacks the structure",
                    curve.slug
                ),
            )?;
            let mut res = rho_walk_with_policy(
                &i.curve,
                &classes,
                i.generator_point(),
                PrimePoint::affine(tx, ty),
                i.r,
                seed,
                rho_cap(i.r, param_u64(m, "cap_multiple")? as f64),
                RhoWalk::negation(),
                if v2 {
                    WalkPolicy::v2()
                } else {
                    WalkPolicy::legacy()
                },
            );
            if v2 {
                let walk_ops = res.group_ops;
                res.group_ops.merge(setup);
                measurement::end_online();
                let f = FieldOps::total(&[res.group_ops]);
                return Ok(rho_report_v2(
                    RhoResult {
                        group_ops: walk_ops,
                        ..res
                    },
                    t.elapsed().as_nanos() as u64,
                    Some(f),
                    setup,
                ));
            }
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
        (Instance::Fp4Torsion(i), _) => {
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
                x: f.unpack(tx),
                y: f.unpack(ty),
                inf: false,
            };
            solve_generic(m, &i.curve, i.generator, target, i.r, seed)
        }
        (Instance::Wide(_), _) | (Instance::Fp4(_), _) => unreachable!("returned above"),
    }
}

/// `E(F_{p⁴})`: `ic.k4_gaudry`, or a generic method on [`Fp4Group`].
fn solve_fp4(
    m: &ResolvedMethod,
    i: &Fp4Instance,
    tx: u64,
    ty: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    let target = i.decode(tx, ty)?;
    if m.id != "ic.k4_gaudry" {
        if !(m.id.starts_with("rho.plain")
            || m.id == "rho.negation"
            || m.id == "rho.negation_v2"
            || m.id.starts_with("bsgs.")
            || m.id == "kangaroo.vow"
            || m.id == "kangaroo.vow_v2")
        {
            return Err(format!("`{}` has no E(F_p⁴) implementation", m.id));
        }
        let g = Fp4Group(i.curve());
        return solve_generic(m, &g, i.g, target, i.r, seed);
    }
    let base = match m.params["base"].as_str() {
        "xline" => K4Base::XLine,
        "yline-d4" => K4Base::YLineD4,
        // Item ic_improve_20261014/01: the D₄ Y-line stacked with double
        // large primes (|F'| = ⌈columns^lp_exponent⌉ of the folded columns).
        "yline-d4-lp" => K4Base::YLineD4,
        // Item ic_improve_20261015/04: the D₄ Y-line with the eager
        // first-dependency stop (filter after every relation).
        "yline-d4-ed" => K4Base::YLineD4,
        other => {
            return Err(format!(
                "base `{other}`: xline, yline-d4, yline-d4-lp or yline-d4-ed"
            ))
        }
    };
    let large_primes = param_u64(m, "large_primes")? as usize;
    if large_primes != 0 && large_primes != 2 {
        return Err("large_primes is 0 or 2".into());
    }
    if (m.params["base"] == "yline-d4" || m.params["base"] == "yline-d4-ed") && large_primes != 0 {
        return Err("yline-d4 and yline-d4-ed run without large primes".into());
    }
    let prm = K4Params {
        base,
        large_primes,
        lp_exponent: param_f64(m, "lp_exponent")?,
        c4: param_f64(m, "c4")?,
        trial_cap_per_column: param_u64(m, "trial_cap")?,
        eager_la: m.params["base"] == "yline-d4-ed",
    };
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let out = fp4::solve_k4(i, target, seed, &prm);
    measurement::end_online();
    let phases: Vec<PhaseRecord> = out
        .phases
        .iter()
        .map(|&(name, adds, gae)| PhaseRecord {
            name: name.into(),
            adds,
            gae,
            ..Default::default()
        })
        .collect();
    let total = phases.iter().map(|p| p.gae).sum();
    Ok(SolveReport {
        recovered: out.recovered.map(u128::from),
        exhausted: out.exhausted,
        phases,
        total_gae: total,
        automorphisms_used: if base == K4Base::YLineD4 { 4 } else { 2 },
        unpriced: unpriced_of(&out.counters),
        counters: out.counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: t.elapsed().as_nanos() as u64,
        factor_base: None,
        detail: out.detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
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
    // `ic.large_prime_v2` also charges the target's `[r]Q` membership check
    // and the final `[d]G = Q` to the verification phase.
    let v2 = m.id == "ic.large_prime_v2";
    let report = if v2 {
        large_prime::solve_candidate_v2(inst, target, config)?
    } else {
        large_prime::solve_candidate(inst, target, config)?
    };
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
    if v2 {
        for (name, value) in [
            ("membership_checks", report.counters.membership_checks),
            ("final_checks", report.counters.final_checks),
            ("final_checks_failed", report.counters.final_checks_failed),
        ] {
            counters.insert(name.into(), value);
        }
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
        let strong = id == "rho.signed_frobenius_strong"
            || id == "rho.signed_frobenius_strong_escape"
            || id == "rho.signed_frobenius_strong_v2";
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
    let v2 = m.id == "claw.pair_table_v2";
    let t = Instant::now();
    let o = if v2 {
        pair_claw_v2(inst, target, seed, shape)
    } else {
        pair_claw(inst, target, seed, shape)
    }
    .ok_or("not a Koblitz instance")?;
    let wall = t.elapsed().as_nanos() as u64;
    Ok(claw_report(o, shape, u128::from(inst.r), inst.n, wall, v2))
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
    let v2 = m.id == "claw.pair_table_v2";
    let t = Instant::now();
    let target = RawPointG::Affine { x: tx, y: ty };
    let o = if v2 {
        pair_claw_wide_v2(wi, target, seed, shape)
    } else {
        pair_claw_wide(wi, target, seed, shape)
    };
    let wall = t.elapsed().as_nanos() as u64;
    Ok(claw_report(o, shape, wi.kc.r, wi.kc.n, wall, v2))
}

fn claw_report(
    o: ClawOutcome,
    shape: ClawShape,
    r: u128,
    n: u32,
    wall: u64,
    v2: bool,
) -> SolveReport {
    let mut phases = vec![
        ops_phase("base", o.base),
        ops_phase("table", o.table),
        ops_phase("search", o.search),
        ops_phase("recover", o.recover),
    ];
    if v2 {
        phases.push(ops_phase("verification", o.verification));
    }
    let total = phases.iter().map(|p| p.gae).sum::<f64>();
    let mut detail = json!({
        "source": "aburan28/cryptanalysis#175, experiments/koblitz-pair-claw-20260929 (known-log orbit base, quotient pair table, unique query schedule)",
        "seed_orbits": shape.seeds(r, n),
        "table_classes_target": shape.table(r, n),
        "expected_s": (shape.table_scale + 1.0 / shape.table_scale) / (n as f64).sqrt(),
        "expected_s_law": "(c + 1/c)/√n with c = table_scale, from a hit probability of 2nM/r per query at two additions each; finite base support and duplicate classes not modelled",
    });
    if v2 {
        detail["accounting"] = json!("v2");
    }
    SolveReport {
        recovered: o.recovered,
        exhausted: o.exhausted,
        phases,
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

/// How the `_v2` strong reference sizes its walk to the subgroup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrongV2Shape {
    pub lanes: usize,
    pub dp_bits: u32,
    /// Distinct jump scalars (a divisor of 32; the table repeats them).
    pub distinct_jumps: usize,
}

/// Operations of the fixed set-up of a `_v2` strong solve with `jumps`
/// distinct jumps on a `bits`-bit subgroup order: the `[2^i]G` table
/// (`bits − 1` doublings), about half a bit of additions a jump, the two
/// start scalar multiplications and the final check at `1.5·bits` each.
pub fn strong_v2_fixed_setup(bits: f64, jumps: usize) -> f64 {
    (bits - 1.0) + jumps as f64 * bits / 2.0 + 3.0 * bits + 1.5 * bits
}

/// The `auto` rule of `rho.signed_frobenius_strong_v2`.  With `ideal` the
/// expected steps `√(πr/2A)` and `bits` the bit length of `r`:
///
/// - `dp_bits = round(log₂(ideal)/2)`, from 1 to 20: a walk of about
///   `√ideal` steps, so the distinguished-point overshoot is about
///   `1/√ideal` of the solve and the table is queried once a `√ideal` steps;
/// - `lanes` = the power of two at most `ideal / 2^(dp_bits+3)`, from 1 to
///   32: at least eight walks a lane, so a solve does not end with most of
///   its lanes just started;
/// - `distinct_jumps` = the largest of 32, 16 and 8 whose fixed set-up
///   ([`strong_v2_fixed_setup`]) is at most a tenth of `ideal`, and 8 when
///   none is: where the subgroup is too small for that, the share is
///   reported (and flagged by `ecbench table`), not hidden by a worse walk.
pub fn strong_v2_shape(bits: f64, ideal: f64) -> StrongV2Shape {
    let dp_bits = ((ideal.max(2.0).log2() / 2.0).round() as u32).clamp(1, 20);
    let want = ideal / (1u64 << (dp_bits + 3)) as f64;
    let lanes = if want < 2.0 {
        1
    } else {
        (1usize << (want.log2().floor() as u32)).min(32)
    };
    let distinct_jumps = [32usize, 16, 8]
        .into_iter()
        .find(|&j| strong_v2_fixed_setup(bits, j) <= 0.1 * ideal)
        .unwrap_or(8);
    StrongV2Shape {
        lanes,
        dp_bits,
        distinct_jumps,
    }
}

/// `rho.signed_frobenius_strong_v2`: [`StrongRhoG::solve_accounted`] on a
/// walk sized by [`strong_v2_shape`] (or the parameters), with a fixed-base
/// jump table counted operation by operation, the start multiples charged,
/// the candidate checks in their own phase and, when the solve gives up, the
/// work it spent.  Seeding is as the reference's.
fn strong_run_v2<F: RhoField, S: RhoScalar>(
    m: &ResolvedMethod,
    rho: &StrongRhoG<F, S>,
    q: RawPointG<F::E>,
    r: f64,
    seed: u64,
    t: Instant,
) -> Result<SolveReport, String> {
    let profile_online = std::env::var("ECBENCH_CALLGRIND_TARGET").as_deref() == Ok("1");
    let ideal = rho.ideal_steps();
    let bits = f64::from(rho.modulus().bit_len());
    let auto = strong_v2_shape(bits, ideal);
    let pick = |name: &str, auto: u64| -> Result<u64, String> {
        match m.params[name].as_str() {
            "auto" => Ok(auto),
            v => v
                .parse()
                .map_err(|_| format!("parameter `{name}` is `auto` or an integer, not `{v}`")),
        }
    };
    let lanes = pick("lanes", auto.lanes as u64)?.max(1) as usize;
    let dp_bits = pick("dp_bits", u64::from(auto.dp_bits))? as u32;
    let distinct = pick("jumps", auto.distinct_jumps as u64)? as usize;
    if dp_bits >= 32 {
        return Err("dp_bits must be below 32".into());
    }
    if !matches!(distinct, 1 | 2 | 4 | 8 | 16 | 32) {
        return Err(format!("jumps must divide 32, not {distinct}"));
    }
    let params = StrongRhoParams {
        lanes,
        dp_bits,
        step_cap_factor: param_u64(m, "step_cap_factor")?,
        escape_fruitless: false,
    };
    let mul_gae = 1.5 * r.log2();
    let mut charges = StrongRhoCharges::default();
    let jump_seed = derive_u64("ecbench.strong_rho.jumps", &[seed]);
    let jumps = rho.jumps_fixed_base(jump_seed, distinct, &mut charges);
    let setup_adds = charges.group_additions;
    let mut start_rng = StdRng::seed_from_u64(seed);
    if profile_online {
        measurement::callgrind_dump(b"ecbench_before_online\0");
    }
    measurement::begin_online(Phase::RhoSolve);
    let outcome = rho.solve_accounted(
        q,
        &jumps,
        &mut start_rng,
        &params,
        charges,
        StrongRhoExtras {
            charge_start_multiples: true,
        },
    );
    measurement::end_online();
    if profile_online {
        measurement::callgrind_dump(b"ecbench_online\0");
    }
    let wall = t.elapsed().as_nanos() as u64;
    // What a success and an exhausted solve both report.
    let (recovered, c, walk_steps, walks, fruitless, capped, wasted, entries, automorphisms, lost) =
        match &outcome {
            Ok(o) => (
                Some(o.scalar.to_u128()),
                o.charges,
                o.walk_steps,
                o.walks,
                o.fruitless,
                o.capped,
                o.wasted_merges,
                o.table_entries,
                o.automorphisms,
                o.abandoned_trail_lengths
                    .iter()
                    .map(|&l| u64::from(l))
                    .sum::<u64>(),
            ),
            Err(sp) => (
                None,
                sp.charges,
                sp.walk_steps,
                sp.walks,
                sp.fruitless,
                sp.capped,
                sp.wasted_merges,
                sp.table_entries,
                sp.automorphisms,
                sp.abandoned_trail_lengths
                    .iter()
                    .map(|&l| u64::from(l))
                    .sum::<u64>(),
            ),
        };
    // Two scalar multiplications start the solve (stride, first cursor);
    // every other is a candidate check.
    let start_mults = 2u64.min(c.scalar_multiplications);
    let checks = c.scalar_multiplications - start_mults;
    let search_adds = c.group_additions - setup_adds;
    let setup = PhaseRecord {
        name: "setup".into(),
        adds: setup_adds,
        gae: setup_adds as f64,
        ..Default::default()
    };
    let search = PhaseRecord {
        name: "search".into(),
        adds: search_adds,
        scalar_mults: start_mults,
        gae: search_adds as f64 + start_mults as f64 * mul_gae,
        ..Default::default()
    };
    let verification = PhaseRecord {
        name: "verification".into(),
        scalar_mults: checks,
        gae: checks as f64 * mul_gae,
        ..Default::default()
    };
    let phases = vec![setup, search, verification];
    let total_gae = phases.iter().map(|p| p.gae).sum::<f64>();
    let mut counters = BTreeMap::new();
    for (k, v) in [
        ("walk_steps", walk_steps),
        ("walks", walks),
        ("fruitless_walks", fruitless),
        ("capped_walks", capped),
        ("steps_in_abandoned_walks", lost),
        ("wasted_merges", wasted),
        ("table_entries", entries as u64),
        ("failed_collisions", c.failed_collisions),
        ("scalar_multiplications", c.scalar_multiplications),
        ("lanes", lanes as u64),
        ("dp_bits", u64::from(dp_bits)),
        ("distinct_jumps", distinct as u64),
        ("canonicalisations_uncharged", c.canonicalizations),
        ("partition_hashes_uncharged", c.partition_hashes),
        ("table_queries_uncharged", c.table_queries),
        ("table_inserts_uncharged", c.table_inserts),
    ] {
        counters.insert(k.to_string(), v);
    }
    let share = setup_share("rho", &phases, total_gae);
    let detail = json!({
        "reference": "koblitz_strong_rho::StrongRho::solve_accounted, the strong reference sized to r (accounting v2)",
        "accounting": "v2",
        "lanes": lanes,
        "dp_bits": dp_bits,
        "distinct_jumps": distinct,
        "auto_shape": {"lanes": auto.lanes, "dp_bits": auto.dp_bits, "distinct_jumps": auto.distinct_jumps},
        "jump_seed": jump_seed.to_string(),
        "ideal_steps": ideal,
        "log2_r": r.log2(),
        "setup_share": share,
        "jump_table": "distinct jumps [s]G from one table of [2^i]G, every doubling and addition counted",
        "scalar_multiplication_charge": "1.5·log2(r) additions each for the two start multiplications and every candidate check (ic_boundary::signed_frobenius_rho's convention); the start multiples j·stride are charged as additions",
    });
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        total_gae,
        automorphisms_used: automorphisms as u32,
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
    if m.id == "rho.signed_frobenius_strong_v2" {
        return strong_run_v2(m, rho, q, r, seed, t);
    }
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
    let outcome = if m.id == "rho.signed_frobenius_strong_legacy" {
        rho.solve_legacy(q, &jumps, &mut start_rng, &params, charges)
    } else {
        rho.solve(q, &jumps, &mut start_rng, &params, charges)
    };
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
pub const PRICED_NATIVE: &[(&str, &str)] = &[
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
    fb: Option<FactorBaseFacts>,
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
        factor_base: fb,
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

/// The factor-base builder `fb_name` names on a prime curve.
fn prime_base_for<'a>(
    fb_name: &str,
    inst: &'a PrimeInstance,
) -> Result<Box<dyn FactorBaseBuilder<crate::cryptanalysis::ic_boundary::PrimeCurve> + 'a>, String>
{
    Ok(match fb_name {
        "prime-abscissa" => Box::new(PrimeAbscissaBase { instance: inst }),
        "prime-projected-abscissa" => Box::new(PrimeProjectedAbscissaBase { instance: inst }),
        "glv-orbit" => Box::new(GlvOrbitBase { instance: inst }),
        "prime-kummer" => Box::new(PrimeKummerBase { instance: inst }),
        "prime-isogeny" => Box::new(PrimeIsogenyBase { instance: inst }),
        "cm-scalar-orbit" => Box::new(CmOrbitBase { instance: inst }),
        other => {
            return Err(format!(
                "factor base `{other}` does not run on a prime curve"
            ))
        }
    })
}

/// The instance context a factor-base builder is handed on a prime curve.
fn prime_ctx<'a>(
    inst: &'a PrimeInstance,
    curve: &CurveFacts,
    target: PrimePoint,
) -> InstanceCtx<'a, crate::cryptanalysis::ic_boundary::PrimeCurve> {
    InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: curve.slug.clone(),
        field_degree: None,
    }
}

fn solve_ic_prime(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    curve: &CurveFacts,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let (spec, fb_name, or_name) = pipeline_spec(m, seed)?;
    let base = prime_base_for(&fb_name, inst)?;
    let ms = spec.oracle_params.u64_or("m", 2)? as u32;
    let mut subtract = SubtractOracle;
    let mut mitm = MitmOracle::new(ms);
    let mut prime_batch = PrimeBatchMitmOracle::new();
    let mut prime_blocked = PrimeBatchMitmOracle::blocked();
    let mut mitm_fiber = FiberMitmOracle::new(ms);
    let mut rootfind_mitm = RootfindMitmOracle::new(ms);
    let mut rootfind_pairs = RootfindPairsOracle::new(ms);
    let mut rootfind = RootfindSubtractOracle::new(ms, inst);
    let oracle: &mut dyn DecompositionOracle<_> = match or_name.as_str() {
        "subtract" => &mut subtract,
        "mitm" => &mut mitm,
        "mitm-prime-batch" => &mut prime_batch,
        "mitm-prime-blocked" => &mut prime_blocked,
        "mitm-fiber" => &mut mitm_fiber,
        "rootfind-mitm" => &mut rootfind_mitm,
        "rootfind-pairs" => &mut rootfind_pairs,
        "rootfind-subtract" => &mut rootfind,
        other => return Err(format!("oracle `{other}` does not run on a prime curve")),
    };
    let ctx = prime_ctx(inst, curve, target);
    let calib = pinned_only_calibration("prime", &curve.slug);
    let t = Instant::now();
    // The runner, not the pipeline, checks the answer: pass a planted
    // value that cannot match so `verified` carries no information.
    let rep = run_pipeline(&ctx, &spec, &*base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    // The factor-base inventory is rebuilt after the measured span: see
    // [`complete`].
    Ok(ic_report(m, rep, &calib, None, wall, 2))
}

/// The inventory of a prime-curve `ic.pipeline` run: its factor base
/// rebuilt, outside every measured span.
fn prime_inventory(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    curve: &CurveFacts,
    target: PrimePoint,
) -> Result<FactorBaseFacts, String> {
    let (spec, fb_name, _) = pipeline_spec(m, 0)?;
    let base = prime_base_for(&fb_name, inst)?;
    let ctx = prime_ctx(inst, curve, target);
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |p| inst.curve.key(p),
    )
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
    Ok(ic_report(m, rep, &calib, None, wall, 2))
}

/// The inventory of a `fghr-y-line` run on a curve over `F_{p⁴}`, rebuilt
/// outside every measured span ([`complete`]).
fn fp4_inventory(
    m: &ResolvedMethod,
    inst: &D4Curve,
    curve: &CurveFacts,
    target: Pt4,
) -> Result<FactorBaseFacts, String> {
    let (spec, _, _) = pipeline_spec(m, 0)?;
    let base = YLineBase { curve: inst };
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
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )
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
    let c = &inst.curve;
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
    Ok(ic_report(m, rep, &calib, None, wall, 2))
}

/// The inventory of a `bin-t-line` run on a curve over `F_{(2^m)⁴}`,
/// rebuilt outside every measured span ([`complete`]).
fn f2m4_inventory(
    m: &ResolvedMethod,
    inst: &BinCurve,
    curve: &CurveFacts,
    target: PtB,
) -> Result<FactorBaseFacts, String> {
    let (spec, _, _) = pipeline_spec(m, 0)?;
    let base = TLineBase { curve: inst };
    let c = &inst.curve;
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
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )
}

/// `F_q` multiplications in one lift (`lift_x`) of a random abscissa
/// that lifts, averaged over 32.
fn f2m4_lift_muls(inst: &BinCurve) -> f64 {
    let c = &inst.curve;
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0x5e);
    let (mut muls, mut n) = (0u64, 0u64);
    while n < 32 {
        let x = c.f.random(&mut rng);
        let before = c.f.f.muls();
        if c.lift_x(&x, &mut rng).is_some() {
            muls += c.f.f.muls() - before;
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

/// The factor-base builder `fb_name` names on a binary curve: a built-in
/// one, or one a binary registers ([`register_binary_plugins`]).
fn binary_base_for<'a>(
    fb_name: &str,
    inst: &'a BinaryInstance,
) -> Result<Box<dyn FactorBaseBuilder<BinaryGroup<'a>> + 'a>, String> {
    Ok(match fb_name {
        "binary-subspace" => Box::new(BinarySubspaceBase { instance: inst }),
        "koblitz-orbit" => Box::new(KoblitzOrbitBase { instance: inst }),
        "compact-orbit-scan" => Box::new(CompactOrbitScanBase { instance: inst }),
        "cyclotomic" => Box::new(CyclotomicBase { instance: inst }),
        "onb-weight" => Box::new(OnbWeightBase { instance: inst }),
        other => match binary_plugins().and_then(|p| (p.base)(other, inst)) {
            Some(b) => b,
            None => {
                return Err(format!(
                    "factor base `{other}` does not run on a binary curve"
                ))
            }
        },
    })
}

/// The inventory of a binary-curve `ic.pipeline` run, rebuilt outside every
/// measured span ([`complete`]).
fn binary_inventory(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
) -> Result<FactorBaseFacts, String> {
    let (spec, fb_name, _) = pipeline_spec(m, 0)?;
    let g = BinaryGroup(&inst.fast);
    let base = binary_base_for(&fb_name, inst)?;
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
    let mut scratch = GroupOps::default();
    let fb = base.build(&ctx, &spec.factor_base_params, &mut scratch)?;
    factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |p| g.key(p),
    )
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
    let base = binary_base_for(&fb_name, inst)?;
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
    let rep = run_pipeline(&ctx, &spec, &*base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let a = if inst.koblitz.is_some() {
        2 * inst.n
    } else {
        2
    };
    // The inventory is rebuilt after the measured span: see [`complete`].
    Ok(ic_report(m, rep, &calib, None, wall, a))
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
        // Rebuilt after the measured span: see [`complete`].
        factor_base: None,
        detail: serde_json::to_value(&report).map_err(|e| e.to_string())?,
        online: Some(online),
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

/// The inventory of an `ic.shared_rank` run, rebuilt outside every measured
/// span ([`complete`]) and checked against the solver's own set-up.
fn shared_rank_inventory(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    detail: &Value,
) -> Result<FactorBaseFacts, String> {
    let (_, params) = Params::parse_spec(&m.params["factor_base"])?;
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
    let rank_figure = |name: &str| {
        detail
            .pointer(&format!("/rank/{name}"))
            .and_then(Value::as_u64)
    };
    if Some(facts.columns) != rank_figure("columns")
        || Some(facts.signed_points) != rank_figure("signed_points")
    {
        return Err("shared rank factor-base inventory differs from solver setup".into());
    }
    Ok(facts)
}

/// Finish a [`solve_measured`] report: rebuild the factor-base inventory of
/// an index-calculus run, which names the base in the record but changes no
/// count.  A measured child calls this after its timing, scheduler,
/// hardware-counter and Callgrind windows have closed, so the rebuild is
/// not in the solve's resource vector; [`solve`] calls both for every other
/// caller.  A report with an inventory already, and every method without
/// one, is left as it is.
pub fn complete(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    target: &[String; 2],
    rep: &mut SolveReport,
) -> Result<(), String> {
    if rep.factor_base.is_some() {
        return Ok(());
    }
    let base_m;
    let m = match full_price::base_of(&m.id) {
        Some(base) => {
            base_m = ResolvedMethod {
                id: base.to_string(),
                ..m.clone()
            };
            &base_m
        }
        None => m,
    };
    let family = decl(&m.id).map(|d| d.family);
    let pipeline = family == Some("ic")
        && !matches!(
            m.id.as_str(),
            "ic.large_prime"
                | "ic.large_prime_v2"
                | "ic.shared_rank"
                | "ic.k4_gaudry"
                | "ic.cm_folded_native_v1"
                | "ic.cm_folded_native_d43_v1"
                | "ic.cm_folded_recurrence_v1"
        );
    let facts = match inst {
        Instance::Prime(i) if pipeline => {
            let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
            Some(prime_inventory(m, i, curve, PrimePoint::affine(tx, ty))?)
        }
        Instance::Binary(i) if pipeline => {
            let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
            Some(binary_inventory(m, i, curve, FastPoint::affine(tx, ty))?)
        }
        Instance::Binary(i) if m.id == "ic.shared_rank" => {
            Some(shared_rank_inventory(m, i, curve, &rep.detail)?)
        }
        Instance::Fp4Torsion(i) if m.id == "ic.pipeline" => {
            let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
            let p = i.p();
            let target = Pt4 {
                x: unpack_e4(p, tx),
                y: unpack_e4(p, ty),
                inf: false,
            };
            Some(fp4_inventory(m, i, curve, target)?)
        }
        Instance::F2m4(i) if m.id == "ic.pipeline" => {
            let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
            let f = &i.curve.f;
            let target = PtB {
                x: f.unpack(tx),
                y: f.unpack(ty),
                inf: false,
            };
            Some(f2m4_inventory(m, i, curve, target)?)
        }
        _ => None,
    };
    rep.factor_base = facts;
    Ok(())
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
    if let Instance::Fp4(_) = &inst {
        return Err("no factor-base plug-in runs on E(F_p⁴); ic.k4_gaudry builds its own".into());
    }
    if let Instance::Fp4Torsion(_) | Instance::F2m4(_) = &inst {
        return Err("factor-base dumps cover prime and binary curves".into());
    }
    match &inst {
        Instance::Prime(i) => {
            let abscissa = PrimeAbscissaBase { instance: i };
            let projected = PrimeProjectedAbscissaBase { instance: i };
            let orbit = GlvOrbitBase { instance: i };
            let kummer = PrimeKummerBase { instance: i };
            let isogeny = PrimeIsogenyBase { instance: i };
            let base: &dyn FactorBaseBuilder<_> = match name.as_str() {
                "prime-abscissa" => &abscissa,
                "prime-projected-abscissa" => &projected,
                "glv-orbit" => &orbit,
                "prime-kummer" => &kummer,
                "prime-isogeny" => &isogeny,
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
            let onb_weight = OnbWeightBase { instance: i };
            let plugin_base = binary_plugins().and_then(|p| (p.base)(name.as_str(), i));
            let base: &dyn FactorBaseBuilder<BinaryGroup> = match name.as_str() {
                "binary-subspace" => &subspace,
                "koblitz-orbit" => &orbit,
                "compact-orbit-scan" => &compact,
                "cyclotomic" => &cyclotomic,
                "onb-weight" => &onb_weight,
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
        Instance::Wide(_) | Instance::Fp4(_) | Instance::Fp4Torsion(_) | Instance::F2m4(_) => {
            unreachable!("returned above")
        }
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

    /// `prime-isogeny` runs end to end: the answer is right, the
    /// auxiliary-curve search is charged in the factor-base phase and
    /// recorded apart, and the dump's id is the run's.
    #[test]
    fn prime_isogeny_base_charges_its_search_and_keeps_its_id() {
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
        let target = prime
            .curve
            .mul(&mut scratch, prime.generator_point(), 77_777);
        let coords = [format!("0x{:x}", target.x), format!("0x{:x}", target.y)];
        let fb_spec = "prime-isogeny:t=7,seed=1";
        let m = resolve(&MethodSpec {
            id: "ic.pipeline".into(),
            params: [
                ("factor_base".to_string(), fb_spec.to_string()),
                ("oracle".to_string(), "rootfind-mitm:m=3".to_string()),
            ]
            .into_iter()
            .collect(),
        })
        .unwrap();
        let rep = solve(&m, &inst, &facts, &coords, 5).unwrap();
        assert_eq!(rep.recovered, Some(77_777 % u128::from(prime.r)));
        let fbp = rep.phases.iter().find(|p| p.name == "factor_base").unwrap();
        let charged = fbp.native["aux_search_gae"] + fbp.native["aux_tower_build_gae"];
        assert!(charged > 0 && fbp.native["aux_curves_tried"] >= 1);
        assert_eq!((fbp.adds + fbp.doubles) as u64, charged);
        assert_eq!(fbp.gae, charged as f64);
        let fb = rep.factor_base.as_ref().unwrap();
        assert_eq!(fb.family, "prime-isogeny");
        let dump = dump_factor_base(&spec, fb_spec).unwrap();
        assert_eq!(dump.factor_base.fb_id, fb.fb_id);
        assert_eq!(dump.build_adds + dump.build_doubles, charged);
        let again = solve(&m, &inst, &facts, &coords, 5).unwrap();
        assert_eq!(again.total_gae, rep.total_gae);
        assert_eq!(again.factor_base.unwrap().fb_id, fb.fb_id);
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
    fn spelled(id: &str, name: &str, value: &str) -> ResolvedMethod {
        let mut params = BTreeMap::new();
        params.insert(name.to_string(), value.to_string());
        resolve(&MethodSpec {
            id: id.into(),
            params,
        })
        .unwrap()
    }

    #[test]
    fn integer_parameters_have_one_spelling_and_so_one_method_id() {
        // `64` is the default of `cap_multiple`: written out, zero-padded
        // or signed it is the same method.
        let default = resolve(&MethodSpec {
            id: "rho.negation".into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        for spelling in ["64", "064", "+64", "0064", "+0064"] {
            let m = spelled("rho.negation", "cap_multiple", spelling);
            assert_eq!(m.params["cap_multiple"], "64", "{spelling}");
            assert_eq!(m.method_id, default.method_id, "{spelling}");
            assert_eq!(m.method_sha256, default.method_sha256, "{spelling}");
        }
        // A different value is a different method; zero stays zero.
        assert_ne!(
            spelled("rho.negation", "cap_multiple", "65").method_id,
            default.method_id
        );
        assert_eq!(
            spelled("rho.negation", "cap_multiple", "+000").params["cap_multiple"],
            "0"
        );
        // A value that is not an integer lexeme is left as written (the
        // method refuses it when it reads it), and a string parameter is
        // never rewritten.
        assert_eq!(
            spelled("rho.negation", "cap_multiple", "6.0").params["cap_multiple"],
            "6.0"
        );
        assert_eq!(
            spelled("rho.negation", "cap_multiple", "1e3").params["cap_multiple"],
            "1e3"
        );
        assert_eq!(normalise_param("backend_sha256", None, "0123"), "0123");
        assert_eq!(normalise_param("summands", None, "n-1"), "n-1");
        assert_eq!(normalise_param("summands", None, "+007"), "7");
        assert_eq!(
            normalise_param("lp_exponent", Some("0.75"), "0.750"),
            "0.750"
        );
    }
}

/// Tests of the accounting rules (README §7 "Accounting v2"): the `_v2`
/// ids recover known logarithms on a prime and a Koblitz curve, every
/// registered method's phases sum to its total (the frozen-known-defect ids
/// exempted by name), and the floor's `A` is never below what a method used.
#[cfg(test)]
mod accounting_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::{legacy_automorphisms, CurveSpec, FLOOR_RULE_V2};

    /// The `j = 0` curve of the deep-dive review (`y² = x³ + 2` over
    /// `F_4194397`, prime order `r = 1048627`): `A = 6`.
    fn j0() -> CurveSpec {
        CurveSpec::PrimeExplicit {
            p: 4194397,
            a: 0,
            b: 2,
            group_order: 4194508,
            r: 1048627,
            gx: 2359468,
            gy: 1145958,
        }
    }

    /// A `j = 1728` curve `y² = x³ + ax` over a prime `p ≡ 1 (mod 4)` below
    /// `2^14`, found by counting points, with a prime subgroup order
    /// `r ≡ 1 (mod 4)` and a cofactor of at most 8: `A = 4`.
    fn j1728() -> CurveSpec {
        use crate::cryptanalysis::ic_boundary::{PrimeCurve, PrimePoint};
        use crate::cryptanalysis::residual_walk::is_prime_u64;
        for p in (8000u64..16384).filter(|&p| p % 4 == 1 && is_prime_u64(p)) {
            for a in 1..12u64 {
                let curve = PrimeCurve { p, a, b: 0 };
                let n = curve.point_count();
                for h in [1u64, 2, 4, 8] {
                    if n % h != 0 {
                        continue;
                    }
                    let r = n / h;
                    if r % 4 != 1 || !is_prime_u64(r) {
                        continue;
                    }
                    // A point of order r: clear the cofactor of a point.
                    for x in 1..p {
                        let rhs = (x * x % p * x + a * x) % p;
                        let Some(y) = curve.sqrt(rhs) else { continue };
                        let mut ops = GroupOps::default();
                        let g = curve.mul(&mut ops, PrimePoint::affine(x, y), h);
                        if g.infinity {
                            continue;
                        }
                        return CurveSpec::PrimeExplicit {
                            p,
                            a,
                            b: 0,
                            group_order: n,
                            r,
                            gx: g.x,
                            gy: g.y,
                        };
                    }
                }
            }
        }
        panic!("no j = 1728 test curve below 2^14");
    }

    fn built(spec: &CurveSpec) -> (Instance, CurveFacts) {
        let inst = spec.build().unwrap();
        let facts = inst.facts(spec);
        (inst, facts)
    }

    fn method(id: &str, params: &[(&str, &str)]) -> ResolvedMethod {
        resolve(&MethodSpec {
            id: id.into(),
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        })
        .unwrap()
    }

    fn run(
        m: &ResolvedMethod,
        inst: &Instance,
        facts: &CurveFacts,
        k: u128,
        seed: u64,
    ) -> SolveReport {
        let target = inst.mul_generator_hex(k).expect("a nonzero target");
        solve(m, inst, facts, &target, seed).unwrap()
    }

    /// The phases of a report add up to its total, exactly.
    fn assert_phases_sum(id: &str, rep: &SolveReport) {
        let sum: f64 = rep.phases.iter().map(|p| p.gae).sum();
        assert_eq!(sum, rep.total_gae, "{id}: phases {:?}", rep.phases);
    }

    #[test]
    fn the_floor_counts_the_automorphisms_a_curve_has() {
        let (_, f0) = built(&j0());
        assert_eq!(f0.automorphisms_available, 6, "j = 0, p ≡ 1 (mod 3)");
        // The first rule read 2 there; a record written under it carries no
        // floor_rule and recomputes to that.
        assert_eq!(legacy_automorphisms(&f0), 2);
        let (_, f1728) = built(&j1728());
        assert_eq!(f1728.automorphisms_available, 4, "j = 1728, p ≡ 1 (mod 4)");
        assert_eq!(legacy_automorphisms(&f1728), 2);
        // An ordinary prime curve keeps negation only.
        let (_, f) = built(&CurveSpec::PrimeSearch {
            bits: 16,
            seed: 59297,
        });
        assert_eq!(f.automorphisms_available, 2);
        // A Koblitz curve is 2n by either rule, constructed or given outright
        // (the explicit form of the committed n = 13 curve).
        let (_, fk) = built(&CurveSpec::Koblitz { a: 0, n: 13 });
        assert_eq!(fk.automorphisms_available, 26);
        let (_, fe) = built(&CurveSpec::BinaryExplicit {
            n: 13,
            modulus: 8219,
            a: 0,
            b: 1,
            group_order: 8012,
            r: 2003,
            gx: 4793,
            gy: 7108,
        });
        assert_eq!(fe.family, "koblitz");
        assert_eq!(fe.automorphisms_available, 26);
        assert_eq!(legacy_automorphisms(&fe), 26);
    }

    #[test]
    fn a_record_names_the_rule_its_floor_was_computed_under() {
        use crate::cryptanalysis::ecbench::record::expected_automorphisms;
        let (_, f) = built(&j0());
        assert_eq!(expected_automorphisms(None, &f), Some(2));
        assert_eq!(expected_automorphisms(Some(FLOOR_RULE_V2), &f), Some(6));
        assert_eq!(expected_automorphisms(Some("automorphisms/v9"), &f), None);
    }

    /// The deep-dive's reproduction: on a `j = 0` curve `rho.cubic` read
    /// 0.91× the floor of `A = 2`.  Against the floor of `A = 6` it reads
    /// above 1, as a generic method must on average.
    #[test]
    fn rho_cubic_reads_at_least_the_j0_floor_in_expectation() {
        use crate::cryptanalysis::ecbench::record::floor_s;
        let spec = j0();
        let (inst, facts) = built(&spec);
        let floor = floor_s(facts.automorphisms_available);
        let m = method("rho.cubic_v2", &[]);
        let runs = 24u64;
        let mut s = 0.0;
        for i in 0..runs {
            let k = 12_345 + 40_503 * u128::from(i);
            let rep = run(&m, &inst, &facts, k, 100 + i);
            assert_eq!(rep.recovered, Some(k % facts.r));
            assert!(rep.automorphisms_used <= facts.automorphisms_available);
            s += rep.total_gae / (facts.r as f64).sqrt();
        }
        let ratio = s / runs as f64 / floor;
        assert!(ratio > 1.0, "mean S over the A = 6 floor is {ratio}");
    }

    /// `rho.cubic` and `rho.quartic` leave the set-up `[κ]G` ledger out of
    /// `total_gae`: the phases sum to more than the total, by exactly that
    /// ledger.  Frozen under their ids and documented as known-undercharged;
    /// the `_v2` ids include it.
    #[test]
    fn cubic_and_quartic_set_up_is_in_the_v2_total_and_not_the_frozen_one() {
        for (spec, old, new) in [
            (j0(), "rho.cubic", "rho.cubic_v2"),
            (j1728(), "rho.quartic", "rho.quartic_v2"),
        ] {
            let (inst, facts) = built(&spec);
            let k = 4321u128 % facts.r;
            let frozen = run(&method(old, &[]), &inst, &facts, k, 7);
            let v2 = run(&method(new, &[]), &inst, &facts, k, 7);
            assert_eq!(frozen.recovered, Some(k));
            assert_eq!(v2.recovered, Some(k));
            let kappa = v2
                .phases
                .iter()
                .find(|p| p.name == "automorphism_setup")
                .expect("the v2 report has the phase");
            assert!(kappa.gae > 0.0 && kappa.scalar_mults >= 1, "{kappa:?}");
            let frozen_sum: f64 = frozen.phases.iter().map(|p| p.gae).sum();
            assert_eq!(
                frozen_sum - frozen.total_gae,
                kappa.gae,
                "{old}: the phases carry the [κ]G ledger the total leaves out"
            );
            assert_phases_sum(new, &v2);
            assert_eq!(
                v2.counters["automorphism_setup_additions"]
                    + v2.counters["automorphism_setup_doublings"],
                kappa.gae as u64
            );
            // The walk itself is the same under both ids' seeds except for
            // Brent detection; the v2 total is the walk's plus the ledger.
            assert!(v2.total_gae > kappa.gae);
        }
    }

    #[test]
    fn automorphisms_used_never_exceed_those_available() {
        let (inst, facts) = built(&j0());
        for id in [
            "rho.plain",
            "rho.negation",
            "rho.negation_v2",
            "rho.cubic",
            "rho.cubic_v2",
            "bsgs.negation_v2",
        ] {
            let rep = run(&method(id, &[]), &inst, &facts, 999, 3);
            assert!(
                rep.automorphisms_used <= facts.automorphisms_available,
                "{id}: used {} of {}",
                rep.automorphisms_used,
                facts.automorphisms_available
            );
        }
        let (inst, facts) = built(&CurveSpec::Koblitz { a: 0, n: 13 });
        for id in [
            "rho.signed_frobenius",
            "rho.signed_frobenius_v2",
            "rho.signed_frobenius_strong",
            "rho.signed_frobenius_strong_v2",
            "rho.negation_v2",
        ] {
            let rep = run(&method(id, &[]), &inst, &facts, 999, 3);
            assert!(
                rep.automorphisms_used <= facts.automorphisms_available,
                "{id}: used {} of {}",
                rep.automorphisms_used,
                facts.automorphisms_available
            );
        }
    }

    /// Every `_v2` id recovers known logarithms on a prime curve and on a
    /// Koblitz curve (both where it applies), with phases that sum to its
    /// total and, for BSGS, the kangaroo and the claw, one charged check.
    #[test]
    fn v2_methods_recover_known_logarithms_and_sum_their_phases() {
        let prime = built(&CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        });
        let koblitz = built(&CurveSpec::Koblitz { a: 0, n: 19 });
        let claw: &[(&str, &str)] = &[("base_scale", "4.0"), ("table_scale", "1.0")];
        // (id, params, on the prime curve too)
        let cases: &[(&str, &[(&str, &str)], bool)] = &[
            ("rho.negation_v2", &[], true),
            ("rho.signed_frobenius_v2", &[], false),
            ("rho.signed_frobenius_strong_v2", &[], false),
            ("bsgs.textbook_v2", &[], true),
            ("bsgs.interleaved_v2", &[], true),
            ("bsgs.negation_v2", &[], true),
            ("kangaroo.vow_v2", &[], true),
            ("claw.pair_table_v2", claw, false),
        ];
        for &(id, params, on_prime) in cases {
            let m = method(id, params);
            let mut curves = vec![&koblitz];
            if on_prime {
                curves.push(&prime);
            }
            for (inst, facts) in curves {
                let r = facts.r;
                for (j, k) in [3u128, r - 2, r / 3 + 5, r / 2 + 11]
                    .into_iter()
                    .enumerate()
                {
                    let rep = run(&m, inst, facts, k, 40 + j as u64);
                    assert!(!rep.exhausted, "{id} on {} k={k}", facts.slug);
                    assert_eq!(rep.recovered, Some(k), "{id} on {} k={k}", facts.slug);
                    assert_phases_sum(id, &rep);
                    assert!(
                        rep.automorphisms_used <= facts.automorphisms_available,
                        "{id}"
                    );
                    let checks = rep
                        .phases
                        .iter()
                        .find(|p| {
                            matches!(p.name.as_str(), "verification" | "internal_verification")
                        })
                        .unwrap_or_else(|| panic!("{id}: no verification phase"));
                    assert!(checks.gae > 0.0, "{id}: the final check is charged");
                }
            }
        }
    }

    /// The verification a BSGS, kangaroo or claw run pays is the one
    /// `[d]G = Q` rho and index calculus already pay: about `1.5·log₂ r`.
    #[test]
    fn the_v2_verification_is_one_scalar_multiplication() {
        let (inst, facts) = built(&CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        });
        let k = 100_003u128;
        for (old, new) in [
            ("bsgs.textbook", "bsgs.textbook_v2"),
            ("bsgs.interleaved", "bsgs.interleaved_v2"),
            ("bsgs.negation", "bsgs.negation_v2"),
            ("kangaroo.vow", "kangaroo.vow_v2"),
        ] {
            let a = run(&method(old, &[]), &inst, &facts, k, 9);
            let b = run(&method(new, &[]), &inst, &facts, k, 9);
            let v = b
                .phases
                .iter()
                .find(|p| p.name == "verification")
                .expect("verification phase");
            assert_eq!(v.scalar_mults, 1, "{new}");
            let bits = 64 - (k as u64).leading_zeros();
            assert!(
                v.gae >= f64::from(bits - 1) && v.gae <= 2.0 * f64::from(bits),
                "{new}: {} for a {bits}-bit scalar",
                v.gae
            );
            // The search is the old one: only the check (and, for the
            // kangaroo, the phase a restart is charged to) differs.
            assert_eq!(a.recovered, b.recovered, "{new}");
            let a_total: f64 = a.phases.iter().map(|p| p.gae).sum();
            assert_eq!(a_total, a.total_gae, "{old}");
            assert_eq!(b.total_gae, a.total_gae + v.gae, "{new}");
        }
    }

    /// A kangaroo restart is search work, not set-up: v2 charges it to
    /// `restarts`.  Frozen `kangaroo.vow` keeps it in `setup`.
    #[test]
    fn kangaroo_v2_charges_restarts_apart_and_the_old_id_does_not() {
        use crate::cryptanalysis::ecbench::generic::{kangaroo, kangaroo_with};
        // Same-kind meetings (a kangaroo on its own trail) are rare; every
        // target of an 8- and a 10-bit group, four seeds each.
        let mut restarts = 0u64;
        for bits in [8u32, 10] {
            let (inst, facts) = built(&CurveSpec::PrimeSearch { bits, seed: 59297 });
            let Instance::Prime(p) = &inst else {
                unreachable!()
            };
            let (g, gen) = (&p.curve, p.generator_point());
            for k in 1..facts.r as u64 {
                let mut scratch = GroupOps::default();
                let q = g.mul(&mut scratch, gen, k);
                for seed in 0..4u64 {
                    let cap = rho_cap(p.r, 64.0);
                    let old = kangaroo(g, gen, q, p.r, seed, cap);
                    let new = kangaroo_with(g, gen, q, p.r, seed, cap, true);
                    assert_eq!(old.recovered, new.recovered);
                    assert_eq!(old.counters, new.counters, "the same walk");
                    assert_eq!(old.search, new.search);
                    // The old setup is the new setup plus the restarts.
                    assert_eq!(old.setup.gae(), new.setup.gae() + new.restarts.gae());
                    let same_kind = old.counters.get("same_kind_collisions").copied();
                    if same_kind.unwrap_or(0) > 0 {
                        assert!(new.restarts.gae() > 0.0, "k={k} seed={seed}");
                        restarts += 1;
                    } else {
                        assert_eq!(new.restarts.gae(), 0.0);
                    }
                }
            }
        }
        // Same-kind meetings did not occur on these groups in this sweep
        // (no committed kangaroo.vow record has one either); every run checked
        // that the walk and its total are the frozen one's and that the
        // restart ledger is exactly what left `setup`.
        let _ = restarts;
        // Through the method: the phase is there and the totals agree.
        let (inst, facts) = built(&CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        });
        let old = run(&method("kangaroo.vow", &[]), &inst, &facts, 4242, 1);
        let new = run(&method("kangaroo.vow_v2", &[]), &inst, &facts, 4242, 1);
        assert!(new.phases.iter().any(|p| p.name == "restarts"));
        assert!(!old.phases.iter().any(|p| p.name == "restarts"));
    }

    /// A walk that cannot see a cycle (window 0) runs to the walk cap; the
    /// Brent detector escapes it.  Both reach the logarithm; the lost steps
    /// are a charged counter, and the walk the frozen ids run is untouched.
    #[test]
    fn brent_detection_escapes_long_cycles_and_counts_the_steps_lost() {
        use crate::cryptanalysis::ic_boundary::{
            rho_walk_with_policy, NegationClasses, RhoWalk, WalkPolicy,
        };
        let (inst, facts) = built(&CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        });
        let Instance::Prime(p) = &inst else {
            unreachable!()
        };
        let blind = RhoWalk {
            negation: true,
            jumps: 2,
            dp_bits: None,
            look_ahead: false,
            cycle_window: 0,
        };
        let classes = NegationClasses { r: p.r };
        let (mut capped_old, mut capped_new, mut cycles, mut lost) = (0u64, 0u64, 0u64, 0u64);
        for seed in 0..8u64 {
            let k = 31_337 + 9_001 * u128::from(seed);
            let mut scratch = GroupOps::default();
            let target = p.curve.mul(&mut scratch, p.generator_point(), k as u64);
            let go = |policy| {
                rho_walk_with_policy(
                    &p.curve,
                    &classes,
                    p.generator_point(),
                    target,
                    p.r,
                    seed,
                    1 << 22,
                    blind,
                    policy,
                )
            };
            let old = go(WalkPolicy::legacy());
            let new = go(WalkPolicy::v2());
            assert_eq!(new.recovered, Some(k as u64 % p.r), "seed {seed}");
            capped_old += old.counters["walks_capped"];
            capped_new += new.counters["walks_capped"];
            cycles += new.counters["cycles_longer_than_window"];
            lost += new.counters["lost_walk_steps"];
            // The extra counters are v2's only; the frozen walk's are as ever.
            assert!(!old.counters.contains_key("lost_walk_steps"));
            assert!(new.counters["lost_walk_steps"] <= new.counters["walk_operations"]);
            // The lost steps are charged: inside `walk_operations`.
            let by_kind = new.counters["steps_in_walks_capped"]
                + new.counters["steps_in_walks_abandoned_in_a_cycle"]
                + new.counters["steps_in_walks_ending_at_the_identity"]
                + new.counters["steps_in_walks_out_of_budget"];
            assert_eq!(by_kind, new.counters["lost_walk_steps"]);
        }
        assert!(cycles > 0, "no long cycle was found");
        assert!(
            capped_new * 4 <= capped_old.max(1),
            "capped walks: legacy {capped_old}, brent {capped_new}"
        );
        let _ = (facts, lost);
    }

    /// The tuned walk's frozen ids keep exactly the counters they were
    /// recorded with; the `_v2` ones add theirs.
    #[test]
    fn frozen_rho_counters_are_unchanged_and_v2_adds_its_own() {
        let (inst, facts) = built(&CurveSpec::Koblitz { a: 0, n: 19 });
        let old = run(&method("rho.signed_frobenius", &[]), &inst, &facts, 5555, 8);
        let new = run(
            &method("rho.signed_frobenius_v2", &[]),
            &inst,
            &facts,
            5555,
            8,
        );
        for key in [
            "lost_walk_steps",
            "cycles_longer_than_window",
            "distinguished_point_table_queries_uncharged",
            "automorphism_setup_additions",
        ] {
            assert!(
                !old.counters.contains_key(key),
                "{key} leaked into the old id"
            );
            assert!(new.counters.contains_key(key), "{key} missing from v2");
        }
        assert!(new
            .unpriced
            .contains(&"distinguished_point_table_queries_uncharged".to_string()));
        assert_eq!(old.phases.len(), 3);
        assert_eq!(new.phases.len(), 4);
    }

    #[test]
    fn strong_v2_scales_its_set_up_with_r() {
        // n = 37: r ≈ 2^35, A = 74: the full table, a small share.
        let ideal = (std::f64::consts::PI * 2f64.powi(35) / 148.0).sqrt();
        let big = strong_v2_shape(35.0, ideal);
        assert_eq!(big.distinct_jumps, 32);
        assert!(
            strong_v2_fixed_setup(35.0, 32) <= 0.1 * ideal,
            "{}",
            strong_v2_fixed_setup(35.0, 32)
        );
        assert!((6..=8).contains(&big.dp_bits), "{big:?}");
        assert!(big.lanes >= 8, "{big:?}");
        // n = 13: r ≈ 2^11, A = 26: ~11 expected steps.  No table is small
        // enough; the shape takes the smallest, one lane and short walks.
        let ideal = (std::f64::consts::PI * 2003.0 / 52.0).sqrt();
        let tiny = strong_v2_shape(11.0, ideal);
        assert_eq!(tiny.distinct_jumps, 8);
        assert_eq!(tiny.lanes, 1);
        assert!(tiny.dp_bits <= 2, "{tiny:?}");
        // Monotone in r.
        let mut last = StrongV2Shape {
            lanes: 0,
            dp_bits: 0,
            distinct_jumps: 0,
        };
        for bits in 10..=80 {
            let ideal = (std::f64::consts::PI * 2f64.powi(bits) / 150.0).sqrt();
            let s = strong_v2_shape(f64::from(bits), ideal);
            assert!(s.lanes >= last.lanes && s.dp_bits >= last.dp_bits, "{bits}");
            assert!(s.distinct_jumps >= last.distinct_jumps, "{bits}");
            last = s;
        }
    }

    /// The v2 strong reference's fixed set-up against the frozen one at the
    /// sizes of the review: n = 13 and n = 19, where the frozen S is 58× and
    /// 15.5× theory.  Both walk the same group; v2 is the cheaper reference
    /// and its fixed share is reported.
    #[test]
    fn strong_v2_cuts_the_fixed_set_up_at_claim_sizes_and_reports_the_share() {
        for (n, max_old_over_new_floor) in [(13u32, 10.0f64), (19, 10.0)] {
            let (inst, facts) = built(&CurveSpec::Koblitz { a: 0, n });
            let (mut old, mut new) = (0.0, 0.0);
            let runs = 16u64;
            for i in 0..runs {
                let k = 977 + 4_099 * u128::from(i);
                let o = run(
                    &method("rho.signed_frobenius_strong", &[]),
                    &inst,
                    &facts,
                    k,
                    i,
                );
                let v = run(
                    &method("rho.signed_frobenius_strong_v2", &[]),
                    &inst,
                    &facts,
                    k,
                    i,
                );
                assert_eq!(v.recovered, Some(k % facts.r));
                assert_phases_sum("strong_v2", &v);
                old += o.total_gae;
                new += v.total_gae;
            }
            assert!(new * 2.0 < old, "n={n}: v1 {old}, v2 {new}");
            let floor_ops = ((facts.r as f64).sqrt()
                * crate::cryptanalysis::ecbench::record::floor_s(facts.automorphisms_available))
                * runs as f64;
            assert!(old / floor_ops > 2.0, "n={n}: frozen set-up dominates");
            let _ = max_old_over_new_floor;
        }
    }

    /// An exhausted strong-reference solve records the work it spent: the
    /// frozen id records total 0 and no phases.
    #[test]
    fn exhausted_strong_v2_records_the_spent_work() {
        let spec = CurveSpec::Koblitz { a: 0, n: 13 };
        let (inst, facts) = built(&spec);
        let Instance::Binary(b) = &inst else {
            unreachable!()
        };
        // A point of the curve outside the order-r subgroup (h = 4).
        let out_of_subgroup = (1u64..)
            .flat_map(|x| b.points_with_x(x))
            .find(|q| !b.fast.mul_u64(*q, b.r).infinity)
            .expect("the cofactor is 4");
        let target = [
            format!("0x{:x}", out_of_subgroup.x),
            format!("0x{:x}", out_of_subgroup.y),
        ];
        let frozen = solve(
            &method("rho.signed_frobenius_strong", &[]),
            &inst,
            &facts,
            &target,
            3,
        )
        .unwrap();
        assert!(frozen.exhausted);
        assert_eq!(frozen.total_gae, 0.0);
        assert!(frozen.phases.is_empty());
        let v2 = solve(
            &method("rho.signed_frobenius_strong_v2", &[]),
            &inst,
            &facts,
            &target,
            3,
        )
        .unwrap();
        assert!(v2.exhausted && v2.recovered.is_none());
        assert!(v2.total_gae > 0.0);
        assert_phases_sum("strong_v2 exhausted", &v2);
        assert!(v2.counters["walk_steps"] > 0);
        assert!(v2.counters["failed_collisions"] > 0 || v2.counters["table_inserts_uncharged"] > 0);
        // The failed candidate checks are charged to verification.
        let checks = v2.phases.iter().find(|p| p.name == "verification").unwrap();
        assert_eq!(checks.scalar_mults, v2.counters["failed_collisions"]);
    }

    /// The claw's budget counts its recover phase in v2; the first rule's
    /// does not.
    #[test]
    fn the_claw_v2_spending_cap_includes_recover_and_a_check_is_charged() {
        use crate::cryptanalysis::ecbench::claw::{pair_claw, pair_claw_v2, ClawShape};
        use crate::cryptanalysis::ic_boundary::koblitz_instance;
        let inst = koblitz_instance(0, 19).unwrap();
        let mut ops = GroupOps::default();
        let q = BinaryGroup(&inst.fast).mul(&mut ops, inst.generator, 1234);
        let shape = ClawShape {
            base_scale: 4.0,
            table_scale: 1.0,
            max_adds: u64::MAX,
        };
        let old = pair_claw(&inst, q, 5, shape).unwrap();
        let new = pair_claw_v2(&inst, q, 5, shape).unwrap();
        assert_eq!(old.recovered, new.recovered);
        assert_eq!(old.search.gae(), new.search.gae());
        assert_eq!(old.verification.gae(), 0.0);
        assert!(new.verification.gae() > 0.0 && new.verification.scalar_mults == 1);
        assert_eq!(new.counters["verification_checks"], 1);
        // A budget that the base, table and search just fit but the recover
        // addition does not: the first rule overspends it, v2 stops short.
        let spent = old.base.gae() + old.table.gae() + old.search.gae();
        assert!(old.recover.gae() > 0.0);
        let tight = ClawShape {
            max_adds: spent as u64 + 1,
            ..shape
        };
        let a = pair_claw(&inst, q, 5, tight).unwrap();
        let b = pair_claw_v2(&inst, q, 5, tight).unwrap();
        let charged = |o: &crate::cryptanalysis::ecbench::claw::ClawOutcome, with_recover: bool| {
            o.base.gae()
                + o.table.gae()
                + o.search.gae()
                + if with_recover { o.recover.gae() } else { 0.0 }
        };
        assert!(charged(&a, false) <= tight.max_adds as f64 + 2.0);
        assert!(
            b.exhausted || charged(&b, true) <= tight.max_adds as f64 + 2.0,
            "v2 charged {}",
            charged(&b, true)
        );
    }

    /// `ic.large_prime_v2` counts the `[r]Q` membership check and the final
    /// `[d]G = Q`; the first rule runs the first on a ledger nobody reads and
    /// leaves the second to the runner.
    #[test]
    fn large_prime_v2_charges_the_membership_check_and_the_final_check() {
        let spec = CurveSpec::BinaryExplicit {
            n: 11,
            modulus: 2323,
            a: 1,
            b: 1,
            group_order: 1982,
            r: 991,
            gx: 1193,
            gy: 1007,
        };
        let (inst, facts) = built(&spec);
        let params: &[(&str, &str)] = &[
            ("small_dimension", "3"),
            ("envelope_dimension", "4"),
            ("summands", "n-1"),
            ("large_primes", "2"),
            ("max_trials", "20000"),
            ("max_states", "100000"),
        ];
        let (old, new) = (
            method("ic.large_prime", params),
            method("ic.large_prime_v2", params),
        );
        for (j, k) in [5u128, 400, 777].into_iter().enumerate() {
            let a = run(&old, &inst, &facts, k, j as u64);
            let b = run(&new, &inst, &facts, k, j as u64);
            assert_eq!(a.recovered, Some(k));
            assert_eq!(b.recovered, Some(k));
            let v = |r: &SolveReport| {
                r.phases
                    .iter()
                    .find(|p| p.name == "verification")
                    .unwrap()
                    .clone()
            };
            // Same search; the check is the only difference.
            assert_eq!(a.counters["trials"], b.counters["trials"]);
            assert_eq!(b.counters["membership_checks"], 1);
            assert_eq!(b.counters["final_checks"], 1);
            assert_eq!(b.counters["final_checks_failed"], 0);
            assert_eq!(v(&b).scalar_mults, v(&a).scalar_mults + 2);
            assert!(v(&b).gae > v(&a).gae);
            assert_phases_sum("ic.large_prime", &a);
            assert_phases_sum("ic.large_prime_v2", &b);
            assert!(!a.counters.contains_key("membership_checks"));
        }
    }

    /// The factor-base inventory of an index-calculus run is built by
    /// [`complete`], after [`solve_measured`]: the measured report has none and
    /// the finished one is what [`solve`] returns.
    #[test]
    fn the_inventory_is_rebuilt_after_the_measured_solve() {
        let cases: Vec<(CurveSpec, ResolvedMethod)> = vec![
            (
                CurveSpec::PrimeSearch {
                    bits: 16,
                    seed: 59297,
                },
                method(
                    "ic.pipeline",
                    &[
                        ("factor_base", "prime-abscissa:size=32"),
                        ("oracle", "mitm:negation_folded=1"),
                    ],
                ),
            ),
            (
                CurveSpec::Koblitz { a: 1, n: 17 },
                method(
                    "ic.pipeline",
                    &[
                        ("factor_base", "koblitz-orbit:divisor=0;1"),
                        ("oracle", "mitm-frobenius:m=2"),
                    ],
                ),
            ),
        ];
        for (spec, m) in cases {
            let (inst, facts) = built(&spec);
            let k = 4242u128;
            let target = inst.mul_generator_hex(k).unwrap();
            let measured = solve_measured(&m, &inst, &facts, &target, 5).unwrap();
            assert!(measured.factor_base.is_none(), "{}", facts.slug);
            assert_eq!(measured.recovered, Some(k));
            let mut done = measured.clone();
            complete(&m, &inst, &facts, &target, &mut done).unwrap();
            let fb = done.factor_base.clone().expect("inventory after complete");
            assert!(fb.columns > 0 && fb.fb_id.starts_with("FB1h"));
            // Nothing but the inventory differs, and solve() agrees.
            let mut same = done.clone();
            same.factor_base = None;
            same.solve_wall_ns = measured.solve_wall_ns;
            assert_eq!(same.phases, measured.phases);
            assert_eq!(same.counters, measured.counters);
            assert_eq!(same.total_gae, measured.total_gae);
            let whole = solve(&m, &inst, &facts, &target, 5).unwrap();
            assert_eq!(whole.factor_base, done.factor_base);
            assert_eq!(whole.total_gae, done.total_gae);
        }
    }

    #[test]
    fn table_flags_name_a_dominant_set_up_and_an_uncalibrated_size() {
        let phases = |setup: f64, search: f64| {
            vec![
                PhaseRecord {
                    name: "setup".into(),
                    gae: setup,
                    ..Default::default()
                },
                PhaseRecord {
                    name: "search".into(),
                    gae: search,
                    ..Default::default()
                },
            ]
        };
        assert_eq!(setup_share("rho", &phases(30.0, 70.0), 100.0), Some(0.3));
        // BSGS's table scales with √r; it has no fixed set-up to flag.
        assert_eq!(setup_share("bsgs", &phases(30.0, 70.0), 100.0), None);
        let f = row_flags("koblitz", 35.0, "rho", &phases(30.0, 70.0), 100.0);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(row_flags("koblitz", 40.0, "rho", &phases(5.0, 95.0), 100.0).is_empty());
        let f = row_flags("prime", 24.0, "bsgs", &phases(30.0, 70.0), 100.0);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(row_flags("prime", 26.0, "bsgs", &phases(30.0, 70.0), 100.0).is_empty());
    }

    /// Every registered method either has a case below or a stated reason
    /// it cannot run on a toy curve; each case's phases sum to its total.
    /// `rho.cubic` and `rho.quartic` are the only frozen ids exempted: their
    /// phases carry a set-up ledger their total omits, kept as recorded.
    #[test]
    fn every_registered_methods_phases_sum_to_its_total() {
        let prime = built(&CurveSpec::PrimeSearch {
            bits: 18,
            seed: 59297,
        });
        let koblitz = built(&CurveSpec::Koblitz { a: 0, n: 19 });
        // The orbit base needs an invariant subspace: K_1 over F_2^17 has one.
        let koblitz17 = built(&CurveSpec::Koblitz { a: 1, n: 17 });
        let ic_prime: &[(&str, &str)] = &[
            ("factor_base", "prime-abscissa:size=32"),
            ("oracle", "mitm:negation_folded=1"),
        ];
        let ic_binary: &[(&str, &str)] = &[
            ("factor_base", "koblitz-orbit:divisor=0;1"),
            ("oracle", "mitm-frobenius:m=2"),
        ];
        let claw: &[(&str, &str)] = &[("base_scale", "4.0"), ("table_scale", "1.0")];
        let cases: Vec<(&str, &[(&str, &str)], &(Instance, CurveFacts))> = vec![
            ("rho.frozen_reference", &[], &prime),
            ("rho.plain", &[], &prime),
            ("rho.negation", &[], &prime),
            ("rho.signed_frobenius", &[], &koblitz),
            ("rho.signed_frobenius_strong", &[], &koblitz),
            ("rho.signed_frobenius_strong_legacy", &[], &koblitz),
            ("rho.signed_frobenius_strong_escape", &[], &koblitz),
            ("bsgs.textbook", &[], &prime),
            ("bsgs.interleaved", &[], &prime),
            ("bsgs.negation", &[], &prime),
            ("kangaroo.vow", &[], &prime),
            ("claw.pair_table", claw, &koblitz),
            ("ic.pipeline", ic_prime, &prime),
            ("ic.pipeline_counted", ic_prime, &prime),
            ("ic.pipeline", ic_binary, &koblitz17),
            ("rho.negation_v2", &[], &prime),
            ("rho.signed_frobenius_v2", &[], &koblitz),
            ("rho.signed_frobenius_strong_v2", &[], &koblitz),
            ("bsgs.textbook_v2", &[], &prime),
            ("bsgs.interleaved_v2", &[], &prime),
            ("bsgs.negation_v2", &[], &prime),
            ("kangaroo.vow_v2", &[], &prime),
            ("claw.pair_table_v2", claw, &koblitz),
        ];
        let mut covered: Vec<&str> = cases.iter().map(|c| c.0).collect();
        for (id, params, (inst, facts)) in cases {
            let rep = run(&method(id, params), inst, facts, 4242, 6);
            assert_eq!(rep.recovered, Some(4242 % facts.r), "{id}");
            assert_phases_sum(id, &rep);
        }
        // The two frozen known defects, pinned: phases exceed the total.
        // Their `_v2` ids sum.
        for (spec, id, v2) in [
            (j0(), "rho.cubic", "rho.cubic_v2"),
            (j1728(), "rho.quartic", "rho.quartic_v2"),
        ] {
            let (inst, facts) = built(&spec);
            let k = 4242 % facts.r;
            let rep = run(&method(id, &[]), &inst, &facts, k, 6);
            let sum: f64 = rep.phases.iter().map(|p| p.gae).sum();
            assert!(sum > rep.total_gae, "{id} is documented as undercharged");
            let rep = run(&method(v2, &[]), &inst, &facts, k, 6);
            assert_eq!(rep.recovered, Some(k), "{v2}");
            assert_phases_sum(v2, &rep);
            covered.extend([id, v2]);
        }
        // Covered elsewhere (own tests, native backends, other curve shapes).
        covered.extend([
            "ic.k4_gaudry",           // curves over F_{p^4}: ecbench::fp4 tests
            "ic.cm_folded_native_v1", // native backend binary
            "ic.cm_folded_recurrence_v1",
            "ic.cm_folded_native_d43_v1",
            "ic.pipeline_full", // reprices ic.pipeline: full_price tests
            "rho.signed_frobenius_full",
            "rho.signed_frobenius_strong_full",
            "ic.large_prime", // large_prime_v2_charges_… test, both ids
            "ic.large_prime_v2",
            "ic.shared_rank", // shared_rank_tests
        ]);
        for d in registry() {
            assert!(
                covered.contains(&d.id),
                "`{}` has no phases-sum case and no stated reason",
                d.id
            );
        }
    }
}
