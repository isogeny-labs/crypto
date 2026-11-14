//! `native-solve`: the independent native arm. Runs `f4_fp` (grevlex) on the same
//! polynomial set as the `.ms` file, records its report and counters, and, for a
//! small zero-dimensional basis, extracts the solutions.
use crate::system::polys_from_json;
use crate::util::*;
use crypto_lib::cryptanalysis::f4_fp::{
    self, f4, field_ops_total, native_ops_total, normalise, take_run_log, F4Options, Ordering, Poly, Verdict,
};
use serde_json::{json, Value};
use std::path::Path;
use std::time::{Duration, Instant};

/// Standard monomials of a zero-dimensional leading ideal (`None` if not
/// zero-dimensional, `Err` if more than `cap`).
fn staircase(basis: &[Poly], nvars: usize, cap: usize) -> Option<Result<usize, ()>> {
    let lms: Vec<&Vec<u32>> = basis.iter().map(|f| &f[0].0).collect();
    let mut bound = vec![0u32; nvars];
    for v in 0..nvars {
        let d = lms
            .iter()
            .filter(|e| e[v] > 0 && e.iter().enumerate().all(|(k, &x)| k == v || x == 0))
            .map(|e| e[v])
            .min()?;
        bound[v] = d;
    }
    fn rec(k: usize, cur: &mut Vec<u32>, bound: &[u32], lms: &[&Vec<u32>], count: &mut usize, cap: usize) -> bool {
        if *count > cap {
            return false;
        }
        // prune: some leading monomial supported on the assigned prefix divides cur
        let divisible = lms
            .iter()
            .any(|e| e.iter().zip(cur.iter()).all(|(a, b)| a <= b) && e.iter().skip(k).all(|&x| x == 0));
        if divisible {
            return true;
        }
        if k == bound.len() {
            *count += 1;
            return true;
        }
        for x in 0..bound[k] {
            cur[k] = x;
            if !rec(k + 1, cur, bound, lms, count, cap) {
                return false;
            }
        }
        cur[k] = 0;
        true
    }
    let mut count = 0usize;
    let mut cur = vec![0u32; nvars];
    let ok = rec(0, &mut cur, &bound, &lms, &mut count, cap);
    Some(if ok { Ok(count) } else { Err(()) })
}

pub fn run(args: &[String]) -> Result<(), String> {
    let sys_path = crate::arg_req(args, "--system")?;
    let max_degree: u32 = crate::arg_req(args, "--max-degree")?.parse().map_err(|_| "max-degree")?;
    let deadline: f64 = crate::arg_req(args, "--deadline")?.parse().map_err(|_| "deadline")?;
    let out = crate::arg_req(args, "--out")?;
    let extract_q: usize = crate::arg_opt(args, "--extract-max-q").map_or(Ok(256), |s| s.parse()).map_err(|_| "extract-max-q")?;
    let sj = read_json(Path::new(&sys_path))?;
    let (p, m, t, polys) = polys_from_json(&sj)?;
    let nvars = crate::system::nvars(m, t);
    let input: Vec<Poly> = polys
        .iter()
        .map(|terms| normalise(terms, p, Ordering::Grevlex))
        .collect();
    let opts = F4Options::new(Ordering::Grevlex, max_degree)
        .with_budget(Duration::from_secs_f64(deadline))
        .capturing_run_log();
    let _ = take_run_log();
    let (ops0, nat0, cpu0, t0) = (field_ops_total(), native_ops_total(), cpu_secs(), Instant::now());
    let rep = f4(&input, nvars, p, &opts);
    let wall = t0.elapsed().as_secs_f64();
    let cpu = cpu_secs() - cpu0;
    let (ops, nat) = (field_ops_total() - ops0, native_ops_total() - nat0);
    let log = take_run_log();
    let steps: Vec<Value> = log
        .steps
        .iter()
        .map(|s| {
            json!({"degree": s.degree, "pairs": s.pairs, "rows": s.rows, "all_rows": s.all_rows,
                "cols_unavailable": true, "ops": s.ops, "new_polys": s.new_polys, "staircase": s.staircase})
        })
        .collect();
    let complete = !rep.timed_out && !rep.inconsistent && rep.pairs_above_bound == 0;
    let zero_dim = if complete && !rep.basis.is_empty() {
        staircase(&rep.basis, nvars, 10_000_000)
    } else {
        None
    };
    let mut rec = json!({
        "schema": "pkm.m8-panel.native-solve/v1",
        "system": sys_path, "system_file_sha256": file_sha256(Path::new(&sys_path))?,
        "p": p, "m": m, "t": t, "nvars": nvars, "npolys": input.len(),
        "order": "grevlex", "max_degree": max_degree, "deadline_secs": deadline,
        "f4": {
            "inconsistent": rep.inconsistent, "timed_out": rep.timed_out,
            "solving_degree": rep.solving_degree, "solving_degree_max": rep.solving_degree_max,
            "degree_reached": rep.degree_reached, "steps": rep.steps, "steps_to_solution": rep.steps_to_solution,
            "max_rows": rep.max_rows, "max_cols": rep.max_cols, "max_cols_to_solution": rep.max_cols_to_solution,
            "pairs_above_bound": rep.pairs_above_bound, "basis_len": rep.basis.len(),
            "report_field_ops": rep.field_ops, "report_ms": rep.ms,
        },
        "counters": {"field_ops_total_delta": ops, "native_ops_total_delta": nat},
        "run_log_steps": steps,
        "censored": !complete && !rep.inconsistent,
        "wall_secs": wall, "cpu_secs": cpu, "vm_hwm_kb": vm_hwm_kb(),
        "tool": crate::tool_info(),
    });
    match zero_dim {
        Some(Ok(q)) => {
            rec["zero_dimensional"] = json!(true);
            rec["quotient_dimension"] = json!(q);
        }
        Some(Err(())) => rec["zero_dimensional"] = json!("staircase larger than 10^7"),
        None => rec["zero_dimensional"] = json!(false),
    }
    let mut basis_only = true;
    if let Some(Ok(q)) = zero_dim {
        if q <= extract_q {
            // second, charged F4 run inside the substitution tree
            let o2 = F4Options::new(Ordering::Grevlex, max_degree).with_budget(Duration::from_secs_f64(deadline));
            let (o_ops, c1, t1) = (field_ops_total(), cpu_secs(), Instant::now());
            let sr = f4_fp::solve(&input, nvars, p, &o2);
            rec["extraction"] = json!({
                "method": "f4_fp::solve (F4 + substitution tree; a second F4 run, charged here)",
                "field_ops": field_ops_total() - o_ops, "wall_secs": t1.elapsed().as_secs_f64(),
                "cpu_secs": cpu_secs() - c1, "f4_runs": sr.f4_runs, "timed_out": sr.timed_out,
                "candidates_rejected": sr.candidates_rejected,
            });
            if let Verdict::Solutions(sols) = sr.verdict {
                basis_only = false;
                rec["extraction"]["solutions_in_fp"] = json!(sols.len());
                if let (Some(c), Some(tg), Some(tt)) = (
                    crate::arg_opt(args, "--cell"),
                    crate::arg_opt(args, "--target"),
                    crate::arg_opt(args, "--targets"),
                ) {
                    let mut ctx = crate::verify::Ctx::load(&c, &tt, &tg)?;
                    let y0s: Vec<Vec<u64>> = sols.iter().map(|s| (0..m).map(|i| s[i * t]).collect()).collect();
                    let found = crate::verify::analyze(&mut ctx, &y0s);
                    rec["extraction"]["relations"] = crate::verify::found_json(&mut ctx, &found);
                    rec["extraction"]["relation_work"] = ctx.work.json();
                }
            } else {
                rec["extraction"]["verdict"] = json!(format!("{:?}", sr.verdict));
            }
        }
    }
    if let Some(gp) = crate::arg_opt(args, "--gb-out") {
        if complete || rep.inconsistent {
            let names = crate::system::var_names(m, t);
            std::fs::write(&gp, gb_text(&rep.basis, &names, p)).map_err(|e| e.to_string())?;
            rec["gb_out"] = json!({"path": gp, "sha256": file_sha256(Path::new(&gp))?});
        } else {
            rec["gb_out"] = json!("not written: run censored or inconsistent");
        }
    }
    rec["basis_only"] = json!(basis_only);
    write_json(Path::new(&out), &seal(rec))?;
    println!(
        "native-solve: inconsistent={} timed_out={} solving_degree={} steps={} field_ops={}",
        rep.inconsistent, rep.timed_out, rep.solving_degree, rep.steps, ops
    );
    Ok(())
}

// ---- canonical reduced Groebner basis text and comparison -------------------

/// msolve-style text of a reduced basis (grevlex, variables in the `.ms` order).
pub fn gb_text(basis: &[Poly], names: &[String], p: u64) -> String {
    use crypto_lib::cryptanalysis::groebner_f4::cmp_monomial;
    let mut b: Vec<Poly> = f4_fp::interreduce(basis, p, Ordering::Grevlex);
    b.sort_by(|x, y| cmp_monomial(&x[0].0, &y[0].0, Ordering::Grevlex));
    let polys: Vec<String> = b
        .iter()
        .map(|f| {
            f.iter()
                .map(|(e, c)| {
                    let mut s = c.to_string();
                    for (k, &x) in e.iter().enumerate() {
                        if x > 0 {
                            s.push_str(&format!("*{}^{x}", names[k]));
                        }
                    }
                    s
                })
                .collect::<Vec<_>>()
                .join("+")
        })
        .collect();
    format!(
        "#Reduced Groebner basis data\n#---\n#field characteristic: {p}\n#variable order:       {}\n#monomial order:       graded reverse lexicographical\n#length of basis:      {} elements sorted by increasing leading monomials\n#---\n[{}]:\n",
        names.join(", "),
        polys.len(),
        polys.join(",\n")
    )
}

type NormPoly = Vec<(Vec<u32>, u64)>;

fn parse_gb(text: &str, order: Option<&[String]>) -> Result<(Vec<String>, u64, std::collections::BTreeSet<NormPoly>), String> {
    let mut names: Vec<String> = Vec::new();
    let mut p = 0u64;
    let mut body = String::new();
    for l in text.lines() {
        if let Some(r) = l.strip_prefix("#variable order:") {
            names = r.split(',').map(|s| s.trim().to_string()).collect();
        } else if let Some(r) = l.strip_prefix("#field characteristic:") {
            p = r.trim().parse().map_err(|_| "characteristic")?;
        } else if !l.starts_with('#') {
            body.push_str(l.trim());
        }
    }
    if names.is_empty() || p == 0 {
        return Err("missing header (variable order / characteristic)".into());
    }
    let body = body.trim().trim_end_matches(':').trim();
    let body = body.strip_prefix('[').and_then(|b| b.strip_suffix(']')).ok_or("no [ ... ] body")?;
    let target: Vec<String> = order.map_or_else(|| names.clone(), <[String]>::to_vec);
    let mut set = std::collections::BTreeSet::new();
    for ptxt in body.split(',') {
        let mut terms: std::collections::BTreeMap<Vec<u32>, u64> = std::collections::BTreeMap::new();
        for term in ptxt.trim().split('+') {
            let mut parts = term.trim().split('*');
            let c: u64 = parts.next().ok_or("coef")?.parse().map_err(|_| format!("bad coefficient in {term:?}"))?;
            let mut e = vec![0u32; target.len()];
            for v in parts {
                let (nm, ex) = v.split_once('^').map_or((v, "1"), |(a, b)| (a, b));
                let k = target.iter().position(|n| n == nm).ok_or_else(|| format!("unknown variable {nm}"))?;
                e[k] += ex.parse::<u32>().map_err(|_| "exponent")?;
            }
            *terms.entry(e).or_insert(0) += c;
        }
        let mut t: Vec<(Vec<u32>, u64)> = terms.into_iter().filter(|x| x.1 % p != 0).map(|(e, c)| (e, c % p)).collect();
        // monic: divide by the grevlex-leading coefficient
        use crypto_lib::cryptanalysis::groebner_f4::cmp_monomial;
        t.sort_by(|x, y| cmp_monomial(&y.0, &x.0, Ordering::Grevlex));
        if let Some(lead) = t.first().map(|x| x.1) {
            let inv = invm(lead, p);
            for x in t.iter_mut() {
                x.1 = mulm(x.1, inv, p);
            }
        }
        set.insert(t);
    }
    Ok((names, p, set))
}

/// `compare-gb --a FILE --b FILE --out FILE`: normalise two reduced-basis files (msolve `-g 2`
/// output or `native-solve --gb-out` text) over file A's variable order and compare as sets.
pub fn compare_gb(args: &[String]) -> Result<(), String> {
    let (pa, pb) = (crate::arg_req(args, "--a")?, crate::arg_req(args, "--b")?);
    let out = crate::arg_req(args, "--out")?;
    let ta = std::fs::read_to_string(&pa).map_err(|e| format!("{pa}: {e}"))?;
    let tb = std::fs::read_to_string(&pb).map_err(|e| format!("{pb}: {e}"))?;
    let (na, p1, a) = parse_gb(&ta, None)?;
    let (nb, p2, b) = parse_gb(&tb, Some(&na))?;
    let equal = p1 == p2 && a == b;
    let rec = json!({
        "schema": "pkm.m8-panel.compare-gb/v1",
        "a": pa, "b": pb, "a_sha256": sha256_hex(ta.as_bytes()), "b_sha256": sha256_hex(tb.as_bytes()),
        "characteristic": [p1, p2],
        "variable_order_a": na, "variable_order_b": nb, "same_variable_order": na == nb,
        "a_elements": a.len(), "b_elements": b.len(),
        "only_in_a": a.difference(&b).count(), "only_in_b": b.difference(&a).count(),
        "equal": equal,
        "tool": crate::tool_info(),
    });
    write_json(Path::new(&out), &seal(rec))?;
    println!("compare-gb: equal={equal} ({} vs {} elements)", a.len(), b.len());
    Ok(())
}
