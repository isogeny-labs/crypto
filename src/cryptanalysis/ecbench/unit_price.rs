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
use crate::cryptanalysis::gaudry_quartic_bin::{fq_muls_per_add, BinCurve, PtB};
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
        // The counters `ic_report` emits (methods.rs): `targets_tried`
        // (decomposition trials), `relations_found`, `matrix_*`.  The keys
        // `trials`, `decompositions` and `accepted_relations` that an
        // earlier version of this readout looked up do not exist, so it
        // reported zero for all of them.
        let get = |k: &str| rep.counters.get(k).copied().unwrap_or(0) as f64;
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
) -> Result<BTreeMap<&'static str, f64>, String> {
    let m = resolve(&MethodSpec {
        id: id.into(),
        params: BTreeMap::new(),
    })?;
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
    let mut per_m = serde_json::Map::new();
    for &m in &o.ms {
        let cof = if m >= 15 { 2 } else { 16 };
        let spec = CurveSpec::F2m4Torsion {
            m,
            seed: o.seed,
            max_cofactor: cof,
        };
        let inst = spec.build()?;
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
        let (ns_local, ns_table) = if o.closure {
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
        let rho_full = if rho_full_on {
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
                        rho_full_once(&inst, &facts, b, &target, id, 7 + k as u64, o.closure)?
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
        } else {
            Value::Null
        };
        let ic = if o.ic_trials > 0 || ic_complete {
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
            blk.insert("reps".into(), json!(ic_reps));
            Value::Object(blk)
        } else {
            Value::Null
        };
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
                "in_situ": { "rho_negation_batched": rb, "rho_negation": ru, "ic_pipeline": ic, "rho_full": rho_full },
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
