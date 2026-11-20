//! Per-arm wall time and complete counted work of the opt-in traced Nagao
//! descent (descent-finder C1) versus the production staircase descent.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/descent-walltime.
//! Derived from descent-finder/traced_descent_check.rs (kept unchanged there).
//! The solver section below is copied from it with two changes, both marked:
//! the hooked-linear-stage diagnostic runs only when jv_bitops hooks are on
//! (so a timing run carries no snapshot overhead in one arm only), and a
//! rejected replay's wall time is added to the fallback's F4 time.
//!
//! One process runs ONE arm over the whole reconstructed residual stream, so
//! arms can be interleaved A/B/A/B by separate processes. Per residual it
//! records the wall time of `nagao_decompose_using` and the solver's own
//! F4 (F4Report.ms) and linear-stage (ZeroDimStats.lin_ms) milliseconds, and
//! a digest of the sorted F_p solutions and decompositions so arms and runs
//! can be checked for identical outputs.
//!
//! Built twice by build/build.sh: the default build links the frozen
//! zero_dim_la.rs (byte-identical to Run50's); `--features counted` links the
//! complete-accounting lane's counted copy and additionally reports the
//! linear stage's LinOps (muls, adds, macs, words, probes, draws).
//! Timing results are indicative, non-isolated local timing. [pcc:unverified]
//!
//! Usage: descent_walltime PUBLIC_JSON SEED N ARM HOOKS OUT_JSON
//!   ARM in {production, traced}; HOOKS in {0, 1} (jv_bitops on/off).
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
    // Diagnostic (hooks on only, so timing runs carry no asymmetric
    // snapshot overhead): how much of the linear stage the hooks see.
    if !work::enabled() {
        let sols = cover::linear_stage_from_basis(&r.basis, input, VARS, p, rng, LinMode::Fast, &mut st, t_lin);
        TALLY.with(|t| t.borrow_mut().linear_stage_lin_muls += st.lin_muls as u128);
        return (sols, st);
    }
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
                let mut full = full;
                full.ms += r.ms;
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
#[allow(dead_code)]
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

fn add_mod(a: u64, b: u64, l: u64) -> u64 {
    ((a as u128 + b as u128) % l as u128) as u64
}

/// FNV-1a over u64 words: an output digest, not a security primitive.
fn fnv(h: &mut u64, x: u64) {
    for byte in x.to_le_bytes() {
        *h ^= byte as u64;
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

fn main() -> Result<(), String> {
    // One worker thread (jv_bitops tallies are thread-local; F4's rayon
    // reduction must run on the claimed thread). Also the timing condition.
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    pool.install(run)
}

fn run() -> Result<(), String> {
    let t_process = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        return Err("usage: descent_walltime PUBLIC_JSON SEED N ARM HOOKS OUT_JSON".into());
    }
    let public: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let n: u64 = args[3].parse().map_err(|_| "N")?;
    let arm = args[4].as_str();
    let solver: cover::NagaoSolver = match arm {
        "production" => production_solver,
        "traced" => traced_solver,
        _ => return Err("ARM must be production or traced".into()),
    };
    let hooks = match args[5].as_str() {
        "0" => false,
        "1" => true,
        _ => return Err("HOOKS must be 0 or 1".into()),
    };
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
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    work::claim();
    work::set_moduli(p, l);
    work::enable(hooks);
    let widths = work::widths(p, l);
    WIDTHS.with(|w| w.set((p, l)));
    let ctx = Ctx::new(&spec);
    assert!(!ctx.trace, "baseline context must not replay traces");
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
    let opts = cover::opts_for_mode(24, 600.0, StopMode::Staircase(STAIRCASE));
    let setup_ms = t_process.elapsed().as_secs_f64() * 1e3;

    let (mut f4_ops, mut lin_muls, mut weil_muls, mut post_muls) = (0u128, 0u128, 0u128, 0u128);
    let (mut wall_ms, mut f4_ms, mut lin_ms, mut total_ms) = (0f64, 0f64, 0f64, 0f64);
    let (mut solutions, mut decompositions, mut timeouts, mut incomplete, mut lin_fallback) = (0u64, 0u64, 0u64, 0u64, 0usize);
    let mut digest: u64 = 0xcbf2_9ce4_8422_2325;
    let mut decomposing = Vec::new();
    let mut descent = work::JvTally::default();
    #[cfg(feature = "counted")]
    let lin0 = crypto_lib::cryptanalysis::zero_dim_la::lin_ops_snapshot();
    for i in 0..n {
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let before = if hooks { Some(work::snapshot()) } else { None };
        let t0 = Instant::now();
        let (decs, cost): (Vec<Dec6>, NagaoCost) = {
            let _d = work::enter(Phase::Descent);
            cover::nagao_decompose_using(&ctx, &base, &by_x, &residual, &opts, &mut rng, Some(solver))?
        };
        wall_ms += t0.elapsed().as_secs_f64() * 1e3;
        if let Some(bf) = before {
            descent.add(&work::snapshot().since(&bf));
        }
        let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
        sols.sort();
        f4_ops += cost.f4_muls as u128;
        lin_muls += cost.lin_muls as u128;
        weil_muls += cost.weil_muls as u128;
        post_muls += cost.post_muls as u128;
        f4_ms += cost.f4_ms;
        lin_ms += cost.lin_ms;
        total_ms += cost.total_ms;
        solutions += sols.len() as u64;
        decompositions += decs.len() as u64;
        timeouts += u64::from(cost.timed_out);
        incomplete += u64::from(cost.incomplete);
        lin_fallback += cost.lin_fallback;
        fnv(&mut digest, i);
        for s in &sols {
            for &x in s {
                fnv(&mut digest, x);
            }
        }
        for d in &decs {
            fnv(&mut digest, 0xDEC);
            fnv(&mut digest, cover::verify_dec(&jac, &base, &residual, d) as u64);
            // Dec6 has no stable public field layout here; its Debug form is hashed.
            for byte in format!("{d:?}").bytes() {
                fnv(&mut digest, byte as u64);
            }
        }
        if !decs.is_empty() {
            decomposing.push(i);
        }
        residual = jac.add(&residual, &step);
        a = add_mod(a, al, l);
        b = add_mod(b, be, l);
    }
    let _ = (a, b);
    let tally = TALLY.with(|t| t.borrow().clone());
    let d = descent.phase(Phase::Descent);
    #[cfg(feature = "counted")]
    let lin_ops = serde_json::to_value(crypto_lib::cryptanalysis::zero_dim_la::lin_ops_snapshot().since(&lin0))
        .map_err(|e| e.to_string())?;
    #[cfg(not(feature = "counted"))]
    let lin_ops = serde_json::Value::Null;
    let summary = serde_json::json!({
        "schema": "descent-walltime.arm-run/v1", "pcc": "unverified",
        "timing_label": "indicative non-isolated local timing (shared host, 1 thread, no CPU pinning)",
        "build": if cfg!(feature = "counted") { "counted-linear-stage" } else { "frozen-linear-stage" },
        "arm": arm, "hooks": hooks, "seed": seed, "residuals": n,
        "ms": {"setup": setup_ms, "descent_wall": wall_ms, "f4": f4_ms, "linear_stage": lin_ms,
               "nagao_total": total_ms, "process": t_process.elapsed().as_secs_f64() * 1e3},
        "counts": {"f4_ops": f4_ops.to_string(), "lin_muls": lin_muls.to_string(),
                   "weil_muls": weil_muls.to_string(), "post_muls": post_muls.to_string(),
                   "lin_fallback": lin_fallback, "lin_ops": lin_ops,
                   "hooked_descent_primitives": if hooks { work::prims_json(d) } else { serde_json::Value::Null },
                   "hooked_descent_bitops": if hooks { serde_json::Value::from(work::price(d, &widths).map_err(|e| e.to_string())?.to_string()) } else { serde_json::Value::Null }},
        "trace": {"learned": tally.learned, "replay_attempts": tally.replay_attempts,
            "accepted": tally.accepted, "accepted_after_mismatch": tally.accepted_after_mismatch,
            "rejected": tally.rejected, "rejected_replay_f4_ops": tally.rejected_replay_f4_ops,
            "fallback_f4_ops": tally.fallback_f4_ops,
            "learning_rref_upper_bound_uncounted_by_f4fp": tally.learning_rref_bound},
        "solutions": solutions, "decompositions": decompositions, "timeouts": timeouts, "incomplete": incomplete,
        "decomposing_residual_indices": decomposing,
        "output_digest": format!("{digest:016x}"),
        "foreign_hook_calls": work::foreign(),
    });
    let mut out = std::fs::OpenOptions::new().write(true).create_new(true).open(&args[6]).map_err(|e| e.to_string())?;
    writeln!(out, "{summary}").map_err(|e| e.to_string())?;
    println!("{summary}");
    Ok(())
}
