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
use crate::cryptanalysis::ecbench::ext::ExtInstance;
use crate::cryptanalysis::ecbench::fp4::{self as fp4, Fp4Group, Fp4Instance, K4Base, K4Params};
use crate::cryptanalysis::ecbench::full_price;
use crate::cryptanalysis::ecbench::generic::{
    bsgs_interleaved, bsgs_negation, bsgs_negation_table, bsgs_textbook, kangaroo, kangaroo_with,
    verify_candidate, GenericOutcome,
};
use crate::cryptanalysis::ecbench::hyper::HyperInstance;
use crate::cryptanalysis::ecbench::hyper_ic::{self, HyperIcParams};
use crate::cryptanalysis::ecbench::koblitz_batched::{self, BatchedParams as KoblitzBatchedParams};
use crate::cryptanalysis::ecbench::koblitz_descent::{
    self as koblitz_descent, DescentParams as KoblitzDescentParams,
};
use crate::cryptanalysis::ecbench::negation_strong::NegStrongParams;
use crate::cryptanalysis::ecbench::wordram_price;
use crate::cryptanalysis::ecbench::workload::{CurveFacts, Instance, WideInstance};
use crate::cryptanalysis::ecbench_large_prime::{
    self as large_prime, SolveConfig as LargePrimeConfig,
};
use crate::cryptanalysis::endo_rho::EndomorphismClasses;
use crate::cryptanalysis::gaudry_quartic::Pt4;
use crate::cryptanalysis::gaudry_quartic_bin::{
    fq_muls_per_add, BinCurve, BinD4Oracle, BinOptions, PtB, TLineBase,
};
use crate::cryptanalysis::gaudry_quartic_d4::{
    fp_muls_per_add, unpack_e4, D4Curve, D4Options, D4Oracle, YLineBase,
};
use crate::cryptanalysis::glv_invariant_base::Endomorphism;
use crate::cryptanalysis::group_op_count::OpCounts;
use crate::cryptanalysis::ic_boundary::{
    price_phase, rho_batch_with, rho_cap, rho_reference, rho_walk_with, rho_walk_with_policy,
    signed_frobenius_rho_tuned, signed_frobenius_rho_tuned_policy, BinaryGroup, BinaryInstance,
    Calibration, CountedGroup, CubicClasses, FieldOps, GroupOps, NegationClasses, PhaseCost,
    PointClasses, PreprocessingRho, PrimeInstance, PrimePoint, QuarticClasses, RhoBatchResult,
    RhoResult, RhoWalk, WalkPolicy,
};
use crate::cryptanalysis::ic_framework::cm_orbit::CmOrbitBase;
use crate::cryptanalysis::ic_framework::fiber::{
    FiberDescentOracle, FiberFrobeniusMitmOracle, FiberMitmOracle,
};
use crate::cryptanalysis::ic_framework::plugins::{
    BinarySubspaceBase, CompactOrbitScanBase, CyclotomicBase, DescentAlgebraicOracle,
    FrobeniusMitmOracle, GlvOrbitBase, KSumOracle, KoblitzOrbitBase, MitmOracle, OnbWeightBase,
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
use crate::cryptanalysis::ic_framework::{run_pipeline_with_base, PipelineSpec, RunReport};
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase, Snapshot};
use crate::cryptanalysis::koblitz_fast::FastPoint;
use crate::cryptanalysis::koblitz_strong_rho::{
    RawPoint as StrongPoint, RawPointG, RhoField, RhoScalar, StrongRho, StrongRhoCharges,
    StrongRhoExtras, StrongRhoG, StrongRhoParams,
};
use num_traits::ToPrimitive;
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
    /// Runs on [`Instance::Prime`] only (a single-word prime-field curve).
    /// The planner refuses any other instance before a session starts, and
    /// [`solve`] refuses it again as a backstop.
    PrimeOnly,
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

const THREE_LEADS_PENCIL_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "small_denominator",
        default: None,
        help: "fixed symbolic small-base denominator: 2, 4, 8 or 16; no adaptive selection",
    },
    ParamDecl {
        name: "matrix_coefficient_cap",
        default: None,
        help:
            "working and retained coefficient cap, 1000000..25000000; pencil has exactly 16 walks",
    },
];
const THREE_LEADS_RHO_PARAMS: &[ParamDecl] = &[ParamDecl {
    name: "trial_cap",
    default: None,
    help: "deterministic maximum collision-search iterations; exhaustion is retained",
}];
const THREE_LEADS_GENUS2_PARAMS:&[ParamDecl]=&[
    ParamDecl{name:"trial_cap",default:None,help:"deterministic maximum processed divisors"},
    ParamDecl{name:"independent",default:None,help:"0: Jacobian walk; 1: independent source scalar combinations followed by tau"},
    ParamDecl{name:"unique_relations",default:None,help:"1 discards repeated divisors up to sign and charges walk restarts; 0 is the duplicate-permitting diagnostic"},
];
const THREE_LEADS_DECOMPOSITION_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "trial_cap",
        default: None,
        help: "maximum relation trials",
    },
    ParamDecl {
        name: "oracle",
        default: None,
        help: "linearized (binary subfield only), semaev (odd short models) or subtract",
    },
    ParamDecl {
        name: "random_base",
        default: None,
        help: "0 structured base; 1 same-size random sign-orbit control",
    },
    ParamDecl {
        name: "base_seed",
        default: None,
        help: "fixed factor-base seed independent of target and algorithm seed",
    },
];

const IC_PIPELINE_PARAMS: &[ParamDecl] = &[
        ParamDecl {
            name: "factor_base",
            default: None,
            help: "plug-in spec: explicit-x:xs=X0;X1;... (exact x list, prime or binary), prime-abscissa:size=N, glv-orbit:size=N, prime-kummer:t=T[,g=G], prime-isogeny:t=T[,seed=S] (auxiliary-curve search charged to the factor-base phase), binary-subspace:dimension=D, koblitz-orbit:divisor=1;2, compact-orbit-scan:columns=N,raw_x_cap=M, cyclotomic:w=W[,columns=N], koblitz-standard-subspace:dimension=D",
        },
        ParamDecl {
            name: "oracle",
            default: None,
            help: "subtract, mitm, mitm-frobenius, mitm-frobenius-counted, mitm-frobenius-lazy-counted, mitm-frobenius-layout-counted (compact-orbit-scan bases) or descent-algebraic, with :m=2|3; mitm-prime-batch:m=2 (prime folded table with batched inversions); mitm-prime-blocked:m=2 (the same table inserted in cache-sized pieces); rootfind-subtract:m=2|3 (prime curves: prime_fast's direct-subtraction root finding, one batched inversion a target); ksum:m=3|4|5 (one probe a target into a negation-folded table of every signed m-sum of the base); pkm-semaev:m=2|3,mode=chain|full,engine=f4|exhaustive on prime-kummer:t=T (prime curves: Semaev S_(m+1) on the base's Kummer tower, F4 over the tower ring, multiply-adds unpriced); the cofactor-fiber-aware mitm-fiber:fiber=closed|lifts|blind, mitm-frobenius-fiber:fiber=closed|lifts|blind and descent-algebraic-fiber:fiber=combined|lifts|blind (ic_framework::fiber); or pdp3-koblitz:m=3,engine=inherited-f4|f6-ic,degree=D,node_budget=N, pdp3-koblitz:m=2|3,engine=rootfind (PairSumTable), pdp3-koblitz:m=3,engine=rootfind-window (cyclic ⌈|F|/3⌉ window), pdp3-koblitz:m=2|3,engine=rootfind-negfold (PairTable negation fold, ~half set-up), and pdp3-koblitz:m=3,engine=rootfind-window-negfold on koblitz-standard-subspace",
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

/// `rho.negation_strong`: the walk's shape changes the search, so neither
/// has a default.
const NEG_STRONG_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "lanes",
        default: None,
        help: "walks stepped in lockstep: the batch-inversion width (1 to 4096)",
    },
    ParamDecl {
        name: "dp_bits",
        default: None,
        help: "distinguished-point bits (1 to 39), or `auto`: round(log2(√(πr/4))/2) from 1 to 20",
    },
    ParamDecl {
        name: "step_cap_factor",
        default: Some("64"),
        help: "give up after this multiple of √(πr/4) walk steps (at least 10⁶)",
    },
];

const RHO_PARAMS: &[ParamDecl] = &[ParamDecl {
    name: "cap_multiple",
    default: Some("64"),
    help: "step budget as a multiple of √r before the run counts as exhausted",
}];

const POOLED_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "cap_multiple",
        default: Some("64"),
        help: "step budget per target as a multiple of √r before that target counts as unsolved",
    },
    ParamDecl {
        name: "batch",
        default: None,
        help: "L, the number of targets pooled into one run: the record's target plus L − 1 public companions",
    },
];

/// Domain of the companion targets of `rho.multitarget_pooled`: the
/// public-target law of [`Instance::hash_to_subgroup`] at indices
/// `POOLED_COMPANION_INDEX + j`, far from any workload's own index.
const POOLED_COMPANION_INDEX: u64 = 1 << 40;

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

const HYPER_IC_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "fb_size",
        default: None,
        help: "factor-base cap: `all` (every degree-1 place, m ≈ p/2 columns after the ± fold) or N (the first N places by x, Gaudry's reduced base)",
    },
    ParamDecl {
        name: "large_primes",
        default: None,
        help: "0 (full relations only) or 1 (single large primes: keep a candidate with one place outside a truncated base and combine pairs sharing it)",
    },
    ParamDecl {
        name: "extra_relations",
        default: None,
        help: "rows collected beyond the m + 1 unknowns before the first solve (the thread uses 8); each further round adds m/4 + 8",
    },
    ParamDecl {
        name: "branches",
        default: None,
        help: "factor-base places the walk steps by, drawn by a seeded shuffle (the thread uses 64; capped at m)",
    },
    ParamDecl {
        name: "d_step_in",
        default: None,
        help: "one step in d_step_in adds D₁, one adds D₂, the rest add a place (the thread uses 4; at least 2)",
    },
    ParamDecl {
        name: "max_trials",
        default: Some("200000"),
        help: "relation trials before the run counts as exhausted",
    },
];

/// Every method `ecbench` knows.  Adding one is an entry here and an arm
/// in [`solve`]; nothing else changes.
const BEST2_MODEL_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "jumps",
        default: None,
        help: "jump table size, 1..256",
    },
    ParamDecl {
        name: "walks",
        default: None,
        help: "parallel model lanes, 1..64",
    },
    ParamDecl {
        name: "dp_bits",
        default: None,
        help: "hashed distinguished-point bits, 0..24",
    },
    ParamDecl {
        name: "cap_multiple",
        default: None,
        help: "physical-step budget / sqrt(r)",
    },
];

pub fn registry() -> &'static [MethodDecl] {
    &[
        MethodDecl {
            id: super::aps::ID,
            family: "ic",
            summary: "bounded APS summation-polynomial instrument: exclusive cold/online wall accounting, retained failures, separate affine witness checks; native operation pricing is incomplete",
            entry: "ecbench::aps::solve",
            applies: Applies::PrimeOnly,
            params: super::aps::PARAMS,
        },
        MethodDecl{id:"ic.three_leads_genus2_pencil_symbolic_v2",family:"ic",summary:"Point-driven pencil with signed symbolic elimination; cold setup per target; cost remains explicitly incomplete",entry:"three_leads::pencil::solve",applies:Applies::Any,params:THREE_LEADS_PENCIL_PARAMS},
        MethodDecl{id:"ic.three_leads_genus2_choice2_v1",family:"ic",summary:"Two-choice component-directed genus2 relation walk; both candidate additions, roots and valid relation insertions charged",entry:"three_leads_followup::solve_genus2_choice2",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"ic.three_leads_genus2_projective_chart_v1",family:"ic",summary:"Two-choice genus2 collector with charged projective CRT numerator chart, one normalization and complete exceptional Cantor fallback",entry:"three_leads_followup::solve_genus2_projective",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"ic.three_leads_genus2_target_rank_v1",family:"ic",summary:"Projective two-choice genus2 collector choosing continuation by charged signed target-coefficient rank preview; both valid relations inserted",entry:"three_leads_followup::solve_genus2_target_rank",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"rho.three_leads_plain_v1",family:"rho",summary:"Fully counted memoized r-adding rho on frozen finite-extension study inputs; collision-table memory reported",entry:"three_leads::solve_rho",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"rho.three_leads_negation_v1",family:"rho",summary:"Fully counted memoized negation rho with fruitless-two-cycle look-ahead and charged escape/restarts",entry:"three_leads::solve_rho",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"rho.three_leads_jacobian_v1",family:"rho",summary:"Explicit Scholten transfer followed by the matched generic Jacobian negation rho, every primitive counted",entry:"three_leads::solve_rho",applies:Applies::Any,params:THREE_LEADS_RHO_PARAMS},
        MethodDecl{id:"ic.three_leads_genus2_v2",family:"ic",summary:"Signed double-large-prime IC with charged restart on every repeated walk divisor, including unsplit states",entry:"three_leads::solve_genus2_v2",applies:Applies::Any,params:THREE_LEADS_GENUS2_PARAMS},
        MethodDecl{id:"ic.three_leads_genus2_v1",family:"ic",summary:"Complete Scholten transfer and signed double-large-prime elimination; independent and duplicate-relation controls are explicit parameters",entry:"three_leads::solve_genus2",applies:Applies::Any,params:THREE_LEADS_GENUS2_PARAMS},
        MethodDecl{id:"ic.three_leads_decomposition_v2",family:"ic",summary:"Complete subfield or PKM IC including initial and restarted relation targets, charged cofactor offsets, actual roots and elimination",entry:"three_leads::solve_subfield_or_pkm_v2",applies:Applies::Any,params:THREE_LEADS_DECOMPOSITION_PARAMS},
        MethodDecl{id:"ic.three_leads_decomposition_v1",family:"ic",summary:"Complete subfield or PKM IC with actual S3 roots, unknown projected base logarithms and incremental elimination",entry:"three_leads::solve_subfield_or_pkm",applies:Applies::Any,params:THREE_LEADS_DECOMPOSITION_PARAMS},
        MethodDecl {
            id: "rho.plus_model_v1",
            family: "rho",
            summary: "counted seeded selection-walk control; hashed predicates, not the CUDA kernel",
            entry: "ecbench::best2_model::solve(best2=false)",
            applies: Applies::PrimeOnly,
            params: BEST2_MODEL_PARAMS,
        },
        MethodDecl {
            id: "rho.best2_model_v1",
            family: "rho",
            summary: "counted P+M/P-M model with two full additions, cycle seeks and DP restarts; not GPU cost",
            entry: "ecbench::best2_model::solve(best2=true)",
            applies: Applies::PrimeOnly,
            params: BEST2_MODEL_PARAMS,
        },
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
            id: "rho.negation_strong",
            family: "rho",
            summary: "strong negation-map rho on {P, −P} (A = 2) over F_(2^m)^4 (f2m4_torsion only): 32-jump additive walk, fruitless-cycle escape by doubling, distinguished points, `lanes` walks with one Montgomery-batched F_(q^4) inversion a step; every F_q multiplication counted (inversions, starts, escapes, checks) and each canonicalisation charged one, GAE = F_q multiplications / fq_muls_per_add (ic.pipeline's divisor on that curve); expected S ≈ √(π/4)·(F_q muls a step / fq_muls_per_add)",
            entry: "ecbench::negation_strong::NegStrongRho::solve",
            applies: Applies::Any,
            params: NEG_STRONG_PARAMS,
        },
        MethodDecl {
            id: "rho.multitarget_pooled",
            family: "rho",
            summary: "Kuhn–Struik batch: the record's target and L − 1 public companions (hashed to ⟨G⟩, logs unknown to anyone) solved in sequence on {P, −P} (A = 2) with one shared distinguished-point table and jumps in G only; S is the whole batch's operations over √r, expected ≈ Σ_{i<L} C(2i,i)4^{−i}·√(π/4) → √L",
            entry: "ic_boundary::rho_batch_with(NegationClasses, RhoWalk::negation())",
            applies: Applies::PrimeOnly,
            params: POOLED_PARAMS,
        },
        MethodDecl {
            id: "rho.preprocessing",
            family: "rho",
            summary: "Bernstein–Lange preprocessing rho on {P, −P} (A = 2): a target-blind table of `entries` distinguished points from walks in G only, then the target's walks on the same map until one ends on a stored point; S counts the table, the precomputation and the online walk",
            entry: "ic_boundary::PreprocessingRho::{build, solve}(NegationClasses, RhoWalk::negation())",
            applies: Applies::PrimeOnly,
            params: &[
                ParamDecl { name: "entries", default: None, help: "S, distinguished points kept in the target-blind table" },
                ParamDecl { name: "dp_bits", default: Some("0"), help: "distinguished-point bits; 0 takes round(log2 √(r/(2S)))" },
                ParamDecl { name: "table_seed", default: Some("20261028"), help: "seed of the jumps and the precomputation starts (target-independent)" },
                ParamDecl { name: "precompute_cap", default: Some("0"), help: "bound on precomputation walk operations; 0 takes 64·√(r·S) + 2^20" },
                ParamDecl { name: "cap_multiple", default: Some("64"), help: "online step budget as a multiple of √r before the run counts as exhausted" },
            ],
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
            id: "rho.frobenius_orbit",
            family: "rho",
            summary: "tuned walk on {±π^i(P)} (A = 2k), the p-power Frobenius of a curve defined over F_p acting on its subgroup over F_{p^k} (subfield_fp3: k = 3, A = 6); Wiener–Zuccherato look-ahead and cycle escape; each canonicalisation applies π k − 1 times, counted as endomorphism_applications_uncharged; refuses any other curve",
            entry: "endo_rho::EndomorphismClasses(FrobeniusType) with RhoWalk::negation()",
            applies: Applies::Any,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.gls",
            family: "rho",
            summary: "tuned walk on {ψ^i(P)} (A = 4), the GLS endomorphism ψ = τ∘π∘τ⁻¹ with ψ² = −1 on the twist over F_{p²} of a curve over F_p (gls_fp2); Wiener–Zuccherato look-ahead and cycle escape; each canonicalisation applies ψ three times, counted as endomorphism_applications_uncharged; refuses any other curve",
            entry: "endo_rho::EndomorphismClasses(GlsEndomorphism) with RhoWalk::negation()",
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
            id: "bsgs.negation_table",
            family: "bsgs",
            summary: "bsgs.negation with the baby table's half-width m given (m + 1 classes, giant stride 2m + 1): the equal-memory comparator for a known-log base",
            entry: "ecbench::generic::bsgs_negation_table",
            applies: Applies::Any,
            params: &[ParamDecl {
                name: "baby",
                default: Some("0"),
                help: "baby table half-width m (table entries m + 1); 0 means ceil(sqrt(r)/2), which is bsgs.negation",
            }],
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
            id: "transfer.isogeny",
            family: "rho",
            summary: "move the instance across a rational ℓ-isogeny (ℓ = 2 or 3, Vélu), solve it on the isogenous curve with a generic inner method, check [k]G = Q on the source; kernel search and the two point maps recorded as unpriced field work; prime-field curves only",
            entry: "ecbench::isogeny::{find_isogeny, Isogeny::map}",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "ell",
                    default: Some("3"),
                    help: "isogeny degree, 2 or 3",
                },
                ParamDecl {
                    name: "inner",
                    default: Some("rho.negation"),
                    help: "the generic method run on the isogenous curve: rho.plain, rho.negation, bsgs.textbook, bsgs.interleaved, bsgs.negation or kangaroo.vow",
                },
                ParamDecl {
                    name: "cap_multiple",
                    default: Some("64"),
                    help: "the inner rho or kangaroo's step cap, in multiples of √r",
                },
            ],
        },
        MethodDecl {
            id: "transfer.quartic_cover",
            family: "rho",
            summary: "pull the instance back along the double cover C : V⁴ = x³ + ax + b → E into the Jacobian of a smooth plane quartic (non-hyperelliptic genus 3), solve it there with a generic inner method, check [k]G = Q on E; Jacobian work counted in F_p multiplications and charged at c_E, the multiplications of one affine addition on E measured in the run; prime-field curves, 29 ≤ p < 2^31",
            entry: "ecbench::cover_quartic::{QuarticCover, transfer}",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "inner",
                    default: Some("rho.plain"),
                    help: "the generic method run in J(C): rho.plain, bsgs.textbook, bsgs.interleaved or kangaroo.vow (negation is not free on the quartic, so rho.negation and bsgs.negation are refused)",
                },
                ParamDecl {
                    name: "cap_multiple",
                    default: Some("64"),
                    help: "the inner rho or kangaroo's step cap, in multiples of √r",
                },
            ],
        },
        MethodDecl {
            id: "transfer.quartic_cover_v2",
            family: "rho",
            summary: "transfer.quartic_cover with its accounting completed (frozen id `transfer.quartic_cover` keeps its counts): the F_p multiplications of the square roots and f(x) evaluations that search for the cover's rational points and the base point B run outside the plane-quartic counter and are tallied as `root_search_field_muls_uncharged`, listed unpriced, so the record is a lower bound; otherwise pull the instance back along the double cover C : V⁴ = x³ + ax + b → E into the Jacobian of a smooth plane quartic (non-hyperelliptic genus 3), solve it there with a generic inner method, check [k]G = Q on E; Jacobian work counted in F_p multiplications and charged at c_E, the multiplications of one affine addition on E measured in the run; prime-field curves, 29 ≤ p < 2^31",
            entry: "ecbench::cover_quartic::{QuarticCover, transfer}",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "inner",
                    default: Some("rho.plain"),
                    help: "the generic method run in J(C): rho.plain, bsgs.textbook, bsgs.interleaved or kangaroo.vow (negation is not free on the quartic, so rho.negation and bsgs.negation are refused)",
                },
                ParamDecl {
                    name: "cap_multiple",
                    default: Some("64"),
                    help: "the inner rho or kangaroo's step cap, in multiples of √r",
                },
            ],
        },
        MethodDecl {
            id: "transfer.ghs_cover",
            family: "rho",
            summary: "carry the instance by the GHS conorm–norm map Φ : E(F_{q³}) → Jac_H(F_q), q = p², into the Jacobian of the genus-3 hyperelliptic cover (Joux–Vitse curves), solve it there with a generic inner method, check [k]G = Q on E; cover, transfers and solve counted in F_p multiplications and charged at c_E, one affine addition on E measured in the run; ghs_fp6 curves only",
            entry: "ecbench::ghs_cover::transfer (jv_cover::Cover::transfer)",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "inner",
                    default: Some("rho.negation"),
                    help: "the generic method run on Jac_H(F_q): rho.plain, rho.negation, bsgs.textbook, bsgs.interleaved, bsgs.negation or kangaroo.vow",
                },
                ParamDecl {
                    name: "cap_multiple",
                    default: Some("64"),
                    help: "the inner rho or kangaroo's step cap, in multiples of √r",
                },
            ],
        },
        MethodDecl {
            id: "ic.hyperelliptic",
            family: "ic",
            summary: "Adleman–DeMarrais–Huang / Gaudry index calculus on a genus-2 Jacobian (genus2 workloads): factor base of degree-1 places, factor-base walk, smoothness by gcd, sparse elimination mod r; Jacobian group operations charged, field and row operations unpriced",
            entry: "hyperelliptic_index_calculus::hec_index_calculus_dlp",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "fb_size",
                    default: Some("0"),
                    help: "cap on the factor base (degree-1 places up to ±); 0 keeps every place",
                },
                ParamDecl {
                    name: "extra_relations",
                    default: Some("8"),
                    help: "relations collected beyond the factor-base size",
                },
                ParamDecl {
                    name: "max_trials",
                    default: Some("2000000"),
                    help: "relation-search trials before giving up",
                },
                ParamDecl {
                    name: "large_primes",
                    default: Some("0"),
                    help: "1 keeps one-large-prime partials and combines pairs",
                },
            ],
        },
        MethodDecl {
            id: "ic.hyperelliptic_v2",
            family: "ic",
            summary: "ic.hyperelliptic with its accounting completed (frozen id `ic.hyperelliptic` keeps its counts): the verification [k]D₁ = D₂ is charged as Jacobian operations (phase `verification`, in the total; the frozen id ran it twice, uncharged) and the factor-base scan's F_p multiplications are tallied (`factor_base_fp_muls_uncharged`, phase `factor_base`, listed unpriced); otherwise Adleman–DeMarrais–Huang / Gaudry index calculus on a genus-2 Jacobian (genus2 workloads): factor base of degree-1 places, factor-base walk, smoothness by gcd, sparse elimination mod r; Jacobian group operations charged, field and row operations unpriced",
            entry: "hyperelliptic_index_calculus::hec_index_calculus_dlp + ecbench::hyper_ic::factor_base_scan_muls",
            applies: Applies::Any,
            params: &[
                ParamDecl {
                    name: "fb_size",
                    default: Some("0"),
                    help: "cap on the factor base (degree-1 places up to ±); 0 keeps every place",
                },
                ParamDecl {
                    name: "extra_relations",
                    default: Some("8"),
                    help: "relations collected beyond the factor-base size",
                },
                ParamDecl {
                    name: "max_trials",
                    default: Some("2000000"),
                    help: "relation-search trials before giving up",
                },
                ParamDecl {
                    name: "large_primes",
                    default: Some("0"),
                    help: "1 keeps one-large-prime partials and combines pairs",
                },
            ],
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
            id: "ic.cm_folded_native_v2",
            family: "ic",
            summary: "CM scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained; _v2 of the frozen `_v1` id: the replay's counted operations are charged (total_gae, S) and the unpriced native counters are named",
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
            id: "ic.cm_folded_recurrence_v2",
            family: "ic",
            summary: "Generic quadratic-recurrence scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained; _v2 of the frozen `_v1` id: the replay's counted operations are charged (total_gae, S) and the unpriced native counters are named",
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
            id: "ic.cm_folded_native_d43_v2",
            family: "ic",
            summary: "D=-43 degree-11 CM scalar-orbit signed MITM2, native backend with independent Rust point/relation replay; unpriced native work retained; _v2 of the frozen `_v1` id: the replay's counted operations are charged (total_gae, S) and the unpriced native counters are named",
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
            id: "ic.hyperelliptic_thread",
            family: "ic",
            summary: "the hyperelliptic thread's index calculus (ADH/Gaudry) on a genus 2-4 Jacobian (hyperelliptic_thread / hyperelliptic_explicit only): degree-1 places folded by ±, the factor-base walk (start D₁ + D₂, then one Cantor addition a trial), u split by gcd(u, x^p − x) and Cantor–Zassenhaus, Markowitz sparse elimination mod r, up to 4 solve rounds; expected ≈ (m + 1 + extra)·g! trials, so S ≈ (p/2)·g!/√r with the full base, plus uncharged F_p work O(g² log p) a trial and the elimination's mulmods mod r",
            entry: "ecbench::hyper_ic::solve",
            applies: Applies::Any,
            params: HYPER_IC_PARAMS,
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
            id: "ic.pipeline_wordram",
            family: "ic",
            summary: "ic.pipeline unchanged, then every native counter priced by ecbench::wordram_price: fixed 256-bit word-RAM budgets over the reference affine addition (CurveSmash's accounting), host-independent; unknown counters stay unpriced",
            entry: "ic_framework::run_pipeline + ecbench::wordram_price::reprice",
            applies: Applies::BinaryOnly,
            params: IC_PIPELINE_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_wordram",
            family: "rho",
            summary: "rho.signed_frobenius unchanged, with its canonicalisations and distinguished-point probes priced by ecbench::wordram_price's fixed word-RAM budgets",
            entry: "ic_boundary::signed_frobenius_rho_tuned + ecbench::wordram_price::reprice",
            applies: Applies::KoblitzOnly,
            params: RHO_PARAMS,
        },
        MethodDecl {
            id: "rho.signed_frobenius_strong_wordram",
            family: "rho",
            summary: "rho.signed_frobenius_strong unchanged, with its canonicalisations, partition hashes and table probes priced by ecbench::wordram_price's fixed word-RAM budgets",
            entry: "koblitz_strong_rho::StrongRho::solve + ecbench::wordram_price::reprice",
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
            id: "ic.tensor_contraction_v1",
            family: "ic",
            summary: "toy index calculus with m keyed SHA-256 hashed factor bases (|F_i| ≈ (2r)^(1/m)), each relation found by exact sparse tensor contraction of the whole-point network (crates/ecdl-tensor-ic); exponential, about r point additions per relation; contraction rows, factor-base listing and elimination counted unpriced",
            entry: "ecdl_tensor_ic::Solver with SparseContractor",
            applies: Applies::PrimeOnly,
            params: &[
                ParamDecl { name: "m", default: None, help: "hashed factor bases, at least 2; the relation network has (2m−1)(2b+1) edges" },
                ParamDecl { name: "max_restarts", default: Some("8"), help: "factor-base redraws before the run counts as exhausted" },
            ],
        },
        MethodDecl {
            id: "ic.known_log_mitm2_v1",
            family: "ic",
            summary: "target-blind seeded known-log signed base, folded two-sum table, injective Q+[a]H queries and immediate scalar recovery; cold construction charged, native work unpriced",
            entry: "ecbench::known_log::{prepare,recover}",
            applies: Applies::Any,
            params: &[
                ParamDecl { name: "size", default: None, help: "unsigned base pairs; signed point count is twice this" },
                ParamDecl { name: "base_seed", default: None, help: "target-independent base seed" },
                ParamDecl { name: "base_multiplier", default: Some("1"), help: "H=[u]G; use c^-1 when transporting a base to generator [c]G" },
                ParamDecl { name: "max_pair_work", default: None, help: "hard bound on size*(size+1) pair-construction attempts" },
                ParamDecl { name: "max_trials", default: None, help: "maximum target-dependent queries; at most one subgroup cycle" },
            ],
        },
        MethodDecl {
            id: "ic.known_log_ksum_v1",
            family: "ic",
            summary: "ic.known_log_mitm2_v1 with a negation-folded table of every signed k-sum (k = 3, 4, 5) of the seeded known-log base in place of the pair table; a hit's log is the sum of k known logs",
            entry: "ecbench::known_log::{prepare_ksum,recover}",
            applies: Applies::PrimeOnly,
            params: &[
                ParamDecl { name: "size", default: None, help: "unsigned base points; signed point count is twice this (at most 2047)" },
                ParamDecl { name: "k", default: None, help: "summands, 3, 4 or 5" },
                ParamDecl { name: "base_seed", default: None, help: "target-independent base seed" },
                ParamDecl { name: "base_multiplier", default: Some("1"), help: "H=[u]G; use c^-1 when transporting a base to generator [c]G" },
                ParamDecl { name: "max_sums", default: None, help: "hard bound on the table's 2^(k-1)·C(size,k) sums, checked before any work" },
                ParamDecl { name: "max_trials", default: None, help: "maximum target-dependent queries; at most one subgroup cycle" },
            ],
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
            id: "ic.trace_line_2lp",
            family: "ic",
            summary: "bounded exact four-summand binary 2LP correctness reference on standard or trace-selected subfield lines; MITM work and native setup are explicit, unpriced; not the algebraic C1 trial-cost measurement",
            entry: "ecbench_large_prime::trace_line + solve_candidate_with_basis",
            applies: Applies::BinaryOnly,
            params: &[
                ParamDecl { name: "line", default: None, help: "standard or trace; n must equal 4d" },
                ParamDecl { name: "d", default: None, help: "subfield dimension, positive with n=4d<=62" },
                ParamDecl { name: "small_dimension", default: None, help: "nested small-base dimension <= d, same policy in both arms" },
                ParamDecl { name: "max_u", default: None, help: "inclusive deterministic multiplier scan limit; no fallback" },
                ParamDecl { name: "max_trials", default: None, help: "relation trial cap" },
                ParamDecl { name: "max_states", default: None, help: "exact MITM state cap" },
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
        MethodDecl {
            id: "ic.koblitz_descent",
            family: "ic",
            summary: "ledger §23 one-target Koblitz index calculus as `ic price --single-target` runs it: subgroup-orbit base, signed-Frobenius pair table, aimed m = 3 windowed collection, sparse logs, walked m = 2 descent; every stage charged by counted point additions and doublings (group_op_count), native work unpriced; S grows with the base and the descent, not as a fixed constant",
            entry: "ecbench::koblitz_descent::run",
            applies: Applies::KoblitzOnly,
            params: KOBLITZ_DESCENT_PARAMS,
        },
        MethodDecl {
            id: "ic.koblitz_batched",
            family: "ic",
            summary: "the candidate tournament worker's pair-table pipeline (examples/ic_tournament_worker.rs): subgroup-orbit base, signed-Frobenius pair table, relation collection in batch_trials queries with a rank check after each batch, sparse logs, one-target descent; every stage charged by counted point additions and doublings (group_op_count), native work unpriced; exists so the tournament's batch_trials candidates can be measured on an isolated host with L2, replay and A/A",
            entry: "ecbench::koblitz_batched::run",
            applies: Applies::KoblitzOnly,
            params: KOBLITZ_BATCHED_PARAMS,
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

/// `ic.koblitz_descent`'s parameters: the rule v8 parameter files'
/// fields.  `rule_v8` takes the file's value at the six §23 sizes
/// (k0n41, k0n53, k0n57, k0n61, k1n47, k1n59) and is refused at any
/// other, where the value must be given.
const KOBLITZ_DESCENT_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "summands",
        default: Some("3"),
        help: "collection summands; the §23 recipe's aimed window needs 3",
    },
    ParamDecl {
        name: "descent_summands",
        default: Some("rule_v8"),
        help: "summands the walked descent asks for (2..4; rule v8: 2, 3 at k1n47)",
    },
    ParamDecl {
        name: "collection_aim",
        default: Some("true"),
        help: "aim each unit's window at the columns still uncovered (ColumnCoverage)",
    },
    ParamDecl {
        name: "collection_window",
        default: Some("rule_v8"),
        help: "factor-base summands each m = 3 probe scans",
    },
    ParamDecl {
        name: "factor_base_points",
        default: Some("rule_v8"),
        help: "points of the subgroup-orbit factor base",
    },
    ParamDecl {
        name: "unit_trials",
        default: Some("rule_v8"),
        help: "walked probes per collection work unit",
    },
    ParamDecl {
        name: "units",
        default: Some("rule_v8"),
        help: "units collected before the first solve",
    },
    ParamDecl {
        name: "max_units",
        default: Some("100000"),
        help: "units the extension may reach before the run fails",
    },
    ParamDecl {
        name: "max_trials",
        default: Some("rule_v8"),
        help: "descent probe cap before an exhausted result",
    },
    ParamDecl {
        name: "seed",
        default: Some("rule_v8"),
        help: "seed of the factor base, the probes and the descent walks (`run`: the run's algorithm seed)",
    },
];

/// `ic.koblitz_batched`'s parameters: the fields of an `icprog tournament`
/// job that the pair-table pipeline reads.  The defaults are the worker's
/// (`batch_trials` 64, `max_trials` 4096, `summands` 3) and the tournament's
/// registered factor-base seed (43); the factor-base size is per cell.
const KOBLITZ_BATCHED_PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "factor_base_points",
        default: None,
        help: "points of the subgroup-orbit factor base (the tournament's cells name it: 78 on K_0/GF(2^13))",
    },
    ParamDecl {
        name: "fb_seed",
        default: Some("43"),
        help: "seed of the factor base (the tournament registers 43)",
    },
    ParamDecl {
        name: "batch_trials",
        default: Some("64"),
        help: "queries collected before each rank check (the tournament's incumbent: 64; its batch16 candidate: 16)",
    },
    ParamDecl {
        name: "max_trials",
        default: Some("4096"),
        help: "cap on the queries drawn in all (at most 65536, at least batch_trials)",
    },
    ParamDecl {
        name: "summands",
        default: Some("3"),
        help: "collection summands m (2..4)",
    },
    ParamDecl {
        name: "seed",
        default: Some("run"),
        help: "seed of the probes and the descent (`run`: the run's algorithm seed)",
    },
];

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
        "rho.negation" | "rho.negation_v2" | "rho.negation_strong" => Some((PI / 4.0).sqrt()),
        "rho.cubic" | "rho.cubic_v2" => Some((PI / 12.0).sqrt()),
        "rho.quartic" | "rho.quartic_v2" | "rho.gls" => Some((PI / 8.0).sqrt()),
        "rho.frobenius_orbit" => Some((PI / 12.0).sqrt()),
        "rho.signed_frobenius"
        | "rho.signed_frobenius_v2"
        | "rho.signed_frobenius_full"
        | "rho.signed_frobenius_strong"
        | "rho.signed_frobenius_strong_v2"
        | "rho.signed_frobenius_strong_full"
        | "rho.signed_frobenius_wordram"
        | "rho.signed_frobenius_strong_wordram"
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

// Keep the historical genus-2 configuration identity distinct from the
// explicitly named thread implementation, including when refusing workloads.
const HYPER_GENUS2_ID: &str = "ic.hyperelliptic";
const HYPER_THREAD_ENTRY: &str = "ecbench::hyper_ic::solve";
const HYPER_GENUS2_ENTRY: &str = "hyperelliptic_index_calculus::hec_index_calculus_dlp";
/// `ic.hyperelliptic` with the verification charged and the factor-base
/// scan tallied; the frozen id above keeps its exact counts.
const HYPER_GENUS2_V2_ID: &str = "ic.hyperelliptic_v2";
const HYPER_GENUS2_V2_ENTRY: &str = "hyperelliptic_index_calculus::hec_index_calculus_dlp + ecbench::hyper_ic::factor_base_scan_muls";

/// Required parameters that hold an integer (a parameter with a default is
/// integer-valued when its default is a canonical integer).  `summands`
/// also takes `n-1`, which the lexeme rule below leaves alone.
const INTEGER_PARAMS_WITHOUT_DEFAULT: &[&str] = &[
    "subset_size",
    "attempts",
    "degree",
    "budget_ms",
    "sampling_draws",
    "beta",
    "lambda",
    "large_primes",
    "max_trials",
    "target_max_attempts",
    "small_dimension",
    "envelope_dimension",
    "summands",
    "rank_seed",
    "rank_max_trials",
    "max_states",
    "extra_relations",
    "branches",
    "d_step_in",
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
    // The genus-2 and thread Jacobian implementations retain one historical
    // method name, but have different parameter contracts. Choose the thread
    // declaration when its distinguishing parameters are present.
    let thread_hyper = spec.id == "ic.hyperelliptic"
        && (spec.params.contains_key("branches")
            || spec.params.contains_key("d_step_in")
            || spec.params.get("fb_size").is_some_and(|s| s == "all"));
    let d = (if thread_hyper {
        registry()
            .iter()
            .find(|m| m.id == spec.id && m.entry == "ecbench::hyper_ic::solve")
    } else {
        decl(&spec.id)
    })
    .ok_or_else(|| {
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
    /// Work counted for `bitop/v1` alone (`ecbench::work`): counters added
    /// after sessions were committed, kept apart from `native` so a replay
    /// of a committed record still compares exactly what it recorded.
    /// Written only when nonempty; the audit compares it only when the
    /// committed record carries it.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub work_native: BTreeMap<String, u64>,
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
        work_native: BTreeMap::new(),
    }
}

pub(super) fn unpriced_of(counters: &BTreeMap<String, u64>) -> Vec<String> {
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
    // A kangaroo re-placement is search.  `detail` says how much of the
    // search it was (informational; `detail` is not compared on replay).
    let detail = if out.restarts == GroupOps::default() {
        Value::Null
    } else {
        json!({"restarts_in_search": {"adds": out.restarts.adds,
            "doubles": out.restarts.doubles, "scalar_mults": out.restarts.scalar_mults}})
    };
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
        detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops,
    }
}

/// Split a tuned rho run into set-up, walk and verification from its own
/// counters; the frozen walk has none and stays one phase.
/// Charge a class set-up performed on its own ledger (the scalar
/// multiplication that fixes ι or ζ₃ on the subgroup) to a finished walk:
/// the ledger, the total `gae` and `S` move together, so the phases
/// `rho_report` derives from the ledger sum to the total it reports.
fn charge_class_setup(res: &mut RhoResult, setup: GroupOps, r: u64) {
    res.group_ops.merge(setup);
    res.gae = res.group_ops.gae();
    res.s = res.gae / (r as f64).sqrt();
}

/// The class set-up, carried in the report's `detail` so that `verify`
/// can reproduce a record written before it was charged to the total
/// (`audit::legacy_class_setup`).  `detail` is not compared on replay.
fn class_setup_detail(setup: GroupOps) -> Value {
    json!({"class_setup": {"adds": setup.adds, "doubles": setup.doubles,
        "scalar_mults": setup.scalar_mults, "gae": setup.gae()}})
}

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
/// [`row_flags`] flags a share above [`SETUP_SHARE_FLAG`], and `ecbench
/// table` prints it in its `flags` column.
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

/// [`row_flags`] for a table row pooling several runs: each phase's GAE and
/// the totals are summed over `runs` (phases, `total_gae`) before the share
/// is taken, so the share is that of the row's summed `S`.
pub fn pooled_row_flags(
    curve_family: &str,
    log2_r: f64,
    method_family: &str,
    runs: &[(&[PhaseRecord], f64)],
) -> Vec<String> {
    let mut pooled: Vec<PhaseRecord> = Vec::new();
    for (phases, _) in runs {
        for p in phases.iter() {
            match pooled.iter_mut().find(|q| q.name == p.name) {
                Some(q) => q.gae += p.gae,
                None => pooled.push(PhaseRecord {
                    name: p.name.clone(),
                    gae: p.gae,
                    ..Default::default()
                }),
            }
        }
    }
    let total: f64 = runs.iter().map(|(_, t)| t).sum();
    row_flags(curve_family, log2_r, method_family, &pooled, total)
}

/// A tuned rho run reported under the `_v2` accounting: the same phases as
/// [`rho_report`] plus `automorphism_setup`, the `[κ]G` ledger of
/// `CubicClasses::new` / `QuarticClasses::new`.  The frozen ids charge that
/// ledger too (since 2026-10-18, through [`charge_class_setup`], folded into
/// their walk's phases); the `_v2` ids give it a phase of its own, beside the
/// `_v2` walk.  Here the total is the sum of the phases.
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

/// The phases and counters of a [`rho_batch_with`] run: the shared jump
/// table and every target's `G + Q_i` and stride as `setup`, the internal
/// `[d]G = Q_i` checks as `internal_verification`, the rest as `search`.
fn pooled_report(res: RhoBatchResult, wall: u64, field_ops: Option<FieldOps>) -> SolveReport {
    let c = &res.counters;
    let get = |k: &str| c.get(k).copied().unwrap_or(0);
    let setup_adds = get("setup_additions") + get("target_setup_additions");
    let setup_dbls = get("setup_doublings") + get("target_setup_doublings");
    let verify = get("verification_operations");
    let total = res.group_ops.adds + res.group_ops.doubles;
    let search = total.saturating_sub(setup_adds + setup_dbls + verify);
    let mk = |name: &str, gae: u64| PhaseRecord {
        name: name.into(),
        gae: gae as f64,
        ..Default::default()
    };
    let mut s = mk("setup", setup_adds + setup_dbls);
    s.adds = setup_adds;
    s.doubles = setup_dbls;
    let mut w = mk("search", search);
    w.scalar_mults = res.group_ops.scalar_mults;
    let phases = vec![s, w, mk("internal_verification", verify)];
    let mut counters = res.counters.clone();
    let steps: u64 = res.per_target.iter().map(|t| t.steps).sum();
    let walks: u64 = res.per_target.iter().map(|t| t.walks).sum();
    let dps: u64 = res.per_target.iter().map(|t| t.distinguished_points).sum();
    counters.insert("steps".into(), steps);
    counters.insert("walks".into(), walks);
    counters.insert("distinguished_points".into(), dps);
    let last = res.per_target.last();
    let recovered = last.and_then(|t| t.recovered);
    let per_target: Vec<Value> = res
        .per_target
        .iter()
        .map(|t| {
            json!({
                "adds": t.group_ops.adds,
                "doubles": t.group_ops.doubles,
                "steps": t.steps,
                "walks": t.walks,
                "distinguished_points": t.distinguished_points,
                "solved_by": t.solved_by,
                "recovered": t.recovered,
            })
        })
        .collect();
    SolveReport {
        recovered: recovered.map(u128::from),
        exhausted: recovered.is_none(),
        total_gae: res.gae,
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
            "targets": res.targets,
            "record_target_index": res.targets.saturating_sub(1),
            "all_verified_internally": res.all_verified,
            "expected_single": res.expected_single,
            "per_target": per_target,
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

/// `rho.multitarget_pooled` on a prime curve: `L − 1` public companion
/// targets, then the record's target last, through one [`rho_batch_with`]
/// run on `{P, −P}`.  The companions are hashed to `⟨G⟩` by the workload's
/// public-target law, so nobody knows their logarithms; each one the batch
/// recovers is checked inside the run (`[d]G = Q_j`, charged as
/// `internal_verification`) and listed in `detail.per_target` with its
/// point in `detail.companions`.  The record's target is verified by the
/// runner as every other method's is.
fn solve_pooled(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    i: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let batch = param_u64(m, "batch")?;
    if !(1..=65_536).contains(&batch) {
        return Err(format!("`batch` must be in 1..=65536, not {batch}"));
    }
    let cap = rho_cap(i.r, param_u64(m, "cap_multiple")? as f64);
    let mut targets = Vec::with_capacity(batch as usize);
    let mut companions = Vec::with_capacity(batch as usize - 1);
    for j in 0..batch - 1 {
        let (xy, _) = inst
            .hash_to_subgroup(curve, seed, POOLED_COMPANION_INDEX + j)
            .ok_or("no public companion target on this curve")?;
        targets.push(PrimePoint::affine(parse_hex(&xy[0])?, parse_hex(&xy[1])?));
        companions.push(xy);
    }
    targets.push(target);
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let res = rho_batch_with(
        &i.curve,
        &NegationClasses { r: i.r },
        i.generator_point(),
        &targets,
        i.r,
        seed,
        cap,
        RhoWalk::negation(),
    );
    measurement::end_online();
    let wall = t.elapsed().as_nanos() as u64;
    let f = field_ops_of(&i.curve, &[res.group_ops]);
    let mut rep = pooled_report(res, wall, f);
    rep.detail["companions"] = json!(companions);
    Ok(rep)
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
        "rho.plus_model_v1" | "rho.best2_model_v1" => {
            let jumps = param_u64(m, "jumps")?;
            let walks = param_u64(m, "walks")?;
            let dp_bits = param_u64(m, "dp_bits")?;
            if !(1..=256).contains(&jumps) || !(1..=64).contains(&walks) || dp_bits > 24 {
                return Err("model requires jumps=1..256, walks=1..64, dp_bits=0..24".into());
            }
            let o = super::best2_model::solve(
                g,
                gen,
                target,
                r,
                seed,
                cap()?,
                jumps as usize,
                walks as usize,
                dp_bits as u32,
                m.id == "rho.best2_model_v1",
            );
            generic(o, t.elapsed().as_nanos() as u64, 2)
        }
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
        "bsgs.negation_table" => {
            let baby = param_u64(m, "baby")?;
            let o = if baby == 0 {
                bsgs_negation(g, gen, target, r)
            } else {
                bsgs_negation_table(g, gen, target, r, baby)
            };
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

/// Whether `m` can run on `inst` at all, decided when the spec is planned
/// with the refusals `solve` would raise on every execution: a wide
/// Koblitz curve past one word, which only the strong rhos and
/// `claw.pair_table` implement; the CM folded controls and
/// `ic.k4_gaudry`, each bound to one construction; and `rho.quartic` /
/// `rho.cubic`, which need the j = 1728 / j = 0 structure on the subgroup;
/// and every [`Applies::PrimeOnly`] method (`rho.multitarget_pooled`) on a
/// non-prime instance. An arm that cannot run is a spec error, not a session of `error`
/// records that `compare` then reports as incomplete.
pub fn applicable(m: &ResolvedMethod, inst: &Instance, curve: &CurveFacts) -> Result<(), String> {
    let id = full_price::base_of(&m.id)
        .or(wordram_price::base_of(&m.id))
        .unwrap_or(m.id.as_str());
    hyper_ic_variant_guard(m, inst, curve)?;
    if id == "rho.multitarget_pooled" {
        let batch = param_u64(m, "batch")?;
        if !(1..=65_536).contains(&batch) {
            return Err(format!("`batch` must be in 1..=65536, not {batch}"));
        }
    }
    if id.starts_with("ic.cm_folded_") && !matches!(inst, Instance::Prime(_)) {
        return Err("CM folded controls require a prime-field instance".into());
    }
    if decl(id).is_some_and(|d| d.applies == Applies::PrimeOnly)
        && !matches!(inst, Instance::Prime(_))
    {
        return Err(prime_only_refusal(id, curve));
    }
    if id == "ic.k4_gaudry" && !matches!(inst, Instance::Fp4(_)) {
        return Err(format!(
            "`ic.k4_gaudry` runs on fp4_translation curves only; {} is {}",
            curve.slug, curve.family
        ));
    }
    if id == NEG_STRONG {
        if !matches!(inst, Instance::F2m4(_)) {
            return Err(neg_strong_refusal(curve));
        }
        neg_strong_params(m, 2)?;
        return Ok(());
    }
    match inst {
        Instance::Prime(i) if id == super::aps::ID => super::aps::applicable(m, i),
        Instance::Genus2(_) | Instance::Quartic(_) | Instance::Ghs(_) => {
            jacobian_applicable(id, m.params.get("inner").map(String::as_str), inst, curve)
        }
        Instance::Hyper(h) => {
            if is_hyper_generic(m.id.as_str()) {
                Ok(())
            } else if m.id == HYPER_IC {
                hyper_ic_params(m)?;
                hyper_ic_cyclic(h)
            } else {
                Err(hyper_refusal(m.id.as_str(), h))
            }
        }
        _ if id == HYPER_IC => Err(hyper_ic_refusal(curve)),
        Instance::Ext(e) => ext_applicable(id, e, curve),
        _ if id == "rho.gls" || id == "rho.frobenius_orbit" => Err(format!(
            "`{id}` runs on subfield_fp3 / gls_fp2 curves only; {} is {}",
            curve.slug, curve.family
        )),
        Instance::Wide(wi) => match id {
            "rho.signed_frobenius_strong"
            | "rho.signed_frobenius_strong_legacy"
            | "rho.signed_frobenius_strong_escape"
            | "claw.pair_table" => Ok(()),
            other => Err(format!(
                "`{other}` has no wide-field implementation; on a curve past one word (n = {}) only the strong rho (with or without the escape) and claw.pair_table run",
                wi.kc.n
            )),
        },
        Instance::Prime(i) if id.starts_with("ic.cm_folded_") => {
            super::cm_folded::applicable(m, i)
        }
        Instance::Prime(i) if id == "rho.quartic" => {
            let mut scratch = GroupOps::default();
            QuarticClasses::new(&i.curve, i.generator_point(), i.r, &mut scratch)
                .map(|_| ())
                .ok_or_else(|| format!(
                    "`rho.quartic` needs a j = 1728 curve over p ≡ 1 (mod 4) with 4 | r − 1 and a nontrivial ι on the subgroup; {} lacks the structure",
                    curve.slug
                ))
        }
        Instance::Prime(i) if id == "rho.cubic" => {
            let mut scratch = GroupOps::default();
            CubicClasses::new(&i.curve, i.generator_point(), i.r, &mut scratch)
                .map(|_| ())
                .ok_or_else(|| format!(
                    "`rho.cubic` needs a j = 0 curve over p ≡ 1 (mod 3) with 3 | r − 1 and a nontrivial ζ₃ on the subgroup; {} lacks the structure",
                    curve.slug
                ))
        }
        Instance::Prime(i) if id == "transfer.isogeny" => {
            let inner = m.params.get("inner").map_or("rho.negation", String::as_str);
            if !JACOBIAN_GENERIC.contains(&inner) || inner == "rho.frozen_reference" {
                return Err(format!("`inner` must be a generic method, not `{inner}`"));
            }
            let ell: u64 = m.params.get("ell").and_then(|v| v.parse().ok()).unwrap_or(3);
            if !matches!(ell, 2 | 3) {
                return Err(format!("`ell` must be 2 or 3, not {ell}"));
            }
            if ell == i.r {
                return Err("ℓ equals the subgroup order; the isogeny would not preserve it".into());
            }
            let mut t = crate::cryptanalysis::ecbench::isogeny::FieldTally::default();
            crate::cryptanalysis::ecbench::isogeny::find_isogeny(&i.curve, ell, &mut t)
                .map(|_| ())
                .ok_or_else(|| format!("{} has no rational {ell}-isogeny", curve.slug))
        }
        Instance::Prime(i) if is_quartic_cover(id) => {
            quartic_cover_applicable(m, i).map(|_| ())
        }
        _ if id == "transfer.ghs_cover" => Err(format!(
            "`transfer.ghs_cover` runs on ghs_fp6 curves only; {} is {}",
            curve.slug, curve.family
        )),
        _ if is_quartic_cover(id) => Err(format!(
            "`{id}` runs on prime-field curves only; {} is {}",
            curve.slug, curve.family
        )),
        _ if id == "transfer.isogeny" => Err(format!(
            "`transfer.isogeny` runs on prime-field curves only; {} is {}",
            curve.slug, curve.family
        )),
        _ if id == "rho.quartic" || id == "rho.cubic" => Err(format!(
            "`{id}` runs on prime-field curves only; {} is {}",
            curve.slug, curve.family
        )),
        Instance::Binary(i) if id == "ic.koblitz_descent" => koblitz_descent_applicable(m, i),
        _ if id == "ic.koblitz_descent" => Err(format!(
            "`ic.koblitz_descent` runs on one-word Koblitz curves only; {} is {}",
            curve.slug, curve.family
        )),
        Instance::Binary(i) if id == "ic.koblitz_batched" => koblitz_batched_applicable(m, i),
        _ if id == "ic.koblitz_batched" => Err(format!(
            "`ic.koblitz_batched` runs on one-word Koblitz curves only; {} is {}",
            curve.slug, curve.family
        )),
        _ => Ok(()),
    }
}

/// What `solve_ext` runs on a GLS or subfield curve: the generic methods,
/// and the endomorphism-orbit rho of the instance's own family.  Anything
/// else is the refusal `solve_ext` raises on every execution.
fn ext_applicable(id: &str, inst: &ExtInstance, curve: &CurveFacts) -> Result<(), String> {
    let generic = id.starts_with("rho.plain")
        || id == "rho.negation"
        || id == "rho.frozen_reference"
        || id.starts_with("bsgs.")
        || id == "kangaroo.vow";
    match (inst, id) {
        _ if generic => Ok(()),
        (ExtInstance::Gls(_), "rho.gls") | (ExtInstance::Subfield(_), "rho.frobenius_orbit") => {
            Ok(())
        }
        _ if id == "rho.gls" || id == "rho.frobenius_orbit" => Err(format!(
            "`{id}` runs on {} curves only; {} is {}",
            if id == "rho.gls" {
                "gls_fp2"
            } else {
                "subfield_fp3"
            },
            curve.slug,
            curve.family
        )),
        (ExtInstance::Gls(_), _) => Err(format!("`{id}` has no gls_fp2 implementation")),
        (ExtInstance::Subfield(_), _) => Err(format!("`{id}` has no subfield_fp3 implementation")),
    }
}

/// `ic.koblitz_descent` needs a Koblitz instance of odd degree below 64
/// (its order fits a word by construction), and every `rule_v8` value
/// to exist at that size.
fn koblitz_descent_applicable(m: &ResolvedMethod, i: &BinaryInstance) -> Result<(), String> {
    let kc = i
        .koblitz
        .as_ref()
        .ok_or("`ic.koblitz_descent` needs a Koblitz instance")?;
    if kc.n.is_multiple_of(2) || kc.n >= 64 || kc.subgroup_order.bits() > 64 {
        return Err(format!(
            "`ic.koblitz_descent` needs odd n < 64 and a subgroup order of at most 64 bits; n = {}",
            kc.n
        ));
    }
    KoblitzDescentParams::from_method(&m.params, kc.n, kc.a, 0).map(|_| ())
}

/// `ic.koblitz_batched` needs a Koblitz instance of odd degree below 64
/// whose subgroup order fits a word, and valid collection limits.
fn koblitz_batched_applicable(m: &ResolvedMethod, i: &BinaryInstance) -> Result<(), String> {
    let kc = i
        .koblitz
        .as_ref()
        .ok_or("`ic.koblitz_batched` needs a Koblitz instance")?;
    if kc.n.is_multiple_of(2) || kc.n >= 64 || kc.subgroup_order.bits() > 64 {
        return Err(format!(
            "`ic.koblitz_batched` needs odd n < 64 and a subgroup order of at most 64 bits; n = {}",
            kc.n
        ));
    }
    KoblitzBatchedParams::from_method(&m.params, 0).map(|_| ())
}

/// The refusal of a [`Applies::PrimeOnly`] method on any other instance.
fn prime_only_refusal(id: &str, curve: &CurveFacts) -> String {
    format!(
        "`{id}` runs on single-word prime-field curves only; {} is {}",
        curve.slug, curve.family
    )
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
    hyper_ic_variant_guard(m, inst, curve)?;
    let d = decl(&m.id).ok_or("unregistered method")?;
    if m.id.contains(".three_leads_") && !matches!(inst, Instance::ThreeLeads(_)) {
        return Err("three-leads methods require a frozen three_leads curve fixture".into());
    }
    if m.id.starts_with("ic.cm_folded_") && !matches!(inst, Instance::Prime(_)) {
        return Err("CM folded controls require a prime-field instance".into());
    }
    // A `*_full` or `*_wordram` method runs its base unchanged and is
    // repriced after.
    let full = full_price::base_of(&m.id);
    let wordram = wordram_price::base_of(&m.id);
    let base_m;
    let (m, run_d) = match full.or(wordram) {
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
    if d.applies == Applies::PrimeOnly && !matches!(inst, Instance::Prime(_)) {
        return Err(prime_only_refusal(&m.id, curve));
    }
    // The phase clock: the methods open and close the online window, the
    // session turns it into exclusive phase times.
    let session = measurement::Session::begin().ok();
    let mut rep = if let Instance::Wide(wi) = inst {
        let (tx, ty) = (parse_hex_wide(&target[0])?, parse_hex_wide(&target[1])?);
        solve_wide(m, wi, tx, ty, seed)?
    } else if let Instance::F2m4(i) = inst {
        let (tx, ty) = (parse_hex_wide(&target[0])?, parse_hex_wide(&target[1])?);
        if u64::try_from(i.r).is_err() {
            // `m ≥ 16` with a small cofactor: the subgroup order needs two
            // words; the u128 implementations (`ic_wide`, `rho_wide`).
            solve_f2m4_wide(m, i, curve, tx, ty, seed)?
        } else {
            solve_inner(m, inst, curve, d, tx as u64, ty as u64, seed)?
        }
    } else if matches!(
        inst,
        Instance::Genus2(_) | Instance::Quartic(_) | Instance::Ghs(_)
    ) {
        solve_jacobian(m, inst, curve, target, seed)?
    } else {
        let (tx, ty) = (parse_hex(&target[0])?, parse_hex(&target[1])?);
        solve_inner(m, inst, curve, run_d, tx, ty, seed)?
    };
    if full.is_some() {
        full_price::reprice(&mut rep, d.family, &curve.slug)?;
    }
    if wordram.is_some() {
        wordram_price::reprice(&mut rep, d.family, curve)?;
    }
    match session.map(|s| s.finish()) {
        Some(Ok(snap)) if rep.online.is_none() => {
            rep.online = online_window(d.family, &m.id, &snap)
        }
        Some(Ok(_)) => {} // A method with its own five-phase online clock supplied the window.
        Some(Err(e)) => rep.online_error = Some(e.to_string()),
        None => rep.online_error = Some("a measurement session was already active".into()),
    }
    if m.id == "ic.three_leads_genus2_pencil_symbolic_v2" {
        if let Some(window) = &mut rep.online {
            let sum = window
                .phases_ns
                .values()
                .try_fold(0u64, |total, ns| total.checked_add(*ns));
            if sum != Some(window.wall_ns) {
                return Err("pencil online phase sum differs from window".into());
            }
            window.start_event = "source target validity check after target-blind curve, base and empty solver setup".into();
            window.stop_event = if rep.recovered.is_some() {
                "first source multiplication check completed inside pencil core"
            } else {
                "pencil coefficient cap or walk exhaustion; no verified solution"
            }
            .into();
            window.mapping = "query: target validation, labels, source query and transfer, plus intervening control bookkeeping; PDP: point-driven sieve; relation check: polynomial, point and Jacobian checks; descent: signed graph and residual elimination; recovery check: first source multiplication. Additional adapter verification is outside online and charged in cold group counts.".into();
            if let Some(reason) = window.zero_phases.get_mut("target_relation_check") {
                *reason = "no sieve candidate reached the separate polynomial, point and Jacobian verification stage".into();
            }
        }
    } else if m.id.starts_with("rho.three_leads_") && rep.recovered.is_none() {
        if let Some(window) = &mut rep.online {
            window.stop_event = "rho cap exhausted; no verified solution".into();
        }
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
    if let Instance::ThreeLeads(i) = inst {
        return super::three_leads::solve(
            m,
            i,
            crate::cryptanalysis::three_leads::Point::affine(tx, ty),
            seed,
        );
    }
    if let Instance::Wide(_) = inst {
        return Err("a wide instance goes through solve_wide".into());
    }
    if let Instance::Fp4(i) = inst {
        return solve_fp4(m, i, tx, ty, seed);
    }
    if let Instance::Ext(i) = inst {
        return solve_ext(m, i, tx, ty, seed);
    }
    if let Instance::Hyper(i) = inst {
        return solve_hyper(m, i, curve, tx, ty, seed);
    }
    if m.id == HYPER_IC {
        return Err(hyper_ic_refusal(curve));
    }
    if m.id == "rho.frobenius_orbit" || m.id == "rho.gls" {
        return Err(format!(
            "`{}` runs on subfield_fp3 / gls_fp2 curves only; {} is {}",
            m.id, curve.slug, curve.family
        ));
    }
    if m.id == "ic.k4_gaudry" {
        return Err(format!(
            "`ic.k4_gaudry` runs on fp4_translation curves only; {} is {}",
            curve.slug, curve.family
        ));
    }
    if m.id == NEG_STRONG {
        let Instance::F2m4(i) = inst else {
            return Err(neg_strong_refusal(curve));
        };
        let f = &i.curve.f;
        let target = PtB {
            x: f.unpack(u128::from(tx)),
            y: f.unpack(u128::from(ty)),
            inf: false,
        };
        return solve_neg_strong(m, i, target, seed);
    }
    match (inst, d.family) {
        (Instance::Prime(i), _) if m.id == super::aps::ID => {
            super::aps::solve(m, i, PrimePoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), _) if m.id == "ic.tensor_contraction_v1" => {
            solve_tensor_contraction(m, i, PrimePoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), _)
            if m.id == "ic.known_log_mitm2_v1" || m.id == "ic.known_log_ksum_v1" =>
        {
            solve_known_log(
                m,
                &i.curve,
                i.generator_point(),
                PrimePoint::affine(tx, ty),
                i.r,
                curve,
                seed,
            )
        }
        (Instance::Prime(i), _) if m.id == "rho.preprocessing" => solve_preprocessing(
            m,
            &i.curve,
            i.generator_point(),
            PrimePoint::affine(tx, ty),
            i.r,
            seed,
        ),
        (Instance::Binary(i), _) if m.id == "ic.known_log_mitm2_v1" => solve_known_log(
            m,
            &BinaryGroup(&i.fast),
            i.generator,
            FastPoint::affine(tx, ty),
            i.r,
            curve,
            seed,
        ),
        (Instance::Binary(i), _)
            if m.id == "ic.large_prime"
                || m.id == "ic.large_prime_v2"
                || m.id == "ic.trace_line_2lp" =>
        {
            solve_large_prime(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _) if m.id == "ic.shared_rank" => {
            solve_shared_rank(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _) if m.id == "ic.koblitz_descent" => {
            solve_koblitz_descent(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Binary(i), _) if m.id == "ic.koblitz_batched" => {
            solve_koblitz_batched(m, i, curve, FastPoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), _)
            if matches!(
                m.id.as_str(),
                "ic.cm_folded_native_v1"
                    | "ic.cm_folded_native_d43_v1"
                    | "ic.cm_folded_recurrence_v1"
                    | "ic.cm_folded_native_v2"
                    | "ic.cm_folded_native_d43_v2"
                    | "ic.cm_folded_recurrence_v2"
            ) =>
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
                // The [κ]G check of the class set-up is part of the cost,
                // reported as its own `automorphism_setup` phase.
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
            charge_class_setup(&mut res, setup, i.r);
            measurement::end_online();
            let f = FieldOps::total(&[res.group_ops]);
            let mut rep = rho_report(res, t.elapsed().as_nanos() as u64, Some(f));
            rep.detail = class_setup_detail(setup);
            Ok(rep)
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
            charge_class_setup(&mut res, setup, i.r);
            measurement::end_online();
            let f = FieldOps::total(&[res.group_ops]);
            let mut rep = rho_report(res, t.elapsed().as_nanos() as u64, Some(f));
            rep.detail = class_setup_detail(setup);
            Ok(rep)
        }
        (Instance::Prime(i), _) if m.id == "transfer.isogeny" => {
            solve_isogeny_transfer(m, i, PrimePoint::affine(tx, ty), seed)
        }
        (Instance::Prime(i), _) if is_quartic_cover(&m.id) => {
            solve_quartic_cover(m, i, PrimePoint::affine(tx, ty), seed)
        }
        (_, _) if is_quartic_cover(&m.id) => Err(format!(
            "`{}` runs on prime-field curves only; {} is {}",
            m.id, curve.slug, curve.family
        )),
        (_, _) if m.id == "transfer.isogeny" => Err(format!(
            "`transfer.isogeny` runs on prime-field curves only; {} is {}",
            curve.slug, curve.family
        )),
        (Instance::Prime(i), _) if m.id == "rho.multitarget_pooled" => {
            solve_pooled(m, inst, curve, i, PrimePoint::affine(tx, ty), seed)
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
                x: f.unpack(u128::from(tx)),
                y: f.unpack(u128::from(ty)),
                inf: false,
            };
            solve_generic(m, &i.curve, i.generator, target, one_word(i.r)?, seed)
        }
        (Instance::Wide(_), _)
        | (Instance::Fp4(_), _)
        | (Instance::ThreeLeads(_), _)
        | (Instance::Ext(_), _)
        | (Instance::Genus2(_), _)
        | (Instance::Quartic(_), _)
        | (Instance::Ghs(_), _)
        | (Instance::Hyper(_), _) => {
            unreachable!("returned above")
        }
    }
}

/// The generic methods every Jacobian family runs, and those that need a
/// free negation, which a plane-quartic Jacobian does not have (its
/// negation costs about one group operation that the counted interface
/// cannot charge).
const JACOBIAN_GENERIC: &[&str] = &[
    "rho.frozen_reference",
    "rho.plain",
    "rho.negation",
    "bsgs.textbook",
    "bsgs.interleaved",
    "bsgs.negation",
    "kangaroo.vow",
];
const NEEDS_FREE_NEGATION: &[&str] = &["rho.negation", "bsgs.negation"];

/// Which methods run on a Jacobian (`ecbench::jacobian`,
/// `ecbench::plane_quartic`), and why the others do not.
fn jacobian_applicable(
    id: &str,
    m_inner: Option<&str>,
    inst: &Instance,
    curve: &CurveFacts,
) -> Result<(), String> {
    let quartic = matches!(inst, Instance::Quartic(_));
    if quartic && NEEDS_FREE_NEGATION.contains(&id) {
        return Err(format!(
            "`{id}` needs a free negation; on a plane-quartic Jacobian ({}) negation costs about one group operation, so run rho.plain or bsgs.textbook",
            curve.slug
        ));
    }
    if JACOBIAN_GENERIC.contains(&id) {
        return Ok(());
    }
    if (id == HYPER_GENUS2_ID || id == HYPER_GENUS2_V2_ID) && matches!(inst, Instance::Genus2(_)) {
        return Ok(());
    }
    if id == "transfer.ghs_cover" && matches!(inst, Instance::Ghs(_)) {
        let inner = m_inner.unwrap_or("rho.negation");
        if !JACOBIAN_GENERIC.contains(&inner) || inner == "rho.frozen_reference" {
            return Err(format!("`inner` must be a generic method, not `{inner}`"));
        }
        return Ok(());
    }
    Err(format!(
        "`{id}` has no implementation on {} ({}); the generic methods {} run there",
        curve.slug,
        curve.family,
        JACOBIAN_GENERIC
            .iter()
            .filter(|m| !(quartic && NEEDS_FREE_NEGATION.contains(m)))
            .copied()
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

/// A Jacobian workload: decode the target class and run the method on
/// the counted Jacobian group.
fn solve_jacobian(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    target: &[String; 2],
    seed: u64,
) -> Result<SolveReport, String> {
    jacobian_applicable(
        &m.id,
        m.params.get("inner").map(String::as_str),
        inst,
        curve,
    )?;
    match inst {
        Instance::Genus2(i) => {
            let t = i.decode(target)?;
            if m.id == HYPER_GENUS2_ID || m.id == HYPER_GENUS2_V2_ID {
                return solve_ic_hyperelliptic(m, i, t, seed);
            }
            solve_generic(m, &i.group(), i.generator, t, i.r, seed)
        }
        Instance::Quartic(i) => {
            let t = crate::cryptanalysis::ecbench::plane_quartic::decode(&i.jac, target)?;
            solve_generic(m, &i.group(), i.generator, t, i.r, seed)
        }
        Instance::Ghs(i) => {
            let t = i.decode(target)?;
            if m.id == "transfer.ghs_cover" {
                return solve_ghs_cover(m, i, t, seed);
            }
            let f = i.field();
            let g = crate::cryptanalysis::ecbench::ghs_cover::GhsCurveGroup(i.curve(&f));
            solve_generic(m, &g, i.generator, t, i.l, seed)
        }
        _ => Err("not a Jacobian".into()),
    }
}

/// `transfer.isogeny`: find the rational ℓ-isogeny, map `G` and `Q`,
/// solve on the isogenous curve with the inner generic method, check the
/// answer on the source.  Phases: `transfer` (0 GAE, its field work
/// native and unpriced), the inner method's phases prefixed `target:`,
/// and `verify` (one scalar multiplication on the source curve).
fn solve_isogeny_transfer(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ecbench::isogeny::{find_isogeny, FieldTally};
    let ell = param_u64(m, "ell")?;
    if !matches!(ell, 2 | 3) {
        return Err(format!("`ell` must be 2 or 3, not {ell}"));
    }
    if ell == inst.r {
        return Err("ℓ equals the subgroup order; the isogeny would not preserve it".into());
    }
    let inner_id = m.params["inner"].clone();
    if !JACOBIAN_GENERIC.contains(&inner_id.as_str()) || inner_id == "rho.frozen_reference" {
        return Err(format!(
            "`inner` must be a generic method, not `{inner_id}`"
        ));
    }
    let t = Instant::now();
    let mut tally = FieldTally::default();
    let phi = find_isogeny(&inst.curve, ell, &mut tally).ok_or(format!(
        "{} has no rational {ell}-isogeny",
        inst.curve_id().slug
    ))?;
    let g1 = phi.map(inst.generator_point(), &mut tally);
    let q1 = phi.map(target, &mut tally);
    if g1.infinity || !phi.target.is_on_curve(g1) || !phi.target.is_on_curve(q1) {
        return Err("the isogeny did not map the instance onto the target curve".into());
    }
    let mut inner_params = BTreeMap::new();
    if inner_id.starts_with("rho.") || inner_id == "kangaroo.vow" {
        inner_params.insert("cap_multiple".to_string(), m.params["cap_multiple"].clone());
    }
    let inner = ResolvedMethod {
        id: inner_id.clone(),
        params: inner_params,
        ..m.clone()
    };
    let mut rep = solve_generic(&inner, &phi.target, g1, q1, inst.r, seed)?;
    // The answer on E′ is the answer on E (φ is injective on ⟨G⟩); the
    // check is what a careful attacker pays to know it.
    let mut verify = GroupOps::default();
    if let Some(k) = rep.recovered {
        let kg = inst.curve.mul(
            &mut verify,
            inst.generator_point(),
            (k % u128::from(inst.r)) as u64,
        );
        if kg != target {
            rep.recovered = None;
        }
    }
    let mut transfer = ops_phase("transfer", GroupOps::default());
    transfer.native.insert("field_muls".into(), tally.muls);
    transfer.native.insert("field_invs".into(), tally.invs);
    let mut phases = vec![transfer];
    for mut ph in rep.phases {
        ph.name = format!("target:{}", ph.name);
        phases.push(ph);
    }
    phases.push(ops_phase("verify", verify));
    rep.total_gae += verify.gae();
    rep.phases = phases;
    rep.counters.insert("isogeny_degree".into(), ell);
    rep.counters
        .insert("transfer_field_muls_uncharged".into(), tally.muls);
    rep.counters
        .insert("transfer_field_invs_uncharged".into(), tally.invs);
    rep.unpriced = unpriced_of(&rep.counters);
    rep.detail = json!({
        "isogeny": {
            "degree": ell,
            "kernel_x": phi.x0,
            "target_curve": {"p": phi.target.p, "a": phi.target.a, "b": phi.target.b},
            "target_slug": PrimeInstance {
                name: String::new(),
                curve: phi.target.clone(),
                group_order: inst.group_order,
                r: inst.r,
                cofactor: inst.cofactor,
                generator: (g1.x, g1.y),
            }.curve_id().slug,
        },
        "inner": inner_id,
        "inner_detail": rep.detail,
    });
    rep.solve_wall_ns = t.elapsed().as_nanos() as u64;
    if let Some(f) = rep.field_ops.as_mut() {
        f.muls += verify.field_ops.muls;
        f.sqrs += verify.field_ops.sqrs;
        f.invs += verify.field_ops.invs;
    }
    Ok(rep)
}

/// `transfer.quartic_cover` and its `_v2` (root-search tally).
fn is_quartic_cover(id: &str) -> bool {
    id == "transfer.quartic_cover" || id == "transfer.quartic_cover_v2"
}

/// The checks `transfer.quartic_cover` makes before it runs: the inner
/// method, the field size and an odd subgroup order, and a certified
/// smooth model of the cover.
fn quartic_cover_applicable(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
) -> Result<crate::cryptanalysis::ecbench::cover_quartic::QuarticCover, String> {
    let inner = m
        .params
        .get("inner")
        .map(String::as_str)
        .unwrap_or("rho.plain");
    if !JACOBIAN_GENERIC.contains(&inner) || inner == "rho.frozen_reference" {
        return Err(format!("`inner` must be a generic method, not `{inner}`"));
    }
    if NEEDS_FREE_NEGATION.contains(&inner) {
        return Err(format!(
            "`{inner}` needs a free negation, which the plane-quartic Jacobian does not have"
        ));
    }
    if inst.r.is_multiple_of(2) {
        return Err("π* is injective on odd-order subgroups only; r is even".into());
    }
    crate::cryptanalysis::ecbench::cover_quartic::QuarticCover::new(&inst.curve)
}

/// `transfer.quartic_cover`: build the double cover `C → E` and its
/// Jacobian, pull `G` and `Q` back, solve in `J(C)` with the inner
/// generic method, check the answer on `E`.  The unit stays one group
/// operation on `E`: Jacobian work is counted in `F_p` multiplications
/// (the plane-quartic module's counter) and divided by `c_E`, measured
/// here with the same counted arithmetic.  Phases: `cover_setup` (the
/// model and its smoothness certificate), `transfer` (two additions on
/// `E` plus the pullbacks), `target:solve` (the inner solve) and
/// `verify` (`[k]G` on `E`).
fn solve_quartic_cover(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ecbench::cover_quartic::{
        affine_add_cost, root_search_field_muls, transfer,
    };
    use crate::cryptanalysis::ecbench::plane_quartic::{field_muls, QuarticGroup};
    let v2 = m.id == "transfer.quartic_cover_v2";
    let t = Instant::now();
    let g = inst.generator_point();
    // c_E first: it prices everything after it.
    let mut calib = GroupOps::default();
    let h = inst.curve.mul(&mut calib, g, 7);
    let c_e = affine_add_cost(&inst.curve, g, h);
    if c_e.is_nan() || c_e <= 0.0 {
        return Err("c_E could not be measured".into());
    }
    let m0 = field_muls();
    let r0 = root_search_field_muls();
    let cover = quartic_cover_applicable(m, inst)?;
    let setup_muls = field_muls() - m0;
    let setup_roots = root_search_field_muls() - r0;
    let m1 = field_muls();
    let r1 = root_search_field_muls();
    let mut e_ops = GroupOps::default();
    let (pg, pq, b) = transfer(&cover, &inst.curve, g, target, &mut e_ops)?;
    let transfer_muls = field_muls() - m1;
    let transfer_roots = root_search_field_muls() - r1;
    let inner_id = m.params["inner"].clone();
    let mut inner_params = BTreeMap::new();
    if inner_id.starts_with("rho.") || inner_id == "kangaroo.vow" {
        inner_params.insert("cap_multiple".to_string(), m.params["cap_multiple"].clone());
    }
    let inner = ResolvedMethod {
        id: inner_id.clone(),
        params: inner_params,
        ..m.clone()
    };
    let grp = QuarticGroup(&cover.jac);
    let m2 = field_muls();
    let mut rep = solve_generic(&inner, &grp, pg, pq, inst.r, seed)?;
    let solve_muls = field_muls() - m2;
    let jac_ops = rep.total_gae;
    let mut verify = GroupOps::default();
    if let Some(k) = rep.recovered {
        let kg = inst
            .curve
            .mul(&mut verify, g, (k % u128::from(inst.r)) as u64);
        if kg != target {
            rep.recovered = None;
        }
    }
    let converted = |muls: u64| muls as f64 / c_e;
    let mut setup = ops_phase("cover_setup", GroupOps::default());
    setup.gae = converted(setup_muls);
    setup.native.insert("field_muls".into(), setup_muls);
    let mut tr = ops_phase("transfer", e_ops);
    tr.gae += converted(transfer_muls);
    tr.native.insert("field_muls".into(), transfer_muls);
    let mut solve = ops_phase("target:solve", GroupOps::default());
    solve.gae = converted(solve_muls);
    solve.native.insert("field_muls".into(), solve_muls);
    solve.native.insert("jacobian_ops".into(), jac_ops as u64);
    if v2 {
        setup
            .native
            .insert("root_search_field_muls_uncharged".into(), setup_roots);
        tr.native
            .insert("root_search_field_muls_uncharged".into(), transfer_roots);
    }
    let verify_phase = ops_phase("verify", verify);
    rep.phases = vec![setup, tr, solve, verify_phase];
    rep.total_gae = rep.phases.iter().map(|p| p.gae).sum();
    rep.automorphisms_used = 1;
    if v2 {
        // The square roots and f(x) evaluations of the point searches, run
        // outside the plane-quartic counter: counted, not priced.
        rep.counters.insert(
            "root_search_field_muls_uncharged".into(),
            setup_roots + transfer_roots,
        );
    }
    rep.counters.insert("cover_model_c".into(), cover.c);
    rep.counters.insert("jacobian_ops".into(), jac_ops as u64);
    rep.counters.insert(
        "jacobian_field_muls".into(),
        setup_muls + transfer_muls + solve_muls,
    );
    rep.unpriced = unpriced_of(&rep.counters);
    rep.field_ops = None;
    rep.detail = json!({
        "quartic_cover": {
            "model": "(X + cY)^4 - Y^3 Z - a Y Z^3 - b Z^4, X = V - c x, Y = x",
            "c": cover.c,
            "c_e_field_muls_per_add": c_e,
            "base_point_b": [format!("{:#x}", b.x), format!("{:#x}", b.y)],
            "field_muls": {"cover_setup": setup_muls, "transfer": transfer_muls, "solve": solve_muls},
            "e_adds_per_jacobian_op": if jac_ops > 0.0 { converted(solve_muls) / jac_ops } else { 0.0 },
        },
        "inner": inner_id,
        "inner_detail": rep.detail,
    });
    rep.solve_wall_ns = t.elapsed().as_nanos() as u64;
    Ok(rep)
}

/// `transfer.ghs_cover`: build the genus-3 cover of a `ghs_fp6` curve,
/// carry `G` and `Q` by `Φ`, solve on `Jac_H(F_q)` with the inner generic
/// method, check the answer on `E`.  The unit stays one affine addition
/// on `E`: the tower's counter of `F_p` multiplications, divided by `c_E`
/// measured here with the same counter.  Phases: `transfer` (the cover
/// and `Φ(G)`, `Φ(Q)`), `target:solve` and `verify` (`[k]G` on `E`).
fn solve_ghs_cover(
    m: &ResolvedMethod,
    inst: &crate::cryptanalysis::ecbench::ghs_cover::GhsInstance,
    target: crate::cryptanalysis::jv_cover::PtE6,
    seed: u64,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::ecbench::ghs_cover::{
        affine_add_cost, transfer, GhsCurveGroup, GhsJacobianGroup,
    };
    use crate::cryptanalysis::jv_cover::Hyp;
    let t = Instant::now();
    let f = inst.field();
    let ec = inst.curve(&f);
    let h = ec.mul(&inst.generator, 7);
    let c_e = affine_add_cost(&f, &ec, &inst.generator, &h);
    if c_e.is_nan() || c_e <= 0.0 {
        return Err("c_E could not be measured".into());
    }
    let m0 = f.muls();
    let (cov, gj, qj) = transfer(&f, inst, &target)?;
    let transfer_muls = f.muls() - m0;
    let inner_id = m.params["inner"].clone();
    let mut inner_params = BTreeMap::new();
    if inner_id.starts_with("rho.") || inner_id == "kangaroo.vow" {
        inner_params.insert("cap_multiple".to_string(), m.params["cap_multiple"].clone());
    }
    let inner = ResolvedMethod {
        id: inner_id.clone(),
        params: inner_params,
        ..m.clone()
    };
    let grp = GhsJacobianGroup(Hyp {
        f: &f.f,
        h: cov.hx.clone(),
        g: 3,
    });
    let m1 = f.muls();
    let mut rep = solve_generic(&inner, &grp, gj, qj, inst.l, seed)?;
    let solve_muls = f.muls() - m1;
    let jac_ops = rep.total_gae;
    let mut verify = GroupOps::default();
    if let Some(k) = rep.recovered {
        let e = GhsCurveGroup(inst.curve(&f));
        let kg = e.mul(&mut verify, inst.generator, (k % u128::from(inst.l)) as u64);
        if kg != target {
            rep.recovered = None;
        }
    }
    let converted = |muls: u64| muls as f64 / c_e;
    let mut tr = ops_phase("transfer", GroupOps::default());
    tr.gae = converted(transfer_muls);
    tr.native.insert("field_muls".into(), transfer_muls);
    let mut solve = ops_phase("target:solve", GroupOps::default());
    solve.gae = converted(solve_muls);
    solve.native.insert("field_muls".into(), solve_muls);
    solve.native.insert("jacobian_ops".into(), jac_ops as u64);
    rep.phases = vec![tr, solve, ops_phase("verify", verify)];
    rep.total_gae = rep.phases.iter().map(|p| p.gae).sum();
    rep.counters.insert("jacobian_ops".into(), jac_ops as u64);
    rep.counters
        .insert("jacobian_field_muls".into(), transfer_muls + solve_muls);
    rep.unpriced = unpriced_of(&rep.counters);
    rep.field_ops = None;
    rep.detail = json!({
        "ghs_cover": {
            "cover": "H : y^2 = F(x) N(x) over F_{p^2}, genus 3 (jv_cover::Cover)",
            "c_e_field_muls_per_add": c_e,
            "field_muls": {"transfer": transfer_muls, "solve": solve_muls},
            "e_adds_per_jacobian_op": if jac_ops > 0.0 { converted(solve_muls) / jac_ops } else { 0.0 },
        },
        "inner": inner_id,
        "inner_detail": rep.detail,
    });
    rep.solve_wall_ns = t.elapsed().as_nanos() as u64;
    Ok(rep)
}

/// `ic.hyperelliptic`: ADH/Gaudry index calculus on a genus-2 Jacobian.
/// The relation search, its step precomputation and the verification are
/// Jacobian group operations (charged, one GAE each, the workload's
/// unit); the smoothness oracle's field multiplications and the
/// elimination's row operations are native work, recorded and unpriced
/// (AGENTS.md §5a prices them across the band).
fn solve_ic_hyperelliptic(
    m: &ResolvedMethod,
    inst: &crate::cryptanalysis::ecbench::jacobian::Genus2Instance,
    target: crate::cryptanalysis::genus2_fp::Divisor,
    seed: u64,
) -> Result<SolveReport, String> {
    use crate::cryptanalysis::hyperelliptic_index_calculus::{
        hec_index_calculus_dlp, HecIndexCalculusParams,
    };
    use crate::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
    use num_bigint::BigUint;
    let p = BigUint::from(inst.p());
    let poly = |c: Vec<u64>| -> FpPoly {
        FpPoly::from_coeffs(c.into_iter().map(BigUint::from).collect(), p.clone())
    };
    let curve = HyperellipticCurveP::try_new(p.clone(), poly(inst.curve.f().to_vec()), 2)
        .map_err(|e| format!("{e:?}"))?;
    let mumford = |d: &crate::cryptanalysis::genus2_fp::Divisor| MumfordDivisorP {
        u: poly(d.u_coeffs()),
        v: poly(d.v_coeffs()),
    };
    let fb = param_u64(m, "fb_size")?;
    let params = HecIndexCalculusParams {
        fb_size: if fb == 0 { usize::MAX } else { fb as usize },
        extra_relations: param_u64(m, "extra_relations")? as usize,
        max_trials: param_u64(m, "max_trials")? as usize,
        seed,
        large_primes: param_u64(m, "large_primes")? != 0,
        ..HecIndexCalculusParams::default()
    };
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let (k, rep) = hec_index_calculus_dlp(
        &curve,
        &mumford(&inst.generator),
        &mumford(&target),
        &BigUint::from(inst.r),
        &params,
    );
    measurement::end_online();
    let wall = t.elapsed().as_nanos() as u64;
    let recovered = k.and_then(|k| u128::try_from(k).ok());
    let v2 = m.id == HYPER_GENUS2_V2_ID;
    let setup = GroupOps {
        adds: rep.precompute_ops as u64,
        ..GroupOps::default()
    };
    let search = GroupOps {
        adds: (rep.jacobian_ops - rep.precompute_ops) as u64,
        ..GroupOps::default()
    };
    let mut relations = ops_phase("relations", search);
    relations.native.insert(
        "smoothness_field_muls".into(),
        rep.smoothness_field_ops as u64,
    );
    let mut la = ops_phase("linear_algebra", GroupOps::default());
    la.native.insert("row_ops".into(), rep.solve_row_ops as u64);
    let mut counters = BTreeMap::new();
    counters.insert("factor_base_size".into(), rep.factor_base_size as u64);
    counters.insert("trials".into(), rep.trials as u64);
    counters.insert("smooth_trials".into(), rep.smooth_trials as u64);
    counters.insert("relations".into(), rep.relations as u64);
    counters.insert("partial_relations".into(), rep.partial_relations as u64);
    counters.insert("combined_relations".into(), rep.combined_relations as u64);
    counters.insert(
        "smoothness_field_muls_uncharged".into(),
        rep.smoothness_field_ops as u64,
    );
    counters.insert("row_ops_uncharged".into(), rep.solve_row_ops as u64);
    let mut phases = vec![ops_phase("setup", setup), relations, la];
    if v2 {
        // The verification [k]D₁ = D₂ the frozen id ran (twice on a
        // success) without charging it: every Cantor doubling and
        // addition it executed, one GAE each.
        let verification = ops_phase(
            "verification",
            GroupOps {
                adds: rep.verification_adds as u64,
                doubles: rep.verification_doubles as u64,
                ..GroupOps::default()
            },
        );
        // The factor-base scan's F_p multiplications, tallied by the
        // thread arm's counted field over the same x's (the frozen id's
        // `build_factor_base` does the work and counts none of it).
        let fb_size = if fb == 0 { usize::MAX } else { fb as usize };
        let (fb_muls, places) =
            hyper_ic::factor_base_scan_muls(inst.p(), &inst.curve.f(), fb_size)?;
        if places != rep.factor_base_size {
            return Err(format!(
                "factor-base tally found {places} places, the run {}",
                rep.factor_base_size
            ));
        }
        let mut fb_phase = ops_phase("factor_base", GroupOps::default());
        fb_phase
            .native
            .insert("factor_base_fp_muls_uncharged".into(), fb_muls);
        counters.insert("factor_base_fp_muls_uncharged".into(), fb_muls);
        counters.insert(
            "verification_jacobian_ops".into(),
            (rep.verification_adds + rep.verification_doubles) as u64,
        );
        phases.insert(0, fb_phase);
        phases.push(verification);
    }
    let total: f64 = phases.iter().map(|p| p.gae).sum();
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        total_gae: total,
        automorphisms_used: 2,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail: json!({
            "smoothness_rate": rep.smoothness_rate(),
            "walk_wall_ns": rep.walk_wall_ns,
            "smoothness_wall_ns": rep.smoothness_wall_ns,
            "solve_wall_ns": rep.solve_wall_ns,
            "partial_wall_ns": rep.partial_wall_ns,
            "discarded_off_base": rep.discarded_off_base,
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

/// The methods that run on a hyperelliptic Jacobian: the generic ones,
/// which see only [`CountedGroup`] (no IC arm, no endomorphism walk).
fn is_hyper_generic(id: &str) -> bool {
    matches!(
        id,
        "rho.plain"
            | "rho.negation"
            | "rho.frozen_reference"
            | "bsgs.textbook"
            | "bsgs.interleaved"
            | "bsgs.negation"
            | "kangaroo.vow"
    )
}

fn hyper_refusal(id: &str, h: &HyperInstance) -> String {
    format!(
        "`{id}` has no hyperelliptic-Jacobian implementation; on a genus-{} Jacobian over F_{} only rho.plain, rho.negation, rho.frozen_reference, bsgs.*, kangaroo.vow and ic.hyperelliptic_thread run",
        h.genus, h.p
    )
}

/// The index calculus that runs on a hyperelliptic Jacobian, and only there.
const HYPER_IC: &str = "ic.hyperelliptic_thread";

fn hyper_ic_variant_guard(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
) -> Result<(), String> {
    let (expected, workload_matches) = match m.id.as_str() {
        HYPER_GENUS2_ID => (HYPER_GENUS2_ENTRY, matches!(inst, Instance::Genus2(_))),
        HYPER_GENUS2_V2_ID => (HYPER_GENUS2_V2_ENTRY, matches!(inst, Instance::Genus2(_))),
        HYPER_IC => (HYPER_THREAD_ENTRY, matches!(inst, Instance::Hyper(_))),
        _ => return Ok(()),
    };
    if !workload_matches || m.entry != expected {
        return Err(format!(
            "`{}` requires its matching Jacobian workload and `{expected}`; {} uses `{}`",
            m.id, curve.family, m.entry
        ));
    }
    Ok(())
}

fn hyper_ic_refusal(curve: &CurveFacts) -> String {
    format!(
        "`{HYPER_IC}` runs on genus2_* or hyperelliptic_* Jacobians only; {} is {}",
        curve.slug, curve.family
    )
}

/// Relations mod `r` need a cyclic `r`-part (see `ecbench::hyper_ic`).
fn hyper_ic_cyclic(h: &HyperInstance) -> Result<(), String> {
    if (h.group_order / h.r).is_multiple_of(h.r) {
        return Err(format!(
            "`{HYPER_IC}` needs r ∥ #J; r² divides #J = {} (r = {})",
            h.group_order, h.r
        ));
    }
    Ok(())
}

fn hyper_ic_params(m: &ResolvedMethod) -> Result<HyperIcParams, String> {
    let fb_size = match m.params["fb_size"].as_str() {
        "all" => usize::MAX,
        _ => match param_u64(m, "fb_size")? {
            0 => return Err("parameter `fb_size` must be `all` or at least 1".into()),
            n => usize::try_from(n).map_err(|_| "fb_size does not fit usize")?,
        },
    };
    let large_primes = match m.params["large_primes"].as_str() {
        "0" => false,
        "1" => true,
        other => return Err(format!("parameter `large_primes` is 0 or 1, not `{other}`")),
    };
    let branches = param_u64(m, "branches")?;
    let d_step_in = param_u64(m, "d_step_in")?;
    if branches == 0 || d_step_in < 2 {
        return Err("`ic.hyperelliptic_thread` needs branches ≥ 1 and d_step_in ≥ 2".into());
    }
    Ok(HyperIcParams {
        fb_size,
        large_primes,
        extra_relations: usize::try_from(param_u64(m, "extra_relations")?)
            .map_err(|_| "extra_relations does not fit usize")?,
        branches: usize::try_from(branches).map_err(|_| "branches does not fit usize")?,
        d_step_in,
        max_trials: param_u64(m, "max_trials")?,
    })
}

/// A hyperelliptic Jacobian (`ecbench::hyper`): a generic method on the
/// Cantor group, one composition-and-reduction per group operation, or
/// the thread's index calculus (`ic.hyperelliptic_thread`).
fn solve_hyper(
    m: &ResolvedMethod,
    i: &HyperInstance,
    curve: &CurveFacts,
    w0: u64,
    w1: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    if m.id == HYPER_IC {
        let target = i.decode(w0, w1)?;
        return solve_hyper_ic(m, i, curve, target, seed);
    }
    if !is_hyper_generic(m.id.as_str()) {
        return Err(hyper_refusal(m.id.as_str(), i));
    }
    let target = i.decode(w0, w1)?;
    solve_generic(m, &i.curve, i.generator, target, i.r, seed)
}

/// `ic.hyperelliptic_thread`: phases `factor_base`, `walk_setup`, `relations`,
/// `linear_algebra`, each with its group ledger (Cantor additions, the
/// only charged work) and its exact native counts, all `*_uncharged`.
fn solve_hyper_ic(
    m: &ResolvedMethod,
    i: &HyperInstance,
    curve: &CurveFacts,
    target: crate::cryptanalysis::hyperelliptic_word::HDiv,
    seed: u64,
) -> Result<SolveReport, String> {
    let params = hyper_ic_params(m)?;
    hyper_ic_cyclic(i)?;
    let t = Instant::now();
    let out = hyper_ic::solve(
        &i.curve,
        i.generator,
        target,
        i.r,
        i.group_order,
        seed,
        &params,
    )?;
    let wall = t.elapsed().as_nanos() as u64;
    let phases: Vec<PhaseRecord> = out
        .phases
        .iter()
        .map(|(name, ops, native, wall_ns)| PhaseRecord {
            wall_ns: Some(*wall_ns),
            native: native.clone(),
            ..ops_phase(name, *ops)
        })
        .collect();
    let total_gae = phases.iter().map(|p| p.gae).sum();
    let mut keys: Vec<u64> = out.columns.iter().map(|d| i.curve.key_of(d)).collect();
    keys.sort_unstable();
    let bytes: Vec<u8> = keys.iter().flat_map(|k| k.to_be_bytes()).collect();
    let points_sha256 = sha256_hex(&bytes);
    let family = "hyperelliptic-degree1-places";
    let columns = out.columns.len() as u64;
    let (fb_id, fb_sha256) = short_id(
        "FB1",
        &json!({
            "schema": "ecbench.factor_base/v1",
            "curve": curve.slug,
            "family": family,
            "params": {"fb_size": m.params["fb_size"]},
            "columns": columns,
            "signed_points": out.signed_points,
            "points_sha256": points_sha256,
        }),
    )?;
    let factor_base = FactorBaseFacts {
        fb_id,
        fb_sha256,
        family: family.into(),
        params: [("fb_size".to_string(), m.params["fb_size"].clone())]
            .into_iter()
            .collect(),
        description: format!(
            "degree-1 places (x, y ≤ (p−1)/2) of y² = f(x) over F_{} in increasing x, one column per ± pair, cap {}",
            i.p, m.params["fb_size"]
        ),
        signed_points: out.signed_points,
        abscissae: columns,
        columns,
        dimension: None,
        points_sha256,
    };
    let detail = json!({
        "genus": i.genus,
        "p": i.p,
        "r": i.r,
        "group_order": i.group_order,
        "rounds_allowed": hyper_ic::ROUNDS,
        "oracle": "deg ≤ 2 closed form; deg ≥ 3 gcd(u, x^p − x), Cantor–Zassenhaus shifts b = 0, 1, …",
        "linear_algebra": "Markowitz sparse elimination mod r (the thread's sparse_solve_for_k)",
    });
    Ok(SolveReport {
        recovered: out.recovered.map(u128::from),
        exhausted: out.exhausted,
        phases,
        total_gae,
        // The factor base is folded by the hyperelliptic involution.
        automorphisms_used: 2,
        unpriced: unpriced_of(&out.counters),
        counters: out.counters,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: Some(factor_base),
        detail,
        online: None,
        online_error: None,
        solver: None,
        // HCurve does not tally the field work inside a Cantor step.
        field_ops: None,
    })
}

/// A GLS or subfield curve (`ecbench::ext`): the endomorphism-orbit rho
/// its family carries, or a generic method on the curve itself.
fn solve_ext(
    m: &ResolvedMethod,
    inst: &ExtInstance,
    tx: u64,
    ty: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    let generic = m.id.starts_with("rho.plain")
        || m.id == "rho.negation"
        || m.id == "rho.frozen_reference"
        || m.id.starts_with("bsgs.")
        || m.id == "kangaroo.vow";
    let folded = match (inst, m.id.as_str()) {
        (ExtInstance::Gls(_), "rho.gls") | (ExtInstance::Subfield(_), "rho.frobenius_orbit") => {
            true
        }
        _ if generic => false,
        (ExtInstance::Gls(_), _) => {
            return Err(format!("`{}` has no gls_fp2 implementation", m.id))
        }
        (ExtInstance::Subfield(_), _) => {
            return Err(format!("`{}` has no subfield_fp3 implementation", m.id))
        }
    };
    match inst {
        ExtInstance::Gls(i) => {
            let target = ExtInstance::decode_gls(i, tx, ty)?;
            if !folded {
                return solve_generic(m, &i.curve, i.generator, target, i.r, seed);
            }
            solve_endo_orbit(m, &i.curve, i.psi, i.generator, target, i.r, seed, "gls-psi orbit walk on {ψ^i(P)}: look-ahead, short-cycle escape by doubling, distinguished points, stride starts")
        }
        ExtInstance::Subfield(i) => {
            let target = ExtInstance::decode_subfield(i, tx, ty)?;
            if !folded {
                return solve_generic(m, &i.curve, i.generator, target, i.r, seed);
            }
            solve_endo_orbit(m, &i.curve, i.frobenius, i.generator, target, i.r, seed, "frobenius-orbit walk on {±π^i(P)}: look-ahead, short-cycle escape by doubling, distinguished points, stride starts")
        }
    }
}

/// Rho on the classes of an endomorphism, the verification of its
/// eigenvalue on `G` charged as set-up, in the `setup` phase and in the
/// total.  Records written before 2026-10-28 carried it in the phases only
/// (`audit::PRE_CLASS_SETUP_BINARIES`).
#[allow(clippy::too_many_arguments)]
fn solve_endo_orbit<G: CountedGroup, E: Endomorphism<G>>(
    m: &ResolvedMethod,
    g: &G,
    endo: E,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    name: &'static str,
) -> Result<SolveReport, String> {
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let mut setup = GroupOps::default();
    let classes = EndomorphismClasses::new(g, endo, gen, r, &mut setup, name)?;
    let mut res = rho_walk_with(
        g,
        &classes,
        gen,
        target,
        r,
        seed,
        rho_cap(r, param_u64(m, "cap_multiple")? as f64),
        RhoWalk::negation(),
    );
    charge_class_setup(&mut res, setup, r);
    res.counters.insert(
        "endomorphism_applications_uncharged".into(),
        classes.applications(),
    );
    res.counters.insert(
        "endomorphism_field_muls_uncharged".into(),
        classes.field_muls(),
    );
    measurement::end_online();
    let f = field_ops_of(g, &[res.group_ops]);
    let mut rep = rho_report(res, t.elapsed().as_nanos() as u64, f);
    if let (Some(d), Value::Object(c)) = (rep.detail.as_object_mut(), class_setup_detail(setup)) {
        d.extend(c);
    }
    Ok(rep)
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
    let start = Instant::now();
    let line = if m.id == "ic.trace_line_2lp" {
        let d = u32::try_from(param_u64(m, "d")?).map_err(|_| "d does not fit u32")?;
        Some(large_prime::trace_line::construct(
            inst,
            d,
            &m.params["line"],
            param_u64(m, "max_u")?,
        )?)
    } else {
        None
    };
    let summands = if line.is_some() {
        4
    } else {
        match m.params["summands"].as_str() {
            "n-1" => inst.n.checked_sub(1).ok_or("field degree has no n-1")?,
            value => value
                .parse::<u32>()
                .map_err(|_| format!("parameter `summands` is not an integer or n-1: `{value}`"))?,
        }
    };
    let config = LargePrimeConfig {
        small_dimension: u32::try_from(param_u64(m, "small_dimension")?)
            .map_err(|_| "small_dimension does not fit u32")?,
        envelope_dimension: if let Some(line) = &line {
            line.d
        } else {
            u32::try_from(param_u64(m, "envelope_dimension")?)
                .map_err(|_| "envelope_dimension does not fit u32")?
        },
        summands,
        max_large_primes: if line.is_some() {
            2
        } else {
            u8::try_from(param_u64(m, "large_primes")?)
                .map_err(|_| "large_primes does not fit u8")?
        },
        max_trials: param_u64(m, "max_trials")?,
        max_states: param_u64(m, "max_states")?,
        seed,
        large_prime_graph: Default::default(),
    };
    // The method returns an algebraic candidate; ecbench's runner verifies it.
    // `ic.large_prime_v2` also charges the target's `[r]Q` membership check
    // and the final `[d]G = Q` to the verification phase.
    let v2 = m.id == "ic.large_prime_v2";
    let report = if let Some(line) = &line {
        large_prime::solve_candidate_with_basis(inst, target, config, line.basis.clone())?
    } else if v2 {
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
    if let Some(line) = &line {
        counters.extend(line.native.clone());
        phases.push(PhaseRecord {
            name: "line_construction".into(),
            native: line.native.clone(),
            ..Default::default()
        });
    }
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
    let family = if line.is_some() {
        "binary-shifted-subfield-2lp"
    } else {
        "binary-nested-large-prime"
    };
    let (fb_id, fb_sha256) = short_id(
        "FB1",
        &json!({
            "schema": "ecbench.factor_base/v1",
            "curve": curve.slug,
            "family": family,
            "params": m.params,
            "columns": report.factor_base.columns,
            "signed_points": report.factor_base.raw_points,
            "points_sha256": points_sha256,
        }),
    )?;
    let factor_base = FactorBaseFacts {
        fb_id,
        fb_sha256,
        family: family.into(),
        params: m.params.clone(),
        description: if let Some(line) = &line {
            format!(
            "x in u*F_(2^{}) with u={}, nested small F2 dimension {}; cofactor projection and negation fold; exact MITM reference",
            line.d, line.multiplier, report.config.small_dimension
        )
        } else {
            format!(
                "x < 2^{} nested at x < 2^{}, projected by cofactor and folded by negation",
                report.config.envelope_dimension, report.config.small_dimension
            )
        },
        signed_points: report.factor_base.raw_points as u64,
        abscissae: 1u64 << report.config.envelope_dimension,
        columns: report.factor_base.columns as u64,
        dimension: Some(report.config.envelope_dimension),
        points_sha256,
    };
    let mut detail =
        serde_json::to_value(&report).map_err(|e| format!("serialise large-prime report: {e}"))?;
    if let Some(line) = &line {
        detail["trace_line"] = serde_json::to_value(line).map_err(|e| e.to_string())?;
    }
    let mut unpriced = vec![
        "combination_states_uncharged".into(),
        "mitm_lookups_uncharged".into(),
        "lp_merge_ops_uncharged".into(),
        "row_ops_uncharged".into(),
    ];
    if let Some(line) = &line {
        unpriced.extend(line.native.keys().cloned());
    }
    Ok(SolveReport {
        recovered: report.recovered.map(u128::from),
        exhausted: report.exhausted,
        phases,
        total_gae,
        automorphisms_used: 2,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: if line.is_some() {
            start.elapsed().as_nanos() as u64
        } else {
            report.solve_wall_ns
        },
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
    if id == "ic.known_log_mitm2_v1" {
        w.start_event = "first target-dependent Q+[a]H query after the target-blind base and pair table are complete".into();
        w.mapping = "known-log two-sum recovery: query generation, pair-table PDP, full-point witness check, direct scalar combination (no matrix), and final [d]G=Q verification".into();
    }
    if id == "ic.known_log_ksum_v1" {
        w.start_event = "first target-dependent Q+[a]H query after the target-blind base and k-sum table are complete".into();
        w.mapping = "known-log k-sum recovery: query generation, k-sum-table PDP, full-point witness check (k − 1 additions), direct scalar combination (no matrix), and final [d]G=Q verification".into();
    }
    if id == "rho.preprocessing" {
        w.start_event = "first target-dependent operation (G + Q and the online stride) after the target-blind jump table and distinguished-point table are complete".into();
        w.mapping = "preprocessing rho: the online walks on the precomputation's map, table lookups at distinguished points, and final [d]G=Q verification".into();
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
        work_native: p.work_native.clone(),
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

fn solve_preprocessing<G: CountedGroup>(
    m: &ResolvedMethod,
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    let entries = param_u64(m, "entries")?;
    if entries == 0 {
        return Err("rho.preprocessing needs entries > 0".into());
    }
    let dp_bits = match param_u64(m, "dp_bits")? {
        0 => None,
        d if d <= 40 => Some(d as u32),
        _ => return Err("rho.preprocessing dp_bits must lie in 0..=40".into()),
    };
    let table_seed = param_u64(m, "table_seed")?;
    let max_precompute = match param_u64(m, "precompute_cap")? {
        0 => (64.0 * (r as f64 * entries as f64).sqrt()) as u64 + (1 << 20),
        c => c,
    };
    let cap = rho_cap(r, param_u64(m, "cap_multiple")? as f64);
    let classes = NegationClasses { r };
    let started = Instant::now();
    measurement::mark(Phase::Precompute);
    let mut table = PreprocessingRho::build(
        g,
        &classes,
        generator,
        r,
        entries,
        dp_bits,
        table_seed,
        max_precompute,
        RhoWalk::negation(),
    );
    measurement::begin_online(Phase::RhoSolve);
    let online = table.solve(target, seed, cap);
    measurement::end_online();
    let wall = started.elapsed().as_nanos() as u64;
    let mut counters = table.counters.clone();
    for (name, v) in &online.counters {
        *counters.entry(name.clone()).or_default() += v;
    }
    let mut setup = ops_phase("setup", table.setup_ops);
    setup.wall_ns = None;
    let mut precompute = ops_phase("precompute", table.precompute_ops);
    precompute.wall_ns = Some(table.precompute_wall_ns);
    let mut search = ops_phase("search", online.ops);
    search.gae = online.ops.gae() - online.verification_ops as f64;
    search.wall_ns = Some(online.wall_ns);
    let verify = PhaseRecord {
        name: "internal_verification".into(),
        gae: online.verification_ops as f64,
        ..Default::default()
    };
    let phases = vec![setup, precompute, search, verify];
    let short = table.entries < entries;
    Ok(SolveReport {
        recovered: online.recovered.map(u128::from),
        exhausted: online.recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: 2,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        solve_wall_ns: wall,
        detail: json!({
            "table": {"entries": table.entries, "requested": entries, "short": short,
                "dp_bits": table.dp_bits, "table_seed": table_seed},
            "query_law": "online walks start at [k]([t_a]G + [t_b]Q), t seeded by the run seed",
            "recovery_law": "a walk ending at a stored [a2]G gives log_G(Q) = (a2 - a)/b mod r",
            "cost_scope": "cold, one target: jump table, precomputation and online walk all charged; table lookups and canonicalisations unpriced",
        }),
        field_ops: field_ops_of(g, &[table.setup_ops, table.precompute_ops, online.ops]),
        ..Default::default()
    })
}

fn solve_known_log<G: CountedGroup>(
    m: &ResolvedMethod,
    group: &G,
    generator: G::Elt,
    target: G::Elt,
    r: u64,
    curve: &CurveFacts,
    seed: u64,
) -> Result<SolveReport, String> {
    let max_trials = param_u64(m, "max_trials")?;
    if max_trials == 0 {
        return Err("known-log max_trials must be positive".into());
    }
    let ksum = m.id == "ic.known_log_ksum_v1";
    let params = super::known_log::Parameters {
        size: usize::try_from(param_u64(m, "size")?).map_err(|_| "base size exceeds usize")?,
        base_seed: param_u64(m, "base_seed")?,
        base_multiplier: param_u64(m, "base_multiplier")?,
        max_pair_work: param_u64(m, if ksum { "max_sums" } else { "max_pair_work" })?,
    };
    let started = Instant::now();
    let prepared = if ksum {
        let k = u32::try_from(param_u64(m, "k")?).map_err(|_| "k exceeds u32")?;
        super::known_log::prepare_ksum(group, generator, r, params, k)?
    } else {
        super::known_log::prepare(group, generator, r, params)?
    };
    let recovered = super::known_log::recover(group, &prepared, target, seed, max_trials)?;
    let wall = started.elapsed().as_nanos() as u64;
    let mut counters = prepared.counters.clone();
    for (name, count) in &recovered.counters {
        *counters.entry(name.clone()).or_default() += count;
    }
    // No matrix was built: the base logarithms are all known and the
    // coefficient of Q in every query is 1. The useful target equation
    // and its rank are reported separately from a nonexistent base matrix.
    counters.insert("matrix_rows".into(), 0);
    counters.insert("matrix_rank".into(), 0);
    counters.insert("matrix_dependent".into(), 0);
    counters.insert("matrix_work".into(), 0);
    let ledgers = [
        prepared.base_ops,
        prepared.table_ops,
        recovered.query_ops,
        recovered.decomposition_ops,
        recovered.relation_ops,
        recovered.recovery_ops,
        recovered.verify_ops,
    ];
    let mut phases: Vec<_> = [
        "factor_base",
        "oracle_setup",
        "target_query",
        "target_PDP",
        "target_relation_check",
        "target_descent",
        "target_recovery_check",
    ]
    .iter()
    .zip(ledgers)
    .map(|(name, ops)| ops_phase(name, ops))
    .collect();
    phases[0].wall_ns = Some(prepared.base_wall_ns);
    phases[1].wall_ns = Some(prepared.table_wall_ns);
    phases.push(ops_phase("linear_algebra", GroupOps::default()));
    let mut fb_params = Params::default();
    for key in ["size", "base_seed", "base_multiplier"] {
        fb_params.set(key, m.params[key].clone());
    }
    let facts = factor_base_facts(
        &curve.slug,
        "known-log-seeded",
        &fb_params,
        &prepared.base,
        |p| group.key(p),
    )?;
    Ok(SolveReport {
        recovered: recovered.recovered.map(u128::from),
        exhausted: recovered.recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: 2,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        solve_wall_ns: wall,
        factor_base: Some(facts),
        detail: json!({
            "factor_base_log_status": "all known; constructed without a target",
            "query_law": "Q+[a+t]H; a seeded uniformly, no repeats before r queries; H=[u]G",
            "recovery_law": "log_G(Q)=u*(log_H(F_i)+log_H(F_j)-a-t) mod r",
            "linear_algebra": {"name":"none", "reason":"no unknown factor-base logarithms"},
            "table_memory": if ksum {
                "negation-folded k-sum table, one witness per distinct signed sum class, summand indices packed in a u64; peak RSS recorded by runner"
            } else {
                "negation-folded pair table, one witness per distinct signed sum class; peak RSS recorded by runner"
            },
            "cost_scope": "cold, one target, all group operations plus explicitly unpriced native work",
        }),
        field_ops: field_ops_of(group, &ledgers),
        ..Default::default()
    })
}

/// The phases of `ic.tensor_contraction_v1`, in the order the solver enters
/// them (`ecdl_tensor_ic::opcount`).
const TENSOR_PHASES: [&str; 6] = [
    "setup",
    "factor_base",
    "relation_target",
    "relation_search",
    "linear_algebra",
    "recovery_check",
];

/// `ic.tensor_contraction_v1`: the toy tensor-contraction index calculus of
/// `crates/ecdl-tensor-ic` on a prime-order curve.  Every non-trivial point
/// addition and doubling the solver performs is charged, phase by phase,
/// from the crate's tally; contraction rows, factor-base listing (the toy
/// enumerates the group) and elimination multiplications are counted
/// `_uncharged`.  The solver checks `[ξ]P = Q` itself (charged in
/// `recovery_check`) and redraws its factor bases on failure.
fn solve_tensor_contraction(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    use ecdl_tensor_ic::{opcount, Curve as TCurve, Point as TPoint, Solver, SparseContractor};

    let fb_count = usize::try_from(param_u64(m, "m")?).map_err(|_| "m exceeds usize")?;
    if fb_count < 2 {
        return Err("ic.tensor_contraction_v1 needs m ≥ 2".into());
    }
    let max_restarts =
        usize::try_from(param_u64(m, "max_restarts")?).map_err(|_| "max_restarts exceeds usize")?;
    if max_restarts == 0 {
        return Err("ic.tensor_contraction_v1 needs max_restarts ≥ 1".into());
    }
    if inst.cofactor != 1 || inst.group_order != inst.r {
        return Err(format!(
            "ic.tensor_contraction_v1 needs a prime-order curve (cofactor 1); {} has cofactor {}",
            inst.name, inst.cofactor
        ));
    }
    let curve = TCurve {
        p: inst.curve.p,
        a: inst.curve.a,
        b: inst.curve.b,
        order: inst.r,
        g: TPoint {
            x: inst.generator.0,
            y: inst.generator.1,
            inf: false,
        },
    };
    let q = TPoint {
        x: target.x,
        y: target.y,
        inf: target.infinity,
    };
    let mut solver = Solver::new(
        curve,
        fb_count,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(seed),
    );
    solver.max_restarts = max_restarts;
    let started = Instant::now();
    opcount::start();
    let (result, stats) = solver.solve_with_stats(q);
    let tally = opcount::take();
    let wall = started.elapsed().as_nanos() as u64;

    let mut phases = Vec::with_capacity(TENSOR_PHASES.len());
    let mut charged = 0usize;
    for name in TENSOR_PHASES {
        let mut get = |k: &str| match tally.get(&format!("{name}.{k}")) {
            Some(&n) => {
                charged += 1;
                n
            }
            None => 0,
        };
        let ops = GroupOps {
            adds: get("adds"),
            doubles: get("doubles"),
            ..GroupOps::default()
        };
        phases.push(ops_phase(name, ops));
    }
    let mut counters: BTreeMap<String, u64> = tally
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    // Every tally entry is either charged above or kept as unpriced work.
    if charged + counters.len() != tally.len() {
        return Err(format!(
            "ic.tensor_contraction_v1: tally has entries outside the known phases: {:?}",
            tally.keys().collect::<Vec<_>>()
        ));
    }
    for (key, value) in [
        ("factor_base_size", stats.factor_base_size),
        ("relations", stats.relations),
        ("attempts", stats.attempts),
        ("restarts", stats.restarts),
        ("network_edges", stats.network.edges),
        ("network_vertices", stats.network.vertices),
        ("network_max_degree", stats.network.max_degree),
    ] {
        counters.insert(key.into(), value as u64);
    }
    let (recovered, error) = match result {
        Ok(xi) => (Some(u128::from(xi)), None),
        Err(e) => (None, Some(e.to_string())),
    };
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: 1,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        solve_wall_ns: wall,
        detail: json!({
            "implementation": "crates/ecdl-tensor-ic (Rust port of the Go ecdltensor prototype)",
            "factor_base": "m keyed SHA-256 membership predicates, listed by enumerating the group (toy shortcut)",
            "relation_network": "whole-point network, (2m−1)(2b+1) edges; counted by SparseContractor, witness by self-reduction",
            "linear_algebra": "dense left kernel mod r",
            "charged": "every non-trivial point addition and doubling (neither input O), including those evaluated inside the contraction",
            "error": error,
            "cost_scope": "cold, one target; no [Ope26] solver, every counter exponential",
        }),
        ..Default::default()
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
        "explicit-x" => {
            Box::new(crate::cryptanalysis::ic_framework::explicit_base::PrimeExplicitXBase)
        }
        "prime-abscissa" => Box::new(PrimeAbscissaBase { instance: inst }),
        "prime-projected-abscissa" => Box::new(PrimeProjectedAbscissaBase { instance: inst }),
        "glv-orbit" => Box::new(GlvOrbitBase { instance: inst }),
        "prime-kummer" => Box::new(PrimeKummerBase { instance: inst }),
        "prime-isogeny" => Box::new(PrimeIsogenyBase { instance: inst }),
        "cm-scalar-orbit" => Box::new(CmOrbitBase { instance: inst }),
        "known-log" => Box::new(crate::cryptanalysis::ic_framework::plugins::KnownLogBase),
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
    let mut ksum = KSumOracle::new(ms);
    let mut prime_batch = PrimeBatchMitmOracle::new();
    let mut prime_blocked = PrimeBatchMitmOracle::blocked();
    let mut mitm_fiber = FiberMitmOracle::new(ms);
    let mut rootfind_mitm = RootfindMitmOracle::new(ms);
    let mut rootfind_pairs = RootfindPairsOracle::new(ms);
    let mut rootfind = RootfindSubtractOracle::new(ms, inst);
    // `pkm-semaev` writes its system on the base's own tower, so it reads
    // the tower from the `prime-kummer` spec and runs on no other base.
    let mut pkm = match or_name.as_str() {
        "pkm-semaev" => {
            if fb_name != "prime-kummer" {
                return Err(format!(
                    "oracle `pkm-semaev` needs the prime-kummer base, whose tower it solves on, not `{fb_name}`"
                ));
            }
            let t = spec.factor_base_params.u64("t")? as u32;
            let g = spec.factor_base_params.u64_or("g", 1)?;
            let budget = (spec.solver_budget_seconds > 0)
                .then(|| std::time::Duration::from_secs(spec.solver_budget_seconds));
            Some(
                crate::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle::new(ms, t, g)
                    .with_budget(budget),
            )
        }
        _ => None,
    };
    let oracle: &mut dyn DecompositionOracle<_> = match or_name.as_str() {
        "pkm-semaev" => pkm.as_mut().expect("built above"),
        "subtract" => &mut subtract,
        "mitm" => &mut mitm,
        "ksum" => &mut ksum,
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
    // The base the run built and used is the one described below: no
    // second build outside the measured run.
    let (rep, fb) = run_pipeline_with_base(&ctx, &spec, &*base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |p| inst.curve.key(p),
    )?;
    Ok(ic_report(m, rep, &calib, Some(facts), wall, 2))
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
    // The base the run built and used is the one described below: no
    // second build outside the measured run.
    let (rep, fb) =
        run_pipeline_with_base(&ctx, &spec, &base, &mut oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )?;
    Ok(ic_report(m, rep, &calib, Some(facts), wall, 2))
}

/// `ic.pipeline` on a curve over `F_{(2^m)⁴}` with `bin-t-line` and
/// `gaudry-d4-bin`, priced as [`solve_ic_fp4`] prices its curve: the
/// oracle's `F_q` multiplications at the curve's own multiplications per
/// addition, lifts at their measured count, row operations at one each.
/// The strong negation-map rho's id.
pub const NEG_STRONG: &str = "rho.negation_strong";

fn neg_strong_refusal(curve: &CurveFacts) -> String {
    format!(
        "`{NEG_STRONG}` runs on f2m4_torsion curves (over F_(2^m)^4) only; {} is {}",
        curve.slug, curve.family
    )
}

/// `lanes`, `dp_bits` and `step_cap_factor` of a `rho.negation_strong`
/// method, `dp_bits = auto` resolved on a subgroup of order `r`.
fn neg_strong_params(m: &ResolvedMethod, r: u64) -> Result<NegStrongParams, String> {
    use super::negation_strong::{auto_dp_bits, ideal_steps};
    let lanes = param_u64(m, "lanes")?;
    if !(1..=4096).contains(&lanes) {
        return Err(format!("`lanes` must be in 1..=4096, not {lanes}"));
    }
    let dp_bits = match m.params["dp_bits"].as_str() {
        "auto" => auto_dp_bits(ideal_steps(r)),
        _ => {
            let v = param_u64(m, "dp_bits")?;
            if !(1..=39).contains(&v) {
                return Err(format!("`dp_bits` must be in 1..=39 or `auto`, not {v}"));
            }
            v as u32
        }
    };
    Ok(NegStrongParams {
        lanes: lanes as usize,
        dp_bits,
        step_cap_factor: param_u64(m, "step_cap_factor")?,
    })
}

/// `rho.negation_strong` on a curve over `F_{(2^m)^4}`: every phase in
/// counted `F_q` multiplications over the curve's `fq_muls_per_add`, the
/// divisor [`solve_ic_f2m4`] prices the IC pipeline with.
fn solve_neg_strong(
    m: &ResolvedMethod,
    inst: &BinCurve,
    target: PtB,
    seed: u64,
) -> Result<SolveReport, String> {
    use super::negation_strong::{NegStrongRho, CANON_FQ_MULS, JUMPS};
    if inst.curve.f.f.m > 16 {
        return Err("rho.negation_strong requires a one-word packed point key".into());
    }
    let r = u64::try_from(inst.r)
        .map_err(|_| "rho.negation_strong supports subgroup orders through u64".to_string())?;
    let params = neg_strong_params(m, r)?;
    if !inst.curve.on_curve(&target) {
        return Err("the target is not on the curve".into());
    }
    let rho = NegStrongRho::new(inst);
    // Measured before the clock, on the curve's own counter (as solve_ic_f2m4
    // measures it); the counter is read in differences, so this does not
    // reach the solve's counts.
    let per_add = rho.fq_muls_per_add();
    let t = Instant::now();
    measurement::begin_online(Phase::RhoSolve);
    let o = rho.solve(target, seed, &params);
    measurement::end_online();
    let wall = t.elapsed().as_nanos() as u64;
    let div = per_add as f64;
    let canon_muls = o.canonicalisations * CANON_FQ_MULS;
    let phase = |name: &str, adds: u64, scalar_mults: u64, muls: u64| PhaseRecord {
        name: name.into(),
        adds,
        scalar_mults,
        gae: muls as f64 / div,
        native: [("fq_muls".to_string(), muls)].into_iter().collect(),
        ..Default::default()
    };
    let check_mults = u64::from(o.scalar.is_some()) + o.failed_collisions;
    let phases = vec![
        phase("setup", 0, JUMPS as u64 + 2, o.setup_fq_muls),
        phase(
            "search",
            o.walk_steps + o.start_additions + o.escapes,
            0,
            o.search_fq_muls,
        ),
        phase("canonicalisation", 0, 0, canon_muls),
        phase("check", 0, check_mults, o.check_fq_muls),
    ];
    let total_gae: f64 = phases.iter().map(|p| p.gae).sum();
    let mut counters = BTreeMap::new();
    for (k, v) in [
        ("fq_muls", o.charged_fq_muls()),
        ("fq_muls_per_add", per_add),
        ("fq_muls_setup", o.setup_fq_muls),
        ("fq_muls_search", o.search_fq_muls),
        ("fq_muls_check", o.check_fq_muls),
        ("fq_muls_canonicalisation", canon_muls),
        ("canonicalisations", o.canonicalisations),
        ("inversions", o.inversions),
        ("batch_inversions", o.batch_inversions),
        ("walk_steps", o.walk_steps),
        ("unbatched_steps", o.unbatched_steps),
        ("walks", o.walks),
        ("start_additions", o.start_additions),
        ("fruitless_escapes", o.escapes),
        ("capped_walks", o.capped_walks),
        ("wasted_merges", o.wasted_merges),
        ("failed_collisions", o.failed_collisions),
        ("table_entries", o.table_entries),
        ("scalar_multiplications", o.scalar_multiplications),
        ("partition_hashes_uncharged", o.partition_hashes),
        ("dp_tests_uncharged", o.dp_tests),
        ("table_queries_uncharged", o.table_queries),
        ("table_inserts_uncharged", o.table_inserts),
    ] {
        counters.insert(k.to_string(), v);
    }
    let detail = json!({
        "reference": "ecbench::negation_strong: negation-map rho with distinguished points, Montgomery-batched inversion and charged canonicalisation (red-team replication plan, research/gaudry_k4_char2_redteam_20261026/REPORT.md §3)",
        "unit": "counted F_q multiplications (Gf2q::muls) / fq_muls_per_add",
        "fq_muls_per_add": per_add,
        "canonicalisation_charge_fq_muls": CANON_FQ_MULS,
        "canonicalisation_rule": "representative of {P, −P} with the smaller packed ordinate of y and x + y: four F_q additions and one comparison, charged one F_q multiplication (an upper bound)",
        "jumps": JUMPS,
        "lanes": params.lanes,
        "dp_bits": params.dp_bits,
        "step_cap_factor": params.step_cap_factor,
        "ideal_steps": o.ideal_steps,
        "fq_muls_per_step": if o.walk_steps > 0 {
            Some((o.search_fq_muls + canon_muls) as f64 / o.walk_steps as f64)
        } else {
            None
        },
    });
    Ok(SolveReport {
        recovered: o.scalar.map(u128::from),
        exhausted: o.scalar.is_none(),
        phases,
        total_gae,
        automorphisms_used: 2,
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
        frob_roots: flag("frob_roots")?,
    };
    let mut oracle = BinD4Oracle::new(inst, macaulay, seed, opts);
    oracle.charge_unsolved = flag("charge_unsolved")?;
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
    // The base the run built and used is the one described below: no
    // second build outside the measured run.
    let (rep, fb) =
        run_pipeline_with_base(&ctx, &spec, &base, &mut oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
    let facts = factor_base_facts(
        &curve.slug,
        &m.params["factor_base"],
        &spec.factor_base_params,
        &fb,
        |q| CountedGroup::key(c, q),
    )?;
    Ok(ic_report(m, rep, &calib, Some(facts), wall, 2))
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
    use crate::cryptanalysis::ic_wide::{rho_negation, run_f2m4};
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
        "ic.pipeline" => {
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
            let mut rep = ic_report(m, run.report, &calib, Some(facts), wall, 2);
            rep.recovered = run.recovered;
            rep.exhausted = run.recovered.is_none();
            Ok(rep)
        }
        other => Err(format!(
            "`{other}` has no implementation past one word on a curve over F_(2^m)^4; rho.negation and ic.pipeline do"
        )),
    }
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
        frob_roots: flag("frob_roots")?,
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
        "explicit-x" => Box::new(
            crate::cryptanalysis::ic_framework::explicit_base::BinaryExplicitXBase {
                instance: inst,
            },
        ),
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
    let budget = (spec.solver_budget_seconds > 0)
        .then(|| std::time::Duration::from_secs(spec.solver_budget_seconds));
    let solver_named = || -> Result<_, String> {
        let name = spec
            .solver
            .as_deref()
            .ok_or_else(|| format!("{or_name} needs `solver`"))?;
        solver_by_name(name)
    };
    // Build only the oracle the method names: every constructor runs inside
    // the measured solve, and building the others (each Frobenius variant
    // with its own canonicalisation tables) was work no counter saw and the
    // instruction audit charged to set-up.
    let mut oracle_box: Box<dyn DecompositionOracle<BinaryGroup> + '_> = match or_name.as_str() {
        "subtract" => Box::new(SubtractOracle),
        "mitm" => Box::new(MitmOracle::new(ms)),
        "mitm-frobenius" => Box::new(FrobeniusMitmOracle::new(ms, inst)),
        "mitm-frobenius-counted" => Box::new(FrobeniusMitmOracle::new_counted(ms, inst)),
        "mitm-frobenius-lazy-counted" => Box::new(FrobeniusMitmOracle::new_lazy_counted(ms, inst)),
        "mitm-frobenius-layout-counted" => {
            Box::new(FrobeniusMitmOracle::new_layout_counted(ms, inst))
        }
        "mitm-fiber" => Box::new(FiberMitmOracle::new(ms)),
        "mitm-frobenius-fiber" => Box::new(FiberFrobeniusMitmOracle::new(ms, inst)),
        "rootfind-mitm" => Box::new(RootfindMitmOracle::new(ms)),
        "rootfind-pairs" => Box::new(RootfindPairsOracle::new(ms)),
        "descent-algebraic" => Box::new(DescentAlgebraicOracle::new(
            ms,
            inst,
            solver_named()?,
            spec.solver_params.clone(),
            budget,
        )),
        "descent-algebraic-fiber" => Box::new(FiberDescentOracle::new(
            ms,
            inst,
            solver_named()?,
            spec.solver_params.clone(),
            budget,
        )),
        other => match binary_plugins().and_then(|p| (p.oracle)(or_name.as_str(), ms, inst)) {
            Some(o) => o,
            None => return Err(format!("oracle `{other}` does not run on a binary curve")),
        },
    };
    let oracle: &mut dyn DecompositionOracle<BinaryGroup> = oracle_box.as_mut();
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
    // The base the run built and used is the one described below: no
    // second build outside the measured run.
    let (rep, fb) = run_pipeline_with_base(&ctx, &spec, &*base, oracle, u64::MAX, &calib, None)?;
    let wall = t.elapsed().as_nanos() as u64;
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
    Ok(ic_report(m, rep, &calib, Some(facts), wall, a))
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
/// an `ic.shared_rank` run, which names the base in the record but changes no
/// count.  A measured child calls this after its timing, scheduler,
/// hardware-counter and Callgrind windows have closed, so the rebuild is
/// not in the solve's resource vector; [`solve`] calls both for every other
/// caller.  The other index-calculus pipelines describe the base the run
/// built and used, in [`solve_measured`] itself (one build, no second one to
/// disagree with it), so their report has an inventory already; a report
/// with an inventory already, and every method without one, is left as it is.
pub fn complete(
    m: &ResolvedMethod,
    inst: &Instance,
    curve: &CurveFacts,
    _target: &[String; 2],
    rep: &mut SolveReport,
) -> Result<(), String> {
    if rep.factor_base.is_some() {
        return Ok(());
    }
    if let Instance::Binary(i) = inst {
        if m.id == "ic.shared_rank" {
            rep.factor_base = Some(shared_rank_inventory(m, i, curve, &rep.detail)?);
        }
    }
    Ok(())
}

/// `ic.koblitz_descent`: the §23 pipeline on the target, every stage
/// charged by the point additions and doublings it performed
/// ([`koblitz_descent`]).  The run's algorithm seed is used only where
/// `seed = run`; the rule v8 recipe pins its own.
fn solve_koblitz_descent(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    koblitz_descent_applicable(m, inst)?;
    let kc = inst.koblitz.as_ref().ok_or("not a Koblitz instance")?;
    let p = KoblitzDescentParams::from_method(&m.params, kc.n, kc.a, seed)?;
    let q = inst.fast.lower(target);
    let started = Instant::now();
    let out = koblitz_descent::run(kc, &q, &p)?;
    let wall = started.elapsed().as_nanos() as u64;
    if !out.single_thread {
        return Err("ic.koblitz_descent did not run on its one counting thread".into());
    }
    let phases: Vec<PhaseRecord> = out
        .phases
        .iter()
        .map(|ph| PhaseRecord {
            name: ph.name.clone(),
            adds: ph.ops.adds,
            doubles: ph.ops.doubles,
            scalar_mults: ph.ops.scalar_mults,
            gae: ph.ops.gae(),
            wall_ns: Some(ph.wall_ns),
            native: koblitz_descent::native_of(&ph.ops),
            work_native: BTreeMap::new(),
        })
        .collect();
    let total_gae: f64 = phases.iter().map(|p| p.gae).sum();
    let c = &out.counts;
    let mut counters: BTreeMap<String, u64> = [
        ("fb_points", c.fb_points as u64),
        ("fb_signed_orbits", c.fb_signed_orbits as u64),
        ("columns", c.columns as u64),
        ("pair_table_entries", c.pair_table_entries as u64),
        ("collection_units", c.units as u64),
        ("collection_units_extended", c.units_extended as u64),
        ("collection_trials", c.trials),
        ("summands_scanned", c.summands_scanned),
        ("relations_collected", c.relations_collected as u64),
        ("relations_accepted", c.relations_accepted as u64),
        ("relations_rejected", c.relations_rejected as u64),
        ("relations_duplicate", c.relations_duplicate as u64),
        ("solve_attempts", c.solve_attempts as u64),
        ("descent_trials", c.descent_trials as u64),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    let mut native = OpCounts::default();
    for ph in &out.phases {
        native.merge(&ph.ops);
    }
    let la_row_ops = c.sparse.as_ref().map_or(0, |r| r.row_ops);
    for (name, v) in [
        ("canonicalisations_uncharged", native.canonicalisations),
        ("pair_filter_probes_uncharged", native.filter_probes),
        ("pair_bucket_reads_uncharged", native.bucket_reads),
        ("frobenius_maps_uncharged", native.frobenius_maps),
        ("la_row_ops_uncharged", la_row_ops),
        ("fb_abscissae_lifted_uncharged", c.selection.abscissae_drawn),
        ("fb_rebuilds_uncharged", u64::from(c.selection.rebuilds)),
    ] {
        counters.insert(name.into(), v);
    }
    let mut unpriced = unpriced_of(&counters);
    // Counted nowhere in the unit, and not tallied one by one: the field
    // arithmetic outside the group law (lifts, half-traces, the normal
    // basis), the projected-orbit maps and hashing, the modular
    // combination of the descent's relation, and the forward pass of a
    // vector batch that declined a degenerate block (its lane count is in
    // `detail.vector_declined_lanes`; it depends on the host's CPU, so it
    // is not a replayed counter).
    unpriced.extend([
        "field_arithmetic_uncharged".into(),
        "hash_and_allocation_uncharged".into(),
        "modular_combination_uncharged".into(),
        "vector_declined_lanes_uncharged".into(),
    ]);
    unpriced.sort();
    unpriced.dedup();
    let facts = koblitz_descent::factor_base_facts(
        &curve.slug,
        kc,
        &out.factor_base,
        c.fb_signed_orbits,
        p.factor_base_points,
        p.seed,
    )?;
    let online = match &out.snapshot {
        Ok(snap) => koblitz_descent_online(snap),
        Err(_) => None,
    };
    let online_error = match (&out.snapshot, &online) {
        (Err(e), _) => Some(e.clone()),
        (Ok(_), None) => Some("the descent's online interval did not close".into()),
        _ => None,
    };
    Ok(SolveReport {
        recovered: out
            .recovered
            .as_ref()
            .and_then(|d| d.to_u64())
            .map(u128::from),
        exhausted: out.recovered.is_none(),
        phases,
        total_gae,
        automorphisms_used: 2 * inst.n,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: Vec::new(),
        solve_wall_ns: wall,
        factor_base: Some(facts),
        detail: koblitz_descent::detail_of(&out, &p),
        online,
        online_error,
        solver: None,
        field_ops: None,
    })
}

/// The descent's online window from the session the pipeline ran under:
/// the five claim phases, which sum to the interval.
fn koblitz_descent_online(snap: &Snapshot) -> Option<OnlineWindow> {
    let mut w = online_window("ic", "ic.koblitz_descent", snap)?;
    w.start_event = "the descent's first target-dependent operation ([a0]G + [b]Q, the walks' start), after the factor base, pair table and every column log are ready".into();
    w.stop_event = "[d]G = Q checked inside the descent (one-word arithmetic)".into();
    w.mapping = "one-target descent over a solved log table: building the walks and stepping them by G is target_query, keying each state and probing the pair table is target_PDP, the witness re-check is target_relation_check, combining the column logs into d is target_descent, and the descent's [d]G = Q is target_recovery_check; the relations and logs are target-independent set-up outside the window".into();
    (w.phases_ns.values().sum::<u64>() == w.wall_ns).then_some(w)
}

/// `ic.koblitz_batched`: the tournament worker's pipeline on the target,
/// every stage charged by the point additions and doublings it performed
/// ([`koblitz_batched`]).  The run's algorithm seed is used where
/// `seed = run`.
fn solve_koblitz_batched(
    m: &ResolvedMethod,
    inst: &BinaryInstance,
    curve: &CurveFacts,
    target: FastPoint,
    seed: u64,
) -> Result<SolveReport, String> {
    koblitz_batched_applicable(m, inst)?;
    let kc = inst.koblitz.as_ref().ok_or("not a Koblitz instance")?;
    let p = KoblitzBatchedParams::from_method(&m.params, seed)?;
    let q = inst.fast.lower(target);
    let started = Instant::now();
    let out = koblitz_batched::run(kc, &q, &p)?;
    let wall = started.elapsed().as_nanos() as u64;
    if !out.single_thread {
        return Err("ic.koblitz_batched did not run on its one counting thread".into());
    }
    let phases: Vec<PhaseRecord> = out
        .phases
        .iter()
        .map(|ph| PhaseRecord {
            name: ph.name.clone(),
            adds: ph.ops.adds,
            doubles: ph.ops.doubles,
            scalar_mults: ph.ops.scalar_mults,
            gae: ph.ops.gae(),
            wall_ns: Some(ph.wall_ns),
            native: koblitz_descent::native_of(&ph.ops),
            work_native: BTreeMap::new(),
        })
        .collect();
    let total_gae: f64 = phases.iter().map(|p| p.gae).sum();
    let c = &out.counts;
    let mut counters: BTreeMap<String, u64> = [
        ("fb_points", c.fb_points as u64),
        ("fb_signed_orbits", c.fb_signed_orbits as u64),
        ("columns", c.columns as u64),
        ("pair_table_entries", c.pair_table_entries as u64),
        ("collection_batches", c.units as u64),
        ("collection_trials", c.trials),
        ("summands_scanned", c.summands_scanned),
        ("relations_collected", c.relations_collected as u64),
        ("relations_accepted", c.relations_accepted as u64),
        ("relations_rejected", c.relations_rejected as u64),
        ("relations_duplicate", c.relations_duplicate as u64),
        ("solve_attempts", c.solve_attempts as u64),
        ("descent_trials", c.descent_trials as u64),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    let mut native = OpCounts::default();
    for ph in &out.phases {
        native.merge(&ph.ops);
    }
    let la_row_ops = c.sparse.as_ref().map_or(0, |r| r.row_ops);
    for (name, v) in [
        ("canonicalisations_uncharged", native.canonicalisations),
        ("pair_filter_probes_uncharged", native.filter_probes),
        ("pair_bucket_reads_uncharged", native.bucket_reads),
        ("frobenius_maps_uncharged", native.frobenius_maps),
        ("la_row_ops_uncharged", la_row_ops),
    ] {
        counters.insert(name.into(), v);
    }
    let mut unpriced = unpriced_of(&counters);
    // Counted nowhere in the unit: the field arithmetic outside the group
    // law, the projected-orbit maps and hashing, the modular combination
    // of the descent's relation, and the forward pass of a vector batch
    // that declined a degenerate block (host-dependent, so not replayed).
    unpriced.extend([
        "field_arithmetic_uncharged".into(),
        "hash_and_allocation_uncharged".into(),
        "modular_combination_uncharged".into(),
        "vector_declined_lanes_uncharged".into(),
    ]);
    unpriced.sort();
    unpriced.dedup();
    let facts = koblitz_descent::factor_base_facts(
        &curve.slug,
        kc,
        &out.factor_base,
        c.fb_signed_orbits,
        p.factor_base_points,
        p.fb_seed,
    )?;
    let online = match &out.snapshot {
        Ok(snap) => koblitz_batched_online(snap),
        Err(_) => None,
    };
    let online_error = match (&out.snapshot, &online) {
        (Err(e), _) => Some(e.clone()),
        (Ok(_), None) => Some("the descent's online interval did not close".into()),
        _ => None,
    };
    Ok(SolveReport {
        recovered: out
            .recovered
            .as_ref()
            .and_then(|d| d.to_u64())
            .map(u128::from),
        exhausted: out.recovered.is_none(),
        phases,
        total_gae,
        automorphisms_used: 2 * inst.n,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: Vec::new(),
        solve_wall_ns: wall,
        factor_base: Some(facts),
        detail: koblitz_batched::detail_of(&out, &p),
        online,
        online_error,
        solver: None,
        field_ops: None,
    })
}

/// The batched pipeline's online window: the same interval and phases as
/// `ic.koblitz_descent`'s, the descent over a solved log table.
fn koblitz_batched_online(snap: &Snapshot) -> Option<OnlineWindow> {
    let mut w = online_window("ic", "ic.koblitz_batched", snap)?;
    w.start_event = "the descent's first target-dependent operation ([a0]G + [b]Q, the walks' start), after the factor base, pair table and every column log are ready".into();
    w.stop_event = "[d]G = Q checked inside the descent (one-word arithmetic)".into();
    w.mapping = "one-target descent over a solved log table: building the walks and stepping them by G is target_query, keying each state and probing the pair table is target_PDP, the witness re-check is target_relation_check, combining the column logs into d is target_descent, and the descent's [d]G = Q is target_recovery_check; the relations and logs are target-independent set-up outside the window".into();
    (w.phases_ns.values().sum::<u64>() == w.wall_ns).then_some(w)
}

#[cfg(test)]
mod koblitz_batched_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::CurveSpec;

    fn method(batch: &str) -> ResolvedMethod {
        let params: BTreeMap<String, String> = [
            ("factor_base_points", "78"),
            ("batch_trials", batch),
            ("seed", "7"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        resolve(&MethodSpec {
            id: "ic.koblitz_batched".into(),
            params,
        })
        .unwrap()
    }

    fn solve_at(batch: &str, scalar: u64) -> SolveReport {
        let spec = CurveSpec::Koblitz { a: 0, n: 13 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let m = method(batch);
        applicable(&m, &inst, &facts).unwrap();
        let Instance::Binary(bi) = &inst else {
            unreachable!()
        };
        let target = bi.fast.mul_u64(bi.generator, scalar % bi.r);
        solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            3,
        )
        .unwrap()
    }

    #[test]
    fn the_tournament_cell_recovers_the_scalar_with_every_stage_charged() {
        let rep = solve_at("64", 1_234);
        assert_eq!(rep.recovered, Some(1_234));
        let names: Vec<&str> = rep.phases.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, koblitz_batched::PHASES);
        assert_eq!(rep.total_gae, rep.phases.iter().map(|p| p.gae).sum::<f64>());
        assert!(rep
            .phases
            .iter()
            .all(|p| p.gae == (p.adds + p.doubles) as f64));
        assert!(rep.phases.iter().all(|p| p.gae > 0.0), "{:?}", rep.phases);
        let online = rep.online.clone().expect("the descent's online window");
        assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
        // The trials are whole batches (the last may be cut by max_trials).
        assert_eq!(rep.counters["collection_trials"] % 64, 0);
        assert_eq!(
            rep.counters["solve_attempts"],
            rep.counters["collection_batches"]
        );
    }

    #[test]
    fn batch_trials_changes_the_rank_checks_and_nothing_else_about_the_relations() {
        // Same seed, same factor base, same probes: a smaller batch checks
        // the rank more often and may stop earlier, but every probe it
        // draws is a probe the larger batch draws too (the work units are
        // indexed by trial), so the relations of the shorter run are a
        // prefix of the longer one's.
        let wide = solve_at("64", 1_234);
        let narrow = solve_at("16", 1_234);
        assert_eq!(narrow.recovered, Some(1_234));
        assert!(narrow.counters["collection_trials"] <= wide.counters["collection_trials"]);
        assert!(narrow.counters["solve_attempts"] >= wide.counters["solve_attempts"]);
        assert_eq!(narrow.counters["collection_trials"] % 16, 0);
    }

    #[test]
    fn the_worker_limits_are_enforced() {
        let spec = CurveSpec::Koblitz { a: 0, n: 13 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let mut m = method("64");
        m.params.insert("batch_trials".into(), "5000".into());
        assert!(applicable(&m, &inst, &facts)
            .unwrap_err()
            .contains("collection limits"));
        let mut m = method("64");
        m.params.insert("max_trials".into(), "32".into());
        assert!(applicable(&m, &inst, &facts).is_err());
        let mut m = method("64");
        m.params.remove("factor_base_points");
        assert!(applicable(&m, &inst, &facts).is_err());
    }
}

#[cfg(test)]
mod koblitz_descent_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::CurveSpec;

    fn method(explicit: bool) -> ResolvedMethod {
        let params: BTreeMap<String, String> = if explicit {
            [
                ("descent_summands", "2"),
                ("collection_window", "64"),
                ("factor_base_points", "256"),
                ("unit_trials", "64"),
                ("units", "1"),
                ("max_trials", "100000"),
                ("seed", "7"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
        } else {
            BTreeMap::new()
        };
        resolve(&MethodSpec {
            id: "ic.koblitz_descent".into(),
            params,
        })
        .unwrap()
    }

    #[test]
    fn the_rule_v8_defaults_are_refused_off_the_six_sizes_and_explicit_values_solve() {
        let spec = CurveSpec::Koblitz { a: 0, n: 23 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let err = applicable(&method(false), &inst, &facts).unwrap_err();
        assert!(err.contains("rule_v8"), "{err}");
        let m = method(true);
        applicable(&m, &inst, &facts).unwrap();
        let Instance::Binary(bi) = &inst else {
            unreachable!()
        };
        let target = bi.fast.mul_u64(bi.generator, 777_777 % bi.r);
        let rep = solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            3,
        )
        .unwrap();
        assert_eq!(rep.recovered, Some(u128::from(777_777 % bi.r)));
        assert_eq!(rep.automorphisms_used, 46);
        let names: Vec<&str> = rep.phases.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, koblitz_descent::PHASES);
        assert_eq!(rep.total_gae, rep.phases.iter().map(|p| p.gae).sum::<f64>());
        assert!(rep
            .phases
            .iter()
            .all(|p| p.gae == (p.adds + p.doubles) as f64));
        // Every stage is charged.
        assert!(rep.phases.iter().all(|p| p.gae > 0.0), "{:?}", rep.phases);
        for label in [
            "canonicalisations_uncharged",
            "pair_filter_probes_uncharged",
            "la_row_ops_uncharged",
            "field_arithmetic_uncharged",
        ] {
            assert!(rep.unpriced.iter().any(|u| u == label), "{label}");
        }
        let online = rep.online.clone().expect("the descent's online window");
        assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
        assert_eq!(online.phases_ns.len(), 5);
        // Deterministic in (workload, seed): a second solve counts the same.
        let again = solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            3,
        )
        .unwrap();
        let ints = |r: &SolveReport| {
            r.phases
                .iter()
                .map(|p| {
                    (
                        p.name.clone(),
                        p.adds,
                        p.doubles,
                        p.scalar_mults,
                        p.native.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(ints(&rep), ints(&again));
        assert_eq!(rep.counters, again.counters);
        assert_eq!(rep.factor_base, again.factor_base);
    }

    #[test]
    fn the_explicit_recipe_applies_at_another_degree() {
        let spec = CurveSpec::Koblitz { a: 1, n: 17 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        // n = 17 builds, the explicit recipe applies.
        applicable(&method(true), &inst, &facts).unwrap();
    }
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
    if matches!(
        &inst,
        Instance::Fp4(_) | Instance::Fp4Torsion(_) | Instance::F2m4(_)
    ) {
        return Err("no factor-base plug-in runs on E(F_p⁴); ic.k4_gaudry builds its own".into());
    }
    if let Instance::Ghs(_) = &inst {
        return Err("no factor-base plug-in runs on a ghs_fp6 curve".into());
    }
    if let Instance::Genus2(_) | Instance::Quartic(_) = &inst {
        return Err(
            "no factor-base plug-in runs on a Jacobian; ic.hyperelliptic builds its own".into(),
        );
    }
    if let Instance::Ext(_) = &inst {
        return Err("no factor-base plug-in runs on a gls_fp2 or subfield_fp3 curve".into());
    }
    if let Instance::Hyper(_) = &inst {
        return Err("no factor-base plug-in runs on a hyperelliptic Jacobian".into());
    }
    if let Instance::Fp4Torsion(_) | Instance::F2m4(_) = &inst {
        return Err("factor-base dumps cover prime and binary curves".into());
    }
    match &inst {
        Instance::Prime(i) => {
            let abscissa = PrimeAbscissaBase { instance: i };
            let explicit = crate::cryptanalysis::ic_framework::explicit_base::PrimeExplicitXBase;
            let projected = PrimeProjectedAbscissaBase { instance: i };
            let orbit = GlvOrbitBase { instance: i };
            let kummer = PrimeKummerBase { instance: i };
            let isogeny = PrimeIsogenyBase { instance: i };
            let base: &dyn FactorBaseBuilder<_> = match name.as_str() {
                "explicit-x" => &explicit,
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
            let explicit = crate::cryptanalysis::ic_framework::explicit_base::BinaryExplicitXBase {
                instance: i,
            };
            let orbit = KoblitzOrbitBase { instance: i };
            let compact = CompactOrbitScanBase { instance: i };
            let cyclotomic = CyclotomicBase { instance: i };
            let onb_weight = OnbWeightBase { instance: i };
            let plugin_base = binary_plugins().and_then(|p| (p.base)(name.as_str(), i));
            let base: &dyn FactorBaseBuilder<BinaryGroup> = match name.as_str() {
                "explicit-x" => &explicit,
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
        Instance::Wide(_)
        | Instance::Fp4(_)
        | Instance::Fp4Torsion(_)
        | Instance::F2m4(_)
        | Instance::ThreeLeads(_)
        | Instance::Ext(_)
        | Instance::Genus2(_)
        | Instance::Quartic(_)
        | Instance::Ghs(_)
        | Instance::Hyper(_) => unreachable!("returned above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best2_model_registry_prime_recovery_and_replay() {
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        let spec = CurveSpec::PrimeSearch {
            bits: 14,
            seed: 59297,
        };
        for id in ["rho.plus_model_v1", "rho.best2_model_v1"] {
            let params = [
                ("jumps", "16"),
                ("walks", "8"),
                ("dp_bits", "2"),
                ("cap_multiple", "10000"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
            let m = resolve(&MethodSpec {
                id: id.into(),
                params,
            })
            .unwrap();
            for index in 0..4 {
                let (w, inst) =
                    Workload::build(&spec, 20261027, index, TargetKind::Planted).unwrap();
                let a = solve(&m, &inst, &w.curve, &w.target, index + 1).unwrap();
                let b = solve(&m, &inst, &w.curve, &w.target, index + 1).unwrap();
                assert_eq!(a.recovered, w.planted);
                assert_eq!(a.recovered, b.recovered);
                assert_eq!(a.counters, b.counters);
                assert_eq!(a.total_gae, b.total_gae);
                assert!(a.counters["physical_steps"] > 0);
            }
        }
    }

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
    fn historical_genus2_method_id_survives_thread_parameter_resolution() {
        let historical = resolve(&MethodSpec {
            id: HYPER_GENUS2_ID.into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        // Committed in research/ecbench_jacobians_transfers_20261025/sessions/genus2/plan.json.
        assert_eq!(historical.method_id, "ECM1ha6ff9c61b9eb");
        assert_eq!(historical.entry, HYPER_GENUS2_ENTRY);

        let thread = resolve(&MethodSpec {
            id: HYPER_IC.into(),
            params: [
                ("fb_size", "all"),
                ("large_primes", "0"),
                ("extra_relations", "8"),
                ("branches", "64"),
                ("d_step_in", "4"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        })
        .unwrap();
        assert_eq!(thread.entry, HYPER_THREAD_ENTRY);
        assert_ne!(thread.method_id, historical.method_id);
    }

    #[test]
    fn hyperelliptic_variants_refuse_other_workloads_and_mismatched_entries() {
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        let genus2 = resolve(&MethodSpec {
            id: HYPER_GENUS2_ID.into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        let thread = resolve(&MethodSpec {
            id: HYPER_IC.into(),
            params: [
                ("fb_size", "all"),
                ("large_primes", "0"),
                ("extra_relations", "8"),
                ("branches", "64"),
                ("d_step_in", "4"),
            ]
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
        })
        .unwrap();
        let cases = [
            CurveSpec::Genus2Search {
                p_bits: 8,
                seed: 3,
                max_cofactor: 16,
            },
            CurveSpec::HyperellipticThread { p: 23, genus: 3 },
            CurveSpec::PrimeSearch { bits: 12, seed: 3 },
        ];
        for spec in cases {
            let (w, inst) = Workload::build(&spec, 7, 0, TargetKind::Planted).unwrap();
            for method in [&genus2, &thread] {
                let matches = match method.id.as_str() {
                    HYPER_GENUS2_ID => matches!(inst, Instance::Genus2(_)),
                    HYPER_IC => matches!(inst, Instance::Hyper(_)),
                    _ => unreachable!(),
                };
                if matches {
                    assert!(applicable(method, &inst, &w.curve).is_ok());
                    let mut wrong_entry = method.clone();
                    wrong_entry.entry = "different implementation".into();
                    assert!(applicable(&wrong_entry, &inst, &w.curve).is_err());
                    assert!(solve(&wrong_entry, &inst, &w.curve, &w.target, 1).is_err());
                } else {
                    assert!(applicable(method, &inst, &w.curve).is_err());
                    assert!(solve(method, &inst, &w.curve, &w.target, 1).is_err());
                }
            }
        }
        assert!(resolve(&MethodSpec {
            id: HYPER_IC.into(),
            params: BTreeMap::new()
        })
        .is_err());
    }

    #[test]
    fn pooled_multitarget_rho_recovers_the_record_target_and_its_companions() {
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        let spec = CurveSpec::PrimeSearch { bits: 24, seed: 3 };
        for batch in ["1", "6"] {
            let mut params = BTreeMap::new();
            params.insert("batch".to_string(), batch.to_string());
            let m = resolve(&MethodSpec {
                id: "rho.multitarget_pooled".into(),
                params,
            })
            .unwrap();
            for index in 0..3 {
                let (w, inst) = Workload::build(&spec, 11, index, TargetKind::Planted).unwrap();
                let rep = solve(&m, &inst, &w.curve, &w.target, 40 + index).unwrap();
                assert_eq!(rep.recovered, w.planted, "record target");
                assert_eq!(rep.automorphisms_used, 2);
                let per = rep.detail["per_target"].as_array().unwrap();
                assert_eq!(per.len(), batch.parse::<usize>().unwrap());
                assert!(rep.detail["all_verified_internally"].as_bool().unwrap());
                // Each companion's log, checked here as well: [d]G = Q_j.
                let Instance::Prime(pi) = &inst else {
                    panic!("prime")
                };
                let comps = rep.detail["companions"].as_array().unwrap();
                for (t, q) in per.iter().zip(comps) {
                    let d = t["recovered"].as_u64().unwrap();
                    let mut ops = GroupOps::default();
                    let got = pi.curve.mul(&mut ops, pi.generator_point(), d);
                    let x = parse_hex(q[0].as_str().unwrap()).unwrap();
                    let y = parse_hex(q[1].as_str().unwrap()).unwrap();
                    assert_eq!(got, PrimePoint::affine(x, y));
                }
                // Deterministic in (workload, seed).
                let again = solve(&m, &inst, &w.curve, &w.target, 40 + index).unwrap();
                assert_eq!(again.total_gae, rep.total_gae);
            }
        }
        // `batch` has no default.
        assert!(resolve(&MethodSpec {
            id: "rho.multitarget_pooled".into(),
            params: BTreeMap::new(),
        })
        .is_err());
    }

    /// `rho.multitarget_pooled` implements the prime-field batch only: it is
    /// registered `PrimeOnly`, and both `applicable` (the planner's check)
    /// and `solve` refuse a binary instance instead of falling through to a
    /// generic solver with an unsupported method id.
    #[test]
    fn pooled_multitarget_rho_is_prime_only_and_refuses_binary_instances() {
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        assert_eq!(
            decl("rho.multitarget_pooled").unwrap().applies,
            Applies::PrimeOnly
        );
        let mut params = BTreeMap::new();
        params.insert("batch".to_string(), "4".to_string());
        let m = resolve(&MethodSpec {
            id: "rho.multitarget_pooled".into(),
            params,
        })
        .unwrap();
        // Negative controls: a Koblitz and a random binary curve.
        for spec in [
            CurveSpec::Koblitz { a: 1, n: 17 },
            CurveSpec::BinaryRandom {
                n: 17,
                seed: 3,
                max_cofactor: 4,
            },
        ] {
            let (w, inst) = Workload::build(&spec, 11, 0, TargetKind::Planted).unwrap();
            assert!(matches!(inst, Instance::Binary(_)));
            let planner = applicable(&m, &inst, &w.curve).unwrap_err();
            assert!(planner.contains("prime-field curves only"), "{planner}");
            let solver = solve(&m, &inst, &w.curve, &w.target, 1).unwrap_err();
            assert!(solver.contains("prime-field curves only"), "{solver}");
        }
        // Positive control: the prime curve it implements is accepted.
        let (w, inst) = Workload::build(
            &CurveSpec::PrimeSearch { bits: 24, seed: 3 },
            11,
            0,
            TargetKind::Planted,
        )
        .unwrap();
        assert!(applicable(&m, &inst, &w.curve).is_ok());
        assert!(solve(&m, &inst, &w.curve, &w.target, 1).is_ok());
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

    /// The quartic walk's class set-up (the scalar multiplication that
    /// fixes ι on the subgroup) is charged on its own ledger and merged
    /// in afterwards; the total and `S` must move with it, so the phases
    /// sum to the total the record reports.
    #[test]
    fn quartic_class_setup_is_in_the_total_and_the_phases_sum_to_it() {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        let m = resolve(&MethodSpec {
            id: "rho.quartic".into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        // A j = 1728 crater curve of research/prime_rho_automorphism_20261015.
        let spec = CurveSpec::PrimeExplicit {
            p: 2463541,
            a: 8,
            b: 0,
            group_order: 2460762,
            r: 136709,
            gx: 1664813,
            gy: 581781,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        applicable(&m, &inst, &facts).unwrap();
        let Instance::Prime(prime) = &inst else {
            unreachable!()
        };
        let mut scratch = GroupOps::default();
        let target = prime.curve.mul(&mut scratch, prime.generator_point(), 4321);
        let rep = solve(
            &m,
            &inst,
            &facts,
            &[format!("0x{:x}", target.x), format!("0x{:x}", target.y)],
            3,
        )
        .unwrap();
        assert_eq!(rep.recovered, Some(4321));
        let phases: f64 = rep.phases.iter().map(|p| p.gae).sum();
        assert_eq!(phases, rep.total_gae, "{:?}", rep.phases);
        let setup = rep.phases.iter().find(|p| p.name == "setup").unwrap();
        assert!(setup.gae > 0.0, "the class set-up is charged");
        // A curve without the structure is refused when planned, with the
        // message the solver would give on every execution.
        let plain = CurveSpec::PrimeSearch {
            bits: 12,
            seed: 59297,
        };
        let plain_inst = plain.build().unwrap();
        let err = applicable(&m, &plain_inst, &plain_inst.facts(&plain)).unwrap_err();
        assert!(err.contains("lacks the structure"), "{err}");
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

    /// `pkm-semaev` solves the ECDLP end to end on a prime curve: F4 on the
    /// Kummer-tower Semaev system decides every target, the grid reference
    /// on the same seed issues the same queries and finds the same
    /// relations, every F4 refutation is grid-checked with no disagreement,
    /// and F4's multiply-adds are on the record as unpriced solver work.
    #[test]
    fn pkm_semaev_solves_on_a_prime_curve_and_matches_the_grid() {
        use crate::cryptanalysis::ecbench::workload::CurveSpec;
        // a_minus_3_prime_order_curves 12 3 20261019 3: 8 | p − 1.
        let spec = CurveSpec::PrimeExplicit {
            p: 2393,
            a: 2390,
            b: 1614,
            group_order: 2377,
            r: 2377,
            gx: 0,
            gy: 80,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Prime(prime) = &inst else {
            unreachable!()
        };
        let mut scratch = GroupOps::default();
        let target = prime.curve.mul(&mut scratch, prime.generator_point(), 1234);
        let coords = [format!("0x{:x}", target.x), format!("0x{:x}", target.y)];
        let run = |base: &str, oracle: &str| {
            let m = resolve(&MethodSpec {
                id: "ic.pipeline".into(),
                params: [
                    ("factor_base".to_string(), base.to_string()),
                    ("oracle".to_string(), oracle.to_string()),
                ]
                .into_iter()
                .collect(),
            })
            .unwrap();
            solve(&m, &inst, &facts, &coords, 5)
        };
        let f4 = run("prime-kummer:t=2", "pkm-semaev:m=3,crosscheck=1").unwrap();
        let grid = run("prime-kummer:t=2", "pkm-semaev:m=3,engine=exhaustive").unwrap();
        assert_eq!(f4.recovered, Some(1234));
        assert_eq!(grid.recovered, f4.recovered);
        for key in ["targets_tried", "relations_found", "matrix_rank"] {
            assert_eq!(f4.counters[key], grid.counters[key], "{key}");
        }
        let rel = f4.phases.iter().find(|p| p.name == "relations").unwrap();
        assert_eq!(
            rel.native
                .get("pkm_crosscheck_disagreements")
                .copied()
                .unwrap_or(0),
            0
        );
        let solver = f4.solver.as_ref().expect("the F4 block");
        assert!(solver.ops > 0);
        assert!(
            f4.unpriced
                .iter()
                .any(|u| u.starts_with("solver_F_p_multiply-adds")),
            "{:?}",
            f4.unpriced
        );
        assert!(f4
            .unpriced
            .contains(&"pkm_system_fp_mults_uncharged".to_string()));
        assert!(grid
            .unpriced
            .contains(&"pkm_grid_fp_mults_uncharged".to_string()));
        // Every hit is re-added: at least one addition per relation in
        // the relations phase beyond the target draws.
        assert!(rel.adds >= f4.counters["relations_found"]);
        let err = run("prime-abscissa:size=8", "pkm-semaev:m=3").unwrap_err();
        assert!(err.contains("prime-kummer"), "{err}");
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
mod accounting_rules_tests {
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
                    if !n.is_multiple_of(h) {
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

    /// The class set-up `[κ]G` is in the total under both ids.  The frozen
    /// `rho.cubic` / `rho.quartic` have charged it since 2026-10-18 (it
    /// moves `total_gae` with the walk's ledger and the record's `detail`
    /// names it, for `audit::legacy_class_setup`); the `_v2` ids carry it as
    /// the `automorphism_setup` phase.  Both report phases that sum to the
    /// total.
    #[test]
    fn cubic_and_quartic_set_up_is_in_the_total_under_both_ids() {
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
            // The frozen id has no such phase: its set-up is in the total
            // (the phases sum to it) and in its `detail`, with the same
            // figure the v2 phase carries.
            assert!(frozen.phases.iter().all(|p| p.name != "automorphism_setup"));
            assert_phases_sum(old, &frozen);
            assert_eq!(
                frozen.detail["class_setup"]["gae"].as_f64(),
                Some(kappa.gae),
                "{old}: the detail names the set-up the v2 phase charges"
            );
            assert!(frozen.total_gae > kappa.gae);
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

    /// A kangaroo restart is neither set-up nor a jump: v2 charges it to
    /// `restarts`.  Frozen `kangaroo.vow` charges it to `search` (since
    /// 2026-10-18; before, to `setup`) and reports it only in `detail`.
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
                    assert_eq!(old.setup, new.setup);
                    // The frozen search is the v2 search plus the restarts.
                    assert_eq!(old.search.gae(), new.search.gae() + new.restarts.gae());
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
        // restart ledger is exactly what left `search`.
        // (`generic::tests::a_kangaroo_restart_is_its_own_ledger_when_kept_apart`
        // pins a group where every seed restarts.)
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

    /// The factor-base inventory of an `ic.pipeline` run describes the base
    /// the run built and used: [`solve_measured`] returns it, with no second
    /// build, and [`complete`] leaves it alone.  An `ic.shared_rank` run is
    /// the one whose inventory [`complete`] builds, after the measured solve.
    #[test]
    fn the_inventory_describes_the_base_the_run_used() {
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
            assert_eq!(measured.recovered, Some(k));
            let fb = measured
                .factor_base
                .clone()
                .expect("the run's own base is described");
            assert!(fb.columns > 0 && fb.fb_id.starts_with("FB1h"));
            let mut done = measured.clone();
            complete(&m, &inst, &facts, &target, &mut done).unwrap();
            assert_eq!(done.factor_base, measured.factor_base);
            assert_eq!(done.phases, measured.phases);
            let whole = solve(&m, &inst, &facts, &target, 5).unwrap();
            assert_eq!(whole.factor_base, done.factor_base);
            assert_eq!(whole.total_gae, done.total_gae);
        }
    }

    /// `ic.shared_rank`: the measured report has no inventory and
    /// [`complete`] adds it, equal to what [`solve`] returns.
    #[test]
    fn the_shared_rank_inventory_is_built_after_the_measured_solve() {
        let spec = CurveSpec::Koblitz { a: 1, n: 17 };
        let (inst, facts) = built(&spec);
        let m = method(
            "ic.shared_rank",
            &[
                (
                    "factor_base",
                    "compact-orbit-scan:columns=4,raw_x_cap=10000",
                ),
                ("rank_seed", "7"),
                ("rank_max_trials", "10000"),
                ("target_max_attempts", "64"),
            ],
        );
        let target = inst.mul_generator_hex(113).unwrap();
        let measured = solve_measured(&m, &inst, &facts, &target, 19).unwrap();
        assert!(measured.factor_base.is_none());
        let mut done = measured.clone();
        complete(&m, &inst, &facts, &target, &mut done).unwrap();
        assert_eq!(done.factor_base.as_ref().unwrap().columns, 4);
        let whole = solve(&m, &inst, &facts, &target, 19).unwrap();
        assert_eq!(whole.factor_base, done.factor_base);
        assert_eq!(whole.total_gae, measured.total_gae);
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
        // Pooled: 5/100 and 25/100 are 30/200 = 15%, flagged though one run
        // alone is under 10%.
        let (a, b) = (phases(5.0, 95.0), phases(25.0, 75.0));
        let f = pooled_row_flags("koblitz", 40.0, "rho", &[(&a, 100.0), (&b, 100.0)]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("15%"), "{f:?}");
        assert!(pooled_row_flags("koblitz", 40.0, "rho", &[(&a, 100.0)]).is_empty());
    }

    /// Every method of the accounting work, frozen and `_v2`, has a case
    /// below whose phases sum to its total (the frozen `rho.cubic` and
    /// `rho.quartic` included, since 2026-10-18); the other ids listed say
    /// where they are covered.  Methods registered later need no entry: the
    /// audit's phases-sum check holds every record they write.
    #[test]
    fn the_listed_methods_phases_sum_to_their_totals() {
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
        // The frozen `rho.cubic` / `rho.quartic` charge their class set-up
        // in the total since 2026-10-18 (records before that have phases
        // that exceed it, replayed through `audit::legacy_class_setup`), so
        // the current frozen ids sum as well, like their `_v2` ids.
        for (spec, id, v2) in [
            (j0(), "rho.cubic", "rho.cubic_v2"),
            (j1728(), "rho.quartic", "rho.quartic_v2"),
        ] {
            let (inst, facts) = built(&spec);
            let k = 4242 % facts.r;
            let rep = run(&method(id, &[]), &inst, &facts, k, 6);
            assert_eq!(rep.recovered, Some(k), "{id}");
            assert_phases_sum(id, &rep);
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
        // Registered on main, outside the accounting `_v2` work: each has
        // its own tests (this module, `wordram_price`, `hyper_ic`,
        // `tests/ecbench.rs`), and the audit's phases-sum check
        // (`audit::consistency_problems`) runs on every record any of them
        // writes, so a session of any of them fails if its phases do not sum.
        covered.extend([
            "bsgs.negation_table",
            "ic.hyperelliptic",
            "ic.hyperelliptic_thread", // hyper_ic tests
            "ic.known_log_mitm2_v1",
            "ic.koblitz_batched", // koblitz_batched tests
            "ic.koblitz_descent", // koblitz_descent tests
            "ic.pipeline_wordram",
            "ic.trace_line_2lp",
            "rho.best2_model_v1", // best2_model tests
            "rho.plus_model_v1",
            "rho.frobenius_orbit", // ext endomorphism walks on gls_fp2 / subfield_fp3
            "rho.gls",
            "rho.multitarget_pooled",
            "rho.signed_frobenius_strong_wordram", // wordram_price tests
            "rho.signed_frobenius_wordram",
            "transfer.ghs_cover", // Jacobians and transfers: tests/ecbench.rs
            "transfer.isogeny",
            "transfer.quartic_cover",
        ]);
        // Not exhaustive on purpose: methods keep being registered on main,
        // and the audit's phases-sum check holds every record any of them
        // writes.  What this list may not do is go stale: every id it names
        // is still registered.
        let unknown: Vec<&str> = covered
            .iter()
            .copied()
            .filter(|id| decl(id).is_none())
            .collect();
        assert!(unknown.is_empty(), "not registered: {unknown:?}");
    }
}

/// Accounting fixes of 2026-10-28: the `_v2` ids against their frozen ids,
/// and the plan-time refusals of arms `solve` would raise on every run.
#[cfg(test)]
mod accounting_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::spec::{plan, Spec, SPEC_SCHEMA};
    use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};

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

    /// Plan one arm on one curve; `Err` is the planner's message.
    fn plan_arm(curve: Value, id: &str, params: Value) -> Result<(), String> {
        let spec = Spec::from_json(
            &json!({
                "schema": SPEC_SCHEMA,
                "label": "accounting",
                "workloads": {"curves": [curve], "targets_per_curve": 1, "target_seed": 1},
                "arms": [{"name": "a", "role": "candidate", "method": {"id": id, "params": params}}],
                "measurement": {"rounds": 1, "seed": 9},
            })
            .to_string(),
        )?;
        plan(spec).map(|_| ())
    }

    fn strip_wall(mut r: SolveReport) -> SolveReport {
        r.solve_wall_ns = 0;
        for p in &mut r.phases {
            p.wall_ns = None;
        }
        r
    }

    fn phase<'a>(r: &'a SolveReport, name: &str) -> &'a PhaseRecord {
        r.phases
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("no phase {name}"))
    }

    #[test]
    fn v2_ids_are_registered_beside_the_frozen_ones() {
        for (v2, frozen) in [
            ("ic.hyperelliptic_v2", "ic.hyperelliptic"),
            ("transfer.quartic_cover_v2", "transfer.quartic_cover"),
            ("ic.cm_folded_native_v2", "ic.cm_folded_native_v1"),
            ("ic.cm_folded_recurrence_v2", "ic.cm_folded_recurrence_v1"),
            ("ic.cm_folded_native_d43_v2", "ic.cm_folded_native_d43_v1"),
        ] {
            let (a, b) = (decl(v2).expect(v2), decl(frozen).expect(frozen));
            assert_eq!(a.family, b.family);
            assert_eq!(
                a.params.iter().map(|p| p.name).collect::<Vec<_>>(),
                b.params.iter().map(|p| p.name).collect::<Vec<_>>()
            );
            assert_ne!(a.summary, b.summary, "{v2} says what it adds");
            // Neither id has a closed-form expected S.
            assert_eq!(expected_s(v2, 2), expected_s(frozen, 2));
        }
        // The frozen genus-2 identity still hashes to the committed one.
        assert_eq!(method(HYPER_GENUS2_ID, &[]).method_id, "ECM1ha6ff9c61b9eb");
    }

    /// `ic.hyperelliptic_v2` minus `ic.hyperelliptic` is exactly the charged
    /// verification and the tallied factor-base scan; the frozen id keeps
    /// its counts.
    #[test]
    fn hyperelliptic_v2_adds_exactly_the_verification_and_the_scan_tally() {
        let spec = CurveSpec::Genus2Search {
            p_bits: 8,
            seed: 3,
            max_cofactor: 16,
        };
        for index in 0..3 {
            let (w, inst) = Workload::build(&spec, 11, index, TargetKind::Planted).unwrap();
            let frozen = method(HYPER_GENUS2_ID, &[]);
            let v2 = method(HYPER_GENUS2_V2_ID, &[]);
            applicable(&v2, &inst, &w.curve).unwrap();
            let a = strip_wall(solve(&frozen, &inst, &w.curve, &w.target, 5).unwrap());
            let b = strip_wall(solve(&v2, &inst, &w.curve, &w.target, 5).unwrap());
            assert_eq!(a.recovered, w.planted);
            assert_eq!(b.recovered, a.recovered);
            // The frozen record keeps its three phases, its counters and its
            // `unpriced`, with no verification and no factor-base tally.
            let names =
                |r: &SolveReport| r.phases.iter().map(|p| p.name.clone()).collect::<Vec<_>>();
            assert_eq!(names(&a), ["setup", "relations", "linear_algebra"]);
            assert!(!a.counters.contains_key("factor_base_fp_muls_uncharged"));
            assert_eq!(
                a.unpriced,
                ["row_ops_uncharged", "smoothness_field_muls_uncharged"]
            );
            assert_eq!(
                names(&b),
                [
                    "factor_base",
                    "setup",
                    "relations",
                    "linear_algebra",
                    "verification"
                ]
            );
            // Every frozen phase and counter is unchanged in v2.
            for p in &a.phases {
                assert_eq!(phase(&b, &p.name), p);
            }
            for (k, v) in &a.counters {
                assert_eq!(b.counters[k], *v, "{k}");
            }
            let extra: Vec<_> = b
                .counters
                .keys()
                .filter(|k| !a.counters.contains_key(*k))
                .collect();
            assert_eq!(
                extra,
                ["factor_base_fp_muls_uncharged", "verification_jacobian_ops"]
            );
            assert!(b
                .unpriced
                .contains(&"factor_base_fp_muls_uncharged".to_string()));
            assert_eq!(b.unpriced.len(), a.unpriced.len() + 1);
            // The verification: [k]D₁ = D₂ twice on a success, each one
            // doubling per bit and one addition per set bit of k.
            let k = a.recovered.unwrap() as u64;
            let one = u64::from(64 - k.leading_zeros()) + u64::from(k.count_ones());
            let v = phase(&b, "verification");
            assert_eq!(v.adds + v.doubles, 2 * one, "k {k}");
            assert_eq!(b.counters["verification_jacobian_ops"], 2 * one);
            assert_eq!(v.gae, (2 * one) as f64);
            assert!(b.counters["factor_base_fp_muls_uncharged"] > 0);
            let fbp = phase(&b, "factor_base");
            assert_eq!(fbp.gae, 0.0);
            assert_eq!(
                fbp.native["factor_base_fp_muls_uncharged"],
                b.counters["factor_base_fp_muls_uncharged"]
            );
            // v2 minus frozen is the verification, nothing else.
            assert_eq!(b.total_gae - a.total_gae, v.gae);
            // The phases sum to the total.
            assert_eq!(b.total_gae, b.phases.iter().map(|p| p.gae).sum::<f64>());
            assert_eq!(a.total_gae, a.phases.iter().map(|p| p.gae).sum::<f64>());
        }
    }

    /// `transfer.quartic_cover_v2` minus `transfer.quartic_cover` is one
    /// counter, the root-search tally; every figure of the frozen id is
    /// unchanged and the record stops claiming nothing is unpriced.
    #[test]
    fn quartic_cover_v2_adds_exactly_the_root_search_tally() {
        let spec = CurveSpec::PrimeSearch { bits: 12, seed: 5 };
        let (w, inst) = Workload::build(&spec, 11, 0, TargetKind::Planted).unwrap();
        let frozen = method("transfer.quartic_cover", &[]);
        let v2 = method("transfer.quartic_cover_v2", &[]);
        applicable(&v2, &inst, &w.curve).unwrap();
        let a = strip_wall(solve(&frozen, &inst, &w.curve, &w.target, 7).unwrap());
        let b = strip_wall(solve(&v2, &inst, &w.curve, &w.target, 7).unwrap());
        assert_eq!(a.recovered, w.planted);
        assert_eq!(b.recovered, a.recovered);
        assert!(a.unpriced.is_empty(), "frozen: {:?}", a.unpriced);
        assert!(!a.counters.contains_key("root_search_field_muls_uncharged"));
        let tally = b.counters["root_search_field_muls_uncharged"];
        assert!(tally > 0);
        assert_eq!(b.unpriced, ["root_search_field_muls_uncharged"]);
        // The charged work is identical, phase by phase.
        assert_eq!(a.total_gae, b.total_gae);
        assert_eq!(a.phases.len(), b.phases.len());
        for (pa, pb) in a.phases.iter().zip(&b.phases) {
            assert_eq!(
                (&pa.name, pa.adds, pa.doubles, pa.gae),
                (&pb.name, pb.adds, pb.doubles, pb.gae)
            );
            for (k, v) in &pa.native {
                assert_eq!(pb.native[k], *v, "{}: {k}", pa.name);
            }
        }
        // The only difference in counters is the tally, and it is the sum of
        // the two phases that search for points.
        let mut stripped = b.counters.clone();
        stripped.remove("root_search_field_muls_uncharged");
        assert_eq!(stripped, a.counters);
        let by_phase: u64 = b
            .phases
            .iter()
            .filter_map(|p| p.native.get("root_search_field_muls_uncharged"))
            .sum();
        assert_eq!(by_phase, tally);
        assert_eq!(b.total_gae, b.phases.iter().map(|p| p.gae).sum::<f64>());
    }

    #[test]
    fn plan_refuses_endomorphism_rhos_on_the_wrong_curve() {
        let gls = json!({"kind": "gls_fp2", "p_bits": 14, "seed": 1, "max_cofactor": 64});
        let sub =
            json!({"kind": "subfield_fp3", "p_bits": 14, "seed": 1, "max_cofactor_ratio": 64});
        let prime = json!({"kind": "prime_search", "bits": 14, "seed": 59297});
        let kob = json!({"kind": "koblitz", "a": 1, "n": 17});
        // Controls: each runs on its own family.
        plan_arm(gls.clone(), "rho.gls", json!({})).unwrap();
        plan_arm(sub.clone(), "rho.frobenius_orbit", json!({})).unwrap();
        plan_arm(gls.clone(), "rho.negation", json!({})).unwrap();
        for (curve, id) in [
            (prime.clone(), "rho.gls"),
            (kob.clone(), "rho.gls"),
            (sub.clone(), "rho.gls"),
            (prime.clone(), "rho.frobenius_orbit"),
            (gls.clone(), "rho.frobenius_orbit"),
        ] {
            let err = plan_arm(curve.clone(), id, json!({})).unwrap_err();
            assert!(
                err.contains(id) && err.contains("curves only"),
                "{id} on {curve}: {err}"
            );
        }
        // An arm with no GLS or subfield implementation at all.
        let err = plan_arm(
            gls,
            "ic.pipeline",
            json!({"factor_base": "prime-abscissa:size=8", "oracle": "subtract:m=2"}),
        )
        .unwrap_err();
        assert!(err.contains("no gls_fp2 implementation"), "{err}");
    }

    #[test]
    fn plan_refuses_an_isogeny_inner_that_is_not_generic() {
        let prime = json!({"kind": "prime_search", "bits": 14, "seed": 59297});
        for inner in ["ic.pipeline", "rho.frozen_reference", "rho.gls"] {
            let err =
                plan_arm(prime.clone(), "transfer.isogeny", json!({"inner": inner})).unwrap_err();
            assert!(
                err.contains("`inner` must be a generic method") && err.contains(inner),
                "{inner}: {err}"
            );
        }
        let err = plan_arm(prime, "transfer.isogeny", json!({"ell": "5"})).unwrap_err();
        assert!(err.contains("`ell` must be 2 or 3"), "{err}");
    }

    #[test]
    fn plan_refuses_a_pooled_batch_outside_its_range() {
        let prime = json!({"kind": "prime_search", "bits": 24, "seed": 3});
        plan_arm(
            prime.clone(),
            "rho.multitarget_pooled",
            json!({"batch": "4"}),
        )
        .unwrap();
        plan_arm(
            prime.clone(),
            "rho.multitarget_pooled",
            json!({"batch": "65536"}),
        )
        .unwrap();
        for batch in ["0", "65537", "1000000"] {
            let err = plan_arm(
                prime.clone(),
                "rho.multitarget_pooled",
                json!({"batch": batch}),
            )
            .unwrap_err();
            assert!(
                err.contains("`batch` must be in 1..=65536"),
                "{batch}: {err}"
            );
        }
        let err = plan_arm(prime, "rho.multitarget_pooled", json!({"batch": "many"})).unwrap_err();
        assert!(err.contains("not an integer"), "{err}");
    }

    #[test]
    fn plan_refuses_cm_folded_where_the_adapter_would_fail() {
        // A random prime-order curve has no CM orbit action.
        let prime = json!({"kind": "prime_search", "bits": 20, "seed": 3});
        for id in [
            "ic.cm_folded_native_v1",
            "ic.cm_folded_native_v2",
            "ic.cm_folded_native_d43_v2",
            "ic.cm_folded_recurrence_v2",
        ] {
            let err = plan_arm(
                prime.clone(),
                id,
                json!({"backend": "/nonexistent/folded", "backend_sha256": "00", "query_limit": "1000"}),
            )
            .unwrap_err();
            assert!(
                err.contains("cofactor-one") || err.contains("no CM orbit action"),
                "{id}: {err}"
            );
        }
    }

    /// `rho.signed_frobenius_strong_wordram` is priced after running its
    /// base, so the planner must look through `wordram_price::base_of` as it
    /// does through `full_price::base_of`.
    #[test]
    fn plan_accepts_the_wordram_strong_rho_on_a_wide_koblitz_curve() {
        let kob = json!({"kind": "koblitz", "a": 0, "n": 83});
        let strong = plan_arm(kob.clone(), "rho.signed_frobenius_strong", json!({}));
        assert!(strong.is_ok(), "{strong:?}");
        let wordram = plan_arm(kob, "rho.signed_frobenius_strong_wordram", json!({}));
        assert!(wordram.is_ok(), "{wordram:?}");
    }
}

#[cfg(test)]
mod neg_strong_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::CurveSpec;

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

    fn f2m4(m: u32, seed: u64) -> (Instance, CurveFacts) {
        let spec = CurveSpec::F2m4Torsion {
            m,
            seed,
            max_cofactor: 16,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        (inst, facts)
    }

    #[test]
    fn recovers_and_agrees_with_rho_negation_over_several_seeds() {
        let strong = method(NEG_STRONG, &[("lanes", "8"), ("dp_bits", "auto")]);
        let plain = method("rho.negation", &[]);
        for m in 4..=6u32 {
            let (inst, facts) = f2m4(m, 1);
            applicable(&strong, &inst, &facts).unwrap();
            for seed in 0..4u64 {
                let k = (17 + 1009 * u128::from(seed) + u128::from(m)) % facts.r;
                let target = inst.mul_generator_hex(k).unwrap();
                let a = solve(&strong, &inst, &facts, &target, seed).unwrap();
                let b = solve(&plain, &inst, &facts, &target, seed).unwrap();
                assert_eq!(a.recovered, Some(k), "m = {m}, seed {seed}");
                assert_eq!(a.recovered, b.recovered, "m = {m}, seed {seed}");
                let sum: f64 = a.phases.iter().map(|p| p.gae).sum();
                assert_eq!(sum, a.total_gae);
                let per_add = a.counters["fq_muls_per_add"] as f64;
                let charged = a.counters["fq_muls"] as f64;
                assert!((a.total_gae - charged / per_add).abs() <= 1e-9 * a.total_gae);
                assert_eq!(
                    a.counters["fq_muls"],
                    a.counters["fq_muls_setup"]
                        + a.counters["fq_muls_search"]
                        + a.counters["fq_muls_check"]
                        + a.counters["fq_muls_canonicalisation"]
                );
                assert!(a
                    .unpriced
                    .contains(&"partition_hashes_uncharged".to_string()));
                // Replays bit for bit.
                let again = solve(&strong, &inst, &facts, &target, seed).unwrap();
                assert_eq!(a.counters, again.counters);
                assert_eq!(a.total_gae, again.total_gae);
            }
        }
    }

    #[test]
    fn refuses_other_curves_and_bad_parameters() {
        let strong = method(NEG_STRONG, &[("lanes", "8"), ("dp_bits", "auto")]);
        let spec = CurveSpec::PrimeSearch {
            bits: 14,
            seed: 59297,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        assert!(applicable(&strong, &inst, &facts).is_err());
        let (inst, facts) = f2m4(4, 1);
        let bad = method(NEG_STRONG, &[("lanes", "0"), ("dp_bits", "4")]);
        assert!(applicable(&bad, &inst, &facts).is_err());
        let bad = method(NEG_STRONG, &[("lanes", "4"), ("dp_bits", "0")]);
        assert!(applicable(&bad, &inst, &facts).is_err());
        // No default for the walk's shape.
        assert!(resolve(&MethodSpec {
            id: NEG_STRONG.into(),
            params: BTreeMap::new(),
        })
        .is_err());
    }
}

#[cfg(test)]
mod tensor_contraction_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::workload::CurveSpec;

    fn method() -> ResolvedMethod {
        resolve(&MethodSpec {
            id: "ic.tensor_contraction_v1".into(),
            params: [("m".to_string(), "2".to_string())].into_iter().collect(),
        })
        .unwrap()
    }

    #[test]
    fn recovers_known_logs_deterministically_and_charges_every_phase() {
        let spec = CurveSpec::PrimeSearch { bits: 10, seed: 1 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let m = method();
        applicable(&m, &inst, &facts).unwrap();
        let Instance::Prime(pi) = &inst else {
            unreachable!()
        };
        for (k, seed) in [(5u64, 1u64), (pi.r - 2, 2)] {
            let t = pi
                .curve
                .mul(&mut GroupOps::default(), pi.generator_point(), k);
            let target = [format!("0x{:x}", t.x), format!("0x{:x}", t.y)];
            let rep = solve(&m, &inst, &facts, &target, seed).unwrap();
            assert_eq!(rep.recovered, Some(u128::from(k)));
            let names: Vec<&str> = rep.phases.iter().map(|p| p.name.as_str()).collect();
            assert_eq!(names, TENSOR_PHASES);
            assert_eq!(rep.total_gae, rep.phases.iter().map(|p| p.gae).sum::<f64>());
            for ph in ["relation_target", "relation_search", "recovery_check"] {
                assert!(
                    rep.phases.iter().any(|p| p.name == ph && p.gae > 0.0),
                    "{ph}"
                );
            }
            for label in [
                "factor_base.fb_points_listed_uncharged",
                "relation_search.contraction_rows_uncharged",
                "linear_algebra.linalg_mulmod_uncharged",
            ] {
                assert!(rep.unpriced.iter().any(|u| u == label), "{label}");
            }
            let again = solve(&m, &inst, &facts, &target, seed).unwrap();
            assert_eq!(rep.counters, again.counters);
            assert_eq!(rep.total_gae, again.total_gae);
        }
    }

    #[test]
    fn needs_m_and_rejects_m_below_two() {
        assert!(resolve(&MethodSpec {
            id: "ic.tensor_contraction_v1".into(),
            params: BTreeMap::new(),
        })
        .is_err());
        let spec = CurveSpec::PrimeSearch { bits: 10, seed: 1 };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let Instance::Prime(pi) = &inst else {
            unreachable!()
        };
        let t = pi
            .curve
            .mul(&mut GroupOps::default(), pi.generator_point(), 3);
        let mut m = method();
        m.params.insert("m".into(), "1".into());
        let target = [format!("0x{:x}", t.x), format!("0x{:x}", t.y)];
        let err = solve(&m, &inst, &facts, &target, 1).unwrap_err();
        assert!(err.contains("m ≥ 2"), "{err}");
    }
}
