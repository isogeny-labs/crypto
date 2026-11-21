//! Opt-in traced Nagao descent finder and its exact local control.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/descent-finder.
//! Status: built and run locally against `mini-build/` (the frozen fcda1e22
//! descent closure plus the lane's nagao-e2e jv_cover overlay, with six
//! byte-copied helper shims; see README), not against a full Cargo build of
//! the crate. Counted work only; no timing claim. [pcc:unverified]
//!
//! What it does. The descent of the deferred graph pipeline
//! (`jv_extension_deferred_pair_dlp.rs`, DescentBackend::Original) calls
//! `cover::nagao_decompose` on each residual R_i = a_i*G_J + b_i*Q_J with F4
//! stopped at the 64-monomial staircase, without trace replay
//! (`Ctx::new` sets `trace = false`; every recorded run in this lane has
//! `descent_replayed = 0`). The candidate keeps the residual stream, the
//! Weil system, the stopping rule, the linear stage and the sextic/ordinate
//! materializer unchanged and replaces only the F4 call by Joux-Vitse's
//! trace replay ("A variant of the F4 algorithm", CT-RSA 2012), behind the
//! existing `NagaoSolver` callback, i.e. as an opt-in path. The default
//! pipeline is not edited.
//!
//! Acceptance rule (the exact correctness condition). Every row a replay
//! reduces is a monomial multiple of an input or of an earlier basis
//! element, so every polynomial the replay returns lies in the input ideal
//! I. A replayed report is accepted iff it did not time out, dropped no pair
//! above the degree bound, and either (a) derived 1 (then V(I) is empty:
//! exact), or (b) stopped at a staircase of at most 64 standard monomials -
//! the same certificate the production arm relies on (J <= I and
//! dim R/J <= 64 = dim R/I for a system with no solution at infinity; for a
//! degenerate system, neither arm is certified, which is why the oracle arm
//! exists). Anything else - a replay that returned early on a shape
//! mismatch (staircase_at_stop == None), a timeout, a truncation - is
//! rejected and the system is re-solved by ordinary staircase F4, with the
//! rejected replay's work charged. The trace is learned from the first
//! certified staircase-stopped system; the learning run's tracked RREF is
//! not counted by f4_fp, so an explicit upper bound is charged here.
//!
//! Arms, on identical residuals and identical per-residual RNG seeds:
//!   production : solve_zero_dim_mode(trace = false), staircase 64 (as Run50)
//!   traced     : this file's `traced_solver`
//!   oracle     : solve_zero_dim_mode, StopMode::Full (certified basis, no stop)
//! Checks per residual: the three F_p-solution sets are identical; every
//! solution satisfies all six input polynomials under this file's own u128
//! evaluator; the three decomposition lists are identical; every
//! decomposition passes `verify_dec` (shared library Jacobian - not
//! independent algebraic geometry); the residual equals a*G_J + b*Q_J.
//! Counted work old vs new: F4 field ops (report counters), linear-stage
//! muls, NagaoCost, and the jv_bitops descent-phase tally priced in bitop/v1.
//!
//! Build (needs a crypto_lib rlib; mirror nagao-e2e/build-native.sh):
//!   rustc --edition=2021 -C opt-level=3 --extern crypto_lib=... \
//!     --extern rand=... --extern serde_json=... traced_descent_check.rs
//! Run: traced_descent_check PUBLIC_JSON SEED N OUT_JSONL
//!   (Run50 deferred arm: public.json of runpod-deferred-pair-20261117-50,
//!    SEED 389, N 3415.)
use crypto_lib::cryptanalysis::{
    f4_fp::{self, F4Options, F4Report, F4Trace, Poly},
    jv_bitops::{self as work, Phase},
    jv_cover::{
        self as cover, Ctx, Dec6, LinMode, NagaoCost, StopMode, ZeroDimStats, E2, E6, PtE6,
    },
};
use rand::{rngs::StdRng, SeedableRng};
use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc, time::Instant};

/// Six unknowns, six quadrics (jv_cover::weil_forms).
const VARS: usize = 6;
/// Bezout bound of six quadrics in six unknowns; the production stop.
const STAIRCASE: usize = 64;

#[derive(Clone, Debug, Default)]
struct TraceTally {
    learned: u64,
    replay_attempts: u64,
    accepted: u64,
    accepted_after_mismatch: u64,
    rejected: u64,
    fallback_f4_ops: u64,
    rejected_replay_f4_ops: u64,
    /// Upper bound charged for the uncounted tracked RREF of each learning run.
    learning_rref_bound: u64,
    /// Traced arm only: jv_bitops-hooked bitops seen inside the linear stage,
    /// and the stage's own lin_muls counter, to test whether it is hooked.
    linear_stage_hooked_bitops: u128,
    linear_stage_lin_muls: u128,
}

thread_local! {
    static TRACE: RefCell<Option<Rc<F4Trace>>> = const { RefCell::new(None) };
    static TALLY: RefCell<TraceTally> = RefCell::new(TraceTally::default());
    static LAST_SOLS: RefCell<Vec<Vec<u64>>> = const { RefCell::new(Vec::new()) };
    static WIDTHS: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
}

/// The certificate both staircase arms rely on, plus `1 in I`.
fn certified(r: &F4Report) -> bool {
    !r.timed_out
        && r.pairs_above_bound == 0
        && (r.inconsistent || r.staircase_at_stop.is_some_and(|c| c <= STAIRCASE))
}

/// tracked_rref_contributors (f4_fp.rs) is a dense Gauss-Jordan with a
/// contributor bitset per row; per step it does at most rows*cols*rank
/// multiply-adds plus rows*rank*ceil(rows/64) word ORs. Charged as a bound.
fn learning_bound(r: &F4Report) -> u64 {
    let (rows, cols) = (r.max_rows as u64, r.max_cols as u64);
    let rank = rows.min(cols);
    (r.steps as u64).saturating_mul(rows * cols * rank + rows * rank * rows.div_ceil(64))
}

fn finish(
    r: F4Report,
    charged_f4: u64,
    input: &[Poly],
    p: u64,
    rng: &mut StdRng,
    replayed: bool,
    mismatch: bool,
) -> (Vec<Vec<u64>>, ZeroDimStats) {
    let t_lin = Instant::now();
    let mut st = ZeroDimStats {
        f4_muls: charged_f4,
        degree_reached: r.degree_reached,
        solving_degree: r.solving_degree,
        max_rows: r.max_rows,
        max_cols: r.max_cols,
        f4_ms: r.ms,
        stopped_at: r.staircase_at_stop,
        replayed,
        trace_mismatch: mismatch,
        ..ZeroDimStats::default()
    };
    if r.timed_out {
        st.timed_out = true;
        return (Vec::new(), st);
    }
    if r.inconsistent {
        st.inconsistent = true;
        return (Vec::new(), st);
    }
    if r.pairs_above_bound > 0 {
        st.incomplete = true;
        return (Vec::new(), st);
    }
    // Diagnostic: how much of the linear stage the jv_bitops hooks see.
    let before = work::snapshot();
    let sols =
        cover::linear_stage_from_basis(&r.basis, input, VARS, p, rng, LinMode::Fast, &mut st, t_lin);
    let seen = work::snapshot().since(&before);
    let (p_, l_) = WIDTHS.with(|w| w.get());
    let priced = work::price(seen.phase(Phase::Descent), &work::widths(p_, l_)).unwrap_or(0);
    TALLY.with(|t| {
        let mut t = t.borrow_mut();
        t.linear_stage_hooked_bitops += priced;
        t.linear_stage_lin_muls += st.lin_muls as u128;
    });
    (sols, st)
}

/// The opt-in solver (a `cover::NagaoSolver`). Deterministic given the
/// residual order; one trace per thread (the pipeline's descent is serial).
fn traced_solver(
    input: &[Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str> {
    let have = TRACE.with(|t| t.borrow().clone());
    let out = match have {
        Some(trace) => {
            let r = f4_fp::f4_replay(input, VARS, p, opts, &trace);
            TALLY.with(|t| t.borrow_mut().replay_attempts += 1);
            if certified(&r) {
                let mismatch = r.trace_mismatch;
                TALLY.with(|t| {
                    let mut t = t.borrow_mut();
                    t.accepted += 1;
                    t.accepted_after_mismatch += u64::from(mismatch);
                });
                let ops = r.field_ops;
                finish(r, ops, input, p, rng, true, mismatch)
            } else {
                let rejected = r.field_ops;
                let full = f4_fp::f4(input, VARS, p, opts);
                TALLY.with(|t| {
                    let mut t = t.borrow_mut();
                    t.rejected += 1;
                    t.rejected_replay_f4_ops += rejected;
                    t.fallback_f4_ops += full.field_ops;
                });
                let ops = rejected + full.field_ops;
                finish(full, ops, input, p, rng, false, true)
            }
        }
        None => {
            let (r, trace) = f4_fp::f4_record(input, VARS, p, opts);
            let bound = learning_bound(&r);
            TALLY.with(|t| {
                let mut t = t.borrow_mut();
                t.learned += 1;
                t.learning_rref_bound += bound;
            });
            if certified(&r) && trace.stopped && !r.inconsistent {
                TRACE.with(|t| *t.borrow_mut() = Some(Rc::new(trace)));
            }
            let ops = r.field_ops;
            finish(r, ops, input, p, rng, false, false)
        }
    };
    if out.0.iter().any(|s| s.len() != VARS || s.iter().any(|&x| x >= p)) {
        return Err("noncanonical traced root");
    }
    LAST_SOLS.with(|s| *s.borrow_mut() = out.0.clone());
    Ok(out)
}

fn production_solver(
    input: &[Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str> {
    let out = cover::solve_zero_dim_mode(input, VARS, p, opts, rng, false, LinMode::Fast);
    LAST_SOLS.with(|s| *s.borrow_mut() = out.0.clone());
    Ok(out)
}

/// Own evaluator: sum c * prod x_i^e_i mod p in u128.
fn eval(poly: &Poly, x: &[u64], p: u64) -> u64 {
    let mut acc: u128 = 0;
    for (mono, c) in poly {
        let mut t = *c as u128 % p as u128;
        for (i, &e) in mono.iter().enumerate() {
            for _ in 0..e {
                t = t * x[i] as u128 % p as u128;
            }
        }
        acc = (acc + t) % p as u128;
    }
    acc as u64
}

#[derive(Default, Clone)]
struct ArmTotals {
    f4_ops: u128,
    lin_muls: u128,
    weil_muls: u128,
    post_muls: u128,
    descent_bitops: u128,
    solutions: u64,
    decompositions: u64,
    timeouts: u64,
    incomplete: u64,
}

struct Arm {
    name: &'static str,
    opts: F4Options,
    solver: cover::NagaoSolver,
    totals: ArmTotals,
}

fn add_mod(a: u64, b: u64, l: u64) -> u64 {
    ((a as u128 + b as u128) % l as u128) as u64
}

/// One worker thread, as nagao-e2e/native/worker.rs: jv_bitops tallies are
/// thread-local, so F4's rayon row reduction must run on the claimed thread
/// or its primitives are lost from the descent tally.
fn main() -> Result<(), String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    pool.install(run)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: traced_descent_check PUBLIC_JSON SEED N OUT_JSONL".into());
    }
    let public: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let n: u64 = args[3].parse().map_err(|_| "N")?;
    let e2 = |v: &serde_json::Value| -> Result<E2, String> {
        let a = v.as_array().ok_or("e2")?;
        Ok(E2([a[0].as_u64().ok_or("e2")?, a[1].as_u64().ok_or("e2")?]))
    };
    let e6 = |v: &serde_json::Value| -> Result<E6, String> {
        let a = v.as_array().ok_or("e6")?;
        Ok(E6([e2(&a[0])?, e2(&a[1])?, e2(&a[2])?]))
    };
    let pt = |v: &serde_json::Value| -> Result<PtE6, String> {
        Ok(PtE6 { x: e6(&v["x"])?, y: e6(&v["y"])?, inf: v["inf"].as_bool().ok_or("inf")? })
    };
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    // Spec.d = 0: the checker never sees an answer.
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let widths = work::widths(p, l);
    WIDTHS.with(|w| w.set((p, l)));
    let ctx = Ctx::new(&spec);
    assert!(!ctx.trace, "baseline context must not replay traces");
    // Same derivations as jv_extension_deferred_pair_dlp.rs.
    let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let jac = ctx.jac();
    let mut stream = StdRng::seed_from_u64(seed ^ 0x51E7E);
    let al = work::uniform_mod_u64(&mut stream, 1, l);
    let be = work::uniform_mod_u64(&mut stream, 1, l);
    let step = jac.add(&jac.mul(&spec.gj, al as u128), &jac.mul(&spec.qj, be as u128));
    let mut a = work::uniform_mod_u64(&mut stream, 0, l);
    let mut b = work::uniform_mod_u64(&mut stream, 1, l);
    let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let staircase = cover::opts_for_mode(24, 600.0, StopMode::Staircase(STAIRCASE));
    let full = cover::opts_for_mode(24, 600.0, StopMode::Full);
    let mut arms = [
        Arm { name: "production", opts: staircase.clone(), solver: production_solver, totals: ArmTotals::default() },
        Arm { name: "traced", opts: staircase, solver: traced_solver, totals: ArmTotals::default() },
        Arm { name: "oracle", opts: full, solver: production_solver, totals: ArmTotals::default() },
    ];
    let mut out = std::io::BufWriter::new(
        std::fs::OpenOptions::new().write(true).create_new(true).open(&args[4]).map_err(|e| e.to_string())?,
    );
    let (mut disagreements, mut bad_solutions, mut bad_decompositions, mut bad_residuals) = (0u64, 0u64, 0u64, 0u64);
    let mut decomposing = Vec::new();
    for i in 0..n {
        if jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128)) != residual {
            bad_residuals += 1;
        }
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let input = cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &residual.u, &v).polys(&ctx.reim, p);
        let mut results: Vec<(Vec<Vec<u64>>, Vec<Dec6>, NagaoCost)> = Vec::new();
        for arm in arms.iter_mut() {
            let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let before = work::snapshot();
            let (decs, cost) = {
                let _d = work::enter(Phase::Descent);
                cover::nagao_decompose_using(&ctx, &base, &by_x, &residual, &arm.opts, &mut rng, Some(arm.solver))?
            };
            let tally = work::snapshot().since(&before);
            let priced = work::price(tally.phase(Phase::Descent), &widths).map_err(|e| e.to_string())?;
            let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
            sols.sort();
            let t = &mut arm.totals;
            t.f4_ops += cost.f4_muls as u128;
            t.lin_muls += cost.lin_muls as u128;
            t.weil_muls += cost.weil_muls as u128;
            t.post_muls += cost.post_muls as u128;
            t.descent_bitops += priced;
            t.solutions += sols.len() as u64;
            t.decompositions += decs.len() as u64;
            t.timeouts += u64::from(cost.timed_out);
            t.incomplete += u64::from(cost.incomplete);
            for s in &sols {
                if input.iter().any(|f| eval(f, s, p) != 0) {
                    bad_solutions += 1;
                }
            }
            for d in &decs {
                if !cover::verify_dec(&jac, &base, &residual, d) {
                    bad_decompositions += 1;
                }
            }
            results.push((sols, decs, cost));
        }
        let agree = results.windows(2).all(|w| w[0].0 == w[1].0 && w[0].1 == w[1].1);
        disagreements += u64::from(!agree);
        if !results[0].1.is_empty() {
            decomposing.push(i);
        }
        let row = serde_json::json!({
            "residual_index": i, "a": a, "b": b, "agree": agree,
            "solutions": results.iter().map(|r| r.0.len()).collect::<Vec<_>>(),
            "decompositions": results.iter().map(|r| &r.1).collect::<Vec<_>>(),
            "f4_ops": results.iter().map(|r| r.2.f4_muls).collect::<Vec<_>>(),
            "lin_muls": results.iter().map(|r| r.2.lin_muls).collect::<Vec<_>>(),
            "traced_replayed": results[1].2.replayed, "traced_mismatch": results[1].2.trace_mismatch,
        });
        writeln!(out, "{row}").map_err(|e| e.to_string())?;
        residual = jac.add(&residual, &step);
        a = add_mod(a, al, l);
        b = add_mod(b, be, l);
    }
    let tally = TALLY.with(|t| t.borrow().clone());
    let totals: Vec<_> = arms
        .iter()
        .map(|arm| {
            let t = &arm.totals;
            serde_json::json!({"arm": arm.name, "f4_ops": t.f4_ops.to_string(), "lin_muls": t.lin_muls.to_string(),
                "weil_muls": t.weil_muls.to_string(), "post_muls": t.post_muls.to_string(),
                "hooked_descent_bitops": t.descent_bitops.to_string(), "solutions": t.solutions,
                "decompositions": t.decompositions, "timeouts": t.timeouts, "incomplete": t.incomplete})
        })
        .collect();
    let summary = serde_json::json!({
        "schema": "descent-finder.traced-check/v1", "pcc": "unverified", "seed": seed, "residuals": n,
        "arms": totals, "trace": {"learned": tally.learned, "replay_attempts": tally.replay_attempts,
            "accepted": tally.accepted, "accepted_after_mismatch": tally.accepted_after_mismatch,
            "rejected": tally.rejected, "rejected_replay_f4_ops": tally.rejected_replay_f4_ops,
            "fallback_f4_ops": tally.fallback_f4_ops,
            "learning_rref_upper_bound_uncounted_by_f4fp": tally.learning_rref_bound,
            "linear_stage_hooked_bitops": tally.linear_stage_hooked_bitops.to_string(),
            "linear_stage_lin_muls": tally.linear_stage_lin_muls.to_string()},
        "foreign_hook_calls": work::foreign(),
        "disagreements": disagreements, "bad_solutions": bad_solutions,
        "bad_decompositions": bad_decompositions, "bad_residuals": bad_residuals,
        "decomposing_residual_indices": decomposing,
        "admitted": disagreements == 0 && bad_solutions == 0 && bad_decompositions == 0 && bad_residuals == 0,
        "note": "traced f4_ops include rejected replays and fallbacks; the learning RREF bound is reported separately and must be added to traced cost"
    });
    writeln!(out, "{summary}").map_err(|e| e.to_string())?;
    println!("{summary}");
    Ok(())
}
