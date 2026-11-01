//! # `ecbench`: one harness for every ECDLP method.
//!
//! Pollard rho in its variants, baby-step giant-step, the kangaroo and
//! index calculus run here behind one interface, on the same single-target
//! workloads, charged in the same unit, under the same isolation, with
//! the same records.  `docs/ecbench/README.md` is the standard this
//! implements; `src/bin/ecbench.rs` is the command line.
//!
//! | module | what it holds |
//! |---|---|
//! | [`canonical`] | canonical JSON, SHA-256 identities, derived seeds |
//! | [`workload`] | curve constructions → ICV1-named single-target workloads |
//! | [`methods`] | the method registry and the one `solve` every method goes through |
//! | [`aps`] | bounded APS polynomial instrument with exclusive wall phases and explicit incomplete native pricing |
//! | [`generic`] | counted BSGS (three forms) and the kangaroo on any [`CountedGroup`] |
//! | [`fp4`] | `E(F_{p⁴})`: the curve, its counted group and `ic.k4_gaudry` (Gaudry `k = 4`) |
//! | [`hyper`] | the hyperelliptic thread's Jacobians (genus 2–4, `p < 256`) for the generic arms |
//! | [`hyper_ic`] | `ic.hyperelliptic`: the thread's index calculus on those Jacobians, counted |
//! | [`koblitz_descent`] | `ic.koblitz_descent`: the ledger §23 one-target Koblitz index calculus, charged by counted group operations |
//! | [`negation_strong`] | `rho.negation_strong`: batched negation-map rho on `E/F_{(2^m)^4}`, charged in counted `F_q` multiplications |
//! | [`claw`] | the four-point signed-Frobenius pair claw (aburan28/cryptanalysis#175) on a Koblitz curve |
//! | [`conductor_gap`] | CM crater/floor curve pairs with a certified prime endomorphism-conductor gap |
//! | [`spec`] | the experiment spec and its deterministic expansion |
//! | [`host`] | the host capsule and its environment class |
//! | [`isolation`] | CPU reservation, eviction, pinning, NUMA binding, kernel counters |
//! | [`legacy`] | the accounting of the past under which committed records still replay |
//! | [`record`] | the record, the child protocol, isolation grading |
//! | [`runner`] | sessions: interleaved measured children, sealed append-only records |
//! | [`signals`] | interruption: the host is restored and no child outlives the runner |
//! | [`compare`] | paired ratios with bootstrap intervals; wall time gated by level |
//! | [`resources`] | cold native-work, memory and PMU vectors without invented GAE prices |
//! | [`gpu`] | bounded CUDA stage runs and matched-workload GPU receipts with independent replay |
//! | [`calibration`] | empirical A/A coverage and contention-admission checks |
//! | [`claim`] | `vs_rho` claims: the IC1 identity, the claim report, the native checker |
//! | [`audit`] | re-derive a session from its files; replay runs exactly |
//! | [`db`] | SQL that loads sessions into the schema in `docs/ecbench/schema.sql` |
//! | [`fbm`] | FBM1: factor-base measurement identity (curve × FB1 × oracle × solver × instrument) |
//! | [`isolab`] | an `isolab.job/v1` that runs a spec on an independent lab worker |
//! | [`bounds`] | bound records: the fitted cost of a method on a domain, sealed and re-derivable |
//! | [`frontier`] | the Pareto frontier of a domain's bounds, and its page |
//! | [`challenge`] | challenges (frozen paired specs) and verdicts (how a frontier moves) |
//!
//! [`CountedGroup`]: crate::cryptanalysis::ic_boundary::CountedGroup

pub mod aps;
pub mod audit;
pub mod band;
pub mod best2_model;
pub mod bounds;
pub mod calibration;
pub mod callgrind;
pub mod canonical;
pub mod challenge;
pub mod claim;
pub mod claw;
pub mod cm_folded;
pub mod compare;
pub mod conductor_gap;
pub mod cover_quartic;
pub mod db;
pub mod diagnostic_binding_core;
pub mod diagnostic_file_adapter;
pub mod diagnostic_inventory;
pub mod diagnostic_launch;
pub mod diagnostic_preflight;
pub mod diagnostic_resources;
pub mod diagnostic_runtime;
pub mod diagnostics;
pub mod ext;
pub mod fbm;
pub mod fp4;
pub mod frontier;
pub mod full_price;
pub mod generic;
pub mod ghs_cover;
pub mod gpu;
pub mod host;
pub mod hyper;
pub mod hyper_ic;
pub mod isogeny;
pub mod isolab;
pub mod isolation;
pub mod jacobian;
pub mod known_log;
pub mod koblitz_descent;
pub mod legacy;
pub mod methods;
pub mod negation_strong;
pub mod plane_quartic;
pub mod prime_orbit;
pub mod record;
pub mod resources;
pub mod runner;
pub mod signals;
pub mod spec;
pub mod stats;
pub mod three_leads;
pub mod trend;
pub mod wordram_price;
pub mod work;
pub mod workload;
