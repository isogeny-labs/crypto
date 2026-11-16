//! Hybrid guess-and-determine study: planted solutions, scoring, row model.
//!
//! `research/dreg_hybrid_guess_20261024/` (PREREGISTRATION.md).  The closure
//! itself is `examples/dreg_closure --fix ...` unchanged; this file adds
//! three subcommands around it and shares nothing with it but the draws:
//!
//! ```sh
//! dreg_hybrid plant DRAWS.jsonl --draw K        # solutions of a satisfiable draw, verified
//! dreg_hybrid score RUNS_DIR [--out summary.json]   # branches -> k*, costs, controls
//! dreg_hybrid model [--kappa K]                 # the m = 4 row under the guess scenarios
//! ```
//!
//! `plant` enumerates every solution `(x1, x2, x3, u)` of the m = 3 chained
//! system exactly (as `chained_s3_solution_count` counts them), packs each
//! as one assignment of the `3 l + n` Boolean unknowns, and checks that
//! assignment against every equation of the built system.  `score` reads the
//! branch records and applies the preregistered rules.  `model` is the
//! `descent_degree_cost` model with the oracle `2^k * C(N-k, <=4)^w`.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use rayon::prelude::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn eval(p: &F2BoolPoly, assign: u64) -> bool {
    p.terms.iter().filter(|t| t.mask & !assign == 0).count() % 2 == 1
}

// ---------------------------------------------------------------- plant

fn plant(args: &[String]) {
    let file = &args[2];
    let draw: u64 = flag(args, "--draw").expect("--draw K").parse().unwrap();
    let text = std::fs::read_to_string(file).unwrap();
    let v: Value = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|v| v["draw"].as_u64() == Some(draw) && v.get("control").is_none())
        .expect("draw");
    assert_eq!(
        v["outcome"]["kind"], "satisfiable",
        "plant needs a satisfiable draw"
    );
    let n = v["n"].as_u64().unwrap() as u32;
    let ell = v["ell"].as_u64().unwrap() as usize;
    assert!(
        v.get("m").and_then(Value::as_u64).unwrap_or(3) == 3,
        "m = 3 only"
    );
    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let b = F2mElement::one(n);
    let words: Vec<u64> = v["v_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.as_u64().unwrap())
        .collect();
    let basis: Vec<F2mElement> = words
        .iter()
        .map(|&w| F2mElement::from_biguint(&BigUint::from(w), n))
        .collect();
    let xr = v["x_r"].as_u64().unwrap();
    let x_r = F2mElement::from_biguint(&BigUint::from(xr), n);
    let sys = build_decomposition_system(&basis, &x_r, &b, 3, &st).expect("system");
    let gf = Gf2::new(&irr);
    let span: Vec<u64> = (0..1u64 << ell)
        .map(|c| {
            (0..ell)
                .filter(|t| (c >> t) & 1 == 1)
                .fold(0, |a, t| a ^ words[t])
        })
        .collect();
    let squares: Vec<u64> = (0..1u64 << n).map(|u| gf.sqr(u)).collect();
    let s3 = |a: u64, c: u64, d: u64| {
        let ac = gf.mul(a, c);
        gf.mul(gf.sqr(a ^ c), gf.sqr(d)) ^ gf.mul(ac, d) ^ gf.sqr(ac) ^ 1
    };
    // Every solution as a packed assignment: bit i*l + t = coordinate t of
    // summand i; bit 3l + j = bit j of u.
    let found: Vec<u64> = (0..span.len())
        .into_par_iter()
        .flat_map_iter(|c1| {
            let mut out = Vec::new();
            let x1 = span[c1];
            for (c2, &x2) in span.iter().enumerate() {
                let (lead, mid) = (gf.sqr(x1 ^ x2), gf.mul(x1, x2));
                let tail = gf.sqr(mid) ^ 1;
                for (u, &uu) in squares.iter().enumerate() {
                    let u = u as u64;
                    if gf.mul(lead, uu) ^ gf.mul(mid, u) ^ tail != 0 {
                        continue;
                    }
                    for (c3, &x3) in span.iter().enumerate() {
                        if s3(u, x3, xr) == 0 {
                            let a = c1 as u64
                                | (c2 as u64) << ell
                                | (c3 as u64) << (2 * ell)
                                | u << (3 * ell);
                            out.push(a);
                        }
                    }
                }
            }
            out
        })
        .collect();
    let mut found = found;
    found.sort_unstable();
    for &a in &found {
        let bad = sys.equations.iter().filter(|p| eval(p, a)).count();
        assert_eq!(
            bad, 0,
            "assignment {a:#x} violates {bad} equations of the built system"
        );
    }
    println!(
        "{}",
        json!({"source": file, "draw": draw, "n": n, "ell": ell, "n_vars": sys.n_vars,
               "reported_solutions": v["solutions"], "solutions": found.len(),
               "verified_against_all_equations": true, "assignments": found})
    );
}

// ---------------------------------------------------------------- score

#[derive(Clone, Debug)]
struct Branch {
    refuted: bool,
    sat_record: bool,
    cols: f64,
    rank: f64,
    secs: f64,
    ops: f64,
    products: f64,
    rss_kb: f64,
    measured_to: u64,
    n_vars: u64,
}

fn log2_monomials(n_vars: f64, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d {
        let i = i as f64;
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) / (i + 1.0);
    }
    total.log2()
}

/// Rule variables (m = 3): R1 the top coordinate of each summand, R2 the lowest
/// intermediate coordinates.
fn rule_vars(rule: &str, ell: u64) -> Vec<u64> {
    match rule {
        "R1" => vec![ell - 1, 2 * ell - 1, 3 * ell - 1],
        "R2" => vec![3 * ell, 3 * ell + 1, 3 * ell + 2],
        _ => vec![],
    }
}

fn cell_ell(cell: &str) -> u64 {
    cell.trim_end_matches('f')
        .rsplit('-')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

/// Last-degree reading of a committed record: (columns, rank, products, ops, secs, n_vars, refuted).
fn committed_reading(path: &str, draw: u64) -> Option<(f64, f64, f64, f64, f64, f64, bool)> {
    let t = std::fs::read_to_string(path).ok()?;
    for l in t.lines().filter(|l| l.trim_start().starts_with('{')) {
        let v: Value = serde_json::from_str(l).ok()?;
        if v["draw"].as_u64() != Some(draw) {
            continue;
        }
        let sc = v["scan"].as_array()?;
        let d4 = sc.iter().find(|d| d["degree"] == 4)?;
        return Some((
            d4["columns"].as_f64()?,
            d4["closure_rank"].as_f64()?,
            d4["closure_products"].as_f64()?,
            d4["closure_word_ops"].as_f64()?,
            d4["secs"].as_f64()?,
            v["n_vars"].as_f64()?,
            !v["closure_degree"].is_null(),
        ));
    }
    None
}

fn parse_branch(path: &std::path::Path) -> Option<Branch> {
    let text = std::fs::read_to_string(path).ok()?;
    let line = text.lines().find(|l| l.trim_start().starts_with('{'))?;
    let v: Value = serde_json::from_str(line).ok()?;
    let sat_record = v.get("refuted_satisfiable").is_some();
    let scan = v["scan"].as_array()?;
    let last = scan.last()?;
    let refuted = if sat_record {
        v["refuted_satisfiable"].as_bool()?
    } else {
        !v["closure_degree"].is_null()
    };
    let sum = |k: &str| {
        scan.iter()
            .map(|d| d[k].as_f64().unwrap_or(0.0))
            .sum::<f64>()
    };
    let log = path.with_extension("log");
    let rss_kb = std::fs::read_to_string(&log)
        .ok()
        .and_then(|t| {
            t.lines().find(|l| l.contains("VmHWM")).and_then(|l| {
                l.split_whitespace()
                    .nth(2)
                    .and_then(|x| x.parse::<f64>().ok())
            })
        })
        .unwrap_or(f64::NAN);
    Some(Branch {
        refuted,
        sat_record,
        cols: last["columns"].as_f64()?,
        rank: last["closure_rank"].as_f64()?,
        secs: sum("secs"),
        ops: sum("closure_word_ops"),
        products: sum("closure_products"),
        rss_kb,
        measured_to: last["degree"].as_u64()?,
        n_vars: v["n_vars"].as_u64()?,
    })
}

fn score(args: &[String]) {
    let dir = std::path::PathBuf::from(&args[2]);
    let out_path = flag(args, "--out");
    // file name: <cell>.d<K>.<rule>.k<k>.b<b>.jsonl   (cell like m3-19-7 or m3-19-7f)
    let mut groups: BTreeMap<(String, u64, String, u32), BTreeMap<u32, Branch>> = BTreeMap::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    for p in files {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let Some(stem) = name.strip_suffix(".jsonl") else {
            continue;
        };
        let parts: Vec<&str> = stem.split('.').collect();
        if parts.len() != 5
            || !parts[1].starts_with('d')
            || !parts[3].starts_with('k')
            || !parts[4].starts_with('b')
        {
            continue;
        }
        let Some(br) = parse_branch(&p) else { continue };
        let key = (
            parts[0].to_string(),
            parts[1][1..].parse().unwrap(),
            parts[2].to_string(),
            parts[3][1..].parse().unwrap(),
        );
        groups
            .entry(key)
            .or_default()
            .insert(parts[4][1..].parse().unwrap(), br);
    }
    // planted solutions: plants/<cell>.d<K>.json
    let plants_dir = dir.parent().unwrap().join("plants");
    let plant_of = |cell: &str, k: u64| -> Option<Vec<u64>> {
        let t = std::fs::read_to_string(plants_dir.join(format!("{cell}.d{k}.json"))).ok()?;
        let v: Value = serde_json::from_str(&t).ok()?;
        Some(
            v["assignments"]
                .as_array()?
                .iter()
                .map(|a| a.as_u64().unwrap())
                .collect(),
        )
    };
    let mut rows = Vec::new();
    println!("| cell | draw | kind | rule | k | branches run / 2^k | refuted | status | sum word ops | sum secs | max cols | max rss GB |");
    println!("|---|--:|---|---|--:|--:|--:|---|--:|--:|--:|--:|");
    let mut control_fail = Vec::new();
    for ((cell, draw, rule, k), br) in &groups {
        let total = 1u32 << k;
        let sat = br.values().any(|b| b.sat_record);
        let refuted = br.values().filter(|b| b.refuted).count();
        let complete = br.len() as u32 == total;
        let status = if sat {
            "control"
        } else if refuted as u32 == total {
            "all-refute"
        } else if br.len() as u32 > refuted as u32 {
            "fails"
        } else {
            "incomplete"
        };
        // Soundness: a branch that holds a planted solution must not refute.
        if sat {
            let sols = plant_of(cell, *draw);
            for (b, r) in br {
                let vars = rule_vars(rule, cell_ell(cell));
                let contains = sols.as_ref().map(|s| {
                    s.iter().any(|a| {
                        (0..*k as usize).all(|j| (a >> vars[j]) & 1 == ((*b as u64 >> j) & 1))
                    })
                });
                if contains == Some(true) && r.refuted {
                    control_fail.push(format!("{cell} d{draw} {rule} k{k} b{b}: refuted a branch holding a planted solution"));
                }
                if contains.is_none() {
                    control_fail.push(format!("{cell} d{draw}: no plant file"));
                }
            }
            if complete && refuted as u32 == total {
                control_fail.push(format!(
                    "{cell} d{draw} {rule} k{k}: every branch of a satisfiable draw refuted"
                ));
            }
        }
        let ops: f64 = br.values().map(|b| b.ops).sum();
        let secs: f64 = br.values().map(|b| b.secs).sum();
        let cols = br.values().map(|b| b.cols).fold(0.0, f64::max);
        let rss = br.values().map(|b| b.rss_kb).fold(0.0, f64::max) / 1e6;
        println!(
            "| {cell} | {draw} | {} | {rule} | {k} | {}/{total} | {refuted} | {status} | {ops:.4e} | {secs:.0} | {cols:.0} | {rss:.2} |",
            if sat { "sat" } else { "unsat" },
            br.len()
        );
        rows.push(json!({"cell": cell, "draw": draw, "kind": if sat {"sat"} else {"unsat"}, "rule": rule, "k": k,
            "branches_run": br.len(), "branches": total, "refuted": refuted, "status": status,
            "sum_word_ops": ops, "sum_products": br.values().map(|b| b.products).sum::<f64>(),
            "sum_secs": secs, "max_cols": cols, "max_rss_gb": rss,
            "sum_cols_x_products_words": br.values().map(|b| b.products * (b.cols / 64.0).ceil()).sum::<f64>(),
            "n_vars": br.values().next().map(|b| b.n_vars),
            "per_branch": br.iter().map(|(b, r)| json!({"b": b, "refuted": r.refuted, "cols": r.cols, "rank": r.rank,
                "secs": r.secs, "word_ops": r.ops, "products": r.products, "rss_kb": r.rss_kb, "to": r.measured_to})).collect::<Vec<_>>()}));
    }
    // k* per unsat (cell, draw, rule)
    println!("\n| cell | draw | rule | k* (smallest k with all 2^k branches refuting) | cost at k* (word ops) | k=1 | k=2 | k=3 |");
    println!("|---|--:|---|--:|--:|---|---|---|");
    let mut kstar = Vec::new();
    let mut seen: BTreeMap<(String, u64, String), BTreeMap<u32, String>> = BTreeMap::new();
    for r in &rows {
        if r["kind"] != "unsat" {
            continue;
        }
        seen.entry((
            r["cell"].as_str().unwrap().into(),
            r["draw"].as_u64().unwrap(),
            r["rule"].as_str().unwrap().into(),
        ))
        .or_default()
        .insert(
            r["k"].as_u64().unwrap() as u32,
            format!(
                "{}/{} {}",
                r["refuted"],
                r["branches"],
                r["status"].as_str().unwrap()
            ),
        );
    }
    for ((cell, draw, rule), ks) in &seen {
        let best = rows
            .iter()
            .filter(|r| {
                r["cell"] == *cell
                    && r["draw"] == *draw
                    && r["rule"] == *rule
                    && r["kind"] == "unsat"
                    && r["status"] == "all-refute"
                    && r["k"].as_u64().unwrap() >= 1
            })
            .min_by_key(|r| r["k"].as_u64().unwrap());
        let cell_of = |k: u32| ks.get(&k).cloned().unwrap_or_else(|| "not run".into());
        println!(
            "| {cell} | {draw} | {rule} | {} | {} | {} | {} | {} |",
            best.map_or("> run".into(), |r| r["k"].to_string()),
            best.map_or("-".into(), |r| format!(
                "{:.4e}",
                r["sum_word_ops"].as_f64().unwrap()
            )),
            cell_of(1),
            cell_of(2),
            cell_of(3)
        );
        kstar.push(json!({"cell": cell, "draw": draw, "rule": rule, "kstar": best.map(|r| r["k"].clone())}));
    }
    // ---- control N1: placebo and A/A against the committed records
    let mut n1 = Vec::new();
    for ((cell, draw, rule, _k), br) in &groups {
        let Some(b) = br.get(&0) else { continue };
        let (reference, secs_tol) = match rule.as_str() {
            "N1a" => (
                format!("research/dreg_closure_scale_20261016/runs/fresh-{cell}.d{draw}.jsonl"),
                f64::INFINITY,
            ),
            "N1b" => (
                format!("research/dreg_closure_specialise_20261017/runs/{cell}.d{draw}.A0.jsonl"),
                0.25,
            ),
            _ => continue,
        };
        let Some(c) = committed_reading(&reference, *draw) else {
            n1.push(format!(
                "{rule} {cell} d{draw}: reference {reference} missing"
            ));
            continue;
        };
        // Registered: products equal exactly.  Amended 2026-10-24 after the first N1 runs showed
        // 0.3-0.7 % run-to-run spread (parallel scheduling): `--prod-tol 0` is the registered reading.
        let prod_tol: f64 = flag(args, "--prod-tol").map_or(0.02, |v| v.parse().unwrap());
        let ok = b.cols == c.0
            && b.rank == c.1
            && (b.products / c.2 - 1.0).abs() <= prod_tol
            && (b.ops / c.3 - 1.0).abs() <= 0.01
            && b.refuted == c.6
            && (b.secs / c.4 - 1.0).abs() <= secs_tol;
        println!("N1 {rule} {cell} d{draw}: cols {}/{} rank {}/{} products {}/{} ops ratio {:.5} secs ratio {:.3} -> {}",
            b.cols, c.0, b.rank, c.1, b.products, c.2, b.ops / c.3, b.secs / c.4, if ok { "pass" } else { "FAIL" });
        if !ok {
            n1.push(format!(
                "{rule} {cell} d{draw} does not reproduce its record"
            ));
        }
    }
    // ---- cost table: hybrid at k* against the projected degree-5 closure (3 exponents, 3 weights)
    let baseline = |cell: &str, draw: u64| -> Option<(f64, f64, f64, f64)> {
        // (n_vars, ops4, products4, cols4)
        if cell.ends_with('f') {
            let b = groups
                .get(&(cell.to_string(), draw, "R1".into(), 0))?
                .get(&0)?;
            return Some((b.n_vars as f64, b.ops, b.products, b.cols));
        }
        let c = committed_reading(
            &format!("research/dreg_closure_scale_20261016/runs/fresh-{cell}.d{draw}.jsonl"),
            draw,
        )?;
        Some((c.5, c.3, c.2, c.0))
    };
    let es = [3.0f64, 4.1, 4.25];
    let gs = [0.0f64, 0.1, 1.0];
    println!("\n| cell | draw | k* | hybrid ops | with failed attempts | hybrid / degree 5 at e = 3 | e = 4.1 | e = 4.25 | order same at every e, g |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|---|");
    let mut costs = Vec::new();
    for ((cell, draw, rule), ks) in &seen {
        if rule != "R1" {
            continue;
        }
        let Some(r) = rows
            .iter()
            .filter(|r| {
                r["cell"] == *cell
                    && r["draw"] == *draw
                    && r["rule"] == "R1"
                    && r["kind"] == "unsat"
                    && r["status"] == "all-refute"
                    && r["k"].as_u64().unwrap() >= 1
            })
            .min_by_key(|r| r["k"].as_u64().unwrap())
        else {
            continue;
        };
        let Some((nv, ops4, prod4, cols4)) = baseline(cell, *draw) else {
            continue;
        };
        let kk = r["k"].as_u64().unwrap();
        let h = r["sum_word_ops"].as_f64().unwrap();
        let hp = r["sum_cols_x_products_words"].as_f64().unwrap();
        let tried: f64 = rows
            .iter()
            .filter(|q| {
                q["cell"] == *cell
                    && q["draw"] == *draw
                    && q["rule"] == "R1"
                    && q["kind"] == "unsat"
                    && q["k"].as_u64().unwrap() >= 1
                    && q["k"].as_u64().unwrap() <= kk
            })
            .map(|q| q["sum_word_ops"].as_f64().unwrap())
            .sum();
        let ratio_col = (log2_monomials(nv, 5).exp2()) / (log2_monomials(nv, 4).exp2());
        let unpriced4 = prod4 * (cols4 / 64.0).ceil();
        let mut ratios = Vec::new();
        let mut same = true;
        for &e in &es {
            let sc = ratio_col.powf(e);
            let (mut lo, mut hi) = (false, false);
            for &g in &gs {
                let hybrid = h + g * hp;
                let d5 = ops4 * sc + g * unpriced4 * sc;
                if hybrid < d5 {
                    lo = true
                } else {
                    hi = true
                }
            }
            same &= !(lo && hi) && lo;
            ratios.push(h / (ops4 * sc));
        }
        println!(
            "| {cell} | {draw} | {kk} | {h:.3e} | {tried:.3e} | {:.2e} | {:.2e} | {:.2e} | {} |",
            ratios[0],
            ratios[1],
            ratios[2],
            if same {
                "hybrid cheaper"
            } else {
                "unresolved or degree 5 cheaper"
            }
        );
        costs.push(json!({"cell": cell, "draw": draw, "kstar": kk, "hybrid_ops": h, "with_failed_attempts": tried, "ops4_unspecialised": ops4,
            "col_ratio_5_over_4": ratio_col, "ratios": ratios, "hybrid_cheaper_everywhere": same}));
        let _ = ks;
    }
    println!(
        "\nControls: {}",
        if control_fail.is_empty() {
            "no soundness failure".to_string()
        } else {
            format!("FAILED: {control_fail:?}")
        }
    );
    println!(
        "N1 reproducibility: {}",
        if n1.is_empty() {
            "pass (or not run)".to_string()
        } else {
            format!("FAILED: {n1:?}")
        }
    );
    if let Some(p) = out_path {
        std::fs::write(p, serde_json::to_string_pretty(&json!({"rows": rows, "kstar": kstar, "costs": costs, "control_failures": control_fail, "n1_failures": n1})).unwrap()).unwrap();
    }
    let _ = log2_monomials(1.0, 1);
}

// ---------------------------------------------------------------- model

const N_FIELD: usize = 131;
const RHO: f64 = 60.81;

fn log2_add(a: f64, b: f64) -> f64 {
    a.max(b) + (1.0 + (-(a - b).abs()).exp2()).log2()
}
fn factorial(m: usize) -> f64 {
    (1..=m).map(|k| k as f64).product()
}

/// Total log2 cost, `descent_degree_cost`'s model, with the oracle
/// `2^k * C(N - k, <=D)^w` per target.
fn total(m: usize, l: usize, degree: u32, w: f64, k: usize) -> f64 {
    let n = N_FIELD;
    let unknowns = (m - 2) * n + m * l;
    let per_relation = (factorial(m).log2() + n as f64 - (m * l) as f64).max(0.0);
    let targets = l as f64 + per_relation;
    let oracle = k as f64 + w * log2_monomials(unknowns.saturating_sub(k) as f64, degree);
    let algebra = (m as f64).log2() + (2 * l) as f64;
    log2_add(targets + oracle, algebra)
}

fn best(m: usize, degree: u32, w: f64, kfun: &dyn Fn(usize) -> usize) -> (usize, usize, f64) {
    (4..=70)
        .map(|l| (l, kfun(l), total(m, l, degree, w, kfun(l))))
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}

fn model(args: &[String]) {
    let kappas: Vec<usize> = flag(args, "--kappa").map_or(vec![1, 2, 3], |s| {
        s.split(',').map(|x| x.parse().unwrap()).collect()
    });
    println!("rho reference: 2^{RHO} (RESEARCH_ECC2K130_DECOMPOSITION.md 5.2); units: column touches, unconverted");
    println!("| m | scenario | kappa | w | best l | k at best l | total | vs rho (bits) |");
    println!("|--:|---|--:|--:|--:|--:|--:|--:|");
    for m in [3usize, 4] {
        for w in [1.0, 2.0] {
            for (name, d) in [
                ("degree 4, k = 0 (contradicted)", 4u32),
                ("degree 5, k = 0", 5),
            ] {
                let (l, k, t) = best(m, d, w, &|_| 0);
                println!(
                    "| {m} | {name} | - | {w} | {l} | {k} | 2^{t:.2} | {:+.2} |",
                    t - RHO
                );
            }
            for &kappa in &kappas {
                type KFn = Box<dyn Fn(usize) -> usize>;
                let scen: Vec<(&str, KFn)> = vec![
                    ("const k = kappa", Box::new(move |_| kappa)),
                    (
                        "k = kappa (l - 6)",
                        Box::new(move |l| kappa * l.saturating_sub(6)),
                    ),
                    (
                        "k = kappa m (l - 6)",
                        Box::new(move |l| kappa * m * l.saturating_sub(6)),
                    ),
                ];
                for (name, f) in scen {
                    let (l, k, t) = best(m, 4, w, &*f);
                    println!(
                        "| {m} | hybrid, {name} | {kappa} | {w} | {l} | {k} | 2^{t:.2} | {:+.2} |",
                        t - RHO
                    );
                }
            }
        }
    }
    // Break-even: largest k at which 2^k * C(N-k,<=4)^w still beats C(N,<=5)^w.
    println!("\nbreak-even guesses against degree 5 (oracle only), N = 2n + 4 l at n = 131:");
    for l in [7usize, 20, 34] {
        let nn = 2 * N_FIELD + 4 * l;
        for w in [1.0, 2.0, 3.0, 4.2] {
            let d5 = w * log2_monomials(nn as f64, 5);
            let kmax = (0..=200usize)
                .take_while(|&k| k as f64 + w * log2_monomials((nn - k) as f64, 4) < d5)
                .count();
            println!("  l = {l:>2}, N = {nn}, w = {w}: hybrid beats degree 5 while k < {kmax}");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("plant") => plant(&args),
        Some("score") => score(&args),
        Some("model") => model(&args),
        _ => eprintln!("usage: dreg_hybrid plant|score|model ..."),
    }
}
