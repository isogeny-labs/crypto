//! A measured unit price for the binary-field arms of the k = 4
//! comparison: nanoseconds per `F_q` multiplication, squaring and
//! inversion, per `F_{q^4}` operation, per affine group addition and per
//! rho step, on `f2m4_torsion` curves, with the exact arithmetic the two
//! arms run (`Gf2q::{mul, inv}` with the counter, `Fq4`, `CurveB::add`,
//! `rho_negation`, `rho_negation_batched`, the `ic.pipeline` solve).
//!
//! Two ways of getting a unit price, both reported:
//!
//! * **micro**: a tight loop over one operation (a dependent chain for
//!   latency, independent operations for throughput);
//! * **in situ**: the wall time of a real run (an `ic.pipeline` partial
//!   solve of `ic_trials` trials, a `rho.negation_batched` and a
//!   `rho.negation` walk segment) divided by the operations that run
//!   counted.  This includes cache effects, the surrounding native work
//!   and (for `ic.pipeline`) whatever the solver does that no counter
//!   charges, so it is a price per *counted* operation of the whole
//!   algorithm, not of the multiplication alone.
//!
//! The ecbench comparison itself is counted: GAE = counted `F_q`
//! operations / `fq_muls_per_add`.  This module changes no count.

use crate::cryptanalysis::ecbench::canonical::sha256_hex;
use crate::cryptanalysis::ecbench::methods::{resolve, solve, MethodSpec};
use crate::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
use crate::cryptanalysis::gaudry_quartic_bin::{
    fq_muls_per_add, fq_muls_per_add_ref, BinCurve, PtB,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Opts {
    pub ms: Vec<u32>,
    pub reps: usize,
    pub seed: u64,
    /// Trials of the partial `ic.pipeline` solve (0 skips it).
    pub ic_trials: u64,
    /// Steps of the rho walk segments.
    pub rho_steps: u64,
    /// Run the `ic.pipeline` readout to a complete solve with the claim's
    /// `max_trials` (50,000,000) instead of `ic_trials` trials, so the
    /// timed work is dominated by the relation search, as in the claim.
    pub ic_complete: bool,
    /// Also time complete solves of the `rho.negation_batched_full` and
    /// `rho.negation_f2m4_full` arms, reported as ns per full-priced FME,
    /// the unit of the `ic.pipeline_f2m4_full` readout.
    pub rho_full: bool,
    /// The one-thread closure check (review 1, issue 5): complete solves of
    /// the `*_full` arms of both methods (the legacy IC run is skipped),
    /// with the process CPU time (user and system) of each solve beside its
    /// full-priced FME and the unit's own F_q multiplication time measured
    /// on this host.  A nonempty `unpriced` list is an error.  Implies
    /// `ic_complete` and `rho_full`.
    pub closure: bool,
    /// Which pricing variants the closure check covers: `full` (the current
    /// unit), `ref` and `refmm` (the reference unit, the code's and the
    /// one-pass min(code, Montgomery) mod-r units).  Empty means `full`
    /// only.  Only read with `closure`.
    pub closure_variants: Vec<String>,
    /// Per-phase CPU timers on a one-thread partial IC solve (user and
    /// system CPU of the window, thread CPU and minor faults of each phase
    /// beside its priced FME), the partial `*_full` rho arms at the same
    /// unit, and the local F_q multiplication time.  Needs `ic_trials > 0`.
    pub phases: bool,
}

/// Run `f` with the calling thread as the only thread of its own rayon
/// pool, so every nested `par_iter` of the solver runs inline on this thread
/// and never hands work across threads.  The global pool (also one thread)
/// parks and wakes its worker on each rayon call, which on this solver is
/// hundreds of thousands of futex calls and `sched_yield` spins: system time
/// and user spin that no count prices.  The closure check measures the
/// solver, not the scheduler.
fn on_one_thread<T>(f: impl FnOnce() -> T) -> T {
    // The closure runs on this very thread (the pool's only thread), so
    // the non-`Sync` counters it borrows are never shared.
    struct Local<F>(F);
    unsafe impl<F> Send for Local<F> {}
    struct Out<T>(T);
    unsafe impl<T> Send for Out<T> {}
    // One pool for the process: a thread can be the current-thread member of
    // only one.
    static POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    let pool = POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .use_current_thread()
            .build()
            .expect("one-thread rayon pool on the current thread")
    });
    let job = Local(f);
    let out = pool.install(move || {
        let job = job;
        Out((job.0)())
    });
    out.0
}

/// User and system CPU time of this process, in nanoseconds.
fn cpu_times_ns() -> (f64, f64) {
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) };
    let tv = |t: libc::timeval| t.tv_sec as f64 * 1e9 + t.tv_usec as f64 * 1e3;
    (tv(ru.ru_utime), tv(ru.ru_stime))
}

/// The closure check of one complete solve: measured CPU time against the
/// priced FME times the nanoseconds of one F_q multiplication, the unit
/// the FME is defined in.  Well above 1 is work the price does not cover
/// (a lower bound the pricing hides); well below 1 is overpricing.
fn closure_of(
    wall_ns: f64,
    user_ns: f64,
    sys_ns: f64,
    fme: f64,
    ns_local: f64,
    ns_table: f64,
) -> Value {
    let priced_local = fme * ns_local;
    json!({
        "wall_ns": wall_ns, "user_cpu_ns": user_ns, "sys_cpu_ns": sys_ns,
        "priced_fme": fme,
        "priced_ns_at_local_fq_mul": priced_local,
        "priced_ns_at_table_fq_mul": fme * ns_table,
        "user_over_priced": user_ns / priced_local,
        "user_plus_sys_over_priced": (user_ns + sys_ns) / priced_local,
        "wall_over_priced": wall_ns / priced_local,
        "user_over_priced_at_table_fq_mul": user_ns / (fme * ns_table),
    })
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

/// `{median, min, max, rel_spread, values}` of repeated measurements,
/// `rel_spread = (max - min) / median`.
fn stat(v: &[f64]) -> Value {
    let med = median(v.to_vec());
    let min = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    json!({
        "median": med, "min": min, "max": max,
        "rel_spread": if med > 0.0 { (max - min) / med } else { f64::NAN },
        "values": v,
    })
}

fn collect(reps: usize, mut f: impl FnMut() -> BTreeMap<&'static str, f64>) -> Value {
    let mut acc: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
    for _ in 0..reps.max(1) {
        for (k, v) in f() {
            acc.entry(k).or_default().push(v);
        }
    }
    Value::Object(
        acc.into_iter()
            .map(|(k, v)| (k.to_string(), stat(&v)))
            .collect(),
    )
}

fn time_ns(iters: u64, mut f: impl FnMut(u64)) -> f64 {
    let t = Instant::now();
    for i in 0..iters {
        f(i);
    }
    t.elapsed().as_nanos() as f64 / iters as f64
}

/// One pass of the micro measurements on `inst`.
fn micro_once(inst: &BinCurve, seed: u64) -> BTreeMap<&'static str, f64> {
    let c = &inst.curve;
    let f = &c.f;
    let g = &f.f;
    let q = g.q;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = BTreeMap::new();
    let xs: Vec<u32> = (0..1024).map(|_| rng.gen_range(1..q)).collect();
    let ys: Vec<u32> = (0..1024).map(|_| rng.gen_range(1..q)).collect();

    // F_q, the counted forms the arms call (Gf2q::mul / inv bump a Cell).
    let n = 20_000_000u64;
    let mut a = xs[0];
    out.insert(
        "fq_mul_counted_dependent",
        time_ns(n, |i| a = g.mul(a, xs[(i & 1023) as usize]) | 1),
    );
    black_box(a);
    let mut a = xs[0];
    out.insert(
        "fq_mul_uncounted_dependent",
        time_ns(n, |i| a = g.tab.mul(a, xs[(i & 1023) as usize]) | 1),
    );
    black_box(a);
    let mut acc = 0u32;
    out.insert(
        "fq_mul_counted_independent",
        time_ns(n, |i| {
            let j = (i & 1023) as usize;
            acc ^= g.mul(xs[j], ys[j]);
        }),
    );
    black_box(acc);
    // Squaring: the library has no separate F_q squaring; a squaring is
    // `mul(a, a)` (one counted operation).
    let mut a = xs[1];
    out.insert(
        "fq_sq_counted_dependent",
        time_ns(n, |i| a = g.mul(a, a) ^ xs[(i & 1023) as usize]),
    );
    black_box(a);
    let mut a = xs[2];
    out.insert(
        "fq_inv_counted_dependent",
        time_ns(n / 4, |i| a = g.inv(xs[((a ^ i as u32) & 1023) as usize])),
    );
    black_box(a);
    let mut acc = 0u32;
    out.insert(
        "fq_inv_counted_independent",
        time_ns(n / 4, |i| acc ^= g.inv(xs[(i & 1023) as usize])),
    );
    black_box(acc);

    // F_{q^4}: ns per operation and the counted F_q operations in it.
    let nf = 400_000u64;
    let el: Vec<_> = (0..256).map(|_| f.random(&mut rng)).collect();
    let mut a = el[0];
    let m0 = g.muls();
    let ns = time_ns(nf, |i| a = f.mul(&a, &el[(i & 255) as usize]));
    let per = (g.muls() - m0) as f64 / nf as f64;
    black_box(a);
    out.insert("fq4_mul_ns", ns);
    out.insert("fq4_mul_counted_fq_ops", per);
    let mut a = el[1];
    let m0 = g.muls();
    let ns = time_ns(nf, |i| a = f.sq(&a).add(&el[(i & 255) as usize]));
    let per = (g.muls() - m0) as f64 / nf as f64;
    black_box(a);
    out.insert("fq4_sq_ns", ns);
    out.insert("fq4_sq_counted_fq_ops", per);
    let mut a = el[2];
    let m0 = g.muls();
    let ns = time_ns(nf / 4, |i| {
        let b = a.add(&el[(i & 255) as usize]);
        a = f.inv(&b)
    });
    let per = (g.muls() - m0) as f64 / (nf / 4) as f64;
    black_box(a);
    out.insert("fq4_inv_ns", ns);
    out.insert("fq4_inv_counted_fq_ops", per);

    // Affine group addition (distinct x) and doubling, dependent chain.
    let na = 40_000u64;
    let gen = inst.generator;
    let mut p = c.add(&gen, &gen);
    let m0 = g.muls();
    let ns = time_ns(na, |_| p = c.add(&p, &gen));
    let per = (g.muls() - m0) as f64 / na as f64;
    black_box(p);
    out.insert("group_add_ns", ns);
    out.insert("group_add_counted_fq_ops", per);
    let mut p = gen;
    let m0 = g.muls();
    let ns = time_ns(na, |_| p = c.add(&p, &p));
    let per = (g.muls() - m0) as f64 / na as f64;
    black_box(p);
    out.insert("group_double_ns", ns);
    out.insert("group_double_counted_fq_ops", per);
    out
}

fn target_of(inst: &BinCurve, d: u128) -> PtB {
    inst.curve.mul(&inst.generator, d % inst.r)
}

/// A `rho.negation_batched` walk segment of `steps` steps.
fn rho_batched_once(inst: &BinCurve, seed: u64, steps: u64) -> BTreeMap<&'static str, f64> {
    use crate::cryptanalysis::ic_wide::{rho_negation_batched, BatchedRhoParams};
    let c = &inst.curve;
    let target = target_of(inst, 0x1234_5678_9abc_def1 ^ seed as u128);
    let par = BatchedRhoParams {
        walks: 256,
        dp_bits: 0,
    };
    let t = Instant::now();
    let res = if let Ok(r64) = u64::try_from(inst.r) {
        rho_negation_batched::<u64>(c, inst.generator, target, r64, seed, steps, par).0
    } else {
        rho_negation_batched::<u128>(c, inst.generator, target, inst.r, seed, steps, par).0
    };
    let wall = t.elapsed().as_nanos() as f64;
    let mut o = BTreeMap::new();
    o.insert("wall_ns", wall);
    o.insert("steps", res.steps as f64);
    o.insert("fq_ops_total", res.fq_total as f64);
    o.insert("fq_ops_setup", res.fq_setup as f64);
    o.insert("fq_ops_batch_inversion", res.fq_batch_inversion as f64);
    o.insert("fq_ops_batched_formula", res.fq_batched_formula as f64);
    o.insert("fq_ops_unbatched", res.fq_single_ops as f64);
    o
}

/// A `rho.negation` (one walk, one inversion per addition) segment.
fn rho_unbatched_once(inst: &BinCurve, seed: u64, steps: u64) -> BTreeMap<&'static str, f64> {
    use crate::cryptanalysis::ic_wide::rho_negation;
    type G = crate::cryptanalysis::gaudry_quartic_bin::CurveB;
    let c = &inst.curve;
    let target = target_of(inst, 0x1234_5678_9abc_def1 ^ seed as u128);
    let m0 = c.f.f.muls();
    let t = Instant::now();
    let res = if let Ok(r64) = u64::try_from(inst.r) {
        rho_negation::<G, u64>(c, inst.generator, target, r64, seed, steps).0
    } else {
        rho_negation::<G, u128>(c, inst.generator, target, inst.r, seed, steps).0
    };
    let wall = t.elapsed().as_nanos() as f64;
    let fq = (c.f.f.muls() - m0) as f64;
    let mut o = BTreeMap::new();
    o.insert("wall_ns", wall);
    o.insert("steps", res.steps as f64);
    o.insert("group_adds", res.group_ops.adds as f64);
    o.insert("group_doubles", res.group_ops.doubles as f64);
    o.insert("fq_ops_total", fq);
    o
}

fn derived(rep: &BTreeMap<&'static str, f64>, per: &str) -> (f64, f64) {
    let w = rep["wall_ns"];
    (w / rep["steps"], w / rep[per])
}

fn rho_block(
    reps: usize,
    steps: u64,
    mut once: impl FnMut(u64, u64) -> BTreeMap<&'static str, f64>,
    note: &str,
) -> Value {
    // Whole segment (includes set-up) and a long segment; the difference
    // of the two is the marginal (set-up free) price.
    let mut acc: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for k in 0..reps.max(1) {
        let seed = 0x5eed + k as u64;
        let short = once(seed, steps / 4);
        let long = once(seed, steps);
        let (ns_step, ns_fq) = derived(&long, "fq_ops_total");
        acc.entry("ns_per_step_whole").or_default().push(ns_step);
        acc.entry("ns_per_counted_fq_op_whole")
            .or_default()
            .push(ns_fq);
        let dw = long["wall_ns"] - short["wall_ns"];
        let ds = long["steps"] - short["steps"];
        let dq = long["fq_ops_total"] - short["fq_ops_total"];
        if ds > 0.0 && dq > 0.0 && dw > 0.0 {
            acc.entry("ns_per_step_marginal").or_default().push(dw / ds);
            acc.entry("ns_per_counted_fq_op_marginal")
                .or_default()
                .push(dw / dq);
            acc.entry("counted_fq_ops_per_step_marginal")
                .or_default()
                .push(dq / ds);
        }
        acc.entry("counted_fq_ops_per_step_whole")
            .or_default()
            .push(long["fq_ops_total"] / long["steps"]);
    }
    let mut o = serde_json::Map::new();
    for (k, v) in acc {
        o.insert(k.to_string(), stat(&v));
    }
    o.insert("steps_requested".into(), json!(steps));
    o.insert("note".into(), json!(note));
    Value::Object(o)
}

/// The `ic-wied` arm of the claim sessions (`e-m15`, `d-ladder`).
const IC_FACTOR_BASE: &str = "bin-t-line";
const IC_ORACLE: &str =
    "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1";
const IC_LINALG: &str = "wiedemann";
const CLAIM_MAX_TRIALS: u64 = 50_000_000;

fn ic_method(
    id: &str,
    trials: u64,
) -> Result<crate::cryptanalysis::ecbench::methods::ResolvedMethod, String> {
    resolve(&MethodSpec {
        id: id.into(),
        params: BTreeMap::from([
            ("factor_base".to_string(), IC_FACTOR_BASE.to_string()),
            ("oracle".to_string(), IC_ORACLE.to_string()),
            ("linalg".to_string(), IC_LINALG.to_string()),
            ("max_trials".to_string(), trials.to_string()),
        ]),
    })
}

/// The IC counters of one solve, keyed `<tag>_targets_tried`,
/// `<tag>_relations_found`, `<tag>_matrix_rank`, `<tag>_matrix_rows`, from
/// the counters `ic_report` (methods.rs) emits: `targets_tried`
/// (decomposition trials), `relations_found`, `matrix_rank`, `matrix_rows`.
/// An earlier version of this readout looked up `trials`, `decompositions`
/// and `accepted_relations`, which no report has, so it recorded zero for
/// all of them (withdrawn in `unit-price/WITHDRAWN.md`).  A counter the
/// report lacks is an error in the test below, not a silent zero.
fn ic_counter_readout(
    tag: &str,
    counters: &BTreeMap<String, u64>,
    o: &mut BTreeMap<&'static str, f64>,
) {
    let get = |k: &str| counters.get(k).copied().unwrap_or(0) as f64;
    let (tk, rk, kk, ak) = match tag {
        "legacy" => (
            "legacy_targets_tried",
            "legacy_relations_found",
            "legacy_matrix_rank",
            "legacy_matrix_rows",
        ),
        _ => (
            "full_targets_tried",
            "full_relations_found",
            "full_matrix_rank",
            "full_matrix_rows",
        ),
    };
    o.insert(tk, get("targets_tried"));
    o.insert(rk, get("relations_found"));
    o.insert(kk, get("matrix_rank"));
    o.insert(ak, get("matrix_rows"));
}

/// A partial `ic.pipeline` solve of `trials` trials (the global rayon pool,
/// set to one thread by [`run`]): the wall time, the counted work, and the full-priced total.
fn ic_once(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    trials: u64,
    seed: u64,
    closure: bool,
) -> Result<BTreeMap<&'static str, f64>, String> {
    let per_add = fq_muls_per_add(&b.curve) as f64;
    let mut o = BTreeMap::new();
    let arms: &[(&str, &str)] = if closure {
        &[("full", "ic.pipeline_f2m4_full")]
    } else {
        &[("legacy", "ic.pipeline"), ("full", "ic.pipeline_f2m4_full")]
    };
    for &(tag, id) in arms {
        let m = ic_method(id, trials)?;
        let (u0, s0) = cpu_times_ns();
        let t = Instant::now();
        let rep = if closure {
            on_one_thread(|| solve(&m, inst, facts, target, seed))?
        } else {
            solve(&m, inst, facts, target, seed)?
        };
        let wall = t.elapsed().as_nanos() as f64;
        let (u1, s1) = cpu_times_ns();
        if closure && !rep.unpriced.is_empty() {
            return Err(format!(
                "ic.pipeline_f2m4_full is a lower bound: unpriced {:?}",
                rep.unpriced
            ));
        }
        if tag == "full" {
            o.insert("full_user_cpu_ns", u1 - u0);
            o.insert("full_sys_cpu_ns", s1 - s0);
        }
        let fq = rep.total_gae * per_add;
        let (wk, fk) = match tag {
            "legacy" => ("legacy_wall_ns", "legacy_priced_fq_ops"),
            _ => ("full_wall_ns", "full_priced_fme"),
        };
        o.insert(wk, wall);
        o.insert(fk, fq);
        ic_counter_readout(tag, &rep.counters, &mut o);
        if tag == "legacy" {
            o.insert("recovered", if rep.recovered.is_some() { 1.0 } else { 0.0 });
        }
        if tag == "full" {
            o.insert("full_unpriced_entries", rep.unpriced.len() as f64);
        }
    }
    Ok(o)
}

/// A complete solve of a `*_full` rho arm on the same curve, one thread:
/// the wall time over the full-priced FME (`total_gae` times the current
/// per-addition divisor), the unit the IC readout reports.
fn rho_full_once(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    id: &str,
    seed: u64,
    closure: bool,
    cap: Option<u64>,
) -> Result<BTreeMap<&'static str, f64>, String> {
    use crate::cryptanalysis::ecbench::methods::F2M4_RHO_STEP_CAP;
    use std::sync::atomic::Ordering;
    let m = resolve(&MethodSpec {
        id: id.into(),
        params: BTreeMap::new(),
    })?;
    // A fixed-length partial walk (`cap` steps) is run on one thread, and
    // is not an error for being unrecovered.
    let one_thread = closure || cap.is_some();
    F2M4_RHO_STEP_CAP.store(cap.unwrap_or(0), Ordering::Relaxed);
    let (u0, s0) = cpu_times_ns();
    let t = Instant::now();
    let rep = if one_thread {
        on_one_thread(|| solve(&m, inst, facts, target, seed))
    } else {
        solve(&m, inst, facts, target, seed)
    };
    F2M4_RHO_STEP_CAP.store(0, Ordering::Relaxed);
    let rep = rep?;
    let wall = t.elapsed().as_nanos() as f64;
    let (u1, s1) = cpu_times_ns();
    if closure && !rep.unpriced.is_empty() {
        return Err(format!(
            "{id} is a lower bound: unpriced {:?}",
            rep.unpriced
        ));
    }
    let fme = rep.total_gae * fq_muls_per_add(&b.curve) as f64;
    let mut o = BTreeMap::new();
    o.insert("wall_ns", wall);
    o.insert("user_cpu_ns", u1 - u0);
    o.insert("sys_cpu_ns", s1 - s0);
    o.insert("priced_fme", fme);
    o.insert("recovered", if rep.recovered.is_some() { 1.0 } else { 0.0 });
    o.insert("unpriced_entries", rep.unpriced.len() as f64);
    Ok(o)
}

/// A partial `ic.pipeline_f2m4_full` solve of `trials` trials on one thread
/// with the per-phase CPU timers on: the window's user and system CPU, each
/// phase's thread CPU, minor faults and entries beside the priced work of
/// the phases that have a count.  The CPU split of a phase is exclusive
/// (see `phase_cpu`); `other_cpu_ns` is the window's CPU that no phase
/// claims (the timers' own clock reads and the solve's wrapper).
fn ic_phased_once(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    trials: u64,
    seed: u64,
    ns_local: f64,
) -> Result<Value, String> {
    use crate::cryptanalysis::phase_cpu as pc;
    let m = ic_method("ic.pipeline_f2m4_full", trials)?;
    let per_add = fq_muls_per_add(&b.curve) as f64;
    let (u0, s0) = cpu_times_ns();
    let t = Instant::now();
    pc::start();
    let rep = on_one_thread(|| solve(&m, inst, facts, target, seed));
    let totals = pc::finish();
    let wall = t.elapsed().as_nanos() as f64;
    let (u1, s1) = cpu_times_ns();
    let rep = rep?;
    let fme = rep.total_gae * per_add;
    let cpu_total: f64 = totals.iter().map(|t| t.cpu_ns as f64).sum();
    let faults_total: f64 = totals.iter().map(|t| t.faults as f64).sum();
    let sys = s1 - s0;
    let priced = &rep.detail["phase_priced"];
    let mut phases = serde_json::Map::new();
    for (i, t) in totals.iter().enumerate() {
        phases.insert(
            pc::NAMES[i].into(),
            json!({
                "cpu_ns": t.cpu_ns,
                "cpu_share": t.cpu_ns as f64 / cpu_total.max(1.0),
                "minor_faults": t.faults,
                "fault_share": t.faults as f64 / faults_total.max(1.0),
                "entries": t.entries,
                // System time attributed by fault share: an estimate, the
                // kernel's per-thread user/system split is tick-sampled.
                "sys_cpu_ns_by_fault_share": sys * t.faults as f64 / faults_total.max(1.0),
                "ns_per_entry": t.cpu_ns as f64 / (t.entries.max(1)) as f64,
            }),
        );
    }
    let cpu = |name: &str| totals[pc::NAMES.iter().position(|n| *n == name).unwrap()].cpu_ns as f64;
    let solve_cpu = [
        "weil_build",
        "macaulay_build",
        "echelon",
        "normal_form",
        "charpoly",
        "roots",
        "eigen",
    ]
    .iter()
    .map(|n| cpu(n))
    .sum::<f64>();
    let groups = phase_groups(&totals, priced, ns_local, fme);
    Ok(json!({
        "trials": trials,
        "one_thread": true,
        "pool_mode": "on_one_thread: this thread is the only member of a one-thread pool, so every parallel region runs inline (no hand-off)",
        "window": {
            "wall_ns": wall, "user_cpu_ns": u1 - u0, "sys_cpu_ns": sys,
            "phase_cpu_ns_total": cpu_total,
            "priced_fme": fme,
            "priced_ns_at_local_fq_mul": fme * ns_local,
            "cpu_ns_per_full_priced_fme": (u1 - u0 + sys) / fme,
            "user_cpu_ns_per_full_priced_fme": (u1 - u0) / fme,
            "wall_ns_per_full_priced_fme": wall / fme,
            "cpu_over_priced_at_local_fq_mul": (u1 - u0 + sys) / (fme * ns_local),
        },
        "phases": Value::Object(phases),
        "groups": groups,
        "solver_cpu_ns": solve_cpu,
        "phase_priced": priced.clone(),
        "counters": {
            "targets_tried": rep.counters.get("targets_tried"),
            "relations_found": rep.counters.get("relations_found"),
            "matrix_rank": rep.counters.get("matrix_rank"),
        },
        "unpriced": rep.unpriced,
    }))
}

/// The CPU phases grouped to the priced work they belong to, each group's CPU
/// share beside its priced-FME share (current unit), and for every solver
/// phase with a multiplication count its CPU per counted multiplication.
///
/// The groups: `setup` (factor base, its second build, the symmetrised
/// precomputation), `solver` (Weil build, Macaulay build, echelon, normal
/// form, charpoly, roots, eigen, split and lift, oracle bookkeeping), `loop`
/// (the walk, the sign-test additions, the target guard) and `linalg` (the
/// relation matrix and the verification).  A native counter the report prices
/// (matrix elements touched, hash operations, table inversions, parallel
/// regions) cannot be told apart by phase, so the solver group takes all of
/// them.  `priced_fme` of the groups adds up to the window's total.
fn phase_groups(
    totals: &[crate::cryptanalysis::phase_cpu::Total],
    priced: &Value,
    ns_local: f64,
    total_fme: f64,
) -> Value {
    use crate::cryptanalysis::phase_cpu::NAMES;
    let cpu = |n: &str| totals[NAMES.iter().position(|x| *x == n).unwrap()].cpu_ns as f64;
    let cpu_total: f64 = totals.iter().map(|t| t.cpu_ns as f64).sum();
    let fme = |p: &str| priced["report_phases"][p]["fme"].as_f64().unwrap_or(0.0);
    let ops = |p: &str| {
        priced["report_phases"][p]["group_ops_fme"]
            .as_f64()
            .unwrap_or(0.0)
    };
    let counted = |k: &str| {
        priced["solver_counted_fq_muls_by_phase"][k]
            .as_f64()
            .unwrap_or(0.0)
    };
    let native = |k: &str| {
        priced["by_native_counter"][k]["fme"]
            .as_f64()
            .unwrap_or(0.0)
    };
    let precompute = priced["precompute_fq_muls"].as_f64().unwrap_or(0.0);
    let loop_natives = [
        "target_guard_probes",
        "walk_jumps",
        "walk_restarts",
        "walk_steps",
    ]
    .iter()
    .map(|k| native(k))
    .sum::<f64>();
    let setup = fme("factor_base") + precompute;
    let looped = ops("relations") + loop_natives;
    let linalg = fme("linear_algebra") + fme("verify");
    let solver = fme("relations") - ops("relations") - precompute - loop_natives;
    let group = |name: &str, phases: &[&str], priced_fme: f64| {
        let c: f64 = phases.iter().map(|p| cpu(p)).sum();
        json!({
            "group": name, "cpu_phases": phases,
            "cpu_ns": c, "cpu_share": c / cpu_total.max(1.0),
            "priced_fme": priced_fme, "priced_share": priced_fme / total_fme.max(1.0),
            "cpu_over_priced_at_local_fq_mul": c / (priced_fme * ns_local).max(1e-9),
        })
    };
    let mul_phase = |name: &str, key: &str| {
        let n = counted(key);
        json!({
            "cpu_phase": name, "counted_fq_muls": n, "cpu_ns": cpu(name),
            "cpu_ns_per_counted_fq_mul": cpu(name) / n.max(1.0),
            "cpu_over_counted_at_local_fq_mul": cpu(name) / (n * ns_local).max(1e-9),
        })
    };
    // Per CPU phase: its counted multiplications plus the native counters of
    // its stage, priced as the record prices them; the rest of the solver's
    // priced work (table inversions, systems built, witness lookups, lifts)
    // has no stage and is listed apart.
    let stage_fme = |k: &str| {
        priced["solver_native_by_stage"][k]["fme"]
            .as_f64()
            .unwrap_or(0.0)
    };
    let per_phase = |name: &str, muls: &[&str]| {
        let counted_fme: f64 = muls.iter().map(|k| counted(k)).sum();
        let p = counted_fme + stage_fme(name);
        json!({
            "cpu_phase": name,
            "counted_fme": counted_fme, "native_fme": stage_fme(name), "priced_fme": p,
            "priced_share": p / total_fme.max(1.0),
            "cpu_ns": cpu(name), "cpu_share": cpu(name) / cpu_total.max(1.0),
            "cpu_ns_per_priced_fme": cpu(name) / p.max(1e-9),
            "native_counts": priced["solver_native_by_stage"][name].clone(),
        })
    };
    let phases = [
        per_phase("weil_build", &["weil"]),
        per_phase("macaulay_build", &["triangularise"]),
        per_phase("echelon", &["echelon"]),
        per_phase("normal_form", &["normal_form"]),
        per_phase("charpoly", &["charpoly"]),
        per_phase("roots", &["roots"]),
        per_phase("eigen", &["eigenvector", "verify"]),
        per_phase("split_lift", &["split"]),
    ];
    let staged: f64 = phases
        .iter()
        .map(|p| p["priced_fme"].as_f64().unwrap())
        .sum();
    json!({
        "solver_phases": phases,
        "solver_unstaged_fme": solver - staged,
        "solver_unstaged_note": "table inversions, systems built, witness lookups and lifts, which have no stage",
        "table": [
            group("setup", &["factor_base", "factor_base_facts_rebuild", "precompute"], setup),
            group("solver", &["weil_build", "macaulay_build", "echelon", "normal_form", "charpoly", "roots", "eigen", "split_lift", "oracle_other"], solver),
            group("loop", &["loop"], looped),
            group("linalg", &["linalg"], linalg),
        ],
        "sum_priced_fme": setup + solver + looped + linalg,
        "counted_multiplications_by_cpu_phase": [
            mul_phase("weil_build", "weil"),
            mul_phase("echelon", "echelon"),
            mul_phase("normal_form", "normal_form"),
            mul_phase("charpoly", "charpoly"),
            mul_phase("roots", "roots"),
            mul_phase("eigen", "eigenvector"),
            mul_phase("split_lift", "split"),
        ],
        "note": "solver priced_fme includes every native counter the report prices for the solver (element touches, hash operations, table inversions, parallel regions, systems built, witness lookups) and the lifts; those have no phase of their own",
    })
}

/// A fixed-length partial `*_full` rho walk on one thread, priced at the
/// full unit, whole (the walk with its set-up) and marginal (long minus
/// quarter-length, set-up free), in ns per full-priced FME, wall and CPU:
/// the rho side of the IC/rho CPU comparison when a complete solve does
/// not fit.
fn rho_full_partial(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    id: &str,
    steps: u64,
    reps: usize,
) -> Result<Value, String> {
    let mut acc: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for k in 0..reps.max(1) {
        let seed = 7 + k as u64;
        let short = rho_full_once(
            inst,
            facts,
            b,
            target,
            id,
            seed,
            false,
            Some((steps / 4).max(1)),
        )?;
        let long = rho_full_once(inst, facts, b, target, id, seed, false, Some(steps))?;
        let cpu = |r: &BTreeMap<&'static str, f64>| r["user_cpu_ns"] + r["sys_cpu_ns"];
        acc.entry("ns_per_full_priced_fme_whole")
            .or_default()
            .push(long["wall_ns"] / long["priced_fme"]);
        acc.entry("cpu_ns_per_full_priced_fme_whole")
            .or_default()
            .push(cpu(&long) / long["priced_fme"]);
        let dfme = long["priced_fme"] - short["priced_fme"];
        if dfme > 0.0 {
            acc.entry("ns_per_full_priced_fme_marginal")
                .or_default()
                .push((long["wall_ns"] - short["wall_ns"]) / dfme);
            acc.entry("cpu_ns_per_full_priced_fme_marginal")
                .or_default()
                .push((cpu(&long) - cpu(&short)) / dfme);
        }
        acc.entry("priced_fme")
            .or_default()
            .push(long["priced_fme"]);
        acc.entry("user_cpu_ns")
            .or_default()
            .push(long["user_cpu_ns"]);
        acc.entry("sys_cpu_ns")
            .or_default()
            .push(long["sys_cpu_ns"]);
    }
    let mut o: serde_json::Map<String, Value> =
        acc.iter().map(|(k, v)| (k.to_string(), stat(v))).collect();
    o.insert("partial".into(), json!(true));
    o.insert("steps_cap".into(), json!(steps));
    o.insert("steps_cap_short".into(), json!((steps / 4).max(1)));
    o.insert("one_thread".into(), json!(true));
    Ok(Value::Object(o))
}

/// The priced total of a solve in F_q-multiplication equivalents of its own
/// unit: `total_gae` times the divisor that unit uses.  `full` is the
/// current unit (the curve's schoolbook `fq_muls_per_add`); `ref` and
/// `refmm` are the reference unit (`fq_muls_per_add_ref`, 42 at odd m and 64
/// at even m).  For the reference ids the second model (polynomial basis,
/// free constant multiplication, divisor 42) is read from the record's own
/// `reference_pricing.poly_free_const` block and returned beside it.
fn priced_fme_of(
    rep: &crate::cryptanalysis::ecbench::methods::SolveReport,
    b: &BinCurve,
    variant: &str,
) -> (f64, Option<f64>) {
    if variant == "full" {
        return (rep.total_gae * fq_muls_per_add(&b.curve) as f64, None);
    }
    let primary = rep.total_gae * fq_muls_per_add_ref(&b.curve) as f64;
    let poly = &rep.detail["reference_pricing"]["poly_free_const"];
    let second = match (poly["total_gae"].as_f64(), poly["fq_muls_per_add"].as_f64()) {
        (Some(g), Some(d)) => Some(g * d),
        _ => None,
    };
    (primary, second)
}

/// The table row's F_q multiplication time (ns) that the FME of a variant
/// is defined in: `f2m4_full_prices.json` for `full` and `ref`, the
/// one-pass table for `refmm`.
fn table_ns_fq_mul(m: u32, wide: bool, variant: &str) -> Result<f64, String> {
    use crate::cryptanalysis::ecbench::f2m4_price::{row_key, F2M4_ONEPASS_PRICES, F2M4_PRICES};
    let src = if variant == "refmm" {
        F2M4_ONEPASS_PRICES
    } else {
        F2M4_PRICES
    };
    let doc: Value = serde_json::from_str(src).map_err(|e| e.to_string())?;
    doc["instances"][row_key(m, wide)]["ns_per_fq_mul_median"]
        .as_f64()
        .ok_or_else(|| format!("no {variant} table row for m = {m}"))
}

/// One complete one-thread solve of `id` with its CPU time, and the closure
/// of it under the primary model and (for the reference ids) the second.
fn closure_solve(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    id: &str,
    variant: &str,
    params: BTreeMap<String, String>,
    seed: u64,
    ns_local: f64,
    ns_table: f64,
) -> Result<Value, String> {
    let m = resolve(&MethodSpec {
        id: id.into(),
        params,
    })?;
    let (u0, s0) = cpu_times_ns();
    let t = Instant::now();
    let rep = on_one_thread(|| solve(&m, inst, facts, target, seed))?;
    let wall = t.elapsed().as_nanos() as f64;
    let (u1, s1) = cpu_times_ns();
    if !rep.unpriced.is_empty() {
        return Err(format!(
            "{id} is a lower bound: unpriced {:?}",
            rep.unpriced
        ));
    }
    let (fme, fme_poly) = priced_fme_of(&rep, b, variant);
    let mut o = serde_json::Map::new();
    o.insert("id".into(), json!(id));
    o.insert("recovered".into(), json!(rep.recovered.is_some()));
    o.insert("total_gae".into(), json!(rep.total_gae));
    o.insert(
        "divisor".into(),
        json!(if variant == "full" {
            fq_muls_per_add(&b.curve)
        } else {
            fq_muls_per_add_ref(&b.curve)
        }),
    );
    o.insert("counters".into(), json!(rep.counters));
    o.insert(
        "closure".into(),
        closure_of(wall, u1 - u0, s1 - s0, fme, ns_local, ns_table),
    );
    if let Some(fp) = fme_poly {
        // The second model prices the same run; the measured CPU time is the
        // same number, only the priced total differs.
        o.insert(
            "closure_poly_free_const".into(),
            closure_of(wall, u1 - u0, s1 - s0, fp, ns_local, ns_table),
        );
    }
    Ok(Value::Object(o))
}

/// The closure check of one pricing variant on one curve: the three arms'
/// complete solves, each on one thread.
fn closure_variant(
    inst: &Instance,
    facts: &crate::cryptanalysis::ecbench::workload::CurveFacts,
    b: &BinCurve,
    target: &[String; 2],
    m: u32,
    variant: &str,
    ns_local: f64,
) -> Result<Value, String> {
    let wide = u64::try_from(b.r).is_err();
    let ns_table = table_ns_fq_mul(m, wide, variant)?;
    let mut blk = serde_json::Map::new();
    blk.insert("variant".into(), json!(variant));
    blk.insert("ns_per_fq_mul_table".into(), json!(ns_table));
    for (name, id) in [
        (
            "rho_negation_batched",
            format!("rho.negation_batched_{variant}"),
        ),
        ("rho_negation_f2m4", format!("rho.negation_f2m4_{variant}")),
    ] {
        let t0 = Instant::now();
        let v = closure_solve(
            inst,
            facts,
            b,
            target,
            &id,
            variant,
            BTreeMap::new(),
            7,
            ns_local,
            ns_table,
        )?;
        eprintln!("m = {m}: {id} {:.1}s", t0.elapsed().as_secs_f64());
        blk.insert(name.into(), v);
    }
    let id = format!("ic.pipeline_f2m4_{variant}");
    let t0 = Instant::now();
    let params = BTreeMap::from([
        ("factor_base".to_string(), IC_FACTOR_BASE.to_string()),
        ("oracle".to_string(), IC_ORACLE.to_string()),
        ("linalg".to_string(), IC_LINALG.to_string()),
        ("max_trials".to_string(), CLAIM_MAX_TRIALS.to_string()),
    ]);
    let v = closure_solve(
        inst, facts, b, target, &id, variant, params, 3, ns_local, ns_table,
    )?;
    eprintln!("m = {m}: {id} {:.1}s", t0.elapsed().as_secs_f64());
    blk.insert("ic_pipeline".into(), v);
    Ok(Value::Object(blk))
}

pub fn run(o: &Opts) -> Result<Value, String> {
    let exe_hash = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::read(p).ok())
        .map(|b| sha256_hex(&b));
    let host = crate::cryptanalysis::ecbench::host::capture()
        .map(|c| json!({"env_class_id": c.env_class_id, "stable": c.stable, "build": c.build, "process_affinity": c.process_affinity}))
        .ok();
    // One solver thread: the unit price is per CPU-second of one core, not
    // of a wake-up pattern on a loaded host.  (Ignored if a pool exists.)
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global();
    let rho_full_on = o.rho_full || o.closure;
    let ic_complete = o.ic_complete || o.closure;
    let variants: Vec<String> = if o.closure_variants.is_empty() {
        vec!["full".to_string()]
    } else {
        o.closure_variants.clone()
    };
    for v in &variants {
        if !matches!(v.as_str(), "full" | "ref" | "refmm") {
            return Err(format!("unknown closure variant `{v}` (full, ref, refmm)"));
        }
    }
    // With `--closure`, the current-unit (`full`) arms run only when asked.
    let full_arms = !o.closure || variants.iter().any(|v| v == "full");
    let mut per_m = serde_json::Map::new();
    for &m in &o.ms {
        let cof = if m >= 15 { 2 } else { 16 };
        let spec = CurveSpec::F2m4Torsion {
            m,
            seed: o.seed,
            max_cofactor: cof,
        };
        let (bu0, bs0) = cpu_times_ns();
        let inst = spec.build()?;
        let (bu1, bs1) = cpu_times_ns();
        let curve_build_cpu_ns = (bu1 - bu0) + (bs1 - bs0);
        let facts = inst.facts(&spec);
        let Instance::F2m4(b) = &inst else {
            return Err("not an f2m4 instance".into());
        };
        let b: &BinCurve = b;
        let per_add = fq_muls_per_add(&b.curve);
        let t0 = Instant::now();
        let micro = collect(o.reps, || micro_once(b, 0xF2 + m as u64));
        eprintln!("m = {m}: micro {:.1}s", t0.elapsed().as_secs_f64());
        let t0 = Instant::now();
        let rb = rho_block(
            o.reps,
            o.rho_steps,
            |seed, n| rho_batched_once(b, seed, n.max(1)),
            "rho.negation_batched, 256 lockstep walks; 'whole' includes set-up, 'marginal' is the difference of a long and a quarter-length segment",
        );
        eprintln!("m = {m}: rho batched {:.1}s", t0.elapsed().as_secs_f64());
        let t0 = Instant::now();
        let ru = rho_block(
            o.reps,
            o.rho_steps / 4,
            |seed, n| rho_unbatched_once(b, seed, n.max(1)),
            "rho.negation (one walk, an F_q^4 inversion per addition)",
        );
        eprintln!("m = {m}: rho unbatched {:.1}s", t0.elapsed().as_secs_f64());
        let d = 0x0123_4567_89ab_cdefu128 % b.r;
        let q = target_of(b, d);
        let target = {
            let f = &b.curve.f;
            [
                format!("0x{:x}", f.pack(&q.x)),
                format!("0x{:x}", f.pack(&q.y)),
            ]
        };
        // The unit's own F_q multiplication time: on this host, and in the
        // committed table row the prices come from.
        let (ns_local, ns_table) = if o.closure || o.phases || rho_full_on {
            let wide = u64::try_from(b.r).is_err();
            let local = crate::cryptanalysis::ecbench::f2m4_price::measure(b, 3)?
                ["ns_per_fq_mul_median"]
                .as_f64()
                .ok_or("no local F_q multiplication time")?;
            let doc: Value =
                serde_json::from_str(crate::cryptanalysis::ecbench::f2m4_price::F2M4_PRICES)
                    .map_err(|e| e.to_string())?;
            let table = doc["instances"]
                [crate::cryptanalysis::ecbench::f2m4_price::row_key(m, wide)]
                ["ns_per_fq_mul_median"]
                .as_f64()
                .ok_or("no table row for this m")?;
            (local, table)
        } else {
            (f64::NAN, f64::NAN)
        };
        // The plain IC block (the global one-worker pool: each parallel
        // region of the solver is handed to the worker) runs before anything
        // calls `on_one_thread`: that makes this thread a permanent member
        // of the one-thread pool, after which every later solve runs inline
        // and the hand-off cost this block measures is gone.
        let ic = if (o.ic_trials > 0 || ic_complete) && full_arms {
            let t0 = Instant::now();
            let mut acc: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
            let ic_reps = if ic_complete { 1 } else { o.reps.min(3) };
            let ic_max_trials = if ic_complete {
                CLAIM_MAX_TRIALS
            } else {
                o.ic_trials
            };
            for k in 0..ic_reps {
                for (kk, v) in ic_once(
                    &inst,
                    &facts,
                    b,
                    &target,
                    ic_max_trials,
                    3 + k as u64,
                    o.closure,
                )? {
                    acc.entry(kk).or_default().push(v);
                }
            }
            eprintln!("m = {m}: ic {:.1}s", t0.elapsed().as_secs_f64());
            let mut blk: serde_json::Map<String, Value> =
                acc.iter().map(|(k, v)| (k.to_string(), stat(v))).collect();
            // Derived: ns per priced operation, per run.
            let w = |k: &str| &acc[k];
            let ratio = |num: &str, den: &str| -> Value {
                let r: Vec<f64> = w(num).iter().zip(w(den)).map(|(a, b)| a / b).collect();
                stat(&r)
            };
            if !o.closure {
                blk.insert(
                    "ns_per_legacy_priced_fq_op".into(),
                    ratio("legacy_wall_ns", "legacy_priced_fq_ops"),
                );
            } else {
                blk.insert(
                    "closure".into(),
                    closure_of(
                        w("full_wall_ns")[0],
                        w("full_user_cpu_ns")[0],
                        w("full_sys_cpu_ns")[0],
                        w("full_priced_fme")[0],
                        ns_local,
                        ns_table,
                    ),
                );
            }
            blk.insert(
                "ns_per_full_priced_fme".into(),
                ratio("full_wall_ns", "full_priced_fme"),
            );
            blk.insert("ic_trials_requested".into(), json!(ic_max_trials));
            blk.insert("ic_complete".into(), json!(ic_complete));
            blk.insert(
                "spec".into(),
                json!({
                    "factor_base": IC_FACTOR_BASE, "oracle": IC_ORACLE, "linalg": IC_LINALG,
                    "source": "claim sessions e-m15 and d-ladder (ic-wied arm); max_cofactor 16 for m <= 12 (d-ladder) and 2 for m >= 15 (e-m15)",
                    "max_cofactor": cof,
                }),
            );
            // The relation search was reached only when relations were found.
            let reached: Vec<f64> = w(if o.closure {
                "full_relations_found"
            } else {
                "legacy_relations_found"
            })
            .iter()
            .map(|&r| (r > 0.0) as u8 as f64)
            .collect();
            blk.insert("reached_relations".into(), stat(&reached));
            blk.insert("rayon_threads".into(), json!(rayon::current_num_threads()));
            blk.insert(
                "pool_mode".into(),
                json!("global rayon pool (one worker); this thread is not a pool member, so each parallel region of the solver is handed to the worker; run before any on_one_thread call"),
            );
            blk.insert("reps".into(), json!(ic_reps));
            Value::Object(blk)
        } else {
            Value::Null
        };
        let rho_full = if rho_full_on && full_arms {
            let mut blk = serde_json::Map::new();
            for (name, id) in [
                ("rho_negation_batched_full", "rho.negation_batched_full"),
                ("rho_negation_f2m4_full", "rho.negation_f2m4_full"),
            ] {
                let t0 = Instant::now();
                let mut acc: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
                let nrep = if o.closure { 1 } else { o.reps.min(3) };
                for k in 0..nrep {
                    for (kk, v) in
                        rho_full_once(&inst, &facts, b, &target, id, 7 + k as u64, o.closure, None)?
                    {
                        acc.entry(kk).or_default().push(v);
                    }
                }
                eprintln!("m = {m}: {id} {:.1}s", t0.elapsed().as_secs_f64());
                let mut e: serde_json::Map<String, Value> =
                    acc.iter().map(|(k, v)| (k.to_string(), stat(v))).collect();
                let r: Vec<f64> = acc["wall_ns"]
                    .iter()
                    .zip(&acc["priced_fme"])
                    .map(|(a, b)| a / b)
                    .collect();
                e.insert("ns_per_full_priced_fme".into(), stat(&r));
                if o.closure {
                    e.insert(
                        "closure".into(),
                        closure_of(
                            acc["wall_ns"][0],
                            acc["user_cpu_ns"][0],
                            acc["sys_cpu_ns"][0],
                            acc["priced_fme"][0],
                            ns_local,
                            ns_table,
                        ),
                    );
                }
                blk.insert(name.into(), Value::Object(e));
            }
            Value::Object(blk)
        } else if o.phases && full_arms {
            let mut blk = serde_json::Map::new();
            for (name, id, steps) in [
                (
                    "rho_negation_batched_full",
                    "rho.negation_batched_full",
                    o.rho_steps,
                ),
                (
                    "rho_negation_f2m4_full",
                    "rho.negation_f2m4_full",
                    o.rho_steps / 4,
                ),
            ] {
                let t0 = Instant::now();
                blk.insert(
                    name.into(),
                    rho_full_partial(&inst, &facts, b, &target, id, steps, o.reps.min(3))?,
                );
                eprintln!("m = {m}: {id} partial {:.1}s", t0.elapsed().as_secs_f64());
            }
            Value::Object(blk)
        } else {
            Value::Null
        };
        let ic_phased = if o.phases && o.ic_trials > 0 && full_arms {
            let t0 = Instant::now();
            // Repeats on seeds 3, 4, ...: the run with the median CPU per
            // priced FME is reported in full, every run's headline beside it.
            let mut runs = Vec::new();
            for k in 0..o.reps.clamp(1, 3) {
                runs.push(ic_phased_once(
                    &inst,
                    &facts,
                    b,
                    &target,
                    o.ic_trials,
                    3 + k as u64,
                    ns_local,
                )?);
            }
            let key = |v: &Value| {
                v["window"]["cpu_ns_per_full_priced_fme"]
                    .as_f64()
                    .unwrap_or(0.0)
            };
            let heads: Vec<Value> = runs
                .iter()
                .map(|v| {
                    json!({
                        "cpu_ns_per_full_priced_fme": key(v),
                        "cpu_over_priced_at_local_fq_mul": v["window"]["cpu_over_priced_at_local_fq_mul"],
                        "user_cpu_ns": v["window"]["user_cpu_ns"],
                        "sys_cpu_ns": v["window"]["sys_cpu_ns"],
                        "relations_found": v["counters"]["relations_found"],
                    })
                })
                .collect();
            runs.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());
            let mut v = runs.swap_remove(runs.len() / 2);
            v["repeats"] = Value::Array(heads);
            v["reported_run"] = json!("the median by cpu_ns_per_full_priced_fme");
            eprintln!("m = {m}: ic phased {:.1}s", t0.elapsed().as_secs_f64());
            v
        } else {
            Value::Null
        };
        let mut closure_variants = serde_json::Map::new();
        if o.closure {
            for v in variants.iter().filter(|v| v.as_str() != "full") {
                closure_variants.insert(
                    v.clone(),
                    closure_variant(&inst, &facts, b, &target, m, v, ns_local)?,
                );
            }
        }
        per_m.insert(
            format!("m{m}"),
            json!({
                "m": m, "q_bits": m, "group_bits": 4 * m,
                "r_bits": 128 - b.r.leading_zeros(), "scalar_width_bits": if u64::try_from(b.r).is_ok() {64} else {128},
                "fq_muls_per_add": per_add,
                "ns_per_fq_mul_local": ns_local,
                "ns_per_fq_mul_table": ns_table,
                "curve": facts.slug,
                "micro": micro,
                "curve_build_cpu_ns": curve_build_cpu_ns,
                "in_situ": { "rho_negation_batched": rb, "rho_negation": ru, "ic_pipeline": ic, "ic_pipeline_phased": ic_phased, "rho_full": rho_full, "closure_variants": Value::Object(closure_variants) },
            }),
        );
    }
    Ok(json!({
        "schema": "ecbench.f2m4_unit_price/v1",
        "binary_sha256": exe_hash,
        "host": host,
        "repeats": o.reps,
        "seed": o.seed,
        "spread": "every statistic carries median, min, max, rel_spread = (max-min)/median and the raw per-repeat values",
        "units": "nanoseconds unless the key says counted_fq_ops",
        "ic_trials": o.ic_trials,
        "ic_complete": ic_complete,
        "closure": o.closure,
        "closure_variants": variants,
        "supersedes": {
            "file": "research/gaudry_k4_char2_replication_20261026/unit-price/unit-price-f2m4-m14-17.json",
            "withdrawn_fields": ["legacy_decompositions", "legacy_trials", "full_trials", "legacy_accepted_relations"],
            "note": "research/gaudry_k4_char2_replication_20261026/unit-price/WITHDRAWN.md: the old file is kept unedited; its IC decomposition/trial/relation counters read keys no report has and are withdrawn, as is the statement that zero decompositions were found",
        },
        "readout": "ic_counter_readout (targets_tried, relations_found, matrix_rank, matrix_rows); supersedes the `decompositions`/`trials`/`accepted_relations` readout of unit-price-f2m4-m14-17.json, see WITHDRAWN.md there",
        "rho_steps": o.rho_steps,
    })
    .as_object()
    .cloned()
    .map(|mut d| {
        d.insert("instances".into(), Value::Object(per_m));
        Value::Object(d)
    })
    .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(
        m: u32,
    ) -> (
        Instance,
        crate::cryptanalysis::ecbench::workload::CurveFacts,
        [String; 2],
    ) {
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
        let q = target_of(b, 0x0123_4567_89ab_cdefu128 % b.r);
        let f = &b.curve.f;
        let t = [
            format!("0x{:x}", f.pack(&q.x)),
            format!("0x{:x}", f.pack(&q.y)),
        ];
        (inst, facts, t)
    }

    /// The readout reads the counters `ic_report` emits.  The keys the
    /// withdrawn readout looked up (`trials`, `decompositions`,
    /// `accepted_relations`) are not among them, which is why it recorded
    /// zero; this reads them back as zero and the real keys as nonzero.
    #[test]
    fn the_ic_readout_reads_the_counters_the_report_emits() {
        let emitted = BTreeMap::from([
            ("targets_tried".to_string(), 47_330u64),
            ("relations_found".to_string(), 251),
            ("matrix_rank".to_string(), 214),
            ("matrix_rows".to_string(), 251),
        ]);
        let mut o = BTreeMap::new();
        ic_counter_readout("full", &emitted, &mut o);
        assert_eq!(o["full_targets_tried"], 47_330.0);
        assert_eq!(o["full_relations_found"], 251.0);
        assert_eq!(o["full_matrix_rank"], 214.0);
        assert_eq!(o["full_matrix_rows"], 251.0);
        let mut l = BTreeMap::new();
        ic_counter_readout("legacy", &emitted, &mut l);
        assert_eq!(l["legacy_targets_tried"], 47_330.0);
        // The withdrawn keys read as zero.
        let old = BTreeMap::from([
            ("trials".to_string(), 9u64),
            ("decompositions".to_string(), 9),
            ("accepted_relations".to_string(), 9),
        ]);
        let mut z = BTreeMap::new();
        ic_counter_readout("full", &old, &mut z);
        assert!(z.values().all(|&v| v == 0.0));
    }

    /// End to end at m = 6: a real partial solve of both arms reports the
    /// decomposition trials it ran, and they equal the solve's own counter.
    /// The withdrawn readout reported 0 here.
    #[test]
    fn the_ic_readout_is_nonzero_on_a_real_solve() {
        let (inst, facts, target) = case(6);
        let Instance::F2m4(b) = &inst else {
            unreachable!()
        };
        let o = ic_once(&inst, &facts, b, &target, 400, 3, false).unwrap();
        for k in ["legacy_targets_tried", "full_targets_tried"] {
            assert!(o[k] > 0.0, "{k} = {}", o[k]);
        }
        let direct = solve(
            &ic_method("ic.pipeline_f2m4_full", 400).unwrap(),
            &inst,
            &facts,
            &target,
            3,
        )
        .unwrap();
        assert_eq!(
            o["full_targets_tried"],
            direct.counters["targets_tried"] as f64
        );
        assert_eq!(
            o["full_relations_found"],
            direct.counters["relations_found"] as f64
        );
        assert!(o["full_priced_fme"] > 0.0 && o["full_wall_ns"] > 0.0);
    }

    /// The reference-unit closure at m = 6 (even: divisor 64): the divisor,
    /// the primary and the second model are both recorded, and the priced
    /// total is `total_gae` times the divisor.  The only test that uses
    /// the one-thread pool (it binds the calling thread).
    #[test]
    fn the_reference_closure_prices_by_the_per_m_divisor() {
        let (inst, facts, target) = case(6);
        let Instance::F2m4(b) = &inst else {
            unreachable!()
        };
        for variant in ["ref", "refmm"] {
            let id = format!("rho.negation_f2m4_{variant}");
            let ns_table = table_ns_fq_mul(6, false, variant).unwrap();
            assert!(ns_table > 0.0);
            let v = closure_solve(
                &inst,
                &facts,
                b,
                &target,
                &id,
                variant,
                BTreeMap::new(),
                7,
                ns_table,
                ns_table,
            )
            .unwrap();
            assert_eq!(v["divisor"].as_u64(), Some(64), "{variant}");
            let c = &v["closure"];
            let want = v["total_gae"].as_f64().unwrap() * 64.0;
            assert!((c["priced_fme"].as_f64().unwrap() / want - 1.0).abs() < 1e-12);
            // The second model: its own divisor (42), the same measured CPU.
            let p = &v["closure_poly_free_const"];
            assert!(p["priced_fme"].as_f64().unwrap() > 0.0);
            assert_eq!(p["user_cpu_ns"], c["user_cpu_ns"]);
            assert!(c["user_over_priced"].as_f64().unwrap().is_finite());
        }
        // The current unit has no second model and the schoolbook divisor.
        let v = closure_solve(
            &inst,
            &facts,
            b,
            &target,
            "rho.negation_f2m4_full",
            "full",
            BTreeMap::new(),
            7,
            1.0,
            1.0,
        )
        .unwrap();
        assert_eq!(v["divisor"].as_u64(), Some(fq_muls_per_add(&b.curve)));
        assert!(v.get("closure_poly_free_const").is_none());
    }

    /// The groups of `phase_groups` add up to the priced total they were
    /// given, and each CPU phase lands in exactly one group.
    #[test]
    fn the_phase_groups_add_up_to_the_priced_total() {
        use crate::cryptanalysis::phase_cpu::{Total, NAMES, N_PHASES};
        let priced = json!({
            "report_phases": {
                "factor_base": {"fme": 10.0, "group_ops_fme": 2.0},
                "relations": {"fme": 100.0, "group_ops_fme": 5.0},
                "linear_algebra": {"fme": 3.0, "group_ops_fme": 0.0},
                "verify": {"fme": 1.0, "group_ops_fme": 0.0},
            },
            "precompute_fq_muls": 7,
            "by_native_counter": {"target_guard_probes": {"fme": 1.0}, "walk_steps": {"fme": 0.5}},
            "solver_counted_fq_muls_by_phase": {"echelon": 40},
        });
        let totals: [Total; N_PHASES] = std::array::from_fn(|i| Total {
            cpu_ns: 1000 * (i as u64 + 1),
            faults: 0,
            entries: 1,
        });
        let g = phase_groups(&totals, &priced, 2.0, 114.0);
        let sum: f64 = g["table"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["priced_fme"].as_f64().unwrap())
            .sum();
        assert!((sum - 114.0).abs() < 1e-9, "{sum}");
        assert!((g["sum_priced_fme"].as_f64().unwrap() - 114.0).abs() < 1e-9);
        let listed: usize = g["table"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["cpu_phases"].as_array().unwrap().len())
            .sum();
        assert_eq!(listed, NAMES.len(), "every phase in one group");
        let cpu: f64 = g["table"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["cpu_share"].as_f64().unwrap())
            .sum();
        assert!((cpu - 1.0).abs() < 1e-9, "{cpu}");
    }

    /// The per-phase timers read clocks and change no count: the priced
    /// total and the counters of a solve are identical with them on.
    #[test]
    fn the_phase_timers_change_no_count() {
        let _lock = crate::cryptanalysis::phase_cpu::TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let (inst, facts, target) = case(6);
        let m = ic_method("ic.pipeline_f2m4_full", 400).unwrap();
        let plain = solve(&m, &inst, &facts, &target, 3).unwrap();
        crate::cryptanalysis::phase_cpu::start();
        let timed = solve(&m, &inst, &facts, &target, 3).unwrap();
        let tot = crate::cryptanalysis::phase_cpu::finish();
        assert_eq!(plain.total_gae, timed.total_gae);
        assert_eq!(plain.counters, timed.counters);
        assert!(
            timed.detail["phase_priced"]["solver_systems"]
                .as_u64()
                .unwrap()
                > 0
        );
        assert!(plain.detail.get("phase_priced").is_none());
        assert!(tot.iter().map(|t| t.cpu_ns).sum::<u64>() > 0);
    }
}
