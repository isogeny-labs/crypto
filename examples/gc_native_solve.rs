//! `gc_native_solve`: one JSON request on stdin, one JSON answer on stdout.
//!
//! Native (Rust) solver arms for the Gröbner solver shootout
//! (`research/groebner_solver_shootout_20261116`, Python side
//! `groebner_compare/native_worker.py`).  The Boolean system is given exactly
//! as `groebner_compare/semaev.py` writes it: `nvars` unknowns and a list of
//! equations, each a list of monomial bitmasks (bit `i` is `x_i`, mask `0` is
//! the constant 1) over GF(2)[x]/(x_i^2 + x_i).  No re-encoding: variable `i`
//! of the instance is variable `i` of the solver.
//!
//! Requests (`op`):
//!
//! * `solve`  — any engine of the `ic` solver registry by name
//!   (`solver_by_name`), default `inherited-f4` with its defaults
//!   (max_degree 3, split auto, node_budget 4096) unless `params` override.
//! * `learn`  — Joux–Vitse record of the dense degree-`degree` Macaulay step
//!   (`koblitz_groebner::matrix_f4_f2_record_counted`) on every training
//!   system, merged with `F4Trace::merge`; returns the trace and the word
//!   XORs and wall time the recording cost (to be charged to the arm).
//! * `replay` — `matrix_f4_f2_replay_checked` of a learned trace on one
//!   system (audit `audit`, default `DEFAULT_AUDIT` = 16); on refusal the full
//!   kernel `matrix_f4_f2_counted` is run instead (charged too) when
//!   `fallback` is true; roots come from the affine elements of the reduced
//!   Macaulay basis (`solutions_from_linear_part` below, a port of
//!   `pq_f4_f2::solutions_from_linear_part` at HEAD; every returned point is
//!   checked against the original equations), refused above `max_free` free
//!   variables.
//!
//! Nothing here is performance evidence by itself; it is an instrument.

use std::collections::{BTreeMap, HashSet};
use std::io::Read;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_framework::solvers::solver_by_name;
use crypto_lib::cryptanalysis::ic_framework::stages::{BooleanSystem, Params, SolverVerdict};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    matrix_f4_f2_counted, matrix_f4_f2_record_counted, matrix_f4_f2_replay_checked, F4Trace,
    F4TraceStep, DEFAULT_AUDIT,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly, Mask, MASK_BITS};
use serde_json::{json, Value};

fn fail(msg: impl Into<String>) -> ! {
    println!("{}", json!({"status": "error", "reason": msg.into()}));
    std::process::exit(2);
}

fn parse_system(nvars: usize, eqs: &Value) -> Vec<F2BoolPoly> {
    if nvars > MASK_BITS || nvars > 64 {
        fail(format!("{nvars} unknowns exceed the 64-bit mask build"));
    }
    let rows = eqs
        .as_array()
        .unwrap_or_else(|| fail("equations must be a list"));
    rows.iter()
        .map(|row| {
            let monos: Vec<F2BoolMono> = row
                .as_array()
                .unwrap_or_else(|| fail("an equation must be a list of masks"))
                .iter()
                .map(|m| {
                    let mask = m
                        .as_u64()
                        .unwrap_or_else(|| fail("a monomial mask must be a u64"));
                    if nvars < 64 && mask >> nvars != 0 {
                        fail(format!("mask {mask} uses a variable past nvars = {nvars}"));
                    }
                    F2BoolMono::from_mask(mask as Mask)
                })
                .collect();
            F2BoolPoly::from_monos(monos, nvars)
        })
        .collect()
}

fn degree(p: &F2BoolPoly) -> u32 {
    p.terms
        .iter()
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(0)
}

/// Every solution of `equations` on the affine space cut out by the degree-<=1
/// elements of `basis` (any generating set of the same ideal): Gauss-Jordan on
/// the affine elements, then each of the `2^free` points checked against
/// `equations`. `None` above `max_free` free variables. Sound and complete:
/// every zero of the ideal satisfies the affine elements, and every returned
/// point satisfies `equations`. Port of `pq_f4_f2::solutions_from_linear_part`
/// (crypto HEAD f8480787ec), kept here so the example builds on commits that
/// predate it. `tests` counts monomial tests, `xors` the row XORs.
fn solutions_from_linear_part(
    basis: &[F2BoolPoly],
    equations: &[F2BoolPoly],
    n_vars: usize,
    max_free: usize,
    tests: &mut u64,
    xors: &mut u64,
) -> Option<Vec<u64>> {
    let is_one = |p: &F2BoolPoly| p.terms.len() == 1 && p.terms[0].mask == 0;
    if basis.iter().any(is_one) {
        return Some(Vec::new());
    }
    let mut pivots: Vec<(usize, u64, u64)> = Vec::new();
    for g in basis {
        if degree(g) > 1 {
            continue;
        }
        let (mut mask, mut c) = (0u64, 0u64);
        for t in &g.terms {
            if t.mask == 0 {
                c ^= 1;
            } else {
                mask ^= t.mask;
            }
        }
        for &(v, pm, pc) in &pivots {
            if (mask >> v) & 1 == 1 {
                mask ^= pm;
                c ^= pc;
                *xors += 1;
            }
        }
        if mask == 0 {
            if c == 1 {
                return Some(Vec::new());
            }
            continue;
        }
        let v = mask.trailing_zeros() as usize;
        for p in pivots.iter_mut() {
            if (p.1 >> v) & 1 == 1 {
                p.1 ^= mask;
                p.2 ^= c;
                *xors += 1;
            }
        }
        pivots.push((v, mask, c));
    }
    let pinned = pivots.iter().fold(0u64, |acc, p| acc | (1u64 << p.0));
    let free: Vec<usize> = (0..n_vars).filter(|&v| (pinned >> v) & 1 == 0).collect();
    if free.len() > max_free || free.len() >= 63 {
        return None;
    }
    let mut out = Vec::new();
    for a in 0u64..(1u64 << free.len()) {
        let mut point = 0u64;
        for (b, &v) in free.iter().enumerate() {
            if (a >> b) & 1 == 1 {
                point |= 1u64 << v;
            }
        }
        for &(v, m, c) in &pivots {
            *tests += 1;
            let rest = m & !(1u64 << v);
            if ((point & rest).count_ones() as u64 + c) & 1 == 1 {
                point |= 1u64 << v;
            }
        }
        let mut ok = true;
        for e in equations {
            *tests += e.terms.len() as u64;
            if e.eval(point as Mask) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(point);
        }
    }
    Some(out)
}

fn masks(points: &[Mask]) -> Vec<u64> {
    let mut v: Vec<u64> = points.to_vec();
    v.sort_unstable();
    v.dedup();
    v
}

fn trace_to_json(t: &F4Trace) -> Value {
    json!({
        "n_vars": t.n_vars,
        "degree": t.degree,
        "poly_degrees": t.poly_degrees,
        "rank": t.rank,
        "leading_by_degree": t.leading_by_degree,
        "record_word_ops": t.record_word_ops,
        "steps": t.steps.iter().map(|s| json!({
            "multiplier_degree": s.multiplier_degree,
            "rows": s.rows,
            "survivors": s.survivors.iter().map(|&(i, m)| json!([i, m])).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn trace_from_json(v: &Value) -> F4Trace {
    let u = |k: &str| {
        v[k].as_u64()
            .unwrap_or_else(|| fail(format!("trace.{k} missing")))
    };
    let list = |k: &str| -> Vec<u64> {
        v[k].as_array()
            .unwrap_or_else(|| fail(format!("trace.{k} missing")))
            .iter()
            .map(|x| {
                x.as_u64()
                    .unwrap_or_else(|| fail(format!("trace.{k} entry")))
            })
            .collect()
    };
    let steps = v["steps"]
        .as_array()
        .unwrap_or_else(|| fail("trace.steps missing"))
        .iter()
        .map(|s| F4TraceStep {
            multiplier_degree: s["multiplier_degree"].as_u64().unwrap_or(0) as u32,
            rows: s["rows"].as_u64().unwrap_or(0) as usize,
            survivors: s["survivors"]
                .as_array()
                .unwrap_or_else(|| fail("trace step survivors"))
                .iter()
                .map(|p| {
                    (
                        p[0].as_u64().unwrap_or_else(|| fail("survivor index")) as u32,
                        p[1].as_u64().unwrap_or_else(|| fail("survivor mask")),
                    )
                })
                .collect(),
        })
        .collect();
    F4Trace {
        n_vars: u("n_vars") as usize,
        degree: u("degree") as u32,
        poly_degrees: list("poly_degrees").into_iter().map(|d| d as u32).collect(),
        steps,
        rank: u("rank") as usize,
        leading_by_degree: list("leading_by_degree")
            .into_iter()
            .map(|d| d as usize)
            .collect(),
        record_word_ops: u("record_word_ops"),
    }
}

/// Columns (distinct monomials) of the replayed rows: telemetry only,
/// computed after the timed section.
fn replay_columns(polys: &[F2BoolPoly], trace: &F4Trace) -> usize {
    let mut cols: HashSet<u64> = HashSet::new();
    for s in &trace.steps {
        for &(i, t) in &s.survivors {
            if let Some(p) = polys.get(i as usize) {
                let mut count: BTreeMap<u64, u32> = BTreeMap::new();
                for term in &p.terms {
                    *count.entry(term.mask | t).or_default() += 1;
                }
                cols.extend(
                    count
                        .into_iter()
                        .filter(|(_, c)| c % 2 == 1)
                        .map(|(m, _)| m),
                );
            }
        }
    }
    cols.len()
}

fn op_solve(req: &Value) -> Value {
    let nvars = req["nvars"]
        .as_u64()
        .unwrap_or_else(|| fail("nvars missing")) as usize;
    let system = BooleanSystem {
        equations: parse_system(nvars, &req["equations"]),
        n_vars: nvars,
    };
    let engine = req["engine"].as_str().unwrap_or("inherited-f4");
    let solver = solver_by_name(engine).unwrap_or_else(|e| fail(e));
    let mut params = Params::default();
    if let Some(map) = req["params"].as_object() {
        for (k, v) in map {
            let s = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            params.0.insert(k.clone(), s);
        }
    }
    if !solver.accepts(&system.shape()) {
        return json!({"status": "unavailable", "engine": engine,
                      "reason": format!("{engine} declines a system of {nvars} unknowns")});
    }
    let started = Instant::now();
    let (verdict, cost) = solver.solve(&system, &params, None);
    let wall_ns = started.elapsed().as_nanos() as u64;
    let (status, roots) = match verdict {
        SolverVerdict::Solved(p) => ("ok", Some(masks(&p))),
        SolverVerdict::Unsatisfiable => ("ok", Some(Vec::new())),
        SolverVerdict::BudgetExceeded => ("unknown", None),
    };
    json!({
        "status": status,
        "engine": engine,
        "finds_every_solution": solver.finds_every_solution(),
        "roots": roots,
        "solver_wall_ns": wall_ns,
        "degree_reached": cost.degree_reached,
        "ops": cost.ops,
        "op_unit": cost.op_unit,
        "timed_out": cost.timed_out,
        "extra": cost.extra,
        "params": params.0,
    })
}

fn op_learn(req: &Value) -> Value {
    let nvars = req["nvars"]
        .as_u64()
        .unwrap_or_else(|| fail("nvars missing")) as usize;
    let degree = req["degree"]
        .as_u64()
        .unwrap_or_else(|| fail("degree missing")) as u32;
    let systems = req["systems"]
        .as_array()
        .unwrap_or_else(|| fail("systems missing"));
    let started = Instant::now();
    let mut merged: Option<F4Trace> = None;
    let mut kernel_ops = 0u64;
    let mut refused = Vec::new();
    let mut max_rows = 0usize;
    for (k, s) in systems.iter().enumerate() {
        let polys = parse_system(nvars, s);
        let Some((_, ops, trace)) = matrix_f4_f2_record_counted(&polys, nvars, degree) else {
            return json!({"status": "unknown", "reason": format!(
                "training system {k}: degree-{degree} Macaulay matrix exceeds the size limits")});
        };
        kernel_ops += ops;
        max_rows = max_rows.max(trace.total_rows());
        match merged.as_mut() {
            None => merged = Some(trace),
            Some(m) => {
                if !m.merge(&trace) {
                    refused.push(k);
                }
            }
        }
    }
    let wall_ns = started.elapsed().as_nanos() as u64;
    let Some(trace) = merged else {
        return json!({"status": "error", "reason": "no training systems"});
    };
    json!({
        "status": "ok",
        "trace": trace_to_json(&trace),
        "learn_wall_ns": wall_ns,
        "learn_kernel_word_xors": kernel_ops,
        "learn_record_word_xors": trace.record_word_ops,
        "training_systems": systems.len(),
        "training_shape_mismatch": refused,
        "max_rows": max_rows,
        "survivors": trace.total_survivors(),
    })
}

fn op_replay(req: &Value) -> Value {
    let nvars = req["nvars"]
        .as_u64()
        .unwrap_or_else(|| fail("nvars missing")) as usize;
    let polys = parse_system(nvars, &req["equations"]);
    let trace = trace_from_json(&req["trace"]);
    let audit = req["audit"].as_u64().unwrap_or(DEFAULT_AUDIT as u64) as usize;
    let max_free = req["max_free"].as_u64().unwrap_or(20) as usize;
    let fallback = req["fallback"].as_bool().unwrap_or(true);
    let started = Instant::now();
    let (basis, ops, path, refusal, refused_ops) =
        match matrix_f4_f2_replay_checked(&polys, nvars, trace.degree, &trace, audit) {
            Ok((b, ops)) => (Some(b), ops, "replay", None, 0u64),
            Err((why, spent)) if fallback => {
                match matrix_f4_f2_counted(&polys, nvars, trace.degree) {
                    Some((b, ops)) => (
                        Some(b),
                        ops,
                        "fallback-full",
                        Some(format!("{why:?}")),
                        spent,
                    ),
                    None => (
                        None,
                        0,
                        "fallback-oversize",
                        Some(format!("{why:?}")),
                        spent,
                    ),
                }
            }
            Err((why, spent)) => (None, 0, "refused", Some(format!("{why:?}")), spent),
        };
    let mut tests = 0u64;
    let mut xors = 0u64;
    let roots = basis.as_ref().and_then(|b| {
        solutions_from_linear_part(b, &polys, nvars, max_free, &mut tests, &mut xors)
    });
    let wall_ns = started.elapsed().as_nanos() as u64;
    let linear = basis
        .as_ref()
        .map(|b| b.iter().filter(|p| degree(p) <= 1).count());
    let status = match (&basis, &roots) {
        (Some(_), Some(_)) => "ok",
        _ => "unknown",
    };
    let reason = match (&basis, &roots) {
        (None, _) => Some(format!("no basis ({path})")),
        (Some(_), None) => Some(format!(
            "degree-{} Macaulay basis leaves more than {max_free} free variables",
            trace.degree
        )),
        _ => None,
    };
    let rows = if path == "replay" {
        trace.total_survivors()
    } else {
        trace.total_rows()
    };
    let cols = if path == "replay" {
        Some(replay_columns(&polys, &trace))
    } else {
        None
    };
    json!({
        "status": status,
        "reason": reason,
        "engine": "trace-f4",
        "path": path,
        "refusal": refusal,
        "roots": roots.map(|r| { let mut r = r; r.sort_unstable(); r.dedup(); r }),
        "solver_wall_ns": wall_ns,
        "degree_reached": trace.degree,
        "elimination_word_xors": ops,
        "refused_attempt_word_xors": refused_ops,
        "extraction_word_xors": xors,
        "extraction_tests": tests,
        "basis_len": basis.as_ref().map(|b| b.len()),
        "basis_linear_elements": linear,
        "matrix_rows": rows,
        "matrix_cols_replayed": cols,
    })
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .unwrap_or_else(|e| fail(format!("stdin: {e}")));
    let req: Value = serde_json::from_str(&input).unwrap_or_else(|e| fail(format!("json: {e}")));
    let out = match req["op"].as_str().unwrap_or("solve") {
        "solve" => op_solve(&req),
        "learn" => op_learn(&req),
        "replay" => op_replay(&req),
        other => fail(format!("unknown op {other}")),
    };
    println!("{out}");
}
