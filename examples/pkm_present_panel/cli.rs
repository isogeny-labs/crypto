//! Subcommands: `cell`, `targets`, `export`, `solve`, `verify-msolve`, `describe`
//! (`[pcc:unverified]` instrument construction; every output is JSON sealed with a `sha256`).
use crate::cell::{derive_from, Cell, RULE_TEXT};
use crate::decode::{self, Loaded};
use crate::ec::{add_opt, lift_small, Aff};
use crate::present::{self, CellPre, Inst, PRESENTATIONS};
use crate::targets::{decompositions_of, enumerate_real, symbols};
use crate::util::*;
use crypto_lib::cryptanalysis::f4_fp::{
    f4, field_ops_total, native_ops_total, normalise, observe_progress, solve_visit, F4Options, F4Progress, Ordering, Poly,
};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Symmetric arms need `B_real >= m + 1` (protocol rule): `m` distinct real abscissae plus one spare.
pub fn sym_applicable(b_real: usize, m: usize) -> bool {
    b_real > m
}

fn distinct_abs(d: &[u8]) -> bool {
    let s: BTreeSet<u8> = d.iter().map(|k| k / 2).collect();
    s.len() == d.len()
}

/// `(real_count, real_count_distinct)` of abscissa `x_r` by the m8 classifier tables.
fn classify(cell: &Cell, m: usize, x_r: u64) -> Result<(usize, usize), String> {
    let tab = enumerate_real(cell, m, 250_000_000)?;
    classify_with(cell, &tab, m, x_r)
}
fn classify_with(cell: &Cell, tab: &crate::targets::RealTable, m: usize, x_r: u64) -> Result<(usize, usize), String> {
    let r = lift_small(cell.p, cell.a, cell.b, x_r).ok_or("x_R is not an abscissa of the curve")?;
    let ds = decompositions_of(cell, &symbols(cell), tab, m, r);
    Ok((ds.len(), ds.iter().filter(|d| distinct_abs(d)).count()))
}

fn verify_data(cell: &Cell, tj: &Value, tg: &Value, counts: (usize, usize), m: usize) -> Value {
    json!({
        "cell_sha256": cell.sha256, "slug": cell.slug,
        "targets_sha256": tj["sha256"],
        "target": tg["name"], "label": tg["label"], "x_r": tg["x_r"],
        "real_count": counts.0, "real_count_distinct": counts.1,
        "real_count_in_targets_json": tg["real_count"], "algebraic_count": tg["algebraic_count"],
        "ordered_algebraic_solutions": tg["ordered_algebraic_solutions"],
        "b_real": cell.real.len(), "sym_applicable": sym_applicable(cell.real.len(), m),
        "real_base": cell.real.iter().map(|e| json!({"v_index": e.v_index, "x": e.x, "y": e.y})).collect::<Vec<_>>(),
    })
}

fn num_arg(args: &[String], name: &str, default: usize) -> Result<Option<usize>, String> {
    let Some(i) = args.iter().position(|a| a == name) else { return Ok(None) };
    match args.get(i + 1).filter(|s| !s.starts_with("--")) {
        Some(s) => s.parse().map(Some).map_err(|_| format!("{name} expects a count")),
        None => Ok(Some(default)),
    }
}

// ---------------------------------------------------------------- cell ----

fn cell_json_nested(big: &Value, t2: usize, slug: &str) -> Result<Value, String> {
    let p = u64_of(big, "p")?;
    let (a, b, g) = (u64_of(big, "a")?, u64_of(big, "b")?, u64_of(big, "g")?);
    let t = u64_of(big, "T")? as usize;
    if t2 >= t {
        return Err("nested t must be below the pinned t".into());
    }
    let zeta = powm(u64_of(big, "zeta")?, 1u64 << (t - t2), p);
    let v: Vec<u64> = (0..1u64 << t2).map(|k| mulm(g, powm(zeta, k, p), p)).collect();
    let real: Vec<(usize, u64, u64)> = v
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| lift_small(p, a, b, x).map(|(x, y)| (i, x, y)))
        .collect();
    let mut j = big.clone();
    j["T"] = json!(t2);
    j["zeta"] = json!(zeta);
    j["B"] = json!(1usize << t2);
    j["B_real"] = json!(real.len());
    j["signed_points"] = json!(2 * real.len());
    j["V"] = json!(v);
    j["V_real"] = json!(real.iter().map(|e| e.1).collect::<Vec<_>>());
    j["real_base"] = json!(real.iter().map(|e| json!({"v_index": e.0, "x": e.1, "y": e.2})).collect::<Vec<_>>());
    j["derivation_slug"] = big["slug"].clone();
    j["slug"] = json!(slug);
    j["nested_from"] = json!({
        "cell_sha256": big["sha256"], "t": t,
        "rule": "same p, a, b, r, g, generator; zeta_t' = zeta_t^(2^(t-t')); V_t' = g*mu_(2^t') is a subset of V_t; real base re-derived (x^3+ax+b a non-zero square)",
    });
    Ok(seal(j))
}

pub fn run_cell(args: &[String]) -> Result<(), String> {
    let t: usize = crate::arg_req(args, "--t")?.parse().map_err(|_| "--t")?;
    let ratio: u32 = crate::arg_opt(args, "--ratio").map_or(Ok(1), |s| s.parse()).map_err(|_| "--ratio")?;
    let seed: u64 = crate::arg_req(args, "--seed")?.parse().map_err(|_| "--seed")?;
    let out = crate::arg_req(args, "--out")?;
    let prime: Option<u64> = crate::arg_opt(args, "--prime").map(|s| s.parse()).transpose().map_err(|_| "--prime")?;
    let req_b: usize = crate::arg_opt(args, "--require-breal").map_or(Ok(0), |s| s.parse()).map_err(|_| "--require-breal")?;
    let slug_over = crate::arg_opt(args, "--slug");
    let nested: Vec<usize> = match crate::arg_opt(args, "--nested-ts") {
        Some(s) => s.split(',').map(|x| x.trim().parse().map_err(|_| "--nested-ts".to_string())).collect::<Result<_, _>>()?,
        None => vec![],
    };
    if prime.is_none() && !nested.is_empty() {
        return Err("--nested-ts needs --prime (one pinned curve)".into());
    }
    // --min-prime N: start the eligibility walk at N instead of 2^(t*ratio) (ignored when --prime pins the curve).
    let min_prime: Option<u64> = crate::arg_opt(args, "--min-prime").map(|s| s.parse()).transpose().map_err(|_| "--min-prime")?;
    // --require-breal-nested t:N,t:N : the candidate curve must also have B_real >= N at the nested base g*mu_{2^t}
    // (checked on the same curve with zeta_t = zeta^(2^(T-t))); a failing prime is rejected and recorded like --require-breal.
    let req_nested: Vec<(usize, usize)> = match crate::arg_opt(args, "--require-breal-nested") {
        Some(s) => s
            .split(',')
            .map(|x| {
                let (a, b) = x.trim().split_once(':').ok_or_else(|| "--require-breal-nested wants t:N".to_string())?;
                Ok((a.parse().map_err(|_| "--require-breal-nested t")?, b.parse().map_err(|_| "--require-breal-nested N")?))
            })
            .collect::<Result<_, String>>()?,
        None => vec![],
    };
    let mut rejections: Vec<Value> = Vec::new();
    let mut lo = prime.or(min_prime);
    let mut tries = 0;
    let d = loop {
        tries += 1;
        if tries > 512 {
            return Err("no prime satisfies --require-breal within 512 eligible primes".into());
        }
        let d = derive_from(t, ratio, seed, true, lo)?;
        if let Some(pp) = prime {
            if d.cell.p != pp {
                return Err(format!("--prime {pp} is not eligible at t = {t}: the rule stepped to {} (see rejected_primes in the derivation)", d.cell.p));
            }
        }
        if d.cell.real.len() < req_b {
            if prime.is_some() {
                return Err(format!("pinned prime {} has B_real = {} < --require-breal {req_b}", d.cell.p, d.cell.real.len()));
            }
            rejections.push(json!({"p": d.cell.p, "reason": format!("B_real = {} < {req_b}", d.cell.real.len())}));
            lo = Some(d.cell.p + 1);
            continue;
        }
        let mut nested_short: Option<String> = None;
        for &(t2, n) in &req_nested {
            let nj = cell_json_nested(&d.json, t2, "probe")?;
            let br = nj["B_real"].as_u64().unwrap_or(0) as usize;
            if br < n {
                nested_short = Some(format!("B_real(t = {t2}) = {br} < {n}"));
                break;
            }
        }
        if let Some(reason) = nested_short {
            if prime.is_some() {
                return Err(format!("pinned prime {} fails --require-breal-nested: {reason}", d.cell.p));
            }
            rejections.push(json!({"p": d.cell.p, "reason": reason}));
            lo = Some(d.cell.p + 1);
            continue;
        }
        break d;
    };
    let mut j = d.json.clone();
    if let Some(s) = &slug_over {
        j["derivation_slug"] = j["slug"].clone();
        j["slug"] = json!(s);
    }
    j["present_wrapper"] = json!({
        "flags": {"no_l1_reuse": true, "prime": prime, "require_breal": req_b, "min_prime": min_prime, "require_breal_nested": req_nested, "slug": slug_over, "nested_ts": nested, "ratio": ratio, "seed": seed, "t": t},
        "require_breal_rejections": rejections,
    });
    let j = seal(j);
    write_json(&Path::new(&out).join("cell.json"), &j)?;
    let mut cells = vec![json!({"t": t, "path": "cell.json", "sha256": j["sha256"], "slug": j["slug"], "p": j["p"], "B_real": j["B_real"]})];
    for &t2 in &nested {
        let slug = format!("{}-nested-t{t2}", j["slug"].as_str().unwrap_or("cell"));
        let nj = cell_json_nested(&j, t2, &slug)?;
        let rel = format!("nested_t{t2}/cell.json");
        write_json(&Path::new(&out).join(&rel), &nj)?;
        cells.push(json!({"t": t2, "path": rel, "sha256": nj["sha256"], "slug": slug, "p": nj["p"], "B_real": nj["B_real"]}));
    }
    let sym: Vec<Value> = cells
        .iter()
        .map(|c| {
            let b = c["B_real"].as_u64().unwrap_or(0) as usize;
            json!({"t": c["t"], "B_real": b, "sym_applicable_m": (2..=8).map(|m| json!({"m": m, "applicable": sym_applicable(b, m)})).collect::<Vec<_>>()})
        })
        .collect();
    let rec = seal(json!({
        "schema": "pkm.present-panel.cell/v1",
        "doc": "[pcc:unverified] instrument output; no claim",
        "rule": RULE_TEXT,
        "wrapper_rule": "derive_from(t, ratio, seed, no_l1_reuse = true, min_prime = prime); --prime P pins the prime (error if the eligibility rule steps off P); --require-breal N steps to the next eligible prime while B_real < N (each rejection recorded; error with --prime); --slug NAME replaces the cell slug that seeds the targets/planted RNG streams (derivation_slug keeps the one that seeded the curve); --nested-ts reuses p, a, b, r, g, generator at smaller t with zeta_t' = zeta_t^(2^(t-t')) and slug <slug>-nested-t<t'>",
        "sym_applicable_rule": "B_real >= m + 1",
        "flags": {"no_l1_reuse": true, "prime": prime, "require_breal": req_b, "min_prime": min_prime, "require_breal_nested": req_nested, "slug": slug_over, "nested_ts": nested, "ratio": ratio, "seed": seed, "t": t},
        "require_breal_rejections": rejections,
        "cells": cells, "sym_applicability": sym,
        "tool": crate::tool_info(),
    }));
    write_json(&Path::new(&out).join("cell.present.json"), &rec)?;
    println!("cell {} p={} t={t} B_real={} nested={nested:?} sha256={}", j["slug"], j["p"], j["B_real"], j["sha256"]);
    Ok(())
}

// ------------------------------------------------------------- targets ----

pub fn run_targets(args: &[String]) -> Result<(), String> {
    let cdir = crate::arg_req(args, "--cell")?;
    let cell = Cell::load(Path::new(&cdir))?;
    let m: usize = crate::arg_opt(args, "--m").map_or(Ok(8), |s| s.parse()).map_err(|_| "--m")?;
    let seed: u64 = crate::arg_req(args, "--seed")?.parse().map_err(|_| "--seed")?;
    let out = crate::arg_opt(args, "--out").unwrap_or_else(|| format!("{cdir}/targets-s{seed}"));
    let n_pd = num_arg(args, "--planted-distinct", 1)?.unwrap_or(0);
    let mut margs: Vec<String> = Vec::new();
    let mut skip = false;
    for (i, a) in args.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if a == "--planted-distinct" {
            skip = args.get(i + 1).is_some_and(|s| !s.starts_with("--"));
            continue;
        }
        margs.push(a.clone());
    }
    crate::targets::run(&margs)?;
    let tpath = Path::new(&out).join("targets.json");
    let mut tj = read_json(&tpath)?;
    write_json(&Path::new(&out).join("targets.m8.json"), &tj)?;
    let tab = enumerate_real(&cell, m, 250_000_000)?;
    let sym = symbols(&cell);
    let m8_hidden = read_json(&Path::new(&out).join("hidden.json"))?;
    let abscissae = |signed: &[i64]| -> Vec<Value> {
        let mut v: Vec<(u64, i64)> = signed.iter().map(|&s| (cell.real[(s.unsigned_abs() - 1) as usize].x, s.signum())).collect();
        v.sort_unstable();
        v.into_iter().map(|(x, s)| json!([x, s])).collect()
    };
    if let Some(ts) = tj["targets"].as_array_mut() {
        for tg in ts.iter_mut() {
            let (rc, rd) = classify_with(&cell, &tab, m, u64_of(tg, "x_r")?)?;
            if tg["real_count"].as_u64() != Some(rc as u64) {
                return Err(format!("classifier mismatch on {}", tg["name"]));
            }
            tg["real_count_distinct"] = json!(rd);
            if tg["label"] == "planted" {
                let hs = m8_hidden["targets"].as_array().and_then(|a| a.iter().find(|h| h["name"] == tg["name"]));
                let signed: Vec<i64> = hs.and_then(|h| h["planted_symbols"].as_array()).map(|a| a.iter().filter_map(Value::as_i64).collect()).unwrap_or_default();
                tg["planted_abscissae"] = json!(abscissae(&signed));
            }
        }
    }
    let mut hidden = Vec::new();
    if n_pd > 0 {
        if cell.real.len() < m {
            return Err(format!("--planted-distinct needs B_real >= m (B_real = {}, m = {m})", cell.real.len()));
        }
        let mut rng = crate::cell::seeded_rng(&cell.slug, seed, "planted-distinct");
        for i in 0..n_pd {
            let (syms, pt) = loop {
                let mut idx: Vec<usize> = Vec::new();
                while idx.len() < m {
                    let k = rng.gen_range(0..cell.real.len());
                    if !idx.contains(&k) {
                        idx.push(k);
                    }
                }
                let s: Vec<u8> = idx.iter().map(|&k| (2 * k + rng.gen_range(0..2)) as u8).collect();
                let mut acc: Option<Aff> = None;
                for &k in &s {
                    acc = add_opt(cell.p, cell.a, acc, Some(sym[k as usize]));
                }
                if let Some(pt) = acc {
                    break (s, pt);
                }
            };
            let (rc, rd) = classify_with(&cell, &tab, m, pt.0)?;
            if rd == 0 {
                return Err("planted-distinct decomposition not classified".into());
            }
            let name = format!("plantedD{i}");
            tj["targets"].as_array_mut().ok_or("targets")?.push(json!({
                "name": name, "label": "planted_distinct", "planted": true, "draw_index": null, "x_r": pt.0, "a": null, "b": null,
                "real_count": rc, "real_count_distinct": rd, "algebraic_count": null,
                "algebraic_signed_classes": null, "ordered_algebraic_solutions": null,
                "planted_abscissae": abscissae(&crate::targets::signed_json(&{ let mut s = syms.clone(); s.sort_unstable(); s })),
            }));
            let mut sorted = syms.clone();
            sorted.sort_unstable();
            hidden.push(json!({"name": name, "planted_symbols": crate::targets::signed_json(&sorted)}));
        }
    }
    let hid = seal(json!({"schema": "pkm.present-panel.hidden/v1", "note": "HIDDEN: planted-distinct decompositions; solvers never read this", "cell_sha256": cell.sha256, "targets": hidden}));
    write_json(&Path::new(&out).join("hidden.present.json"), &hid)?;
    tj["slug"] = json!(cell.slug);
    tj["b_real"] = json!(cell.real.len());
    tj["sym_applicable"] = json!(sym_applicable(cell.real.len(), m));
    tj["sym_applicable_rule"] = json!("B_real >= m + 1");
    tj["hidden_present_sha256"] = hid["sha256"].clone();
    tj["present_note"] = json!("superset of targets.m8.json: real_count_distinct per target, planted-distinct targets (plantedD*), slug; [pcc:unverified]");
    write_json(&tpath, &seal(tj))?;
    println!("targets: wrote {out} (m = {m}, planted-distinct = {n_pd}, sym_applicable = {})", sym_applicable(cell.real.len(), m));
    Ok(())
}

// -------------------------------------------------------------- export ----

pub fn run_export(args: &[String]) -> Result<(), String> {
    let cell = Cell::load(Path::new(&crate::arg_req(args, "--cell")?))?;
    let tdir = crate::arg_req(args, "--targets")?;
    let target = crate::arg_req(args, "--target")?;
    let m: usize = crate::arg_req(args, "--m")?.parse().map_err(|_| "--m")?;
    let pres = crate::arg_req(args, "--presentation")?;
    let out = crate::arg_req(args, "--out")?;
    let format = crate::arg_opt(args, "--format").unwrap_or_else(|| "both".into());
    if !["ms", "native", "both"].contains(&format.as_str()) {
        return Err("--format must be ms, native or both".into());
    }
    let all = pres == "all";
    let list: Vec<&str> = if all {
        PRESENTATIONS.to_vec()
    } else if PRESENTATIONS.contains(&pres.as_str()) {
        vec![pres.as_str()]
    } else {
        return Err(format!("unknown presentation {pres}; one of {PRESENTATIONS:?} or all"));
    };
    let tj = read_json(&Path::new(&tdir).join("targets.json"))?;
    if !check_seal(&tj) {
        return Err("targets.json seal mismatch".into());
    }
    if tj["cell_sha256"].as_str() != Some(&cell.sha256) {
        return Err("targets.json belongs to another cell".into());
    }
    if u64_of(&tj, "m")? as usize != m {
        return Err(format!("--m {m} differs from targets.json m = {}", tj["m"]));
    }
    let tgs = tj["targets"].as_array().ok_or("targets")?;
    let chosen: Vec<&Value> = tgs.iter().filter(|t| target == "all" || t["name"].as_str() == Some(&target)).collect();
    if chosen.is_empty() {
        return Err(format!("no target {target}"));
    }
    let b_real = cell.real.len();
    let applicable = sym_applicable(b_real, m);
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let tab = enumerate_real(&cell, m, 250_000_000)?;
    let inst0 = Inst { p: cell.p, a: cell.a, b: cell.b, g: cell.g, zeta: cell.zeta, m, t: cell.t, x_r: 0 };
    let mut pre = CellPre::new(&inst0);
    for tg in chosen {
        let name = tg["name"].as_str().ok_or("name")?;
        let x_r = u64_of(tg, "x_r")?;
        let inst = Inst { x_r, ..inst0.clone() };
        let counts = classify_with(&cell, &tab, m, x_r)?;
        if tg["real_count"].as_u64() != Some(counts.0 as u64) {
            return Err(format!("{name}: classifier real_count {} differs from targets.json {}", counts.0, tg["real_count"]));
        }
        let vd = verify_data(&cell, &tj, tg, counts, m);
        let mut entries = Vec::new();
        for pr in &list {
            if all && present::is_symmetric(pr) && !applicable {
                entries.push(json!({"presentation": pr, "skipped": true,
                    "reason": format!("sym_applicable = false: B_real = {b_real} < m + 1 = {}", m + 1)}));
                continue;
            }
            let t0 = Instant::now();
            let sys = present::build_pre(&mut pre, pr, &inst)?;
            let c = &sys.cost;
            let mut e = json!({
                "presentation": pr, "nvars": sys.nvars(), "nequations": sys.polys.len(),
                "max_input_degree": sys.max_input_degree(), "max_total_degree": sys.max_input_degree(),
                "nterms_total": sys.nterms_total(),
                "construction_field_multiplications": c.target_muls, "construction_cpu_s": c.target_cpu,
                "construction_cell_field_multiplications": c.cell_muls, "construction_cell_cpu_s": c.cell_cpu,
                "construction_per_cell": {"field_multiplications": c.cell_muls, "cpu_s": c.cell_cpu},
                "construction_per_target": {"field_multiplications": c.target_muls, "cpu_s": c.target_cpu},
                "build_wall_s": t0.elapsed().as_secs_f64(), "solution_semantics": present::semantics_class(pr), "solution_semantics_detail": present::semantics(pr),
                "sym_applicable": applicable,
            });
            if format != "native" {
                let text = present::ms_text(&sys);
                let path = Path::new(&out).join(format!("{name}.{pr}.ms"));
                std::fs::write(&path, &text).map_err(|x| x.to_string())?;
                e["ms_sha256"] = json!(sha256_hex(text.as_bytes()));
                e["ms_bytes"] = json!(text.len());
            }
            if format != "ms" {
                let sj = present::system_json(&sys, &vd);
                let path = Path::new(&out).join(format!("{name}.{pr}.system.json"));
                write_json(&path, &sj)?;
                e["system_sha256"] = json!(file_sha256(&path)?);
                e["system_seal_sha256"] = sj["sha256"].clone();
            }
            entries.push(e);
        }
        let man = json!({
            "schema": "pkm.present-panel.export-manifest/v1",
            "doc": "[pcc:unverified] instrument construction; no claim",
            "cell_sha256": cell.sha256, "slug": cell.slug, "targets_sha256": tj["sha256"], "target": name, "label": tg["label"],
            "m": m, "t": cell.t, "p": cell.p, "b_real": b_real, "sym_applicable": applicable,
            "real_count": counts.0, "real_count_distinct": counts.1,
            "entries": entries, "tool": crate::tool_info(),
        });
        write_json(&Path::new(&out).join(format!("{name}.manifest.json")), &seal(man))?;
        println!("exported {name}: {} presentation(s) to {out}", list.len());
    }
    Ok(())
}

pub fn run_describe(args: &[String]) -> Result<(), String> {
    let ld = decode::load(&crate::arg_req(args, "--system")?)?;
    let n = ld.vars.len();
    let degs: Vec<u32> = ld.polys.iter().map(present::total_degree).collect();
    let per_var: Vec<u32> = (0..n).map(|v| ld.polys.iter().flat_map(|f| f.iter().map(move |(e, _)| e[v])).max().unwrap_or(0)).collect();
    let j = &ld.json;
    let rec = seal(json!({
        "schema": "pkm.present-panel.describe/v1",
        "presentation": ld.presentation, "p": ld.p, "m": ld.m, "t": ld.t,
        "nvars": n, "nequations": ld.polys.len(), "variables": ld.vars,
        "degrees": degs, "max_input_degree": degs.iter().max(), "max_total_degree": degs.iter().max(),
        "max_degree_per_variable": per_var,
        "term_counts": ld.polys.iter().map(Vec::len).collect::<Vec<_>>(),
        "terms_total": ld.polys.iter().map(Vec::len).sum::<usize>(),
        "nterms_total": ld.polys.iter().map(Vec::len).sum::<usize>(),
        "construction_field_multiplications": j["construction_field_multiplications"],
        "construction_cpu_s": j["construction_cpu_s"],
        "construction": j["construction"],
        "solution_semantics": present::semantics_class(&ld.presentation),
        "solution_semantics_detail": present::semantics(&ld.presentation),
        "construction_cell_field_multiplications": j["construction_cell_field_multiplications"],
        "construction_cell_cpu_s": j["construction_cell_cpu_s"],
    }));
    if let Some(o) = crate::arg_opt(args, "--out") {
        write_json(Path::new(&o), &rec)?;
    }
    println!("{}", serde_json::to_string_pretty(&rec).map_err(|e| e.to_string())?);
    Ok(())
}

// --------------------------------------------------------------- solve ----

/// Standard monomials of a zero-dimensional leading ideal (`None` if not zero-dimensional,
/// `Err` if more than `cap`); same algorithm as `native_solve::staircase` (private there).
fn staircase(basis: &[Poly], nvars: usize, cap: usize) -> Option<Result<usize, ()>> {
    let lms: Vec<&Vec<u32>> = basis.iter().map(|f| &f[0].0).collect();
    let mut bound = vec![0u32; nvars];
    for (v, slot) in bound.iter_mut().enumerate() {
        let d = lms
            .iter()
            .filter(|e| e[v] > 0 && e.iter().enumerate().all(|(k, &x)| k == v || x == 0))
            .map(|e| e[v])
            .min()?;
        *slot = d;
    }
    fn rec(k: usize, cur: &mut Vec<u32>, bound: &[u32], lms: &[&Vec<u32>], count: &mut usize, cap: usize) -> bool {
        if *count > cap {
            return false;
        }
        let divisible = lms.iter().any(|e| e.iter().zip(cur.iter()).all(|(a, b)| a <= b) && e.iter().skip(k).all(|&x| x == 0));
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

/// Streams F4 progress to `path` as JSON lines `{"ops","degree","step","elapsed_s", ...}`: every step of
/// the solving run, and (throttled to one line per 50 ms) the steps of the extraction runs. `ops` is the
/// process-wide field-multiplication count since the start, a lower bound if the run is killed.
struct ProgressWriter {
    tx: std::sync::mpsc::Sender<F4Progress>,
    handle: std::thread::JoinHandle<()>,
    phase: Arc<Mutex<&'static str>>,
}
impl ProgressWriter {
    fn start(path: String, t0: Instant, cpu0: f64, ops0: u64) -> ProgressWriter {
        use std::io::Write;
        let (tx, rx) = std::sync::mpsc::channel::<F4Progress>();
        let phase = Arc::new(Mutex::new("solve"));
        let ph = phase.clone();
        let handle = std::thread::spawn(move || {
            let Ok(mut file) = std::fs::File::create(&path) else { return };
            let mut last_write = Instant::now() - Duration::from_secs(1);
            let mut pending: Option<String> = None;
            while let Ok(ev) = rx.recv() {
                let phase_now = *ph.lock().unwrap();
                let line = json!({
                    "ops": field_ops_total() - ops0, "degree": ev.degree, "step": ev.step, "elapsed_s": t0.elapsed().as_secs_f64(),
                    "phase": phase_now, "rows": ev.rows, "columns": ev.columns, "productive_degree": ev.productive_degree,
                    "cpu_s": cpu_secs() - cpu0,
                })
                .to_string();
                if phase_now == "solve" || last_write.elapsed() >= Duration::from_millis(50) {
                    let _ = writeln!(file, "{line}");
                    let _ = file.flush();
                    last_write = Instant::now();
                    pending = None;
                } else {
                    pending = Some(line);
                }
            }
            if let Some(l) = pending {
                let _ = writeln!(file, "{l}");
            }
        });
        ProgressWriter { tx, handle, phase }
    }
}

pub fn run_solve(args: &[String]) -> Result<(), String> {
    let sys_path = crate::arg_req(args, "--system")?;
    let max_degree: u32 = crate::arg_opt(args, "--max-degree").map_or(Ok(64), |s| s.parse()).map_err(|_| "--max-degree")?;
    let deadline: f64 = crate::arg_req(args, "--deadline")?.parse().map_err(|_| "--deadline")?;
    let out = crate::arg_req(args, "--out")?;
    let extract_q: usize = crate::arg_opt(args, "--extract-max-q").map_or(Ok(4096), |s| s.parse()).map_err(|_| "--extract-max-q")?;
    let ld = decode::load(&sys_path)?;
    let (p, n) = (ld.p, ld.vars.len());
    let input: Vec<Poly> = ld.polys.iter().map(|f| normalise(f, p, Ordering::Grevlex)).collect();
    let (ops0, nat0, cpu0, t0) = (field_ops_total(), native_ops_total(), cpu_secs(), Instant::now());
    let pw = ProgressWriter::start(format!("{out}.progress.json"), t0, cpu0, ops0);
    let opts = F4Options::new(Ordering::Grevlex, max_degree).with_budget(Duration::from_secs_f64(deadline));
    let rep = observe_progress(pw.tx.clone(), || f4(&input, n, p, &opts));
    let (wall, cpu) = (t0.elapsed().as_secs_f64(), cpu_secs() - cpu0);
    let (ops, nat) = (field_ops_total() - ops0, native_ops_total() - nat0);
    let complete = !rep.timed_out && !rep.inconsistent && rep.pairs_above_bound == 0;
    let j = &ld.json;
    let threads_env = std::env::var("RAYON_NUM_THREADS").ok();
    let mut rec = json!({
        "schema": "pkm.present-panel.solve/v2",
        "doc": "[pcc:unverified] instrument output; no claim",
        "presentation": ld.presentation,
        "system": sys_path, "system_file_sha256": file_sha256(Path::new(&sys_path))?,
        "p": p, "m": ld.m, "t": ld.t, "nvars": n, "nequations": input.len(),
        "max_input_degree": ld.polys.iter().map(present::total_degree).max(),
        "max_total_degree": ld.polys.iter().map(present::total_degree).max(),
        "nterms_total": ld.polys.iter().map(Vec::len).sum::<usize>(),
        "construction_field_multiplications": j["construction_field_multiplications"],
        "construction_cpu_s": j["construction_cpu_s"],
        "construction_cell_field_multiplications": j["construction_cell_field_multiplications"],
        "construction_cell_cpu_s": j["construction_cell_cpu_s"],
        "construction": j["construction"],
        "order": "grevlex", "max_degree": max_degree, "deadline_secs": deadline,
        "degree_bound_binding": rep.pairs_above_bound > 0,
        "D": rep.solving_degree_max,
        "solving_degree": rep.solving_degree, "solving_degree_max": rep.solving_degree_max,
        "degree_reached": rep.degree_reached,
        "field_ops": rep.field_ops, "solve_field_multiplications": rep.field_ops,
        "field_ops_total_delta": ops, "native_ops_total_delta": nat,
        "basis_size": rep.basis.len(), "inconsistent": rep.inconsistent, "timed_out": rep.timed_out,
        "pairs_above_bound": rep.pairs_above_bound, "steps": rep.steps, "steps_to_solution": rep.steps_to_solution,
        "max_rows": rep.max_rows, "max_cols": rep.max_cols,
        "censored": if complete || rep.inconsistent { "no" } else { "timeout" },
        "censored_reason": if complete || rep.inconsistent { Value::Null } else if rep.timed_out { json!("deadline") } else { json!("degree_bound") },
        "cpu_s": cpu, "wall_s": wall, "cpu_over_wall": cpu / wall.max(1e-9), "vm_hwm_kb": vm_hwm_kb(),
        "threads_used": rayon::current_num_threads(),
        "threads": {"rayon_current_num_threads": rayon::current_num_threads(), "RAYON_NUM_THREADS": threads_env,
            "note": "cpu_over_wall near 1 under RAYON_NUM_THREADS=1 means the engine honours it; well above 1 means it does not"},
        "solution_semantics": present::semantics_class(&ld.presentation),
        "solution_semantics_detail": present::semantics(&ld.presentation),
        "tool": crate::tool_info(),
    });
    let mut extract_ops: u64 = 0;
    rec["extract_field_multiplications"] = json!(0);
    rec["verified_real_decompositions"] = Value::Null;
    rec["unlifted_solutions"] = Value::Null;
    rec["fp_rational_solutions"] = Value::Null;
    if crate::flag(args, "--extract") {
        *pw.phase.lock().map_err(|_| "lock")? = "extract";
        let mut ej;
        if rep.inconsistent {
            let ex = decode::process(&ld, &[], 1)?;
            ej = decode::extract_json(&ld, &ex);
            ej["status"] = json!("inconsistent: 1 in the ideal, no solution over the algebraic closure");
        } else if !complete {
            ej = json!({"status": "skipped: F4 run censored"});
        } else {
            let q = staircase(&rep.basis, n, extract_q);
            let skip = match q {
                Some(Ok(_)) => None,
                Some(Err(())) => Some(format!("quotient dimension above --extract-max-q {extract_q}")),
                None => Some("not zero-dimensional".to_string()),
            };
            ej = json!({"extract_max_q": extract_q, "quotient_dimension": match q { Some(Ok(d)) => json!(d), _ => Value::Null }});
            if let Some(why) = skip {
                ej["status"] = json!(format!("skipped: {why}"));
            } else {
                let budget = (deadline - wall).max(1.0);
                let o2 = F4Options::new(Ordering::Grevlex, max_degree).with_budget(Duration::from_secs_f64(budget));
                let mut sols: Vec<Vec<u64>> = Vec::new();
                let (o_ops, c1, t1) = (field_ops_total(), cpu_secs(), Instant::now());
                let vr = observe_progress(pw.tx.clone(), || {
                    solve_visit(&input, n, p, &o2, &vec![None; n], |s| {
                        sols.push(s.to_vec());
                        false
                    })
                })?;
                let f4_ops = field_ops_total() - o_ops;
                ej["method"] = json!("f4_fp::solve_visit, unrestricted domains (a second F4 run plus substitution tree, charged to extraction)");
                ej["complete"] = json!(vr.complete);
                ej["timed_out"] = json!(vr.timed_out);
                ej["f4_runs"] = json!(vr.f4_runs);
                ej["extraction_budget_s"] = json!(budget);
                ej["f4_field_ops"] = json!(f4_ops);
                ej["root_field_multiplications"] = json!(vr.root_field_multiplications);
                ej["cpu_s"] = json!(cpu_secs() - c1);
                ej["wall_s"] = json!(t1.elapsed().as_secs_f64());
                ej["solutions_visited"] = json!(vr.solutions_visited);
                ej["candidates_rejected"] = json!(vr.candidates_rejected);
                extract_ops = f4_ops + vr.root_field_multiplications;
                if vr.complete {
                    let ex = decode::process(&ld, &sols, 1)?;
                    let xj = decode::extract_json(&ld, &ex);
                    extract_ops += xj["lift_and_root_field_multiplications"].as_u64().unwrap_or(0);
                    if let (Some(a), Some(b)) = (ej.as_object_mut(), xj.as_object()) {
                        a.extend(b.iter().map(|(k, v)| (k.clone(), v.clone())));
                    }
                } else {
                    ej["status"] = json!("extraction censored; no solutions reported");
                }
            }
        }
        for k in ["verified_real_decompositions", "verified_real_decompositions_count", "unlifted_solutions", "fp_rational_solutions", "matches_classifier"] {
            rec[k] = ej.get(k).cloned().unwrap_or(Value::Null);
        }
        rec["extract_field_multiplications"] = json!(extract_ops);
        rec["extraction"] = ej;
    }
    rec["total_wall_s"] = json!(t0.elapsed().as_secs_f64());
    rec["total_cpu_s"] = json!(cpu_secs() - cpu0);
    // finish the progress stream: the writer ends when every sender is gone
    let ProgressWriter { tx, handle, .. } = pw;
    drop(tx);
    let _ = handle.join();
    {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(format!("{out}.progress.json")).map_err(|e| e.to_string())?;
        let fin = json!({"ops": field_ops_total() - ops0, "degree": rep.degree_reached, "step": rep.steps, "elapsed_s": t0.elapsed().as_secs_f64(), "final": true, "censored": rec["censored"]});
        writeln!(f, "{fin}").map_err(|e| e.to_string())?;
    }
    write_json(Path::new(&out), &seal(rec))?;
    println!(
        "solve {}: inconsistent={} timed_out={} censored={} D={} field_ops={} cpu_s={:.3}",
        ld.presentation, rep.inconsistent, rep.timed_out, !complete && !rep.inconsistent, rep.solving_degree_max, rep.field_ops, cpu
    );
    Ok(())
}

fn jidx(v: &Value, i: usize) -> Result<u64, String> {
    v[i].as_u64().ok_or_else(|| format!("expected integer at {i}"))
}

/// Full assignments (in the system's variable order) from a `-P 1` msolve output.
pub fn msolve_assignments(
    ld: &Loaded,
    v: &Value,
    seed: u64,
    root_muls: &mut u64,
    recon_muls: &mut u64,
    meta: &mut Value,
) -> Result<Vec<Vec<u64>>, String> {
    let p = ld.p;
    let hdr = &v[1];
    if jidx(hdr, 0)? != p {
        return Err("msolve characteristic differs from the system".into());
    }
    let names: Vec<String> = hdr[3].as_array().ok_or("names")?.iter().map(|n| n.as_str().unwrap_or("").to_string()).collect();
    let lin: Vec<i64> = hdr[4].as_array().ok_or("lin")?.iter().map(|n| n.as_i64().unwrap_or(0)).collect();
    if lin.len() != names.len() {
        return Err("lin/names length mismatch".into());
    }
    let body = &hdr[5][1];
    let f = u64_vec(&body[0][1])?;
    if f.last() != Some(&1) {
        return Err("eliminating polynomial not monic".into());
    }
    let w = u64_vec(&body[1][1])?;
    let params: Vec<Vec<u64>> = body[2].as_array().ok_or("params")?.iter().map(|e| u64_vec(&e[0][1])).collect::<Result<_, _>>()?;
    let t_var: usize = if names.last().map(String::as_str) == Some("A") {
        names.len() - 1
    } else {
        lin.iter().position(|&c| c == 1).ok_or("no unit separating variable")?
    };
    let others: Vec<usize> = (0..names.len()).filter(|&i| i != t_var).collect();
    if others.len() != params.len() {
        return Err(format!("param count {} != {}", params.len(), others.len()));
    }
    let roots = poly_roots_fp_counted_until(&f, p, seed, root_muls, None).unwrap_or_default();
    meta["quotient_dimension_with_multiplicity"] = json!(jidx(hdr, 2)?);
    meta["eliminating_degree"] = json!(f.len() - 1);
    meta["separating_variable"] = json!(names[t_var]);
    meta["fp_roots_of_eliminating_polynomial"] = json!(roots.len());
    let pos: Vec<usize> = ld
        .vars
        .iter()
        .map(|nm| names.iter().position(|n| n == nm).ok_or_else(|| format!("variable {nm} absent from msolve output")))
        .collect::<Result<_, _>>()?;
    let mut sols = Vec::new();
    for &t0 in &roots {
        let value_of = |ix: usize, recon: &mut u64| -> u64 {
            if ix == t_var {
                return t0;
            }
            let k = others.iter().position(|&o| o == ix).unwrap_or(0);
            let mut acc = 0u64;
            for &c in params[k].iter().rev() {
                acc = addm(mulm(acc, t0, p), c, p);
                *recon += 1;
            }
            let mut val = (p - acc) % p;
            if w.len() > 1 {
                let mut wv = 0u64;
                for &c in w.iter().rev() {
                    wv = addm(mulm(wv, t0, p), c, p);
                    *recon += 1;
                }
                val = mulm(val, invm(wv, p), p);
                *recon += 1;
            }
            val
        };
        sols.push(pos.iter().map(|&ix| value_of(ix, recon_muls)).collect());
    }
    Ok(sols)
}

pub fn run_verify_msolve(args: &[String]) -> Result<(), String> {
    let t0 = Instant::now();
    let cpu0 = cpu_secs();
    let sp = crate::arg_req(args, "--system-json")?;
    let ms_path = crate::arg_req(args, "--msolve-out")?;
    let out = crate::arg_req(args, "--out")?;
    let ld = decode::load(&sp)?;
    let text = std::fs::read_to_string(&ms_path).map_err(|e| format!("read {ms_path}: {e}"))?;
    let mut rec = json!({
        "schema": "pkm.present-panel.verify-msolve/v1",
        "doc": "[pcc:unverified] instrument output; no claim",
        "presentation": ld.presentation, "system": sp, "system_file_sha256": file_sha256(Path::new(&sp))?,
        "msolve_output": ms_path, "msolve_output_sha256": sha256_hex(text.as_bytes()),
        "nvars": ld.vars.len(), "nequations": ld.polys.len(),
    });
    let (mut root_muls, mut recon_muls) = (0u64, 0u64);
    let mut sols: Vec<Vec<u64>> = Vec::new();
    match crate::verify::parse_msolve(&text)? {
        crate::verify::Outcome::NoSolution => rec["msolve_outcome"] = json!("no_solution_in_closure"),
        crate::verify::Outcome::PosDim => rec["msolve_outcome"] = json!("positive_dimension"),
        crate::verify::Outcome::ZeroDim(v) => {
            rec["msolve_outcome"] = json!("zero_dimensional");
            let seed: u64 = crate::arg_opt(args, "--seed").map_or(Ok(1), |s| s.parse()).map_err(|_| "--seed")?;
            let mut meta = json!({});
            sols = msolve_assignments(&ld, &v, seed, &mut root_muls, &mut recon_muls, &mut meta)?;
            if let (Some(a), Some(b)) = (rec.as_object_mut(), meta.as_object()) {
                a.extend(b.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
        }
    }
    let ex = decode::process(&ld, &sols, 1)?;
    let xj = decode::extract_json(&ld, &ex);
    rec["verified_real_decompositions"] = xj["verified_real_decompositions"].clone();
    rec["verified_real_decompositions_count"] = xj["verified_real_decompositions_count"].clone();
    rec["fp_rational_solutions"] = xj["fp_rational_solutions"].clone();
    rec["unlifted_solutions"] = xj["unlifted_solutions"].clone();
    rec["extraction"] = xj;
    rec["reconstruction_field_muls"] = json!(recon_muls);
    rec["root_finding_field_muls"] = json!(root_muls);
    rec["wall_s"] = json!(t0.elapsed().as_secs_f64());
    rec["cpu_s"] = json!(cpu_secs() - cpu0);
    rec["tool"] = crate::tool_info();
    println!(
        "verify-msolve {}: outcome={} verified={} unlifted={}",
        ld.presentation, rec["msolve_outcome"], rec["verified_real_decompositions_count"], rec["unlifted_solutions"]
    );
    write_json(Path::new(&out), &seal(rec))?;
    Ok(())
}
