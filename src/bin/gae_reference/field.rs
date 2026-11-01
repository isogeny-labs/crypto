//! `gae-reference field`: the field-level speedups behind the class (b) native
//! units of the tuned GAE table.
//!
//! The legacy unit prices a native operation in additions of the *untuned*
//! reference group (`u128 %` per field multiplication and an extended-Euclid
//! inversion on a prime curve; one Itoh-Tsujii inversion per addition on a
//! binary curve).  A native operation that runs on the same untuned
//! arithmetic would also speed up under a tuned definition of the unit.  This
//! command times, per table curve, the exact reference functions the
//! class (b) units are measured on against the tuned kernels, after checking
//! that the two agree on every input timed:
//!
//! * **prime `sqrt`**: `PrimeCurve::sqrt` (`powmod` over `mulmod`, or
//!   Tonelli-Shanks) against the same algorithm over `prime_fast::Fp64`.
//! * **prime `inversion`**: `ic_boundary::invmod` against `Fp64::inv`
//!   (`f_inv_single`) and against `Fp64::batch_inv` amortised over 256
//!   independent inversions (`f_inv_batched`).
//! * **binary `field_inv`**: `Gf2::inv` against `Gf2::batch_inv` over 256
//!   lanes (the shared-inversion form the tuned addition uses); the single
//!   inversion is the same kernel, so `f_inv_single` is 1.
//! * **`row_op`** (both families): `calibrate_row_ops(r)`, which is `u128 %`
//!   arithmetic modulo the subgroup order, against the same loop over
//!   `Fp64::new(r)`.
//! * **binary `field_mul` / `field_sqr`**: the reference and the tuned
//!   addition call the same `Gf2` kernels, so the applied factor is 1; the
//!   chained-latency and 256-lane-throughput times are recorded as the
//!   sensitivity `f_mul_lanes` / `f_sqr_lanes` and are not applied.
//!
//! It also records the reference kernels' times on this host
//! (`host_local_ns`), so that `retable` can state how far the legacy ratios
//! (measured on another host) drift from this host's.
//!
//! One JSON object per table curve on stdout.  A result that disagrees with
//! the reference aborts with status 2.

use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    calibrate_binary_instance, calibrate_group, calibrate_row_ops, invmod, Calibration, PrimeCurve,
    PrimeInstance,
};
use crypto_lib::cryptanalysis::prime_fast::Fp64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};

use super::{
    calibration_rows, cpu_model, median, random_multiples, resolve_calibration_inst,
    resolve_slug_inst, BinaryInstance, Inst,
};

/// Lanes sharing one inversion, as in the tuned addition.
const LANES: usize = 256;

#[derive(Clone, Copy)]
struct Opts {
    passes: usize,
}

/// Median over `passes` timed passes of `f`, in ns per `units` operations.
fn time_ns(passes: usize, units: f64, mut f: impl FnMut()) -> f64 {
    f();
    let mut v = Vec::with_capacity(passes);
    for _ in 0..passes {
        let t = Instant::now();
        f();
        v.push(t.elapsed().as_nanos() as f64 / units);
    }
    median(v)
}

fn median3(mut f: impl FnMut() -> f64) -> f64 {
    median((0..3).map(|_| f()).collect())
}

fn mismatch(what: &str, label: &str, detail: String) -> String {
    format!("MISMATCH {what} on {label}: {detail}")
}

// ── The arithmetic of `calibrate_row_ops` ───────────────────────────────

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn submod(a: u64, b: u64, m: u64) -> u64 {
    (a + m - b) % m
}

/// The multiply-subtract loop of `calibrate_row_ops` with `u128 %`: the
/// reference result.
fn row_reference(m: u64, pivot: &[u64], row: &mut [u64], reps: u64) {
    for k in 0..reps {
        let f = (k + 3) % m;
        for j in 0..pivot.len() {
            row[j] = submod(row[j], mulmod(f, pivot[j], m), m);
        }
    }
}

/// The same loop over the Montgomery field of the modulus.
fn row_tuned(f: &Fp64, m: u64, pivot: &[u64], row: &mut [u64], reps: u64) {
    for k in 0..reps {
        let fac = f.to_mont((k + 3) % m);
        for j in 0..pivot.len() {
            row[j] = f.sub(row[j], f.mul(fac, pivot[j]));
        }
    }
}

/// Whether the tuned row-operation loop reproduces the reference one on a
/// small fixed problem modulo `m`.
fn row_kernels_agree(m: u64) -> bool {
    let f = Fp64::new(m);
    let mut rng = StdRng::seed_from_u64(m ^ 0x70B);
    let cols = 96usize;
    let pivot: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..m)).collect();
    let row0: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..m)).collect();
    let mut a = row0.clone();
    row_reference(m, &pivot, &mut a, 40);
    let pm: Vec<u64> = pivot.iter().map(|&v| f.to_mont(v)).collect();
    let mut b: Vec<u64> = row0.iter().map(|&v| f.to_mont(v)).collect();
    row_tuned(&f, m, &pm, &mut b, 40);
    a.iter().zip(&b).all(|(&x, &y)| x == f.from_mont(y))
}

/// Reference and tuned ns per row operation modulo `m`: the reference is
/// the repository's `calibrate_row_ops`, the tuned loop has its shape.
fn row_op_ns(m: u64, label: &str, o: Opts) -> Result<(f64, f64), String> {
    if m <= 3 || m & 1 == 0 || m >= (1u64 << 62) {
        return Err(format!(
            "{label}: subgroup order {m} is outside Fp64's range"
        ));
    }
    if !row_kernels_agree(m) {
        return Err(mismatch("row_op", label, format!("modulus {m}")));
    }
    let reference = median3(|| {
        let mut c = Calibration::default();
        calibrate_row_ops(m, &mut c);
        c.ns_per_row_op
    });
    let f = Fp64::new(m);
    let mut rng = StdRng::seed_from_u64(11);
    let cols = 2048usize;
    let pivot: Vec<u64> = (0..cols).map(|_| f.to_mont(rng.gen_range(0..m))).collect();
    let mut row: Vec<u64> = (0..cols).map(|_| f.to_mont(rng.gen_range(0..m))).collect();
    let reps = 500u64;
    let tuned = time_ns(o.passes.min(7), (reps * cols as u64) as f64, || {
        row_tuned(&f, m, &pivot, &mut row, reps);
        std::hint::black_box(&row);
    });
    Ok((reference, tuned))
}

// ── Prime ───────────────────────────────────────────────────────────────

/// A square root of the Montgomery value `a` by the reference algorithm
/// (`p = 3 mod 4`: one power and a check; otherwise Euler test and
/// Tonelli-Shanks), over `Fp64`.
fn tuned_sqrt(f: &Fp64, a: u64) -> Option<u64> {
    let p = f.p;
    if a == 0 {
        return Some(0);
    }
    if p % 4 == 3 {
        let r = f.pow(a, (p + 1) / 4);
        return (f.sqr(r) == a).then_some(r);
    }
    f.sqrt(a)
}

fn field_prime(label: &str, inst: &PrimeInstance, o: Opts) -> Result<Value, String> {
    let curve: &PrimeCurve = &inst.curve;
    let p = curve.p;
    let f = Fp64::new(p);
    let mut rng = StdRng::seed_from_u64(p ^ 0xF1E1D);
    let mut residues = Vec::new();
    while residues.len() < 2048 {
        let v = rng.gen_range(1..p);
        if curve.legendre(v) == 1 {
            residues.push(v);
        }
    }
    let values: Vec<u64> = (0..2048).map(|_| rng.gen_range(1..p)).collect();

    // Verification before any timing.
    let res_m: Vec<u64> = residues.iter().map(|&v| f.to_mont(v)).collect();
    for (&v, &vm) in residues.iter().zip(&res_m) {
        let want = curve
            .sqrt(v)
            .ok_or_else(|| mismatch("sqrt", label, format!("{v}")))?;
        let got = tuned_sqrt(&f, vm).map(|r| f.from_mont(r));
        if got != Some(want) && got != Some(p - want) {
            return Err(mismatch(
                "sqrt",
                label,
                format!("{v}: ref {want}, tuned {got:?}"),
            ));
        }
    }
    let val_m: Vec<u64> = values.iter().map(|&v| f.to_mont(v)).collect();
    for (&v, &vm) in values.iter().zip(&val_m) {
        let want = invmod(v, p).ok_or_else(|| mismatch("inv", label, format!("{v}")))?;
        if f.from_mont(f.inv(vm)) != want {
            return Err(mismatch("inv", label, format!("single {v}")));
        }
    }
    let mut scratch = Vec::new();
    for chunk in val_m.chunks(LANES) {
        let mut buf = chunk.to_vec();
        f.batch_inv(&mut buf, &mut scratch);
        for (&x, &y) in chunk.iter().zip(&buf) {
            if f.mul(x, y) != f.one {
                return Err(mismatch("inv", label, "batched".into()));
            }
        }
    }

    let n_sqrt = (8 * residues.len()) as f64;
    let sqrt_ref = time_ns(o.passes, n_sqrt, || {
        let mut acc = 0u64;
        for _ in 0..8 {
            for &v in &residues {
                acc ^= curve.sqrt(v).unwrap_or(0);
            }
        }
        std::hint::black_box(acc);
    });
    let sqrt_tuned = time_ns(o.passes, n_sqrt, || {
        let mut acc = 0u64;
        for _ in 0..8 {
            for &v in &res_m {
                acc ^= tuned_sqrt(&f, v).unwrap_or(0);
            }
        }
        std::hint::black_box(acc);
    });
    let n_val = (8 * values.len()) as f64;
    let leg_ref = time_ns(o.passes, n_val, || {
        let mut acc = 0i32;
        for _ in 0..8 {
            for &v in &values {
                acc += curve.legendre(v);
            }
        }
        std::hint::black_box(acc);
    });
    let inv_ref = time_ns(o.passes, n_val, || {
        let mut acc = 0u64;
        for _ in 0..8 {
            for &v in &values {
                acc ^= invmod(v, p).unwrap_or(0);
            }
        }
        std::hint::black_box(acc);
    });
    let inv_single = time_ns(o.passes, n_val, || {
        let mut acc = 0u64;
        for _ in 0..8 {
            for &v in &val_m {
                acc ^= f.inv(v);
            }
        }
        std::hint::black_box(acc);
    });
    let mut buf = vec![0u64; LANES];
    let inv_batched = time_ns(o.passes, n_val, || {
        let mut acc = 0u64;
        for _ in 0..8 {
            for chunk in val_m.chunks(LANES) {
                buf.clear();
                buf.extend_from_slice(chunk);
                f.batch_inv(&mut buf, &mut scratch);
                acc ^= buf[0];
            }
        }
        std::hint::black_box(acc);
    });
    let (row_ref, row_tuned_ns) = row_op_ns(inst.r, label, o)?;

    // Host-local reference add and doubling on the instance's own points.
    let g = inst.generator_point();
    let pts = random_multiples(curve, g, 64, inst.r, &mut rng);
    let (mut add, mut dbl) = (Vec::new(), Vec::new());
    for _ in 0..3 {
        let mut c = Calibration::default();
        calibrate_group(curve, &pts, &mut c);
        add.push(c.ns_per_add);
        dbl.push(c.ns_per_double);
    }
    let mut local = Map::new();
    local.insert("add".into(), json!(median(add)));
    local.insert("double".into(), json!(median(dbl)));
    local.insert("sqrt".into(), json!(sqrt_ref));
    local.insert("legendre".into(), json!(leg_ref));
    local.insert("inversion".into(), json!(inv_ref));
    local.insert("row_op".into(), json!(row_ref));
    Ok(json!({
        "curve": label,
        "family": "prime",
        "field_bits": 64 - p.leading_zeros(),
        "order_bits": 64 - inst.r.leading_zeros(),
        "p_mod_4": p % 4,
        "sqrt_ref_ns": sqrt_ref,
        "sqrt_tuned_ns": sqrt_tuned,
        "f_sqrt": sqrt_ref / sqrt_tuned,
        "inv_ref_ns": inv_ref,
        "inv_single_tuned_ns": inv_single,
        "inv_batched_tuned_ns": inv_batched,
        "f_inv_single": inv_ref / inv_single,
        "f_inv_batched": inv_ref / inv_batched,
        "row_op_ref_ns": row_ref,
        "row_op_tuned_ns": row_tuned_ns,
        "f_row_op": row_ref / row_tuned_ns,
        "host_local_ns": local,
        "verified": {
            "sqrt": residues.len(),
            "inversions_single": values.len(),
            "inversions_batched": values.len(),
            "row_op_problem": "96 columns x 40 pivots modulo the subgroup order",
        },
        "passes": o.passes,
        "kernel": "montgomery-u64 REDC (prime_fast::Fp64)",
        "host_cpu_model": cpu_model(),
    }))
}

// ── Binary ──────────────────────────────────────────────────────────────

fn field_binary(label: &str, inst: &BinaryInstance, o: Opts) -> Result<Value, String> {
    let gf = &inst.gf;
    let mut rng = StdRng::seed_from_u64(inst.r ^ 0xF1E1D);
    let mask = (1u64 << inst.n) - 1;
    let xs: Vec<u64> = (0..1024)
        .map(|_| (rng.gen::<u64>() & mask).max(1))
        .collect();

    // Verification before any timing: the batched inverse is the inverse.
    let mut scratch = Vec::new();
    for chunk in xs.chunks(LANES) {
        let mut buf = chunk.to_vec();
        gf.batch_inv(&mut buf, &mut scratch);
        for (&x, &y) in chunk.iter().zip(&buf) {
            if y != gf.inv(x) || gf.mul(x, y) != 1 {
                return Err(mismatch("field_inv", label, format!("{x}")));
            }
        }
    }
    let n = xs.len() as f64;
    let inv_ref = time_ns(o.passes, n * 4.0, || {
        let mut acc = 0u64;
        for _ in 0..4 {
            for &x in &xs {
                acc ^= gf.inv(x);
            }
        }
        std::hint::black_box(acc);
    });
    let mut buf = vec![0u64; LANES];
    let inv_batched = time_ns(o.passes, n * 4.0, || {
        let mut acc = 0u64;
        for _ in 0..4 {
            for chunk in xs.chunks(LANES) {
                buf.clear();
                buf.extend_from_slice(chunk);
                gf.batch_inv(&mut buf, &mut scratch);
                acc ^= buf[0];
            }
        }
        std::hint::black_box(acc);
    });

    // Multiplication and squaring: chained latency (the legacy
    // measurement, `full_price::field_and_lookup_ns`) and 256-lane
    // throughput (the tuned step's regime), for the record.
    let chain = 4096u64;
    let mul_chain = time_ns(o.passes, chain as f64, || {
        let mut acc = xs[0];
        for i in 0..chain {
            acc = gf.mul(acc, xs[(i % 1024) as usize]) ^ 1;
        }
        std::hint::black_box(acc);
    });
    let sqr_chain = time_ns(o.passes, chain as f64, || {
        let mut acc = xs[1];
        for i in 0..chain {
            acc = gf.sqr(acc) ^ xs[(i % 1024) as usize];
        }
        std::hint::black_box(acc);
    });
    let mut lanes: Vec<u64> = xs[..LANES].to_vec();
    let rounds = 16u64;
    let mul_lanes = time_ns(o.passes, (rounds as usize * LANES) as f64, || {
        for _ in 0..rounds {
            for (v, &t) in lanes.iter_mut().zip(&xs[LANES..2 * LANES]) {
                *v = gf.mul(*v, t) ^ 1;
            }
        }
        std::hint::black_box(&lanes);
    });
    let sqr_lanes = time_ns(o.passes, (rounds as usize * LANES) as f64, || {
        for _ in 0..rounds {
            for (v, &t) in lanes.iter_mut().zip(&xs[LANES..2 * LANES]) {
                *v = gf.sqr(*v) ^ t;
            }
        }
        std::hint::black_box(&lanes);
    });
    let (row_ref, row_tuned_ns) = row_op_ns(inst.r, label, o)?;

    // Host-local reference kernels through the repository's calibrator.
    let cals: Vec<Calibration> = (0..3).map(|_| calibrate_binary_instance(inst)).collect();
    let med = |f: &dyn Fn(&Calibration) -> Option<f64>| -> Option<f64> {
        let v: Vec<f64> = cals.iter().filter_map(f).collect();
        (!v.is_empty()).then(|| median(v))
    };
    let mut local = Map::new();
    local.insert("add".into(), json!(med(&|c| Some(c.ns_per_add))));
    local.insert("double".into(), json!(med(&|c| Some(c.ns_per_double))));
    local.insert("row_op".into(), json!(med(&|c| Some(c.ns_per_row_op))));
    local.insert("as_solve".into(), json!(med(&|c| c.ns_per_as_solve)));
    local.insert("frobenius".into(), json!(med(&|c| c.ns_per_frobenius)));
    local.insert("canon".into(), json!(med(&|c| c.ns_per_canon)));
    local.insert("word_xor".into(), json!(med(&|c| c.ns_per_word_xor)));
    local.insert("field_inv".into(), json!(inv_ref));
    local.insert("field_mul".into(), json!(mul_chain));
    local.insert("field_sqr".into(), json!(sqr_chain));
    Ok(json!({
        "curve": label,
        "family": "binary",
        "field_bits": inst.n,
        "order_bits": 64 - inst.r.leading_zeros(),
        "inv_ref_ns": inv_ref,
        "inv_batched_tuned_ns": inv_batched,
        "f_inv_single": 1.0,
        "f_inv_batched": inv_ref / inv_batched,
        "mul_chained_ns": mul_chain,
        "mul_lanes_ns": mul_lanes,
        "sqr_chained_ns": sqr_chain,
        "sqr_lanes_ns": sqr_lanes,
        "f_mul": 1.0,
        "f_sqr": 1.0,
        "f_mul_lanes": mul_chain / mul_lanes,
        "f_sqr_lanes": sqr_chain / sqr_lanes,
        "row_op_ref_ns": row_ref,
        "row_op_tuned_ns": row_tuned_ns,
        "f_row_op": row_ref / row_tuned_ns,
        "host_local_ns": local,
        "verified": {
            "field_inv_batched": xs.len(),
            "row_op_problem": "96 columns x 40 pivots modulo the subgroup order",
        },
        "passes": o.passes,
        "kernel": "Gf2 (pclmulqdq fold when available) + Gf2::batch_inv",
        "host_cpu_model": cpu_model(),
    }))
}

/// `gae-reference field --from-calibration [--only SUBSTR] [--passes N]`.
pub fn run(args: &[String]) -> i32 {
    let mut only: Option<String> = None;
    let mut o = Opts { passes: 15 };
    let mut from_cal = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--from-calibration" => from_cal = true,
            "--only" => {
                i += 1;
                only = args.get(i).cloned();
            }
            "--passes" => {
                i += 1;
                o.passes = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(15)
                    .max(1);
            }
            other => {
                eprintln!("field: unknown argument {other}");
                return 64;
            }
        }
        i += 1;
    }
    if !from_cal {
        eprintln!("field: give --from-calibration");
        return 64;
    }
    let mut failed = false;
    for (key, is_slug) in calibration_rows() {
        if only.as_ref().is_some_and(|s| !key.contains(s.as_str())) {
            continue;
        }
        let inst = if is_slug {
            resolve_slug_inst(&key)
        } else {
            resolve_calibration_inst(&key)
        };
        let out = inst.and_then(|inst| match &inst {
            Inst::Prime(label, i) => field_prime(label, i, o),
            Inst::Binary(label, i) => field_binary(label, i, o),
        });
        match out {
            Ok(v) => println!("{v}"),
            Err(e) if e.starts_with("MISMATCH") => {
                eprintln!("FATAL: {e}");
                return 2;
            }
            Err(e) => {
                eprintln!("error: {key}: {e}");
                failed = true;
            }
        }
    }
    i32::from(failed)
}
