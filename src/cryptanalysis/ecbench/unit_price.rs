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

fn ic_method(
    id: &str,
    trials: u64,
) -> Result<crate::cryptanalysis::ecbench::methods::ResolvedMethod, String> {
    resolve(&MethodSpec {
        id: id.into(),
        params: BTreeMap::from([
            ("factor_base".to_string(), "bin-t-line".to_string()),
            (
                "oracle".to_string(),
                "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1"
                    .to_string(),
            ),
            ("linalg".to_string(), "wiedemann".to_string()),
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
) -> Result<BTreeMap<&'static str, f64>, String> {
    let per_add = fq_muls_per_add(&b.curve) as f64;
    let mut o = BTreeMap::new();
    for (tag, id) in [("legacy", "ic.pipeline"), ("full", "ic.pipeline_f2m4_full")] {
        let m = ic_method(id, trials)?;
        let t = Instant::now();
        let rep = solve(&m, inst, facts, target, seed)?;
        let wall = t.elapsed().as_nanos() as f64;
        let fq = rep.total_gae * per_add;
        let (wk, fk, tk) = match tag {
            "legacy" => ("legacy_wall_ns", "legacy_priced_fq_ops", "legacy_trials"),
            _ => ("full_wall_ns", "full_priced_fme", "full_trials"),
        };
        o.insert(wk, wall);
        o.insert(fk, fq);
        let tr = rep
            .counters
            .get("trials")
            .or_else(|| rep.counters.get("walk_trials"))
            .copied()
            .unwrap_or(0) as f64;
        o.insert(tk, tr);
        if tag == "legacy" {
            for (name, key) in [
                ("legacy_decompositions", "decompositions"),
                ("legacy_accepted_relations", "accepted_relations"),
            ] {
                o.insert(name, rep.counters.get(key).copied().unwrap_or(0) as f64);
            }
            o.insert("recovered", if rep.recovered.is_some() { 1.0 } else { 0.0 });
        }
    }
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
        let ic = if o.ic_trials > 0 {
            let d = 0x0123_4567_89ab_cdefu128 % b.r;
            let q = target_of(b, d);
            let f = &b.curve.f;
            let target = [
                format!("0x{:x}", f.pack(&q.x)),
                format!("0x{:x}", f.pack(&q.y)),
            ];
            let t0 = Instant::now();
            let mut acc: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
            let ic_reps = o.reps.min(3);
            for k in 0..ic_reps {
                for (kk, v) in ic_once(&inst, &facts, b, &target, o.ic_trials, 3 + k as u64)? {
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
            blk.insert(
                "ns_per_legacy_priced_fq_op".into(),
                ratio("legacy_wall_ns", "legacy_priced_fq_ops"),
            );
            blk.insert(
                "ns_per_full_priced_fme".into(),
                ratio("full_wall_ns", "full_priced_fme"),
            );
            blk.insert("ic_trials_requested".into(), json!(o.ic_trials));
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
                "curve": facts.slug,
                "micro": micro,
                "in_situ": { "rho_negation_batched": rb, "rho_negation": ru, "ic_pipeline": ic },
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
