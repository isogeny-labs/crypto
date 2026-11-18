//! Signature stub of the crypto_lib items traced_descent_check.rs uses,
//! copied from the frozen-library overlay (nagao-e2e/library-overlay/
//! jv_cover.rs) and src/cryptanalysis/{f4_fp,jv_bitops}.rs. Type-check only;
//! every body is unimplemented. Not a library.
#![allow(dead_code, unused_variables, clippy::all)]
pub mod cryptanalysis {
    pub mod f4_fp {
        pub type Poly = Vec<(Vec<u32>, u64)>;
        #[derive(Clone, Debug)]
        pub struct F4Options {
            pub max_degree: u32,
        }
        #[derive(Clone, Debug, Default)]
        pub struct TraceStep {
            pub degree: u32,
            pub rows: Vec<(usize, Vec<u32>)>,
            pub new_lms: Vec<Vec<u32>>,
        }
        #[derive(Clone, Debug, Default)]
        pub struct F4Trace {
            pub input_lms: Vec<Vec<u32>>,
            pub steps: Vec<TraceStep>,
            pub stopped: bool,
        }
        #[derive(Clone, Debug)]
        pub struct F4Report {
            pub basis: Vec<Poly>,
            pub inconsistent: bool,
            pub degree_reached: u32,
            pub solving_degree: u32,
            pub steps: usize,
            pub max_rows: usize,
            pub max_cols: usize,
            pub ms: f64,
            pub timed_out: bool,
            pub pairs_above_bound: usize,
            pub staircase_at_stop: Option<usize>,
            pub field_ops: u64,
            pub trace_mismatch: bool,
        }
        pub fn f4(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options) -> F4Report {
            unimplemented!()
        }
        pub fn f4_record(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options) -> (F4Report, F4Trace) {
            unimplemented!()
        }
        pub fn f4_replay(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options, trace: &F4Trace) -> F4Report {
            unimplemented!()
        }
    }
    pub mod jv_bitops {
        #[derive(Clone, Copy)]
        pub enum Phase {
            Descent,
        }
        pub struct OpTally;
        #[derive(Default)]
        pub struct JvTally;
        impl JvTally {
            pub fn phase(&self, phase: Phase) -> &OpTally {
                unimplemented!()
            }
            pub fn since(&self, before: &JvTally) -> JvTally {
                unimplemented!()
            }
        }
        pub struct Widths;
        #[derive(Debug)]
        pub struct NoPrice;
        impl std::fmt::Display for NoPrice {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                unimplemented!()
            }
        }
        pub struct PhaseGuard;
        pub fn claim() {}
        pub fn set_moduli(p: u64, l: u64) {}
        pub fn enable(on: bool) {}
        pub fn snapshot() -> JvTally {
            unimplemented!()
        }
        pub fn enter(phase: Phase) -> PhaseGuard {
            unimplemented!()
        }
        pub fn widths(p: u64, l: u64) -> Widths {
            unimplemented!()
        }
        pub fn price(t: &OpTally, w: &Widths) -> Result<u128, NoPrice> {
            unimplemented!()
        }
        pub fn uniform_mod_u64(rng: &mut rand::rngs::StdRng, low: u64, high: u64) -> u64 {
            unimplemented!()
        }
    }
    pub mod jv_cover {
        use super::f4_fp::{self, F4Options};
        use rand::rngs::StdRng;
        use std::collections::HashMap;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct E2(pub [u64; 2]);
        impl E2 {
            pub const ZERO: E2 = E2([0, 0]);
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct E6(pub [E2; 3]);
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PtE6 {
            pub x: E6,
            pub y: E6,
            pub inf: bool,
        }
        pub type Poly<E> = Vec<E>;
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Div<E> {
            pub u: Poly<E>,
            pub v: Poly<E>,
        }
        pub struct Fq;
        pub struct Fq3 {
            pub f: Fq,
        }
        pub struct Cover {
            pub hx: Vec<E2>,
        }
        pub struct ReIm;
        pub struct Hyp<'a, F> {
            pub f: &'a F,
        }
        impl<'a, F> Hyp<'a, F> {
            pub fn add(&self, a: &Div<E2>, b: &Div<E2>) -> Div<E2> {
                unimplemented!()
            }
            pub fn mul(&self, a: &Div<E2>, k: u128) -> Div<E2> {
                unimplemented!()
            }
        }
        pub struct Spec {
            pub p: u64,
            pub l: u64,
            pub gj: Div<E2>,
            pub qj: Div<E2>,
        }
        pub struct Ctx {
            pub f: Fq3,
            pub cov: Cover,
            pub reim: ReIm,
            pub trace: bool,
        }
        impl Ctx {
            pub fn new(spec: &Spec) -> Ctx {
                unimplemented!()
            }
            pub fn jac(&self) -> Hyp<'_, Fq> {
                unimplemented!()
            }
        }
        pub struct BaseEl {
            pub x: u64,
            pub y: E2,
            pub d: Div<E2>,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
        pub struct Dec6 {
            pub terms: [(usize, i8); 6],
        }
        #[derive(Clone, Debug, Default)]
        pub struct NagaoCost {
            pub weil_muls: u64,
            pub f4_muls: u64,
            pub lin_muls: u64,
            pub post_muls: u64,
            pub timed_out: bool,
            pub incomplete: bool,
            pub replayed: bool,
            pub trace_mismatch: bool,
        }
        #[derive(Clone, Debug, Default)]
        pub struct ZeroDimStats {
            pub degree_reached: u32,
            pub solving_degree: u32,
            pub max_rows: usize,
            pub max_cols: usize,
            pub f4_muls: u64,
            pub lin_muls: u64,
            pub delta: usize,
            pub inconsistent: bool,
            pub incomplete: bool,
            pub timed_out: bool,
            pub nonseparating: usize,
            pub lin_fallback: usize,
            pub f4_ms: f64,
            pub lin_ms: f64,
            pub stopped_at: Option<usize>,
            pub hilbert_applied: bool,
            pub hilbert_fallback: bool,
            pub hilbert_check_muls: u64,
            pub replayed: bool,
            pub trace_mismatch: bool,
        }
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum LinMode {
            Dense,
            Sparse,
            L1a,
            Fast,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum StopMode {
            Full,
            Staircase(usize),
            Hilbert(usize),
        }
        pub struct WeilForms;
        impl WeilForms {
            pub fn polys(&self, ri: &ReIm, p: u64) -> Vec<f4_fp::Poly> {
                unimplemented!()
            }
        }
        pub type NagaoSolver = fn(
            &[f4_fp::Poly],
            u64,
            &F4Options,
            &mut StdRng,
        ) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str>;
        pub fn weil_forms(f: &Fq, hx: &[E2], u: &[E2], v: &[E2]) -> WeilForms {
            unimplemented!()
        }
        pub fn spec_from_curve(p: u64, alpha: E6, l: u64, g: PtE6, q: PtE6, d: u64) -> Option<Spec> {
            unimplemented!()
        }
        pub fn factor_base(ctx: &Ctx, rng: &mut StdRng) -> Vec<BaseEl> {
            unimplemented!()
        }
        pub fn opts_for_mode(max_degree: u32, budget_secs: f64, mode: StopMode) -> F4Options {
            unimplemented!()
        }
        pub fn verify_dec(jac: &Hyp<Fq>, base: &[BaseEl], r: &Div<E2>, d: &Dec6) -> bool {
            unimplemented!()
        }
        pub fn solve_zero_dim_mode(
            input: &[f4_fp::Poly],
            n: usize,
            p: u64,
            opts: &F4Options,
            rng: &mut StdRng,
            trace: bool,
            mode: LinMode,
        ) -> (Vec<Vec<u64>>, ZeroDimStats) {
            unimplemented!()
        }
        #[allow(clippy::too_many_arguments)]
        pub fn linear_stage_from_basis(
            basis: &[f4_fp::Poly],
            input: &[f4_fp::Poly],
            n: usize,
            p: u64,
            rng: &mut StdRng,
            mode: LinMode,
            st: &mut ZeroDimStats,
            t_lin: std::time::Instant,
        ) -> Vec<Vec<u64>> {
            unimplemented!()
        }
        pub fn nagao_decompose_using(
            ctx: &Ctx,
            base: &[BaseEl],
            by_x: &HashMap<u64, usize>,
            r: &Div<E2>,
            opts: &F4Options,
            rng: &mut StdRng,
            solver: Option<NagaoSolver>,
        ) -> Result<(Vec<Dec6>, NagaoCost), &'static str> {
            unimplemented!()
        }
    }
}
