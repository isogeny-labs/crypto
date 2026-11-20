//! L5 kernel split: per-kernel tallies of the counted linear stage on the
//! reconstructed Run50 stream. Copy of complete-accounting's
//! `linear_stage_recount.rs` with the per-kernel accumulation added (marked
//! `L5`); every complete-accounting check is kept unchanged.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l5-kernel-split
//! (copied from complete-accounting).
//! Status: local, one thread, counted work only (no timing claim).
//! [pcc:unverified]
//!
//! The residual stream, the Weil systems and the per-residual RNG seeds are
//! derived exactly as in `descent-finder/traced_descent_check.rs` (production
//! arm: staircase-64 F4 without trace, `LinMode::Fast`), which reproduced
//! Run50's stop after 3,415 residuals with 2 decompositions. This driver
//! replaces nothing in the pipeline except that `zero_dim_la` is the counted
//! copy (`counted-linear-stage/zero_dim_la.rs`), whose only change is that
//! its own `F_p` work now reaches the existing `jv_bitops` hooks.
//!
//! For every residual it also runs, in the `Harness` phase (never charged to
//! the descent), the frozen uncounted `zero_dim_la_orig::fast_stage` and the
//! counted `zero_dim_la::fast_stage` on the same basis with cloned RNGs, and
//! checks exactly:
//!   (d) identical outputs and identical legacy `FastStats` counters;
//!   (a) tally(counted) − tally(orig) equals the stage's charged `LinOps`,
//!       primitive by primitive, with zero difference in every other kind;
//!   (b) legacy `muls()` = mac + mul − mul_outside_legacy + legacy_overcount
//!       + ring_muls;
//!   (c) the pipeline's own linear-stage tally (Descent phase) equals the
//!       counted harness tally when the fast stage did not fall back.
//! Run: linear_stage_recount PUBLIC_JSON SEED N OUT_JSON
use crypto_lib::cryptanalysis::{
    f4_fp::{self, F4Options, Poly},
    jv_bitops::{self as work, Kind, OpTally, Phase},
    jv_cover::{self as cover, Ctx, LinMode, StopMode, ZeroDimStats, E2, E6, PtE6},
    zero_dim_la::{self as counted, LinOps, KERNELS, NK},
    zero_dim_la_orig as orig,
};
use rand::{rngs::StdRng, SeedableRng};
use std::{cell::RefCell, collections::HashMap, time::Instant};

const VARS: usize = 6;
const STAIRCASE: usize = 64;

#[derive(Default, Clone)]
struct Acc {
    residuals: u64,
    fast_calls: u64,
    fast_gave_up: u64,
    fallbacks: u64,
    legacy_lin_muls: u128,
    lin: LinOps,
    /// L5: harness counted call's LinOps by kernel (from `FastStats::kernel_ops`).
    kern: [LinOps; NK],
    /// L5: per-kernel conservation and legacy cross-check failures.
    kern_failures: u64,
    /// L5: per legacy phase (nf, krylov, bm, roots, readout): legacy muls and
    /// the kernel group's net products (mac + mul + legacy_overcount + ring_muls
    /// − mul_outside_legacy).
    legacy_phase: [u128; 5],
    kernel_group_net: [u128; 5],
    /// Descent-phase tally of the pipeline's linear stage (counted copy).
    pipeline_linear: OpTally,
    /// Harness-phase tallies of the frozen and counted fast stage.
    harness_orig: OpTally,
    harness_counted: OpTally,
    failures: Vec<String>,
}

thread_local! {
    static ACC: RefCell<Acc> = RefCell::new(Acc::default());
}

fn kinds_json(t: &OpTally) -> serde_json::Value {
    let mut m = serde_json::Map::new();
    for k in Kind::ALL {
        m.insert(work::kind_name(k).to_string(), serde_json::Value::from(t.get(k)));
    }
    m.insert("rng_words".into(), serde_json::Value::from(t.rng_words));
    serde_json::Value::Object(m)
}

fn diff_ok(c: &OpTally, o: &OpTally, d: &LinOps) -> Result<(), String> {
    for k in Kind::ALL {
        let want = match k {
            Kind::FieldMul => d.field_mul(),
            Kind::FieldAdd => d.field_add(),
            Kind::WordOp => d.word_op(),
            Kind::HashProbe => d.probe,
            _ => 0,
        };
        let got = c.get(k).wrapping_sub(o.get(k));
        if got != want {
            return Err(format!("{}: counted−orig {} != charged {}", work::kind_name(k), got, want));
        }
    }
    if c.rng_words.wrapping_sub(o.rng_words) != d.rng {
        return Err("rng_words".into());
    }
    Ok(())
}

fn harness_phase(f: impl FnOnce()) -> OpTally {
    let before = work::snapshot();
    {
        let _g = work::enter(Phase::Harness);
        f();
    }
    *work::snapshot().since(&before).phase(Phase::Harness)
}

/// `cover::solve_zero_dim_mode(trace = false, LinMode::Fast)`, with the
/// differential checks around its linear stage.
fn checked_solver(
    input: &[Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str> {
    let mut st = ZeroDimStats::default();
    let r = f4_fp::f4(input, VARS, p, opts);
    let t_lin = Instant::now();
    st.f4_muls = r.field_ops;
    st.degree_reached = r.degree_reached;
    st.solving_degree = r.solving_degree;
    st.max_rows = r.max_rows;
    st.max_cols = r.max_cols;
    st.f4_ms = r.ms;
    st.stopped_at = r.staircase_at_stop;
    st.hilbert_applied = r.hilbert == f4_fp::HilbertStop::Applied;
    st.hilbert_fallback =
        matches!(r.hilbert, f4_fp::HilbertStop::CheckFailed | f4_fp::HilbertStop::Violated);
    st.hilbert_check_muls = r.hilbert_check_ops;
    if r.timed_out {
        st.timed_out = true;
        return Ok((Vec::new(), st));
    }
    if r.inconsistent {
        st.inconsistent = true;
        return Ok((Vec::new(), st));
    }
    if r.pairs_above_bound > 0 {
        st.incomplete = true;
        return Ok((Vec::new(), st));
    }
    // differential: frozen vs counted fast stage, same basis, cloned RNGs
    let (mut ra, mut rb) = (rng.clone(), rng.clone());
    let (mut fa, mut fb) = (orig::FastStats::default(), counted::FastStats::default());
    let (mut oa, mut ob) = (None, None);
    let ho = harness_phase(|| oa = Some(orig::fast_stage(&r.basis, input, VARS, p, &mut ra, &mut fa)));
    let l0 = counted::lin_ops_snapshot();
    let hc = harness_phase(|| ob = Some(counted::fast_stage(&r.basis, input, VARS, p, &mut rb, &mut fb)));
    let d = counted::lin_ops_snapshot().since(&l0);
    let mut fail = Vec::new();
    if oa != ob {
        fail.push("outputs differ".to_string());
    }
    let legacy_a = (fa.delta, fa.dense_cols, fa.nnz, fa.nf_rows, fa.nf_muls, fa.krylov_muls, fa.bm_muls, fa.roots_muls, fa.readout_muls, fa.functionals_tried, fa.roots, fa.gave_up);
    let legacy_b = (fb.delta, fb.dense_cols, fb.nnz, fb.nf_rows, fb.nf_muls, fb.krylov_muls, fb.bm_muls, fb.roots_muls, fb.readout_muls, fb.functionals_tried, fb.roots, fb.gave_up);
    if legacy_a != legacy_b {
        fail.push("legacy FastStats differ".to_string());
    }
    if let Err(e) = diff_ok(&hc, &ho, &d) {
        fail.push(format!("(a) {e}"));
    }
    let rhs = d.mac + d.mul + d.legacy_overcount + d.ring_muls;
    if fb.muls() + d.mul_outside_legacy != rhs {
        fail.push(format!("(b) legacy {} + outside {} != {}", fb.muls(), d.mul_outside_legacy, rhs));
    }
    // L5 (e): per-kernel tallies sum to the call's LinOps, field by field,
    // and nothing is unscoped
    let mut ksum = LinOps::default();
    for k in 0..NK {
        ksum.add_assign(&fb.kernel_ops[k]);
    }
    let mut kfail = 0u64;
    if ksum != d {
        fail.push(format!("(e) kernel sum {ksum:?} != {d:?}"));
        kfail += 1;
    }
    if fb.kernel_ops[0] != LinOps::default() {
        fail.push("(e) unscoped kernel counts".to_string());
        kfail += 1;
    }
    // L5 (f): each legacy phase counter equals its kernel group's net products
    let net = |ks: &[usize]| -> u64 {
        ks.iter()
            .map(|&k| {
                let o = &fb.kernel_ops[k];
                o.mac + o.mul + o.legacy_overcount + o.ring_muls - o.mul_outside_legacy
            })
            .sum()
    };
    let groups: [(&[usize], u64); 5] = [
        (&[1, 2], fb.nf_muls),
        (&[3], fb.krylov_muls),
        (&[4], fb.bm_muls),
        (&[5, 6, 7], fb.roots_muls),
        (&[8, 9], fb.readout_muls),
    ];
    let mut gnet = [0u64; 5];
    for (g, (ks, legacy)) in groups.iter().enumerate() {
        gnet[g] = net(ks);
        if gnet[g] != *legacy {
            fail.push(format!("(f) group {g}: kernel net {} != legacy {}", gnet[g], legacy));
            kfail += 1;
        }
    }
    // the pipeline's own call (counted copy), on the real RNG
    let before = work::snapshot();
    let sols = cover::linear_stage_from_basis(&r.basis, input, VARS, p, rng, LinMode::Fast, &mut st, t_lin);
    let seen = *work::snapshot().since(&before).phase(Phase::Descent);
    let fell_back = st.lin_fallback > 0;
    if !fell_back && seen != hc {
        fail.push("(c) pipeline linear tally != counted harness tally".to_string());
    }
    ACC.with(|acc| {
        let mut a = acc.borrow_mut();
        a.fast_calls += 1;
        a.fast_gave_up += u64::from(fb.gave_up.is_some());
        a.fallbacks += u64::from(fell_back);
        a.legacy_lin_muls += st.lin_muls as u128;
        a.lin.add_assign(&d);
        for k in 0..NK {
            a.kern[k].add_assign(&fb.kernel_ops[k]);
        }
        a.kern_failures += kfail;
        for (g, (_, legacy)) in groups.iter().enumerate() {
            a.legacy_phase[g] += *legacy as u128;
            a.kernel_group_net[g] += gnet[g] as u128;
        }
        a.pipeline_linear.add(&seen);
        a.harness_orig.add(&ho);
        a.harness_counted.add(&hc);
        let n = a.residuals;
        for f in fail {
            if a.failures.len() < 50 {
                a.failures.push(format!("residual {n}: {f}"));
            } else {
                a.failures.push(String::new());
            }
        }
    });
    Ok((sols, st))
}

fn add_mod(a: u64, b: u64, l: u64) -> u64 {
    ((a as u128 + b as u128) % l as u128) as u64
}

fn main() -> Result<(), String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    pool.install(run)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: linear_stage_recount PUBLIC_JSON SEED N OUT_JSON".into());
    }
    let public: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
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
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let widths = work::widths(p, l);
    let ctx = Ctx::new(&spec);
    assert!(!ctx.trace, "baseline context must not replay traces");
    // Same derivations as jv_extension_deferred_pair_dlp.rs / traced_descent_check.rs.
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
    let mut descent = OpTally::default();
    let (mut decompositions, mut f4_ops, mut lin_muls) = (0u64, 0u128, 0u128);
    let mut decomposing = Vec::new();
    for i in 0..n {
        ACC.with(|acc| acc.borrow_mut().residuals = i);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let before = work::snapshot();
        let (decs, cost) = {
            let _d = work::enter(Phase::Descent);
            cover::nagao_decompose_using(&ctx, &base, &by_x, &residual, &opts, &mut rng, Some(checked_solver))?
        };
        descent.add(work::snapshot().since(&before).phase(Phase::Descent));
        f4_ops += cost.f4_muls as u128;
        lin_muls += cost.lin_muls as u128;
        if !decs.is_empty() {
            decomposing.push(i);
        }
        decompositions += decs.len() as u64;
        residual = jac.add(&residual, &step);
        a = add_mod(a, al, l);
        b = add_mod(b, be, l);
    }
    let acc = ACC.with(|a| a.borrow().clone());
    let price = |t: &OpTally| work::price(t, &widths).map(|v| v.to_string()).unwrap_or_else(|e| e.to_string());
    let lin_only = {
        let mut t = descent;
        // the descent subtotal the pre-patch hooks would have recorded on
        // this stream: the complete-er subtotal minus the new charges
        t.counts[Kind::FieldMul as usize] -= acc.lin.field_mul();
        t.counts[Kind::FieldAdd as usize] -= acc.lin.field_add();
        t.counts[Kind::WordOp as usize] -= acc.lin.word_op();
        t.counts[Kind::HashProbe as usize] -= acc.lin.probe;
        t.rng_words -= acc.lin.rng;
        t
    };
    // L5: per-kernel table and conservation against the LinOps total
    let mut ksum = LinOps::default();
    let mut kernels = serde_json::Map::new();
    for k in 0..NK {
        ksum.add_assign(&acc.kern[k]);
        let o = &acc.kern[k];
        kernels.insert(KERNELS[k].to_string(), serde_json::json!({
            "ops": serde_json::to_value(o).map_err(|e| e.to_string())?,
            "field_mul": o.field_mul(), "field_add": o.field_add(), "word_op": o.word_op(),
            "mac_share": o.mac as f64 / acc.lin.mac.max(1) as f64,
            "field_mul_share": o.field_mul() as f64 / acc.lin.field_mul().max(1) as f64,
        }));
    }
    let kernel_conserved = ksum == acc.lin;
    // L5: each kernel's charge as an OpTally (charge_lin's mapping), priced
    let charged = |o: &LinOps| -> OpTally {
        let mut t = OpTally::default();
        t.counts[Kind::FieldMul as usize] = o.field_mul();
        t.counts[Kind::FieldAdd as usize] = o.field_add();
        t.counts[Kind::WordOp as usize] = o.word_op();
        t.counts[Kind::HashProbe as usize] = o.probe;
        t.rng_words = o.rng;
        t
    };
    let descent_bitops = work::price(&descent, &widths).map_err(|e| format!("{e:?}"))?;
    let lin_bitops = work::price(&charged(&acc.lin), &widths).map_err(|e| format!("{e:?}"))?;
    let mut kbits_sum = 0u128;
    let mut kernel_bitops = serde_json::Map::new();
    for k in 0..NK {
        let b = work::price(&charged(&acc.kern[k]), &widths).map_err(|e| format!("{e:?}"))?;
        kbits_sum += b;
        kernel_bitops.insert(KERNELS[k].to_string(), serde_json::json!({
            "bitops": b.to_string(),
            "share_of_linear_stage_charge": b as f64 / lin_bitops as f64,
            "share_of_descent": b as f64 / descent_bitops as f64,
        }));
    }
    let lp = ["nf(setup+nf)", "krylov", "bm", "roots(xpow+gcd+ring)", "readout(readout+cand)"];
    let legacy_cross: Vec<serde_json::Value> = (0..5)
        .map(|g| serde_json::json!({"legacy_phase": lp[g], "legacy_muls": acc.legacy_phase[g].to_string(),
            "kernel_net_products": acc.kernel_group_net[g].to_string(), "equal": acc.legacy_phase[g] == acc.kernel_group_net[g]}))
        .collect();
    let summary = serde_json::json!({
        "schema": "l5-kernel-split.linear-stage-recount/v1",
        "kernel_split": {
            "kernels": serde_json::Value::Object(kernels),
            "kernel_sum": serde_json::to_value(ksum).map_err(|e| e.to_string())?,
            "kernel_sum_equals_lin_ops": kernel_conserved,
            "per_residual_kernel_failures": acc.kern_failures,
            "legacy_phase_cross_check": legacy_cross,
            "kernel_bitops": serde_json::Value::Object(kernel_bitops),
            "linear_stage_charge_bitops": lin_bitops.to_string(),
            "kernel_bitops_sum_equals_linear_stage_charge": kbits_sum == lin_bitops,
            "descent_bitops": descent_bitops.to_string(),
        },
        "pcc": "unverified",
        "label": "recount by local replay of the reconstructed Run50 descent stream; not a recorded Run50 measurement",
        "seed": seed, "residuals": n, "p": p, "l_bits": 64 - l.leading_zeros(),
        "decompositions": decompositions, "decomposing_residual_indices": decomposing,
        "f4_ops": f4_ops.to_string(), "legacy_lin_muls": lin_muls.to_string(),
        "fast_stage_calls": acc.fast_calls, "fast_stage_gave_up": acc.fast_gave_up,
        "dense_fallbacks": acc.fallbacks,
        "lin_ops": serde_json::to_value(acc.lin).map_err(|e| e.to_string())?,
        "descent_with_counted_linear_stage": {"primitives": kinds_json(&descent), "bitops": price(&descent)},
        "descent_pre_patch_hooks": {"primitives": kinds_json(&lin_only), "bitops": price(&lin_only)},
        "pipeline_linear_stage_counted": {"primitives": kinds_json(&acc.pipeline_linear), "bitops": price(&acc.pipeline_linear)},
        "harness_fast_stage_frozen": {"primitives": kinds_json(&acc.harness_orig), "bitops": price(&acc.harness_orig)},
        "harness_fast_stage_counted": {"primitives": kinds_json(&acc.harness_counted), "bitops": price(&acc.harness_counted)},
        "foreign_hook_calls": work::foreign(),
        "other_modulus_ops": work::snapshot_other().iter().sum::<u64>(),
        "conservation_failures": acc.failures.len(),
        "first_failures": acc.failures.iter().take(10).collect::<Vec<_>>(),
        "admitted": acc.failures.is_empty() && work::foreign() == 0 && kernel_conserved,
    });
    std::fs::write(&args[4], format!("{summary}\n")).map_err(|e| e.to_string())?;
    println!("{summary}");
    Ok(())
}
