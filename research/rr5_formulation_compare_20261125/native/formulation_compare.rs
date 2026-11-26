#![allow(static_mut_refs)]
//! Same-input comparison of RR5 extraction formulations on the frozen B677 M31 base.
//!
//! Arms, on identical public targets and identical caps:
//!   coupled   the unchanged 122-variable structured circuit (`extract_hybrid677`);
//!   sectors   the five coprime-selector sector systems (`construct_hybrid677_sector`,
//!             PR #5119), solved with the same native bounded F4 visitor, quartic root
//!             extraction and signed replay, under ONE shared deadline for all five;
//!   sector-k  one sector alone with the full budget (diagnostic only; a single sector
//!             cannot establish completeness).
//! Every reported relation is replayed on the curve and checked to sum to the target.
//! A cap, deadline or resource stop is incomplete, never UNSAT.
//!
//! Usage: rr5-formulation-compare --arm coupled|sectors|sector0..sector4 --target <name>

mod graeffe_jets;
use crypto_lib::cryptanalysis::f4_fp::{
    normalise, solve_visit_traced, F4Options, F4ResourceLimits, Ordering, Poly, VisitEvent,
};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use prime31_four_summand::extractor::{extract_hybrid677, Limits};
use prime31_four_summand::selector_sectors::{construct_hybrid677_sector, ConstructionLimits};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

/// Frozen #4974 protocol caps; `--scale s` multiplies the three resource caps
/// by s and `--seconds` overrides the deadline (resource ladder).
static mut SECONDS: u64 = 30;
static mut MAX_DEGREE: u32 = 10;
static mut RESOURCES: F4ResourceLimits = F4ResourceLimits {
    max_pairs: 20_000,
    max_symbolic_monomials: 8_192,
    max_matrix_entries: 2_000_000,
};
const Q: Point = Some((976_979_610, 1_228_979_761));
const P: u64 = 2_147_483_647;

fn mm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, a);
        }
        a = mm(a, a);
        e >>= 1;
    }
    r
}
fn eval(f: &Poly, a: &[u64]) -> u64 {
    let mut s = 0u64;
    for (exp, c) in f {
        let mut t = *c;
        for (i, &e) in exp.iter().enumerate() {
            if e > 0 {
                t = mm(t, pow(a[i], e as u64));
            }
        }
        s = (s + t) % P;
    }
    s
}
fn next_word(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Frozen target panel. Planted targets use fixed leaves and signs; public targets
/// carry no witness. Returns (target, planted sector if any).
fn target(name: &str, fx: &Hybrid677, ar: &mut Arithmetic) -> Result<(Point, Option<usize>), String> {
    let pts = fx.points();
    let is_dickson = |x: u64| {
        let mut z = x;
        for _ in 0..10 {
            z = (mm(z, z) + P - 2) % P;
        }
        z == fx.terminal()
    };
    let dickson: Vec<usize> = (0..pts.len()).filter(|&i| is_dickson(pts[i].0)).collect();
    let y7: Vec<usize> = (0..pts.len()).filter(|&i| !is_dickson(pts[i].0)).collect();
    let planted = |k: usize, ar: &mut Arithmetic| -> Point {
        // k Dickson leaves then 4-k Y7 leaves, spread over the base, signs (+,-,+,+).
        let mut idx: Vec<usize> = (0..k).map(|t| dickson[37 + 101 * t]).collect();
        idx.extend((0..4 - k).map(|t| y7[11 + 41 * t]));
        let signs = [1i8, -1, 1, 1];
        let mut s = None;
        for (t, &i) in idx.iter().enumerate() {
            let pt = Some(pts[i]);
            s = ar.add(PRIME31, s, if signs[t] > 0 { pt } else { ar.neg(PRIME31, pt) });
        }
        s
    };
    Ok(match name {
        "h0" => (Some((18, 221_278_090)), None),
        "w0" => (Some((101_000_006, 1_077_058_598)), None),
        n if n.starts_with("planted") => {
            let k: usize = n[7..].parse().map_err(|_| "planted0..planted4")?;
            if k > 4 {
                return Err("planted0..planted4".into());
            }
            (planted(k, ar), Some(k))
        }
        n if n.starts_with("query") => {
            // Public query j from the B677 graph-control seed (crypto #5097/#5184).
            let j: usize = n[5..].parse().map_err(|_| "query1..")?;
            let mut seed = 0x7033_315f_6d34_0101u64;
            let (mut a, mut b) = (0, 0);
            for _ in 0..j {
                a = next_word(&mut seed) % PRIME31.n;
                b = next_word(&mut seed) % PRIME31.n;
            }
            let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
            let bq = ar.scalar(PRIME31, b, Q);
            (ar.add(PRIME31, ag, bq), None)
        }
        _ => return Err(format!("unknown target {name}")),
    })
}

/// Solve one sector system with the shared deadline; lift and replay relations.
fn solve_sector(
    fx: &Hybrid677,
    q: Point,
    k: usize,
    deadline: Instant,
    ar: &mut Arithmetic,
) -> Value {
    let started = Instant::now();
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return json!({"sector": k, "status": "Deadline", "reason": "shared deadline exhausted before construction"});
    }
    let secs = remaining.as_secs().max(1);
    let sys = match construct_hybrid677_sector(
        fx,
        q,
        k,
        ConstructionLimits { seconds: secs, max_polynomial_terms: 20_000 },
    ) {
        Ok(s) => s,
        Err(why) => return json!({"sector": k, "status": "BuildLimit", "reason": why}),
    };
    let construct_s = started.elapsed().as_secs_f64();
    let mut v = solve_system(fx, q, sys.variables, &sys.equations, &sys.quartic, deadline, ar);
    v["sector"] = json!(k);
    v["construct_s"] = json!(construct_s);
    v["input_terms"] = json!(sys.terms);
    v["input_max_degree"] = json!(sys.max_total_degree);
    v
}

/// Bounded native F4 visitor, quartic roots and signed replay for any presentation
/// whose first three coordinates are the RR5 parameters (b,c,d).
fn solve_system(
    fx: &Hybrid677,
    q: Point,
    variables: usize,
    equations: &[Poly],
    quartic: &[Poly],
    deadline: Instant,
    ar: &mut Arithmetic,
) -> Value {
    let started = Instant::now();
    struct S<'a> { variables: usize, equations: &'a [Poly], quartic: &'a [Poly] }
    let sys = S { variables, equations, quartic };
    let construct_s = 0.0;
    let k = -1i32;
    let mut opts = F4Options::new(Ordering::Grevlex, unsafe { MAX_DEGREE });
    opts.deadline = Some(deadline);
    opts.resource_limits = Some(unsafe { RESOURCES });
    let mut assignments: Vec<Vec<u64>> = Vec::new();
    let mut output_cap = false;
    let mut refused = 0usize;
    let traversal = solve_visit_traced(
        sys.equations,
        sys.variables,
        P,
        &opts,
        &vec![None; sys.variables],
        |a| {
            if assignments.len() >= 4096 {
                output_cap = true;
                return true;
            }
            assignments.push(a.to_vec());
            false
        },
        |event| {
            if let VisitEvent::Branch { source, .. } = event {
                if source != "univariate_roots" {
                    refused += 1;
                    return Err(format!("forbidden enumeration branch {source}"));
                }
            }
            Ok(())
        },
    );
    let solve_s = started.elapsed().as_secs_f64() - construct_s;
    let t = match traversal {
        Ok(t) => t,
        Err(why) => {
            return json!({"status": "Error", "reason": why, "variables": sys.variables,
                          "equations": sys.equations.len(), "solve_s": solve_s})
        }
    };
    let status = if output_cap {
        "OutputLimit"
    } else if t.resource_limit.is_some() {
        "ResourceLimit"
    } else if t.timed_out {
        "Deadline"
    } else if t.trace_stopped || !t.complete {
        "Inconclusive"
    } else {
        "Complete"
    };
    // Lift each assignment: roots of the literal quartic, signed replay to the target.
    let pts = fx.points();
    let index: BTreeMap<u64, usize> = pts.iter().enumerate().map(|(i, p)| (p.0, i)).collect();
    let (tx, ty) = q.unwrap();
    let mut relations = BTreeSet::new();
    let mut rejected = 0usize;
    let mut root_mults = 0u64;
    for a in &assignments {
        let coeff: Vec<u64> = sys.quartic.iter().map(|f| eval(f, a)).collect();
        let Some(roots) = poly_roots_fp_counted_until(&coeff, P, 0x5252355f524f4f54, &mut root_mults, Some(deadline)) else {
            rejected += 1;
            continue;
        };
        if roots.len() != 4 {
            rejected += 1;
            continue;
        }
        let mut terms = Vec::new();
        let mut sum = None;
        let mut ok = true;
        for x in roots {
            let Some(&i) = index.get(&x) else { ok = false; break };
            let den = (x + a[0]) % P;
            if den == 0 {
                ok = false;
                break;
            }
            let a0 = (mm(a[1], x) + a[2]) % P;
            let frac = mm(mm((x + P - tx) % P, a0), pow(den, P - 2));
            let y = (2 * P - ty - frac) % P;
            let sign = if y == pts[i].1 { 1i64 } else if y == (P - pts[i].1) % P { -1 } else { ok = false; break };
            sum = ar.add(PRIME31, sum, Some((x, y)));
            terms.push((i, sign));
        }
        terms.sort_unstable();
        if ok && sum == q && terms.len() == 4 {
            relations.insert(terms);
        } else {
            rejected += 1;
        }
    }
    let _ = (k, construct_s);
    json!({
        "status": status, "variables": sys.variables, "equations": sys.equations.len(),
        "solve_s": solve_s,
        "solving_degree": t.solving_degree, "resource_limit": t.resource_limit.as_ref().map(|r| r.to_string()),
        "timed_out": t.timed_out, "solutions_visited": t.solutions_visited,
        "assignments": assignments.len(), "rejected_assignments": rejected, "forbidden_refused": refused,
        "relations": relations.iter().map(|r| json!(r)).collect::<Vec<_>>(),
    })
}

/// RR5 parameters through four signed points (as in PR #5136's `interpolate`).
fn interpolate(points: &[(u64, u64); 4], (r, s): (u64, u64)) -> Option<[u64; 3]> {
    let mut rows: Vec<[u64; 4]> = points.iter().map(|&(x, y)| {
        let sy = (s + y) % P;
        let xr = (x + P - r) % P;
        [sy, mm(xr, x), xr, (P - mm(sy, x)) % P]
    }).collect();
    let mut next = 0;
    for col in 0..3 {
        let piv = (next..rows.len()).find(|&i| rows[i][col] != 0)?;
        rows.swap(next, piv);
        let inv = pow(rows[next][col], P - 2);
        for k in col..4 { rows[next][k] = mm(rows[next][k], inv); }
        let pr = rows[next];
        for (i, row) in rows.iter_mut().enumerate() {
            if i == next { continue; }
            let sc = row[col];
            for k in col..4 { row[k] = (row[k] + P - mm(sc, pr[k])) % P; }
        }
        next += 1;
    }
    Some([rows[0][3], rows[1][3], rows[2][3]])
}

/// Forward-evaluate the lifted circuit at the planted witness's parameters and check that
/// every equation vanishes (defining equations first, then terminal ones).
fn planted_control(g: &graeffe_jets::GraeffeSystem, tname: &str, fx: &Hybrid677, q: Point, ar: &mut Arithmetic) -> Value {
    let Some(k) = tname.strip_prefix("planted").and_then(|v| v.parse::<usize>().ok()) else { return json!(null) };
    if k != 4 { return json!({"applicable": false, "reason": "Graeffe/Dickson presentation covers the pure Dickson sector only"}); }
    let pts = fx.points();
    let is_d = |x: u64| { let mut z = x; for _ in 0..10 { z = (mm(z, z) + P - 2) % P; } z == fx.terminal() };
    let dickson: Vec<usize> = (0..pts.len()).filter(|&i| is_d(pts[i].0)).collect();
    let signs = [1i8, -1, 1, 1];
    let leaves: [(u64, u64); 4] = std::array::from_fn(|t| {
        let (x, y) = pts[dickson[37 + 101 * t]];
        (x, if signs[t] > 0 { y } else { (P - y) % P })
    });
    let _ = ar;
    let Some(theta) = interpolate(&leaves, q.unwrap()) else { return json!({"applicable": true, "pass": false, "reason": "interpolation"}) };
    let n = g.variables;
    let mut val = vec![0u64; n];
    val[..3].copy_from_slice(&theta);
    let ev = |m: &graeffe_jets::MP, val: &[u64]| -> u64 {
        let mut acc = 0u64;
        for (e, c) in m { let mut t = *c; for (i, &d) in e.iter().enumerate() { if d > 0 { t = mm(t, pow(val[i], d as u64)); } } acc = (acc + t) % P; }
        acc
    };
    let defining = n - 3;
    for (i, eq) in g.equations[..defining].iter().enumerate() {
        // eq = x_v - a(prev): with x_v = 0 it evaluates to -a(prev).
        let v = 3 + i;
        val[v] = 0;
        val[v] = (P - ev(eq, &val)) % P;
    }
    let residuals: Vec<u64> = g.equations.iter().map(|e| ev(e, &val)).collect();
    let nonzero = residuals.iter().filter(|&&r| r != 0).count();
    // Jacobian of the whole lifted system at the witness; rank n means the witness is a
    // regular (multiplicity-one) point of this presentation.
    let mut rows: Vec<Vec<u64>> = g.equations.iter().map(|eq| {
        let mut row = vec![0u64; n];
        for (e, c) in eq {
            for i in 0..n {
                if e[i] == 0 { continue; }
                let mut t = mm(*c, e[i] as u64);
                for (j, &d) in e.iter().enumerate() {
                    let d = if j == i { d - 1 } else { d };
                    if d > 0 { t = mm(t, pow(val[j], d as u64)); }
                }
                row[i] = (row[i] + t) % P;
            }
        }
        row
    }).collect();
    let mut rank = 0;
    for col in 0..n {
        let Some(piv) = (rank..rows.len()).find(|&r| rows[r][col] != 0) else { continue };
        rows.swap(rank, piv);
        let inv = pow(rows[rank][col], P - 2);
        let pr: Vec<u64> = rows[rank].iter().map(|&x| mm(x, inv)).collect();
        for r in 0..rows.len() {
            if r != rank && rows[r][col] != 0 {
                let f = rows[r][col];
                for c in col..n { rows[r][c] = (rows[r][c] + P - mm(f, pr[c])) % P; }
            }
        }
        rows[rank] = pr;
        rank += 1;
    }
    json!({"applicable": true, "theta": theta, "pass": nonzero == 0, "nonzero_residuals": nonzero,
           "equations": residuals.len(), "variables": n, "jacobian_rank": rank, "regular": rank == n})
}

fn main() {
    let mut arm = String::new();
    let mut tname = String::new();
    let mut args = std::env::args().skip(1);
    while let Some(f) = args.next() {
        let v = args.next().expect("flag value");
        match f.as_str() {
            "--arm" => arm = v,
            "--target" => tname = v,
            "--scale" => unsafe {
                let k: usize = v.parse().expect("scale");
                RESOURCES = F4ResourceLimits {
                    max_pairs: 20_000 * k,
                    max_symbolic_monomials: 8_192 * k,
                    max_matrix_entries: 2_000_000 * k,
                };
            },
            "--seconds" => unsafe { SECONDS = v.parse().expect("seconds") },
            "--max-degree" => unsafe { MAX_DEGREE = v.parse().expect("degree") },
            _ => panic!("unknown flag {f}"),
        }
    }
    let cold = Instant::now();
    let fx = Hybrid677::build().expect("frozen fixture");
    let fixture_s = cold.elapsed().as_secs_f64();
    let mut ar = Arithmetic::default();
    let (q, planted) = target(&tname, &fx, &mut ar).expect("target");
    assert!(ar.on_curve(PRIME31, q));
    let (seconds, max_degree, resources) = unsafe { (SECONDS, MAX_DEGREE, RESOURCES) };
    let t0 = Instant::now();
    let deadline = t0 + Duration::from_secs(seconds);
    let body = match arm.as_str() {
        "coupled" => {
            let row = extract_hybrid677(
                &fx,
                q,
                Limits { max_degree, seconds, max_polynomial_terms: 20_000, max_assignments: 4096 },
                resources,
            );
            serde_json::to_value(&row).unwrap()
        }
        "sectors" => {
            let rows: Vec<Value> = (0..5).map(|k| solve_sector(&fx, q, k, deadline, &mut ar)).collect();
            json!({"sectors": rows})
        }
        "graeffe" | "deflated" => {
            let order = if arm == "graeffe" { 0 } else { 2 };
            let tb = Instant::now();
            let g = graeffe_jets::build(order, PRIME31.a, q.unwrap(), fx.terminal());
            let to_poly = |m: &graeffe_jets::MP| -> Poly {
                normalise(&m.iter().map(|(e, c)| (e.clone(), *c)).collect::<Vec<_>>(), P, Ordering::Grevlex)
            };
            let eqs: Vec<Poly> = g.equations.iter().map(to_poly).collect();
            let quartic: Vec<Poly> = g.quartic.iter().map(to_poly).collect();
            let mut monic = quartic.clone();
            monic.push(vec![(vec![0; g.variables], 1)]);
            let construct_s = tb.elapsed().as_secs_f64();
            let control = planted_control(&g, &tname, &fx, q, &mut ar);
            let mut v = solve_system(&fx, q, g.variables, &eqs, &monic, deadline, &mut ar);
            v["construct_s"] = json!(construct_s);
            v["input_terms"] = json!(eqs.iter().map(Vec::len).sum::<usize>());
            v["input_max_degree"] = json!(eqs.iter().flat_map(|f| f.iter().map(|(e, _)| e.iter().sum::<u32>())).max());
            v["planted_witness_control"] = control;
            v
        }
        s if s.starts_with("sector") => {
            let k: usize = s[6..].parse().expect("sector0..sector4");
            solve_sector(&fx, q, k, deadline, &mut ar)
        }
        _ => panic!("unknown arm"),
    };
    println!(
        "{}",
        serde_json::to_string(&json!({
            "arm": arm, "target": tname, "target_point": q, "planted_sector": planted,
            "limits": {"seconds": seconds, "max_degree": max_degree, "max_pairs": resources.max_pairs,
                       "max_symbolic_monomials": resources.max_symbolic_monomials,
                       "max_matrix_entries": resources.max_matrix_entries},
            "fixture_s": fixture_s, "arm_wall_s": t0.elapsed().as_secs_f64(), "cold_wall_s": cold.elapsed().as_secs_f64(),
            "result": body,
        }))
        .unwrap()
    );
}
